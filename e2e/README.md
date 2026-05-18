# E2E Testing Plan for ApexDisk

## Context

ApexDisk is a Tauri v2 macOS disk analyzer (Vue 3 + Rust). The existing e2e suite only covers footer navigation. We need comprehensive e2e tests for the core flows: scanning, checkbox selection (complex 4-state model with parent/child relationships), trash review (including error/0KB case), and settings. The main challenge is **determinism**: the app scans the real home directory, which varies per machine and CI runner.

---

## Strategy: Rust-Side Mock via the Existing `e2e` Feature Flag

Mock at the Rust layer (combined into the existing `e2e` flag) so the full IPC serialization path (Rust → serde → Tauri IPC → TypeScript) is exercised. The existing `create_test_home()` fixture from `src-tauri/tests/support/mod.rs` already builds a realistic temp directory, which we **enhance and reuse** at runtime. Every e2e build uses fixture data.

### Why not frontend mocking?

- Misses serialization bugs between Rust structs and TS interfaces
- Requires polluting production frontend code with mock wiring
- Doesn't test the actual Tauri command handlers

---

## Phase 1: Foundation (Rust + Config Changes)

### 1.1 Add `tempfile` to `e2e` feature

**File:** `src-tauri/Cargo.toml`

- Expand feature: `e2e = ["tauri-plugin-webdriver", "dep:tempfile"]`
- Move `tempfile` from `[dev-dependencies]` to `[dependencies]` as optional

### 1.2 Extract and enhance fixture into runtime module

**New file:** `src-tauri/src/e2e_fixtures.rs` (gated `#[cfg(feature = "e2e")]`)

- Copy `create_test_home()` and `write_file()` from `tests/support/mod.rs`
- Use `LazyLock<TempDir>` so the fixture is created once per process and lives until exit
- **Enhance the fixture** with deeper nesting required for checkbox tests (see 1.2a below)

#### 1.2a Enhanced Fixture Structure

The current fixture is too flat to test the `explodeAncestorExcluding` logic (which walks down from ancestor to excluded child, selecting siblings at each level). We need **nested subdirectories** under `MyData`:

```
MyData/                          # Normal folder, selectable
├── big.txt          (2048 B)    # ≥ 1KB file
├── small.txt        (100 B)     # < 1KB file
├── empty.txt        (0 B)       # Zero-byte file
├── .hidden          (50 B)      # Hidden file
├── SubFolder/                   # Nested subfolder for explode tests
│   ├── alpha.txt    (1024 B)    # File in subfolder
│   ├── beta.txt     (512 B)     # File in subfolder
│   └── Deep/                    # 3-level nesting for multi-level explode
│       └── gamma.txt (256 B)    # Deep nested file
Documents/                       # Protected folder
├── report.txt       (2048 B)
├── note.txt         (500 B)
Projects/                        # Normal folder, selectable
├── app              (5120 B)
├── src/                         # Subfolder for selection tests
│   └── main.rs      (1024 B)
```

This enables testing:

- **Single-level explode:** Select MyData → navigate in → deselect big.txt → siblings (small.txt, SubFolder, etc.) get selected
- **Multi-level explode:** Select MyData → navigate into SubFolder → deselect alpha.txt → beta.txt and Deep/ get selected, plus MyData's direct children (big.txt, etc.) also get individually selected
- **Deep nesting:** Select MyData → navigate to SubFolder/Deep → deselect gamma.txt → all intermediate siblings selected at each level
- **Multiple folder navigation:** Verify indeterminate state propagates up through multiple levels

### 1.3 Redirect home dir in scan

**File:** `src-tauri/src/scan.rs` (line 409)

```rust
#[cfg(feature = "e2e")]
let user_dir = crate::e2e_fixtures::test_home_path();
#[cfg(not(feature = "e2e"))]
let user_dir = dirs::home_dir().ok_or("Unable to determine user directory")?;
```

### 1.4 Mock FDA check

**File:** `src-tauri/src/permissions.rs`

- Under `e2e`, `is_full_disk_access_granted()` reads `E2E_FDA` env var (defaults to `false`)

### 1.5 Mock trash_paths (configurable)

