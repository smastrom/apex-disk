# Code Style

The conventions actually used in the codebase. Two tools own the mechanical rules; don't hand-format or hand-reorder.

## Formatting, imports, blank lines

- **Oxfmt** owns code formatting for `.ts` / `.tsx` / `.vue` / `.css` / `.json` / `.html` / `.md`. Config: `.oxfmtrc.jsonc`. Run `pnpm fmt` / `pnpm fmt:check`.
- **Oxlint** owns import order and blank-line rules via the JS plugin in `oxlint-plugins.mjs`:
   - `stylistic/sort-imports` (perfectionist) — group order: `vue-files → type → builtin → external → internal → constants → relative → side-effect → style`. Internal pattern: `^@/.+`. One blank line between groups.
   - `stylistic/padding-line-between-statements` — blank line after the last import; blank lines around `block-like`, between `const`/`let` runs and expressions, before terminators.

Config: `.oxlintrc.json`. Run `pnpm oxlint --fix`. With the project `.vscode/settings.json`, save triggers both (`source.fixAll.oxc: always` then format).

## License headers

**Every first-party source file** carries an SPDX + copyright header on the top two lines. Covered files: `.ts`, `.tsx`, `.vue`, `.rs`, `.sh` under `src/`, `src-tauri/src/`, `e2e/`, `tests/`, `scripts/`. Shell scripts keep the shebang on line 1.

Vue (HTML comment):

```html
<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->
```

TS / Rust:

```ts
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei
```

`pnpm headers` adds the header to any new file and is idempotent. `pnpm headers:check` exits non-zero if any covered file is missing the header — suitable for CI. Both `/sync` and `/force-sync` run `pnpm headers` as step 1.

## TypeScript / Vue Script

Applies to `.ts` files and the `<script setup>` block of `.vue` files.

### Other rules

- **`if` bodies:** braces required unless condition + statement fit on one line. One-line form is fine: `if (cond) return`.
- **Prefer `!value`** over `value === false`.
- **No `as any`, no `// @ts-ignore`** to silence the checker. Code must be type-clean (`pnpm typecheck`).
- **Boundary objects use `snake_case`** (Tauri/Rust IPC). Frontend-only objects use `camelCase`. See `[architecture.md](architecture.md)` — boundary conventions.
- **Types live in `src/types/`.** Boundary types mirror Rust structs exactly.

## Vue file shape

```vue
<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ComponentName

Purpose: One sentence on what this component does.

Props: prop1 (Type), prop2 (Type)

Example:
 <ComponentName :prop1="value" @event="handler" />
-->

<script setup lang="ts">
import ...

const props = defineProps<{ ... }>()
const emit = defineEmits<{ ... }>()

const { t } = useTranslations()
// ...
</script>

<template>
   <!-- ... -->
</template>

<style scoped>
.ComponentName-root { ... }
.ComponentName-root--modifier { ... }
</style>
```

### Script

- **Always `<script setup lang="ts">`.** No Options API. No plain `<script>`.
- **Order:** `defineProps` → `defineEmits` → **blank line** → all other logic.
- **Template refs:** `useTemplateRef('name')`, not `ref<HTMLElement>(null)`.
- **Props:** camelCase. **Emits:** kebab-case event names. **Components:** PascalCase tags in templates.
- **Boolean props/vars** must have a leading verb: `is*`, `has*`, `can*`.

### Template

- **Semantic HTML:** use landmarks (`<header>`, `<main>`, `<nav>`), correct heading hierarchy, lists for lists.
- **SVG icons:** always `aria-hidden="true"`.
- **Live regions:** `aria-live="polite"` for dynamic status/navigation changes; `assertive` only for urgent messages.

### Style

- `**<style scoped>` always.\*\* Component CSS is local.
- **Class format:** `ComponentName-element`, matching the filename. E.g. `.ScanResultsListItem-root`, `.ScanResultsListItem-check`, `.ScanResultsListItem-info`.
- **No top-level chained selectors.** Each top-level rule is one flat selector — write `.ScanResultsListItem-root--selected { … }` as its own rule, not nested under `.ScanResultsListItem-root`.
- **BEM modifiers** use the full class name with `--`: `.ScanResultsListItem-root--selected`, never `&--selected`. The modifier is a separate top-level rule.
- **Use `&` only for pseudo-selectors and states** inside an existing selector: `&:hover`, `&:focus`, `&[disabled]`.
- **Media queries:**
   - Component-local responsive tweaks: nest the `@media` inside the selector.
   - Cross-cutting overrides like `@media (prefers-reduced-motion: reduce)`: keep at the top level with the relevant selectors inside (matches how `ScanResultsListItem.vue` handles reduced motion).
