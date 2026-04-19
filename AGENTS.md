# AGENTS.md

## Project

ApexDisk — macOS-only Tauri 2 desktop app (Rust backend + Vue 3 frontend) for disk usage analysis and cleanup. Ships as a universal binary (Intel + Apple Silicon). Minimum macOS 10.15. Supported macOS/Safari/architecture ranges are documented in `docs/COMPATIBILITY.md` — keep new dependencies and syntax within that range.

## Agent-facing docs (`docs/`)

`docs/` holds reference material agents are expected to read and keep current:

- **`docs/ARCHITECTURE.md`** — frontend/backend split: what each side owns, how they talk (commands/events/store), subsystem walkthroughs, directory map.
- **`docs/COMPATIBILITY.md`** — macOS / Safari / architecture targets and per-feature progressive-enhancement matrix.
- **`docs/LOGGING.md`** — unified Vue + Rust diagnostic scheme (`[apex:…]` prefixes, channels, `APEX_DISK_DEBUG`).
- **`docs/RELEASES.md`** — how to cut stable and Beta builds: version fields, changelog conventions, workflows.
- **`docs/UPDATES.md`** — in-app updater behavior (auto/manual, endpoint, signing, dialogs).

**Before every commit, analyze every file in `docs/` for update eligibility and update any whose contents no longer match the change.** Do not skim for matches — open each file and check. Triggers include (non-exhaustive): bumping dependencies, touching `vite.config.ts` / Cargo deps, switching a UI feature to a different API (e.g. native Popover ↔ Floating UI), adding/changing a log category or Rust trace channel, changing the updater flow or menu items, adding a progressive-enhancement CSS feature, renaming modules or restructuring directories, moving responsibility across the Rust/webview boundary, adjusting the release or Beta workflows. If none of the docs need changes, that is fine — just confirm you looked at each one. Never commit code that contradicts `docs/`.

Root-level `RELEASES.md`, `RELEASES_BETA.md`, `LICENSE.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `README.md` stay at the repo root — they are user-/CI-facing, not agent instructions.

## Stack

| Layer           | Tech                                                           |
| --------------- | -------------------------------------------------------------- |
| Backend         | Rust, Tauri 2, objc2 (Foundation/AppKit bindings)              |
| Frontend        | Vue 3 (`<script setup lang="ts">`), TypeScript, Vite           |
| Styling         | Scoped CSS with CSS nesting, lightningcss (Safari 13 target)   |
| Testing         | Rust: `cargo test` (src-tauri/tests/), E2E: WebdriverIO (e2e/) |
| Formatting      | Oxfmt (import sorting, code formatting)                        |
| Package manager | pnpm                                                           |

## Architecture

- **Settings**: Rust-side persistence via `tauri_plugin_store`. Frontend store in `src/stores/app-settings.ts` — no provide/inject.
- **Translations**: Per-component YAML files in `src/assets/translations/` (key-first: one entry per phrase with a language sub-key under `en, it, es, fr, pt, de, ru, zh, ja, ar`). Imported directly via `@rollup/plugin-yaml` registered in `vite.config.ts`. Composable: `useTranslations()` → `t(module, key, vars?)`. 10 languages. Long prose uses `>-` folded scalars; CJK (zh, ja) values must stay on one line — `>-` inserts a space at line joins and there are no inter-word spaces in CJK.
- **Themes**: CSS variables in `src/assets/css/theme.css`. `data-theme` attribute on `<html>`. 8 themes.
- **Scanning**: Rust (`src-tauri/src/scan.rs`) builds a `FolderInfo` tree, emits progress events. Frontend navigates the tree with browser-style back/forward stacks.
- **Deletion**: Items moved to macOS Trash (recoverable). Protected/skipped folders filtered in Rust before trashing.
- **Native menu**: Built in Rust, localized via `menu_translations.rs`, rebuilt on language change.
- **Diagnostics**: Vue `log()` + Rust `dev_rust_trace` / updater lines — see **`docs/LOGGING.md`** (`APEX_DISK_DEBUG`, `[apex:…]` prefixes).

## Code conventions

### Comments

- Default to no comments. Code must be readable and clean; comments support code, they do not drive it.
- Only add a comment when the WHY is non-obvious (workaround, subtle invariant, surprising behavior, version constraint, etc.).
- Do not restate what well-named code already says (e.g. `// Walk up to find the nearest scrollable ancestor` above an obvious loop).
- Do not write library-introduction or rationale chatter (e.g. "Positioning via @floating-ui/dom" — the import says that).
- No commented-out code. Delete it.
- If a function needs a paragraph to explain it, prefer renaming or splitting it instead.

