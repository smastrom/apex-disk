# Architecture

ApexDisk is a **macOS-only Tauri 2 desktop app**. Two processes, one bundle:

```
┌───────────────────────────────────────────────────────────────┐
│                    macOS .app bundle                          │
│                                                               │
│  ┌──────────────────────┐       ┌──────────────────────────┐  │
│  │   Rust (native)      │       │   Webview (WKWebView)    │  │
│  │   src-tauri/src/     │ ◀──▶  │   Vue 3 + TS + Vite      │  │
│  │                      │       │   src/                   │  │
│  │  - filesystem I/O    │       │  - UI rendering          │  │
│  │  - disk APIs         │       │  - navigation state      │  │
│  │  - macOS bindings    │       │  - translations          │  │
│  │  - native menu       │       │  - themes                │  │
│  │  - persistent store  │       │  - selection / flow      │  │
│  │  - updater           │       │                          │  │
│  └──────────────────────┘       └──────────────────────────┘  │
│           │                               │                   │
│           └──── Tauri IPC (JSON over ─────┘                   │
│                 WKWebView message handler)                    │
└───────────────────────────────────────────────────────────────┘
```

The boundary is the only real design axis in this codebase: every feature has a **Rust side** (anything touching the OS, the filesystem, or persistent state) and a **webview side** (anything you can see or click). Everything else — themes, animations, translations — is pure frontend with Rust as a passive store.

For related docs: see [`UPDATES.md`](UPDATES.md) (in-app updater behavior), [`RELEASES.md`](RELEASES.md) (how builds ship), [`LOGGING.md`](LOGGING.md) (diagnostics), [`COMPATIBILITY.md`](COMPATIBILITY.md) (macOS / Safari targets).

## What each side owns

### Rust (`src-tauri/src/`) owns

| Responsibility | Module |
| --- | --- |
| Home-folder scan (walk, size, filter protected/skipped paths, emit progress, support cancel) | `scan.rs` |
| Move-to-Trash (with protected-path filter applied again before delete) | `trash.rs` |
| Persistent settings (read/write/merge with defaults, concurrent-write locking) | `store.rs` |
| Protected + skipped folder lists (single source of truth, consumed by scan *and* trash) | `safe_folders.rs` |
| Disk volume usage (total / free / used for the home volume) | `disk.rs` |
| System info (OS version, arch, memory) | `system_info.rs` |
| Full Disk Access (FDA) TCC check | `permissions.rs` |
| macOS locale detection + `AppleLanguages` sync via objc2 | `locale.rs` |
| Native menu build + per-language rebuild | `menu.rs`, `menu_translations.rs` |
| Native message / ask dialogs | `native_dialog.rs` |
| Update check / download / restart, plus menu-text updates | `updater.rs` |
| Diagnostic logging (Rust-side `dev_rust_trace`, IPC logging command) | `log.rs` |
| macOS extended-attribute helpers | `xattr.rs` |
| App-wide constants (menu ids, URLs, default language/theme) | `constants.rs` |
| E2E-only fixtures (trash dry-run, state reset) | `e2e_fixtures.rs` |

Entry points: `main.rs` delegates to `lib.rs::run()`, which installs plugins (`tauri_plugin_store`, `tauri_plugin_updater`, `tauri_plugin_os`, `tauri_plugin_opener`; `tauri_plugin_webdriver` under the `e2e` feature), registers every command in `tauri::generate_handler!`, and runs the setup hook (store defaults, locale bootstrap, menu build).

**Rust never renders UI.** The native menu is the only OS-visible surface it controls directly.

### Frontend (`src/`) owns

| Responsibility | Location |
| --- | --- |
| Vue component tree + routing between views (scan / settings / information) | `src/components/`, `src/lib/use-app-views.ts` |
| Scan flow UI (launch, progress, results list, trash review, confirmation) | `src/components/Scan*.vue` |
| Browser-style **back/forward navigation stacks** inside a folder tree | `src/components/ScanResultsList.vue` (state lives in the component) |
| Selection state (which rows are checked for trash) | Scan result components, not a global store |
| Settings **UI** + typed access (hidden-files toggle, theme, language, auto-updates) | `src/stores/app-settings.ts`, `src/components/SettingsView.vue` |
| Theme application — `data-theme` attribute on `<html>`, CSS variables | `src/assets/css/theme.css`, `src/stores/app-settings.ts` |
| Translations — per-component `.ts` files, reactive via `useTranslations()` | `src/assets/translations/`, `src/lib/use-translations.ts` |
| All presentational primitives (buttons, popovers, spinners, icons) | `src/components/ui/` |
| Animations, view transitions, reduced-motion handling | `src/assets/css/animations.css`, `src/lib/use-view-transition.ts`, `src/lib/use-reduced-motion.ts` |
| Vue-side diagnostic log (forwards to Rust when `APEX_DISK_DEBUG` is on) | `src/lib/log.ts` |

