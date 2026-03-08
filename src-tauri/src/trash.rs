//! Safe trashing of user-selected files and folders.
//!
//! Items are moved to the macOS Trash so the user can recover them.
//! Protected system folders (see `safe_folders`) are never trashed.
//! Individual failures (permission, in-use, etc.) are ignored so the UI can report
//! success and the user experience stays macOS-style: no error dialogs.

use std::path::Path;

use crate::safe_folders;

/// Payload item for the trash command. Matches frontend TrashListItem shape used for trashing.
#[derive(serde::Deserialize)]
pub struct TrashPathItem {
    pub path: String,
    pub is_file: bool,
    pub size: u64,
}

/// Result returned after trashing: how many items were actually trashed and the total size freed.
#[derive(serde::Serialize, Clone)]
pub struct TrashResult {
    pub count: usize,
    pub size: u64,
}

/// Filters items, removing any whose path is protected or skipped.
/// Takes `home` so tests can pass a temp dir; production calls with `dirs::home_dir()?`.
pub fn filter_items(
    home: &Path,
    items: Vec<TrashPathItem>,
) -> (Vec<TrashPathItem>, Vec<TrashPathItem>) {
    items
        .into_iter()
        .filter(|i| {
            let p = Path::new(&i.path);
            let canonical = match p.canonicalize() {
                Ok(c) => c,
                Err(_) => return false,
            };
            !safe_folders::is_path_protected(&canonical, home)
                && !safe_folders::is_path_skipped(&canonical, home)
        })
        .partition(|i| i.is_file)
}

/// Moves items to the macOS Trash via the system API, one by one.
/// Returns how many items were actually trashed and their total size.
/// Takes `home` so tests can pass a temp dir; production uses `dirs::home_dir()`.
pub fn trash_paths_sync_with_home(home: &Path, items: Vec<TrashPathItem>) -> TrashResult {
    let (files, dirs) = filter_items(home, items);

    let mut count: usize = 0;
    let mut size: u64 = 0;

    // Trash files first, then dirs. Each item is trashed individually so we
    // can track which ones actually succeeded.
    for item in files.iter().chain(dirs.iter()) {
        if trash::delete(&item.path).is_ok() {
            count += 1;
            size += item.size;
        }
    }

    TrashResult { count, size }
}

fn trash_paths_sync(items: Vec<TrashPathItem>) -> TrashResult {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return TrashResult { count: 0, size: 0 },
    };
    trash_paths_sync_with_home(&home, items)
}

/// Tauri command: runs trashing on a blocking task so the UI stays responsive.
/// Returns the real count and size of successfully trashed items.
#[tauri::command]
pub async fn trash_paths(
    _app: tauri::AppHandle,
    items: Vec<TrashPathItem>,
) -> Result<TrashResult, String> {
    tauri::async_runtime::spawn_blocking(move || trash_paths_sync(items))
        .await
        .map_err(|e| e.to_string())
}
