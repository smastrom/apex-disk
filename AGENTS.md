# Mac Disk Lens — Agent Guidelines

This document defines code style and conventions. **Always follow these rules** when editing this codebase.

## For AI agents

- **Read the relevant section** before making edits (e.g. Vue rules when editing `.vue` files).
- **"Do not"** = never do this. **"Avoid"** = prefer not to, but allowed in edge cases.
- **When in doubt**, prefer the simpler option (no extra files, no extra imports, no extra reactivity).

---

## File creation

### Do not create

- **Do not create tests** unless explicitly requested
- **Do not create documentation** unless explicitly requested

### Types

- **Single-file types**: Define in the same file (e.g. `ScanProgress` only used in `App.vue`)
- **Shared types**: Define in `src/types/` — e.g. `FolderInfo` used by `App.vue`, `FolderNode.vue`, and Rust → `src/types/structures.ts`

### Functions

- Before adding a function, **check `src/lib/`** — it may already exist (e.g. `formatBytes` in `format.ts`)
- If the function **might be used outside** the current file, define it in `src/lib/`:
  - **Generic utilities** → `src/lib/utils.ts` (e.g. `isClient`, `noop`)
  - **Domain-specific** → new file (e.g. `src/lib/format.ts` for `formatBytes`, `formatDuration`)
- **Do not hesitate to create new files** in `src/lib/` when a function is reusable but not generic enough for `utils.ts`

### Vue components

- **Prefer splitting big components** into smaller ones using a "blocks" logic: each sub-component handles a distinct UI block
- Example: instead of one `ScanView.vue` with 200+ lines, split into `ScanViewHeader.vue`, `ScanViewProgress.vue`, `ScanViewTree.vue`—each with a single responsibility

### CSS files

| File                         | Purpose                                              |
| ---------------------------- | ---------------------------------------------------- |
| `src/assets/css/theme.css`   | Variables (colors, spacing, etc.) and fonts          |
| `src/assets/css/global.css`  | Styles for `html`, `body`, and other global elements |
| `src/assets/css/reset.css`   | Style normalization                                  |
| `src/assets/css/classes.css` | Reusable utility classes used across the project     |

**These rules are mandatory.** Do not mix concerns between files.

---

## Code style

### Package / development

- **Use pnpm** as package manager.
- **Do not propose to launch dev servers** when testing implementations. The dev server is always running when the agent is working for the human.
- **Do not install any npm package** unless explicitly asked.

---

### Vue

- **Always use `<script setup lang="ts">`** — no Options API, no plain `<script>`.

#### Component documentation

Above each component, add a comment block with:

- **Purpose**: One-line description (max 200 chars) of what the component does
- **Props**: List of props with types
- **Example**: Minimal usage snippet (how to use the component in a parent)

The format, blank lines, and structure below must be respected exactly. The Example shows template usage; use kebab-case for prop bindings in templates (e.g. `:expanded-paths`, not `:expandedPaths`).

```vue
<!--
FolderNode

Purpose: Recursive tree node for folder/file display. Renders expandable rows with size.

Props: folder (FolderInfo), depth (number?), expandedPaths (Set<string>), formatBytes (fn), toggleExpand (fn)

Example:
   <FolderNode
      :folder="item"
      :depth="0"
      :expanded-paths="paths"
      :format-bytes="fmt"
      :toggle-expand="toggle"
   />
-->

<script setup lang="ts">
// component implementation...
</script>

<template>
  <!-- ... -->
</template>
```

#### Reactivity

- Use `ref`, `computed`, `reactive`, etc. **only when strictly necessary**
- **Avoid `computed`** when the value is not meant to be reactive
- Prefer plain variables when reactivity is not needed

#### Lifecycle

- Use `onMounted` **only when strictly required**
- Avoid it when the same can be achieved without lifecycle hooks

#### Class naming

- Format: `ComponentName-nestedElement`
- `ComponentName` must equal the **filename** (e.g. `FolderNode.vue` → `FolderNode-item`, `FolderNode-row`)

#### Props and component usage

- **Prop definitions**: Always camelCase (e.g. `expandedPaths`, `formatBytes`)
- **Prop bindings in templates**: Use kebab-case (e.g. `:expanded-paths`, `:format-bytes`) — matches Vue/HTML convention
- **Component tags**: Use PascalCase (e.g. `<FolderNode />`, not `<folder-node />`)
- **Never use snake_case** for props or component names (e.g. `expanded_paths` is wrong)

---

### Imports (Vue & JS/TS)

Sort imports in this order, with a **blank line between each group**:

1. **Components** (local `.vue` / component imports)
2. **External modules** (Vue first, then 3rd-party like `@tauri-apps/`)
3. **Internal modules** (`@/` paths or relative `./utils/...`)
4. **Other** (JSON, assets, CSS)

```ts
import FolderNode from "@/components/FolderNode.vue";

import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

import { formatBytes } from "@/lib/format";
import type { FolderInfo } from "@/types/structures";

import "@/assets/css/reset.css";
```

---

### JavaScript / TypeScript

- Add comments **only when necessary**
- Prefer **function declarations** over `const fn = () => {}`
- **Prefer `interface`** over `type` when possible (see File creation for placement rules)

#### JSDoc

Add JSDoc to all functions **except** those that are: (a) defined and used only in the same file, and (b) less than 4 lines.

```ts
// ✅ JSDoc required — exported or 4+ lines
/** Formats bytes into human-readable string (e.g. "1.2 GB"). */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

// ✅ No JSDoc — same file only, < 4 lines
function clamp(n: number) {
  return Math.max(0, Math.min(1, n));
}
```

---

### Object keys (Vue & JS/TS)

- **snake_case**: Objects crossing boundaries — Tauri commands, Rust structs, API payloads, `invoke()` responses
- **camelCase**: Objects used only in Vue/JS (component state, local variables, computed values)

```ts
// Tauri/Rust — must match Rust struct field names
interface FolderInfo {
  is_file: boolean;
  children: FolderInfo[];
}
const data = await invoke<FolderInfo[]>("get_user_folders");

// Component-internal only
const localState = { expandedPaths: new Set(), currentFolder: "" };
const progress = { current: 0, total: 1, folder: "" };
```

---

### CSS

#### General rules

- Add a **blank line** between any selector declaration
- Use **CSS nesting**
- **Never** nest different selectors under the same root-level media query
- Media queries must be **nested under the selector**, not at the root

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

#### Variables

- Variable names: **kebab-case** (e.g. `--primary-color`, `--spacing-md`)
- If the same value appears **more than 2 times** in a component, move it to `theme.css`:
  - e.g. `12px` used for indentation in 3 places → `--tree-indent: 12px` in `theme.css`

---

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

---
