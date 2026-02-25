import { locale } from '@tauri-apps/plugin-os'

import type { Language } from '@/types/settings'

const SUPPORTED_LANGUAGES: Language[] = ['en', 'it']

/**
 * Maps system locale (BCP-47) to app language. Falls back to 'en' if unsupported.
 * Uses Tauri's os plugin locale(); maps primary tag (e.g. "it-IT" → "it").
 */
export async function getSystemLanguage(): Promise<Language> {
   try {
      const systemLocale = await locale()
      if (!systemLocale) return 'en'
      const primary = systemLocale.split(/[-_]/)[0]?.toLowerCase()
      return SUPPORTED_LANGUAGES.includes(primary as Language) ? (primary as Language) : 'en'
   } catch {
      return 'en'
   }
}
