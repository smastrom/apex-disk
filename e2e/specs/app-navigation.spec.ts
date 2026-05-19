// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * E2E: App main view navigation (Scan ↔ Settings ↔ Information).
 * Ensures footer navigation works LEFT → RIGHT, RIGHT → LEFT, and each view renders its content.
 */

import {
   sel,
   goToScanView,
   goToSettingsView,
   waitForAppReady,
   waitForListSlideSettled,
   waitForScanLaunch,
} from '../helpers/navigation'

describe('App navigation', () => {
   const VIEW_READY_TIMEOUT = 10000
   const BUTTON_DISPLAY_TIMEOUT = 10000

   const settingsView = '[data-testid="settings-view"]'
   const settingsContent = '[data-testid="settings-content"]'
   const informationView = '[data-testid="information-view"]'
   const informationTitle = '[data-testid="information-title"]'
   const informationLicenseText = 'GPL-3.0'

   async function expectScanViewRendered() {
      await waitForScanLaunch()
   }

   async function expectSettingsViewRendered() {
      const main = $(settingsView)

      await main.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
      await expect(main).toBeDisplayed()

      const content = $(settingsContent)

      await content.waitForDisplayed({ timeout: BUTTON_DISPLAY_TIMEOUT })
      await expect(content).toBeDisplayed()
   }

   async function expectInformationViewRendered() {
      const main = $(informationView)

      await main.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
      await expect(main).toBeDisplayed()

      const title = $(informationTitle)

      await title.waitForDisplayed({ timeout: BUTTON_DISPLAY_TIMEOUT })
      await expect(title).toBeDisplayed()
   }

   async function expectInformationFooterRendered() {
      const license = $(sel.informationLicense)

      await license.waitForExist({ timeout: BUTTON_DISPLAY_TIMEOUT })
      await license.scrollIntoView()
      await license.waitForDisplayed({ timeout: BUTTON_DISPLAY_TIMEOUT })

      const text = await license.getText()

      expect(text).toContain(informationLicenseText)
   }

   before(async () => {
      await waitForAppReady()
      await waitForScanLaunch()
   })

   it('starts on Scan view and shows scan launch content', async () => {
      await expectScanViewRendered()
   })

   it('navigates LEFT → RIGHT: Scan → Settings → Information', async () => {
      await goToSettingsView()
      await expectSettingsViewRendered()

      const informationBtn = $(sel.footerInformation)

      await informationBtn.click()
      await waitForListSlideSettled()
      await expectInformationViewRendered()
      await expectInformationFooterRendered()
   })

   it('navigates RIGHT → LEFT: Information → Settings → Scan', async () => {
      await goToSettingsView()
      await expectSettingsViewRendered()

      await goToScanView()
      await expectScanViewRendered()
   })

   it('navigates forward again and back to Scan from Information', async () => {
      await goToSettingsView()
      await expectSettingsViewRendered()

      const informationBtn = $(sel.footerInformation)

      await informationBtn.click()
      await waitForListSlideSettled()
      await expectInformationViewRendered()

      await goToScanView()
      await expectScanViewRendered()
   })
})
