# Mac User Lens — Agent Guidelines

This document defines code style and conventions. **Always follow these rules** when editing this codebase.

---

### For AI agents

- **Read the relevant section** before making edits (e.g. Vue rules when editing `.vue` files).
- **"Do not"** = never do this. **"Avoid"** = prefer not to, but allowed in edge cases.
- **When in doubt**, prefer the simpler option (no extra files, no extra imports, no extra reactivity).

## 1. Project guidelines

### Target platform

The app targets **macOS only**. Do not add or retain platform-specific code or conditionals for Windows or Linux.

### File organization

#### New files

- **Do not create tests** unless explicitly requested
- **Do not create documentation** unless explicitly requested

#### Functions

- Before adding a function, **check `src/lib/`** — it may already exist (e.g. `formatBytes` in `format.ts`)
- If the function **might be used outside** the current file, define it in `src/lib/`:
   - **Generic utilities** → `src/lib/utils.ts` (e.g. `isClient`, `noop`)
   - **Domain-specific** → new file (e.g. `src/lib/format.ts` for `formatBytes`, `formatDuration`)
- **Do not hesitate to create new files** in `src/lib/` when a function is reusable but not generic enough for `utils.ts`

#### Constants

- **Module-local constants**: Define in the same file when used only within that module
- **Shared constants**: If the scope extends beyond the module where they are declared, define them in `src/lib/constants.ts`
- **App constants shared with Rust** (app name, author/credits, release notes URL, license URL): Define in `src/lib/constants.ts` (frontend) and in `src-tauri/src/constants.rs` (Rust). Keep both files in sync when changing these values.

#### Types

- **Single-file types**: Define in the same file (e.g. `ScanProgress` only used in `App.vue`)
- **Shared types**: Define in `src/types/` — e.g. `FolderInfo` used by `App.vue`, `FolderNode.vue`, and Rust → `src/types/structures.ts`

#### CSS files

| File                         | Purpose                                              |
| ---------------------------- | ---------------------------------------------------- |
| `src/assets/css/theme.css`   | Variables (colors, spacing, etc.) and fonts          |
| `src/assets/css/global.css`  | Styles for `html`, `body`, and other global elements |
| `src/assets/css/reset.css`   | Style normalization                                  |
| `src/assets/css/classes.css` | Reusable utility classes used across the project     |

**These rules are mandatory.** Do not mix concerns between files.

### Project-specific implementation

#### Translations

Translations are stored in `src/assets/translations/`. The active language comes from the settings store (`settings.language`).

##### File structure

- **`global.ts`** — Strings shared across multiple components (e.g. `appName`, `scan`, `settings`, `donate`)
- **Component-named files** — One file per component: `AppHeader.ts`, `MainView.ts`, `SettingsView.ts`, `AppFooter.ts`, `AppLayout.ts`
- **`index.ts`** — Exports `translations` and `createT(lang)`

##### Translation file format

Each file exports an object keyed by language code (matching `Language` in `src/types/settings.ts`):

```ts
// SettingsView.ts
export const SettingsView = {
   en: { loadingSettings: 'Loading settings…', language: 'Language', ... },
   it: { loadingSettings: 'Caricamento impostazioni…', language: 'Lingua', ... },
} as const
```

##### Usage in components

1. Import `useTranslations` from `@/lib/useTranslations`
2. Call `const { t } = useTranslations()`
3. Use `t(module, key)` or `t(module, key, vars)` for interpolation

```vue
<script setup lang="ts">
import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()
</script>

<template>
   <p>
      {{
         t('MainView', 'scanning', {
            current: progress.current,
            total: progress.total,
         })
      }}
   </p>
</template>
```

##### Interpolation

Use `{{varName}}` in translation strings. Pass variables as the third argument:

```ts
// MainView.ts: scanning: 'Scanning… {{current}} of {{total}}'
t('MainView', 'scanning', { current: 1, total: 10 }) // → "Scanning… 1 of 10"
```