**Frontend owns no filesystem logic.** If a feature needs to know *what's on disk* (including whether a path is protected), it asks Rust. The "protected" flag on a `FolderInfo` is filled server-side so the UI never re-evaluates it.

## How the two sides communicate

Three channels, all provided by Tauri:

### 1. `invoke()` — request/response

Webview → Rust, returns a `Promise`. Used for everything transactional.

```
src/lib/use-scanner.ts
   └─ await invoke('get_user_folders', { options })
       └─ src-tauri/src/scan.rs::get_user_folders()
```

Commands are registered in `src-tauri/src/lib.rs` inside `tauri::generate_handler!`. Two handler blocks — one for the default build, one behind `#[cfg(feature = "e2e")]` that also exposes `trash::set_e2e_trash_mode` and `store::reset_e2e_state`. **Both blocks must be updated when adding a new command** (the macro can't expand a nested macro, hence the duplication — noted inline in `lib.rs`).

Current command surface, grouped:

- **Disk / scan** — `get_disk_usage`, `get_user_folders`, `cancel_scan`
- **Trash** — `trash_paths` (+ `set_e2e_trash_mode` under `e2e`)
- **Permissions** — `check_full_disk_access`
- **Native dialogs** — `show_message_dialog`, `show_ask_dialog`
- **Locale + menu** — `set_app_locale`, `get_system_language`, `resolve_app_language`, `set_menu_language`
- **Settings store** — `get_settings`, `set_settings`, `get_setting`, `update_setting` (+ `reset_e2e_state` under `e2e`)
- **System** — `get_system_info`
- **Logging** — `is_debug_mode`, `log_message`, `log_error_message`
- **Updater** — `check_for_updates_silent`, `check_for_updates_dialog`, `download_update`, `restart_app`, `set_update_menu_ready`, `reset_update_menu`

### 2. Events — Rust → webview push

One direction, many subscribers. Used when Rust needs to stream progress without blocking.

Currently emitted:

- **`folder-scan-progress`** — emitted from `scan.rs` during a walk. Payload shape is `ScanProgress { current, total, folder, size, scanned_size_total, completed_size }`. The scanner throttles emissions to ~150 ms via `LiveScanState::add_size_and_maybe_emit` so the IPC channel isn't flooded during deep walks. Consumer: `src/lib/use-scanner.ts`, which listens with `listen('folder-scan-progress', …)` and exposes a reactive progress ref.

No other events today — trash and updater are fully request/response.

### 3. Persistent state — the settings store

`tauri_plugin_store` writes a JSON file inside the app support directory. Both sides read it, but **only Rust writes it**. The webview always mutates via `set_settings` / `update_setting`; `store.rs` takes an internal `STORE_LOCK` mutex so concurrent writes don't lose updates, and it re-merges defaults on every read so new fields added in a Rust upgrade don't fail deserialization.

Adding a new setting is a one-line change: register the key and its default in `get_default_settings()` inside `store.rs`. `is_valid_setting_key()` accepts it automatically, and existing installs get the field backfilled on next `get_settings`.

Frontend mirrors the store in a reactive module-level singleton: `src/stores/app-settings.ts` calls `initTauriAppSettings()` once from `main.ts`, then `useAppSettings()` returns refs. **No `provide` / `inject`** — an explicit init + assertion caught "used before ready" bugs early; we kept it.

## Boundary conventions

A handful of rules that make the IPC hop easy to reason about:

- **Names cross the wire unchanged.** Rust structs serialize field names as-is; the frontend consumes them verbatim. That means **Tauri-boundary objects use `snake_case`** even in TypeScript (e.g. `folder_info.is_protected`), while **frontend-only objects use `camelCase`**. Types for boundary shapes live in `src/types/` and mirror the Rust structs.
- **Do not reimplement Rust logic in Vue.** The "is this path protected?" / "how big is this folder?" / "is FDA granted?" questions belong to Rust — even when the answer is cached in a ref.
- **Commands are total.** A command either resolves with data or rejects with an error string; no partial states on the wire. Progress-style feedback goes on events, not on the invoke promise.
- **Cancellation is cooperative.** `cancel_scan` flips an atomic flag; `scan.rs` checks it inside the walk and exits early. The frontend `useScanner` also carries a `scanGeneration` counter so stale event payloads (from a scan the user already cancelled) are dropped in the webview.
- **macOS-only.** There's no Windows/Linux branching in either side — see [`COMPATIBILITY.md`](COMPATIBILITY.md). objc2 AppKit/Foundation bindings are used freely in Rust.

## Feature walkthroughs (boundary view)

### Scan

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

Cancellation: `cancel_scan` → atomic flag read by the walker → early return. The UI drops any in-flight progress via its generation counter.

### Trash

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

### Settings

```
UI toggle (SettingsView.vue)
     │
     ▼
useAppSettings().setFoo(value)              src/stores/app-settings.ts
  ├─ updates reactive ref optimistically
  └─ invoke('update_setting', { key, value })
           │
           ▼
store.rs::update_setting                    src-tauri/src/store.rs
  ├─ STORE_LOCK.lock()
  ├─ read → merge defaults → mutate → write
  └─ returns the full merged settings object
```

Theme changes additionally write `data-theme` on `document.documentElement`; language changes additionally invoke `set_menu_language` so the native menu rebuilds immediately.

### Locale + menu

`locale.rs` detects the user's preferred language at first run (macOS `AppleLanguages`). `menu.rs` builds the native menu once during setup and rebuilds it whenever the webview calls `set_menu_language`, pulling strings from `menu_translations.rs::labels_for(lang)`. The webview never constructs menu items.

### Updater

Boundary details live in [`UPDATES.md`](UPDATES.md). In terms of split:

- Rust (`updater.rs`) owns the `tauri-plugin-updater` calls, signature verification, and menu-label state machine ("Checking…" / "Downloading…" / "Restart to Update").
- Frontend (`src/lib/use-app-update.ts`) owns the silent-check-on-start gate (auto-updates only, skipped in dev), the reactive `updateReady` flag, and the Settings UI button.

### Logging / diagnostics

Walkthrough in [`LOGGING.md`](LOGGING.md). At the boundary:

- `src/lib/log.ts` prints `[apex:vue:<channel>]` in the webview console and, when `APEX_DISK_DEBUG` is set, forwards lines to Rust via `log_message` so both sides land in the same stdout stream.
- `log.rs::dev_rust_trace(channel, msg)` does the Rust-side equivalent with `[apex:rust:<channel>]`.

## Build + testing boundary

- **Frontend build:** `pnpm build` → `vue-tsc --noEmit && vite build`. Vite pipes CSS through **lightningcss** with a Safari 13 target so modern CSS downlevels to flat syntax (matches the declared minimum macOS; see [`COMPATIBILITY.md`](COMPATIBILITY.md)).
- **App build:** `pnpm tauri:build` bundles Rust + Vite output into a universal-binary `.app` / `.dmg`. `pnpm tauri:build:beta` layers `tauri.beta.conf.json` as a merge config (different bundle id / product name, `createUpdaterArtifacts: false`).
- **Unit / integration tests:** `pnpm test:unit` runs `cargo test` inside `src-tauri/` with `--test-threads=1` (serial — some tests mutate process-global state). Integration tests live in `src-tauri/tests/` (`scan_test.rs`, `trash_test.rs`, `store_test.rs`, `safe_folders_test.rs`, etc.); shared helpers in `tests/support/mod.rs`.
- **E2E:** `pnpm test:e2e` drives the app with WebdriverIO against a build with the `e2e` cargo feature enabled (which exposes `tauri_plugin_webdriver` and the `set_e2e_trash_mode` / `reset_e2e_state` commands). Specs in `e2e/specs/`.

Release workflow and Beta channel: see [`RELEASES.md`](RELEASES.md).

## Directory map

```
src/                           # Webview
  components/
    ui/                        # Presentational primitives (buttons, popovers, spinners, icons)
    Scan*.vue                  # Scan flow (Launch, Progress, ResultsList, TrashList, TrashConfirmation)
    SettingsView.vue, ...      # Top-level views
  lib/                         # Composables + utilities (use-* convention)
  stores/
    app-settings.ts            # Reactive mirror of the Rust settings store
  assets/
    css/                       # theme.css, classes.css, animations.css, global.css, reset.css, rtl.css
    translations/              # Per-component .ts files + index factory
  types/                       # Boundary types mirroring Rust structs (snake_case fields)

src-tauri/
  src/                         # Native side (see table above)
  tests/                       # Integration tests + support/
  tauri.conf.json              # Base Tauri config
  tauri.beta.conf.json         # Beta merge config

e2e/                           # WebdriverIO specs + helpers
docs/                          # Agent-facing docs (this file, COMPATIBILITY, LOGGING, UPDATES, RELEASES)
```
