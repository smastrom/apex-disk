import { load } from '@tauri-apps/plugin-store'
import { invoke } from '@tauri-apps/api/core'
import { ref, type Ref } from 'vue'

import { DEFAULT_SETTINGS, THEME_COLORS } from '@/lib/constants'

import type { AppSettings, ThemeColor, Language } from '@/types/settings'

/** Normalizes stored theme value. */
function normalizeThemeColor(value: unknown): ThemeColor {
   if (typeof value === 'string' && (THEME_COLORS as readonly string[]).includes(value)) {
      return value as ThemeColor
   }

   return DEFAULT_SETTINGS.themeColor
}

/** Legacy keys that may exist in stored settings. */
interface LegacyAppSettings extends Partial<AppSettings> {
   showZeroByteFiles?: boolean
   showZeroByteFolders?: boolean
   moveToTrash?: boolean
}

/** Builds AppSettings from stored raw. */
function normalizeStoredSettings(raw: Partial<LegacyAppSettings> | null): AppSettings {
   const base = raw && typeof raw === 'object' ? raw : {}
   const showZeroByte =
      (base as Partial<AppSettings>).showZeroByte ??
      (base.showZeroByteFiles || base.showZeroByteFolders) ??
      DEFAULT_SETTINGS.showZeroByte

   return {
      language: base.language ?? DEFAULT_SETTINGS.language,
      themeColor:
         base.themeColor != null
            ? normalizeThemeColor(base.themeColor)
            : DEFAULT_SETTINGS.themeColor,
      permanentlyDelete:
         (base as Partial<AppSettings>).permanentlyDelete ??
         (base.moveToTrash !== undefined ? !base.moveToTrash : DEFAULT_SETTINGS.permanentlyDelete),
      showHiddenFiles: base.showHiddenFiles ?? DEFAULT_SETTINGS.showHiddenFiles,
      showUnder1Kb: base.showUnder1Kb ?? DEFAULT_SETTINGS.showUnder1Kb,
      showZeroByte,
   }
}

const STORE_PATH = 'settings.json'

export interface AppSettingsStore {
   settings: Ref<AppSettings>
   getThemeColor: () => ThemeColor
   setLanguage: (lang: Language) => Promise<void>
   setThemeColor: (theme: ThemeColor) => Promise<void>
   setPermanentlyDelete: (value: boolean) => Promise<void>
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

   const store = await load(STORE_PATH, { autoSave: true, defaults: {} })

   const raw = (await store.get('app')) as Partial<LegacyAppSettings> | null
   const settings = ref<AppSettings>(normalizeStoredSettings(raw))

   async function persist() {
      await store.set('app', settings.value)
      await store.save()
   }

   globalStore = {
      settings,
      getThemeColor: () => settings.value.themeColor,
      setLanguage: async (lang) => {
         settings.value = { ...settings.value, language: lang }
         await persist()
         // Update macOS system locale (for context menus) and sync app menu language
         await invoke('set_app_locale', { language: lang })
      },
      setThemeColor: async (theme) => {
         settings.value = { ...settings.value, themeColor: theme }
         await persist()
      },
      setPermanentlyDelete: async (value) => {
         settings.value = { ...settings.value, permanentlyDelete: value }
         await persist()
      },
      setShowHiddenFiles: async (value) => {
         settings.value = { ...settings.value, showHiddenFiles: value }
         await persist()
      },
      setShowUnder1Kb: async (value) => {
         settings.value = { ...settings.value, showUnder1Kb: value }
         await persist()
      },
      setShowZeroByte: async (value) => {
         settings.value = { ...settings.value, showZeroByte: value }
         await persist()
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
