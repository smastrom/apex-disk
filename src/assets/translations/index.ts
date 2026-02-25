import { global } from './global'
import { Header } from './Header'
import { Layout } from './Layout'
import { ListItem } from './ListItem'
import { DiskUsageProgress } from './DiskUsageProgress'
import { ScanSplashScreen } from './ScanSplashScreen'
import { ScanLoadingView } from './ScanLoadingView'
import { ScanResults } from './ScanResults'
import { SettingsView } from './SettingsView'
import { FooterMenu } from './FooterMenu'

import type { Language } from '@/types/settings'

export const translations = {
   global,
   DiskUsageProgress,
   Header,
   Layout,
   ListItem,
   ScanSplashScreen,
   ScanLoadingView,
   ScanResults,
   SettingsView,
   FooterMenu,
} as const

export type TranslationModule = keyof typeof translations

function interpolate(str: string, vars: Record<string, string | number>): string {
   return str.replace(/\{\{(\w+)\}\}/g, (_, key) => String(vars[key] ?? ''))
}

/** Returns a translation function bound to the given language. */
export function createT(lang: Language) {
   return function t(module: TranslationModule, key: string, vars?: Record<string, string | number>): string {
      const mod = translations[module] as Record<Language, Record<string, string>>
      const str = mod[lang]?.[key] ?? mod.en?.[key] ?? key
      return vars ? interpolate(str, vars) : str
   }
}