**File:** `src-tauri/src/trash.rs`

- Under `e2e`, return mock result controlled by `E2E_TRASH_RESULT` env var:
   - Default / `"success"`: returns `{ count: <actual count>, size: <actual size> }` (mirrors real behavior)
   - `"zero"`: returns `{ count: 0, size: 0 }` (simulates all items failing to delete)
   - `"error"`: returns `Err(...)` (simulates invoke failure)
- No actual file moves in any case

### 1.6 Add `reset_e2e_state` command

**File:** `src-tauri/src/store.rs` + `src-tauri/src/lib.rs`

- New command (gated `e2e`) that resets settings to defaults
- Called from tests via `browser.execute(() => window.__TAURI__.core.invoke('reset_e2e_state'))`

### 1.7 Update WDIO config

**File:** `e2e/wdio.conf.ts`

- Pass `E2E_FDA` and `E2E_TRASH_RESULT` env vars to spawned app process

### 1.8 Update CI workflow

**File:** `.github/workflows/tests.yml`

- Add `E2E_FDA: 'false'` env var to e2e step

---

## Phase 2: Test Suites

### Known Test Data (enhanced fixture)

| Path                              | Type   | Size   | Properties          |
| --------------------------------- | ------ | ------ | ------------------- |
| **MyData/**                       | folder | -      | Normal, selectable  |
| `MyData/big.txt`                  | file   | 2048 B | Normal, ≥ 1KB       |
| `MyData/small.txt`                | file   | 100 B  | < 1KB               |
| `MyData/empty.txt`                | file   | 0 B    | Zero-byte           |
| `MyData/.hidden`                  | file   | 50 B   | Hidden              |
| **MyData/SubFolder/**             | folder | -      | Nested, selectable  |
| `MyData/SubFolder/alpha.txt`      | file   | 1024 B | Normal              |
| `MyData/SubFolder/beta.txt`       | file   | 512 B  | Normal              |
| **MyData/SubFolder/Deep/**        | folder | -      | 3rd level nesting   |
| `MyData/SubFolder/Deep/gamma.txt` | file   | 256 B  | Normal              |
| **Documents/**                    | folder | -      | Protected           |
| `Documents/report.txt`            | file   | 2048 B | In protected folder |
| `Documents/note.txt`              | file   | 500 B  | < 1KB, in protected |
| **Projects/**                     | folder | -      | Normal, selectable  |
| `Projects/app`                    | file   | 5120 B | Normal              |
| **Projects/src/**                 | folder | -      | Nested subfolder    |
| `Projects/src/main.rs`            | file   | 1024 B | Normal              |
| `.ssh`                            | folder | -      | Skipped (invisible) |
| `.Trash`                          | folder | -      | Skipped (invisible) |

---

### Suite 1: `scan-flow.spec.ts` — Scan Lifecycle

- Shows scan launch with start button
- Starts scan and shows progress indicator
- Shows results list after scan completes
- Results contain expected folders (MyData, Documents, Projects) sorted by size descending
- Skipped folders absent (.ssh, .Trash)
- Protected folders (Documents) show disabled checkbox
- Normal folders (MyData, Projects) show enabled checkbox
- Can abort scan and return to launch screen
- Can re-scan after abort

---

### Suite 2: `selection-checkbox.spec.ts` — Comprehensive Selection Model

This is the most critical suite. The selection model has complex parent/child state interactions.

#### Key Implementation Details Being Tested

From `ScanResultsList.vue`:

- `selectedMap`: reactive Map<path, FolderInfo> — stores explicit selections
- `someSelectedPaths`: Set of folders with selected descendants (indeterminate state)
- `isSelectedForUI(path)`: true if explicitly selected OR has selected ancestor
- `isSelectable` prop: `true` | `false` | `'deselect-only'` (protected with selected descendants)
- `SelectionIcon` states: `'empty'` | `'partial'` | `'selected'`

#### 2a. Basic Selection (State 1 & 2)

```
it('clicking unselected folder checkbox shows selected state')
  - Find "MyData" row → checkbox aria-pressed="false", SelectionIcon state="empty"
  - Click checkbox
  - Assert aria-pressed="true", row has class "--selected"
  - Assert SelectionIcon renders with class "checkFilled"

it('clicking selected folder checkbox deselects it')
  - (MyData selected) → click checkbox again
  - Assert aria-pressed="false", row loses "--selected" class
  - Assert SelectionIcon renders with class "checkEmpty"

it('clicking unselected file checkbox selects the file')
  - Navigate into MyData → find "big.txt" file row
  - Click its checkbox → assert selected

it('selecting item updates Review button text with size')
  - Select "MyData" → assert review-selection button text contains size (e.g. "2 KB" or localized)
  - Assert button is NOT disabled

it('deselecting all items disables Review button')
  - Select MyData → deselect MyData
  - Assert review-selection button is disabled

it('selecting multiple items accumulates size correctly')
  - Select "MyData" and "Projects"
  - Assert review button shows combined size
```

#### 2b. Inherited Selection (State 3: Ancestor Selected → Children Inherit)

```
it('selecting folder then navigating in shows all children as inherited-selected')
  - Select "MyData" checkbox at root level
  - Click "MyData" row (navigate into it)
  - Wait for children to render
  - Assert "big.txt" has aria-pressed="true" and "--selected" class
  - Assert "SubFolder" has aria-pressed="true" and "--selected" class
  - Assert ALL visible children show selected state (inherited from parent)

it('inherited-selected children have "selected" SelectionIcon, not "partial"')
  - (Inside MyData with MyData selected)
  - Assert all children's SelectionIcon have class "checkFilled" (fully selected, not partial)
```

#### 2c. Explode Ancestor (State 3: Click Inherited Child → Explode)

This tests `explodeAncestorExcluding()` which walks from ancestor down to the excluded item, selecting all siblings at each intermediate level.

```
it('clicking inherited child deselects that child and selects all siblings')
  - Select "MyData" at root
  - Navigate into MyData
  - Click "big.txt" checkbox (inherited-selected → explode)
  - Assert "big.txt" is NOT selected (it was excluded)
  - Assert "small.txt" IS selected (sibling gets explicitly selected)
  - Assert "SubFolder" IS selected (sibling folder gets explicitly selected)
  - Navigate back to root
  - Assert "MyData" is NO LONGER directly selected (it was exploded)
  - Assert "MyData" shows INDETERMINATE state (has selected descendants)

it('explode at nested level selects siblings at every intermediate level')
  - Select "MyData" at root
  - Navigate into MyData → navigate into SubFolder
  - Click "alpha.txt" checkbox (2 levels deep from selected ancestor)
  - Assert "alpha.txt" is NOT selected
  - Assert "beta.txt" IS selected (sibling at SubFolder level)
  - Assert "Deep" folder IS selected (sibling at SubFolder level)
  - Navigate back to MyData level
  - Assert "big.txt" IS selected (sibling at MyData level; explode selected siblings here too)
  - Assert "small.txt" IS selected
  - Assert "SubFolder" shows INDETERMINATE (has selected children but not itself)

it('explode preserves size accuracy — no double counting')
  - Select "MyData" → record its total size from review button
  - Navigate into MyData → click "big.txt" to explode
  - Navigate back to root → check review button size
  - Assert size = (MyData total size) - (big.txt size)
```

#### 2d. Indeterminate State (State 4)

```
it('selecting a child inside a folder makes parent show partial/indeterminate icon')
  - Navigate into MyData → select "big.txt" → navigate back
  - Find "MyData" row
  - Assert SelectionIcon has class "checkPartial" (indeterminate)
  - Assert aria-pressed="true" (isSomeSelected triggers this)

it('clicking indeterminate folder deselects ALL descendants')
  - (MyData has big.txt selected inside)
  - Click "MyData" checkbox (indeterminate → deselect all)
  - Navigate into MyData
  - Assert "big.txt" is NOT selected
  - Assert ALL children are unselected

it('multi-level indeterminate: selecting deep file propagates partial state up')
  - Navigate into MyData → into SubFolder → select "alpha.txt"
  - Navigate back to SubFolder level (back once)
  - Assert "SubFolder" shows indeterminate
  - Navigate back to root
  - Assert "MyData" shows indeterminate

it('clicking indeterminate at root clears deeply nested selections')
  - (alpha.txt selected 3 levels deep)
  - At root, click "MyData" checkbox (indeterminate)
  - Navigate all the way into SubFolder
  - Assert "alpha.txt" is unselected
```

#### 2e. Protected Folder Behavior

```
it('protected folder checkbox is disabled when nothing selected inside')
  - Find "Documents" row at root
  - Assert checkbox has attribute "disabled"
  - Assert checkbox has class "--disabled"
  - Assert SelectionIcon state is "empty"

it('clicking disabled protected checkbox does nothing')
  - Click "Documents" checkbox
  - Assert still unselected, no state change

it('protected folder shows "deselect-only" when it has selected descendants')
  - Navigate into Documents → select "report.txt" (contents are selectable)
  - Navigate back to root
  - Assert Documents shows INDETERMINATE icon (has selected child)
  - Assert Documents checkbox is NOT fully disabled (isSelectable='deselect-only')
  - Click Documents checkbox → descendants deselected
  - Assert Documents checkbox returns to disabled state

it('cannot select a protected folder directly, even after its descendants were deselected')
  - (Documents has no selections)
  - Click Documents checkbox
  - Assert no change — still unselected and disabled
```

#### 2f. Navigation + Selection Interaction

```
it('selection persists across folder navigation')
  - Select "Projects" at root
  - Navigate into MyData → navigate back
  - Assert "Projects" still selected

it('forward/back navigation preserves selection')
  - Select "MyData" → navigate into Projects → navigate back
  - Assert "MyData" still selected

it('reset button clears all selections and returns to root')
  - Navigate into MyData → select "big.txt" → navigate into SubFolder → select "alpha.txt"
  - Click reset button (data-testid="results-cancel")
  - Assert returned to root view
  - Assert all selections cleared
  - Assert Review button disabled
```

#### 2g. Size Computation Edge Cases

```
it('selecting a folder then selecting a child inside does not double-count')
  - Select "MyData" at root (captures full folder size)
  - Navigate into MyData → also select "big.txt" (redundant)
  - Navigate back → check review button size
  - Assert size equals MyData's total size (not MyData + big.txt)

it('selectedSize updates immediately on selection change')
  - Select "Projects" → note size in review button
  - Select "MyData" → assert review button size increased
  - Deselect "Projects" → assert review button size decreased
```

---

### Suite 3: `trash-review.spec.ts` — Selection → Trash → Confirmation Flow

#### 3a. Entering Trash Review

```
it('Review button disabled when nothing selected')
  - Assert review-selection button is disabled

it('selecting items and clicking Review shows trash list')
  - Select "MyData" → click review-selection
  - Assert trash-list displayed
  - Assert trash-list-row elements exist

it('trash list shows flattened items sorted by size descending')
  - Select "MyData" and "Projects" → click Review
  - Assert rows ordered: Projects/app (5120), MyData (aggregated), etc.
  - Items whose ancestor is already selected are filtered out (no double entries)
```

#### 3b. Trash List Checkbox Behavior

```
it('all items checked by default')
  - Assert all trash-list-row-checkbox elements are checked

it('unchecking item updates button size')
  - Uncheck one item → assert button text shows reduced size

it('unchecking all items disables Move to Trash button')
  - Uncheck all items → assert confirm-trash button is disabled
```

#### 3c. Back Navigation from Trash

```
it('back button returns to results with selection preserved')
  - Select MyData → Review → click back (nav-back)
  - Assert results-list displayed
  - Assert "MyData" still selected

it('unchecking item in trash list and going back updates selection')
  - Select MyData and Projects → Review
  - Uncheck Projects in trash list → click back
  - Assert MyData still selected, Projects deselected
```

#### 3d. Move to Trash — Safety Countdown

```
it('Move to Trash button starts disabled with countdown')
  - Select item → click Review
  - Assert confirm-trash button is disabled immediately
  - Wait for TRASH_COUNTDOWN_MS (1000ms)
  - Assert confirm-trash button becomes enabled

it('countdown resets when re-entering trash view')
  - Review → back → Review again
  - Assert button is disabled again (countdown restarted)
```

#### 3e. Trash Confirmation — Success

```
it('clicking Move to Trash shows spinner then success screen')
  - Select MyData → Review → wait for countdown → click confirm-trash
  - Assert spinner/deleting state appears
  - Wait for TRASH_POST_TRASH_SLEEP_MS
  - Assert confirmation screen renders with:
    - AnimatedCheckCircle (success icon, not error)
    - Title text (success variant)
    - Resume text with count and size
    - restart button (data-testid="restart")
    - Close app button

it('clicking Scan Again returns to scan launch')
  - (On confirmation screen) → click restart button
  - Assert scan-launch displayed with start-scan button
```

#### 3f. Trash Confirmation — Error/0KB Case

```
it('when trash returns count=0, shows error state with alert icon')
  - This test requires launching app with E2E_TRASH_RESULT=zero
  - Select MyData → Review → wait → click confirm-trash
  - Assert confirmation screen renders with:
    - AnimatedAlertCircle (error icon, class "iconError")
    - Error title text (hasErrors variant)
    - Error resume text
    - restart button still available
```

**Implementation note:** The 0KB test may need a separate WDIO session/config with `E2E_TRASH_RESULT=zero`, or we add a Tauri command `set_e2e_trash_mode(mode)` callable from the test to switch the mock behavior mid-session without restarting the app.

---

### Suite 4: `settings-flow.spec.ts` — Settings ↔ Scan Integration

```
beforeEach: reset settings via invoke('reset_e2e_state')

it('default settings: all toggles off')
  - Navigate to Settings
  - Assert hidden files toggle aria-checked="false"
  - Assert under 1KB toggle aria-checked="false"
  - Assert zero byte toggle aria-checked="false"

it('toggling showHiddenFiles persists after view switch')
  - Toggle hidden files → navigate to Scan → back to Settings
  - Assert toggle still on

it('scan with showHiddenFiles=true includes .hidden file')
  - Enable hidden files → scan → navigate into MyData
  - Assert ".hidden" file row present

it('scan with showHiddenFiles=false excludes .hidden file')
  - (default) → scan → navigate into MyData
  - Assert ".hidden" file row NOT present

it('scan with showUnder1Kb=true includes small.txt (100 B)')
  - Enable showUnder1Kb → scan → navigate into MyData
  - Assert "small.txt" present

it('scan with showUnder1Kb=false excludes small.txt')
  - (default, showUnder1Kb=false) → scan → navigate into MyData
  - Assert "small.txt" NOT present

it('scan with showZeroByte=true includes empty.txt')
  - Enable showZeroByte AND showUnder1Kb → scan → navigate into MyData
  - Assert "empty.txt" present

it('scan with showZeroByte=false excludes empty.txt')
  - (default) → scan → navigate into MyData
  - Assert "empty.txt" NOT present

it('combined: hidden + under1Kb shows all files')
  - Enable both → scan → navigate into MyData
  - Assert big.txt, small.txt, .hidden all present
```

### Suite 5: `app-navigation.spec.ts` (existing, keep as-is)

---

## Phase 3: Test Helpers

### Shared helpers file: `e2e/helpers/navigation.ts`

```typescript
// Navigate into a folder by name (clicks the row, not checkbox)
async function navigateIntoFolder(name: string)

// Click back button
async function navigateBack()

// Find a row element by item name (iterates rows, matches text)
async function getRowByName(name: string): Promise<WebdriverIO.Element | null>

// Get checkbox element within a row
async function getCheckbox(row: WebdriverIO.Element): Promise<WebdriverIO.Element>

// Assert checkbox visual state: 'empty' | 'partial' | 'selected'
async function assertCheckboxState(
   row: WebdriverIO.Element,
   expected: 'empty' | 'partial' | 'selected'
)

// Assert row is selected (class and aria)
async function assertRowSelected(row: WebdriverIO.Element, selected: boolean)

// Reset app state between tests
async function resetState()

// Start a scan and wait for results list to appear
async function scanAndWaitForResults()

// Get review button size text
async function getReviewButtonText(): Promise<string>

// Check if review button is disabled
async function isReviewButtonDisabled(): Promise<boolean>
```

---

## Gotchas & Mitigations

### CI Permissions / FDA

- GitHub Actions macOS runners never have FDA → mocked via `E2E_FDA` env var
- No real disk access needed since scan targets the temp fixture dir
- No special permissions required for reading/writing temp dirs

### Clicking checkbox vs row

- Clicking the row navigates into a folder; clicking `[data-testid="results-row-checkbox"]` selects
- The checkbox has `@click.stop` so events don't propagate; WebDriverIO must target the checkbox specifically
- Files don't navigate on row click (only folders), but checkbox click still selects

### View transitions

- App uses Vue `<Transition>` and `view-transition-name`, so elements may briefly leave the DOM
- Always `waitForDisplayed` before interacting
- Add small `browser.pause(300)` after navigation to let transitions settle

### Row identification

- Rows don't embed item name in `data-testid`; must iterate `$$('[data-testid="results-row-folder"], [data-testid="results-row-file"]')` and match by text content
- Consider adding `data-name` attributes in a future pass for easier targeting

### Settings reset

- Tests that change settings must reset between runs via `reset_e2e_state` command
- Use `beforeEach` hooks in settings suite

### Trash 0KB case

- Requires mock to return `{ count: 0, size: 0 }`
- Best approach: add a `set_e2e_trash_mode` Tauri command (gated behind `e2e`) so tests can switch the mock behavior via `browser.execute()` without restarting the app
- Avoids needing separate WDIO sessions for each trash behavior

### SelectionIcon state detection

- The icon uses CSS classes: `checkEmpty`, `checkPartial`, `checkFilled`
- Use `element.getAttribute('class')` to detect which state is rendered
- The checkbox's `aria-pressed` reflects both `isSelected` and `isSomeSelected`

---

## Rollout: Incremental

**First PR: Foundation + scan-flow**

1. All Phase 1 changes (Rust feature gate, enhanced fixtures, mocks, WDIO config)
2. `scan-flow.spec.ts` + helpers
3. Validate the approach works locally and in CI

**Follow-up PRs:** 4. `selection-checkbox.spec.ts` (most complex suite, all 2a through 2g sections) 5. `trash-review.spec.ts` (including 0KB error case) 6. `settings-flow.spec.ts`

## Verification

1. **Local:** `pnpm tauri build --debug --no-bundle -- --features e2e` → verify app launches and scans fixture dir
2. **Unit tests still pass:** `pnpm test:unit` (no changes to non-gated code)
3. **E2E locally:** `pnpm test:e2e` → all suites green (scan-flow + existing navigation)
4. **CI:** Push PR → GitHub Actions runs both unit + e2e on `macos-latest`

---

## Files to Create/Modify

| File                                   | Action                                                                        |
| -------------------------------------- | ----------------------------------------------------------------------------- |
| `src-tauri/Cargo.toml`                 | Expand `e2e` feature with `tempfile` as optional dep                          |
| `src-tauri/src/e2e_fixtures.rs`        | **New** — enhanced `create_test_home()` with nested dirs, `LazyLock<TempDir>` |
| `src-tauri/src/lib.rs`                 | Register `reset_e2e_state` + `set_e2e_trash_mode` commands under `e2e` gate   |
| `src-tauri/src/scan.rs`                | Conditional home dir override (line 409)                                      |
| `src-tauri/src/permissions.rs`         | Conditional FDA mock reading `E2E_FDA` env var                                |
| `src-tauri/src/trash.rs`               | Conditional trash mock with configurable result                               |
| `src-tauri/src/store.rs`               | Add `reset_e2e_state` command                                                 |
| `e2e/wdio.conf.ts`                     | Pass env vars to spawned app process                                          |
| `e2e/helpers/navigation.ts`            | **New** — shared test helpers with state assertions                           |
| `e2e/specs/scan-flow.spec.ts`          | **New**                                                                       |
| `e2e/specs/selection-checkbox.spec.ts` | **New**                                                                       |
| `e2e/specs/trash-review.spec.ts`       | **New**                                                                       |
| `e2e/specs/settings-flow.spec.ts`      | **New**                                                                       |
| `.github/workflows/tests.yml`          | Add `E2E_FDA: 'false'` env var                                                |
