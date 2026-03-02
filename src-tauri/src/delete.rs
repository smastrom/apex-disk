//! Safe, fast deletion of user-selected files and folders.
//!
//! When the `permanentlyDelete` setting is false (the default), items are moved to the
//! macOS Trash so the user can recover them. When true, deletion is permanent and
//! performed in parallel (files first, then directories from deepest to shallowest)
//! for speed. Protected system folders (see `safe_folders`) are never deleted.
//! Individual failures (permission, in-use, etc.) are ignored so the UI can report
//! success and the user experience stays macOS-style: no error dialogs.

use std::path::Path;

use rayon::prelude::*;
use tauri_plugin_store::StoreExt;

use crate::safe_folders;

/// Payload item for the delete command. Matches frontend DeleteListItem shape used for deletion.
#[derive(serde::Deserialize)]
pub struct DeletePathItem {
    pub path: String,
    pub is_file: bool,
}

/// Filters items, removing any whose path is protected or skipped.
fn filter_items(items: Vec<DeletePathItem>) -> Option<(Vec<DeletePathItem>, Vec<DeletePathItem>)> {
    let home = dirs::home_dir()?;

    let (files, dirs): (Vec<_>, Vec<_>) = items
        .into_iter()
        .filter(|i| {
            let p = Path::new(&i.path);
            // Canonicalize first so ../../ sequences can't bypass is_path_protected.
            // If the path doesn't exist or can't be resolved, skip it.
            let canonical = match p.canonicalize() {
                Ok(c) => c,
                Err(_) => return false,
            };
            !safe_folders::is_path_protected(&canonical, &home)
                && !safe_folders::is_path_skipped(&canonical, &home)
        })
        .partition(|i| i.is_file);

    Some((files, dirs))
}

/// Moves items to the macOS Trash via the system API.
fn trash_paths_sync(items: Vec<DeletePathItem>) {
    let (files, dirs) = match filter_items(items) {
        Some(v) => v,
        None => return,
    };

    let paths: Vec<_> = files
        .iter()
        .map(|i| &*i.path)
        .chain(dirs.iter().map(|i| &*i.path))
        .collect();

    let _ = trash::delete_all(&paths);
}

/// Permanently deletes the given paths. Skips protected folders. Fails silently per path.
fn permanent_delete_sync(items: Vec<DeletePathItem>) {
    let (files, dirs) = match filter_items(items) {
        Some(v) => v,
        None => return,
    };

    files.par_iter().for_each(|i| {
        let _ = std::fs::remove_file(&i.path);
    });

    let mut dir_paths: Vec<_> = dirs.into_iter().map(|i| i.path).collect();
    dir_paths.sort_by_key(|p| std::cmp::Reverse(p.len()));

    dir_paths.par_iter().for_each(|p| {
        let _ = std::fs::remove_dir_all(p);
    });
}

/// Reads the `permanentlyDelete` setting from the persisted store. Defaults to `false`.
/// Supports legacy `moveToTrash` (permanentlyDelete = !moveToTrash).
fn read_permanently_delete(app: &tauri::AppHandle) -> bool {
    let store = match app.store("settings.json") {
        Ok(s) => s,
        Err(_) => return false,
    };
    let app_val = match store.get("app") {
        Some(v) => v,
        None => return false,
    };
    if let Some(permanent) = app_val.get("permanentlyDelete").and_then(|v| v.as_bool()) {
        return permanent;
    }
    app_val
        .get("moveToTrash")
        .and_then(|v| v.as_bool())
        .map(|move_to_trash| !move_to_trash)
        .unwrap_or(false)
}

/// Tauri command: runs deletion on a blocking task so the UI stays responsive.
/// Reads the `permanentlyDelete` setting from the store to decide between trash and permanent delete.
#[tauri::command]
pub async fn delete_paths(app: tauri::AppHandle, items: Vec<DeletePathItem>) {
    let permanently_delete = read_permanently_delete(&app);
    tauri::async_runtime::spawn_blocking(move || {
        if permanently_delete {
            permanent_delete_sync(items);
        } else {
            trash_paths_sync(items);
        }
    })
    .await
    .ok();
}
