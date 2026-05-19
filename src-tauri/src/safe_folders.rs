// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Protected system folders and sensitive credential paths.
//!
//! ApexDisk uses these lists to ensure the user doesn't break their macOS
//! installation or accidentally delete irreplaceable security credentials.

use std::{collections::HashSet, path::Path, sync::LazyLock};

/// Paths relative to home that are protected from deletion.
///
/// These are structural folders. Their CONTENTS can be deleted/managed,
/// but the folder itself must remain to prevent breaking system hooks,
/// iCloud syncing, or app registration.
pub const PROTECTED_RELATIVE_PATHS: &[&str] = &[
    "Applications",
    "Desktop",
    "Documents",
    "Downloads",
    "Library",
    "Library/Accounts",
    "Library/Application Support",
    "Library/Containers",
    "Library/Group Containers",
    "Library/Messages",
    "Library/MobileSync",
    "Library/Preferences",
    "Library/Safari",
    "Movies",
    "Music",
    "Pictures",
    "Public",
    // Cloud Sync Roots - Deleting these can trigger mass-deletion on remote servers.
    "Dropbox",
    "OneDrive",
    "Google Drive",
    "Creative Cloud Files",
];

/// Paths relative to home that are completely excluded from the scan.
///
/// These contain sensitive data (SSH keys, Cloud tokens, Password vaults).
/// They are ignored entirely to ensure privacy and prevent accidental exposure.
pub const SKIPPED_RELATIVE_PATHS: &[&str] = &[
    ".ssh",
    ".gnupg",
    ".aws",
    ".kube",
    ".azure",
    ".heroku",
    ".docker",
    ".config/gh",
    ".password-store",
    ".pki",
    "Library/Keychains",
    "Library/IdentityServices",
    ".Trash",
];

/// Pre-computed lowercased protected paths for O(1) lookup during scans.
static PROTECTED_SET: LazyLock<HashSet<String>> =
    LazyLock::new(|| PROTECTED_RELATIVE_PATHS.iter().map(|p| p.to_lowercase()).collect());

/// Pre-computed lowercased skipped paths for fast lookup during scans.
/// Stored as a Vec (not HashSet) because we need prefix matching for descendants.
static SKIPPED_LOWERED: LazyLock<Vec<String>> =
    LazyLock::new(|| SKIPPED_RELATIVE_PATHS.iter().map(|p| p.to_lowercase()).collect());

/// Returns true if the path is a descendant of (or is) a skipped directory.
/// Comparison is case-insensitive to match macOS APFS behavior.
pub fn is_path_skipped(path: &Path, home: &Path) -> bool {
    let rel = match get_relative_to_home(path, home) {
        Some(r) => r.to_lowercase(),
        None => return false,
    };

    if rel.is_empty() {
        return false;
    }

    for skipped_low in SKIPPED_LOWERED.iter() {
        // Check if it's the folder itself OR a child (e.g., .ssh/id_rsa)
        if rel == *skipped_low
            || (rel.len() > skipped_low.len()
                && rel.starts_with(skipped_low.as_str())
                && rel.as_bytes()[skipped_low.len()] == b'/')
        {
            return true;
        }
    }
    false
}

/// Returns true if the path exactly matches a protected directory.
/// Comparison is case-insensitive.
pub fn is_path_protected(path: &Path, home: &Path) -> bool {
    let rel = match get_relative_to_home(path, home) {
        Some(r) => r.to_lowercase(),
        None => return true, // Protecting the home directory itself
    };

    if rel.is_empty() {
        return true;
    }

    PROTECTED_SET.contains(&rel)
}

/// Helper to extract the relative path from the home directory.
fn get_relative_to_home<'a>(path: &'a Path, home: &Path) -> Option<&'a str> {
    let home_str = home.to_str()?;
    let path_str = path.to_str()?;

    path_str.strip_prefix(home_str).map(|r| r.trim_start_matches('/'))
}