##### Adding a new language

1. Add the language to `Language` in `src/types/settings.ts`
2. Add the locale key to every translation file (e.g. `de: { ... }`)
3. Add a new match arm in `src-tauri/src/menu_translations.rs` → `labels_for()` for the native menu bar
4. Update `createT` in `index.ts` if the type needs to change

##### Adding translations for a new component

1. Create `src/assets/translations/ComponentName.ts` with a key per supported language
2. Import and add it to `translations` in `index.ts`
3. Use `t('ComponentName', 'key')` in the component

#### Theme

The app theme is controlled by CSS variables in `src/assets/css/theme.css` and persisted in the settings store via `themeColor` in `src/types/settings.ts`.

##### CSS variables

- The **default palette** lives under `:root` in `theme.css` and is used when the theme is `ROOT_THEME` (`'mac-user-lens'`).
- **Additional themes** use a `data-theme` attribute on `html` and override only the variables they need:

```css
:root {
   --color-bg: #050508;
   --color-text: #f8fafc;
   /* ... */
}

html[data-theme='ayu'] {
   --color-bg: #0b0e14;
   --color-text: #e6edf7;
   /* override only what changes for this theme */
}
```

The helper `applyTheme(theme)` in `src/lib/theme.ts` is responsible for toggling the `data-theme` attribute:

```ts
// src/lib/theme.ts
export function applyTheme(theme: string): void {
   if (theme === ROOT_THEME) {
      document.documentElement.removeAttribute('data-theme')
   } else {
      document.documentElement.setAttribute('data-theme', theme)
   }
}
```

##### Settings and types

Theme options are defined in `src/types/settings.ts` and consumed by the settings store in `src/stores/settings.ts`:

```ts
// src/types/settings.ts
export const THEME_COLORS = ['mac-user-lens', 'ayu'] as const
export type ThemeColor = (typeof THEME_COLORS)[number]
export const ROOT_THEME: ThemeColor = 'mac-user-lens'

export interface AppSettings {
   language: Language
   themeColor: ThemeColor
   /* ... */
}

export const DEFAULT_SETTINGS: AppSettings = {
   language: 'en',
   themeColor: 'mac-user-lens',
   /* ... */
}
```

```ts
// src/stores/settings.ts
export interface SettingsStore {
   settings: Ref<AppSettings>
   getThemeColor: () => string
   setThemeColor: (theme: ThemeColor) => Promise<void>
   /* ... */
}
```

`themeColor` is loaded from disk, validated against `THEME_COLORS`, and then used by the UI to render the theme picker and call `setThemeColor`.

##### Usage and examples

- **Applying the theme on startup** (e.g. in `App.vue` or `main.ts`):

```ts
const store = await createSettingsStore()
const currentTheme = store.getThemeColor()
applyTheme(currentTheme)
```

- **Reacting to user selection in a settings view**:

```ts
const settingsStore = inject(SETTINGS_KEY)!

async function onThemeChange(theme: ThemeColor) {
   await settingsStore.setThemeColor(theme)
   applyTheme(theme)
}
```

##### Adding a new theme

1. **Define the theme ID** in `src/types/settings.ts`:
   - Add it to `THEME_COLORS` (e.g. `'dracula'`) and update any unions if needed.
   - Optionally set it as the default by changing `DEFAULT_SETTINGS.themeColor`.
2. **Add CSS overrides** in `src/assets/css/theme.css`:
   - Create a new block `html[data-theme='dracula'] { ... }` and override only the variables that differ from `:root`.
3. **Expose in the UI**:
   - Use `THEME_COLORS` (or a local list) to render theme options in `SettingsView.vue`.
   - When the user selects a theme, call `setThemeColor(newTheme)` and then `applyTheme(newTheme)`.

#### Safe / protected folders

