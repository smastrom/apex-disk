import { version } from '../../package.json'

/** English, then European (it, es, fr, pt, de), then Russian, then Asiatic (zh, ja, ar). */
export const APP_LANGUAGES = ['en', 'it', 'es', 'fr', 'pt', 'de', 'ru', 'zh', 'ja', 'ar'] as const

export const APP_LANGUAGES_TO_LOCALE_MAP: Record<(typeof APP_LANGUAGES)[number], string> = {
   en: 'en-US',
   it: 'it-IT',
   es: 'es-ES',
   fr: 'fr-FR',
   pt: 'pt-PT',
   de: 'de-DE',
   ru: 'ru-RU',
   zh: 'zh-CN',
   ja: 'ja-JP',
   ar: 'ar-SA',
} as const

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

/** Ms to show countdown on the Delete button before it becomes clickable. */
export const DELETE_COUNTDOWN_MS = 1000

/** Ms to wait after delete_paths completes before emitting complete and clearing spinner. */
export const DELETE_POST_DELETE_SLEEP_MS = 500

/** Donate / support link. Opens in default browser when user taps Donate. */
export const DONATE_URL = 'https://buymeacoffee.com/smastrom'

/**
 * App-level constants shared with Rust. Keep in sync with src-tauri/src/constants.rs.
 */
export const APP_NAME = 'MacDiskTree' as const
export const APP_VERSION = version
export const APP_CREDITS = 'Simone Mastromattei (smastrom)' as const
export const RELEASE_YEAR = 2026 as const
/** Author website or GitHub profile. Used in Settings app info. */
export const AUTHOR_URL = 'https://github.com/smastrom' as const
export const REPOSITORY_URL = 'https://github.com/smastrom/mac-disk-tree' as const
export const RELEASE_NOTES_URL = 'https://github.com/smastrom/mac-disk-tree/releases' as const
export const LICENSE_URL = 'https://github.com/smastrom/mac-disk-tree/blob/main/LICENSE' as const

/** Safe fallback date when release date fetching fails */
export const FALLBACK_RELEASE_DATE = '2026' as const
