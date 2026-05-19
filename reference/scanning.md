# Scanning + Trash

The scan/trash subsystem is the core feature loop: walk the home folder, return a folder tree to the webview, let the user select rows, move selected paths to the macOS Trash. The Rust side does all filesystem work; the frontend just navigates and selects.

## Scan flow

```
[click Scan]  ScanLaunch.vue
     │
     ▼
useScanner()                         src/lib/use-scanner.ts
  ├─ listen('folder-scan-progress')  ◀─┐
  └─ invoke('get_user_folders', …)     │ progress events
                                       │ (throttled ~150 ms)
            ┌──────────────────────────┘
            ▼
scan.rs::get_user_folders                  src-tauri/src/scan.rs
  ├─ rayon parallel walk of $HOME/*
  ├─ safe_folders::is_path_protected / is_path_skipped
  ├─ LiveScanState::add_size_and_maybe_emit  → emit
  └─ returns FolderInfo tree
            │
            ▼
 ScanResultsList.vue — renders tree, manages back/forward stack
```

### `FolderInfo` tree

`scan.rs` builds a `FolderInfo` tree rooted at `$HOME`. Each node carries:

- `name`, `path`, `size`
- `is_file`, `is_protected`, `is_fda_required`
- `last_modified`
- `truncated` (see [Per-folder file cap](#per-folder-file-cap) below)
- nested children (for directories)

The **`is_protected` flag is filled server-side** — the UI must not re-evaluate "is this path protected?" because that knowledge belongs to `safe_folders.rs` (single source of truth, also consumed by `trash.rs`).

Fields cross the wire `snake_case` (see [`architecture.md`](architecture.md) — boundary conventions). The matching TypeScript type lives in `src/types/structs.ts`.

### Per-folder file cap

`scan::MAX_FILES_PER_DIR` (currently **300**) bounds the number of file entries retained per directory. The walker keeps the **N largest by size** via a min-heap and counts every other file's size into the folder total — so sizes stay accurate even when entries are dropped.

The constant has a UI-visible contract:

- The cap applies to **files only**. Subfolders are always retained.
- When at least one file is dropped, the folder's `truncated` field is set to `true`. The frontend reads this to show a notice at the end of the list.
- The frontend caps **rendered rows per view** at the same number (`MAX_DISPLAYED_ITEMS` in `ScanResultsList.vue`). This bounds DOM size without a row virtualizer; the dropped tail is always the smallest entries because children arrive sorted by size desc. When the frontend slice fires (items > cap), the notice also shows.
- Either trigger uses the same copy (`ScanResultsList.yaml` → `truncated`). The user doesn't need to distinguish between "Rust dropped files" and "UI hid small entries"; either way they're not seeing everything.

**If you bump the cap, change both sides together** — the Rust constant and the frontend `MAX_DISPLAYED_ITEMS`. They have to stay in sync, otherwise the notice copy ("Only the N largest entries are shown") lies in one direction.

### Progress events: `folder-scan-progress`

Emitted by `scan.rs` during the walk. Payload shape:

```ts
ScanProgress {
   current: number
   total: number
   folder: string
   size: number
   scanned_size_total: number
   completed_size: number
}
```

The scanner throttles emissions to **~150 ms** via `LiveScanState::add_size_and_maybe_emit` so the IPC channel isn't flooded during deep walks. The consumer is `src/lib/use-scanner.ts`, which listens via `listen('folder-scan-progress', …)` and exposes a reactive `progress` ref.

This is currently the only event in the app — trash and updater are fully request/response.

### Cancellation

Cooperative two-step:

1. **Atomic flag** — `cancel_scan` invoke flips an atomic; `scan.rs` checks it inside the walk and exits early.
2. **Generation counter** — `useScanner` carries a `scanGeneration` ref. Every `loadFolders` / abort bumps it. Async callbacks capture the value at start and compare on resolve; mismatched ones are silently discarded. This drops stale event payloads from a scan the user already cancelled.

Both are needed because the Rust walker may emit one more progress event after the cancel flag is read on a different thread.

## Trash flow

```
[select rows → Trash → confirm]
  ScanResultsList → ScanTrashList → ScanTrashConfirmation
     │
     ▼
 invoke('trash_paths', { items })
     │
     ▼
trash.rs::trash_paths                       src-tauri/src/trash.rs
  ├─ filter_items() — drops protected/skipped paths again
  ├─ trash::delete() per survivor (macOS Trash, recoverable)
  └─ returns TrashResult { count, size }
     │
     ▼
 UI shows summary; user can "Scan again" to re-read disk state
```

No frontend-side trash store: the selection list is rebuilt from the current scan tree on demand.

**Protected-path filter runs twice** — once in `scan.rs` (so the UI never marks them selectable), once in `trash.rs::filter_items` before delete (defense in depth, since `safe_folders.rs` is the single source of truth and might change between scan and trash).

The trash uses macOS Trash (recoverable) via the `trash` crate, not `unlink(2)`. Under the `e2e` cargo feature, `trash::set_e2e_trash_mode` swaps in a dry-run mode that records intent without touching disk — used by WebdriverIO specs.

## Selection state

Selection lives in `ScanResultsList.vue` as a `reactive(Map<path, FolderInfo>)`, **not** a Set. Reasoning:

- Vue tracks `Map.get(key)` per-key (not `ITERATE_KEY`), so toggling one item only re-renders that row, not the entire list.
- Storing the full `FolderInfo` (not just the path) means the trash list doesn't have to re-walk the tree to recover metadata.

Selection state survives navigation between folders. The back/forward stacks track the items being shown, while `selectedMap` remains independent so selected rows keep their state as the user drills in and returns.

## Browser-style navigation

`ScanResultsList.vue` maintains two `shallowRef<NavEntry[]>` stacks (`backStack`, `forwardStack`) plus a `current` entry. `shallowRef` is intentional: we always replace the whole array, never mutate in place, so deep reactivity would be wasted work.

Each `NavEntry` is `{ items: FolderInfo[], label: string, path: string }`. Drilling into a folder pushes the previous `current` onto `backStack` and clears `forwardStack`; back/forward navigation moves entries between stacks without invoking Rust.

## Module index

| Side  | File                                       | Responsibility                                         |
| ----- | ------------------------------------------ | ------------------------------------------------------ |
| Rust  | `src-tauri/src/scan.rs`                    | Walker, FolderInfo, progress throttling, cancel flag   |
| Rust  | `src-tauri/src/trash.rs`                   | `trash_paths`, filter_items, e2e dry-run mode          |
| Rust  | `src-tauri/src/safe_folders.rs`            | Protected + skipped lists (consumed by scan AND trash) |
| Vue   | `src/lib/use-scanner.ts`                   | Scan lifecycle, progress listener, generation counter  |
| Vue   | `src/components/ScanLaunch.vue`            | Entry point UI                                         |
| Vue   | `src/components/ScanResultsList.vue`       | Tree render, back/forward stacks, selection map        |
| Vue   | `src/components/ScanResultsListItem.vue`   | Row, selection circle, press state                     |
| Vue   | `src/components/ScanTrashList.vue`         | Review selected items before trash                     |
| Vue   | `src/components/ScanTrashConfirmation.vue` | Post-delete summary, "Scan again"                      |
| Types | `src/types/structs.ts`                     | `FolderInfo`, `ScanProgress`, `TrashListItem`          |
| Tests | `src-tauri/tests/scan_test.rs`             | Walker + filter behavior                               |
| Tests | `src-tauri/tests/trash_test.rs`            | Trash + filter_items                                   |
| Tests | `src-tauri/tests/safe_folders_test.rs`     | Protected/skipped list correctness                     |
