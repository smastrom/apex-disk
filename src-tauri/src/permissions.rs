//! Full Disk Access (FDA) for macOS.

fn check_full_disk_access_sync() -> bool {
    // Try to list ~/Library/Safari/ which is TCC-protected and requires FDA.
    // This directory always exists on macOS and listing it does NOT trigger
    // a permission prompt — it simply returns an error without FDA.
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return false,
    };
    std::fs::read_dir(home.join("Library/Safari")).is_ok() // TODO: To be fixed
}

/// Tauri command: checks whether the app has Full Disk Access (runs on a blocking task).
#[tauri::command]
pub async fn check_full_disk_access() -> bool {
    tauri::async_runtime::spawn_blocking(check_full_disk_access_sync)
        .await
        .unwrap_or(false)
}
