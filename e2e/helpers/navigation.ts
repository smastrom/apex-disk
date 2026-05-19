// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * Shared E2E test helpers for ApexDisk.
 *
 * These helpers abstract common operations (navigation, checkbox state assertions,
 * scanning) so that spec files stay concise and readable.
 */

const VIEW_READY_TIMEOUT = 20000
const ELEMENT_TIMEOUT = 10000
const RESULTS_TIMEOUT = 30000
const SCAN_START_RETRY_DELAY_MS = 1000
const SCAN_ATTEMPTS = 3
const TRANSITION_SETTLE_MS = 400

// ---------------------------------------------------------------------------
// Selectors
// ---------------------------------------------------------------------------

export const sel = {
   appHeader: '[data-testid="app-header"]',
   footerScan: '[data-testid="footer-scan"]',
   footerSettings: '[data-testid="footer-settings"]',
   footerInformation: '[data-testid="footer-information"]',
   footerScanDot: '[data-testid="footer-scan-dot"]',
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
   resultsTruncated: '[data-testid="results-truncated"]',
   trashList: '[data-testid="trash-list"]',
   trashRow: '[data-testid="trash-list-row"]',
   trashRowCheckbox: '[data-testid="trash-list-row-checkbox"]',
   confirmTrash: '[data-testid="confirm-trash"]',
   restart: '[data-testid="restart"]',
   informationLicense: '.InformationFooter-credits',
   settingsView: '[data-testid="settings-view"]',
   settingsContent: '[data-testid="settings-content"]',
   settingsTheme: '[aria-labelledby="label-theme"]',
   settingsToggleHiddenFiles: '[aria-labelledby="label-hidden-files"]',
   settingsToggleDsStore: '[aria-labelledby="label-ds-store"]',
   settingsToggleUnder1Kb: '[aria-labelledby="label-under-1kb"]',
   settingsToggleZeroByte: '[aria-labelledby="label-zero-byte"]',
   settingsToggleAutoCheckUpdates: '[aria-labelledby="label-auto-check-updates"]',
   settingsToggleAutoInstallUpdates: '[aria-labelledby="label-auto-install-updates"]',
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
   await browser
      .waitUntil(
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
      .catch(async () => {
         await browser.pause(TRANSITION_SETTLE_MS)
      })
}

/**
 * WebDriver-driven WebKit occasionally starves the rAF that drives Vue's
 * `<Transition>` `*-from` → `*-to` class swap, leaving the leaving element
 * parked mid-transition forever. setTimeout isn't throttled the same way.
 */
async function patchRequestAnimationFrame(): Promise<void> {
   await browser.execute(() => {
      const w = window as unknown as { __e2eMotionPatched?: boolean }

      if (w.__e2eMotionPatched) return

      w.__e2eMotionPatched = true
      window.requestAnimationFrame = ((cb: FrameRequestCallback) =>
         setTimeout(
            () => cb(performance.now()),
            16
         ) as unknown as number) as typeof requestAnimationFrame

      const style = document.createElement('style')

      style.id = 'e2e-disable-motion'
      style.textContent = `
        *, *::before, *::after {
          animation: none !important;
          transition: none !important;
        }
      `

      document.head.append(style)
   })
}

/** Wait for the app to be ready (header visible). */
export async function waitForAppReady() {
   const header = $(sel.appHeader)

   await header.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
   await patchRequestAnimationFrame()
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
   await browser.waitUntil(async () => (await getResultRowCount()) > 0, {
      timeout: RESULTS_TIMEOUT,
      interval: 100,
      timeoutMsg: 'results rows did not appear within timeout',
   })
}

/** Count visible result rows in the current scan view. */
async function getResultRowCount(): Promise<number> {
   const rows = await $$(`${sel.rowFolder}, ${sel.rowFile}`)

   return await rows.length
}

/** Wait for a scan attempt to either render rows or fall back to Launch. */
async function waitForScanAttempt(): Promise<'results' | 'launch'> {
   let outcome: 'results' | 'launch' | null = null

   const startedAt = Date.now()

   await browser.waitUntil(
      async () => {
         if ((await getResultRowCount()) > 0) {
            outcome = 'results'

            return true
         }

         const isLaunchDisplayed = await $(sel.scanLaunch)
            .isDisplayed()
            .catch(() => false)

         if (isLaunchDisplayed && Date.now() - startedAt > SCAN_START_RETRY_DELAY_MS) {
            outcome = 'launch'

            return true
         }

         return false
      },
      {
         timeout: RESULTS_TIMEOUT,
         interval: 100,
         timeoutMsg: 'scan attempt did not produce results or return to launch',
      }
   )

   return outcome ?? 'launch'
}

// ---------------------------------------------------------------------------
// Scan helpers
// ---------------------------------------------------------------------------

/** Click Start Scan and wait for the results list to render. */
export async function scanAndWaitForResults() {
   for (let attempt = 1; attempt <= SCAN_ATTEMPTS; attempt++) {
      const startBtn = $(sel.startScan)

      await startBtn.waitForDisplayed({ timeout: ELEMENT_TIMEOUT })
      await startBtn.click()

      if ((await waitForScanAttempt()) === 'results') return

      if (attempt < SCAN_ATTEMPTS) {
         await browser.pause(TRANSITION_SETTLE_MS)
      }
   }

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
      const isDisplayed = await row.isDisplayed().catch(() => false)

      if (!isDisplayed) continue

      const text = await row.getText()

      if (text.includes(name)) return row
   }

   return null
}

