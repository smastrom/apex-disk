/**
 * Shared E2E test helpers for ApexDisk.
 *
 * These helpers abstract common operations (navigation, checkbox state assertions,
 * scanning) so that spec files stay concise and readable.
 */

const VIEW_READY_TIMEOUT = 15000
const ELEMENT_TIMEOUT = 5000
const TRANSITION_SETTLE_MS = 400

// ---------------------------------------------------------------------------
// Selectors
// ---------------------------------------------------------------------------

export const sel = {
   appHeader: '[data-testid="app-header"]',
   footerScan: '[data-testid="footer-scan"]',
   footerSettings: '[data-testid="footer-settings"]',
   footerInformation: '[data-testid="footer-information"]',
   scanLaunch: '[data-testid="scan-launch"]',
   startScan: '[data-testid="start-scan"]',
   scanProgress: '[data-testid="scanning-results"]',
   scanAbort: '[data-testid="scan-abort"]',
   resultsList: '[data-testid="results-list"]',
   resultsNav: '[data-testid="results-nav"]',
   navBack: '[data-testid="nav-back"]',
   navForward: '[data-testid="nav-forward"]',
   navPathLabel: '[data-testid="nav-path-label"]',
   resultsCancel: '[data-testid="results-cancel"]',
   reviewSelection: '[data-testid="review-selection"]',
   rowFolder: '[data-testid="results-row-folder"]',
   rowFile: '[data-testid="results-row-file"]',
   rowCheckbox: '[data-testid="results-row-checkbox"]',
   trashList: '[data-testid="trash-list"]',
   trashRow: '[data-testid="trash-list-row"]',
   trashRowCheckbox: '[data-testid="trash-list-row-checkbox"]',
   confirmTrash: '[data-testid="confirm-trash"]',
   restart: '[data-testid="restart"]',
   settingsView: '[data-testid="settings-view"]',
   settingsContent: '[data-testid="settings-content"]',
} as const

// ---------------------------------------------------------------------------
// Wait helpers
// ---------------------------------------------------------------------------

/** Wait for the app to be ready (header visible). */
export async function waitForAppReady() {
   const header = $(sel.appHeader)
   await header.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
}

/** Wait for the scan launch screen. */
export async function waitForScanLaunch() {
   const launch = $(sel.scanLaunch)
   await launch.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
   const btn = $(sel.startScan)
   await btn.waitForDisplayed({ timeout: ELEMENT_TIMEOUT })
}

/** Wait for the results list to appear after a scan. */
export async function waitForResultsList() {
   const list = $(sel.resultsList)
   await list.waitForDisplayed({ timeout: 30000 })
}

// ---------------------------------------------------------------------------
// Scan helpers
// ---------------------------------------------------------------------------

/** Click Start Scan and wait for the results list to render. */
export async function scanAndWaitForResults() {
   const startBtn = $(sel.startScan)
   await startBtn.waitForDisplayed({ timeout: ELEMENT_TIMEOUT })
   await startBtn.click()
   await waitForResultsList()
}

// ---------------------------------------------------------------------------
// Row lookup
// ---------------------------------------------------------------------------

/** Find a result row (folder or file) by its display name. Returns null if not found. */
export async function getRowByName(name: string): Promise<WebdriverIO.Element | null> {
   const rows = await $$(`${sel.rowFolder}, ${sel.rowFile}`)
   for (const row of rows) {
      const text = await row.getText()
      if (text.includes(name)) return row
   }
   return null
}

/** Find a result row by name, throws if not found. */
export async function requireRowByName(name: string): Promise<WebdriverIO.Element> {
   const row = await getRowByName(name)
   if (!row) throw new Error(`Row "${name}" not found in results`)
   return row
}

/** Get the checkbox button inside a result row. */
export async function getCheckbox(row: WebdriverIO.Element) {
   return row.$(sel.rowCheckbox)
}

// ---------------------------------------------------------------------------
// Navigation
// ---------------------------------------------------------------------------

