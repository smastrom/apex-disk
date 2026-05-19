//! Tests for `safe_folders`: `is_path_skipped` and `is_path_protected`.
//!
//! Uses a fake home path (`/Users/testuser`) and realistic path strings so behavior
//! matches production. Skipped paths never appear in scan results; protected paths
//! cannot be deleted.

use std::path::Path;

use apex_disk_lib::safe_folders;

const HOME: &str = "/Users/testuser";

fn home() -> &'static Path {
    Path::new(HOME)
}

/// Every entry in SKIPPED_RELATIVE_PATHS must return true when used as an exact path under home.
/// These dirs are excluded from the scan entirely.
#[test]
fn is_path_skipped_exact_match_all() {
    for skipped in safe_folders::SKIPPED_RELATIVE_PATHS {
        let path = Path::new(HOME).join(skipped);
        assert!(
            safe_folders::is_path_skipped(&path, home()),
            "SKIPPED entry must be skipped: {}",
            skipped
        );
    }
}

/// Paths that exactly match a skipped entry (e.g. `.ssh`, `.Trash`, `Library/Keychains`)
/// must return true. These dirs are excluded from the scan entirely.
#[test]
fn is_path_skipped_exact_match() {
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.ssh"), home()));
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.gnupg"), home()));
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.Trash"), home()));
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/Library/Keychains"), home()));
}

/// For every skipped entry, a path that is a descendant (e.g. `.ssh/id_rsa` under `.ssh`)
/// must return true so the whole subtree is excluded.
#[test]
fn is_path_skipped_descendant_all() {
    for skipped in safe_folders::SKIPPED_RELATIVE_PATHS {
        let child = format!("{}/child", skipped);
        let path = Path::new(HOME).join(&child);
        assert!(
            safe_folders::is_path_skipped(&path, home()),
            "Descendant of SKIPPED entry must be skipped: {}",
            skipped
        );
    }
}

/// Paths under a skipped dir (e.g. `.ssh/id_rsa`, `Library/Keychains/something`)
/// must return true so the whole subtree is excluded.
#[test]
fn is_path_skipped_descendant() {
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.ssh/id_rsa"), home()));
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.ssh/known_hosts"), home()));
    assert!(safe_folders::is_path_skipped(
        Path::new("/Users/testuser/Library/Keychains/something"),
        home()
    ));
}

/// Comparison is case-insensitive (macOS APFS). `.SSH` and `.SSH/id_rsa` must be
/// treated as skipped like `.ssh`.
#[test]
fn is_path_skipped_case_insensitive() {
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.SSH"), home()));
    assert!(safe_folders::is_path_skipped(Path::new("/Users/testuser/.SSH/id_rsa"), home()));
}

/// Paths not under home (e.g. `/etc/.ssh`) are not considered skipped; only
/// home-relative skipped entries matter.
#[test]
fn is_path_skipped_outside_home_returns_false() {
    assert!(!safe_folders::is_path_skipped(Path::new("/etc/.ssh"), home()));
    assert!(!safe_folders::is_path_skipped(Path::new("/tmp/.ssh"), home()));
}

/// Paths under home that are not in SKIPPED_RELATIVE_PATHS (e.g. Documents, MyFolder)
/// must return false so they appear in the scan.
#[test]
fn is_path_skipped_under_home_but_not_skipped() {
    assert!(!safe_folders::is_path_skipped(Path::new("/Users/testuser/Documents"), home()));
    assert!(!safe_folders::is_path_skipped(Path::new("/Users/testuser/MyFolder"), home()));
}

/// Every entry in PROTECTED_RELATIVE_PATHS must return true when used as an exact path under home.
/// These dirs cannot be deleted (folder itself protected; contents can be managed).
#[test]
fn is_path_protected_exact_match_all() {
    for protected in safe_folders::PROTECTED_RELATIVE_PATHS {
        let path = Path::new(HOME).join(protected);
        assert!(
            safe_folders::is_path_protected(&path, home()),
            "PROTECTED entry must be protected: {}",
            protected
        );
    }
}

/// Paths that exactly match a protected dir (Library, Documents, Desktop,
/// Library/Application Support) must return true so they cannot be deleted.
#[test]
fn is_path_protected_exact_match() {
    assert!(safe_folders::is_path_protected(Path::new("/Users/testuser/Library"), home()));
    assert!(safe_folders::is_path_protected(Path::new("/Users/testuser/Documents"), home()));
    assert!(safe_folders::is_path_protected(Path::new("/Users/testuser/Desktop"), home()));
    assert!(safe_folders::is_path_protected(
        Path::new("/Users/testuser/Library/Application Support"),
        home()
    ));
}

/// Protected check is case-insensitive; LIBRARY and documents match like Library/Documents.
#[test]
fn is_path_protected_case_insensitive() {
    assert!(safe_folders::is_path_protected(Path::new("/Users/testuser/LIBRARY"), home()));
    assert!(safe_folders::is_path_protected(Path::new("/Users/testuser/documents"), home()));
}

/// The home directory itself is protected (empty relative path is treated as protected).
#[test]
fn is_path_protected_home_itself() {
    assert!(safe_folders::is_path_protected(Path::new("/Users/testuser"), home()));
}

/// Paths not under home cannot be resolved to a relative path; they are treated as
/// protected so we never allow deleting e.g. /etc or /tmp/foo.
#[test]
fn is_path_protected_path_not_under_home_returns_true() {
    assert!(safe_folders::is_path_protected(Path::new("/etc"), home()));
    assert!(safe_folders::is_path_protected(Path::new("/tmp/foo"), home()));
}

/// Dirs under home that are not in PROTECTED_RELATIVE_PATHS (e.g. MyData, Projects)
/// must return false so the user can delete them.
#[test]
fn is_path_protected_non_protected_sibling() {
    assert!(!safe_folders::is_path_protected(Path::new("/Users/testuser/MyData"), home()));
    assert!(!safe_folders::is_path_protected(Path::new("/Users/testuser/Projects"), home()));
}

/// Only exact relative path matches are protected. A descendant like
/// `Library/Application Support/SomeApp` is not the protected folder itself,
/// so it must return false (contents can be deleted, not the protected folder).
#[test]
fn is_path_protected_descendant_not_exact_match() {
    assert!(!safe_folders::is_path_protected(
        Path::new("/Users/testuser/Library/Application Support/SomeApp"),
        home()
    ));
}