### File naming

- `.vue`: PascalCase (`ScanResultsList.vue`)
- `.ts`: kebab-case (`use-scanner.ts`), except component-coupled files (PascalCase, e.g. `ScanResultsListItem.ts`)

### License headers

- Every first-party source file (`.ts`, `.tsx`, `.vue`, `.rs`, `.sh` under `src/`, `src-tauri/src/`, `e2e/`, `tests/`, `scripts/`) carries an SPDX + copyright header on the top two lines (comment syntax varies by language; shell scripts keep the shebang on line 1).
- `pnpm headers` adds the header to any new file and is idempotent; it runs automatically as step 1 of `/sync` and `/force-sync`.
- `pnpm headers:check` exits non-zero if any covered file is missing the header — suitable for CI.

### Vue

- Always `<script setup lang="ts">`. No Options API.
- Script order: `defineProps` → `defineEmits` → blank line → all other logic.
- Template refs: `useTemplateRef('name')`, not `ref<HTMLElement>(null)`.
- Props: camelCase. Emits: kebab-case. Components: PascalCase tags.
- Boolean props/vars must have a leading verb: `is*`, `has*`, `can*`.
- SVG icons: always `aria-hidden="true"`.
- Semantic HTML: use landmarks, correct heading hierarchy, lists. Use `aria-live="polite"` for dynamic status/navigation changes; `assertive` only for urgent messages.

### CSS

- Class format: `ComponentName-nestedElement` (matches filename).
- Use CSS nesting with `&` for pseudo-selectors/states.
- BEM modifiers: full class names at root (`ComponentName-element--modifier`), never `&--modifier`.
- Media queries: nested inside the selector, never at root.

### TypeScript

- Blank line between groups of different statement types (`const`, `let`, expressions, `return`).
- `if` bodies: braces required unless condition + statement fit on one line.
- Prefer `!value` over `value === false`.
- Tauri boundary objects: snake_case. Frontend-only objects: camelCase.

### Rust

- Import order: framework → std → 3rd-party → crate-internal. Blank line between groups.

### Commits

- No conventional commit prefixes. Use imperative verb + short description.
- Co-authored-by trailer when commit is agent-made.

### External scripts (codegen, build tooling, one-off utilities)

- Run on **Bun** and use **Bun APIs** (`Bun.file`, `Bun.write`, `Bun.Glob`, `Bun.spawnSync`, `Bun.YAML.parse`, etc.). Do not use `tsx` / `ts-node` / plain `node`.
- Scripts live in `scripts/` as `.ts` files with a `// SPDX…` header and run via `bun run scripts/<name>.ts` (wrap behind a `pnpm <name>` alias).
- YAML parsing: use the built-in `Bun.YAML.parse` — no extra package.
- If a script needs to emit `.d.ts` files, use **tsdown**, not `tsc`/`dts-bundle-generator`/etc.
- Vite bundling runs on Rolldown, which is Rollup-plugin-compatible — reach for `@rollup/plugin-*` before writing custom transforms.

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
```

## Testing rules

When touching Rust code or `src-tauri/`:

1. Run `pnpm test:unit` (or `cd src-tauri && cargo test`).
2. If tests fail: fix the test if your change is correct and the test is outdated; add tests if none exist for the changed code.
3. Tests use temp dirs, never the real user home.

## What not to do

- Do not install npm packages unless asked.
- Do not create tests or docs unless asked.
- Do not add platform-specific code for Windows/Linux.
- Do not propose launching dev servers (it's always running).
- Do not use provide/inject for settings.
- Do not use Options API or plain `<script>`.
