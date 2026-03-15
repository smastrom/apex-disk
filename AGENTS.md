# AGENTS.md

## Project

ApexDisk — macOS-only Tauri 2 desktop app (Rust backend + Vue 3 frontend) for disk usage analysis and cleanup. Ships as a universal binary (Intel + Apple Silicon). Minimum macOS 10.15. Supported macOS/Safari/architecture ranges are documented in `COMPATIBILITY.md` — keep new dependencies and syntax within that range.

## Stack

| Layer | Tech |
|-------|------|
| Backend | Rust, Tauri 2, objc2 (Foundation/AppKit bindings) |
| Frontend | Vue 3 (`<script setup lang="ts">`), TypeScript, Vite |
| Styling | Scoped CSS with CSS nesting, lightningcss (Safari 13 target) |
| Testing | Rust: `cargo test` (src-tauri/tests/), E2E: WebdriverIO (e2e/) |
| Formatting | Oxfmt (import sorting, code formatting) |
| Package manager | pnpm |

## Architecture

- **Settings**: Rust-side persistence via `tauri_plugin_store`. Frontend store in `src/stores/app-settings.ts` — no provide/inject.
- **Translations**: Per-component files in `src/assets/translations/`. Composable: `useTranslations()` → `t(module, key, vars?)`. 10 languages.
- **Themes**: CSS variables in `src/assets/css/theme.css`. `data-theme` attribute on `<html>`. 8 themes.
- **Scanning**: Rust (`src-tauri/src/scan.rs`) builds a `FolderInfo` tree, emits progress events. Frontend navigates the tree with browser-style back/forward stacks.
- **Deletion**: Items moved to macOS Trash (recoverable). Protected/skipped folders filtered in Rust before trashing.
- **Native menu**: Built in Rust, localized via `menu_translations.rs`, rebuilt on language change.

## Code conventions

### File naming
- `.vue`: PascalCase (`ScanResultsList.vue`)
- `.ts`: kebab-case (`use-scanner.ts`), except component-coupled files (PascalCase, e.g. `ScanResultsListItem.ts`)

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
