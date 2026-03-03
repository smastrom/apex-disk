# Mac User Lens — Agent Guidelines

**Always follow these rules** when editing this codebase. This document is split into two sections:

1. **Project guidelines** — Platform target, file organization, project-specific implementation details (translations, theme, Tauri), workflow conventions.
2. **Code rules** — Language-specific style and patterns for Vue, CSS, TypeScript, and Rust.

### For AI agents

- **Read the relevant section** before making edits (e.g. Vue rules when editing `.vue` files).
- **"Do not"** = never do this. **"Avoid"** = prefer not to, but allowed in edge cases.
- **When in doubt**, prefer the simpler option (no extra files, no extra imports, no extra reactivity).

## 1. Project guidelines

### Target platform

The app targets **macOS only**. Do not add or retain platform-specific code or conditionals for Windows or Linux.

### File organization

#### File naming

- **`.ts` / `.js` files**: Always use **kebab-case** (e.g. `use-scan.ts`, `provide-settings-store.ts`, `format.ts`, `constants.ts`)
- **Exception — component-coupled files**: When a file is directly consumed by a single component and cannot exist without it (e.g. translation files), name it to match the component: **PascalCase** (e.g. `SettingsView.ts`, `AppFooter.ts`)
- **`.vue` files**: Always **PascalCase** (e.g. `ScanResultsList.vue`)

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

| File                            | Purpose                                              |
| ------------------------------- | ---------------------------------------------------- |
| `src/assets/css/theme.css`      | Variables (colors, spacing, etc.) and fonts          |
| `src/assets/css/global.css`     | Styles for `html`, `body`, and other global elements |
| `src/assets/css/reset.css`      | Style normalization                                  |
| `src/assets/css/classes.css`    | Reusable utility classes used across the project     |
| `src/assets/css/animations.css` | CSS animations and transitions                       |

Do not mix concerns between files.

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

1. Import `useTranslations` from `@/lib/use-translations`
2. Call `const { t } = useTranslations()`
3. Use `t(module, key)` or `t(module, key, vars)` for interpolation

```vue
<script setup lang="ts">
import { useTranslations } from '@/lib/use-translations'

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
const storeRef = useSettingsStore()

async function onThemeChange(theme: ThemeColor) {
   await storeRef.value?.setThemeColor(theme)
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

#### Skipped folders (credential directories)

Some directories contain irreplaceable credentials or security keys that should never appear in a disk cleanup tool. These are **completely excluded from scan results** — they are never scanned, never shown to the user, and their size is not counted.

##### Constants

- **Rust only**: `src-tauri/src/safe_folders.rs` — `SKIPPED_RELATIVE_PATHS`
- No frontend constant is needed since these paths never reach the UI.

Current list: `.ssh`, `.gnupg`, `.aws`, `.kube`

##### Implementation

- `safe_folders::is_path_skipped()` checks both exact matches and descendants (unlike protected paths, the entire subtree is excluded).
- The scanner skips these directories at both the top-level enumeration and recursive traversal in `scan.rs`.
- `delete.rs` also rejects skipped paths as a safety net, even though they should never reach the frontend.

##### Differences from protected folders

|                          | Protected | Skipped |
| ------------------------ | --------- | ------- |
| Appears in scan results  | Yes       | No      |
| Contents browsable       | Yes       | No      |
| Contents deletable       | Yes       | No      |
| Folder itself deletable  | No        | No      |
| Frontend constant needed | Yes       | No      |

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
- Example: instead of one `UserList.vue` with 200+ lines, split into `UserListHeader.vue`, `UserListFilters.vue`, `UserListTable.vue` — each with a single responsibility

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

#### Composables

Composables (`use*` functions in `src/lib/`) encapsulate reusable reactive logic. For deeper patterns and best practices, load the **`vue-best-practices`** skill.

##### When to extract a composable

1. **Global or shared logic** — If the logic is not scoped to a single component but rather affects the app globally or is reused across components, it belongs in a composable. Examples: store bootstrap (`provideAuthStore`), keyboard focus-ring detection (`useFocusRing`), translations (`useTranslations`).
2. **Grouped lifecycle and reactivity** — When multiple Vue APIs (`onMounted`, `onUnmounted`, `watch`, `ref`, `computed`) serve the **same purpose**, group them into a composable instead of scattering them across the component. For instance, if a feature needs `onMounted` to set up listeners, `onUnmounted` to tear them down, and a `watch` to react to changes — that's one composable, not three unrelated blocks in the component.

##### When to keep logic inline

If the logic is **scoped only to that component** and consists of simple, isolated statements (a single `ref`, a one-off `onMounted`), keep it in the component. Not every `ref` or lifecycle hook needs extraction — extract when there's a grouping or reuse benefit.

```ts
// ✅ GOOD — global concern extracted to composable
// src/lib/use-focus-ring.ts
export function useFocusRing() {
   onMounted(() => {
      document.addEventListener('keydown', onKeyDown, true)
   })
   onUnmounted(() => {
      document.removeEventListener('keydown', onKeyDown, true)
   })
}

