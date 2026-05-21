// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Recursive folder tree scan with parallel I/O and progress events.
//!
//! With debug builds or `APEX_DISK_DEBUG`, [`crate::log::dev_rust_trace`] on channel **`scan`**
//! logs the directory path being accumulated (throttled) and each completed top-level folder size.
//! See **`reference/logging.md`**.
//!
//! Builds a FolderInfo tree for the user's home top-level directories,
//! limiting retained file entries per directory to keep memory and IPC bounded.
//!
//! On macOS, when the app has Full Disk Access (FDA), all `read_dir` / file
//! access here succeed for Desktop, Documents, Music, Library, etc. without
//! any per-folder permission prompts. No special code path is needed; the
//! same I/O is used and the OS grants access process-wide when FDA is granted.

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering as AtomicOrdering},
        Arc, Mutex,
    },
    time::Instant,
};

use rayon::prelude::*;
use tauri::Emitter;

use crate::{log, safe_folders, xattr, FolderInfo, ScanOptions};

/// Max file entries kept per directory. We still count ALL file sizes for
/// accuracy, but only retain the N largest as tree entries to avoid millions
/// of allocations and a massive IPC payload. The frontend caps rendered rows
/// at the same number per view; see [`reference/state-lifecycle.md`].
pub const MAX_FILES_PER_DIR: usize = 300;

/// Minimum interval between progress events emitted during recursive scanning.
const PROGRESS_THROTTLE_MS: u64 = 150;

/// Guard that prevents concurrent scans. Only one scan can run at a time.
static SCAN_RUNNING: AtomicBool = AtomicBool::new(false);

/// Per-scan cancellation token, registered by `get_user_folders` and consulted
/// by `cancel_scan`. The current walker carries a clone via `LiveScanState`.
/// `None` between scans, so any `cancel_scan` outside a scan is a no-op.
static ACTIVE_CANCEL: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

/// Number of add_size calls between time checks. Avoids an `Instant::now()`
/// call on every single directory. At ~150ms throttle and typical I/O rates,
/// checking every 512 calls is still well within the emission window.
const EMIT_CHECK_INTERVAL: u64 = 512;

/// RAII guard that releases the global scan lock and clears the registered
/// cancel token on drop. Ensures a scan never gets stuck after a panic or
/// early return from a `?` operator.
struct ScanRunningGuard;

impl Drop for ScanRunningGuard {
    fn drop(&mut self) {
        if let Ok(mut slot) = ACTIVE_CANCEL.lock() {
            *slot = None;
        }
        SCAN_RUNNING.store(false, AtomicOrdering::Release);
    }
}

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
    /// Monotonic elapsed milliseconds since `start` at the last emit.
    /// Using `Instant` (not `SystemTime`) avoids progress freezing or
    /// flooding when the wall clock jumps.
    last_emit_ms: AtomicU64,
    start: Instant,
    app: tauri::AppHandle,
    cancel: Arc<AtomicBool>,
}

impl LiveScanState {
    fn add_size_and_maybe_emit(&self, file_size: u64, folder_path: &Path, home: &Path) {
        self.scanned_size_total.fetch_add(file_size, AtomicOrdering::Relaxed);

        if self.call_count.fetch_add(1, AtomicOrdering::Relaxed) % EMIT_CHECK_INTERVAL != 0 {
            return;
        }

        let now = self.start.elapsed().as_millis() as u64;
        let last = self.last_emit_ms.load(AtomicOrdering::Relaxed);

        if now.saturating_sub(last) >= PROGRESS_THROTTLE_MS
            && self
                .last_emit_ms
                .compare_exchange(last, now, AtomicOrdering::Relaxed, AtomicOrdering::Relaxed)
                .is_ok()
        {
            let scanned = self.scanned_size_total.load(AtomicOrdering::Relaxed);
            let completed_sz = self.completed_size.load(AtomicOrdering::Relaxed);
            let current_top = self.completed.load(AtomicOrdering::Relaxed);
            let folder_name =
                folder_path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string();

            log::dev_rust_trace_lazy("scan", || {
                format!(
                    "Scan: live {} (top-level {}/{}) scanned_total={} completed_top={}",
                    scan_path_for_log(folder_path, home),
                    current_top,
                    self.total,
                    log::format_bytes_si(scanned),
                    log::format_bytes_si(completed_sz),
                )
            });

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

    fn is_cancelled(&self) -> bool {
        self.cancel.load(AtomicOrdering::Acquire)
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

/// Min-heap key for retained "top N largest" files in a directory.
/// Ordering is purely by `size`, with `idx` as a deterministic O(1)
/// tiebreaker so we never compare filenames during heap operations.
struct LargestFileKey {
    size: u64,
    idx: u32,
}

impl PartialEq for LargestFileKey {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.idx == other.idx
    }
}
impl Eq for LargestFileKey {}

impl Ord for LargestFileKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size).then(self.idx.cmp(&other.idx))
    }
}
impl PartialOrd for LargestFileKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct HeapEntry {
    key: Reverse<LargestFileKey>,
    name: String,
    last_modified: Option<i64>,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for HeapEntry {}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sort_children(children: &mut [FolderInfo]) {
    children.sort_unstable_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));
}

