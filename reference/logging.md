# Logging (Vue + Rust)

Single diagnostic scheme for maintainers and bug reports: **`[apex:…]`**-prefixed lines, UTC **`HH:MM:SS.mmm`** timestamps on intentional logs, and clear separation of **stdout** vs **stderr**.

## When output appears

| Build                | Intentional `log()` / Rust diagnostics      | WebView `console` for `log()`                             |
| -------------------- | ------------------------------------------- | --------------------------------------------------------- |
| **`pnpm tauri dev`** | On                                          | Yes (`console.log` / `console.error` for uncaught errors) |
| **Release**          | Only if **`APEX_DISK_DEBUG=1`** (or `true`) | No (DevTools not exposed to typical users)                |

Bug reports: run the app from Terminal — see **`.github/ISSUE_TEMPLATE/bug_report.md`** and **`scripts/run-with-debug.sh`**. **`open -a`** does not pass custom environment variables on macOS.

## Vue layer (`src/lib/log.ts`)

| Export                                | Role                                                                                    |
| ------------------------------------- | --------------------------------------------------------------------------------------- |
| **`initLog()`**                       | Must run early in **`main.ts`**. Prod: asks Rust `is_debug_mode` for `APEX_DISK_DEBUG`. |
| **`registerDiagnosticHandlers(app)`** | Registers `window` error, `unhandledrejection`, and Vue **`errorHandler`**.             |
| **`log(category, message, data?)`**   | Structured app logging; mirrored to Rust via **`log_message`**.                         |

### Categories (`LogCategory`)

Use **one** category per call. Names align with **Rust `dev_rust_trace` channels** where the feature is the same (e.g. `scan`, `trash`, `app`).

| Category       | Use for                                                                    | Typical call sites                                               |
| -------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **`app`**      | Process / startup, in-app updates                                          | `main.ts`, `use-app-update.ts`                                   |
| **`disk`**     | Home volume usage (total / free / used), refetch on window focus           | `use-disk-usage.ts`                                              |
| **`view`**     | High-level scan shell / screen (`ScanView`, confirmations), app shell view | `ScanView.vue`, `ScanTrashConfirmation.vue`, `use-app-views.ts`  |
| **`nav`**      | Folder tree navigation inside results                                      | `ScanResultsList.vue`                                            |
| **`file`**     | Row selection / reset in results list                                      | `ScanResultsList.vue`                                            |
| **`scan`**     | Scan lifecycle (start, cancel, errors)                                     | `use-scanner.ts`                                                 |
| **`trash`**    | Delete review list, move to Trash, completion, post-delete scan again      | `ScanTrashList.vue`, `ScanView.vue`, `ScanTrashConfirmation.vue` |
| **`settings`** | Persisted settings toggles                                                 | `stores/app-settings.ts`                                         |

**Message style:** short phrases with a **domain prefix** when the same category spans multiple screens: **`App:`** (startup / shell), **`Disk:`** (volume usage), **`Scan:`** (scanner + scan `ScanView` states), **`Results:`** (results list selection + folder nav), **`Trash:`** (review, move, complete, scan-again), **`Settings:`**, **`Updates:`**. The **`[apex:vue:category]`** tag still groups by kind (`view` vs `file` vs `trash`, etc.).

**Optional `data`:** third argument is JSON-stringified on the Terminal line; use for small objects (e.g. scan options), not large trees.

## Rust layer

### `src-tauri/src/log.rs`

| Item                                   | Output                             | Meaning                                                                                   |
| -------------------------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| **`dev_rust_trace(channel, message)`** | **stdout** `[apex:rust:{channel}]` | IPC traces and extra detail (debug build or `APEX_DISK_DEBUG`).                           |
| **`format_bytes_si(n)`**               | —                                  | SI byte strings for **`scan`** / **`disk`** diagnostic lines (matches Vue `formatBytes`). |
| **`log_message`** (command)            | **stdout**                         | Prints the full line from Vue (already contains `[apex:vue:…]`).                          |
| **`log_error_message`** (command)      | **stderr**                         | Uncaught web / Vue errors mirrored from TS.                                               |
| **`is_debug_mode`** (command)          | —                                  | Reads `APEX_DISK_DEBUG` for `initLog()`.                                                  |

