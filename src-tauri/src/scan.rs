// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Recursive folder tree scan with parallel I/O and progress events.
//!
//! With debug builds or `APEX_DISK_DEBUG`, [`crate::log::dev_rust_trace`] on channel **`scan`**
//! logs the directory path being accumulated (throttled) and each completed top-level folder size.
//! See **`docs/LOGGING.md`**.
//!
//! Builds a FolderInfo tree for the user's home top-level directories,
//! limiting retained file entries per directory to keep memory and IPC bounded.
//!
//! On macOS, when the app has Full Disk Access (FDA), all `read_dir` / file
//! access here succeed for Desktop, Documents, Music, Library, etc. without
//! any per-folder permission prompts. No special code path is needed — the
//! same I/O is used; the OS grants access process-wide when FDA is granted.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use rayon::prelude::*;
use tauri::Emitter;

use crate::log;
use crate::safe_folders;
use crate::xattr;
use crate::FolderInfo;
use crate::ScanOptions;

/// Max file entries kept per directory. We still count ALL file sizes for
/// accuracy, but only retain the N largest as tree entries to avoid millions
/// of allocations and a massive IPC payload.
const MAX_FILES_PER_DIR: usize = 100;

/// Minimum interval between progress events emitted during recursive scanning.
const PROGRESS_THROTTLE_MS: u64 = 150;

/// Global cancellation flag for ongoing scans
static SCAN_CANCELLED: AtomicBool = AtomicBool::new(false);

/// Guard that prevents concurrent scans. Only one scan can run at a time.
static SCAN_RUNNING: AtomicBool = AtomicBool::new(false);

/// Number of add_size calls between time checks. Avoids a SystemTime::now()
/// syscall on every single directory — at ~150ms throttle and typical I/O
/// rates, checking every 512 calls is still well within the emission window.
const EMIT_CHECK_INTERVAL: u64 = 512;

fn scan_path_for_log(path: &Path, home: &Path) -> String {
    match path.strip_prefix(home) {
        Ok(p) if !p.as_os_str().is_empty() => p.to_string_lossy().into_owned(),
        _ => path.to_string_lossy().into_owned(),
    }
}

/// Shared state for emitting live progress events during recursive scanning.
/// File sizes are accumulated at every directory level (not just top-level),
/// and progress events are throttled to avoid overwhelming the frontend.
struct LiveScanState {
    completed: AtomicUsize,
    total: usize,
    scanned_size_total: AtomicU64,
    /// Cumulative size of fully completed top-level folders. Unlike
    /// `scanned_size_total` (which includes partial in-progress work),
    /// this only increases when a folder finishes. The frontend uses
    /// `completed_size * total / current` to estimate the user home size.
    completed_size: AtomicU64,
    call_count: AtomicU64,
    last_emit_ms: AtomicU64,
    app: tauri::AppHandle,
}

impl LiveScanState {
    fn add_size_and_maybe_emit(&self, file_size: u64, folder_path: &Path, home: &Path) {
        self.scanned_size_total
            .fetch_add(file_size, Ordering::Relaxed);

        // Skip the syscall unless enough calls have accumulated
        if self.call_count.fetch_add(1, Ordering::Relaxed) % EMIT_CHECK_INTERVAL != 0 {
            return;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let last = self.last_emit_ms.load(Ordering::Relaxed);

        if now.saturating_sub(last) >= PROGRESS_THROTTLE_MS
            && self
                .last_emit_ms
                .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
        {
            let folder_name = folder_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let scanned = self.scanned_size_total.load(Ordering::Relaxed);
            let completed_sz = self.completed_size.load(Ordering::Relaxed);
            let current_top = self.completed.load(Ordering::Relaxed);

            log::dev_rust_trace(
                "scan",
                &format!(
                    "Scan: live — {} (top-level {}/{}) scanned_total={} completed_top={}",
                    scan_path_for_log(folder_path, home),
                    current_top,
                    self.total,
                    log::format_bytes_si(scanned),
                    log::format_bytes_si(completed_sz),
                ),
            );

            let _ = self.app.emit(
                "folder-scan-progress",
                &FolderScanProgress {
                    current: current_top,
                    total: self.total,
                    folder: folder_name,
                    size: 0,
                    scanned_size_total: scanned,
                    completed_size: completed_sz,
                    scanning: None,
                },
            );
        }
    }
}

#[derive(serde::Serialize, Clone)]
struct FolderScanProgress {
    current: usize,
    total: usize,
    folder: String,
    size: u64,
    scanned_size_total: u64,
    completed_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scanning: Option<String>,
}

fn sort_children(children: &mut [FolderInfo]) {
    children.sort_unstable_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));
}

