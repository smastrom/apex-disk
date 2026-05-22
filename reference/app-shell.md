# App shell

Keywords: AppLayout, useAppViews, transitions, KeepAlive, popover, floating-ui,
focus ring, scrollbars, FDA, ui components, use-label-popover,
use-list-row-popovers, WebDriver.

Cross-cutting shell wiring, transition rules, and shared presentational
primitives. Per-feature flows live elsewhere: scan/trash state in
`[scan-trash-flow.md](scan-trash-flow.md)`, code style in
`[code-style.md](code-style.md)`, theming in `[themes.md](themes.md)`.

## Shell wiring (`AppLayout`)

`AppLayout.vue` wires the top-level shell once at bootstrap:

| Composable / call            | Role                                                                                   |
| ---------------------------- | -------------------------------------------------------------------------------------- |
| `useAppViews()`              | Footer-driven view switch (`scan` / `settings` / `information`), slide animation state |
| `useScanner()`               | Scan lifecycle shared with `ScanView`                                                  |
| `setupFocusRing()`           | Keyboard-only focus ring on `<html>`                                                   |
| `disableNativeContextMenu()` | Blocks WKWebView Reload menu in production (allows Copy on text selection)             |
| `useAppUpdate()`             | Silent/manual update orchestration                                                     |
| Theme/direction watchers     | `applyTheme`, `applyDirection` from `@/lib/dom`                                        |

Views render from `viewStates` (`active` / `leaving` / `entering` / `hidden`).
Outer shell transitions take **272 ms** (`use-app-views.ts`).

## Transitions and `<KeepAlive>`

Three patterns; pick by scope:

| Pattern                                                  | Use for                                                | Notes                                                                                                                                                                 |
| -------------------------------------------------------- | ------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Vanilla CSS class swaps driven by a JS state machine     | Outer shell only (`scan` / `settings` / `information`) | Wrapping the outer shell in `<KeepAlive>` + `<Transition>` previously crashed when patches landed on cached subtrees mid-removal. See `use-app-views.ts` doc comment. |
| Vue `<Transition>` (+ `<KeepAlive>` when state persists) | Inner view swaps inside a feature                      | `ScanView` uses `name="fade" mode="out-in"` around its `<KeepAlive>` view stack. Small caption swaps follow the same pattern.                                         |
| `onActivated` / `onDeactivated` hooks                    | Any component hosted under a `<KeepAlive>`             | Restart timers, re-sync derived state, reset transient flags. `ScanTrashList` is the canonical example.                                                               |

Inner list slides (folder nav in `ScanResultsList`) set
`mode="out-in"` only for manual navigation; programmatic/KeepAlive swaps drop
the mode so the scroll container is never empty.

### WebDriver / E2E

When `isWebDriverSession` is true (`@/lib/utils`), `useAppViews` **skips**
outer view animations and snaps instantly. E2E helpers in
`[e2e/helpers/navigation.ts](../e2e/helpers/navigation.ts)` patch rAF and
disable CSS transitions for inner list slides. Do not rely on animation timing
in specs.

## Focus ring (Safari 13 portable)

No `:focus-visible` dependency for the primary ring behavior.

- `setupFocusRing()` in `AppLayout.vue` toggles `focus-ring-keyboard` on
  `<html>` after keydown, removes it on pointer down.
- Styles live in `src/assets/css/global.css` under that class.

Call `setupFocusRing()` only from the root layout, not per component.

## Popovers (not the Popover API)

Safari 13 has no Popover API. Tooltips use `**@floating-ui/dom`\*\*
(`computePosition`, `flip`, `offset`, `shift`) and a teleported element with
class `is-open`.

