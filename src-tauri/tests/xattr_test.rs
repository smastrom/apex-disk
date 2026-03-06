//! Tests for `xattr` module.
//!
//! Uses temp directories for deterministic behavior (no dependency on real system state).
//! Also tests against real container paths when available (best-effort).

use mac_disk_tree_lib::xattr;
use std::path::Path;

/// A freshly created temp directory must not have the container manager attribute.
#[test]
fn temp_dir_has_no_container_attribute() {
    let dir = tempfile::tempdir().expect("temp dir");
    assert!(
        !xattr::has_container_manager_attribute(dir.path()),
        "Temp dir should not have container manager attribute"
    );
}

/// A nonexistent path must return false (no panic, no crash).
#[test]
fn nonexistent_path_returns_false() {
    assert!(!xattr::has_container_manager_attribute(Path::new(
        "/nonexistent/path/that/does/not/exist"
    )));
}

/// A file (not a directory) must return false.
#[test]
fn regular_file_has_no_container_attribute() {
    let dir = tempfile::tempdir().expect("temp dir");
    let file_path = dir.path().join("test.txt");
    std::fs::write(&file_path, b"hello").expect("create file");
    assert!(!xattr::has_container_manager_attribute(&file_path));
}

/// Real container paths (best-effort): if ~/Library/Containers exists,
/// test that we can read the attribute without crashing.
#[test]
fn real_container_detection_no_panic() {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };

    let containers_path = home.join("Library/Containers");
    if !containers_path.exists() {
        return;
    }

    let entries = match std::fs::read_dir(&containers_path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.filter_map(|e| e.ok()).take(3) {
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            // Should not panic regardless of the result
            let _ = xattr::has_container_manager_attribute(&entry.path());
        }
    }
}

/// Desktop (a regular user folder) should not have container manager attributes.
#[test]
fn regular_folder_no_attribute() {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };

    let desktop = home.join("Desktop");
    if desktop.exists() {
        assert!(!xattr::has_container_manager_attribute(&desktop));
    }
}