- **Blank line between rule blocks** at the top level.
- **No hardcoded values** for color, spacing, font sizes, or border radii — use CSS variables from `src/assets/css/theme.css` (see `[themes.md](themes.md)`). Examples: `var(--color-bg)`, `var(--spacing-md)`, `var(--font-size-xl)`, `var(--touch-height-default)`.

## File naming

- `**.vue`:\*\* PascalCase — `ScanResultsList.vue`, `AppHeader.vue`.
- `**.ts`:\*\* kebab-case — `use-scanner.ts`, `format.ts`, `app-settings.ts`. Exception: component-coupled `.ts` files keep PascalCase to match their `.vue` sibling — e.g. `ScanResultsListItem.ts`.
- **YAML translation files:** kebab-case matching the component — `scan-results-list.yaml`.

## Comments

- **Default to no comments.** Code must be readable and clean; comments support code, they do not drive it.
- **Only add a comment when the WHY is non-obvious** (workaround, subtle invariant, surprising behavior, version constraint).
- **Do not restate what well-named code already says.** E.g. don't write `// Walk up to find the nearest scrollable ancestor` above an obvious loop.
- **Do not write library-introduction or rationale chatter.** `// Positioning via @floating-ui/dom` is what the import line says.
- **No commented-out code.** Delete it.
- **If a function needs a paragraph to explain it**, prefer renaming or splitting it instead.
- **No em dashes (`—`) as parenthetical interrupts in comments, and no en dashes (`–`) at all.** Comments are user-facing prose: follow [`voice.md`](voice.md). Em dashes are only acceptable as label separators in a bulleted comment list, which is rare; almost always rewrite with a comma, parentheses, period, or colon.

## Rust

- **Formatting:** rustfmt with `rustfmt.toml` at the repo root (cargo walks up from `src-tauri/Cargo.toml`). Requires **nightly rustfmt** because the config uses unstable options (`group_imports`, `imports_granularity`, `wrap_comments`, etc.). Run `pnpm rust:fmt` / `pnpm rust:fmt:check`. The build still runs on stable; only the formatter needs nightly.
- **Install once:** `rustup toolchain install nightly --component rustfmt --profile minimal`.
- **Import order** (enforced by rustfmt `group_imports = "StdExternalCrate"`): `std` → external crates → crate-internal. Blank line between groups. Adjacent `use` lines from the same crate collapse into nested form.
- Cargo deps managed in `src-tauri/Cargo.toml`. MSRV is implicit (1.70+) — no `rust-toolchain.toml`.
- See `[compatibility.md](compatibility.md)` for the macOS API surface allowed within the 10.15 floor.

## External scripts (codegen, build tooling, one-off utilities)

- **Run on Bun**, not `tsx` / `ts-node` / plain `node`.
- Use **Bun APIs**: `Bun.file`, `Bun.write`, `Bun.Glob`, `Bun.spawnSync`, `Bun.YAML.parse`.
- Scripts live in `scripts/` as `.ts` files with the SPDX header and run via `bun run scripts/<name>.ts`, wrapped behind a `pnpm <name>` alias.
- **YAML parsing:** use the built-in `Bun.YAML.parse` — no extra package.
- **If a script needs to emit `.d.ts`**, use **tsdown**, not `tsc`/`dts-bundle-generator`.
- Vite bundling runs on Rolldown (Rollup-plugin-compatible) — reach for `@rollup/plugin-`\* before writing custom transforms.

## Commits

- **No conventional-commit prefixes.** Use an imperative action-title subject (≤70 chars).
- Body is concise prose or bullets, **only when the why isn't obvious** from the subject.
- Always include the `Co-Authored-By` trailer when the commit is agent-made.
- Never skip hooks (`--no-verify`) and never bypass signing — investigate failures instead.

## What not to do

- Do not install npm packages unless asked.
- Do not create tests or docs unless asked.
- Do not add platform-specific code for Windows/Linux.
- Do not propose launching dev servers (it's always running).
- Do not use `provide` / `inject` for settings — use `useAppSettings()`.
- Do not use Options API or plain `<script>`.