Specific macOS home directory paths cannot be selected or deleted. Both top-level folders (e.g. `Library`) and nested paths (e.g. `Library/Application Support`) are supported. Only the exact paths listed are protected — their descendants remain selectable.

##### Constants

- **Rust**: `src-tauri/src/safe_folders.rs` — `PROTECTED_RELATIVE_PATHS` (paths relative to home)
- **Frontend**: `src/lib/constants.ts` — `PROTECTED_FOLDER_NAMES` (must match Rust for reference)

To add or remove protected paths: edit both files. Both simple folder names (`"Library"`) and nested paths (`"Library/Application Support"`) are valid entries — protection is exact-match only, descendants are not affected.

##### Implementation

- **Rust**: `FolderInfo.is_protected` is set when building the tree; `safe_folders::is_path_protected()` strips the home prefix and checks the relative path against the list. Future delete command must reject protected paths.
- **Frontend**: `ListItem` receives `selectable={!item.is_protected}`; `toggleSelect` ignores protected items.

#### Tauri bundle targets

The `"app"` target **must** remain in `bundle.targets` alongside `"dmg"` in `tauri.conf.json`. The `"app"` target produces the `.app` bundle, and when `createUpdaterArtifacts` is `true`, Tauri generates the `.app.tar.gz` and `.app.tar.gz.sig` files from it. Without `"app"`, the updater signature is never created and the release workflow fails.

#### Native menu bar

The macOS menu bar is built in Rust (`src-tauri/src/menu.rs`) and localized via `src-tauri/src/menu_translations.rs`. It is rebuilt dynamically when the user changes language in settings.

##### Structure

Menu bar order: **App** (`Mac User Lens`) → **Window** → **Help**.

- **App submenu**: About, separator, Services, separator, Hide / Hide Others / Show All, separator, Quit
- **Window submenu**: Minimize, Close Window (uses `WINDOW_SUBMENU_ID` for macOS system integration)
- **Help submenu**: Release Notes, License (uses `HELP_SUBMENU_ID` for macOS system integration)

##### Language sync

- On startup, the frontend calls `invoke('set_menu_language', { lang })` after loading persisted settings (`src/stores/settings.ts`)
- On language change, `setLanguage()` calls the same command after persisting
- The Tauri command `set_menu_language` (in `menu.rs`) rebuilds and replaces the entire menu via `app.set_menu()`
- The initial menu is built with English (`"en"`) as a safe default before the frontend is ready

### Releases

For release workflow and instructions, refer to `releases/README.md`.

### Workflow

#### Package manager

- **Use pnpm** as package manager.
- **Do not install any npm package** unless explicitly asked.
- **Do not propose to launch dev servers** when testing implementations. The dev server is always running when the agent is working for the human.

#### Commits

- **Do not use Conventional Commits** — no prefixes like `feat:`, `fix:`, `refactor:`, `docs:`, etc.
- Use the project's existing nomenclature: **imperative verb + short description** (e.g. _Add protected system folders_, _Fix window drag region_, _Improve default settings_, _Move animations to their own settings group_).
- Keep the first line concise; add a body or scope after a colon when useful (e.g. _Fix abort, view switch lag, layout jumps, and startup crash_).
- **When the commit is made by the agent**, add a `Co-authored-by` trailer (e.g. `Co-authored-by: Cursor <cursoragent@cursor.com>`).

---

## 2. Code rules

### Vue

#### Component structure

- **Always use `<script setup lang="ts">`** — no Options API, no plain `<script>`.
- **Prefer splitting big components** into smaller ones using a "blocks" logic: each sub-component handles a distinct UI block
- Example: instead of one `ScanView.vue` with 200+ lines, split into `ScanViewHeader.vue`, `ScanViewProgress.vue`, `ScanViewTree.vue` — each with a single responsibility

#### Script setup order

After imports, declare **Vue-specific API only** in this order; then put all other logic below.

