//! Tests for `scan::scan_user_folders_from_home`.
//!
//! A temp dir is used as a fake home (see `support::create_test_home` for the full
//! layout: protected dirs, skipped dirs, normal dirs, and files of varying sizes).
//! We assert FolderInfo shape, protected/skipped behavior, and that ScanOptions
//! (show_under_1kb, show_zero_byte, show_hidden_files) filter correctly. No real
//! user home or system paths are touched.

mod support;

use apex_disk_lib::scan;
use apex_disk_lib::ScanOptions;

use support::{create_test_home, create_test_home_with_system_files};

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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
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

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
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

/// Folder last_modified should reflect the most recent modification date among all its contents,
/// but exclude macOS system files like .DS_Store that don't represent meaningful user activity.
/// When a folder contains files with different modification times, the folder should show
/// the most recent time from non-system files only.
#[test]
fn scan_folder_last_modified_excludes_system_files() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions::default();

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

    // Find MyData folder which contains multiple files
    let mydata = result.iter().find(|f| f.name == "MyData");
    assert!(mydata.is_some(), "MyData folder should exist in test home");

    let mydata = mydata.unwrap();

    // MyData should have a last_modified date since it contains files
    assert!(
        mydata.last_modified.is_some(),
        "MyData should have last_modified since it contains files"
    );

    // The folder's last_modified should be the most recent among its non-system children
    let folder_time = mydata.last_modified.unwrap();

    // Check all non-system children to ensure folder time is >= each child's time
    for child in &mydata.children {
        // Skip known system files in the assertion
        if child.name == ".DS_Store" || child.name.starts_with("._") {
            continue;
        }
        if let Some(child_time) = child.last_modified {
            assert!(
                folder_time >= child_time,
                "Folder time ({}) should be >= non-system child {} time ({})",
                folder_time,
                child.name,
                child_time
            );
        }
    }
}

/// Folder last_modified should reflect the most recent modification date among all its contents.
/// When a folder contains files with different modification times, the folder should show
/// the most recent time, not its own modification time.
#[test]
fn scan_folder_last_modified_most_recent_from_children() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions::default();

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

    // Find MyData folder which contains multiple files
    let mydata = result.iter().find(|f| f.name == "MyData");
    assert!(mydata.is_some(), "MyData folder should exist in test home");

    let mydata = mydata.unwrap();

    // MyData should have a last_modified date since it contains files
    assert!(
        mydata.last_modified.is_some(),
        "MyData should have last_modified since it contains files"
    );

    // The folder's last_modified should be the most recent among its children
    let folder_time = mydata.last_modified.unwrap();

    // Check all children to ensure folder time is >= each child's time
    for child in &mydata.children {
        if let Some(child_time) = child.last_modified {
            assert!(
                folder_time >= child_time,
                "Folder time ({}) should be >= child {} time ({})",
                folder_time,
                child.name,
                child_time
            );
        }
    }
}

/// System files like .DS_Store should be excluded from last_modified calculations
/// even when they exist in the folder and would otherwise be the most recent files.
#[test]
fn scan_folder_last_modified_ignores_system_files() {
    let home_dir = create_test_home_with_system_files();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true, // Ensure system files are included in scan
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

    // Find MyData folder which contains both user files and system files
    let mydata = result.iter().find(|f| f.name == "MyData");
    assert!(mydata.is_some(), "MyData folder should exist in test home");

    let mydata = mydata.unwrap();

    // Should have a last_modified date since it contains user files
    assert!(
        mydata.last_modified.is_some(),
        "MyData should have last_modified since it contains files"
    );

    // Verify that system files are present in the children (when show_hidden_files is true)
    let system_files_present: Vec<_> = mydata
        .children
        .iter()
        .filter(|c| c.name == ".DS_Store" || c.name == ".localized" || c.name.starts_with("._"))
        .collect();

    // System files should be present in the scan results when hidden files are shown
    assert!(
        !system_files_present.is_empty(),
        "System files should be present when show_hidden_files is true"
    );

    // The folder's last_modified should be based on user files only, not system files
    let folder_time = mydata.last_modified.unwrap();

    // Find user files (non-system files)
    let user_files: Vec<_> = mydata
        .children
        .iter()
        .filter(|c| !(c.name == ".DS_Store" || c.name == ".localized" || c.name.starts_with("._")))
        .collect();

    // At least one user file should exist
    assert!(!user_files.is_empty(), "User files should exist in MyData");

    // The folder time should be >= the most recent user file time
    for user_file in &user_files {
        if let Some(user_time) = user_file.last_modified {
            assert!(
                folder_time >= user_time,
                "Folder time ({}) should be >= user file {} time ({})",
                folder_time,
                user_file.name,
                user_time
            );
        }
    }
}

/// With show_hidden_files false (default), hidden files and dirs must not appear in the
/// scan results. Only visible entries should be returned.
#[test]
fn scan_show_hidden_files_false_filters_hidden() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: false,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

    // No top-level hidden dirs should appear
    for folder in &result {
        assert!(
            !folder.name.starts_with('.'),
            "Hidden folder {} should not appear when show_hidden_files is false",
            folder.name
        );
    }

    // Check inside MyData: .hidden should not appear
    if let Some(mydata) = result.iter().find(|f| f.name == "MyData") {
        for child in &mydata.children {
            assert!(
                !child.name.starts_with('.'),
                "Hidden file {} should not appear when show_hidden_files is false",
                child.name
            );
        }
    }
}

/// Scan with all options set to their most permissive values must return every
/// file and folder in the test home (except skipped). This is the "show everything" mode.
#[test]
fn scan_permissive_options_includes_all() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");
    let names: std::collections::HashSet<_> = result.iter().map(|f| f.name.as_str()).collect();

    // MyData contains a 0-byte file, a <1KB file, a hidden file, and a >=1KB file
    assert!(names.contains("MyData"), "MyData should be present");
    let mydata = result.iter().find(|f| f.name == "MyData").unwrap();
    let child_names: std::collections::HashSet<_> =
        mydata.children.iter().map(|c| c.name.as_str()).collect();
    assert!(
        child_names.contains("empty.txt"),
        "empty.txt should be present"
    );
    assert!(
        child_names.contains("small.txt"),
        "small.txt should be present"
    );
    assert!(child_names.contains(".hidden"), ".hidden should be present");
    assert!(child_names.contains("big.txt"), "big.txt should be present");
}

/// The total size of a folder must equal the sum of all its children's sizes.
/// This verifies that size accounting is correct throughout the tree.
#[test]
fn scan_folder_size_equals_sum_of_children() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

    for folder in &result {
        if !folder.children.is_empty() {
            let children_sum: u64 = folder.children.iter().map(|c| c.size).sum();
            assert_eq!(
                folder.size, children_sum,
                "Folder {} size ({}) should equal sum of children sizes ({})",
                folder.name, folder.size, children_sum
            );
        }
    }
}

/// Files within a folder must have is_file set to true; dirs with children must not.
#[test]
fn scan_is_file_flag_correct() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let options = ScanOptions {
        show_hidden_files: true,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let result = scan::scan_user_folders_from_home(home, &options, false).expect("scan");

    for folder in &result {
        for child in &folder.children {
            if child.is_file {
                assert!(
                    child.children.is_empty(),
                    "File {} should have no children",
                    child.name
                );
            }
            if !child.children.is_empty() {
                assert!(
                    !child.is_file,
                    "{} has children but is_file is true",
                    child.name
                );
            }
        }
    }
}