/// Returns true if the file/folder is a macOS system file that shouldn't affect last_modified
/// dates. This holds even when `show_ds_store` is on: .DS_Store can be visible in the tree, but is
/// never a candidate for the folder's most-recent-modified estimate.
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
/// Respects options: hidden files, items under 1 KB, and 0 B files/folders are excluded when
/// disabled.
fn build_folder_tree(
    root: &Path,
    home: &Path,
    options: &ScanOptions,
    has_fda: bool,
    cancel: &AtomicBool,
    live: Option<&LiveScanState>,
) -> FolderInfo {
    if cancel.load(AtomicOrdering::Acquire) {
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
            truncated: false,
        };
    }

    let name = root.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string();

    let root_metadata = std::fs::metadata(root);

    let entries = match std::fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => {
            let path = root.to_string_lossy().into_owned();
            let is_protected = safe_folders::is_path_protected(root, home);
            // Skip the getxattr syscall entirely when FDA is granted — the result must be false.
            let is_fda_required = !has_fda && xattr::has_container_manager_attribute(root);
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
                truncated: false,
            };
        },
    };

    // Min-heap retains only the N largest files, capping memory at O(MAX_FILES_PER_DIR)
    // per directory instead of O(total_files). We still accumulate ALL file sizes for accuracy.
    let mut top_files: BinaryHeap<HeapEntry> = BinaryHeap::new();
    let mut dir_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut file_size = 0u64;
    let mut next_idx: u32 = 0;
    // Set to true if at least one file is dropped because the directory has
    // more than MAX_FILES_PER_DIR files. Surfaced to the UI so it can show
    // a "list truncated" notice. Folders are never dropped here.
    let mut truncated = false;

    for (i, entry) in entries.filter_map(|e| e.ok()).enumerate() {
        // Periodic cancellation check inside large directories
        if i % 1000 == 0 && i > 0 && cancel.load(AtomicOrdering::Acquire) {
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
                truncated: false,
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
        if !options.show_ds_store && name == ".DS_Store" {
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

            let idx = next_idx;
            next_idx = next_idx.wrapping_add(1);

            if top_files.len() < MAX_FILES_PER_DIR {
                top_files.push(HeapEntry {
                    key: Reverse(LargestFileKey { size, idx }),
                    name,
                    last_modified,
                });
            } else {
                truncated = true;
                let push = top_files.peek().map(|e| size > e.key.0.size).unwrap_or(false);
                if push {
                    top_files.pop();
                    top_files.push(HeapEntry {
                        key: Reverse(LargestFileKey { size, idx }),
                        name,
                        last_modified,
                    });
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
        .map(|HeapEntry { key, name, last_modified }| (name, key.0.size, last_modified))
        .collect();
    file_entries.reverse();

    // Calculate the most recent last_modified date from non-system files.
    // Only the top N largest files are retained. Small files (which include system files
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
    let is_root_fda_required = !has_fda && xattr::has_container_manager_attribute(root);
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
                truncated: false,
            }
        })
        .collect();

    let dir_children: Vec<FolderInfo> = dir_paths
        .par_iter()
        .map(|p| build_folder_tree(p, home, options, has_fda, cancel, live))
        .filter(|c| {
            (options.show_zero_byte || c.size > 0) && (options.show_under_1kb || c.size >= 1024)
        })
        .collect();

    let dir_size: u64 = dir_children.iter().map(|c| c.size).sum();

    let mut children = files;
    children.extend(dir_children);
    sort_children(&mut children);

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
        truncated,
    }
}

