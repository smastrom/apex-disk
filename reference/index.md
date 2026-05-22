# Reference index

Keywords: routing, aliases, tier, commit sweep, discoverability.

Agent-facing specs for ApexDisk. Read on demand; do not load the whole folder
for every task.

**Start here when confused:** [`agent-workflow.md`](agent-workflow.md) (what
lint-staged runs vs what `/sync` owns, commit gate, scoped doc sweep).

## Task routing

Before writing or changing code, read the matching file(s) below. Match on
**Operation** or **Aliases** (grep-friendly). If a task spans multiple rows,
read every applicable file. Never commit code that contradicts a reference
file — if a reference is wrong, update it in the same change.

| Operation | Aliases | Read first |
| --------- | ------- | ---------- |
| Agent workflow, commit/push gate, lint-staged vs `/sync` | sync, lint-staged, husky, gate | [`agent-workflow.md`](agent-workflow.md) |
| Index, tiers, commit-time doc sweep | reference index, routing | [`index.md`](index.md) |
| New feature / refactor / multi-layer change | architecture, boundary, IPC | [`architecture.md`](architecture.md) |
| Scan/trash flow, cancel, `FolderInfo`, progress, memory | scanning, state-lifecycle, use-scanner, ScanView, trash | [`scan-trash-flow.md`](scan-trash-flow.md) |
| Protected/skipped paths, `safe_folders.rs` | protected, skipped, credentials | [`protected-files.md`](protected-files.md) |
| App shell, transitions, KeepAlive, popovers, focus ring | frontend-patterns, AppLayout, floating-ui, ui/ | [`app-shell.md`](app-shell.md) |
| Tauri command, event, or settings field | invoke, IPC, store, menu | [`tauri-commands.md`](tauri-commands.md) |
| Vue component, `.ts` file, or CSS | oxfmt, oxlint, BEM, script setup | [`code-style.md`](code-style.md) |
| Translations / language support | i18n, yaml, useTranslations | [`translations.md`](translations.md) |
| Tests, e2e or unit failure (suites) | cargo test, wdio, typecheck | [`testing.md`](testing.md) |
| E2E selectors, `data-testid`, fixtures | sel, navigation.ts, webdriver | [`e2e.md`](e2e.md) |
| Themes / CSS variables in `theme.css` | data-theme, palette | [`themes.md`](themes.md) |
| Bumping deps or new JS/CSS/macOS APIs | safari13, lightningcss, MSRV | [`compatibility.md`](compatibility.md) |
| User-facing prose: README, RELEASES, copy | voice, changelog | [`voice.md`](voice.md) |
| Logs, diagnostics, trace channels | apex, dev_rust_trace | [`logging.md`](logging.md) |
| In-app updater or `latest.json` | updater, signing | [`updates.md`](updates.md) |
| Stable or beta release cut | RELEASES, version bump | [`releases.md`](releases.md) |

## Read tiers

| Tier | When | Files |
| ---- | ---- | ----- |
| **0 — Workflow** | Before committing, or when unsure what is automated | [`agent-workflow.md`](agent-workflow.md) |
| **1 — Orientation** | New feature, refactor, or multi-layer change | [`architecture.md`](architecture.md) |
| **2 — Task** | One row from **Task routing** above | The matched file(s) |
| **3 — Deep dive** | Scan cancel, memory, truncation, KeepAlive | [`scan-trash-flow.md`](scan-trash-flow.md) |
| **3 — Deep dive** | Bumping deps or adding APIs | [`compatibility.md`](compatibility.md) |

Skip tier 3 unless the task touches that surface.

User-facing root docs (`README.md`, `RELEASES*.md`, etc.) are not in this
folder. See [`AGENTS.md`](../AGENTS.md).

## Changed paths → re-verify (commit sweep)

Use this table during `/sync` step 4 before the repo-wide `.md` skim. Update
any listed doc that drifted; then check remaining `.md` files for eligibility.

| Changed paths | Re-verify |
| ------------- | --------- |
| `src-tauri/src/safe_folders.rs`, scan/trash filters | [`protected-files.md`](protected-files.md), [`scan-trash-flow.md`](scan-trash-flow.md) |
| Scan/trash Vue, `use-scanner.ts`, `Scan*.vue` | [`scan-trash-flow.md`](scan-trash-flow.md), [`app-shell.md`](app-shell.md) if shell/transitions/popovers |
| New/changed Tauri command, event, settings field | [`tauri-commands.md`](tauri-commands.md) |
| `src/components/**`, `src/lib/**`, CSS | [`code-style.md`](code-style.md); [`app-shell.md`](app-shell.md) if shell/transitions/composables/popovers |
| `src/assets/translations/**` | [`translations.md`](translations.md) |
| `src/assets/css/theme.css`, theme picker | [`themes.md`](themes.md) |
| `data-testid`, `e2e/**`, `e2e_fixtures.rs` | [`e2e.md`](e2e.md), [`testing.md`](testing.md) |
| User-facing copy | [`voice.md`](voice.md) |
| Logs / `[apex:…]` | [`logging.md`](logging.md) |
| `updater.rs`, update UI | [`updates.md`](updates.md) |
| Version fields, release workflows | [`releases.md`](releases.md) |
| Deps, new JS/CSS/macOS APIs | [`compatibility.md`](compatibility.md) |
| Agent workflow, hooks, `/sync` | [`agent-workflow.md`](agent-workflow.md), `AGENTS.md`, `.claude/rules/`, `.claude/commands/` |
| Conventions CodeRabbit reviews | `.coderabbit.yaml` `path_instructions` |

Canonical cross-boundary rules (naming, IPC totals, macOS-only) live in
[`architecture.md`](architecture.md) only. Other files link there instead of
repeating them.