1. **defineProps** (if the component has props)
2. **defineEmits** (if the component emits)
3. **Other Vue-specific declarations** (e.g. `inject`, `defineExpose`) when needed
4. **Blank line**
5. **All other logic**: composables (`useTranslations()`, etc.), `ref`, `computed`, `watch`, lifecycle hooks, functions

Do not call composables, create refs, or run logic before `defineProps` / `defineEmits`.

```ts
// ✅ GOOD — props and emits first, then logic
const props = defineProps<{ items: Item[] }>()
const emit = defineEmits<{ (e: 'complete', items: Item[]): void }>()

const { t } = useTranslations()
const list = ref<Item[]>([])
watch(/* ... */)
```

```ts
// ❌ BAD — composable and logic before emits
const { t } = useTranslations()
const props = defineProps<{ items: Item[] }>()
const count = ref(0)
watch(/* ... */)
const emit = defineEmits<{ (e: 'complete'): void }>()
```

#### Component documentation

Above each component, add a comment block with:

- **Purpose**: One-line description (max 200 chars) of what the component does
- **Props**: List of props with types
- **Example**: Minimal usage snippet (how to use the component in a parent)

The format, blank lines, and structure below must be respected exactly. The Example shows template usage; use camelCase for prop bindings in templates (e.g. `:expandedPaths`, not `:expanded-paths`).

```vue
<!--
FolderNode

Purpose: Recursive tree node for folder/file display. Renders expandable rows with size.

Props: folder (FolderInfo), depth (number?), expandedPaths (Set<string>), formatBytes (fn), toggleExpand (fn)

Example:
   <FolderNode
      :folder="item"
      :depth="0"
      :expandedPaths="paths"
      :formatBytes="fmt"
      :toggleExpand="toggle"
   />
-->

<script setup lang="ts">
// component implementation...
</script>

<template>
   <!-- ... -->
</template>
```

#### Script logic documentation

When the `<script setup>` block contains **more than 2 declarations** (functions, watchers, computed properties, watchEffects, or any other logic blocks), each declaration must have a **JSDoc comment** explaining its purpose, design choices, or performance rationale.

This applies to all declarations in the script tag — not just exported functions. The goal is to make complex components self-documenting, especially when they contain performance-specific patterns (e.g. `shallowRef` vs `reactive`, `Map` clone-and-replace, virtual scrolling, `onActivated` workarounds for `KeepAlive`).

```vue
<script setup lang="ts">
// ✅ GOOD — each declaration is documented

/**
 * Checked-state map: Map<path, boolean>.
 *
 * Performance design choices:
 * - shallowRef(Map) instead of reactive(Map): every toggle replaces the whole
 *   Map reference, which triggers a single reactive notification.
 */
const checkedMapRef = shallowRef(new Map<string, boolean>())

/** Total size of currently checked items. Drives the button label and parent disk-usage bar. */
const selectedSize = computed(() => { /* ... */ })

/** Emits size changes to parent so the disk usage bar stays in sync. */
watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })

/**
 * Toggles a single item's checked state by cloning the Map and replacing the ref.
 * Clone-and-replace ensures Vue sees a new reference and triggers dependents.
 */
function toggle(path: string) { /* ... */ }
</script>
```

```vue
<script setup lang="ts">
// ❌ BAD — complex logic with no documentation

const checkedMapRef = shallowRef(new Map<string, boolean>())
const selectedSize = computed(() => { /* ... */ })
watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })
function toggle(path: string) { /* ... */ }
</script>
```

#### Template block comments

When the template contains **similar or repeated blocks** of elements (e.g. multiple `<section>`, groups of rows, repeated list items), add a short **HTML comment** above each block describing what that block represents. Use one line, concise wording (e.g. "Section name" or "List of X, Y, Z").

Example (from `SettingsView.vue`):