/// Scans top-level folders under `home` with the given options. No progress events.
/// Used by tests and by get_user_folders_sync_with_progress (which adds progress emission).
pub fn scan_user_folders_from_home(
    home: &Path,
    options: &ScanOptions,
    has_fda: bool,
) -> Result<Vec<FolderInfo>, String> {
    // Tests don't cancel; this token is never set, so the walker takes the fast load path.
    let never_cancelled = AtomicBool::new(false);

    let mut folder_paths: Vec<std::path::PathBuf> = Vec::new();
    for entry in
        std::fs::read_dir(home).map_err(|e| format!("Failed to read user directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let is_dir_not_symlink =
            entry.file_type().map(|ft| ft.is_dir() && !ft.is_symlink()).unwrap_or(false);
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
        .map(|path| build_folder_tree(path, home, options, has_fda, &never_cancelled, None))
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
    cancel: Arc<AtomicBool>,
) -> Result<Vec<FolderInfo>, String> {
    let has_fda = crate::permissions::is_full_disk_access_granted();

    #[cfg(feature = "e2e")]
    let user_dir = crate::e2e_fixtures::test_home_path();
    #[cfg(not(feature = "e2e"))]
    let user_dir = dirs::home_dir().ok_or("Unable to determine user directory")?;

    // The e2e fixture is tiny enough that a debug scan can finish before
    // WebdriverIO polls the abort button. Hold the walker for a beat so the
    // SCANNING view is reliably observable, and bail early if abort fires.
    #[cfg(feature = "e2e")]
    {
        let deadline = Instant::now() + std::time::Duration::from_millis(500);
        while Instant::now() < deadline {
            if cancel.load(AtomicOrdering::Acquire) {
                return Err("Scan cancelled".to_string());
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    let folder_paths: Vec<std::path::PathBuf> = {
        let mut paths = Vec::new();
        for entry in std::fs::read_dir(&user_dir)
            .map_err(|e| format!("Failed to read user directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let is_dir_not_symlink =
                entry.file_type().map(|ft| ft.is_dir() && !ft.is_symlink()).unwrap_or(false);
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
        start: Instant::now(),
        app,
        cancel,
    };

    let options_ref = &options;
    let mut folders: Vec<FolderInfo> = folder_paths
        .into_par_iter()
        .map(|path| {
            let info = build_folder_tree(
                &path,
                &user_dir,
                options_ref,
                has_fda,
                &state.cancel,
                Some(&state),
            );

            // Bump completed counter / size and emit a final event for this top-level folder.
            let cur = state.completed.fetch_add(1, AtomicOrdering::Relaxed) + 1;
            let done =
                state.completed_size.fetch_add(info.size, AtomicOrdering::Relaxed) + info.size;
            let scanned_live = state.scanned_size_total.load(AtomicOrdering::Relaxed);

            log::dev_rust_trace_lazy("scan", || {
                format!(
                    "Scan: top-level done {} size={} progress {}/{} scanned_total={} \
                     completed_top={}",
                    scan_path_for_log(&path, &user_dir),
                    log::format_bytes_si(info.size),
                    cur,
                    state.total,
                    log::format_bytes_si(scanned_live),
                    log::format_bytes_si(done),
                )
            });

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

    if state.is_cancelled() {
        return Err("Scan cancelled".to_string());
    }

    sort_children(&mut folders);
    folders.retain(|f| {
        (options.show_zero_byte || f.size > 0) && (options.show_under_1kb || f.size >= 1024)
    });
    Ok(folders)
}

/// Tauri command: runs the folder scan on a blocking task and emits progress events.
/// Only one scan can run at a time; concurrent calls return an error.
#[tauri::command]
pub async fn get_user_folders(
    app: tauri::AppHandle,
    options: Option<crate::ScanOptions>,
) -> Result<Vec<crate::FolderInfo>, String> {
    log::dev_rust_trace("scan", "get_user_folders");

    if SCAN_RUNNING
        .compare_exchange(false, true, AtomicOrdering::Acquire, AtomicOrdering::Relaxed)
        .is_err()
    {
        return Err("A scan is already in progress".to_string());
    }

    // The guard releases `SCAN_RUNNING` and clears `ACTIVE_CANCEL` on drop,
    // including on panic or `?` early-return inside the await below.
    let _guard = ScanRunningGuard;

    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut slot = ACTIVE_CANCEL.lock().unwrap_or_else(|e| e.into_inner());
        *slot = Some(cancel.clone());
    }

    let options = options.unwrap_or_default();
    let cancel_for_task = cancel.clone();
    tauri::async_runtime::spawn_blocking(move || {
        get_user_folders_sync_with_progress(app, options, cancel_for_task)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// Tauri command: cancels any ongoing scan and cleans up resources.
#[tauri::command]
pub fn cancel_scan() -> Result<(), String> {
    log::dev_rust_trace("scan", "cancel_scan");
    if let Ok(slot) = ACTIVE_CANCEL.lock() {
        if let Some(token) = slot.as_ref() {
            token.store(true, AtomicOrdering::Release);
        }
    }
    Ok(())
}
