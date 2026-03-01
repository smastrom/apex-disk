import { locale } from '@tauri-apps/plugin-os'

import { APP_LANGUAGES, DEFAULT_LANGUAGE } from '@/lib/constants'

import type { Language } from '@/types/settings'

/**
 * Maps system locale (BCP-47) to app language. Falls back to 'en' if unsupported.
 * Uses Tauri's os plugin locale(); maps primary tag (e.g. "it-IT" → "it").
 */
export async function getSystemLanguage(): Promise<Language> {
   try {
      const systemLocale = await locale()
      if (!systemLocale) return DEFAULT_LANGUAGE
      const primary = systemLocale.split(/[-_]/)[0]?.toLowerCase()
      return APP_LANGUAGES.includes(primary as Language) ? (primary as Language) : DEFAULT_LANGUAGE
   } catch {
      return DEFAULT_LANGUAGE
   }
}