```vue
<template>
   <main class="SettingsView-root">
      <div v-else class="SettingsView-content">
         <!-- FDA -->

         <section class="SettingsGroup">
            <!-- ... FDA row, description, buttons ... -->
         </section>

         <!-- App Settings -->

         <section class="SettingsGroup">
            <!-- ... language select, theme select ... -->
         </section>

         <!-- Results -->

         <section class="SettingsGroup">
            <!-- ... toggles ... -->
         </section>
      </div>
   </main>
</template>
```

#### SVG icons

**Always add `aria-hidden="true"` to SVG elements used as icons.** This applies to:

- **Internal SVGs** — inline `<svg>` in templates or components
- **External icon packages** — e.g. Phosphor icons; ensure the rendered `<svg>` has `aria-hidden="true"` (via wrapper, component props, or package configuration)

Decorative icons should be hidden from assistive technologies so that screen readers don’t announce them; use visible text or `aria-label` on the parent control (e.g. button) when the icon alone isn’t sufficient.

```vue
<!-- ✅ GOOD -->
<svg aria-hidden="true" ...>...</svg>
<PhX size="16" weight="bold" aria-hidden="true" />

<!-- ❌ BAD — SVG exposed to assistive tech as redundant/confusing -->
<svg ...>...</svg>
<PhX size="16" weight="bold" />
```

#### Props and component usage

- **Prop definitions**: Always camelCase (e.g. `expandedPaths`, `formatBytes`)
- **Prop bindings in templates**: Use camelCase (e.g. `:expandedPaths`, `:formatBytes`) — JSX-like convention
- **Boolean props that are `true`**: Omit the value and pass the prop name only (JSX-like shorthand). Use `:prop="false"` when the value is `false`.
- **Component tags**: Use PascalCase (e.g. `<FolderNode />`, not `<folder-node />`)
- **Emits**: Use kebab-case (e.g. `emit('select-item')`, not `emit('selectItem')`)
- **Never use snake_case** for props or component names (e.g. `expanded_paths` is wrong)

```vue
<!-- ✅ GOOD -->
<ScanResultsNav showForward showActions :backDisabled="backStack.length === 0" />
<SomeForm :submitDisabled="false" />

<!-- ❌ BAD — redundant :prop="true" -->
<ScanResultsNav :showForward="true" :showActions="true" :backDisabled="backStack.length === 0" />
```

#### Reactivity and lifecycle

- Use `ref`, `computed`, `reactive`, etc. **only when strictly necessary** — prefer plain variables when reactivity is not needed
- **Avoid `computed`** when the value is not meant to be reactive
- Use `onMounted` **only when strictly required** — avoid it when the same can be achieved without lifecycle hooks

#### Template refs

- **Use `useTemplateRef(name)`** for refs that are bound in the template via `ref="name"` (DOM elements or component instances).
- Do not use `ref<HTMLElement | null>(null)` (or similar) for template refs — the string-based `useTemplateRef` aligns with Vue’s template ref resolution and avoids manual binding.

```vue
<script setup lang="ts">
import { useTemplateRef } from 'vue'

const mainContentRef = useTemplateRef<HTMLElement>('mainContentRef')
const resultsListRef = useTemplateRef<InstanceType<typeof ScanResultsList>>('resultsListRef')
</script>

<template>
  <div ref="mainContentRef">...</div>
  <ScanResultsList ref="resultsListRef" />
</template>
```

#### Imports

Sort imports in this order, with a **blank line between each group**:

1. **Components** (local `.vue` / component imports)
2. **External modules** (Vue first, then 3rd-party like `@tauri-apps/`)
3. **Internal modules** (`@/` paths or relative `./utils/...`)
4. **Constants** (`import { SETTINGS_KEY } from "@/stores/settings";`, `import { DEFAULT_SETTINGS } from "@/types/settings";`)
5. **Types** (`import type { SettingsStore } from "@/stores/settings";`, `import type { FolderInfo } from "@/types/structures";`)
6. **JSON** (`import Package from 'package.json'`)
7. **CSS** (`import '@/assets/css/reset.css'`)
8. **Other** (assets, etc.)

