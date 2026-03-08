//! Safe deletion of user-selected files and folders.
//!
//! Items are moved to the macOS Trash so the user can recover them.
//! Protected system folders (see `safe_folders`) are never deleted.
//! Individual failures (permission, in-use, etc.) are ignored so the UI can report
//! success and the user experience stays macOS-style: no error dialogs.

use std::path::Path;

use crate::safe_folders;

/// Payload item for the delete command. Matches frontend DeleteListItem shape used for deletion.
#[derive(serde::Deserialize)]
pub struct DeletePathItem {
    pub path: String,
    pub is_file: bool,
    pub size: u64,
}

/// Result returned after deletion: how many items were actually trashed and the total size freed.
#[derive(serde::Serialize, Clone)]
pub struct DeleteResult {
    pub count: usize,
    pub size: u64,
}

/// Filters items, removing any whose path is protected or skipped.
/// Takes `home` so tests can pass a temp dir; production calls with `dirs::home_dir()?`.
pub fn filter_items(
    home: &Path,
    items: Vec<DeletePathItem>,
) -> (Vec<DeletePathItem>, Vec<DeletePathItem>) {
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
pub fn trash_paths_sync_with_home(home: &Path, items: Vec<DeletePathItem>) -> DeleteResult {
    let (files, dirs) = filter_items(home, items);

    let mut count: usize = 0;
    let mut size: u64 = 0;

    // Delete files first, then dirs. Each item is trashed individually so we
    // can track which ones actually succeeded.
    for item in files.iter().chain(dirs.iter()) {
        if trash::delete(&item.path).is_ok() {
            count += 1;
            size += item.size;
        }
    }

    DeleteResult { count, size }
}

fn trash_paths_sync(items: Vec<DeletePathItem>) -> DeleteResult {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return DeleteResult { count: 0, size: 0 },
    };
    trash_paths_sync_with_home(&home, items)
}

/// Tauri command: runs deletion on a blocking task so the UI stays responsive.
/// Returns the real count and size of successfully trashed items.
#[tauri::command]
pub async fn delete_paths(
    _app: tauri::AppHandle,
    items: Vec<DeletePathItem>,
) -> Result<DeleteResult, String> {
    tauri::async_runtime::spawn_blocking(move || trash_paths_sync(items))
        .await
        .map_err(|e| e.to_string())
}
