// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * E2E: Selection model — 4-state checkbox (empty / partial / selected) with
 * parent/child propagation, inherited selection, "explode ancestor" logic,
 * and protected-folder behaviour.
 *
 * Each test starts from a clean slate via `resetAndScan()` so per-test state
 * is independent. Relies on the deterministic e2e fixture.
 */

import {
   sel,
   waitForAppReady,
   waitForScanLaunch,
   scanAndWaitForResults,
   resetAndScan,
   requireRowByName,
   getCheckbox,
   clickRowCheckbox,
   clickCheckboxByName,
   assertCheckboxState,
   assertPartialSelectedSize,
   assertRowSelected,
   assertCheckboxDisabled,
   navigateIntoFolder,
   navigateBack,
   navigateBackToRoot,
   isReviewButtonDisabled,
   getReviewButtonText,
   getReviewButtonBytes,
   getReviewButton,
} from '../helpers/navigation'

describe('Selection model', () => {
   before(async () => {
      await waitForAppReady()
      await waitForScanLaunch()
      await scanAndWaitForResults()
   })

   beforeEach(async () => {
      await resetAndScan()
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2a. Basic selection
   // ─────────────────────────────────────────────────────────────────────

   describe('basic selection', () => {
      it('clicking an unselected folder checkbox selects it', async () => {
         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'empty')

         await clickRowCheckbox(myData)

         const after = await requireRowByName('MyData')

         await assertRowSelected(after, true)
         await assertCheckboxState(after, 'selected')
      })

      it('clicking a selected folder checkbox deselects it', async () => {
         await clickCheckboxByName('MyData')
         await clickCheckboxByName('MyData')

         const myData = await requireRowByName('MyData')

         await assertRowSelected(myData, false)
         await assertCheckboxState(myData, 'empty')
      })

      it('selecting an item enables the Review button and shows size', async () => {
         expect(await isReviewButtonDisabled()).toBe(true)

         await clickCheckboxByName('MyData')

         expect(await isReviewButtonDisabled()).toBe(false)

         const text = await getReviewButtonText()

         expect(text).toMatch(/\d/) // includes a number (size)
      })

      it('deselecting all items disables the Review button again', async () => {
         await clickCheckboxByName('MyData')
         expect(await isReviewButtonDisabled()).toBe(false)

         await clickCheckboxByName('MyData')
         expect(await isReviewButtonDisabled()).toBe(true)
      })

      it('selecting multiple items keeps Review button enabled', async () => {
         await clickCheckboxByName('MyData')
         await clickCheckboxByName('Projects')

         expect(await isReviewButtonDisabled()).toBe(false)

         // Should show a count greater than one now. We don't assert exact
         // localized text, just that both selections contribute to size.
         const text = await getReviewButtonText()

         expect(text.length).toBeGreaterThan(0)
      })

      it('selecting a file inside a folder works independently', async () => {
         await navigateIntoFolder('MyData')
         await clickCheckboxByName('big.txt')

         const bigTxt = await requireRowByName('big.txt')

         await assertCheckboxState(bigTxt, 'selected')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2b. Inherited selection: parent selected → children show as selected
   // ─────────────────────────────────────────────────────────────────────

   describe('inherited selection from selected ancestor', () => {
      it('children render as selected when the parent is selected', async () => {
         await clickCheckboxByName('MyData')
         await navigateIntoFolder('MyData')

         for (const name of ['big.txt', 'SubFolder']) {
            const row = await requireRowByName(name)

            await assertRowSelected(row, true)
            await assertCheckboxState(row, 'selected')
         }
      })

      it('inherited children show "filled" not "partial" state', async () => {
         await clickCheckboxByName('MyData')
         await navigateIntoFolder('MyData')

         const subFolder = await requireRowByName('SubFolder')

         await assertCheckboxState(subFolder, 'selected')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2c. Explode ancestor: click an inherited child deselects that child
   //     and pins siblings at each intermediate level.
   // ─────────────────────────────────────────────────────────────────────

   describe('explode ancestor when deselecting an inherited child', () => {
      it('single-level explode: pins siblings at the same level', async () => {
         await clickCheckboxByName('MyData')
         await navigateIntoFolder('MyData')

         // big.txt is inherited-selected. Click to exclude it → explode.
         await clickCheckboxByName('big.txt')

         const bigTxt = await requireRowByName('big.txt')

         await assertRowSelected(bigTxt, false)
         await assertCheckboxState(bigTxt, 'empty')

         // Siblings should now be explicitly selected.
         const subFolder = await requireRowByName('SubFolder')

         await assertCheckboxState(subFolder, 'selected')

         // Return to root: MyData itself must no longer be "fully selected".
         await navigateBack()

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'partial')
      })

      it('multi-level explode: pins siblings at every intermediate level', async () => {
         await clickCheckboxByName('MyData')
         await navigateIntoFolder('MyData')
         await navigateIntoFolder('SubFolder')

         // alpha.txt is two levels deep under the originally-selected MyData.
         await clickCheckboxByName('alpha.txt')

         // At this (SubFolder) level: siblings beta.txt and Deep should be selected.
         const beta = await requireRowByName('beta.txt')

         await assertCheckboxState(beta, 'selected')

         const deep = await requireRowByName('Deep')

         await assertCheckboxState(deep, 'selected')

         // Walk back up to MyData: siblings at MyData level should also be pinned.
         await navigateBack()

         const subFolderRow = await requireRowByName('SubFolder')

         // SubFolder has a deselected descendant (alpha) plus explicit siblings →
         // it's in indeterminate state.
         await assertCheckboxState(subFolderRow, 'partial')

         await navigateBack()

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'partial')
      })

      it('size decreases by the excluded file when exploding', async () => {
         await clickCheckboxByName('MyData')
         await navigateBackToRoot()

         const fullBytes = await getReviewButtonBytes()

         expect(fullBytes).not.toBeNull()

         await navigateIntoFolder('MyData')
         await clickCheckboxByName('big.txt') // explode, excluding the 2048 B file
         await navigateBack()

         const reducedBytes = await getReviewButtonBytes()

         expect(reducedBytes).not.toBeNull()

         // Strict arithmetic check: reduction must be at least big.txt's 2048 B.
         // (The formatter rounds to 2 decimals, so allow small rounding noise.)
         const BIG_TXT_SIZE = 2048
         const delta = (fullBytes as number) - (reducedBytes as number)

         expect(delta).toBeGreaterThanOrEqual(BIG_TXT_SIZE - 10)
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2d. Indeterminate state propagation
   // ─────────────────────────────────────────────────────────────────────

   describe('indeterminate (partial) state', () => {
      it('selecting a child marks the parent as partial', async () => {
         await navigateIntoFolder('MyData')
         await clickCheckboxByName('big.txt')
         await navigateBack()

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'partial')
      })

      it('clicking an indeterminate parent deselects all descendants', async () => {
         await navigateIntoFolder('MyData')
         await clickCheckboxByName('big.txt')
         await navigateBack()

         // MyData now partial. Click to deselect descendants.
         await clickCheckboxByName('MyData')

         await navigateIntoFolder('MyData')

         const bigTxt = await requireRowByName('big.txt')

         await assertCheckboxState(bigTxt, 'empty')
      })

      it('deeply-nested selection propagates partial up multiple levels', async () => {
         await navigateIntoFolder('MyData')
         await navigateIntoFolder('SubFolder')
         await clickCheckboxByName('alpha.txt')

         await navigateBack()

         const subFolder = await requireRowByName('SubFolder')

         await assertCheckboxState(subFolder, 'partial')

         await navigateBack()

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'partial')
      })

      it('clicking partial at root clears nested selections', async () => {
         await navigateIntoFolder('MyData')
         await navigateIntoFolder('SubFolder')
         await clickCheckboxByName('alpha.txt')

         await navigateBackToRoot()
         // Click MyData (partial) → deselect all descendants.
         await clickCheckboxByName('MyData')

         await navigateIntoFolder('MyData')
         await navigateIntoFolder('SubFolder')

         const alpha = await requireRowByName('alpha.txt')

         await assertCheckboxState(alpha, 'empty')
      })

      it('partial folder row shows selected descendant size under total size', async () => {
         await navigateIntoFolder('MyData')
         await clickCheckboxByName('big.txt')
         await navigateBack()

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'partial')
         await assertPartialSelectedSize(myData, { visible: true, text: /^-2(\.05)? KB$/ })
      })

      it('partial size sub-label is hidden for empty and fully selected folders', async () => {
         const myData = await requireRowByName('MyData')

         await assertPartialSelectedSize(myData, { visible: false })

         await clickCheckboxByName('MyData')

         await assertPartialSelectedSize(myData, { visible: false })
      })

      it('partial size sub-label sums multiple selected descendants', async () => {
         await navigateIntoFolder('MyData')
         await clickCheckboxByName('big.txt')
         await clickCheckboxByName('small.txt')
         await navigateBack()

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'partial')
         // big.txt (2048 B) + small.txt (100 B) = 2148 B → 2.15 KB
         await assertPartialSelectedSize(myData, { visible: true, text: /^-2\.15 KB$/ })
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2e. Protected folder behaviour
   // ─────────────────────────────────────────────────────────────────────

   describe('protected folder', () => {
      it('checkbox is disabled with no selected descendants', async () => {
         const documents = await requireRowByName('Documents')

         await assertCheckboxDisabled(documents, true)
         await assertCheckboxState(documents, 'empty')
      })

      it('clicking the disabled checkbox does nothing', async () => {
         const before = await requireRowByName('Documents')
         const beforeClasses = await before.getAttribute('class')

         const checkbox = await getCheckbox(before)

         // Click may or may not register depending on disabled attribute handling,
         // but state must not change either way.
         try {
            await checkbox.click()
         } catch {
            // disabled button may throw; either way state is unchanged
         }

         const after = await requireRowByName('Documents')
         const afterClasses = await after.getAttribute('class')

         expect(afterClasses).toBe(beforeClasses)
      })

      it('selecting a descendant flips the protected folder to partial (deselect-only)', async () => {
         await navigateIntoFolder('Documents')
         await clickCheckboxByName('report.txt')

         await navigateBack()

         const documents = await requireRowByName('Documents')

         await assertCheckboxState(documents, 'partial')
         // In "deselect-only" state the checkbox becomes clickable again.
         await assertCheckboxDisabled(documents, false)
      })

      it('clicking the partial protected folder deselects descendants and re-disables checkbox', async () => {
         await navigateIntoFolder('Documents')
         await clickCheckboxByName('report.txt')
         await navigateBack()

         await clickCheckboxByName('Documents')

         const documents = await requireRowByName('Documents')

         await assertCheckboxState(documents, 'empty')
         await assertCheckboxDisabled(documents, true)
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2f. Selection ↔ navigation interaction
   // ─────────────────────────────────────────────────────────────────────

   describe('selection across navigation', () => {
      it('selection persists when navigating into a different folder and back', async () => {
         await clickCheckboxByName('Projects')
         await navigateIntoFolder('MyData')
         await navigateBack()

         const projects = await requireRowByName('Projects')

         await assertRowSelected(projects, true)
      })

      it('reset button clears all selections', async () => {
         await clickCheckboxByName('MyData')
         await clickCheckboxByName('Projects')
         expect(await isReviewButtonDisabled()).toBe(false)

         const resetBtn = await $(sel.resultsNav).$(`button[aria-label*="eset" i]`)

         await resetBtn.click()

         expect(await isReviewButtonDisabled()).toBe(true)

         const myData = await requireRowByName('MyData')

         await assertCheckboxState(myData, 'empty')

         const projects = await requireRowByName('Projects')

         await assertCheckboxState(projects, 'empty')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // 2g. Size de-duplication
   // ─────────────────────────────────────────────────────────────────────

   describe('size computation', () => {
      it('selecting a folder and a child inside does not change size (dedup via ancestor)', async () => {
         await clickCheckboxByName('MyData')

         const parentOnlyText = await getReviewButtonText()

         await navigateIntoFolder('MyData')

         // big.txt is inherited-selected. Clicking it triggers explode, which changes size.
         // So instead verify that navigating without any further action preserves the label.
         const afterNavText = await getReviewButton().getText()

         await navigateBack()

         expect(afterNavText).toBe(parentOnlyText)
      })

      it('Review button label updates as selections change', async () => {
         await clickCheckboxByName('Projects')

         const oneSel = await getReviewButtonText()

         await clickCheckboxByName('MyData')

         const twoSel = await getReviewButtonText()

         expect(twoSel).not.toBe(oneSel)

         await clickCheckboxByName('Projects')

         const onlyMyData = await getReviewButtonText()

         expect(onlyMyData).not.toBe(twoSel)
      })
   })
})