```ts
import FolderNode from '@/components/FolderNode.vue'

import { ref, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

import { formatBytes } from '@/lib/format'

import type { FolderInfo } from '@/types/structures'

import Package from 'package.json'

import '@/assets/css/reset.css'
```

#### Renaming components

When renaming a Vue component:

1. **Rename the file** (e.g. `FooterMenu.vue` → `AppFooter.vue`).
2. **Rename CSS classes** that start with the component (file) name so they still match the new filename (e.g. `FooterMenu-root` → `AppFooter-root`, `FooterMenu-btn` → `AppFooter-btn`). Use the same convention: `ComponentName-nestedElement`.
3. **Rename translations** in `src/assets/translations/` if the component has a dedicated translation file: rename the file (e.g. `FooterMenu.ts` → `AppFooter.ts`), the exported object name, and add the new module to `index.ts` (and remove the old one). Update all `t('OldName', 'key')` calls in the component to use the new module name.

### CSS

#### Class naming

- Format: `ComponentName-nestedElement`
- `ComponentName` must match the **filename** (e.g. `FolderNode.vue` → `FolderNode-item`, `FolderNode-row`, `FolderNode-children`)

```css
/* FolderNode.vue */
.FolderNode-item {
}
.FolderNode-row {
}
.FolderNode-arrow {
}
.FolderNode-children {
}
```

#### Nesting

Use **CSS nesting** — always. Add a **blank line** between selector declarations.

**Pseudo-selectors and state modifiers** (`:hover`, `:disabled`, `:focus`, etc.) must always be nested under the parent using `&`:

```css
/* ✅ GOOD */
.DeleteResults-cancelBtn {
   flex: 1;

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

/* ❌ BAD — flat pseudo-selectors */
.DeleteResults-cancelBtn {
   flex: 1;
}

.DeleteResults-cancelBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}
```

**Element tags** (e.g. `p`, `span`, `strong`) when styling children must be nested under the parent selector, not declared at root level:

```css
/* ✅ GOOD */
.ScanResults-stats {
   display: flex;
   flex-direction: column;

   p {
      display: flex;
   }

   span {
      color: var(--color-text-muted);
   }

   strong {
      max-width: 65%;
   }
}

/* ❌ BAD — element selectors at root */
.ScanResults-stats {
   display: flex;
   padding: var(--spacing-md);
}

.ScanResults-stats p {
   margin: 0;
}

.ScanResults-stats span {
   color: var(--color-text-muted);
}
```

**Never use `&--modifier`** to construct BEM modifier class names — this is SASS-style and does not belong in native CSS. Always write BEM modifier selectors as explicit, full class names at the top level:

```css
/* ✅ GOOD — explicit full class names */
.ListItem-check {
   cursor: pointer;
}

.ListItem-check--selected .ListItem-checkFilled {
   color: var(--color-accent);
}

.ListItem-check--disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

/* ❌ BAD — SASS-style modifier nesting */
.ListItem-check {
   cursor: pointer;

   &--selected {
      .ListItem-checkFilled {
         color: var(--color-accent);
      }
   }

   &--disabled {
      opacity: 0.5;
      cursor: not-allowed;
   }
}
```

#### Media queries

Media queries must be **nested inside the selector**, not at the root level. Never group multiple selectors under the same root-level media query:

```css
/* ✅ GOOD */
.card {
   padding: 1rem;

   @media (min-width: 768px) {
      padding: 1.5rem;
   }
}

/* ❌ BAD */
@media (min-width: 768px) {
   .card {
      padding: 1.5rem;
   }

   .sidebar {
      width: 200px;
   }
}
```

#### Variables

