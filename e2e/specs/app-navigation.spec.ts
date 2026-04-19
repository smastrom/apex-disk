// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * E2E: App main view navigation (Scan ↔ Settings ↔ Information).
 * Ensures footer navigation works LEFT → RIGHT, RIGHT → LEFT, and each view renders its content.
 */

describe('App navigation', () => {
   const VIEW_READY_TIMEOUT = 10000
   const BUTTON_DISPLAY_TIMEOUT = 5000

   const appHeader = '[data-testid="app-header"]'
   const footerScan = '[data-testid="footer-scan"]'
   const footerSettings = '[data-testid="footer-settings"]'
   const footerInformation = '[data-testid="footer-information"]'
   const scanLaunch = '[data-testid="scan-launch"]'
   const startScanBtn = '[data-testid="start-scan"]'
   const settingsView = '[data-testid="settings-view"]'
   const settingsContent = '[data-testid="settings-content"]'
   const informationView = '[data-testid="information-view"]'
   const informationTitle = '[data-testid="information-title"]'

   async function expectScanViewRendered() {
      const launch = $(scanLaunch)
      await launch.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
      const btn = $(startScanBtn)
      await btn.waitForDisplayed({ timeout: BUTTON_DISPLAY_TIMEOUT })
      await expect(launch).toBeDisplayed()
      await expect(btn).toBeDisplayed()
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

   before(async () => {
      const header = $(appHeader)
      await header.waitForDisplayed({ timeout: VIEW_READY_TIMEOUT })
      const scanBtn = $(footerScan)
      await scanBtn.waitForDisplayed({ timeout: BUTTON_DISPLAY_TIMEOUT })
   })

   it('starts on Scan view and shows scan launch content', async () => {
      await expectScanViewRendered()
   })

   it('navigates LEFT → RIGHT: Scan → Settings → Information', async () => {
      const settingsBtn = $(footerSettings)
      await settingsBtn.click()
      await expectSettingsViewRendered()

      const informationBtn = $(footerInformation)
      await informationBtn.click()
      await expectInformationViewRendered()
   })

   it('navigates RIGHT → LEFT: Information → Settings → Scan', async () => {
      const settingsBtn = $(footerSettings)
      await settingsBtn.click()
      await expectSettingsViewRendered()

      const scanBtn = $(footerScan)
      await scanBtn.click()
      await expectScanViewRendered()
   })

   it('navigates forward again and back to Scan from Information', async () => {
      const settingsBtn = $(footerSettings)
      await settingsBtn.click()
      await expectSettingsViewRendered()

      const informationBtn = $(footerInformation)
      await informationBtn.click()
      await expectInformationViewRendered()

      const scanBtn = $(footerScan)
      await scanBtn.click()
      await expectScanViewRendered()
   })
})
