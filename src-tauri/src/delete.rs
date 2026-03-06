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

/// Moves items to the macOS Trash via the system API.
/// Takes `home` so tests can pass a temp dir; production uses `dirs::home_dir()`.
pub fn trash_paths_sync_with_home(home: &Path, items: Vec<DeletePathItem>) {
    let (files, dirs) = filter_items(home, items);

    let paths: Vec<_> = files
        .iter()
        .map(|i| i.path.as_str())
        .chain(dirs.iter().map(|i| i.path.as_str()))
        .collect();

    let _ = trash::delete_all(&paths);
}

fn trash_paths_sync(items: Vec<DeletePathItem>) {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };
    trash_paths_sync_with_home(&home, items);
}

/// Tauri command: runs deletion on a blocking task so the UI stays responsive.
/// Always moves files to trash.
#[tauri::command]
pub async fn delete_paths(_app: tauri::AppHandle, items: Vec<DeletePathItem>) {
    tauri::async_runtime::spawn_blocking(move || {
        trash_paths_sync(items);
    })
    .await
    .ok();
}
