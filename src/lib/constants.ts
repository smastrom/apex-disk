import { version } from '../../package.json'

/** App official description for each language. */
export const APP_DESCRIPTIONS = {
   en: 'A macOS tool to easily identify and get rid of big, unused files and folders in seconds.',
   it: 'Uno strumento per macOS per individuare ed eliminare facilmente file e cartelle ingombranti e inutilizzati in pochi secondi.',
   es: 'Una herramienta para macOS para identificar y eliminar fácilmente archivos y carpetas grandes que no usas en segundos.',
   fr: 'Un outil macOS pour identifier et supprimer facilement les fichiers et dossiers volumineux et inutilisés en quelques secondes.',
   pt: 'Uma ferramenta para macOS para identificar e remover facilmente ficheiros e pastas grandes e não utilizados em segundos.',
   de: 'Ein macOS-Tool, um große, ungenutzte Dateien und Ordner in Sekundenschnelle zu finden und zu löschen.',
   ru: 'Инструмент для macOS, позволяющий за считанные секунды находить и удалять большие неиспользуемые файлы и папки.',
   zh: '一款 macOS 工具，可在几秒钟内轻松识别并清除占用空间大且未使用的文件和文件夹。',
   ja: '不要な大容量ファイルやフォルダーを数秒で特定し、簡単に削除できる macOS ツール。',
   ar: 'أداة لنظام macOS لتحديد وحذف الملفات والمجلدات الكبيرة وغير المستخدمة بسهولة في ثوانٍ معدودة.',
}

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

export const ROOT_THEME = 'apex-disk' as const
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

/** Ms to show countdown on the Move to Trash button before it becomes clickable. */
export const TRASH_COUNTDOWN_MS = 1000

/** Ms to wait after trash_paths completes before emitting complete and clearing spinner. */
export const TRASH_POST_TRASH_SLEEP_MS = 500

/** Donate / support link. Opens in default browser when user taps Donate. */
export const DONATE_URL = 'https://buymeacoffee.com/smastrom'

/**
 * App-level constants shared with Rust. Keep in sync with src-tauri/src/constants.rs.
 */
export const APP_NAME = 'ApexDisk' as const
export const APP_VERSION = version
export const APP_CREDITS = 'Simone Mastromattei (smastrom)' as const
export const RELEASE_YEAR = 2026 as const
export const RELEASE_NOTES_URL = 'https://github.com/smastrom/apex-disk/releases' as const
export const LICENSE_URL = 'https://github.com/smastrom/apex-disk/blob/main/LICENSE' as const

/** Frontend-only URLs. */
export const AUTHOR_URL = 'https://github.com/smastrom' as const
export const REPOSITORY_URL = 'https://github.com/smastrom/apex-disk' as const
export const LATEST_RELEASE_URL =
   'https://api.github.com/repos/smastrom/apex-disk/releases/latest' as const

/** File extension sets for icon categorization. */
export const DOC_EXTENSIONS = new Set([
   'pdf',
   'doc',
   'docx',
   'txt',
   'md',
   'rtf',
   'odt',
   'pages',
   'numbers',
   'key',
   'ppt',
   'pptx',
   'xls',
   'xlsx',
   'csv',
   'tex',
   'docm',
   'dotx',
   'dotm',
   'xlsm',
   'pptm',
   'ods',
   'odg',
   'odp',
   'odb',
   'odc',
   'odm',
   'odf',
])

export const AUDIO_EXTENSIONS = new Set([
   'mp3',
   'wav',
   'aac',
   'flac',
   'm4a',
   'ogg',
   'wma',
   'aiff',
   'aif',
   'ape',
   'alac',
])

export const VIDEO_EXTENSIONS = new Set([
   'mp4',
   'mov',
   'avi',
   'mkv',
   'webm',
   'wmv',
   'm4v',
   'flv',
   'mpg',
   'mpeg',
   '3gp',
])

export const ARCHIVE_EXTENSIONS = new Set([
   'zip',
   'tar',
   'gz',
   'rar',
   '7z',
   'dmg',
   'bz2',
   'xz',
   'z',
   'lz',
   'lzma',
   'tgz',
   'tbz',
   'txz',
])

export const IMAGE_EXTENSIONS = new Set([
   'jpg',
   'jpeg',
   'png',
   'gif',
   'webp',
   'svg',
   'bmp',
   'ico',
   'heic',
   'heif',
   'tiff',
   'tif',
])
