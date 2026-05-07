// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

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
   settingsToggleHiddenFiles: '[aria-labelledby="label-hidden-files"]',
   settingsToggleUnder1Kb: '[aria-labelledby="label-under-1kb"]',
   settingsToggleZeroByte: '[aria-labelledby="label-zero-byte"]',
} as const

// ---------------------------------------------------------------------------
// Wait helpers
// ---------------------------------------------------------------------------

/**
 * Poll until no Vue <Transition> is mid-flight anywhere on the page.
 *
 * Vue applies `*-enter-active` / `*-leave-active` classes only while a
 * transition is running, so absence of both = nothing animating right now.
 * Cheaper and more accurate than fixed `browser.pause()` constants.
 */
export async function waitForListSlideSettled(): Promise<void> {
   await browser.waitUntil(
      async () =>
         !(await browser.execute(
            () =>
               !!document.querySelector(
                  '.list-slide-enter-active, .list-slide-leave-active, .app-slide-enter-active, .app-slide-leave-active'
               )
         )),
      {
         timeout: ELEMENT_TIMEOUT,
         interval: 50,
         timeoutMsg: 'transitions did not settle within timeout',
      }
   )
}

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

/**
 * Single-shot lookup: returns the row if it's in the DOM right now, else null.
 * Use this for negative assertions ("row should not be present"). For positive
 * lookups during/after navigation, prefer `requireRowByName` so the helper
 * polls past any in-flight Vue <Transition> on the list slide.
 */
export async function getRowByName(name: string): Promise<WebdriverIO.Element | null> {
   const rows = await $$(`${sel.rowFolder}, ${sel.rowFile}`)
   for (const row of rows) {
      const text = await row.getText()
      if (text.includes(name)) return row
   }
   return null
}

/**
 * Polls for a result row until it appears (Cypress-style retry). Tolerates
 * the `<Transition mode="out-in">` window in `ScanResultsList` where the
 * leaving list is unmounted and the entering list hasn't yet mounted.
 */