| Composable                                                              | Use when                                                                                                                                                       |
| ----------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useLabelPopover(triggerRef, popoverRef, opts?)`                        | Single trigger (e.g. truncated label on one control). Wrap popover in `<Teleport to="body">`. Shows only when `scrollWidth > clientWidth` unless `alwaysShow`. |
| `useListRowPopovers(rootRef, namePopoverRef, checkboxPopoverRef, opts)` | **One pair of popovers for an entire list**, event-delegated on the list root. Used by `ScanResultsList`.                                                      |

Shared timing: **400 ms** enter delay, **200 ms** leave delay. Scroll on the
nearest scrollable ancestor dismisses the popover.

### List row opt-in (`useListRowPopovers`)

Rows expose hooks via classes/attributes (do not add per-row popover instances):

| Hook                                                            | Behavior                                    |
| --------------------------------------------------------------- | ------------------------------------------- |
| `.ScanResultsListItem-name`                                     | Truncation tooltip; text from `textContent` |
| `.ScanResultsListItem-check[data-disabled-tooltip="fda"]`       | FDA-required tooltip copy                   |
| `.ScanResultsListItem-check[data-disabled-tooltip="protected"]` | Protected-folder tooltip copy               |

Pass `resolveCheckboxText(kind)` for the disabled-checkbox strings (from
translations).

## Reduced motion

- `useReducedMotion()` — reactive `prefers-reduced-motion: reduce`.
- Global overrides in `src/assets/css/animations.css` under
  `@media (prefers-reduced-motion: reduce)`.
- Theme switches are attribute swaps with no transition dependency.

## Custom scrollbars

`useScrollbarVisibility(elementRef, mode, opts?)` toggles
`data-scrollbar-active="true"` on a scroll container. Modes:

- `scroll-and-hover` — thumb while scrolling or gutter hovered
- `hover-only` — thumb on gutter hover only

Painting rules are in `global.css`. Use activation hooks (`onActivated` /
`onDeactivated`) when the scroll host lives inside `<KeepAlive>`.

## Full Disk Access

- Rust: `check_full_disk_access` (`permissions.rs`).
- Vue: `useFullDiskAccess()` in bootstrap; result passed to `AppLayout` /
  `ScanView` as `isFdaGranted`.
- Rows that need FDA show `data-disabled-tooltip="fda"` on the checkbox;
  do not re-check FDA in the UI beyond the prop.

## Presentational primitives (`src/components/ui/`)

Reuse before adding new markup:

| Component                                             | Role                                        |
| ----------------------------------------------------- | ------------------------------------------- |
| `Spinner.vue`                                         | Indeterminate loading                       |
| `SelectionIcon.vue` / `CheckboxIcon.vue`              | Checkbox visuals (empty / partial / filled) |
| `AnimatedCheckCircle.vue` / `AnimatedAlertCircle.vue` | Trash confirmation states                   |
| `Logo.vue`                                            | App mark                                    |

These are dumb presentational components: no store access, no `invoke`.
Feature components (`Scan*.vue`, `SettingsView.vue`) own behavior and IPC.

## Settings and store access

- Persisted settings: always through `useAppSettings()` from
  `@/stores/app-settings.ts`. No `provide` / `inject`.
- See `[tauri-commands.md](tauri-commands.md)` for the Rust merge-on-read flow.

## Module index

| Location                              | What                                       |
| ------------------------------------- | ------------------------------------------ |
| `src/components/AppLayout.vue`        | Shell wiring (focus ring, views, scanner)  |
| `src/lib/use-app-views.ts`            | View state machine + WebDriver snap        |
| `src/lib/use-focus-ring.ts`           | Keyboard focus ring                        |
| `src/lib/use-context-menu.ts`         | Suppress Reload context menu               |
| `src/lib/use-label-popover.ts`        | Single-element floating tooltip            |
| `src/lib/use-list-row-popovers.ts`    | Delegated list row tooltips                |
| `src/lib/use-reduced-motion.ts`       | `prefers-reduced-motion` ref               |
| `src/lib/use-scrollbar-visibility.ts` | Custom scrollbar visibility                |
| `src/lib/use-full-disk-access.ts`     | FDA invoke on bootstrap                    |
| `src/components/ui/`                  | Shared presentational widgets              |
| `src/assets/css/animations.css`       | `list-slide`, shell slides, reduced motion |
| `src/assets/css/global.css`           | Focus ring, scrollbar thumb styles         |
