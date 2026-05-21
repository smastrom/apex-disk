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

The boundary is the only real design axis in this codebase: every feature has a **Rust side** (anything touching the OS, the filesystem, or persistent state) and a **webview side** (anything you can see or click). Everything else (themes, animations, translations) is pure frontend with Rust as a passive store.

Subsystem deep-dives live in their own files: [`state-lifecycle.md`](state-lifecycle.md) (scan + trash flow, Vue/Rust memory lifecycle), [`tauri-commands.md`](tauri-commands.md) (IPC channels, command surface, settings store), [`translations.md`](translations.md), [`themes.md`](themes.md). See also [`updates.md`](updates.md), [`releases.md`](releases.md), [`logging.md`](logging.md), [`compatibility.md`](compatibility.md).

## What each side owns

### Rust (`src-tauri/src/`) owns

| Responsibility                                                                               | Module                            |
| -------------------------------------------------------------------------------------------- | --------------------------------- |
| Home-folder scan (walk, size, filter protected/skipped paths, emit progress, support cancel) | `scan.rs`                         |
| Move-to-Trash (with protected-path filter applied again before delete)                       | `trash.rs`                        |
| Persistent settings (read/write/merge with defaults, concurrent-write locking)               | `store.rs`                        |
| Protected + skipped folder lists (single source of truth, consumed by scan _and_ trash)      | `safe_folders.rs`                 |
| Disk volume usage (total / free / used for the home volume)                                  | `disk.rs`                         |
| System info (OS version, arch, memory)                                                       | `system_info.rs`                  |
| Full Disk Access (FDA) TCC check                                                             | `permissions.rs`                  |
| macOS locale detection + `AppleLanguages` sync via objc2                                     | `locale.rs`                       |
| Native menu build + per-language rebuild                                                     | `menu.rs`, `menu_translations.rs` |
| Native message / ask dialogs                                                                 | `native_dialog.rs`                |
| Update check / download / restart, plus menu-text updates                                    | `updater.rs`                      |
| Diagnostic logging (Rust-side `dev_rust_trace`, IPC logging command)                         | `log.rs`                          |
| macOS extended-attribute helpers                                                             | `xattr.rs`                        |
| App-wide constants (menu ids, URLs, default language/theme)                                  | `constants.rs`                    |
| E2E-only fixtures (trash dry-run, state reset)                                               | `e2e_fixtures.rs`                 |

Entry points: `main.rs` delegates to `lib.rs::run()`, which installs plugins (`tauri_plugin_store`, `tauri_plugin_updater`, `tauri_plugin_opener`; `tauri_plugin_webdriver` under the `e2e` feature), registers every command in `tauri::generate_handler!`, and runs the setup hook (store defaults, locale bootstrap, menu build).

**Rust never renders UI.** The native menu is the only OS-visible surface it controls directly.

### Frontend (`src/`) owns

| Responsibility                                                                           | Location                                                                                           |
| ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Vue component tree + routing between views (scan / settings / information)               | `src/components/`, `src/lib/use-app-views.ts`                                                      |
| Scan flow UI (launch, progress, results list, trash review, confirmation)                | `src/components/Scan*.vue`                                                                         |
| Browser-style **back/forward navigation stacks** inside a folder tree                    | `src/components/ScanResultsList.vue` (state lives in the component)                                |
| Selection state (which rows are checked for trash)                                       | Scan result components, not a global store                                                         |
| Settings **UI** + typed access (hidden-files toggle, theme, language, auto-updates)      | `src/stores/app-settings.ts`, `src/components/SettingsView.vue`                                    |
| Theme application — `data-theme` attribute on `<html>`, CSS variables                    | `src/assets/css/theme.css`, `src/stores/app-settings.ts`                                           |
| Translations — per-component `.yaml` files (key-first), reactive via `useTranslations()` | `src/assets/translations/`, `src/lib/use-translations.ts`                                          |
| All presentational primitives (buttons, popovers, spinners, icons)                       | `src/components/ui/`                                                                               |
| Animations, view transitions, reduced-motion handling                                    | `src/assets/css/animations.css`, `src/lib/use-view-transition.ts`, `src/lib/use-reduced-motion.ts` |
| Vue-side diagnostic log (forwards to Rust when `APEX_DISK_DEBUG` is on)                  | `src/lib/log.ts`                                                                                   |

