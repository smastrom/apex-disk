//! Safe, fast deletion of user-selected files and folders.
//!
//! Deletion is performed in parallel (files first, then directories from deepest to
//! shallowest) for speed. Protected system folders (see `safe_folders`) are never
//! deleted. Individual failures (permission, in-use, etc.) are ignored so the UI
//! can report success and the user experience stays macOS-style: no error dialogs.

use std::path::Path;

use rayon::prelude::*;

use crate::safe_folders;

/// Payload item for the delete command. Matches frontend DeleteListItem shape used for deletion.
#[derive(serde::Deserialize)]
pub struct DeletePathItem {
    pub path: String,
    pub is_file: bool,
}

/// Deletes the given paths. Skips protected folders. Fails silently per path.
fn delete_paths_sync(items: Vec<DeletePathItem>) {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };

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
        })
        .partition(|i| i.is_file);

    files.par_iter().for_each(|i| {
        let _ = std::fs::remove_file(&i.path);
    });

    let mut dir_paths: Vec<_> = dirs.into_iter().map(|i| i.path).collect();
    dir_paths.sort_by_key(|p| std::cmp::Reverse(p.len()));

    dir_paths.par_iter().for_each(|p| {
        let _ = std::fs::remove_dir_all(p);
    });
}

/// Tauri command: runs deletion on a blocking task so the UI stays responsive.
#[tauri::command]
pub async fn delete_paths(items: Vec<DeletePathItem>) {
    tauri::async_runtime::spawn_blocking(move || delete_paths_sync(items))
        .await
        .ok();
}
