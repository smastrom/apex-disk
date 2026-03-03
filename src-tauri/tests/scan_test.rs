//! Tests for `scan::scan_user_folders_from_home`.
//!
//! A temp dir is used as a fake home (see `support::create_test_home` for the full
//! layout: protected dirs, skipped dirs, normal dirs, and files of varying sizes).
//! We assert FolderInfo shape, protected/skipped behavior, and that ScanOptions
//! (show_under_1kb, show_zero_byte, show_hidden_files) filter correctly. No real
//! user home or system paths are touched.

mod support;

use mac_disk_tree_lib::scan;
use mac_disk_tree_lib::ScanOptions;

use support::create_test_home;

/// Scan succeeds and every top-level folder has the shape the frontend expects:
/// non-empty name/path, path under home, and children (if any) with names. Roots are dirs (is_file false).
#[test]
fn scan_returns_folder_info_shape() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");
    assert!(!result.is_empty());

    for folder in &result {
        assert!(!folder.name.is_empty());
        assert!(!folder.path.is_empty());
        assert!(folder.path.starts_with(home.to_str().unwrap()));
        assert!(folder.children.is_empty() || folder.children.iter().all(|c| !c.name.is_empty()));
        assert!(!folder.is_file);
    }
}

/// Folders that are in PROTECTED_RELATIVE_PATHS must have is_protected true so the UI
/// can mark them non-deletable. Test home includes Applications, Desktop, Documents,
/// Downloads, Library, Movies, Music, Pictures, Public.
#[test]
fn scan_protected_roots_have_is_protected_true() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");

    let names: std::collections::HashSet<_> = result.iter().map(|f| f.name.as_str()).collect();
    let protected_roots = [
        "Applications",
        "Desktop",
        "Documents",
        "Downloads",
        "Library",
        "Movies",
        "Music",
        "Pictures",
        "Public",
    ];
    for protected in protected_roots {
        if names.contains(protected) {
            let folder = result.iter().find(|f| f.name == protected).unwrap();
            assert!(folder.is_protected, "{} should be protected", protected);
        }
    }
}

/// Skipped dirs (.ssh, .Trash) must not appear in the result at all; they are excluded
/// during enumeration so their size is never counted.
#[test]
fn scan_skipped_dir_not_present() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");
    let names: std::collections::HashSet<_> = result.iter().map(|f| f.name.as_str()).collect();
    assert!(
        !names.contains(".ssh"),
        ".ssh should be skipped and not appear"
    );
    assert!(
        !names.contains(".Trash"),
        ".Trash should be skipped and not appear"
    );
}

/// With the full test home, Library has subdirs including Application Support and Preferences
/// (protected) and Keychains (skipped). Keychains must not appear as a child of Library;
/// only non-skipped children are included.
#[test]
fn scan_library_children_exclude_skipped_keychains() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");
    let library = result
        .iter()
        .find(|f| f.name == "Library")
        .expect("Library exists in test home");
    let child_names: Vec<_> = library.children.iter().map(|c| c.name.as_str()).collect();
    assert!(
        !child_names.iter().any(|n| *n == "Keychains"),
        "Library/Keychains (skipped) must not appear under Library"
    );
}

/// With show_under_1kb false, files (and dirs) under 1024 bytes must not appear.
/// small.txt (100 bytes) must be excluded from MyData’s children.
#[test]
fn scan_show_under_1kb_false_filters_small() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: false,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");
    let mydata = result.iter().find(|f| f.name == "MyData");
    if let Some(m) = mydata {
        let child_names: Vec<_> = m.children.iter().map(|c| c.name.as_str()).collect();
        assert!(
            !child_names.iter().any(|n| *n == "small.txt"),
            "small.txt (< 1KB) should be excluded when show_under_1kb is false"
        );
    }
}

/// With show_zero_byte false, 0-byte files (and empty dirs) must not appear.
/// empty.txt must be excluded from MyData’s children.
#[test]
fn scan_show_zero_byte_false_filters_zero() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: false,
    };

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");
    let mydata = result.iter().find(|f| f.name == "MyData");
    if let Some(m) = mydata {
        let child_names: Vec<_> = m.children.iter().map(|c| c.name.as_str()).collect();
        assert!(
            !child_names.iter().any(|n| *n == "empty.txt"),
            "empty.txt should be excluded when show_zero_byte is false"
        );
    }
}

/// Children in each folder must be sorted by size descending, then by name ascending
/// for ties, matching the UI’s size-ordered tree.
#[test]
fn scan_children_sorted_by_size_desc_then_name() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions::default();

    let result = scan::scan_user_folders_from_home(home, &options).expect("scan");
    for folder in &result {
        let children = &folder.children;
        for w in children.windows(2) {
            let (a, b) = (&w[0], &w[1]);
            assert!(
                a.size >= b.size,
                "children should be sorted by size desc: {} ({}) vs {} ({})",
                a.name,
                a.size,
                b.name,
                b.size
            );
            if a.size == b.size {
                assert!(
                    a.name <= b.name,
                    "same size: name should be asc: {} vs {}",
                    a.name,
                    b.name
                );
            }
        }
    }
}
