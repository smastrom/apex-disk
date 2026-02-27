//! Protected system folders that cannot be selected or deleted.
//!
//! These are the standard macOS home directory folders managed by the OS.
//! Deleting them would break Finder, iCloud, system services, or app configs.
//! Their contents remain selectable — only the parent folders are protected.
//! To add/remove: edit PROTECTED_RELATIVE_PATHS.

use std::path::Path;

/// Paths relative to home that are protected. Only these exact folders are protected,
/// not their descendants. To add/remove: edit this list.
///
/// - Desktop, Documents, Downloads: Finder sidebar + iCloud sync
/// - Library: app preferences, caches, keychains, LaunchAgents
/// - Movies, Music, Pictures: media apps (TV, Music, Photos) store data here
/// - Public: macOS file-sharing drop box
/// - Applications: user-installed apps
pub const PROTECTED_RELATIVE_PATHS: &[&str] = &[
    "Applications",
    "Desktop",
    "Documents",
    "Downloads",
    "Library",
    "Movies",
    "Music",
    "Pictures",
    "Public",
    "Application Support",
]; // TODO: Add support for e.g. Library/Application Support and other nested protected folders

/// Returns true if the given path is exactly one of the protected parent folders.
/// Descendants (e.g. Library/Application Support) are NOT protected.
pub fn is_path_protected(path: &Path, home: &Path) -> bool {
    let home_str = home.to_string_lossy();
    let path_str = path.to_string_lossy();
    let rel = match path_str.strip_prefix(home_str.as_ref()) {
        Some(r) => r.trim_start_matches('/'),
        None => return false,
    };
    if rel.is_empty() {
        return true; // The home directory itself is protected.
    }
    for protected in PROTECTED_RELATIVE_PATHS {
        if rel == *protected {
            return true;
        }
    }
    false
}