- Variable names: **kebab-case** (e.g. `--primary-color`, `--spacing-md`)
- If the same value appears **more than 2 times** in a component, move it to `theme.css`:
   - e.g. `12px` used for indentation in 3 places → `--tree-indent: 12px` in `theme.css`

### TypeScript / JavaScript

#### General

- Add comments **only when necessary**
- Prefer **function declarations** over `const fn = () => {}`
- **Prefer `interface`** over `type` when possible (see File organization for placement rules)

#### If statements

`if` (and `else`, `else if`) bodies must use curly brackets. The only exception is when the condition and the single statement fit on **one line**.

```ts
// ✅ GOOD — single line, no braces needed
if (bytes === 0) return '0 B'

// ✅ GOOD — multiple statements or multi-line: use braces
if (theme === ROOT_THEME) {
   document.documentElement.removeAttribute('data-theme')
} else {
   document.documentElement.setAttribute('data-theme', theme)
}

// ❌ BAD — body on next line without braces
if (theme === ROOT_THEME) document.documentElement.removeAttribute('data-theme')

// ❌ BAD — else without braces
if (x) doSomething()
else doOther()
```

#### Boolean expressions

**Avoid direct comparison with `false`** where the same intent can be expressed with `!`. Prefer `!value` over `value === false` (and likewise `value` over `value === true` when you only need truthiness).

```ts
// ✅ GOOD
:disabled="!selectable"
:aria-disabled="!selectable"

// ❌ BAD — unnecessary comparison with false
:disabled="selectable === false"
:aria-disabled="selectable === false"
```

Use `=== false` or `=== true` only when you must distinguish from other falsy/truthy values (e.g. `undefined`, `null`).

#### Variables

**Avoid declaring many variables just to shorten names** — prefer using the expression directly. Do not introduce `const s = x`, `const curr = y` etc. only to save a few characters.

```ts
// ❌ BAD — extra variables for shortening
const s = store.value
const curr = settings.value ?? s?.settings.value
if (s && curr) s.setShowHiddenFiles(!curr.showHiddenFiles)

// ✅ GOOD — use the expression directly
if (store.value && settings.value) store.value.setShowHiddenFiles(!settings.value.showHiddenFiles)
```

#### JSDoc

Add JSDoc to all functions **except** those that are: (a) defined and used only in the same file, and (b) less than 4 lines.

```ts
// ✅ JSDoc required — exported or 4+ lines
/** Formats bytes into human-readable string (e.g. "1.2 GB"). */
export function formatBytes(bytes: number): string {
   if (bytes === 0) return '0 B'
   const k = 1024
   const sizes = ['B', 'KB', 'MB', 'GB']
   const i = Math.floor(Math.log(bytes) / Math.log(k))
   return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`
}

// ✅ No JSDoc — same file only, < 4 lines
function clamp(n: number) {
   return Math.max(0, Math.min(1, n))
}
```

#### Object keys (Tauri / Rust boundary)

- **snake_case**: Objects crossing boundaries — Tauri commands, Rust structs, API payloads, `invoke()` responses
- **camelCase**: Objects used only in Vue/JS (component state, local variables, computed values)

```ts
// Tauri/Rust — must match Rust struct field names
interface FolderInfo {
   is_file: boolean
   children: FolderInfo[]
}
const data = await invoke<FolderInfo[]>('get_user_folders')

// Component-internal only
const localState = { expandedPaths: new Set(), currentFolder: '' }
const progress = { current: 0, total: 1, folder: '' }
```

### Rust

#### Imports

Sort imports in this order, with a **blank line between each group**:

1. **Framework crates** (e.g. `tauri`, `tauri::Manager`)
2. **Standard library** (`std::path`, `std::sync`)
3. **3rd-party crates** (e.g. `rayon`, `serde`)
4. **Crate-internal** (`crate::FolderInfo`, `super::scan`)

```rust
use tauri::Emitter;

use std::path::Path;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use crate::FolderInfo;
```
