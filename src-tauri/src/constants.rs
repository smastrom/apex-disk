//! App-level constants shared with the frontend. Keep in sync with src/lib/constants.ts.

/// App name shown in the menu bar and About dialog. Keep in sync with tauri.conf.json productName.
pub const APP_NAME: &str = "ApexDisk";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_CREDITS: &str = "Simone Mastromattei (smastrom)";
pub const RELEASE_YEAR: &str = "2026";

/// URL opened when the user chooses "Read release notes" from the Help menu.
pub const RELEASE_NOTES_URL: &str = "https://github.com/smastrom/apex-disk/releases";

/// URL opened when the user chooses "Read license" from the Help menu.
pub const LICENSE_URL: &str = "https://github.com/smastrom/apex-disk/blob/main/LICENSE.md";

/// Menu item id for the release notes action. Handled in lib.rs setup.
pub const RELEASE_NOTES_MENU_ID: &str = "release-notes";

/// Menu item id for the license action. Handled in lib.rs setup.
pub const LICENSE_MENU_ID: &str = "license";

/// Default language for the application.
pub const DEFAULT_LANGUAGE: &str = "en";

/// Default theme for the application. Must match ROOT_THEME in src/lib/constants.ts.
pub const DEFAULT_THEME: &str = "apex";
