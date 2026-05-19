# Tauri Commands + IPC

How the Rust ↔ webview boundary actually works: the three channels, the full command surface, registration patterns in `lib.rs`, and the settings flow. See [`architecture.md`](architecture.md) for the higher-level "what each side owns" picture; see [`scanning.md`](scanning.md) for the scan/trash flow specifically.

## Three channels

Tauri provides three communication primitives. The codebase uses all three:

### 1. `invoke()` — request/response

Webview → Rust, returns a `Promise`. Used for everything transactional.

```
src/lib/use-scanner.ts
   └─ await invoke('get_user_folders', { options })
       └─ src-tauri/src/scan.rs::get_user_folders()
```

**Rule:** commands are **total**. A command either resolves with data or rejects with an error string; no partial states on the wire. Streaming-style feedback goes on **events**, not on the invoke promise.

### 2. Events — Rust → webview push

One direction, many subscribers. Used when Rust needs to stream progress without blocking.

Currently the only event is `folder-scan-progress` (see [`scanning.md`](scanning.md)). Trash and updater are fully request/response.

### 3. Persistent state — the settings store

`tauri_plugin_store` writes a JSON file inside the app support directory. Both sides read it, but **only Rust writes it**. The webview always mutates via `set_settings` / `update_setting` (see [Settings flow](#settings-flow) below).

## Command surface

All commands registered in `src-tauri/src/lib.rs`, grouped by area:

| Area              | Commands                                                                                                                               |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| **Disk / scan**   | `get_disk_usage`, `get_user_folders`, `cancel_scan`                                                                                    |
| **Trash**         | `trash_paths` (+ `set_e2e_trash_mode` under `e2e`)                                                                                     |
| **Permissions**   | `check_full_disk_access`                                                                                                               |
| **Native dialog** | `show_message_dialog`, `show_ask_dialog`                                                                                               |
| **Locale + menu** | `set_app_locale`, `get_system_language`, `resolve_app_language`, `set_menu_language`                                                   |
| **Settings**      | `get_settings`, `set_settings`, `get_setting`, `update_setting` (+ `reset_e2e_state` under `e2e`)                                      |
| **System**        | `get_system_info`                                                                                                                      |
| **Logging**       | `is_debug_mode`, `log_message`, `log_error_message`                                                                                    |
| **Updater**       | `check_for_updates_silent`, `check_for_updates_dialog`, `download_update`, `restart_app`, `set_update_menu_ready`, `reset_update_menu` |

## Registration in `lib.rs`

Two `tauri::generate_handler!` blocks live side-by-side — one for the default build, one behind `#[cfg(feature = "e2e")]` that additionally exposes `trash::set_e2e_trash_mode` and `store::reset_e2e_state`.

**Both blocks must be updated when adding a new command.** The macro can't expand a nested macro, hence the duplication — noted inline in `lib.rs`.

The setup hook also runs at startup:

1. Initialize `tauri_plugin_store` defaults (`store.rs::get_default_settings()`)
2. Bootstrap locale via `locale.rs` (`AppleLanguages` detection)
3. Build the native menu (`menu.rs`, `menu_translations.rs`)

Plugins installed: `tauri_plugin_store`, `tauri_plugin_updater`, `tauri_plugin_opener`. Under the `e2e` feature, `tauri_plugin_webdriver` is added.

## Settings flow

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

`store.rs` takes an internal `STORE_LOCK` mutex so concurrent writes don't lose updates, and it **re-merges defaults on every read** so new fields added in a Rust upgrade don't fail deserialization for existing installs. `set_settings` (full-object write) runs under the same lock and is whitelisted: unknown keys are dropped silently, and keys not present in the payload are preserved from the current persisted state, so a partial write can't wipe other settings.

**Adding a new setting is a one-line change:** register the key and its default in `get_default_settings()` inside `store.rs`. The cached `VALID_SETTING_KEYS` set picks it up on next process start, both `update_setting` and `set_settings` accept it automatically, and existing installs get the field backfilled on next `get_settings`.

The frontend mirrors the store in a reactive module-level singleton: `src/stores/app-settings.ts` calls `initTauriAppSettings()` once from `main.ts`, then `useAppSettings()` returns refs. **No `provide` / `inject`** — an explicit init + assertion caught "used before ready" bugs early; we kept it.

Side-effects of specific setting changes:

- **Theme** — also writes `data-theme` on `document.documentElement` (see [`themes.md`](themes.md)).
- **Language** — also invokes `set_menu_language` so the native menu rebuilds immediately.
- **Auto-updates** — read by `updater.rs` on each silent check (see [`updates.md`](updates.md)).

## Locale + native menu

`locale.rs` detects the user's preferred language at first run via macOS `AppleLanguages`. `menu.rs` builds the native menu once during setup and rebuilds it whenever the webview calls `set_menu_language`, pulling strings from `menu_translations.rs::labels_for(lang)`.

**The webview never constructs menu items.** Menu structure (file/edit/view/help) is defined entirely in Rust; the webview only triggers rebuilds when the user changes language in Settings.

## Module index

| File                                 | Role                                                                       |
| ------------------------------------ | -------------------------------------------------------------------------- |
| `src-tauri/src/lib.rs`               | Plugin install, command registration (default + e2e blocks), setup hook    |
| `src-tauri/src/store.rs`             | Settings store, defaults, STORE_LOCK, merge-on-read                        |
| `src-tauri/src/locale.rs`            | `AppleLanguages` detection, system language resolution                     |
| `src-tauri/src/menu.rs`              | Native menu build + rebuild                                                |
| `src-tauri/src/menu_translations.rs` | `labels_for(lang)` — strings for every menu item, including updater states |
| `src-tauri/src/native_dialog.rs`     | `show_message_dialog`, `show_ask_dialog` — NSAlert wrappers                |
| `src-tauri/src/permissions.rs`       | `check_full_disk_access` — runtime macOS-version-aware FDA probe           |
| `src-tauri/src/system_info.rs`       | `get_system_info` — OS version, arch, memory                               |
| `src-tauri/src/disk.rs`              | `get_disk_usage` — total/free/used for home volume                         |
| `src/stores/app-settings.ts`         | Frontend reactive mirror of the settings store                             |
| `src/types/structs.ts`               | Boundary types mirroring Rust structs (snake_case fields)                  |
| `src-tauri/tests/store_test.rs`      | Store concurrency, default merging, key validation                         |
