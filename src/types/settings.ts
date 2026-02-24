/** Supported app languages. */
export type Language = 'en' | 'it'

/** Supported theme color presets. */
export type ThemeColor =
   | 'oceanic'
   | 'light'
   | 'dark'
   | 'tokyo-night'
   | 'ayu-dark'
   | 'ayu-mirage'
   | 'dracula'
   | 'catppuccin'
   | 'gruvbox'
   | 'nord'
   | 'solarized'
   | 'one-dark'
   | 'deep-purple'
   | 'kanagawa'

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
   themeColor: 'dark',
   showHiddenFiles: false,
   showZeroByteFiles: false,
   showZeroByteFolders: false,
   enableAnimations: true,
}
