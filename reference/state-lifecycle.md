# State and Memory Lifecycle

ApexDisk holds non-trivial state on both sides of the IPC: a `FolderInfo` tree on Rust, then again as reactive refs on Vue; nav stacks and selection maps in components; a handful of process-lifetime statics. This file maps every UI event to what allocates, replaces, or frees that state, so a change to the scan flow, a new view transition, or a new IPC event doesn't introduce leaks, double-starts, or orphaned walkers.

For the higher-level "what each side owns" picture, see [`architecture.md`](architecture.md). For the IPC channel inventory, see [`tauri-commands.md`](tauri-commands.md).

## State map at a glance

```
                Webview (Vue 3 + TS)                              Rust (Tauri 2)
┌────────────────────────────────────────┐   ┌────────────────────────────────────────────┐
│  Task-scoped (per scan)                │   │  Task-scoped (per scan)                    │
│   • folders: shallowRef<FolderInfo[]>  │   │   • Arc<AtomicBool> cancel token           │
│   • progress: ref<ScanProgress>        │   │   • LiveScanState (atomics + AppHandle)    │
│   • scanGeneration: ref<number>        │   │   • rayon walker threads                   │
│   • backStack / forwardStack / current │   │   • per-dir BinaryHeap<HeapEntry> (≤300)   │
│   • selectedMap: reactive(Map)         │   │   • ScanRunningGuard (RAII)                │
│                                        │   │                                            │
│  View-scoped (until KeepAlive drops)   │   │  Process-lifetime                          │
│   • checkedMapRef: shallowRef(Map)     │   │   • SCAN_RUNNING: AtomicBool               │
│   • renderedCount, expand/elapsed timers │ │   • ACTIVE_CANCEL: Mutex<Option<Arc<…>>>   │
│                                        │   │   • STORE_LOCK: Mutex<()>                  │
│  App-lifetime                          │   │   • VALID_SETTING_KEYS (LazyLock, leaked)  │
│   • settings: ref<AppSettings>         │   │   • PROTECTED_SET / SKIPPED_LOWERED        │
│   • translation YAMLs (imported once)  │   │   • UpdateState.ready_version / .last_…    │
└────────────────────────────────────────┘   └────────────────────────────────────────────┘
                                ▲                                       ▲
                                └─────────────  IPC  ───────────────────┘
                                  invoke('…') · listen('…')
```

The `FolderInfo` tree is the single largest payload that crosses this boundary. Rust builds it under the per-directory file cap (`MAX_FILES_PER_DIR = 300`), serializes it to the webview as the resolved value of `invoke('get_user_folders')`, and **does not retain a copy**. Vue assigns the result into `folders` (`shallowRef`); the previous tree becomes garbage as soon as no nav stack still points at it.

## Scan lifecycle

### Start triggers

