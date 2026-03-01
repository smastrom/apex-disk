import type { AppSettings } from '@/types/settings'

export const APP_LANGUAGES = ['en', 'it', 'es', 'zh', 'ja'] as const

export const DEFAULT_LANGUAGE = 'en' as const

export const ROOT_THEME = 'mac-user-lens' as const
/** Supported theme color presets. Single source of truth for theme IDs. */
export const THEME_COLORS = [ROOT_THEME, 'ayu', 'smastrom'] as const

/** Theme that uses :root palette (no data-theme attribute). */
export const DEFAULT_SETTINGS: AppSettings = {
   language: DEFAULT_LANGUAGE,
   themeColor: ROOT_THEME,
   showHiddenFiles: false,
   showUnder1Kb: false,
   showZeroByte: false,
}

/**
 * Protected paths relative to home. Must match Rust PROTECTED_RELATIVE_PATHS.
 * Both top-level names ("Library") and nested paths ("Library/Application Support") are supported.
 * These cannot be selected or deleted. Add/remove here and in src-tauri/src/safe_folders.rs.
 */
export const PROTECTED_FOLDER_NAMES = [
   'Applications',
   'Desktop',
   'Documents',
   'Downloads',
   'Library',
   'Library/Application Support',
   'Movies',
   'Music',
   'Pictures',
   'Public',
] as const

/** Seconds to show countdown on the Delete button before it becomes clickable. */
export const DELETE_COUNTDOWN_SECONDS = 1

/** Ms to wait after delete_paths completes before emitting complete and clearing spinner. */
export const DELETE_POST_DELETE_SLEEP_MS = 2000

/** Donate / support link. Opens in default browser when user taps Donate. */
export const DONATE_URL = 'https://buymeacoffee.com/smastrom'