// App.vue — clean, declarative
useFocusRing()
```

```ts
// ❌ BAD — global logic inlined in a component (or worse, in main.ts without lifecycle)
document.addEventListener('keydown', onKeyDown, true)
document.addEventListener('mousedown', onPointerDown, true)
```

##### Provide / inject pattern

For store-like state shared via `provide` / `inject`, use a paired naming convention:

- **`provideXxx()`** — called once in the root component to create and provide the state. This is a setup function, not a `use*` composable, because it is not meant to be injected or reused — it is the provider.
- **`useXxx()`** — called by any descendant component to inject and consume the state. This is the composable consumers import.

```ts
// src/stores/auth.ts — consumer composable
export function useAuthStore(): ShallowRef<AuthStore | null> {
   const store = inject<ShallowRef<AuthStore | null>>(AUTH_KEY)
   if (!store) throw new Error('useAuthStore() called without a provider.')
   return store
}

// src/lib/provide-auth-store.ts — provider setup
export function provideAuthStore() {
   const authStore = shallowRef<AuthStore | null>(null)
   provide(AUTH_KEY, authStore)
   // ... onMounted, watchers ...
   return { authStore, ready }
}
```

##### Naming and location

- Prefix with `use-` for composables, `provide-` for provider setup functions (see **File naming** for casing)
- Place in `src/lib/`, one composable per file

##### Composables that control DOM elements

When a composable controls **HTMLElements** (e.g. positioning a popover, measuring a node), **do not** create or return refs from inside the composable. The component owns the template and the elements; it must declare the refs (with `useTemplateRef`) and pass them in as parameters. The composable receives `Ref<HTMLElement | null>` (or similar) and uses them internally.

- **Component**: Declare `triggerRef` and `popoverRef` with `useTemplateRef` (before any composable call), bind them in the template, then call `useLabelPopover(triggerRef, popoverRef)`.
- **Composable**: Accept refs as parameters; return only state and handlers (e.g. `isOpen`, `onPointerEnter`, `onPointerLeave`).

This keeps ownership of DOM refs in the component, makes the composable easier to test and reuse, and aligns with the rule that template refs are declared in the component.

#### Component documentation

Above each component, add a comment block with:

- **Purpose**: One-line description (max 200 chars) of what the component does
- **Props**: List of props with types
- **Example**: Minimal usage snippet (how to use the component in a parent)

The format, blank lines, and structure below must be respected exactly. The Example shows template usage; use camelCase for prop bindings in templates (e.g. `:expandedPaths`, not `:expanded-paths`).

```vue
<!--
TreeNode

Purpose: Recursive tree node that renders expandable rows with label and size.

Props: node (TreeItem), depth (number?), expandedIds (Set<string>), toggle (fn)

Example:
   <TreeNode
      :node="item"
      :depth="0"
      :expandedIds="expanded"
      :toggle="onToggle"
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
 * Selection map: Map<id, boolean>.
 *
 * Uses shallowRef(Map) instead of reactive(Map): every toggle replaces the whole
 * Map reference, which triggers a single reactive notification.
 */
const selectionMap = shallowRef(new Map<string, boolean>())

/** Total cost of currently selected items. Drives the summary label. */
const totalCost = computed(() => {
   /* ... */
})

/** Emits cost changes to parent so the progress bar stays in sync. */
watch(totalCost, (cost) => emit('update:totalCost', cost), { immediate: true })

/**
 * Toggles a single item by cloning the Map and replacing the ref.
 * Clone-and-replace ensures Vue sees a new reference and triggers dependents.
 */
function toggle(id: string) {
   /* ... */
}
</script>
```

```vue
<script setup lang="ts">
// ❌ BAD — complex logic with no documentation

const selectionMap = shallowRef(new Map<string, boolean>())
const totalCost = computed(() => {
   /* ... */
})
watch(totalCost, (cost) => emit('update:totalCost', cost), { immediate: true })
function toggle(id: string) {
   /* ... */
}
</script>
```

#### Template block comments

When the template contains **similar or repeated blocks** of elements (e.g. multiple `<section>`, groups of rows, repeated list items), add a short **HTML comment** above each block describing what that block represents. Use one line, concise wording (e.g. "Section name" or "List of X, Y, Z").

```vue
<template>
   <main class="Settings-root">
      <div class="Settings-content">
         <!-- Account -->

         <section class="Settings-group">
            <!-- ... avatar, name, email ... -->
         </section>

         <!-- Preferences -->

         <section class="Settings-group">
            <!-- ... language select, theme select ... -->
         </section>

         <!-- Danger zone -->

         <section class="Settings-group">
            <!-- ... delete account button ... -->
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

