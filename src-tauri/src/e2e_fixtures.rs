// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! E2E test fixtures: builds a deterministic fake home directory for e2e tests.
//!
//! This module is only compiled when the `e2e` feature is enabled. It provides a
//! static temp directory that persists for the lifetime of the app process, ensuring
//! consistent scan results regardless of the host machine.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;

static TEST_HOME: LazyLock<tempfile::TempDir> = LazyLock::new(create_test_home);

/// Returns the path to the static test home directory.
pub fn test_home_path() -> PathBuf {
    TEST_HOME.path().to_path_buf()
}

/// Builds a temp dir that looks like a minimal but realistic home directory.
///
/// **Protected (folders themselves must not be deletable):**
/// - Top-level: Applications, Desktop, Documents, Downloads, Library, Movies, Music, Pictures, Public
/// - Under Library: Application Support, Preferences
/// - Files inside (deletable): `Documents/report.txt` (2 KB), `Documents/note.txt` (500 B)
///
/// **Skipped (excluded from scan, must not be deletable):**
/// - `.ssh` (with a fake `id_rsa` so descendant is skipped)
/// - `.Trash`
/// - `Library/Keychains`
///
/// **Normal (user data, deletable):**
/// - `MyData` — files of varying sizes, hidden file, nested subdirectories for explode tests
/// - `Projects` — a file and a nested subfolder
///
/// **Nested structure for selection/explode tests:**
/// - `MyData/SubFolder/` — alpha.txt, beta.txt, Deep/gamma.txt
/// - `Projects/src/` — main.rs
fn create_test_home() -> tempfile::TempDir {
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

    // Nested subdirectory for explode/indeterminate tests.
    // Siblings are ≥ 1 KB so they remain visible under the default `showUnder1Kb=false`
    // filter. The multi-level explode test pins these siblings at each intermediate
    // level and needs to see them in the scan results.
    fs::create_dir(path.join("MyData/SubFolder")).expect("MyData/SubFolder");
    write_file(path.join("MyData/SubFolder/alpha.txt"), 1024);
    write_file(path.join("MyData/SubFolder/beta.txt"), 1500);

    // 3-level nesting for deep explode tests. Deep/gamma.txt is ≥ 1 KB so the
    // Deep folder itself passes the default directory size filter (>= 1024).
    fs::create_dir(path.join("MyData/SubFolder/Deep")).expect("MyData/SubFolder/Deep");
    write_file(path.join("MyData/SubFolder/Deep/gamma.txt"), 1024);

    // Files in Projects
    write_file(path.join("Projects/app"), 5120);

    // Nested subfolder in Projects
    fs::create_dir(path.join("Projects/src")).expect("Projects/src");
    write_file(path.join("Projects/src/main.rs"), 1024);

    dir
}

fn write_file(path: PathBuf, size: usize) {
    let mut f = fs::File::create(&path).expect("create file");
    f.write_all(&vec![0u8; size]).expect("write");
}
