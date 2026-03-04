//! Recursive folder tree scan with parallel I/O and progress events.
//!
//! Builds a FolderInfo tree for the user's home top-level directories,
//! limiting retained file entries per directory to keep memory and IPC bounded.
//!
//! On macOS, when the app has Full Disk Access (FDA), all `read_dir` / file
//! access here succeed for Desktop, Documents, Music, Library, etc. without
//! any per-folder permission prompts. No special code path is needed — the
//! same I/O is used; the OS grants access process-wide when FDA is granted.

use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use tauri::Emitter;

use crate::safe_folders;
use crate::xattr;
use crate::FolderInfo;
use crate::ScanOptions;

/// Max file entries kept per directory. We still count ALL file sizes for
/// accuracy, but only retain the N largest as tree entries to avoid millions
/// of allocations and a massive IPC payload.
const MAX_FILES_PER_DIR: usize = 50;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct FolderScanProgress {
    current: usize,
    total: usize,
    folder: String,
    size: u64,
    scanned_size_total: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scanning: Option<String>,
}

fn sort_children(children: &mut [FolderInfo]) {
    children.sort_unstable_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));
}

/// Recursively builds the folder tree with parallelism at every directory level.
/// Rayon's work-stealing scheduler naturally balances I/O across cores --
/// threads that finish small directories steal subtasks from large ones like Library.
/// Respects options: hidden files, items under 1 KB, and 0 B files/folders are excluded when disabled.
fn build_folder_tree(root: &Path, home: &Path, options: &ScanOptions, has_fda: bool) -> FolderInfo {
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let entries = match std::fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => {
            let path = root.to_string_lossy().into_owned();
            let is_protected = safe_folders::is_path_protected(root, home);
            let is_fda_required = xattr::has_container_manager_attribute(root) && !has_fda;
            return FolderInfo {
                name,
                path,
                size: 0,
                icon: None,
                children: Vec::new(),
                is_file: false,
                is_protected,
                is_fda_required,
            };
        }
    };

    let mut file_entries: Vec<(String, u64)> = Vec::new();
    let mut dir_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut file_size = 0u64;

    for entry in entries.filter_map(|e| e.ok()) {
        let ft = match entry.file_type() {
            Ok(ft) if !ft.is_symlink() => ft,
            _ => continue,
        };

        let name = entry.file_name().to_string_lossy().into_owned();
        if !options.show_hidden_files && name.starts_with('.') {
            continue;
        }

        if ft.is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            if !options.show_zero_byte && size == 0 {
                continue;
            }
            if !options.show_under_1kb && size < 1024 {
                continue;
            }
            file_size += size;
            file_entries.push((name, size));
        } else if ft.is_dir() {
            let path = entry.path();
            if safe_folders::is_path_skipped(&path, home) {
                continue;
            }
            dir_paths.push(path);
        }
    }

    file_entries.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    file_entries.truncate(MAX_FILES_PER_DIR);

    let root_path = root.to_string_lossy();
    let is_root_protected = safe_folders::is_path_protected(root, home);
    let is_root_fda_required = xattr::has_container_manager_attribute(root) && !has_fda;
    let files: Vec<FolderInfo> = file_entries
        .into_iter()
        .map(|(fname, size)| {
            let path = format!("{}/{}", root_path, fname);
            let file_path = Path::new(&path);
            let is_protected = safe_folders::is_path_protected(file_path, home);
            let is_fda_required = xattr::has_container_manager_attribute(file_path) && !has_fda;
            FolderInfo {
                path,
                name: fname,
                size,
                icon: None,
                children: Vec::new(),
                is_file: true,
                is_protected,
                is_fda_required,
            }
        })
        .collect();

    let dir_children: Vec<FolderInfo> = dir_paths
        .par_iter()
        .map(|p| build_folder_tree(p, home, options, has_fda))
        .collect();

    let dir_children: Vec<FolderInfo> = dir_children
        .into_iter()
        .filter(|c| {
            (options.show_zero_byte || c.size > 0) && (options.show_under_1kb || c.size >= 1024)
        })
        .collect();

    let dir_size: u64 = dir_children.iter().map(|c| c.size).sum();

    let mut children = files;
    children.extend(dir_children);
    sort_children(&mut children);

    FolderInfo {
        name,
        path: root_path.into_owned(),
        size: file_size + dir_size,
        icon: None,
        children,
        is_file: false,
        is_protected: is_root_protected,
        is_fda_required: is_root_fda_required,
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

    let options_ref = options;
    let mut folders: Vec<FolderInfo> = folder_paths
        .par_iter()
        .map(|path| build_folder_tree(path, home, options_ref, has_fda))
        .collect();

    sort_children(&mut folders);
    folders.retain(|f| {
        (options.show_zero_byte || f.size > 0) && (options.show_under_1kb || f.size >= 1024)
    });
    Ok(folders)
}

/// Scans top-level folders in parallel for I/O concurrency, then builds each subtree.
pub fn get_user_folders_sync_with_progress(
    app: tauri::AppHandle,
    options: ScanOptions,
) -> Result<Vec<FolderInfo>, String> {
    // Check FDA status once at the beginning
    let has_fda = crate::permissions::check_full_disk_access_sync().unwrap_or(false);
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

    let total = folder_paths.len();
    let completed = AtomicUsize::new(0);
    let scanned_size_total = AtomicU64::new(0);
    let app_ref = Arc::new(Mutex::new(app));

    let options_ref = &options;
    let mut folders: Vec<FolderInfo> = folder_paths
        .into_par_iter()
        .map(|path| {
            let info = build_folder_tree(&path, &user_dir, options_ref, has_fda);

            let cur = completed.fetch_add(1, Ordering::Relaxed) + 1;
            let total_size = scanned_size_total.fetch_add(info.size, Ordering::Relaxed) + info.size;
            if let Ok(guard) = app_ref.lock() {
                let _ = guard.emit(
                    "folder-scan-progress",
                    &FolderScanProgress {
                        current: cur,
                        total,
                        folder: info.name.clone(),
                        size: info.size,
                        scanned_size_total: total_size,
                        scanning: None,
                    },
                );
            }

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
#[tauri::command]
pub async fn get_user_folders(
    app: tauri::AppHandle,
    options: Option<crate::ScanOptions>,
) -> Result<Vec<crate::FolderInfo>, String> {
    let options = options.unwrap_or_default();
    tauri::async_runtime::spawn_blocking(move || get_user_folders_sync_with_progress(app, options))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}
