/** Donate / support link. Opens in default browser when user taps Donate. */
export const DONATE_URL = 'https://buymeacoffee.com/smastrom'

/**
 * Protected system folder names (relative to home). Must match Rust PROTECTED_RELATIVE_PATHS.
 * These cannot be selected or deleted. Add/remove here and in src-tauri/src/safe_folders.rs.
 */
export const PROTECTED_FOLDER_NAMES = [
   'Applications',
   'Desktop',
   'Documents',
   'Downloads',
   'Library',
   'Movies',
   'Music',
   'Pictures',
   'Public',
   'Application Support',
] as const