### `dev_rust_trace` channels

Keep **`channel`** strings in sync with this table (and with Vue categories where they match).

| `channel`         | Command / area                                                                                               | Vue counterpart                  |
| ----------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------- |
| **`app`**         | `get_system_info`                                                                                            | `app`                            |
| **`scan`**        | `get_user_folders`, `cancel_scan`, throttled **`Scan: live — …`**, per-folder **`Scan: top-level done — …`** | `scan`                           |
| **`disk`**        | `get_disk_usage`, **`Disk: usage — …`** after successful read                                                | `disk`                           |
| **`trash`**       | `trash_paths`                                                                                                | `trash`                          |
| **`permissions`** | `check_full_disk_access`                                                                                     | _(no dedicated Vue `log` today)_ |

### Updater (`src-tauri/src/updater.rs`)

Uses **`log_update`** → **`[apex:rust:updater]`** on stdout with the **same visibility rule** as `dev_rust_trace` (debug build or `APEX_DISK_DEBUG`). Messages use prefixes like **`Dialog:`** / **`Menu:`**.

## Streams and prefixes (Terminal)

| Pattern                                                   | Stream | Source                                    |
| --------------------------------------------------------- | ------ | ----------------------------------------- |
| `[apex:vue:<category>]`                                   | stdout | `log()` → `log_message`                   |
| `[apex:vue/window-error]`, `/unhandled-rejection`, `/vue` | stderr | Diagnostic handlers → `log_error_message` |
| `[apex:rust:<channel>]`                                   | stdout | `dev_rust_trace` or updater `log_update`  |
| _(unprefixed)_                                            | either | Tauri, Wry, other crates, panics          |

**Rejected `invoke`** errors usually surface as caught logs in TS; if uncaught, they may appear under **`[apex:vue/unhandled-rejection]`**.

## Source index

| Location                                                     | What                                                             |
| ------------------------------------------------------------ | ---------------------------------------------------------------- |
| `src/lib/log.ts`                                             | Logger API, categories, diagnostic registration                  |
| `src/main.ts`                                                | `initLog()`, `registerDiagnosticHandlers`                        |
| `src/lib/use-disk-usage.ts`                                  | `disk`                                                           |
| `src/lib/use-scanner.ts`                                     | `scan`                                                           |
| `src/lib/use-app-update.ts`                                  | `app` (updates)                                                  |
| `src/lib/use-app-views.ts`                                   | `view` (app shell)                                               |
| `src/stores/app-settings.ts`                                 | `settings`                                                       |
| `src/components/ScanView.vue`                                | `view`, `trash`                                                  |
| `src/components/ScanResultsList.vue`                         | `file`, `nav`                                                    |
| `src/components/ScanTrashList.vue`                           | `trash`                                                          |
| `src/components/ScanTrashConfirmation.vue`                   | `trash` (post-delete **Scan again**)                             |
| `src-tauri/src/log.rs`                                       | Rust helpers + IPC log commands                                  |
| `src-tauri/src/updater.rs`                                   | Updater stdout lines                                             |
| `src-tauri/src/scan.rs`                                      | `get_user_folders` / `cancel_scan` + live scan folder/size lines |
| `src-tauri/src/disk.rs`                                      | `get_disk_usage` + **`Disk: usage — …`**                         |
| `src-tauri/src/trash.rs`, `permissions.rs`, `system_info.rs` | `dev_rust_trace` at command entry                                |

## Adding a new log

1. **Vue:** `import { log } from '@/lib/log'`, pick a **category** from the table (extend `LogCategory` in `log.ts` only if you add a new area).
2. **Rust IPC:** optional **`log::dev_rust_trace("channel", "…")`** at the start of the handler or after a meaningful result; add the channel to the table above if new.
3. Update this file if you introduce a new category, channel, or stream.