- **Prop definitions**: Always camelCase (e.g. `userName`, `isActive`)
- **Prop bindings in templates**: Use camelCase (e.g. `:userName`, `:isActive`) — JSX-like convention
- **Boolean props that are `true`**: Omit the value and pass the prop name only (JSX-like shorthand). Use `:prop="false"` when the value is `false`.
- **Component tags**: Use PascalCase (e.g. `<TreeNode />`, not `<tree-node />`)
- **Emits**: Use kebab-case (e.g. `emit('select-item')`, not `emit('selectItem')`)
- **Never use snake_case** for props or component names (e.g. `user_name` is wrong)

```vue
<!-- ✅ GOOD -->
<NavBar showBack showActions :forwardDisabled="stack.length === 0" />
<LoginForm :submitDisabled="false" />

<!-- ❌ BAD — redundant :prop="true" -->
<NavBar :showBack="true" :showActions="true" :forwardDisabled="stack.length === 0" />
```

#### Reactivity and lifecycle

- Use `ref`, `computed`, `reactive`, etc. **only when strictly necessary** — prefer plain variables when reactivity is not needed
- **Avoid `computed`** when the value is not meant to be reactive
- Use `onMounted` **only when strictly required** — avoid it when the same can be achieved without lifecycle hooks

#### Template refs

- **Use `useTemplateRef(name)`** for refs that are bound in the template via `ref="name"` (DOM elements or component instances).
- Do not use `ref<HTMLElement | null>(null)` (or similar) for template refs — the string-based `useTemplateRef` aligns with Vue’s template ref resolution and avoids manual binding.
- **Declare all `useTemplateRef` calls before any composable or store call.** Template refs are Vue API declarations (like `defineProps`), not logic — they must not be scattered between composable calls.

```vue
<script setup lang="ts">
import { useTemplateRef } from 'vue'

const contentRef = useTemplateRef<HTMLElement>('contentRef')
const listRef = useTemplateRef<InstanceType<typeof ItemList>>('listRef')
</script>

<template>
   <div ref="contentRef">...</div>
   <ItemList ref="listRef" />
</template>
```

#### Imports

Sort imports in this order, with a **blank line between each group**:

1. **Components** (local `.vue` / component imports)
2. **External modules** (Vue first, then 3rd-party)
3. **Internal modules** (`@/` paths or relative `./utils/...`)
4. **Constants** (`import { API_URL } from "@/lib/constants"`)
5. **Types** (`import type { User } from "@/types/user"`)
6. **JSON** (`import Package from 'package.json'`)
7. **CSS** (`import '@/assets/css/reset.css'`)
8. **Other** (assets, etc.)

```ts
import TreeNode from '@/components/TreeNode.vue'

import { ref, onUnmounted } from 'vue'
import { someLib } from 'some-lib'

import { formatDate } from '@/lib/format'

import type { User } from '@/types/user'

import Package from 'package.json'

import '@/assets/css/reset.css'
```

#### Renaming components

When renaming a Vue component:

1. **Rename the file** (e.g. `OldName.vue` → `NewName.vue`).
2. **Rename CSS classes** that start with the component name so they match the new filename (e.g. `OldName-root` → `NewName-root`, `OldName-btn` → `NewName-btn`). Use the same convention: `ComponentName-nestedElement`.
3. **Rename translations** if the component has a dedicated translation file: rename the file, the exported object, and update `index.ts`. Update all `t('OldName', 'key')` calls to use the new module name.

### CSS

#### Class naming

- Format: `ComponentName-nestedElement`
- `ComponentName` must match the **filename** (e.g. `UserCard.vue` → `UserCard-root`, `UserCard-avatar`, `UserCard-name`)

```css
/* UserCard.vue */
.UserCard-root {
}
.UserCard-avatar {
}
.UserCard-name {
}
.UserCard-actions {
}
```

#### Nesting

Use **CSS nesting** — always. Add a **blank line** between selector declarations.

**Pseudo-selectors and state modifiers** (`:hover`, `:disabled`, `:focus`, etc.) must always be nested under the parent using `&`:

```css
/* ✅ GOOD */
.Dialog-cancelBtn {
   flex: 1;

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

/* ❌ BAD — flat pseudo-selectors */
.Dialog-cancelBtn {
   flex: 1;
}

.Dialog-cancelBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}
```

**Element tags** (e.g. `p`, `span`, `strong`) when styling children must be nested under the parent selector, not declared at root level:

```css
/* ✅ GOOD */
.Card-stats {
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
.Card-stats {
   display: flex;
   padding: var(--spacing-md);
}

.Card-stats p {
   margin: 0;
}

.Card-stats span {
   color: var(--color-text-muted);
}
```

