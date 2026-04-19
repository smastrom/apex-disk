//! Shared test support: builds a realistic fake home directory for scan and delete tests.
//!
//! The layout mirrors a subset of macOS home: protected top-level dirs (and some
//! Library subdirs), skipped dirs (credential/store paths), and normal user dirs with
//! files of varying sizes and visibility so ScanOptions and filter behavior can be
//! tested against a proper tree.

use std::fs;
use std::io::Write;

/// Builds a temp dir that looks like a minimal but realistic home directory.
///
/// **Protected (must not be deletable):**
/// - Top-level: Applications, Desktop, Documents, Downloads, Library, Movies, Music, Pictures, Public
/// - Under Library: Application Support, Preferences (so nested protected paths are covered)
///
/// **Skipped (excluded from scan, must not be deletable):**
/// - `.ssh` (with a fake `id_rsa` so descendant is skipped)
/// - `.Trash`
/// - `Library/Keychains`
///
/// **Normal (user data, deletable):**
/// - `MyData` — contains small.txt (100 B), empty.txt (0 B), big.txt (2 KB), .hidden (50 B)
/// - `Projects` — empty dir and a 5 KB file
///
/// **Files for scan option tests:**
/// - 0 B: MyData/empty.txt
/// - &lt; 1 KB: MyData/small.txt, MyData/.hidden, Documents/note.txt (500 B)
/// - ≥ 1 KB: MyData/big.txt (2 KB), Documents/report.txt (2 KB), Projects/app (5 KB)
///
/// This allows tests to assert: protected vs non-protected roots, skipped dirs absent,
/// show_under_1kb / show_zero_byte / show_hidden_files filtering, and delete filter/trash/permanent behavior.
pub fn create_test_home() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path();

    // Protected top-level
    fs::create_dir(path.join("Applications")).expect("Applications");
    fs::create_dir(path.join("Desktop")).expect("Desktop");
    fs::create_dir(path.join("Documents")).expect("Documents");
    fs::create_dir(path.join("Downloads")).expect("Downloads");
    fs::create_dir(path.join("Library")).expect("Library");
    fs::create_dir(path.join("Movies")).expect("Movies");
    fs::create_dir(path.join("Music")).expect("Music");
    fs::create_dir(path.join("Pictures")).expect("Pictures");
    fs::create_dir(path.join("Public")).expect("Public");

    // Protected under Library
    fs::create_dir_all(path.join("Library/Application Support"))
        .expect("Library/Application Support");
    fs::create_dir(path.join("Library/Preferences")).expect("Library/Preferences");

    // Skipped (credential / sensitive)
    fs::create_dir(path.join(".ssh")).expect(".ssh");
    fs::File::create(path.join(".ssh/id_rsa")).expect(".ssh/id_rsa");
    fs::create_dir(path.join(".Trash")).expect(".Trash");
    fs::create_dir_all(path.join("Library/Keychains")).expect("Library/Keychains");

    // Normal user dirs
    fs::create_dir(path.join("MyData")).expect("MyData");
    fs::create_dir(path.join("Projects")).expect("Projects");

    // Files in protected dirs (contents are deletable; dir itself is protected)
    write_file(path.join("Documents/report.txt"), 2048);
    write_file(path.join("Documents/note.txt"), 500);

    // Files in MyData: 0 B, < 1 KB, hidden, ≥ 1 KB
    fs::File::create(path.join("MyData/empty.txt")).expect("MyData/empty.txt");
    write_file(path.join("MyData/small.txt"), 100);
    write_file(path.join("MyData/big.txt"), 2048);
    write_file(path.join("MyData/.hidden"), 50);

    // Files in Projects
    write_file(path.join("Projects/app"), 5120);

    dir
}

fn write_file(path: std::path::PathBuf, size: usize) {
    let mut f = fs::File::create(&path).expect("create file");
    f.write_all(&vec![0u8; size]).expect("write");
}

/// Creates a test home with system files that have recent modification dates to test exclusion logic
#[allow(dead_code)]
pub fn create_test_home_with_system_files() -> tempfile::TempDir {
    let dir = create_test_home();
    let path = dir.path();

    // Create system files with recent modification dates
    let ds_store_path = path.join("MyData/.DS_Store");
    write_file(ds_store_path, 100);

    let localized_path = path.join("MyData/.localized");
    write_file(localized_path, 50);

    // Create an AppleDouble file
    let apple_double_path = path.join("MyData/._big.txt");
    write_file(apple_double_path, 200);

    // For now, just create the files - the actual test will verify they're excluded by name
    // Setting specific modification times requires platform-specific code

    dir
}