/// Returns true if the file/folder is a macOS system file that shouldn't affect last_modified dates
fn is_system_file(name: &str) -> bool {
    matches!(
        name,
        ".DS_Store"
            | ".localized"
            | ".DocumentRevisions-V100"
            | ".fseventsd"
            | ".Spotlight-V100"
            | ".TemporaryItems"
            | ".Trashes"
            | ".VolumeIcon.icns"
            | ".com.apple.timemachine.donotpresent"
            | ".com.apple.timemachine.supported"
            | ".metadata_never_index"
            | ".file"
    ) || name.starts_with("._") // AppleDouble resource fork files
}

/// Recursively builds the folder tree with parallelism at every directory level.
/// Rayon's work-stealing scheduler naturally balances I/O across cores --
/// threads that finish small directories steal subtasks from large ones like Library.
/// Respects options: hidden files, items under 1 KB, and 0 B files/folders are excluded when disabled.
fn build_folder_tree(
    root: &Path,
    home: &Path,
    options: &ScanOptions,
    has_fda: bool,
    live: Option<&LiveScanState>,
) -> FolderInfo {
    // Check if scan has been cancelled
    if SCAN_CANCELLED.load(Ordering::Relaxed) {
        return FolderInfo {
            name: "Cancelled".to_string(),
            path: root.to_string_lossy().into_owned(),
            size: 0,
            icon: None,
            children: Vec::new(),
            is_file: false,
            is_protected: false,
            is_fda_required: false,
            last_modified: None,
        };
    }

    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    // Get metadata once for the root directory
    let root_metadata = std::fs::metadata(root);

    let entries = match std::fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => {
            let path = root.to_string_lossy().into_owned();
            let is_protected = safe_folders::is_path_protected(root, home);
            let is_fda_required = xattr::has_container_manager_attribute(root) && !has_fda;
            // For unreadable directories, we can't determine children modification dates
            let last_modified: Option<i64> = root_metadata
                .ok()
                .and_then(|m: std::fs::Metadata| m.modified().ok())
                .and_then(|t: std::time::SystemTime| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d: std::time::Duration| d.as_secs() as i64);

            return FolderInfo {
                name,
                path,
                size: 0,
                icon: None,
                children: Vec::new(),
                is_file: false,
                is_protected,
                is_fda_required,
                last_modified,
            };
        }
    };

    // Min-heap retains only the N largest files, capping memory at O(MAX_FILES_PER_DIR)
    // per directory instead of O(total_files). We still accumulate ALL file sizes for accuracy.
    let mut top_files: BinaryHeap<Reverse<(u64, String, Option<i64>)>> = BinaryHeap::new();
    let mut dir_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut file_size = 0u64;

    for (i, entry) in entries.filter_map(|e| e.ok()).enumerate() {
        // Periodic cancellation check inside large directories
        if i % 1000 == 0 && i > 0 && SCAN_CANCELLED.load(Ordering::Relaxed) {
            return FolderInfo {
                name,
                path: root.to_string_lossy().into_owned(),
                size: 0,
                icon: None,
                children: Vec::new(),
                is_file: false,
                is_protected: false,
                is_fda_required: false,
                last_modified: None,
            };
        }

        let ft = match entry.file_type() {
            Ok(ft) if !ft.is_symlink() => ft,
            _ => continue,
        };

        let name = entry.file_name().to_string_lossy().into_owned();
        if !options.show_hidden_files && name.starts_with('.') {
            continue;
        }

        if ft.is_file() {
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let size = metadata.len();
            let last_modified: Option<i64> = metadata
                .modified()
                .ok()
                .and_then(|t: std::time::SystemTime| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d: std::time::Duration| d.as_secs() as i64);

            if !options.show_zero_byte && size == 0 {
                continue;
            }
            if !options.show_under_1kb && size < 1024 {
                continue;
            }
            file_size += size;

            if top_files.len() < MAX_FILES_PER_DIR {
                top_files.push(Reverse((size, name, last_modified)));
            } else if let Some(&Reverse((smallest, _, _))) = top_files.peek() {
                if size > smallest {
                    top_files.pop();
                    top_files.push(Reverse((size, name, last_modified)));
                }
            }
        } else if ft.is_dir() {
            let path = entry.path();
            if safe_folders::is_path_skipped(&path, home) {
                continue;
            }
            dir_paths.push(path);
        }
    }

    // Accumulate file sizes live so the UI gets a smooth size stream.
    if let Some(state) = live {
        if file_size > 0 {
            state.add_size_and_maybe_emit(file_size, root, home);
        }
    }

    // Drain the min-heap into a sorted vec (largest first).
    let mut file_entries: Vec<(String, u64, Option<i64>)> = top_files
        .into_sorted_vec()
        .into_iter()
        .map(|Reverse((size, name, lm))| (name, size, lm))
        .collect();
    file_entries.reverse();

    // Calculate the most recent last_modified date from non-system files.
    // Only the top N largest files are retained — small files (which include system files
    // like .DS_Store) are intentionally excluded so they don't alter the folder's date.
    let mut most_recent_modified: Option<i64> = None;
    for (fname, _, file_last_modified) in &file_entries {
        if is_system_file(fname) {
            continue;
        }
        if let Some(file_time) = file_last_modified {
            most_recent_modified = match most_recent_modified {
                None => Some(*file_time),
                Some(current) => Some(current.max(*file_time)),
            };
        }
    }

    let is_root_protected = safe_folders::is_path_protected(root, home);
    let is_root_fda_required = xattr::has_container_manager_attribute(root) && !has_fda;
    let files: Vec<FolderInfo> = file_entries
        .into_iter()
        .map(|(fname, size, last_modified)| {
            let file_path = root.join(&fname);
            let is_protected = safe_folders::is_path_protected(&file_path, home);
            let path = file_path.to_string_lossy().into_owned();
            FolderInfo {
                path,
                name: fname,
                size,
                icon: None,
                children: Vec::new(),
                is_file: true,
                is_protected,
                // Only directories can have container-manager xattrs.
                is_fda_required: false,
                last_modified,
            }
        })
        .collect();

    let dir_children: Vec<FolderInfo> = dir_paths
        .par_iter()
        .map(|p| build_folder_tree(p, home, options, has_fda, live))
        .filter(|c| {
            (options.show_zero_byte || c.size > 0) && (options.show_under_1kb || c.size >= 1024)
        })
        .collect();

    let dir_size: u64 = dir_children.iter().map(|c| c.size).sum();

    let mut children = files;
    children.extend(dir_children);
    sort_children(&mut children);

    // Check subdirectories for most recent modification and combine with files (excluding system files)
    for child in &children {
        // Skip system files for last_modified calculation
        if is_system_file(&child.name) {
            continue;
        }
        if let Some(child_time) = child.last_modified {
            most_recent_modified = match most_recent_modified {
                None => Some(child_time),
                Some(current) => Some(current.max(child_time)),
            };
        }
    }

    FolderInfo {
        name,
        path: root.to_string_lossy().into_owned(),
        size: file_size + dir_size,
        icon: None,
        children,
        is_file: false,
        is_protected: is_root_protected,
        is_fda_required: is_root_fda_required,
        last_modified: most_recent_modified,
    }
}