**Never use `&--modifier`** to construct BEM modifier class names — this is SASS-style and does not belong in native CSS. Always write BEM modifier selectors as explicit, full class names at the top level:

```css
/* ✅ GOOD — explicit full class names */
.Checkbox-icon {
   cursor: pointer;
}

.Checkbox-icon--selected .Checkbox-fill {
   color: var(--color-accent);
}

.Checkbox-icon--disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

/* ❌ BAD — SASS-style modifier nesting */
.Checkbox-icon {
   cursor: pointer;

   &--selected {
      .Checkbox-fill {
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

#### Blank lines in code

Applies to **any block** (function body, module top-level, callback, loop body, etc.). Use a single blank line only when **switching from one logical group to another**. Do **not** add a blank between consecutive same-type, same-scope one-line statements.

- **No blank** between: multiple `const`/`let`, multiple function or method calls, multiple property assignments — when they form one logical group.
- **Add a blank** when switching groups: (1) between a declaration block and the next group (guard, calls, or mutations); (2) after an early return (or guard) before the next group; (3) before a final return. So in a block that has declarations, then a guard, then a return: blank after declarations, blank after the guard, then the return.

```ts
// ✅ GOOD — one group of declarations, blank, then group of actions
const el = containerRef.value
const offset = getScrollOffset(el)

el?.classList.add('transitioning')
await animate(el, offset)
el?.classList.remove('transitioning')
```

```ts
// ✅ GOOD — same-type one-liners stay together; blank only at group boundaries
export function formatBytes(bytes: number): string {
   if (bytes === 0) return '0 B'

   const k = 1024
   const sizes = ['B', 'KB', 'MB', 'GB']
   const i = Math.floor(Math.log(bytes) / Math.log(k))

   return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`
}
```

```ts
// ✅ GOOD — top-level: init group, blank, then app creation group
const store = await createSettingsStore()
applyTheme(store.getThemeColor())
applyDirection(store.settings.value.language)

const app = createApp(App)
app.provide(SETTINGS_KEY, store)
app.mount('#app')
```

```ts
// ❌ BAD — no blank between groups (declarations vs actions)
const el = containerRef.value
const offset = getScrollOffset(el)
el?.classList.add('transitioning')
```

```ts
// ✅ GOOD — declarations, blank, guard, blank, final return (e.g. inside computed/callback)
const newFreeSpace = computed(() => {
   const u = usage.value
   const sel = props.selectedSize ?? 0

   if (!u || sel === 0) return null

   return u.free + sel
})
```

```ts
// ❌ BAD — blank inside the same group (const/const)
const k = 1024

const sizes = ['B', 'KB', 'MB', 'GB']
const i = Math.floor(Math.log(bytes) / Math.log(k))
```

```ts
// ❌ BAD — no blank after declarations or after guard (computed/callback)
const newFreeSpace = computed(() => {
   const u = usage.value
   const sel = props.selectedSize ?? 0
   if (!u || sel === 0) return null
   return u.free + sel
})
```

#### If statements

`if` (and `else`, `else if`) bodies must use curly brackets. The only exception is when the condition and the single statement fit on **one line**.

```ts
// ✅ GOOD — single line, no braces needed
if (bytes === 0) return '0 B'

// ✅ GOOD — multiple statements or multi-line: use braces
if (mode === 'dark') {
   document.documentElement.classList.add('dark')
} else {
   document.documentElement.classList.remove('dark')
}

// ❌ BAD — body on next line without braces
if (mode === 'dark')
   // prettier-ignore
   document.documentElement.classList.add('dark')

// ❌ BAD — else without braces
if (x) doSomething()
else doOther()
```

#### Boolean variable and prop names

Every boolean object property, Vue prop, or variable must be named with a **leading verb** so the meaning is clear at the call site. Prefer `is*`, `has*`, or `can*` (e.g. `isActive`, `isLoaded`, `hasPermissions`, `canEdit`).

- **Good**: `isSelected`, `isLoading`, `isFdaGranted`, `hasSomeSelected`, `isBackDisabled`
- **Bad**: `selected`, `loading`, `active`, `disabled`, `granted` (noun/adjective alone)

Apply to: component props, reactive refs, object properties, and local variables that hold a boolean.

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
const u = user.value ?? s?.user.value
if (s && u) s.setName(!u.name)

// ✅ GOOD — use the expression directly
if (store.value && user.value) store.value.setName(!user.value.name)
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
interface FileEntry {
   is_directory: boolean
   file_size: number
}
const data = await invoke<FileEntry[]>('list_files')

// Component-internal only
const localState = { selectedIds: new Set(), currentPage: 1 }
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
