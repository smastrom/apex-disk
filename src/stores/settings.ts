import { load } from '@tauri-apps/plugin-store'
import { invoke } from '@tauri-apps/api/core'
import { inject, ref, type Ref } from 'vue'

import { getSystemLanguage } from '@/lib/settings'

import { DEFAULT_SETTINGS, THEME_COLORS } from '@/lib/constants'

import type { AppSettings, ThemeColor, Language } from '@/types/settings'

/** Normalizes stored theme value: valid theme, legacy (oceanic/catppuccin) → mac-disk-tree, or default. */
function normalizeThemeColor(value: unknown): ThemeColor {
   if (typeof value === 'string' && (THEME_COLORS as readonly string[]).includes(value)) {
      return value as ThemeColor
   }

   return DEFAULT_SETTINGS.themeColor
}

/** Legacy keys that may exist in stored settings; merged into showZeroByte when migrating. */
interface LegacyAppSettings extends Partial<AppSettings> {
   showZeroByteFiles?: boolean
   showZeroByteFolders?: boolean
   /** @deprecated use permanentlyDelete (inverted) */
   moveToTrash?: boolean
}

/** Builds AppSettings from stored raw, migrating legacy showZeroByteFiles/showZeroByteFolders into showZeroByte. */
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

/** Injection key for the settings store. Provided value is Ref<SettingsStore | null>. */
export const SETTINGS_KEY = Symbol('settings')

export interface SettingsStore {
   settings: Ref<AppSettings>
   getThemeColor: () => string
   setLanguage: (lang: Language) => Promise<void>
   setThemeColor: (theme: ThemeColor) => Promise<void>
   setPermanentlyDelete: (value: boolean) => Promise<void>
   setShowHiddenFiles: (value: boolean) => Promise<void>
   setShowUnder1Kb: (value: boolean) => Promise<void>
   setShowZeroByte: (value: boolean) => Promise<void>
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
         invoke('set_menu_language', { lang }).catch(() => {})
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

      const raw = (await store.get('app')) as Partial<LegacyAppSettings> | null
      const systemLanguage = await getSystemLanguage()
      const settings = ref<AppSettings>(
         normalizeStoredSettings({
            ...(raw && typeof raw === 'object' ? raw : {}),
            language: raw?.language ?? systemLanguage,
         })
      )

      async function persist() {
         await store.set('app', settings.value)
         await store.save()
      }

      const result = createStoreFromSettings(settings, persist)

      invoke('set_menu_language', { lang: settings.value.language }).catch(() => {})
      result.load = async () => {
         await store.reload()

         const raw = (await store.get('app')) as Partial<LegacyAppSettings> | null

         if (raw && typeof raw === 'object') {
            const systemLanguage = await getSystemLanguage()

            settings.value = normalizeStoredSettings({
               ...raw,
               language: raw.language ?? systemLanguage,
            })
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

/** Injects the settings store provided by main.ts. Must be called inside setup(). */
export function useSettingsStore(): SettingsStore {
   const store = inject<SettingsStore>(SETTINGS_KEY)
   if (!store) {
      throw new Error(
         'useSettingsStore() called without a provider. Did you forget app.provide() in main.ts?'
      )
   }

   return store
}
