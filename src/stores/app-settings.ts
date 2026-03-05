import { invoke } from '@tauri-apps/api/core'
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
}

let globalStore: AppSettingsStore | null = null

/**
 * Initializes the app settings store. Call once in main.ts.
 */
export async function initTauriAppSettings(): Promise<AppSettingsStore> {
   if (globalStore) return globalStore

   // Load settings from backend
   const settingsData = await invoke<AppSettings>('get_settings')
   const settings = ref<AppSettings>(settingsData)

   async function saveSettings() {
      await invoke('set_settings', { settings: settings.value })
   }

   globalStore = {
      settings,
      getThemeColor: () => settings.value.themeColor,
      setLanguage: async (lang) => {
         settings.value = { ...settings.value, language: lang }
         await saveSettings()
         // Update macOS system locale (for context menus) and sync app menu language
         await invoke('set_app_locale', { language: lang })
      },
      setThemeColor: async (theme) => {
         settings.value = { ...settings.value, themeColor: theme }
         await saveSettings()
      },
      setShowHiddenFiles: async (value) => {
         settings.value = { ...settings.value, showHiddenFiles: value }
         await saveSettings()
      },
      setShowUnder1Kb: async (value) => {
         settings.value = { ...settings.value, showUnder1Kb: value }
         await saveSettings()
      },
      setShowZeroByte: async (value) => {
         settings.value = { ...settings.value, showZeroByte: value }
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
