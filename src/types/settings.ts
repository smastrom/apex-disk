/** Supported app languages. */
export type Language = 'en' | 'it'

/** Supported theme color presets. Single source of truth for theme IDs. */
export const THEME_COLORS = ['mac-user-lens', 'ayu'] as const

export type ThemeColor = (typeof THEME_COLORS)[number]

/** Theme that uses :root palette (no data-theme attribute). */
export const ROOT_THEME: ThemeColor = 'mac-user-lens'

/** App settings persisted to disk. */
export interface AppSettings {
   language: Language
   themeColor: ThemeColor
   showHiddenFiles: boolean
   showZeroByteFiles: boolean
   showZeroByteFolders: boolean
   enableAnimations: boolean
}

export const DEFAULT_SETTINGS: AppSettings = {
   language: 'en',
   themeColor: 'mac-user-lens',
   showHiddenFiles: false,
   showZeroByteFiles: false,
   showZeroByteFolders: false,
   enableAnimations: true,
}
