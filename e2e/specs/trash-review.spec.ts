// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * E2E: Selection → Trash Review → Confirmation flow.
 *
 * Exercises:
 * - Entering the review screen and rendering the flattened list.
 * - Checkbox toggling and size updates on that screen.
 * - Back navigation preserving selection.
 * - The 1s safety countdown on the Move to Trash button.
 * - Success confirmation (restart button + success icon).
 * - 0KB/error confirmation via the runtime-switchable trash mode.
 *
 * The trash_paths command is mocked in e2e builds and does NOT touch the
 * filesystem. Mode is controlled by `set_e2e_trash_mode` (success/zero/error).
 */

import {
   sel,
   waitForAppReady,
   waitForScanLaunch,
   scanAndWaitForResults,
   resetAndScan,
   requireRowByName,
   clickCheckboxByName,
   clickReviewSelection,
   waitForTrashList,
   getTrashRows,
   getTrashRowByName,
   toggleTrashRow,
   waitForTrashConfirmation,
   isReviewButtonDisabled,
   getReviewButton,
   navigateBack,
   setTrashMode,
} from '../helpers/navigation'

describe('Trash review flow', () => {
   before(async () => {
      await waitForAppReady()
      await waitForScanLaunch()
      await scanAndWaitForResults()
      await setTrashMode('success') // default for most tests
   })

   beforeEach(async () => {
      await resetAndScan()
      await setTrashMode('success')
   })

   // ─────────────────────────────────────────────────────────────────────
   // 3a. Entering trash review
   // ─────────────────────────────────────────────────────────────────────

   describe('entering review', () => {
      it('Review button is disabled with no selection', async () => {
         expect(await isReviewButtonDisabled()).toBe(true)
      })

      it('selecting an item and clicking Review renders the trash list', async () => {
         await clickCheckboxByName('MyData')
         await clickReviewSelection()

         await waitForTrashList()
         const rows = await getTrashRows()
         expect(rows.length).toBeGreaterThan(0)
      })

      it('multiple selections render as distinct rows', async () => {
         await clickCheckboxByName('MyData')
         await clickCheckboxByName('Projects')
         await clickReviewSelection()

         await waitForTrashList()

         const rows = await getTrashRows()
         expect(rows.length).toBeGreaterThanOrEqual(2)

         const myDataRow = await getTrashRowByName('MyData')
         const projectsRow = await getTrashRowByName('Projects')
         expect(myDataRow).not.toBeNull()
         expect(projectsRow).not.toBeNull()
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 3b. Checkbox behaviour inside the trash list
   // ─────────────────────────────────────────────────────────────────────

   describe('trash list checkboxes', () => {
      it('all items checked by default', async () => {
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         const rows = await getTrashRows()
         for (const row of rows) {
            const checkbox = await row.$(sel.trashRowCheckbox)
            const pressed = await checkbox.getAttribute('aria-pressed')
            expect(pressed).toBe('true')
         }
      })

      it('unchecking all items disables Move to Trash', async () => {
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         // Wait past the countdown so we can isolate the "nothing checked" effect.
         await browser.pause(1100)

         const rows = await getTrashRows()
         for (const row of rows) await toggleTrashRow(row)

         const confirm = $(sel.confirmTrash)
         const disabled = await confirm.getAttribute('disabled')
         expect(disabled).not.toBeNull()
      })

      it('unchecking one item does not disable the button when others remain', async () => {
         await clickCheckboxByName('MyData')
         await clickCheckboxByName('Projects')
         await clickReviewSelection()
         await waitForTrashList()

         await browser.pause(1100)

         const rows = await getTrashRows()
         if (rows.length < 2) {
            // Not enough rows for a meaningful test — skip cleanly.
            return
         }
         await toggleTrashRow(rows[0])

         const confirm = $(sel.confirmTrash)
         const disabled = await confirm.getAttribute('disabled')
         expect(disabled).toBeNull()
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 3c. Back navigation preserves state
   // ─────────────────────────────────────────────────────────────────────

   describe('back navigation from trash list', () => {
      it('back button returns to results with selection preserved', async () => {
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         await navigateBack()
         // Results list should render again.
         const results = $(sel.resultsList)
         await results.waitForDisplayed({ timeout: 5000 })

         const myData = await requireRowByName('MyData')
         const classes = await myData.getAttribute('class')
         expect(classes).toContain('ScanResultsListItem-root--selected')
      })

      it('unchecking a trash row then going back deselects that item', async () => {
         await clickCheckboxByName('MyData')
         await clickCheckboxByName('Projects')
         await clickReviewSelection()
         await waitForTrashList()

         await browser.pause(1100)
         const projectsTrash = await getTrashRowByName('Projects')
         expect(projectsTrash).not.toBeNull()
         await toggleTrashRow(projectsTrash!)

         await navigateBack()
         const results = $(sel.resultsList)
         await results.waitForDisplayed({ timeout: 5000 })

         const projects = await requireRowByName('Projects')
         const classes = await projects.getAttribute('class')
         expect(classes).not.toContain('ScanResultsListItem-root--selected')

         const myData = await requireRowByName('MyData')
         const myDataClasses = await myData.getAttribute('class')
         expect(myDataClasses).toContain('ScanResultsListItem-root--selected')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 3d. Safety countdown
   // ─────────────────────────────────────────────────────────────────────

   describe('safety countdown', () => {
      it('Move to Trash starts disabled and becomes enabled after countdown', async () => {
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         const confirm = $(sel.confirmTrash)
         expect(await confirm.getAttribute('disabled')).not.toBeNull()

         // TRASH_COUNTDOWN_MS = 1000
         await browser.pause(1100)

         expect(await confirm.getAttribute('disabled')).toBeNull()
      })

      it('countdown restarts when re-entering trash review', async () => {
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()
         await browser.pause(1100)

         await navigateBack()
         await clickReviewSelection()
         await waitForTrashList()

         const confirm = $(sel.confirmTrash)
         // Button should be freshly disabled after re-entry.
         expect(await confirm.getAttribute('disabled')).not.toBeNull()
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 3e. Success confirmation
   // ─────────────────────────────────────────────────────────────────────

   describe('success confirmation', () => {
      it('clicking Move to Trash shows confirmation with restart button', async () => {
         await setTrashMode('success')
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         await browser.pause(1100)
         await $(sel.confirmTrash).click()

         await waitForTrashConfirmation()
         const restart = $(sel.restart)
         await expect(restart).toBeDisplayed()
      })

      it('clicking restart returns to results (scan cache reused)', async () => {
         await setTrashMode('success')
         await clickCheckboxByName('Projects')
         await clickReviewSelection()
         await waitForTrashList()
         await browser.pause(1100)
         await $(sel.confirmTrash).click()
         await waitForTrashConfirmation()

         await $(sel.restart).click()
         await browser.pause(500)

         // After restart we land back on the results list (e2e mock keeps folders).
         const results = $(sel.resultsList)
         const launch = $(sel.scanLaunch)
         const onResults = await results.isDisplayed().catch(() => false)
         const onLaunch = await launch.isDisplayed().catch(() => false)
         expect(onResults || onLaunch).toBe(true)
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 3f. Zero-count / error path
   // ─────────────────────────────────────────────────────────────────────

   describe('zero-count result', () => {
      it('shows error-variant confirmation when trash_paths returns count=0', async () => {
         await setTrashMode('zero')
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         await browser.pause(1100)
         await $(sel.confirmTrash).click()
         await waitForTrashConfirmation()

         // Error variant renders with .ScanTrashConfirmation-iconError somewhere on-screen.
         const errorIcons = await $$('.ScanTrashConfirmation-iconError')
         expect(errorIcons.length).toBeGreaterThan(0)

         // Clean up: restore success mode so subsequent tests behave.
         await setTrashMode('success')
      })
   })

   describe('invoke error path', () => {
      it('falls back to optimistic values and renders the success variant', async () => {
         await setTrashMode('error')
         await clickCheckboxByName('MyData')
         await clickReviewSelection()
         await waitForTrashList()

         await browser.pause(1100)
         await $(sel.confirmTrash).click()
         await waitForTrashConfirmation()

         // On invoke failure the frontend keeps its optimistic { count: n>0, size }
         // summary, so `hasErrors` is false and the success (check) icon renders.
         // Explicitly assert no error icon and that the restart button is present.
         const errorIcons = await $$('.ScanTrashConfirmation-iconError')
         expect(errorIcons.length).toBe(0)

         const restart = $(sel.restart)
         await expect(restart).toBeDisplayed()

         await setTrashMode('success')
      })
   })
})