export async function requireRowByName(name: string): Promise<WebdriverIO.Element> {
   let row: WebdriverIO.Element | null = null
   await browser.waitUntil(
      async () => {
         row = await getRowByName(name)
         return row !== null
      },
      {
         timeout: ELEMENT_TIMEOUT,
         interval: 100,
         timeoutMsg: `Row "${name}" not found in results`,
      }
   )
   return row!
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

/**
 * Classic WebDriver's sync `execute` can't serialize a Promise return, and
 * async script bodies still return one. `executeAsync` is the only way to
 * await an async Tauri `invoke` over the chromedriver session.
 */

/** Reset app settings to defaults via the e2e Tauri command. */
export async function resetE2eState() {
   const err = await browser.executeAsync<string | null, []>((done: any) => {
      ;(window as any).__TAURI_INTERNALS__
         .invoke('reset_e2e_state')
         .then(() => done(null))
         .catch((e: unknown) => done(String(e && (e as Error).message ? (e as Error).message : e)))
   })
   if (err) throw new Error(`reset_e2e_state failed: ${err}`)

   // The Vue settings ref is loaded once at startup and doesn't re-read after
   // a Rust-side reset, so any scan filter that's currently "on" in the UI
   // would still bias the next scan. Normalize the UI to match the store
   // defaults by flipping any lingering "on" toggle back to "off".
   await normalizeScanFilterToggles()
}

async function normalizeScanFilterToggles() {
   await goToSettingsView()
   const selectors = [
      sel.settingsToggleHiddenFiles,
      sel.settingsToggleUnder1Kb,
      sel.settingsToggleZeroByte,
   ]
   for (const selector of selectors) {
      const on = await getToggleState(selector)
      if (on) {
         await $(selector).click()
         await browser.pause(80)
      }
   }
}

/** Set the trash mock mode ('success' | 'zero' | 'error'). */
export async function setTrashMode(mode: 'success' | 'zero' | 'error') {
   const err = await browser.executeAsync<string | null, [string]>((m: string, done: any) => {
      ;(window as any).__TAURI_INTERNALS__
         .invoke('set_e2e_trash_mode', { mode: m })
         .then(() => done(null))
         .catch((e: unknown) => done(String(e && (e as Error).message ? (e as Error).message : e)))
   }, mode)
   if (err) throw new Error(`set_e2e_trash_mode failed: ${err}`)
}

// ---------------------------------------------------------------------------
// Assertion helpers for missing/present rows
// ---------------------------------------------------------------------------

/** Assert a row with the given name exists in the current view (polls). */
export async function assertRowExists(name: string) {
   await requireRowByName(name)
}

/**
 * Assert a row with the given name does NOT exist in the current view.
 *
 * Negative assertions are inherently racy with Vue <Transition>: a row might
 * be transiently in the DOM during the leaving list's slide-out. Wait for the
 * list slide to settle before checking, so we measure the steady state.
 */
export async function assertRowNotExists(name: string) {
   await waitForListSlideSettled()
   const row = await getRowByName(name)
   expect(row).toBeNull()
}

// ---------------------------------------------------------------------------
// Selection helpers
// ---------------------------------------------------------------------------

/** Click the checkbox button inside a row. Does not navigate into the folder. */
export async function clickRowCheckbox(row: WebdriverIO.Element) {
   const checkbox = await getCheckbox(row)
   await checkbox.click()
}

/** Click the checkbox for a row identified by name. */
export async function clickCheckboxByName(name: string) {
   const row = await requireRowByName(name)
   await clickRowCheckbox(row)
}

/** Click the reset-selection button in the results nav. */
export async function clickResetSelection() {
   const resetBtn = await $(sel.resultsNav).$(`button[aria-label*="eset" i]`)
   await resetBtn.click()
}

/**
 * Navigate back until the back button is disabled (i.e. at root).
 * Useful in `beforeEach` when preceding tests may have descended into folders.
 */
export async function navigateBackToRoot() {
   const back = $(sel.navBack)
   for (let i = 0; i < 10; i++) {
      const disabled = await back.getAttribute('disabled')
      if (disabled !== null) return
      await back.click()
      await browser.pause(TRANSITION_SETTLE_MS)
   }
}

/** Get the current review button text (includes size and count when selected). */
export async function getReviewButtonText(): Promise<string> {
   return getReviewButton().getText()
}

/**
 * Extract the formatted byte count from the review button text.
 * Returns the value in bytes (SI decimal, matching `formatBytes` in the app),
 * or null if no size is rendered (e.g. empty selection).
 */
export async function getReviewButtonBytes(): Promise<number | null> {
   const text = await getReviewButtonText()
   const match = text.match(/([\d.,]+)\s*(B|KB|MB|GB|TB)/)
   if (!match) return null
   const n = parseFloat(match[1].replace(',', '.'))
   if (Number.isNaN(n)) return null
   const unit = match[2]
   const mult = { B: 1, KB: 1000, MB: 1e6, GB: 1e9, TB: 1e12 }[unit]!
   return n * mult
}

/** Click the review button to enter the trash review view. */
export async function clickReviewSelection() {
   const btn = getReviewButton()
   await btn.click()
}

// ---------------------------------------------------------------------------
// App view navigation
// ---------------------------------------------------------------------------

/** Click the Scan footer tab and wait for the scan view to be ready. */
export async function goToScanView() {
   await $(sel.footerScan).click()
   await browser.pause(TRANSITION_SETTLE_MS)
}

/** Click the Settings footer tab and wait for the settings view. */
export async function goToSettingsView() {
   await $(sel.footerSettings).click()
   const view = $(sel.settingsView)
   await view.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
}

// ---------------------------------------------------------------------------
// Trash list helpers
// ---------------------------------------------------------------------------

/** Wait for the trash review list to be displayed. */
export async function waitForTrashList() {
   const list = $(sel.trashList)
   await list.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
}

/** Get all trash review rows in display order. */
export async function getTrashRows(): Promise<WebdriverIO.Element[]> {
   return (await $$(sel.trashRow)) as unknown as WebdriverIO.Element[]
}

/** Find a trash row by item name. Returns null if not found. */
export async function getTrashRowByName(name: string): Promise<WebdriverIO.Element | null> {
   const rows = await getTrashRows()
   for (const row of rows) {
      const text = await row.getText()
      if (text.includes(name)) return row
   }
   return null
}

/** Toggle a trash row's checkbox by clicking it (has @click.stop). */
export async function toggleTrashRow(row: WebdriverIO.Element) {
   const checkbox = await row.$(sel.trashRowCheckbox)
   await checkbox.click()
}

/** Wait for the post-trash confirmation screen (either success or error variant). */
export async function waitForTrashConfirmation() {
   const restart = $(sel.restart)
   await restart.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
}

// ---------------------------------------------------------------------------
// Settings helpers
// ---------------------------------------------------------------------------

/** Read the aria-checked state of a settings toggle (boolean). */
export async function getToggleState(selector: string): Promise<boolean> {
   const toggle = $(selector)
   const v = await toggle.getAttribute('aria-checked')
   return v === 'true'
}

/** Click a settings toggle and wait a tick for the store to persist. */
export async function clickToggle(selector: string) {
   await $(selector).click()
   await browser.pause(100)
}

/**
 * Cancel the current scan results (X button) and return to the scan launch screen.
 * No-op if already on the launch screen.
 */
export async function cancelToLaunch() {
   const cancel = $(sel.resultsCancel)
   const onResults = await cancel.isDisplayed().catch(() => false)
   if (onResults) {
      await cancel.click()
      await browser.pause(TRANSITION_SETTLE_MS)
   }
   await waitForScanLaunch()
}

/**
 * Throw away any existing scan results and kick off a fresh scan.
 * Use this after toggling settings that only take effect on the next scan.
 */
export async function rescanFresh() {
   await goToScanView()
   await cancelToLaunch()
   await scanAndWaitForResults()
}

// ---------------------------------------------------------------------------
// Whole-suite setup: reset state, navigate to Scan, run fresh scan
// ---------------------------------------------------------------------------

/**
 * Bring the app back to a known state and run a fresh scan.
 * - Resets settings to defaults (so hidden/<1 KB/0 B filters are off).
 * - Switches to Scan view.
 * - If on the trash confirmation screen, restarts back to results.
 * - If on the trash list, navigates back to results.
 * - If on results, navigates to root and clears any selection.
 * - If on launch, starts a fresh scan.
 */
export async function resetAndScan() {
   await resetE2eState()
   await goToScanView()

   // Trash confirmation screen (TRASH_COMPLETE): click restart to return to results.
   const restart = $(sel.restart)
   const onConfirm = await restart.isDisplayed().catch(() => false)
   if (onConfirm) {
      await restart.click()
      await browser.pause(TRANSITION_SETTLE_MS)
   }

   // Trash list (TRASH): click back to return to results.
   const trashList = $(sel.trashList)
   const onTrash = await trashList.isDisplayed().catch(() => false)
   if (onTrash) {
      const back = $(sel.navBack)
      await back.click()
      await browser.pause(TRANSITION_SETTLE_MS)
   }

   // If results are displayed (either directly or after the step above), go
   // back to root and clear any lingering selection.
   const results = $(sel.resultsList)
   const hasResults = await results.isDisplayed().catch(() => false)
   if (hasResults) {
      await navigateBackToRoot()
      const resetBtn = await $(sel.resultsNav).$(`button[aria-label*="eset" i]`)
      const disabled = await resetBtn.getAttribute('disabled')
      if (disabled === null) await resetBtn.click()
      return
   }

   // Otherwise we're on the launch screen — start a new scan.
   const launch = $(sel.scanLaunch)
   const onLaunch = await launch.isDisplayed().catch(() => false)
   if (onLaunch) {
      await scanAndWaitForResults()
   }
}