**Frontend owns no filesystem logic.** If a feature needs to know _what's on disk_ (including whether a path is protected), it asks Rust. The "protected" flag on a `FolderInfo` is filled server-side so the UI never re-evaluates it.

## How the two sides communicate

Three channels, all provided by Tauri:

1. **`invoke()`** — Webview → Rust, returns a `Promise`. Used for everything transactional.
2. **Events** — Rust → webview push, one direction, many subscribers. Used for streaming progress.
3. **Persistent state** — `tauri_plugin_store` writes JSON to the app support directory. Both sides read; only Rust writes.

Full channel semantics, the complete command surface, registration patterns in `lib.rs`, and the settings flow live in [`tauri-commands.md`](tauri-commands.md).

## Boundary conventions

A handful of rules that make the IPC hop easy to reason about:

- **Names cross the wire unchanged.** Rust structs serialize field names as-is; the frontend consumes them verbatim. That means **Tauri-boundary objects use `snake_case`** even in TypeScript (e.g. `folder_info.is_protected`), while **frontend-only objects use `camelCase`**. Types for boundary shapes live in `src/types/` and mirror the Rust structs.
- **Do not reimplement Rust logic in Vue.** The "is this path protected?" / "how big is this folder?" / "is FDA granted?" questions belong to Rust, even when the answer is cached in a ref.
- **Commands are total.** A command either resolves with data or rejects with an error string; no partial states on the wire. Progress-style feedback goes on events, not on the invoke promise.
- **Cancellation is cooperative.** Two-step: Rust `Arc<AtomicBool>` token + Vue `scanGeneration` counter. Full mechanics in [`state-lifecycle.md`](state-lifecycle.md).
- **Truncation is a shared contract.** Two scanner caps (`scan::MAX_FILES_PER_DIR`, `scan::MAX_FOLDERS_PER_DIR`) bound the tree on the Rust side; one frontend cap (`MAX_DISPLAYED_ITEMS`) bounds the DOM. All three feed the same user-facing "list truncated" notice; flipping either scanner cap without updating the message lies to the user. Full contract in [`state-lifecycle.md`](state-lifecycle.md).
- **macOS-only.** There's no Windows/Linux branching in either side; see [`compatibility.md`](compatibility.md). objc2 AppKit/Foundation bindings are used freely in Rust.

## Build + testing boundary

- **Frontend build:** `pnpm build` → `vue-tsc --noEmit && vite build`. Vite pipes CSS through **lightningcss** with a Safari 13 target so modern CSS downlevels to flat syntax (matches the declared minimum macOS; see [`compatibility.md`](compatibility.md)).
- **App build:** `pnpm tauri:build` bundles Rust + Vite output into a universal-binary `.app` / `.dmg`. `pnpm tauri:build:beta` layers `tauri.beta.conf.json` as a merge config (different bundle id / product name, `createUpdaterArtifacts: false`).
- **Unit / integration tests:** `pnpm test:unit` runs `cargo test` inside `src-tauri/` with `--test-threads=1` (serial; some tests mutate process-global state). Integration tests live in `src-tauri/tests/` (`scan_test.rs`, `trash_test.rs`, `store_test.rs`, `safe_folders_test.rs`, etc.); shared helpers in `tests/support/mod.rs`.
- **E2E:** `pnpm test:e2e` drives the app with WebdriverIO against a build with the `e2e` cargo feature enabled (which exposes `tauri_plugin_webdriver` and the `set_e2e_trash_mode` / `reset_e2e_state` commands). Specs in `e2e/specs/`.

Release workflow and Beta channel: see [`releases.md`](releases.md).

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
    translations/              # Per-component .yaml files + index factory
  types/                       # Boundary types mirroring Rust structs (snake_case fields)

src-tauri/
  src/                         # Native side (see table above)
  tests/                       # Integration tests + support/
  tauri.conf.json              # Base Tauri config
  tauri.beta.conf.json         # Beta merge config

e2e/                           # WebdriverIO specs + helpers
reference/                     # Agent-facing reference files (this folder)
```
