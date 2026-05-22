# E2E conventions

Keywords: data-testid, sel, webdriver, fixture, MyData, resetE2eState, wdio.

Agent-facing contract for WebdriverIO specs. Run instructions and suite
overview also live in [`e2e/README.md`](../e2e/README.md). Rust/E2E feature
flags: [`testing.md`](testing.md).

## Principles

- Drive a **real** debug Tauri build with `--features e2e`. No frontend mocks.
- Fixture home is **`e2e_fixtures.rs`** tempdir, not the user's home.
- Prefer helpers in [`e2e/helpers/navigation.ts`](../e2e/helpers/navigation.ts)
  over ad-hoc selectors or clicks.
- After mutating settings or scan state, call `resetE2eState()` or
  `resetAndScan()` from helpers.

## Selector registry

**Single source of truth:** export `sel` in `e2e/helpers/navigation.ts`.
Add new selectors there first, then `data-testid` in Vue, then use `sel.*`
in specs.

Do not scatter raw `[data-testid="…"]` strings in specs when `sel` already
exports the selector.

### Naming rules

| Pattern                        | Example                                 | Use for                               |
| ------------------------------ | --------------------------------------- | ------------------------------------- |
| kebab-case, feature prefix     | `scan-launch`, `start-scan`             | Top-level regions and primary actions |
| `results-*`                    | `results-list`, `results-row-folder`    | Scan results surface                  |
| `nav-*`                        | `nav-back`, `nav-path-label`            | Folder tree navigation                |
| `trash-*`                      | `trash-list`, `trash-list-row-checkbox` | Trash review flow                     |
| `footer-*`                     | `footer-scan`, `footer-settings`        | App shell tabs                        |
| `settings-*` / `information-*` | `settings-view`, `information-view`     | Secondary views                       |

Settings toggles use **`aria-labelledby="label-…"`** selectors in `sel`
(not `data-testid`) because the toggle component exposes stable label ids.

### Current `data-testid` map (Vue → spec)

| `data-testid`                                                                | Component / area            |
| ---------------------------------------------------------------------------- | --------------------------- |
| `app-header`                                                                 | `AppHeader.vue`             |
| `footer-scan`, `footer-settings`, `footer-information`, `footer-scan-dot`    | `AppFooter.vue`             |
| `scan-launch`, `start-scan`                                                  | `ScanLaunch.vue`            |
| `scanning-results`, `scan-abort`                                             | `ScanProgress.vue`          |
| `results-list`, `review-selection`, `results-truncated`                      | `ScanResultsList.vue`       |
| `results-row-folder` / `results-row-file`, `results-row-checkbox`            | `ScanResultsListItem.vue`   |
| `results-nav`, `nav-back`, `nav-forward`, `nav-path-label`, `results-cancel` | `ScanListNav.vue`           |
| `trash-list`, `confirm-trash`                                                | `ScanTrashList.vue`         |
| `trash-list-row`, `trash-list-row-checkbox`                                  | `ScanTrashListItem.vue`     |
| `restart`                                                                    | `ScanTrashConfirmation.vue` |
| `settings-view`, `settings-content`                                          | `SettingsView.vue`          |
| `information-view`                                                           | `InformationView.vue`       |

When you rename, add, or remove a `data-testid`, update **`sel`**, the
Vue template, and any spec that asserted on the old id in the same change.

## Fixture names (stable strings)

Specs locate rows by **displayed folder/file name** (`requireRowByName`,
`getRowByName`). These names come from `src-tauri/src/e2e_fixtures.rs`:

| Name             | Role in tests                                              |
| ---------------- | ---------------------------------------------------------- |
| `MyData`         | Normal selectable folder; nested `SubFolder` / `Deep`      |
| `Documents`      | Protected folder (checkbox disabled)                       |
| `Projects`       | Normal folder; `Bulk` subfolder triggers truncation notice |
| `.ssh`, `.Trash` | Skipped; must **not** appear in results                    |

File fixtures inside `MyData`: `big.txt`, `small.txt`, `empty.txt`, `.hidden`,
`.DS_Store`. Changing fixture layout requires updating **`e2e_fixtures.rs`**,
this table, [`e2e/README.md`](../e2e/README.md), and affected specs.

## Interaction gotchas

| Do                                                    | Don't                                                        |
| ----------------------------------------------------- | ------------------------------------------------------------ |
| Click **checkbox** (`results-row-checkbox`) to select | Click folder row body to select (row click navigates in)     |
| `waitForListSlideSettled()` after folder nav          | Assert row absence during Vue `list-slide` transition        |
| `resetE2eState()` when tests change settings          | Replay many toggle clicks to reset (flakes view transitions) |
| Use `executeAsync` for Tauri `invoke` in WDIO         | `execute()` with async invoke (cannot serialize Promise)     |

Outer footer view switches snap under WebDriver; inner scan list transitions
still run unless the helper patches motion (see `patchRequestAnimationFrame`).

## Test-only commands

Behind `#[cfg(feature = "e2e")]` only. Update **both** `generate_handler!`
blocks in `lib.rs` when adding production commands.

| Command              | Helper                                         | Purpose                                   |
| -------------------- | ---------------------------------------------- | ----------------------------------------- |
| `reset_e2e_state`    | `resetE2eState()`                              | Default settings + frontend store refresh |
| `set_e2e_trash_mode` | `setTrashMode('success' \| 'zero' \| 'error')` | Dry-run trash                             |

Release/beta builds must not ship these symbols
(`scripts/verify-no-e2e-symbols.sh`).

## Spec files

| Spec                         | Focus                                            |
| ---------------------------- | ------------------------------------------------ |
| `app-navigation.spec.ts`     | Footer tabs, Information footer                  |
| `scan-flow.spec.ts`          | Scan, sort, protected rows, truncation, scan dot |
| `selection-checkbox.spec.ts` | Selection, partial state, protected behavior     |
| `settings-flow.spec.ts`      | Settings persistence and scan filters            |
| `trash-review.spec.ts`       | Review list, countdown, dry-run trash            |

## Module index

| Location                                                    | What                                 |
| ----------------------------------------------------------- | ------------------------------------ |
| [`e2e/helpers/navigation.ts`](../e2e/helpers/navigation.ts) | `sel`, waits, scan/nav/trash helpers |
| [`e2e/specs/`](../e2e/specs/)                               | WDIO scenarios                       |
| [`e2e/wdio.conf.ts`](../e2e/wdio.conf.ts)                   | Debug build + session lifecycle      |
| `src-tauri/src/e2e_fixtures.rs`                             | Temp home fixture tree               |
| [`e2e/README.md`](../e2e/README.md)                         | How to run, runtime model            |