/// Scans top-level folders under `home` with the given options. No progress events.
/// Used by tests and by get_user_folders_sync_with_progress (which adds progress emission).
pub fn scan_user_folders_from_home(
    home: &Path,
    options: &ScanOptions,
    has_fda: bool,
) -> Result<Vec<FolderInfo>, String> {
    let mut folder_paths: Vec<std::path::PathBuf> = Vec::new();
    for entry in
        std::fs::read_dir(home).map_err(|e| format!("Failed to read user directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let is_dir_not_symlink = entry
            .file_type()
            .map(|ft| ft.is_dir() && !ft.is_symlink())
            .unwrap_or(false);
        if !is_dir_not_symlink {
            continue;
        }
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if !options.show_hidden_files && name.starts_with('.') {
            continue;
        }
        if safe_folders::is_path_skipped(&path, home) {
            continue;
        }
        folder_paths.push(path);
    }

    let mut folders: Vec<FolderInfo> = folder_paths
        .par_iter()
        .map(|path| build_folder_tree(path, home, options, has_fda, None))
        .collect();

    sort_children(&mut folders);
    folders.retain(|f| {
        (options.show_zero_byte || f.size > 0) && (options.show_under_1kb || f.size >= 1024)
    });
    Ok(folders)
}

/// Scans top-level folders in parallel for I/O concurrency, then builds each subtree.
/// File sizes are accumulated live during recursion for smooth progress updates.
pub fn get_user_folders_sync_with_progress(
    app: tauri::AppHandle,
    options: ScanOptions,
) -> Result<Vec<FolderInfo>, String> {
    // Check FDA status once at the beginning
    let has_fda = crate::permissions::is_full_disk_access_granted();

    #[cfg(feature = "e2e")]
    let user_dir = crate::e2e_fixtures::test_home_path();
    #[cfg(not(feature = "e2e"))]
    let user_dir = dirs::home_dir().ok_or("Unable to determine user directory")?;

    let folder_paths: Vec<std::path::PathBuf> = {
        let mut paths = Vec::new();
        for entry in std::fs::read_dir(&user_dir)
            .map_err(|e| format!("Failed to read user directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let is_dir_not_symlink = entry
                .file_type()
                .map(|ft| ft.is_dir() && !ft.is_symlink())
                .unwrap_or(false);
            if !is_dir_not_symlink {
                continue;
            }
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if !options.show_hidden_files && name.starts_with('.') {
                continue;
            }
            if safe_folders::is_path_skipped(&path, &user_dir) {
                continue;
            }
            paths.push(path);
        }
        paths
    };

    let state = LiveScanState {
        completed: AtomicUsize::new(0),
        total: folder_paths.len(),
        scanned_size_total: AtomicU64::new(0),
        completed_size: AtomicU64::new(0),
        call_count: AtomicU64::new(0),
        last_emit_ms: AtomicU64::new(0),
        app,
    };

    let options_ref = &options;
    let mut folders: Vec<FolderInfo> = folder_paths
        .into_par_iter()
        .map(|path| {
            let info = build_folder_tree(&path, &user_dir, options_ref, has_fda, Some(&state));

            // Bump completed counter / size and emit a final event for this top-level folder.
            let cur = state.completed.fetch_add(1, Ordering::Relaxed) + 1;
            let done = state.completed_size.fetch_add(info.size, Ordering::Relaxed) + info.size;
            let scanned_live = state.scanned_size_total.load(Ordering::Relaxed);

            log::dev_rust_trace(
                "scan",
                &format!(
                    "Scan: top-level done — {} size={} progress {}/{} scanned_total={} completed_top={}",
                    scan_path_for_log(&path, &user_dir),
                    log::format_bytes_si(info.size),
                    cur,
                    state.total,
                    log::format_bytes_si(scanned_live),
                    log::format_bytes_si(done),
                ),
            );

            let _ = state.app.emit(
                "folder-scan-progress",
                &FolderScanProgress {
                    current: cur,
                    total: state.total,
                    folder: info.name.clone(),
                    size: info.size,
                    scanned_size_total: scanned_live,
                    completed_size: done,
                    scanning: None,
                },
            );

            info
        })
        .collect();

    sort_children(&mut folders);
    folders.retain(|f| {
        (options.show_zero_byte || f.size > 0) && (options.show_under_1kb || f.size >= 1024)
    });
    Ok(folders)
}

/// Tauri command: runs the folder scan on a blocking task and emits progress events.
/// Only one scan can run at a time — concurrent calls return an error.
#[tauri::command]
pub async fn get_user_folders(
    app: tauri::AppHandle,
    options: Option<crate::ScanOptions>,
) -> Result<Vec<crate::FolderInfo>, String> {
    log::dev_rust_trace("scan", "get_user_folders");

    // Prevent concurrent scans
    if SCAN_RUNNING
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        return Err("A scan is already in progress".to_string());
    }

    // Reset cancellation flag at the start of a new scan
    SCAN_CANCELLED.store(false, Ordering::Relaxed);

    let options = options.unwrap_or_default();
    let result = tauri::async_runtime::spawn_blocking(move || {
        get_user_folders_sync_with_progress(app, options)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?;

    SCAN_RUNNING.store(false, Ordering::Release);
    result
}

/// Tauri command: cancels any ongoing scan and cleans up resources.
#[tauri::command]
pub fn cancel_scan() -> Result<(), String> {
    log::dev_rust_trace("scan", "cancel_scan");
    SCAN_CANCELLED.store(true, Ordering::Release);
    Ok(())
}
