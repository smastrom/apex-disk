// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import type { Language } from '@/types/settings'

import AppFooter from './AppFooter.yaml'
import InformationFooter from './InformationFooter.yaml'
import InformationView from './InformationView.yaml'
import ScanLaunch from './ScanLaunch.yaml'
import ScanProgress from './ScanProgress.yaml'
import ScanResultsList from './ScanResultsList.yaml'
import ScanResultsListItem from './ScanResultsListItem.yaml'
import ScanTrashConfirmation from './ScanTrashConfirmation.yaml'
import ScanTrashList from './ScanTrashList.yaml'
import ScanViewHeader from './ScanViewHeader.yaml'
import SettingsView from './SettingsView.yaml'

export const translations = {
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
      const mod = translations[module] as Record<string, Record<string, string>>
      const entry = mod[key]
      const str = entry?.[lang] ?? entry?.en ?? key
      return vars ? interpolate(str, vars) : str
   }
}
