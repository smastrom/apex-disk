// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Safe trashing of user-selected files and folders.
//!
//! Items are moved to the macOS Trash so the user can recover them. Items
//! whose canonical path is protected OR skipped (see `safe_folders`) are
//! filtered out before trashing. Survivors carry the canonical `PathBuf`
//! through to `trash::delete`, so the system call operates on the exact
//! identity the filter approved (no symlink/rename TOCTOU between the
//! check and the delete). Individual `trash::delete` failures (permission,
//! in-use, etc.) are silently skipped; the returned count/size reflect
//! only the items actually trashed, so the UI can surface a real number
//! without per-item error dialogs.

use std::path::{Path, PathBuf};

use crate::{log, safe_folders};

/// Payload item for the trash command. Matches frontend TrashListItem shape used for trashing.
#[derive(serde::Deserialize)]
pub struct TrashPathItem {
    pub path: String,
    pub is_file: bool,
    pub size: u64,
}

/// Survivor of `filter_items`. Carries the canonical `PathBuf` so the
/// eventual `trash::delete` operates on the exact identity the
/// protected/skipped checks approved, closing the symlink/rename TOCTOU
/// window between filter and delete.
pub struct FilteredItem {
    pub canonical: PathBuf,
    pub is_file: bool,
    pub size: u64,
}

/// Result returned after trashing: how many items were actually trashed and the total size freed.
#[derive(serde::Serialize)]
pub struct TrashResult {
    pub count: usize,
    pub size: u64,
}

/// Filters items, removing any whose canonical path is protected or skipped.
/// Survivors carry their canonical `PathBuf` so trashing operates on the same
/// identity that was approved. Items whose `canonicalize` fails (broken
/// symlinks, permission errors, missing files) are dropped fail-closed.
/// Takes `home` so tests can pass a temp dir; production calls with `dirs::home_dir()?`.
pub fn filter_items(
    home: &Path,
    items: Vec<TrashPathItem>,
) -> (Vec<FilteredItem>, Vec<FilteredItem>) {
    items
        .into_iter()
        .filter_map(|i| {
            let p = Path::new(&i.path);
            let canonical = match p.canonicalize() {
                Ok(c) => c,
                Err(err) => {
                    log::dev_rust_trace(
                        "trash",
                        &format!(
                            "filter_items: canonicalize failed for {}: {}",
                            sanitize_log_path(&i.path, home),
                            err
                        ),
                    );
                    return None;
                },
            };
            if safe_folders::is_path_protected(&canonical, home)
                || safe_folders::is_path_skipped(&canonical, home)
            {
                return None;
            }
            Some(FilteredItem { canonical, is_file: i.is_file, size: i.size })
        })
        .partition(|i| i.is_file)
}

// Strips `home` from `raw`; if outside home, falls back to the basename so
// logs never carry full absolute paths that may live outside the user tree.
fn sanitize_log_path(raw: &str, home: &Path) -> String {
    let p = Path::new(raw);
    match p.strip_prefix(home) {
        Ok(rel) if !rel.as_os_str().is_empty() => rel.to_string_lossy().into_owned(),
        _ => p
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "<unknown>".to_string()),
    }
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
        if trash::delete(&item.canonical).is_ok() {
            count += 1;
            size += item.size;
        }
    }

    TrashResult { count, size }
}

#[cfg(not(feature = "e2e"))]
fn trash_paths_sync(items: Vec<TrashPathItem>) -> Result<TrashResult, String> {
    let home = dirs::home_dir().ok_or("Unable to determine user directory")?;
    // Canonicalize home so symlink-resolved item paths still string-prefix it.
    // Surface the failure rather than silently zeroing the batch: the
    // un-canonical fallback would compare canonical items against a
    // non-canonical home, dropping legitimate items as "protected".
    let home =
        home.canonicalize().map_err(|e| format!("Failed to canonicalize home directory: {}", e))?;
    Ok(trash_paths_sync_with_home(&home, items))
}

/// Tauri command: runs trashing on a blocking task so the UI stays responsive.
/// Returns the real count and size of successfully trashed items.
///
/// In e2e mode, returns a mock result without actually moving files to Trash.
/// Controlled by `E2E_TRASH_MODE` (set at runtime via `set_e2e_trash_mode` command):
/// - `"success"` (default): returns the real count/size as if trashing succeeded
/// - `"zero"`: returns `{ count: 0, size: 0 }` (simulates all items failing)
/// - `"error"`: returns `Err(...)` (simulates invoke failure)
#[tauri::command]
pub async fn trash_paths(items: Vec<TrashPathItem>) -> Result<TrashResult, String> {
    log::dev_rust_trace("trash", &format!("trash_paths ({} items)", items.len()));

    #[cfg(feature = "e2e")]
    {
        let mode = E2E_TRASH_MODE.lock().unwrap_or_else(|e| e.into_inner()).clone();
        return match mode.as_str() {
            "zero" => Ok(TrashResult { count: 0, size: 0 }),
            "error" => Err("E2E simulated trash error".to_string()),
            _ => {
                // "success": return optimistic values as if everything was trashed
                let count = items.len();
                let size = items.iter().map(|i| i.size).sum();
                Ok(TrashResult { count, size })
            },
        };
    }

    #[cfg(not(feature = "e2e"))]
    {
        tauri::async_runtime::spawn_blocking(move || trash_paths_sync(items))
            .await
            .map_err(|e| e.to_string())?
    }
}

/// Global trash mode for e2e tests. Defaults to "success" (handled by the `_` match arm).
#[cfg(feature = "e2e")]
static E2E_TRASH_MODE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
// Note: empty string maps to the `_` arm in trash_paths which behaves as "success".

/// Tauri command: sets the e2e trash mock mode at runtime.
/// Callable from tests via `window.__TAURI_INTERNALS__.invoke('set_e2e_trash_mode', { mode })`.
#[cfg(feature = "e2e")]
#[tauri::command]
pub fn set_e2e_trash_mode(mode: String) -> Result<(), String> {
    let valid = ["success", "zero", "error"];
    if !valid.contains(&mode.as_str()) {
        return Err(format!("Invalid trash mode: {mode}. Must be one of: {valid:?}"));
    }
    *E2E_TRASH_MODE.lock().unwrap_or_else(|e| e.into_inner()) = mode;
    Ok(())
}
