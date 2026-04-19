/**
 * E2E: Settings ↔ Scan integration.
 *
 * Verifies that the three scan-settings toggles (hidden files, under 1 KB,
 * zero-byte) persist in the store and are honoured by the next scan.
 *
 * Uses `reset_e2e_state` to restore defaults between tests and `rescanFresh`
 * to re-run the scan after toggling a filter (settings only take effect on
 * the next scan, by design — surfaced via the "effective next scan" notice).
 */

import {
   sel,
   waitForAppReady,
   waitForScanLaunch,
   scanAndWaitForResults,
   navigateIntoFolder,
   assertRowExists,
   assertRowNotExists,
   goToSettingsView,
   goToScanView,
   resetE2eState,
   getToggleState,
   clickToggle,
   rescanFresh,
} from '../helpers/navigation'

describe('Settings flow', () => {
   before(async () => {
      await waitForAppReady()
      await waitForScanLaunch()
      await scanAndWaitForResults()
   })

   beforeEach(async () => {
      await resetE2eState()
   })

   // ─────────────────────────────────────────────────────────────────────
   // Defaults and UI persistence
   // ─────────────────────────────────────────────────────────────────────

   describe('default state', () => {
      it('all scan toggles default to off', async () => {
         await goToSettingsView()
         expect(await getToggleState(sel.settingsToggleHiddenFiles)).toBe(false)
         expect(await getToggleState(sel.settingsToggleUnder1Kb)).toBe(false)
         expect(await getToggleState(sel.settingsToggleZeroByte)).toBe(false)
      })

      it('toggling hidden-files persists across a view switch', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleHiddenFiles)
         expect(await getToggleState(sel.settingsToggleHiddenFiles)).toBe(true)

         await goToScanView()
         await goToSettingsView()
         expect(await getToggleState(sel.settingsToggleHiddenFiles)).toBe(true)
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Hidden files
   // ─────────────────────────────────────────────────────────────────────

   describe('showHiddenFiles filter', () => {
      it('default (off): .hidden does not appear', async () => {
         await rescanFresh()
         await navigateIntoFolder('MyData')
         await assertRowNotExists('.hidden')
      })

      it('on (with under 1 KB also on, since .hidden is 50 B): .hidden appears', async () => {
         await goToSettingsView()
         // .hidden is 50 B, so the under-1 KB cutoff also hides it — both
         // toggles must be on for the file to show.
         await clickToggle(sel.settingsToggleHiddenFiles)
         await clickToggle(sel.settingsToggleUnder1Kb)

         await rescanFresh()
         await navigateIntoFolder('MyData')
         await assertRowExists('.hidden')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Under 1 KB
   // ─────────────────────────────────────────────────────────────────────

   describe('showUnder1Kb filter', () => {
      it('default (off): small.txt (100 B) is filtered out', async () => {
         await rescanFresh()
         await navigateIntoFolder('MyData')
         await assertRowNotExists('small.txt')
      })

      it('on: small.txt appears', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleUnder1Kb)

         await rescanFresh()
         await navigateIntoFolder('MyData')
         await assertRowExists('small.txt')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Zero-byte
   // ─────────────────────────────────────────────────────────────────────

   describe('showZeroByte filter', () => {
      it('default (off): empty.txt (0 B) is filtered out', async () => {
         await rescanFresh()
         await navigateIntoFolder('MyData')
         await assertRowNotExists('empty.txt')
      })

      it('on (with under 1 KB also on, so the size cutoff does not re-hide it): empty.txt appears', async () => {
         await goToSettingsView()
         // 0-byte files are under 1 KB, so both filters must be on for the
         // file to appear. This mirrors real scan behaviour.
         await clickToggle(sel.settingsToggleUnder1Kb)
         await clickToggle(sel.settingsToggleZeroByte)

         await rescanFresh()
         await navigateIntoFolder('MyData')
         await assertRowExists('empty.txt')
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Combined
   // ─────────────────────────────────────────────────────────────────────

   describe('combined filters', () => {
      it('hidden + under1Kb + zeroByte reveals all MyData files', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleHiddenFiles)
         await clickToggle(sel.settingsToggleUnder1Kb)
         await clickToggle(sel.settingsToggleZeroByte)

         await rescanFresh()
         await navigateIntoFolder('MyData')

         await assertRowExists('big.txt')
         await assertRowExists('small.txt')
         await assertRowExists('empty.txt')
         await assertRowExists('.hidden')
      })
   })
})
