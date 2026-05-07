// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { THEME_COLORS, ROOT_THEME } from '@/lib/constants'
import { log } from '@/lib/log'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref, type Ref } from 'vue'

import type { AppSettings, ThemeColor, Language } from '@/types/settings'

export interface AppSettingsStore {
   settings: Ref<AppSettings>
   getThemeColor: () => ThemeColor
   setLanguage: (lang: Language) => Promise<void>
   setThemeColor: (theme: ThemeColor) => Promise<void>
   setShowHiddenFiles: (value: boolean) => Promise<void>
   setShowUnder1Kb: (value: boolean) => Promise<void>
   setShowZeroByte: (value: boolean) => Promise<void>
   setAutoUpdates: (value: boolean) => Promise<void>
}

let globalStore: AppSettingsStore | null = null

/**
 * Initializes the app settings store. Call once in main.ts.
 */
export async function initTauriAppSettings(): Promise<AppSettingsStore> {
   if (globalStore) return globalStore

   // Load settings from backend, falling back to default theme if invalid
   const settingsData = await invoke<AppSettings>('get_settings')
   if (!THEME_COLORS.includes(settingsData.themeColor)) {
      settingsData.themeColor = ROOT_THEME
   }
   const settings = ref<AppSettings>(settingsData)

   // Backend-driven resets (e.g. e2e `reset_e2e_state`) bypass the UI, so the
   // local ref would otherwise drift. The listener keeps it in sync.
   listen<AppSettings>('settings:reset', (event) => {
      const next = event.payload
      if (!THEME_COLORS.includes(next.themeColor)) next.themeColor = ROOT_THEME
      settings.value = next
      log('settings', 'Settings: reset from backend')
   })

   async function saveSettings() {
      await invoke('set_settings', { settings: settings.value })
   }

   globalStore = {
      settings,
      getThemeColor: () => settings.value.themeColor,
      setLanguage: async (lang) => {
         const prev = settings.value.language
         settings.value = { ...settings.value, language: lang }
         log('settings', `Settings: language ${prev} → ${lang}`)
         await saveSettings()
         // Update macOS system locale (for context menus) and sync app menu language
         await invoke('set_app_locale', { language: lang })
      },
      setThemeColor: async (theme) => {
         const prev = settings.value.themeColor
         settings.value = { ...settings.value, themeColor: theme }
         log('settings', `themeColor: ${prev} → ${theme}`)
         await saveSettings()
      },
      setShowHiddenFiles: async (value) => {
         settings.value = { ...settings.value, showHiddenFiles: value }
         log('settings', `Settings: showHiddenFiles ${!value} → ${value}`)
         await saveSettings()
      },
      setShowUnder1Kb: async (value) => {
         settings.value = { ...settings.value, showUnder1Kb: value }
         log('settings', `Settings: showUnder1Kb ${!value} → ${value}`)
         await saveSettings()
      },
      setShowZeroByte: async (value) => {
         settings.value = { ...settings.value, showZeroByte: value }
         log('settings', `Settings: showZeroByte ${!value} → ${value}`)
         await saveSettings()
      },
      setAutoUpdates: async (value) => {
         settings.value = { ...settings.value, autoUpdates: value }
         log('settings', `Settings: autoUpdates ${!value} → ${value}`)
         await saveSettings()
      },
   }

   return globalStore
}

/**
 * Hook to use app settings. Returns the settings ref and methods to update them.
 * Must be called after initTauriAppSettings() has been awaited.
 */
export function useAppSettings(): AppSettingsStore {
   if (!globalStore) {
      throw new Error(
         'useAppSettings() called before initTauriAppSettings() was initialized in main.ts'
      )
   }

   return globalStore
}
