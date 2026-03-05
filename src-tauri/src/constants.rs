//! App-level constants shared with the frontend. Keep in sync with src/lib/constants.ts.

/// App name shown in the menu bar and About dialog. Keep in sync with tauri.conf.json productName.
pub const APP_NAME: &str = "MacDiskTree";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_CREDITS: &str = "Simone Mastromattei (smastrom)";
pub const RELEASE_YEAR: &str = "2026";

/// Author website or GitHub profile URL.
#[allow(dead_code)]
pub const AUTHOR_URL: &str = "https://github.com/smastrom";

/// URL opened when the user chooses "Read release notes" from the Help menu.
pub const RELEASE_NOTES_URL: &str = "https://github.com/smastrom/mac-disk-tree/releases";

/// URL opened when the user chooses "Read license" from the Help menu.
#[allow(dead_code)]
pub const LICENSE_URL: &str = "https://github.com/smastrom/mac-disk-tree/blob/main/LICENSE";

/// Safe fallback date when release date fetching fails.
#[allow(dead_code)]
pub const FALLBACK_RELEASE_DATE: &str = "2026";

/// Menu item id for the release notes action. Handled in lib.rs setup.
pub const RELEASE_NOTES_MENU_ID: &str = "release-notes";

/// Menu item id for the license action. Handled in lib.rs setup.
pub const LICENSE_MENU_ID: &str = "license";

/// Default language for the application.
pub const DEFAULT_LANGUAGE: &str = "en";

/// Default theme for the application.
pub const DEFAULT_THEME: &str = "mac-disk-tree";
