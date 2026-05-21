//! Scaling probe for the scan walker. Read-only. Opt-in: every probe is `#[ignore]`d
//! so it never runs under `pnpm test:unit` / `/sync` / CI. Intended for one-off
//! measurement when reasoning about how the frontend would behave on much larger disks.
//!
//! Run a single probe:
//!   cargo test --test scaling_probe probe_system_library -- --ignored --nocapture
//! Run them all:
//!   cargo test --test scaling_probe -- --ignored --nocapture
//!
//! What it measures (per path):
//!   - top-level entries discovered
//!   - total folder + file nodes after the per-dir 300-largest-files cap
//!   - serialized JSON size of the full tree (proxy for the IPC payload that crosses into the
//!     webview at scan end)
//!   - aggregate path-string bytes (the dominant memory cost on the JS side)
//!   - wall-clock scan time
//!
//! These map directly to the four real scaling bottlenecks documented in
//! `reference/temp/scaling-probe.md`: IPC payload, JS heap retention,
//! webview deserialization, scan time.

use std::{path::Path, time::Instant};

use apex_disk_lib::{scan, FolderInfo, ScanOptions};

struct NodeStats {
    folders: usize,
    files: usize,
    path_bytes: usize,
    max_depth: usize,
}

fn walk(node: &FolderInfo, depth: usize, stats: &mut NodeStats) {
    if node.is_file {
        stats.files += 1;
    } else {
        stats.folders += 1;
    }
    stats.path_bytes += node.path.len();
    if depth > stats.max_depth {
        stats.max_depth = depth;
    }
    for child in &node.children {
        walk(child, depth + 1, stats);
    }
}

fn probe(label: &str, root: &Path) {
    if !root.exists() {
        println!("[{label}] skipped: {} does not exist", root.display());
        return;
    }

    let opts = ScanOptions {
        show_hidden_files: false,
        show_ds_store: false,
        show_under_1kb: true,
        show_zero_byte: true,
    };

    let start = Instant::now();
    let result = match scan::scan_user_folders_from_home(root, &opts, false) {
        Ok(r) => r,
        Err(e) => {
            println!("[{label}] scan failed: {e}");
            return;
        },
    };
    let elapsed = start.elapsed();

    let mut stats = NodeStats { folders: 0, files: 0, path_bytes: 0, max_depth: 0 };
    for top in &result {
        walk(top, 1, &mut stats);
    }
    let total_nodes = stats.folders + stats.files;
    let total_size: u64 = result.iter().map(|r| r.size).sum();

    let json = serde_json::to_string(&result).expect("serialize tree");
    let json_bytes = json.len();

    let mib = 1024.0 * 1024.0;
    let gib = mib * 1024.0;

    println!("------------------------------------------------------------");
    println!("[{label}] {}", root.display());
    println!("  top-level entries  : {}", result.len());
    println!("  total folders      : {}", stats.folders);
    println!("  total files (300/dir cap): {}", stats.files);
    println!("  total nodes        : {}", total_nodes);
    println!("  max tree depth     : {}", stats.max_depth);
    println!("  total scanned size : {:.2} GiB", total_size as f64 / gib);
    println!("  path-string bytes  : {:.2} MiB", stats.path_bytes as f64 / mib);
    println!("  serialized JSON    : {:.2} MiB", json_bytes as f64 / mib);
    if total_nodes > 0 {
        println!("  avg bytes/node     : {:.0} (JSON)", json_bytes as f64 / total_nodes as f64);
        println!(
            "  avg path len       : {:.1} chars",
            stats.path_bytes as f64 / total_nodes as f64
        );
    }
    println!("  wall-clock         : {:.2} s", elapsed.as_secs_f64());
    if elapsed.as_secs_f64() > 0.0 {
        println!("  nodes / s          : {:.0}", total_nodes as f64 / elapsed.as_secs_f64());
    }
}

/// Real user home. Baseline matching what production `get_user_folders` scans.
#[test]
#[ignore]
fn probe_user_home() {
    let home = dirs::home_dir().expect("home dir");
    probe("HOME", &home);
}

/// /Applications is small and shallow. Sanity floor for the metric units.
#[test]
#[ignore]
fn probe_applications() {
    probe("/Applications", Path::new("/Applications"));
}

/// /System/Library is large, deep, and present on every Mac. Best universal proxy
/// for "what does a high-folder-count tree look like".
#[test]
#[ignore]
fn probe_system_library() {
    probe("/System/Library", Path::new("/System/Library"));
}

/// Apple-silicon Homebrew prefix. Dev machines often have tens of thousands of
/// folders here (cellars, taps, formulae). Skipped if Homebrew isn't installed.
#[test]
#[ignore]
fn probe_homebrew() {
    probe("/opt/homebrew", Path::new("/opt/homebrew"));
}

/// Intel Homebrew prefix / classic Unix tree. Skipped if absent.
#[test]
#[ignore]
fn probe_usr_local() {
    probe("/usr/local", Path::new("/usr/local"));
}
