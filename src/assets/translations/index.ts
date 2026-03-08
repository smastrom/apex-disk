import type { Language } from '@/types/settings'

import { App } from './App'
import { AppFooter } from './AppFooter'
import { AppHeader } from './AppHeader'
import { global } from './global'
import { InformationFooter } from './InformationFooter'
import { InformationView } from './InformationView'
import { ScanLaunch } from './ScanLaunch'
import { ScanProgress } from './ScanProgress'
import { ScanResultsList } from './ScanResultsList'
import { ScanResultsListItem } from './ScanResultsListItem'
import { ScanTrashConfirmation } from './ScanTrashConfirmation'
import { ScanTrashList } from './ScanTrashList'
import { ScanViewHeader } from './ScanViewHeader'
import { SettingsView } from './SettingsView'

export const translations = {
   global,
   App,
   AppHeader,
   AppFooter,
   SettingsView,
   InformationFooter,
   InformationView,
   ScanLaunch,
   ScanProgress,
   ScanResultsList,
   ScanResultsListItem,
   ScanTrashConfirmation,
   ScanTrashList,
   ScanViewHeader,
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
