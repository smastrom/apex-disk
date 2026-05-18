# AGENTS.md

## Project

ApexDisk — macOS-only Tauri 2 desktop app (Rust backend + Vue 3 frontend) for disk usage analysis and cleanup. Ships as a universal binary (Intel + Apple Silicon). Minimum macOS 10.15.

## Stack

| Layer           | Tech                                                           |
| --------------- | -------------------------------------------------------------- |
| Backend         | Rust, Tauri 2, objc2 (Foundation/AppKit bindings)              |
| Frontend        | Vue 3 (`<script setup lang="ts">`), TypeScript, Vite           |
| Styling         | Scoped CSS with CSS nesting, lightningcss (Safari 13 target)   |
| Testing         | Rust: `cargo test` (src-tauri/tests/), E2E: WebdriverIO (e2e/) |
| Formatting      | Oxfmt (import sorting, code formatting)                        |
| Package manager | pnpm                                                           |

## Agent-facing reference (`reference/`)

`reference/` holds deep specs that agents read on demand. The
`reference-loader` rule in `.claude/rules/` maps operations to the right
file; the `pre-commit-protocol` rule guarantees a docs sweep before any
commit.

| File                          | Covers                                                                                      |
| ----------------------------- | ------------------------------------------------------------------------------------------- |
| `reference/architecture.md`   | Frontend/backend split, what each side owns, boundary conventions, build/testing.           |
| `reference/scanning.md`       | Scan + trash flow, `FolderInfo`, progress events, cancellation, selection model.            |
| `reference/tauri-commands.md` | IPC channels, command surface, `lib.rs` registration, settings store, locale + menu.        |
| `reference/translations.md`   | Per-component YAML, 10 languages, `useTranslations()`, CJK folding rules.                   |
| `reference/themes.md`         | CSS variables, `data-theme` switching, 8 themes, adding a new theme.                        |
| `reference/code-style.md`     | Oxfmt, import sorting, Vue/CSS/TS conventions, file naming, license headers, comments.      |
| `reference/testing.md`        | Suites, when to run, Rust integration patterns, E2E + `e2e` cargo feature, what not to add. |
| `reference/compatibility.md`  | macOS / Safari / architecture targets, progressive enhancement matrix, oxfmt fallbacks.     |
| `reference/logging.md`        | `[apex:…]` diagnostic scheme, Vue categories, Rust trace channels, `APEX_DISK_DEBUG`.       |
| `reference/releases.md`       | How to cut stable and Beta builds, version fields, changelog conventions, workflows.        |
| `reference/updates.md`        | In-app updater (auto/manual), endpoint, signing, dialogs.                                   |

Root-level `RELEASES.md`, `RELEASES_BETA.md`, `LICENSE.md`,
`CODE_OF_CONDUCT.md`, `SECURITY.md`, `README.md` stay at the repo root —
they are user-/CI-facing, not agent instructions.

## Outcome-facing content (`marketing/`)

`marketing/` holds content describing the product to users — FAQs, feature
descriptions, marketing copy. Unlike `reference/` (how the code works), this
is about what the user gets. Agents update it only when outcomes change
(new feature shipped, behavior reframed, FAQ becomes stale); they do not
edit it for implementation reasons. The `/sync` sweep still covers it.

## Slash commands (`.claude/commands/`)

| Command                | Purpose                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------ |
| `/sync`                | Group uncommitted work into logical commits, sweep all `.md`, run tests, push.       |
| `/force-sync`          | Reconcile `.md` against commits that bypassed `/sync`, then commit drift.            |
| `/compatibility-check` | Full compatibility verification against macOS 10.15 / Safari 13 / MSRV.              |
| `/release`             | Prepare a stable release: bump version in 3 files, prepend section to `RELEASES.md`. |
| `/beta-notes`          | Add a dated section to `RELEASES_BETA.md` for the Beta workflow's pre-release body.  |

## Testing

Full suite matrix, commands, and conventions live in `reference/testing.md`. Key always-on rules:

1. `/sync` and `/force-sync` run the relevant suites before pushing. Never push red code, never bypass with `--no-verify` / `--force`. If a suite fails, stop and surface — fix forward in a follow-up commit.
2. Tests use temp dirs, never the real user home.
3. Do not add tests unless asked.

## Key directories

```
src/
  components/        # Vue components (ui/ for presentational)
  assets/css/        # theme.css, global.css, reset.css, classes.css, animations.css, rtl.css
  assets/translations/  # Per-component translation files + index.ts
  lib/               # Composables (use-*), utilities, constants
  stores/            # App settings store
  types/             # Shared TypeScript types

src-tauri/
  src/               # Rust backend modules
  tests/             # Rust integration tests
  tauri.conf.json    # Tauri config (bundle targets, min macOS version)

reference/           # Agent-facing reference (see table above)
.claude/
  rules/             # Always-loaded routing + protocol rules
  commands/          # Slash commands
```

## What not to do

- Do not install npm packages unless asked.
- Do not create tests or docs unless asked.
- Do not add platform-specific code for Windows/Linux.
- Do not propose launching dev servers (it's always running).
- Do not use provide/inject for settings.
- Do not use Options API or plain `<script>`.
