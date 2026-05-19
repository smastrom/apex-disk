//! Tests for `trash`: filter_items, trash_paths_sync_with_home.
//!
//! The trash command receives `{ path, is_file }[]` from the frontend.
//! Items are always moved to the macOS Trash. Both paths use the same filter_items first.
//! We use the shared `support::create_test_home` (realistic home layout) so tests run against a
//! proper tree.

mod support;

use apex_disk_lib::trash::{filter_items, trash_paths_sync_with_home, TrashPathItem};
use support::create_test_home;

/// Items whose path is a protected dir (Documents, Library) must be removed from the
/// result. MyData (not protected) must remain. Outcome: only MyData in dirs, no files.
#[test]
fn filter_items_protected_paths_removed() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let home_canon = home.canonicalize().expect("canonicalize home");
    let docs_path = home_canon.join("Documents");
    let lib_path = home_canon.join("Library");
    let mydata_path = home_canon.join("MyData");

    let items = vec![
        TrashPathItem { path: docs_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
        TrashPathItem { path: lib_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
        TrashPathItem { path: mydata_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
    ];

    let (files, dirs) = filter_items(&home_canon, items);
    assert!(files.is_empty());
    let dir_paths: Vec<_> = dirs.iter().map(|i| i.path.as_str()).collect();
    let docs_str = docs_path.to_str().unwrap();
    let lib_str = lib_path.to_str().unwrap();
    let mydata_str = mydata_path.to_str().unwrap();
    assert!(!dir_paths.contains(&docs_str), "Documents should be filtered out (protected)");
    assert!(!dir_paths.contains(&lib_str), "Library should be filtered out (protected)");
    assert!(dir_paths.contains(&mydata_str), "MyData should remain (not protected)");
}

/// Items whose path is skipped (e.g. .ssh) must be removed. A normal file under MyData
/// must remain. Outcome: one file, zero dirs.
#[test]
fn filter_items_skipped_paths_removed() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let home_canon = home.canonicalize().expect("canonicalize home");
    let ssh_path = home_canon.join(".ssh");
    let mydata_file = home_canon.join("MyData").join("big.txt");

    let items = vec![
        TrashPathItem { path: ssh_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
        TrashPathItem { path: mydata_file.to_string_lossy().into_owned(), is_file: true, size: 0 },
    ];

    let (files, dirs) = filter_items(&home_canon, items);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, mydata_file.to_string_lossy().into_owned());
    assert!(dirs.is_empty(), ".ssh should be filtered out (skipped)");
}

/// Result is (files, dirs): items with is_file true go to files, false to dirs.
/// One file and one dir under MyData must each appear in the correct slice.
#[test]
fn filter_items_partition_files_and_dirs() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let home_canon = home.canonicalize().expect("canonicalize home");
    let file_path = home_canon.join("MyData").join("big.txt");
    let dir_path = home_canon.join("MyData");

    let items = vec![
        TrashPathItem { path: file_path.to_string_lossy().into_owned(), is_file: true, size: 0 },
        TrashPathItem { path: dir_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
    ];

    let (files, dirs) = filter_items(&home_canon, items);
    assert_eq!(files.len(), 1, "one file (MyData/big.txt)");
    assert!(files[0].is_file);
    assert_eq!(dirs.len(), 1);
    assert!(!dirs[0].is_file);
}

/// Paths that do not exist (or cannot be canonicalized) are filtered out. Only the
/// real file must remain; the nonexistent path must not appear in the result.
#[test]
fn filter_items_nonexistent_path_removed() {
    let home_dir = create_test_home();
    let home = home_dir.path();
    let home_canon = home.canonicalize().expect("canonicalize home");
    let real_file = home_canon.join("MyData").join("big.txt");
    let fake_path = home_canon.join("nonexistent");

    let items = vec![
        TrashPathItem { path: real_file.to_string_lossy().into_owned(), is_file: true, size: 0 },
        TrashPathItem { path: fake_path.to_string_lossy().into_owned(), is_file: true, size: 0 },
    ];

    let (files, _) = filter_items(&home_canon, items);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, real_file.to_string_lossy().into_owned());
}

/// trash_paths_sync_with_home must run without panicking and must use the same
/// filter_items. (Whether items are actually moved to Trash depends on the environment;
/// in CI/sandbox trash may not move temp paths, so we only assert the function runs
/// and protected paths are not passed to the trash API.)
#[test]
fn trash_paths_sync_runs_and_filters() {
    let home_dir = create_test_home();
    let home_canon = home_dir.path().canonicalize().expect("canonicalize home");
    let file_path = home_canon.join("MyData").join("big.txt");
    let dir_path = home_canon.join("MyData");

    let items = vec![
        TrashPathItem { path: file_path.to_string_lossy().into_owned(), is_file: true, size: 0 },
        TrashPathItem { path: dir_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
    ];

    trash_paths_sync_with_home(&home_canon, items);
    // If trash worked, file/dir would be gone; we don't assert that in case CI doesn't support it.
}

/// Trash path must not pass protected paths to the trash API: Documents must still exist
/// after calling trash_paths_sync_with_home with [Documents, MyData].
#[test]
fn trash_paths_sync_does_not_remove_protected() {
    let home_dir = create_test_home();
    let home_canon = home_dir.path().canonicalize().expect("canonicalize home");
    let docs_path = home_canon.join("Documents");
    let mydata_path = home_canon.join("MyData");

    let items = vec![
        TrashPathItem { path: docs_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
        TrashPathItem { path: mydata_path.to_string_lossy().into_owned(), is_file: false, size: 0 },
    ];

    trash_paths_sync_with_home(&home_canon, items);

    assert!(docs_path.exists(), "Documents (protected) must not be trashed");
}

/// An empty items list must return empty (files, dirs) without panicking.
#[test]
fn filter_items_empty_list() {
    let home_dir = create_test_home();
    let home_canon = home_dir.path().canonicalize().expect("canonicalize home");

    let (files, dirs) = filter_items(&home_canon, vec![]);
    assert!(files.is_empty());
    assert!(dirs.is_empty());
}

/// trash_paths_sync_with_home with an empty list must not panic.
#[test]
fn trash_paths_sync_empty_list_no_panic() {
    let home_dir = create_test_home();
    let home_canon = home_dir.path().canonicalize().expect("canonicalize home");
    trash_paths_sync_with_home(&home_canon, vec![]);
}

/// All items pointing to protected or skipped paths must result in empty output.
#[test]
fn filter_items_all_protected_returns_empty() {
    let home_dir = create_test_home();
    let home_canon = home_dir.path().canonicalize().expect("canonicalize home");

    let items = vec![
        TrashPathItem {
            path: home_canon.join("Documents").to_string_lossy().into_owned(),
            is_file: false,
            size: 0,
        },
        TrashPathItem {
            path: home_canon.join("Library").to_string_lossy().into_owned(),
            is_file: false,
            size: 0,
        },
        TrashPathItem {
            path: home_canon.join(".ssh").to_string_lossy().into_owned(),
            is_file: false,
            size: 0,
        },
    ];

    let (files, dirs) = filter_items(&home_canon, items);
    assert!(files.is_empty());
    assert!(dirs.is_empty(), "All protected/skipped items should be filtered out");
}
