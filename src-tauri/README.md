# ApexDisk — Rust Backend

Rust backend for ApexDisk, a macOS desktop app built with Tauri 2 that helps users identify and remove large, unused files.

## Architecture Overview

```
┌──────────────────────────────────────────────────────┐
│                 Tauri Frontend (Vue/TS)               │
└────────────────────────┬─────────────────────────────┘
                         │ IPC Commands
                         ▼
┌──────────────────────────────────────────────────────┐
│         Tauri Command Handlers (lib.rs)               │
├──────────────────────────────────────────────────────┤
│  scan │ trash │ permissions │ store │ updater │ menu │
└────────────────────────┬─────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────┐
│             Core Business Logic Modules               │
├──────────────────────────────────────────────────────┤
│ scan.rs │ trash.rs │ disk.rs │ permissions.rs        │
│ updater.rs │ store.rs │ locale.rs │ safe_folders.rs  │
└────────────────────────┬─────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────┐
│        System Integration (objc2, libc, tauri)        │
└──────────────────────────────────────────────────────┘
```

## Modules

| Module                 | Responsibility                                                                                              |
| ---------------------- | ----------------------------------------------------------------------------------------------------------- |
| `lib.rs`               | App entry point, command registration, shared types (`FolderInfo`, `ScanOptions`)                           |
| `scan.rs`              | Recursive folder tree scanning with Rayon parallelism and live progress events                              |
| `trash.rs`             | Safe file/folder trashing with protection guards via `safe_folders`                                         |
| `disk.rs`              | Disk usage via macOS `NSURL` resource values API, volume naming via `diskutil`                              |
| `store.rs`             | Centralized settings persistence via `tauri-plugin-store`, with key validation and concurrency-safe updates |
| `updater.rs`           | In-app update lifecycle (check, download, install, restart) via `tauri-plugin-updater`                      |
| `permissions.rs`       | Full Disk Access (FDA) detection by probing TCC-protected paths                                             |
| `safe_folders.rs`      | Protected and skipped path lists with pre-computed `LazyLock` sets for fast lookup                          |
| `locale.rs`            | System locale detection and macOS `NSUserDefaults` language preference                                      |
| `menu.rs`              | Native application menu construction with Tauri Menu API                                                    |
| `menu_translations.rs` | Menu label translations for 10+ languages                                                                   |
| `native_dialog.rs`     | macOS `NSAlert` wrapper dispatched on the main thread                                                       |
| `system_info.rs`       | macOS system info collection (parallelized subprocess calls)                                                |
| `xattr.rs`             | Extended attribute detection for container-managed apps                                                     |
| `constants.rs`         | Shared constants (app name, version, URLs, menu IDs)                                                        |
| `log.rs`               | Debug logging (`APEX_DISK_DEBUG=1`)                                                                         |
| `e2e_fixtures.rs`      | E2E test fixtures (feature-gated behind `e2e`)                                                              |

## Key Data Types

```rust
// Passed from the frontend to configure scan behavior
struct ScanOptions {
    show_hidden_files: bool,  // default: false
    show_under_1kb: bool,     // default: false
    show_zero_byte: bool,     // default: false
}

// Recursive tree returned to the frontend
struct FolderInfo {
    name: String,
    path: String,
    size: u64,
    icon: Option<String>,
    children: Vec<FolderInfo>,
    is_file: bool,
    is_protected: bool,      // matches PROTECTED_RELATIVE_PATHS
    is_fda_required: bool,   // container-managed, needs Full Disk Access
    last_modified: Option<i64>,
}
```

## Data Flows

### Scan

1. Frontend calls `get_user_folders(options)`
2. Backend acquires `SCAN_RUNNING` guard (only one scan at a time)
3. Top-level home directories enumerated, filtered (symlinks, hidden, skipped)
4. `build_folder_tree` recurses in parallel via Rayon work-stealing
5. Per-directory: retains top 100 largest files (min-heap), accumulates all sizes
6. `LiveScanState` emits throttled `folder-scan-progress` events (every 150ms)
7. Returns `Vec<FolderInfo>` tree sorted by size descending

### Trash

1. Frontend calls `trash_paths(items)`
2. `filter_items` canonicalizes paths, removes protected/skipped entries
3. Files trashed first, then directories, via macOS `trash` crate
4. Returns `TrashResult { count, size }` of successfully trashed items

### Update

1. Frontend calls `check_for_updates_silent()` on app start
2. If update found, frontend calls `download_update()` to stage it
3. Menu item text changes to "Restart to Update (vX.Y.Z)"
4. User clicks restart button or menu item -> `restart_app()` applies update

## Safety Mechanisms

