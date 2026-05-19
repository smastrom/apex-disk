// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * E2E: Settings ↔ Scan integration.
 *
 * Verifies scan-setting toggles persist in the store and are honoured by the
 * next scan. Also covers dependent toggles added to SettingsView such as
 * .DS_Store scanning and split automatic update preferences.
 *
 * Uses `reset_e2e_state` to restore defaults between tests. Filter behaviour is
 * verified through direct Tauri scans because the scan UI lifecycle is already
 * covered by scan-flow.spec.ts.
 */

import {
   waitForAppReady,
   waitForScanLaunch,
   goToSettingsView,
   goToScanView,
   resetE2eState,
   getToggleState,
   clickToggle,
   sel,
} from '../helpers/navigation'

interface TestFolderInfo {
   name: string
   children: TestFolderInfo[]
}

interface TestSettings {
   themeColor: string
   showHiddenFiles: boolean
   showDsStore: boolean
   showUnder1Kb: boolean
   showZeroByte: boolean
   autoCheckUpdates: boolean
   autoInstallUpdates: boolean
}

/** Invoke a Tauri command from the app webview and surface errors in Mocha. */
async function invokeTauri<T>(command: string, args?: Record<string, unknown>): Promise<T> {
   const result = await browser.executeAsync<
      { value?: T; error?: string },
      [string, Record<string, unknown> | undefined]
   >(
      (cmd: string, commandArgs: Record<string, unknown> | undefined, done: any) => {
         ;(window as any).__TAURI_INTERNALS__
            .invoke(cmd, commandArgs)
            .then((value: unknown) => done({ value }))
            .catch((e: unknown) =>
               done({ error: String(e && (e as Error).message ? (e as Error).message : e) })
            )
      },
      command,
      args
   )

   if (result.error) throw new Error(`${command} failed: ${result.error}`)

   return result.value as T
}

/** Read persisted scan settings so filter scans use the same values the app saved. */
async function getBackendSettings(): Promise<TestSettings> {
   return await invokeTauri<TestSettings>('get_settings')
}

/** Wait until async settings persistence catches up with a clicked toggle. */
async function waitForBackendSettings(expected: Partial<TestSettings>) {
   await browser.waitUntil(
      async () => {
         const settings = await getBackendSettings()

         return Object.entries(expected).every(
            ([key, value]) => settings[key as keyof TestSettings] === value
         )
      },
      {
         timeout: 5000,
         interval: 100,
         timeoutMsg: `settings did not persist: ${JSON.stringify(expected)}`,
      }
   )
}

/** Run the scanner directly with the currently persisted settings. */
async function scanWithCurrentSettings(): Promise<TestFolderInfo[]> {
   const settings = await getBackendSettings()

   return await invokeTauri<TestFolderInfo[]>('get_user_folders', {
      options: {
         show_hidden_files: settings.showHiddenFiles,
         show_ds_store: settings.showDsStore,
         show_under_1kb: settings.showUnder1Kb,
         show_zero_byte: settings.showZeroByte,
      },
   })
}

/** Find a direct child node by name in a scan result list. */
function findChild(items: TestFolderInfo[], name: string): TestFolderInfo | null {
   return items.find((item) => item.name === name) ?? null
}

/** Scan and return the deterministic MyData fixture folder. */
async function scanMyData(): Promise<TestFolderInfo> {
   const folders = await scanWithCurrentSettings()
   const myData = findChild(folders, 'MyData')

   if (!myData) throw new Error('MyData fixture folder not found')

   return myData
}