| Trigger                                   | UI source                                                                                         | Wired via                                                                                                                               | What it calls                                                         |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| User clicks **Scan** on the launch screen | [ScanLaunch.vue](../src/components/ScanLaunch.vue) `emit('start-scan')`                           | [ScanView.vue:184](../src/components/ScanView.vue#L184) `@start-scan="loadFolders"`                                                     | `loadFolders()` in [use-scanner.ts:77](../src/lib/use-scanner.ts#L77) |
| User clicks **Scan again** after trashing | [ScanTrashConfirmation.vue:49](../src/components/ScanTrashConfirmation.vue#L49) `emit('restart')` | [ScanView.onRestart](../src/components/ScanView.vue#L168) → `onCancel()` wipes Vue state → folders=[] → activeView falls back to LAUNCH | User then clicks **Scan** again (no auto-restart)                     |

`loadFolders()` in order:

1. Unlisten any prior `folder-scan-progress` handle.
2. Bump `scanGeneration`; snapshot it as `gen`.
3. Set `isScanning = true`, reset `progress`, start the elapsed timer.
4. `listen('folder-scan-progress')` → assign to `unlistenProgress`.
5. `invoke('get_user_folders', { options })` with the four show-\* flags from settings.
6. On resolve, if `gen === scanGeneration`: assign `folders.value = result`. Otherwise drop silently.
7. `finally` unlistens, sets `isScanning = false`, stops the timer — only if `gen` still matches.

What Rust allocates inside `get_user_folders` ([scan.rs:644](../src-tauri/src/scan.rs#L644)):

- `Arc<AtomicBool>` cancel token. Stored in `ACTIVE_CANCEL` (under `Mutex`) and cloned into `LiveScanState`.
- `ScanRunningGuard` (RAII). Setup acquires `SCAN_RUNNING`; drop releases it and clears `ACTIVE_CANCEL`.
- `LiveScanState`: atomics for size + count, an `Instant`, an `AppHandle` clone, the cancel `Arc`.
- Rayon parallel walker. Per directory: `Vec<PathBuf>` for subdirs, `BinaryHeap<HeapEntry>` capped at `MAX_FILES_PER_DIR`, `Vec<FolderInfo>` for children. All freed on function return.
- Returned `FolderInfo` tree — moved into the IPC result. Rust holds no reference once `get_user_folders` returns.

What Vue replaces on the new scan:

- `folders` is reassigned in `loadFolders`'s `try` block.
- The prior nav stacks, `selectedMap`, and `current` are all reset via the `watch(props.folders)` in [ScanResultsList.vue:111](../src/components/ScanResultsList.vue#L111) — fires immediately when `folders` is replaced.

### Cancel paths

| Trigger                                                      | UI source                                                                                       | `cancel_scan` | `scanGeneration++` | Wipes Vue state                                 |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------- | ------------- | ------------------ | ----------------------------------------------- |
| User clicks **Abort** during scan                            | `ScanProgress` emit → `ScanView.onAbort` → [useScanner.onAbort](../src/lib/use-scanner.ts#L153) | Yes           | Yes                | Yes (folders=[], progress reset, timer stopped) |
| Window reloads / closes mid-scan                             | [use-scanner.ts:139](../src/lib/use-scanner.ts#L139) `beforeunload` listener                    | Yes           | No                 | No (page is unloading)                          |
| User finishes **trash → confirm** then clicks **Scan again** | [ScanView.onRestart](../src/components/ScanView.vue#L168) → `onCancel()` (= `onAbort`)          | Yes           | Yes                | Yes                                             |
| User leaves Scan view while on TRASH_COMPLETE                | [ScanView.vue:114](../src/components/ScanView.vue#L114) `watch(props.isActive)` → `onCancel()`  | Yes           | Yes                | Yes                                             |

**Two-step cooperative cancel.** A single AtomicBool isn't enough because the walker may emit one more progress event after the flag is read on a different thread. So:

1. **Rust side.** `cancel_scan` ([scan.rs:678](../src-tauri/src/scan.rs#L678)) locks `ACTIVE_CANCEL`, loads the current `Arc<AtomicBool>` (if any), and flips it with `AtomicOrdering::Release`. The walker reads the flag with `AtomicOrdering::Acquire` at entry, every 1000 directory entries, and after the rayon collect, returning early on `true`. The `Release`/`Acquire` pair establishes a happens-before relationship so the flip is visible promptly across rayon worker threads. The `ScanRunningGuard` drops, releases `SCAN_RUNNING`, and clears the token slot.
2. **Vue side.** `onAbort` bumps `scanGeneration` **before** awaiting `invoke('cancel_scan')`. Every async callback captured the previous generation; the comparisons at [use-scanner.ts:101](../src/lib/use-scanner.ts#L101) and [use-scanner.ts:114](../src/lib/use-scanner.ts#L114) drop their results silently.

Either step alone leaves a hole. Just the AtomicBool: late progress events still update `progress` in Vue. Just the generation counter: a Rust walker keeps running on a cancelled scan until it finishes naturally, burning CPU and emitting events that get dropped.

### Natural completion

When `get_user_folders` returns without cancellation:

- `ScanRunningGuard::drop` ([scan.rs:61](../src-tauri/src/scan.rs#L61)) releases `SCAN_RUNNING` and clears `ACTIVE_CANCEL`. The cancel `Arc` drops with the last walker reference.
- The `FolderInfo` tree is serialized to JSON and moved across the IPC. Rust drops its owning value.
- `useScanner.loadFolders`'s `try` block assigns `folders.value = result`. The previous tree (if any) is dropped when the watch in `ScanResultsList` replaces nav state.
- `finally` unlistens the progress handle, clears `isScanning`, stops the elapsed timer.

## Trash lifecycle

Trash starts on Vue and ends on Rust without leaving long-lived state on either side.

```
selectedMap (Vue)  ──►  ScanTrashList (review with checkedMapRef)
                              │
                              ▼
                  invoke('trash_paths', { items })
                              │
                              ▼
        trash.rs::trash_paths  (filter_items → trash::delete loop)
                              │
                              ▼
                  TrashResult { count, size }
```

- Selection lives only in Vue ([ScanResultsList.vue:99](../src/components/ScanResultsList.vue#L99) — `reactive(Map<path, FolderInfo>)`). Rust receives only the paths it needs to delete.
- `filter_items` ([trash.rs:42](../src-tauri/src/trash.rs#L42)) reapplies the protected-path filter against each item's **canonical** path — defense in depth, since `safe_folders.rs` is the single source of truth and the cached tree might be stale. Survivors are wrapped in `FilteredItem` carrying the canonical `PathBuf`, so `trash::delete` operates on the same identity the checks approved (no symlink/rename TOCTOU between filter and delete).
- `trash_paths_sync` ([trash.rs](../src-tauri/src/trash.rs)) surfaces a `Result<TrashResult, String>` so a home-directory canonicalize failure is reported instead of silently zeroing the batch (which would otherwise compare canonical item paths against a non-canonical home and drop everything as "protected").
- `trash_paths_sync_with_home` iterates files then dirs and calls `trash::delete(&item.canonical)` per path. The `trash` crate moves the input vec and retains nothing.
- E2E dry-run: `E2E_TRASH_MODE: Mutex<String>` ([trash.rs:123](../src-tauri/src/trash.rs#L123)) holds `"success"` / `"zero"` / `"error"`. Set by `set_e2e_trash_mode`, reset by [reset_e2e_state](../src-tauri/src/store.rs#L236). The command, its mode store, and the `reset_e2e_state` helper are all behind `#[cfg(feature = "e2e")]`; release and beta workflows ship without `--features e2e`, and [`scripts/verify-no-e2e-symbols.sh`](../scripts/verify-no-e2e-symbols.sh) is run from both workflows to fail the build if any of the e2e command symbols leak into a signed binary.

The Vue `checkedMapRef` in `ScanTrashList` sits inside a `<KeepAlive>` boundary and survives view switches. See [Vue retention pitfalls](#vue-retention-pitfalls).

## Persistent process state (Rust)

Every `static` / `LazyLock` / `Mutex` that outlives a single command. Items not listed here are task-scoped.

| Symbol                      | File:line                                                  | Type                               | Holds                                                    | Mutated by                                                   | Cleared by                   |
| --------------------------- | ---------------------------------------------------------- | ---------------------------------- | -------------------------------------------------------- | ------------------------------------------------------------ | ---------------------------- |
| `SCAN_RUNNING`              | [scan.rs:44](../src-tauri/src/scan.rs#L44)                 | `AtomicBool`                       | One-scan-at-a-time gate                                  | `get_user_folders` acquire; guard drop release               | Guard drop                   |
| `ACTIVE_CANCEL`             | [scan.rs:49](../src-tauri/src/scan.rs#L49)                 | `Mutex<Option<Arc<AtomicBool>>>`   | Per-scan cancel token                                    | `get_user_folders` set; `cancel_scan` flip; guard drop clear | Guard drop                   |
| `STORE_LOCK`                | [store.rs:24](../src-tauri/src/store.rs#L24)               | `Mutex<()>`                        | Serializes settings RMW                                  | `set_settings_with_handle`, `update_setting_with_handle`     | Per-call release             |
| `VALID_SETTING_KEYS`        | [store.rs:146](../src-tauri/src/store.rs#L146)             | `LazyLock<HashSet<&'static str>>`  | Whitelisted setting keys, `Box::leak`'d                  | Init once from `get_default_settings()`                      | Never (intentional; ~8 keys) |
| `PROTECTED_SET`             | [safe_folders.rs:81](../src-tauri/src/safe_folders.rs#L81) | `LazyLock<HashSet<String>>`        | Lowercased protected paths                               | Init once                                                    | Never                        |
| `SKIPPED_LOWERED`           | [safe_folders.rs:86](../src-tauri/src/safe_folders.rs#L86) | `LazyLock<Vec<String>>`            | Lowercased skipped path prefixes                         | Init once                                                    | Never                        |
| `CONTAINER_MANAGER_ATTR`    | [xattr.rs:13](../src-tauri/src/xattr.rs#L13)               | `LazyLock<CString>`                | `"com.apple.containermanager.identifier"` for `getxattr` | Init once                                                    | Never                        |
| `UpdateState.last_checked`  | [updater.rs:50](../src-tauri/src/updater.rs#L50)           | `Mutex<Option<Update>>`            | Cached check result                                      | `check_for_update` set; `download_update` consume            | Consumed by download         |
| `UpdateState.ready_version` | [updater.rs:46](../src-tauri/src/updater.rs#L46)           | `Mutex<Option<String>>`            | Downloaded version, for menu copy                        | `download_update` set; `reset_update_menu` clear             | Restart, reset               |
| `E2E_TRASH_MODE`            | [trash.rs:123](../src-tauri/src/trash.rs#L123)             | `Mutex<String>` (e2e feature only) | Dry-run mode label                                       | `set_e2e_trash_mode`                                         | `reset_e2e_state`            |
| `TEST_HOME`                 | `e2e_fixtures.rs`                                          | `LazyLock<TempDir>` (e2e only)     | Fake home tempdir                                        | Init on first access                                         | App exit (`TempDir::drop`)   |

`tauri_plugin_store` is **not cached in memory** between calls — `store.rs` opens the store via `app.store(SETTINGS_STORE_PATH)` and calls `store.close_resource()` on every read/write. The cost is one file I/O per call; the upside is no stale in-memory copy to invalidate.

`STORE_LOCK` only serializes the read-modify-write paths (`set_settings_with_handle`, `update_setting_with_handle`). Pure reads (`get_settings_with_handle`, `get_setting_with_handle`) do not take the lock — `tauri_plugin_store` opens the file from a single in-process resource map, so a torn JSON read isn't possible from a concurrent RMW (the writer flushes the full object atomically).

## Vue retention pitfalls

### `KeepAlive` keeps state alive across view switches

`ScanLaunch`, `ScanProgress`, `ScanResultsList`, `ScanTrashList`, and `ScanTrashConfirmation` sit inside a `<KeepAlive>` boundary in [ScanView.vue:180](../src/components/ScanView.vue#L180). Component-local refs survive activation toggles. Two cases to know:

- **`checkedMapRef`** in `ScanTrashList` survives until the `items` prop changes. Switching to Settings and back keeps the same check state.
- **`renderedCount`** and **`expandTimer`** in [ScanResultsList.vue:163](../src/components/ScanResultsList.vue#L163) — `cancelExpand` clears the timer in `onDeactivated`. `KeepAlive` skips `onUnmounted`, so timers must use the activation hooks or they leak.

If you add a new ref to a Scan\* component, decide whether `KeepAlive` should preserve it. If not, reset it in `onDeactivated`.

### `shallowRef` over deep reactivity

The scan tree (`folders`) and nav stacks (`backStack`, `forwardStack`, `current`) use `shallowRef` because we **always replace the whole array, never mutate in place**. Deep reactivity on a 1.5M-node tree would walk every node to install proxies, which is wasted work for a payload that's effectively immutable from Vue's perspective.

The selection map is the exception: `reactive(new Map(...))` in [ScanResultsList.vue:99](../src/components/ScanResultsList.vue#L99) because Vue tracks `Map.get(key)` per-key, so toggling one row only re-renders that row.

### The watch that resets everything on a new scan

[ScanResultsList.vue:111](../src/components/ScanResultsList.vue#L111) watches `props.folders` and on every change resets `backStack`, `forwardStack`, `selectedMap`, `current`, and `homePath`. This is the single point where nav and selection are dropped. Any new "carryover across scans" feature has to coexist with this watch or it'll be wiped.

### Listeners and timers

- **`folder-scan-progress`** is owned by `useScanner` via `unlistenProgress`. Always unlistened before a new scan and in the `finally` of `loadFolders`. Do not subscribe to this event from outside `useScanner`.
- **`beforeunload`** is registered in `useScanner.onMounted` and removed in `onUnmounted`. It exists only to fire `cancel_scan` on reload so Rust isn't left with a running walker.
- **`elapsedInterval`** and **`expandTimer`** both have explicit cleanup paths. New timers added to scan components must follow the same pattern.

## Cross-cutting contracts

- **Truncation cap is shared.** `scan::MAX_FILES_PER_DIR` ([scan.rs:38](../src-tauri/src/scan.rs#L38)) and `MAX_DISPLAYED_ITEMS` ([ScanResultsList.vue:60](../src/components/ScanResultsList.vue#L60)) must stay in lockstep. The same `truncated` notice fires from either trigger, so they need to match or the copy lies. The cap applies to **files only**; subfolders are always retained.
- **Cancel needs both halves.** Rust's `AtomicBool` token stops the walker; Vue's `scanGeneration` drops late events. Adding a third async surface (e.g. a new IPC event) means routing it through the same generation check.
- **Boundary objects are `snake_case`.** `FolderInfo`, `ScanProgress`, `TrashListItem` carry Rust field names across the wire unchanged. Frontend-only objects use `camelCase`. See [`architecture.md`](architecture.md).
- **Rust never keeps the scan tree.** `get_user_folders` returns it across IPC and drops its owning value. Anything that needs the tree later must hold it in Vue.

## Module index

| Side  | File                                       | Responsibility                                                |
| ----- | ------------------------------------------ | ------------------------------------------------------------- |
| Rust  | `src-tauri/src/scan.rs`                    | Walker, `FolderInfo`, progress throttling, cancel token, RAII |
| Rust  | `src-tauri/src/trash.rs`                   | `trash_paths`, `filter_items`, e2e dry-run                    |
| Rust  | `src-tauri/src/safe_folders.rs`            | Protected + skipped lists (consumed by scan AND trash)        |
| Rust  | `src-tauri/src/store.rs`                   | Settings persistence, per-call store open/close               |
| Rust  | `src-tauri/src/updater.rs`                 | Updater state (`last_checked`, `ready_version`)               |
| Vue   | `src/lib/use-scanner.ts`                   | Scan lifecycle, progress listener, generation counter         |
| Vue   | `src/components/ScanLaunch.vue`            | Entry point UI                                                |
| Vue   | `src/components/ScanView.vue`              | Local view-state machine, restart wiring                      |
| Vue   | `src/components/ScanResultsList.vue`       | Tree render, back/forward stacks, selection map               |
| Vue   | `src/components/ScanTrashList.vue`         | Review selected items, `checkedMapRef`                        |
| Vue   | `src/components/ScanTrashConfirmation.vue` | Post-delete summary, "Scan again"                             |
| Types | `src/types/structs.ts`                     | `FolderInfo`, `ScanProgress`, `TrashListItem`                 |
| Tests | `src-tauri/tests/scan_test.rs`             | Walker + filter behavior                                      |
| Tests | `src-tauri/tests/trash_test.rs`            | Trash + `filter_items`                                        |
| Tests | `src-tauri/tests/safe_folders_test.rs`     | Protected/skipped correctness                                 |