/**
 * Polls for a result row until it appears (Cypress-style retry). Tolerates
 * the gap inside `<Transition mode="out-in">` on the list slide where the
 * leaving list is unmounted and the entering list hasn't yet mounted.
 *
 * Note: the navigation helpers (`navigateIntoFolder`, `navigateBack`,
 * `navigateForward`, `navigateBackToRoot`) are responsible for waiting until
 * the slide has settled before this is called; otherwise this can lock onto
 * a row from the leaving list and click on an about-to-detach node.
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
   await waitForListSlideSettled()
}

/** Click the back navigation button and wait for the transition to settle. */
export async function navigateBack() {
   const btn = $(sel.navBack)

   await btn.click()
   await waitForListSlideSettled()
}

/** Click the forward navigation button and wait for the transition to settle. */
export async function navigateForward() {
   const btn = $(sel.navForward)

   await btn.click()
   await waitForListSlideSettled()
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

/**
 * Reset app settings to defaults via the e2e Tauri command. The Rust side
 * emits `settings:reset` after writing, which the frontend store listens for
 * and uses to refresh its in-memory ref, so the UI is in sync without
 * having to replay toggle clicks (which used to fire app-slide transitions
 * and was a source of flake).
 */
export async function resetE2eState() {
   const err = await browser.executeAsync<string | null, []>((done: any) => {
      ;(window as any).__TAURI_INTERNALS__
         .invoke('reset_e2e_state')
         .then(() => done(null))
         .catch((e: unknown) => done(String(e && (e as Error).message ? (e as Error).message : e)))
   })

   if (err) throw new Error(`reset_e2e_state failed: ${err}`)
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

/** Wait for any pending Rust-side scan cancellation to finish before starting another scan. */
async function settleScanCancellation() {
   const err = await browser.executeAsync<string | null, []>((done: any) => {
      ;(window as any).__TAURI_INTERNALS__
         .invoke('cancel_scan')
         .then(() => done(null))
         .catch((e: unknown) => done(String(e && (e as Error).message ? (e as Error).message : e)))
   })

   if (err) throw new Error(`cancel_scan failed: ${err}`)
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
      await waitForListSlideSettled()
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
   await waitForListSlideSettled()
}

/** Click the Settings footer tab and wait for the settings view. */
export async function goToSettingsView() {
   await $(sel.footerSettings).click()

   const view = $(sel.settingsView)

   await view.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
   await waitForListSlideSettled()
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
      await settleScanCancellation()
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

   // Otherwise we're on the launch screen, so start a new scan.
   const launch = $(sel.scanLaunch)
   const onLaunch = await launch.isDisplayed().catch(() => false)

   if (onLaunch) {
      await scanAndWaitForResults()
   }
}