/** Navigate into a folder by clicking its row (not its checkbox). */
export async function navigateIntoFolder(name: string) {
   const row = await requireRowByName(name)
   // Click the row itself, not the checkbox. The checkbox has @click.stop,
   // so clicking it selects rather than navigates.
   await row.click()
   await browser.pause(TRANSITION_SETTLE_MS)
}

/** Click the back navigation button and wait for the transition to settle. */
export async function navigateBack() {
   const btn = $(sel.navBack)
   await btn.click()
   await browser.pause(TRANSITION_SETTLE_MS)
}

/** Click the forward navigation button and wait for the transition to settle. */
export async function navigateForward() {
   const btn = $(sel.navForward)
   await btn.click()
   await browser.pause(TRANSITION_SETTLE_MS)
}

// ---------------------------------------------------------------------------
// Checkbox state assertions
// ---------------------------------------------------------------------------

/**
 * Assert that a row's SelectionIcon has a specific visual state.
 * Maps to CSS classes: checkEmpty, checkPartial, checkFilled.
 */
export async function assertCheckboxState(
   row: WebdriverIO.Element,
   expected: 'empty' | 'partial' | 'selected'
) {
   const checkbox = await getCheckbox(row)
   const classMap = {
      empty: 'ScanResultsListItem-checkEmpty',
      partial: 'ScanResultsListItem-checkPartial',
      selected: 'ScanResultsListItem-checkFilled',
   }
   // The class is on the SVG icon inside the checkbox, not the button itself
   const icon = await checkbox.$('svg')
   const classes = await icon.getAttribute('class')
   expect(classes).toContain(classMap[expected])
}

/** Assert that a row appears selected (has the --selected class). */
export async function assertRowSelected(row: WebdriverIO.Element, selected: boolean) {
   const classes = await row.getAttribute('class')
   if (selected) {
      expect(classes).toContain('ScanResultsListItem-root--selected')
   } else {
      expect(classes).not.toContain('ScanResultsListItem-root--selected')
   }
}

/** Assert that a row's checkbox is disabled. */
export async function assertCheckboxDisabled(row: WebdriverIO.Element, disabled: boolean) {
   const checkbox = await getCheckbox(row)
   const isDisabled = await checkbox.getAttribute('disabled')
   if (disabled) {
      expect(isDisabled).not.toBeNull()
   } else {
      expect(isDisabled).toBeNull()
   }
}

// ---------------------------------------------------------------------------
// Review button helpers
// ---------------------------------------------------------------------------

/** Get the review button element. */
export function getReviewButton() {
   return $(sel.reviewSelection)
}

/** Check if the review button is disabled. */
export async function isReviewButtonDisabled(): Promise<boolean> {
   const btn = getReviewButton()
   const val = await btn.getAttribute('disabled')
   return val !== null
}

// ---------------------------------------------------------------------------
// Tauri invoke helpers (call Rust commands from tests)
// ---------------------------------------------------------------------------

/** Reset app settings to defaults via the e2e Tauri command. */
export async function resetE2eState() {
   await browser.execute(() => {
      return (window as any).__TAURI__.core.invoke('reset_e2e_state')
   })
}

/** Set the trash mock mode ('success' | 'zero' | 'error'). */
export async function setTrashMode(mode: 'success' | 'zero' | 'error') {
   await browser.execute((m: string) => {
      return (window as any).__TAURI__.core.invoke('set_e2e_trash_mode', { mode: m })
   }, mode)
}

// ---------------------------------------------------------------------------
// Assertion helpers for missing/present rows
// ---------------------------------------------------------------------------

/** Assert a row with the given name exists in the current view. */
export async function assertRowExists(name: string) {
   const row = await getRowByName(name)
   expect(row).not.toBeNull()
}

/** Assert a row with the given name does NOT exist in the current view. */
export async function assertRowNotExists(name: string) {
   const row = await getRowByName(name)
   expect(row).toBeNull()
}
