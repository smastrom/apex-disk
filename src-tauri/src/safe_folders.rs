//! Protected system folders that cannot be selected or deleted.
//!
//! These are the standard macOS home directory folders managed by the OS.
//! Deleting them would break Finder, iCloud, system services, or app configs.
//! Their contents remain selectable — only the exact paths listed are protected.
//! To add/remove: edit PROTECTED_RELATIVE_PATHS.

use std::path::Path;

/// Paths relative to home that are protected. Only these exact paths are protected —
/// not their descendants. Both top-level names ("Library") and nested paths
/// ("Library/Application Support") are supported. To add/remove: edit this list.
///
/// - Desktop, Documents, Downloads: Finder sidebar + iCloud sync
/// - Library: app preferences, caches, keychains, LaunchAgents
/// - Library/Application Support: shared app data managed by macOS
/// - Movies, Music, Pictures: media apps (TV, Music, Photos) store data here
/// - Public: macOS file-sharing drop box
/// - Applications: user-installed apps
pub const PROTECTED_RELATIVE_PATHS: &[&str] = &[
    "Applications",
    "Desktop",
    "Documents",
    "Downloads",
    "Library",
    "Library/Application Support",
    "Movies",
    "Music",
    "Pictures",
    "Public",
];

/// Paths relative to home that are completely excluded from scan results.
/// These contain irreplaceable credentials or security keys that should
/// never appear in a disk cleanup tool — not even as browsable entries.
/// ".Trash" is excluded because "delete" on items already in Trash means
/// permanent deletion.
pub const SKIPPED_RELATIVE_PATHS: &[&str] = &[".ssh", ".gnupg", ".aws", ".kube", ".Trash"];

/// Returns true if the given path matches a skipped directory or is a descendant of one.
/// Unlike protected paths, skipped paths are never scanned or shown to the user.
pub fn is_path_skipped(path: &Path, home: &Path) -> bool {
    let home_str = home.to_string_lossy();
    let path_str = path.to_string_lossy();
    let rel = match path_str.strip_prefix(home_str.as_ref()) {
        Some(r) => r.trim_start_matches('/'),
        None => return false,
    };
    if rel.is_empty() {
        return false;
    }
    for skipped in SKIPPED_RELATIVE_PATHS {
        if rel == *skipped || rel.starts_with(&format!("{}/", skipped)) {
            return true;
        }
    }
    false
}

/// Returns true if the given path exactly matches one of the protected paths.
/// Descendants of protected paths are NOT protected.
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
