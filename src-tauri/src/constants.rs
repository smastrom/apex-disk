//! App-level constants shared with the frontend. Keep in sync with src/lib/constants.ts.

/// App name shown in the menu bar and About dialog. Keep in sync with tauri.conf.json productName.
pub const APP_NAME: &str = "Mac User Lens";

/// Author line shown in the About dialog (macOS credits field).
pub const APP_CREDITS: &str = "Simone Mastromattei (smastrom)";

/// Author website or GitHub profile URL.
pub const AUTHOR_URL: &str = "https://github.com/smastrom";

/// URL opened when the user chooses "Read release notes" from the Help menu.
pub const RELEASE_NOTES_URL: &str = "https://github.com/smastrom/mac-user-lens/releases";

/// URL opened when the user chooses "Read license" from the Help menu.
pub const LICENSE_URL: &str = "https://github.com/smastrom/mac-user-lens/blob/main/LICENSE";

/// Menu item id for the release notes action. Handled in lib.rs setup.
pub const RELEASE_NOTES_MENU_ID: &str = "release-notes";

/// Menu item id for the license action. Handled in lib.rs setup.
pub const LICENSE_MENU_ID: &str = "license";

/// Menu item id for the "Check for updates" action. Emits an event to the frontend.
pub const CHECK_UPDATES_MENU_ID: &str = "check-for-updates";