describe('Settings flow', () => {
   before(async () => {
      await waitForAppReady()
      await waitForScanLaunch()
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
         expect(await getToggleState(sel.settingsToggleDsStore)).toBe(false)
         expect(await getToggleState(sel.settingsToggleUnder1Kb)).toBe(false)
         expect(await getToggleState(sel.settingsToggleZeroByte)).toBe(false)
      })

      it('theme options include Apex Light', async () => {
         await goToSettingsView()

         const options = await $$(`${sel.settingsTheme} option`)
         const values: string[] = []

         for (const option of options) {
            values.push(await option.getAttribute('value'))
         }

         expect(values).toContain('apex-light')
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
   // Dependent settings
   // ─────────────────────────────────────────────────────────────────────

   describe('dependent toggles', () => {
      it('.DS_Store toggle is disabled until hidden files are enabled', async () => {
         await goToSettingsView()

         const dsStoreToggle = $(sel.settingsToggleDsStore)

         expect(await dsStoreToggle.getAttribute('disabled')).not.toBeNull()
         expect(await dsStoreToggle.getAttribute('aria-disabled')).toBe('true')

         await clickToggle(sel.settingsToggleHiddenFiles)
         await waitForBackendSettings({ showHiddenFiles: true })

         expect(await dsStoreToggle.getAttribute('disabled')).toBeNull()
         expect(await dsStoreToggle.getAttribute('aria-disabled')).toBe('false')
      })

      it('turning hidden files off also turns .DS_Store off', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleHiddenFiles)
         await waitForBackendSettings({ showHiddenFiles: true })
         await clickToggle(sel.settingsToggleDsStore)
         await waitForBackendSettings({ showDsStore: true })

         await clickToggle(sel.settingsToggleHiddenFiles)
         await waitForBackendSettings({ showHiddenFiles: false, showDsStore: false })

         expect(await getToggleState(sel.settingsToggleDsStore)).toBe(false)
      })

      it('auto-install updates is disabled until auto-check updates is enabled', async () => {
         await goToSettingsView()

         const autoInstallToggle = $(sel.settingsToggleAutoInstallUpdates)

         expect(await autoInstallToggle.getAttribute('disabled')).not.toBeNull()
         expect(await autoInstallToggle.getAttribute('aria-disabled')).toBe('true')

         await clickToggle(sel.settingsToggleAutoCheckUpdates)
         await waitForBackendSettings({ autoCheckUpdates: true })

         expect(await autoInstallToggle.getAttribute('disabled')).toBeNull()
         expect(await autoInstallToggle.getAttribute('aria-disabled')).toBe('false')
      })

      it('turning auto-check updates off also turns auto-install updates off', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleAutoCheckUpdates)
         await waitForBackendSettings({ autoCheckUpdates: true })
         await clickToggle(sel.settingsToggleAutoInstallUpdates)
         await waitForBackendSettings({ autoInstallUpdates: true })

         await clickToggle(sel.settingsToggleAutoCheckUpdates)
         await waitForBackendSettings({ autoCheckUpdates: false, autoInstallUpdates: false })

         expect(await getToggleState(sel.settingsToggleAutoInstallUpdates)).toBe(false)
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Hidden files
   // ─────────────────────────────────────────────────────────────────────

   describe('showHiddenFiles filter', () => {
      it('default (off): .hidden does not appear', async () => {
         const myData = await scanMyData()

         expect(findChild(myData.children, '.hidden')).toBeNull()
      })

      it('on (with under 1 KB also on, since .hidden is 50 B): .hidden appears', async () => {
         await goToSettingsView()
         // .hidden is 50 B, so the under-1 KB cutoff also hides it; both
         // toggles must be on for the file to show.
         await clickToggle(sel.settingsToggleHiddenFiles)
         await waitForBackendSettings({ showHiddenFiles: true })
         await clickToggle(sel.settingsToggleUnder1Kb)
         await waitForBackendSettings({ showUnder1Kb: true })

         const myData = await scanMyData()

         expect(findChild(myData.children, '.hidden')).not.toBeNull()
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Under 1 KB
   // ─────────────────────────────────────────────────────────────────────

   describe('showUnder1Kb filter', () => {
      it('default (off): small.txt (100 B) is filtered out', async () => {
         const myData = await scanMyData()

         expect(findChild(myData.children, 'small.txt')).toBeNull()
      })

      it('on: small.txt appears', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleUnder1Kb)
         await waitForBackendSettings({ showUnder1Kb: true })

         const myData = await scanMyData()

         expect(findChild(myData.children, 'small.txt')).not.toBeNull()
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Zero-byte
   // ─────────────────────────────────────────────────────────────────────

   describe('showZeroByte filter', () => {
      it('default (off): empty.txt (0 B) is filtered out', async () => {
         const myData = await scanMyData()

         expect(findChild(myData.children, 'empty.txt')).toBeNull()
      })

      it('on (with under 1 KB also on, so the size cutoff does not re-hide it): empty.txt appears', async () => {
         await goToSettingsView()
         // 0-byte files are under 1 KB, so both filters must be on for the
         // file to appear. This mirrors real scan behaviour.
         await clickToggle(sel.settingsToggleUnder1Kb)
         await waitForBackendSettings({ showUnder1Kb: true })
         await clickToggle(sel.settingsToggleZeroByte)
         await waitForBackendSettings({ showZeroByte: true })

         const myData = await scanMyData()

         expect(findChild(myData.children, 'empty.txt')).not.toBeNull()
      })
   })

   // ─────────────────────────────────────────────────────────────────────
   // Combined
   // ─────────────────────────────────────────────────────────────────────

   describe('combined filters', () => {
      it('hidden + under1Kb + zeroByte reveals all MyData files', async () => {
         await goToSettingsView()
         await clickToggle(sel.settingsToggleHiddenFiles)
         await waitForBackendSettings({ showHiddenFiles: true })
         await clickToggle(sel.settingsToggleUnder1Kb)
         await waitForBackendSettings({ showUnder1Kb: true })
         await clickToggle(sel.settingsToggleZeroByte)
         await waitForBackendSettings({ showZeroByte: true })

         const myData = await scanMyData()

         expect(findChild(myData.children, 'big.txt')).not.toBeNull()
         expect(findChild(myData.children, 'small.txt')).not.toBeNull()
         expect(findChild(myData.children, 'empty.txt')).not.toBeNull()
         expect(findChild(myData.children, '.hidden')).not.toBeNull()
      })
   })
})
