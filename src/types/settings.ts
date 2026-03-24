import { THEME_COLORS, APP_LANGUAGES } from '@/lib/constants'

export type Language = (typeof APP_LANGUAGES)[number]

export type ThemeColor = (typeof THEME_COLORS)[number]

/** App settings persisted to disk. */
export interface AppSettings {
   language: Language
   themeColor: ThemeColor
   showHiddenFiles: boolean
   showUnder1Kb: boolean
   showZeroByte: boolean
   autoUpdates: boolean
}
