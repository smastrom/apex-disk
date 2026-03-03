import type { AppSettings } from '@/types/settings'

/** English, then European (it, es, fr, pt, de), then Russian, then Asiatic (zh, ja, ar). */
export const APP_LANGUAGES = ['en', 'it', 'es', 'fr', 'pt', 'de', 'ru', 'zh', 'ja', 'ar'] as const

export const DEFAULT_LANGUAGE = 'en' as const

/** Languages that use right-to-left script direction. */
export const RTL_LANGUAGES: ReadonlySet<string> = new Set(['ar'])

export const ROOT_THEME = 'mac-disk-tree' as const
/** Supported theme color presets. Single source of truth for theme IDs. */
export const THEME_COLORS = [
   ROOT_THEME,
   'macos-dark',
   'macos-light',
   'macos-graphite',
   'coral-orange',
   'ayu',
   'smastrom',
] as const

/** Theme that uses :root palette (no data-theme attribute). */
export const DEFAULT_SETTINGS: AppSettings = {
   language: DEFAULT_LANGUAGE,
   themeColor: ROOT_THEME,
   permanentlyDelete: false,
   showHiddenFiles: false,
   showUnder1Kb: false,
   showZeroByte: false,
}

/** Seconds to show countdown on the Delete button before it becomes clickable. */
export const DELETE_COUNTDOWN_SECONDS = 1

/** Ms to wait after delete_paths completes before emitting complete and clearing spinner. */
export const DELETE_POST_DELETE_SLEEP_MS = 2000

/** Donate / support link. Opens in default browser when user taps Donate. */
export const DONATE_URL = 'https://buymeacoffee.com/smastrom'

/**
 * App-level constants shared with Rust. Keep in sync with src-tauri/src/constants.rs.
 */
export const APP_NAME = 'MacDiskTree' as const
export const APP_CREDITS = 'Simone Mastromattei (smastrom)' as const
/** Author website or GitHub profile. Used in Settings app info. */
export const AUTHOR_URL = 'https://github.com/smastrom' as const
export const RELEASE_NOTES_URL = 'https://github.com/smastrom/mac-disk-tree/releases' as const
export const LICENSE_URL = 'https://github.com/smastrom/mac-disk-tree/blob/main/LICENSE' as const
