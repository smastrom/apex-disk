import { global } from './global'
import { AppHeader } from './AppHeader'
import { App } from './App'
import { ScanViewDiskUsage } from './ScanViewDiskUsage'
import { ScanLaunch } from './ScanLaunch'
import { ScanScanningResults } from './ScanScanningResults'
import { ScanResultsList } from './ScanResultsList'
import { ScanResultsListItem } from './ScanResultsListItem'
import { ScanResultsDeleteList } from './ScanResultsDeleteList'
import { ScanResultsDeleteConfirmation } from './ScanResultsDeleteConfirmation'
import { SettingsView } from './SettingsView'
import { AppFooter } from './AppFooter'

import type { Language } from '@/types/settings'

export const translations = {
   global,
   AppHeader,
   App,
   ScanViewDiskUsage,
   ScanLaunch,
   ScanScanningResults,
   ScanResultsList,
   ScanResultsListItem,
   ScanResultsDeleteList,
   ScanResultsDeleteConfirmation,
   SettingsView,
   AppFooter,
} as const

export type TranslationModule = keyof typeof translations

function interpolate(str: string, vars: Record<string, string | number>): string {
   return str.replace(/\{\{(\w+)\}\}/g, (_, key) => String(vars[key] ?? ''))
}

/** Returns a translation function bound to the given language. */
export function createT(lang: Language) {
   return function t(
      module: TranslationModule,
      key: string,
      vars?: Record<string, string | number>
   ): string {
      const mod = translations[module] as Record<Language, Record<string, string>>
      const str = mod[lang]?.[key] ?? mod.en?.[key] ?? key
      return vars ? interpolate(str, vars) : str
   }
}
