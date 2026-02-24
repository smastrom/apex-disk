use tauri::Emitter;

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub icon: Option<String>,
    pub children: Vec<FolderInfo>,
    pub is_file: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct FolderScanProgress {
    current: usize,
    total: usize,
    folder: String,
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scanning: Option<String>,
}

fn sort_children(children: &mut [FolderInfo]) {
    children.sort_unstable_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));
}

/// Max file entries kept per directory. We still count ALL file sizes for
/// accuracy, but only retain the N largest as tree entries to avoid millions
/// of allocations and a massive IPC payload.
const MAX_FILES_PER_DIR: usize = 50;

/// Recursively builds the folder tree with parallelism at every directory level.
/// Rayon's work-stealing scheduler naturally balances I/O across cores --
/// threads that finish small directories steal subtasks from large ones like Library.
fn build_folder_tree(root: &Path) -> FolderInfo {
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let entries = match std::fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => {
            return FolderInfo {
                name,
                path: root.to_string_lossy().into_owned(),
                size: 0,
                icon: None,
                children: Vec::new(),
                is_file: false,
            };
        }
    };

    // Lightweight tuple: only allocate the filename, defer path construction
    let mut file_entries: Vec<(String, u64)> = Vec::new();
    let mut dir_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut file_size = 0u64;

    for entry in entries.filter_map(|e| e.ok()) {
        let ft = match entry.file_type() {
            Ok(ft) if !ft.is_symlink() => ft,
            _ => continue,
        };

        if ft.is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            file_size += size;
            file_entries.push((entry.file_name().to_string_lossy().into_owned(), size));
        } else if ft.is_dir() {
            dir_paths.push(entry.path());
        }
    }

    // Keep only the N largest files -- avoids millions of FolderInfo allocations
    file_entries.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    file_entries.truncate(MAX_FILES_PER_DIR);

    // Build FolderInfo + path string only for the files we keep
    let root_path = root.to_string_lossy();
    let files: Vec<FolderInfo> = file_entries
        .into_iter()
        .map(|(fname, size)| FolderInfo {
            path: format!("{}/{}", root_path, fname),
            name: fname,
            size,
            icon: None,
            children: Vec::new(),
            is_file: true,
        })
        .collect();

    let dir_children: Vec<FolderInfo> =
        dir_paths.par_iter().map(|p| build_folder_tree(p)).collect();

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
    }
}

/// Scans top-level folders in parallel for I/O concurrency, then builds each subtree.
fn get_user_folders_sync_with_progress(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    let user_dir = dirs::home_dir().ok_or("Unable to determine user directory")?;

    let mut folder_paths: Vec<std::path::PathBuf> = Vec::new();
    for entry in
        std::fs::read_dir(&user_dir).map_err(|e| format!("Failed to read user directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let is_dir_not_symlink = entry
            .file_type()
            .map(|ft| ft.is_dir() && !ft.is_symlink())
            .unwrap_or(false);
        if is_dir_not_symlink {
            folder_paths.push(entry.path());
        }
    }

    let total = folder_paths.len();
    let completed = AtomicUsize::new(0);
    let app_ref = Arc::new(Mutex::new(app));

    let mut folders: Vec<FolderInfo> = folder_paths
        .into_par_iter()
        .map(|path| {
            let info = build_folder_tree(&path);

            let cur = completed.fetch_add(1, Ordering::Relaxed) + 1;
            if let Ok(guard) = app_ref.lock() {
                let _ = guard.emit(
                    "folder-scan-progress",
                    &FolderScanProgress {
                        current: cur,
                        total,
                        folder: info.name.clone(),
                        size: info.size,
                        scanning: None,
                    },
                );
            }

            info
        })
        .collect();

    sort_children(&mut folders);
    Ok(folders)
}

#[tauri::command]
async fn get_user_folders(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    tauri::async_runtime::spawn_blocking(move || get_user_folders_sync_with_progress(app))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, get_user_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
