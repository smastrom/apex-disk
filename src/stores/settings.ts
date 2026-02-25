import { load } from '@tauri-apps/plugin-store'
import { ref, type Ref } from 'vue'

import { getSystemLanguage } from '@/lib/systemDefaults'

import { DEFAULT_SETTINGS } from '@/types/settings'

import type { AppSettings, ThemeColor, Language } from '@/types/settings'

const STORE_PATH = 'settings.json'

/** Injection key for the settings store. Provided value is Ref<SettingsStore | null>. */
export const SETTINGS_KEY = Symbol('settings')

export interface SettingsStore {
   settings: Ref<AppSettings>
   getThemeColor: () => string
   setLanguage: (lang: Language) => Promise<void>
   setThemeColor: (theme: ThemeColor) => Promise<void>
   setShowHiddenFiles: (value: boolean) => Promise<void>
   setShowZeroByteFiles: (value: boolean) => Promise<void>
   setShowZeroByteFolders: (value: boolean) => Promise<void>
   setEnableAnimations: (value: boolean) => Promise<void>
   load: () => Promise<void>
}

function createStoreFromSettings(
   settings: Ref<AppSettings>,
   persist: () => Promise<void>
): SettingsStore {
   return {
      settings,
      getThemeColor: () => settings.value.themeColor,
      setLanguage: async (lang) => {
         settings.value = { ...settings.value, language: lang }
         await persist()
      },
      setThemeColor: async (theme) => {
         settings.value = { ...settings.value, themeColor: theme }
         await persist()
      },
      setShowHiddenFiles: async (value) => {
         settings.value = { ...settings.value, showHiddenFiles: value }
         await persist()
      },
      setShowZeroByteFiles: async (value) => {
         settings.value = { ...settings.value, showZeroByteFiles: value }
         await persist()
      },
      setShowZeroByteFolders: async (value) => {
         settings.value = { ...settings.value, showZeroByteFolders: value }
         await persist()
      },
      setEnableAnimations: async (value) => {
         settings.value = { ...settings.value, enableAnimations: value }
         await persist()
      },
      load: async () => {},
   }
}

const LOAD_TIMEOUT_MS = 3000

/** Creates and initializes the settings store. Call once at app root, then inject. */
export async function createSettingsStore(): Promise<SettingsStore> {
   try {
      const store = await Promise.race([
         load(STORE_PATH, { autoSave: true, defaults: {} }),
         new Promise<never>((_, reject) =>
            setTimeout(() => reject(new Error('Store load timeout')), LOAD_TIMEOUT_MS)
         ),
      ])
      const raw = (await store.get('app')) as Partial<AppSettings> | null
      const systemLanguage = await getSystemLanguage()
      const settings = ref<AppSettings>({
         ...DEFAULT_SETTINGS,
         language: raw?.language ?? systemLanguage,
         ...(raw && typeof raw === 'object' ? raw : {}),
      })

      async function persist() {
         await store.set('app', settings.value)
         await store.save()
      }

      const result = createStoreFromSettings(settings, persist)
      result.load = async () => {
         await store.reload()
         const raw = (await store.get('app')) as Partial<AppSettings> | null
         if (raw && typeof raw === 'object') {
            const systemLanguage = await getSystemLanguage()
            settings.value = {
               ...DEFAULT_SETTINGS,
               language: raw.language ?? systemLanguage,
               ...raw,
            }
         }
      }
      return result
   } catch {
      const systemLanguage = await getSystemLanguage()
      const settings = ref<AppSettings>({
         ...DEFAULT_SETTINGS,
         language: systemLanguage,
      })
      return createStoreFromSettings(settings, async () => {})
   }
}