- **Protected folders**: `PROTECTED_RELATIVE_PATHS` (24 entries) prevents trashing structural macOS folders (Desktop, Library, cloud sync roots)
- **Skipped folders**: `SKIPPED_RELATIVE_PATHS` (13 entries) completely excludes sensitive credential paths from scanning (.ssh, .gnupg, Library/Keychains)
- **Path canonicalization**: `trash.rs` resolves symlinks before checking protection
- **Concurrency guards**: `SCAN_RUNNING` atomic prevents concurrent scans; `STORE_LOCK` mutex prevents settings update races
- **Mutex poison resilience**: All `Mutex::lock()` calls use `.unwrap_or_else(|e| e.into_inner())`
- **Setting key validation**: `update_setting` rejects keys not in `get_default_settings()`

## Dependencies

| Crate                                          | Purpose                                            |
| ---------------------------------------------- | -------------------------------------------------- |
| `tauri` v2                                     | App framework                                      |
| `tauri-plugin-store` v2                        | Persistent settings (JSON file)                    |
| `tauri-plugin-updater` v2                      | GitHub releases update checking                    |
| `tauri-plugin-opener` v2                       | URL/file opening                                   |
| `tauri-plugin-os` v2                           | OS info                                            |
| `rayon` v1.10                                  | Parallel directory scanning                        |
| `trash` v5                                     | macOS Trash API                                    |
| `objc2` / `objc2-foundation` / `objc2-app-kit` | Native macOS APIs (NSAlert, NSUserDefaults, NSURL) |
| `dirs` v5                                      | Home directory resolution                          |
| `libc` v0.2                                    | `getxattr` for extended attributes                 |
| `nix` v0.29                                    | POSIX filesystem operations                        |
| `sys-locale` v0.1                              | System locale detection                            |
| `serde` / `serde_json`                         | Serialization                                      |
| `tokio` v1                                     | Async runtime (`spawn_blocking`)                   |

## Features

| Feature   | Effect                                                                                                                                              |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default` | Production build (no extras)                                                                                                                        |
| `e2e`     | Enables `tauri-plugin-webdriver` + `tempfile` for end-to-end tests. Adds mock commands (`set_e2e_trash_mode`, `reset_e2e_state`) and test fixtures. |

## Testing

Tests live in `src-tauri/tests/` with shared fixtures in `tests/support/mod.rs`.

```sh
# Run all tests
cargo test

# Run a specific test module
cargo test --test scan_test
cargo test --test store_test
```

| Test File                             | Coverage                                                                        |
| ------------------------------------- | ------------------------------------------------------------------------------- |
| `scan_test.rs` (14 tests)             | Folder shape, protected/skipped, all ScanOptions, sorting, sizes, last_modified |
| `safe_folders_test.rs` (14 tests)     | Exact match, descendants, case-insensitivity, outside-home                      |
| `trash_test.rs` (9 tests)             | Filter partitioning, protected/skipped, actual trashing                         |
| `store_test.rs` (8 tests)             | Defaults, init, merge, round-trip, key validation                               |
| `disk_test.rs` (6 tests)              | Volume name parsing                                                             |
| `xattr_test.rs` (5 tests)             | Temp dir, nonexistent path, real containers                                     |
| `menu_translations_test.rs` (3 tests) | All languages, English fallback                                                 |
| `system_info_test.rs` (2 tests)       | Structure, idempotency                                                          |

## Adding a New Command

1. Create the `#[tauri::command]` function in the appropriate module
2. Register it in **both** `generate_handler!` blocks in `lib.rs` (the `e2e` and `not(e2e)` branches must stay in sync since `generate_handler!` is a proc macro that cannot accept macro invocations)
3. If it's e2e-only, add it only to the `#[cfg(feature = "e2e")]` block

## Adding a New Setting

1. Add the key and default value to `get_default_settings()` in `store.rs`
2. The key will automatically be allowed by `is_valid_setting_key()` validation
3. Missing keys are backfilled when `get_settings` merges with defaults

## Environment Variables

| Variable                      | Effect                                                                                                                                                                                                 |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `APEX_DISK_DEBUG=1` or `true` | **Release:** enables intentional `log()` lines on stdout, updater diagnostics on stdout, and mirrored **stderr** lines for uncaught web/Vue errors. **Dev:** same, except updater stdout also uses debug builds without the env var. |
| Launch from Terminal          | Use `APEX_DISK_DEBUG=1 /Applications/ApexDisk.app/Contents/MacOS/ApexDisk`, or **`./scripts/run-with-debug.sh`**. `open -a` does **not** pass this env var.                                            |

Full Vue/Rust logging scheme (categories, `[apex:…]` prefixes, stdout vs stderr, source files): **`docs/LOGGING.md`**.
