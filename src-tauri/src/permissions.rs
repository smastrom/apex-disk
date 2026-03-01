//! Full Disk Access (FDA) for macOS.

use std::ffi::CString;

/// Probes FDA by attempting to open a TCC-protected directory via POSIX `opendir`.
/// Unlike `std::fs::read_dir`, raw `opendir` bypasses the App Sandbox layer
/// and hits the TCC check directly.
fn check_full_disk_access_sync() -> Option<bool> {
    let home = dirs::home_dir()?;
    let c_path = CString::new(home.join("Library/Safari").to_string_lossy().as_bytes()).ok()?;

    unsafe {
        let dir = libc::opendir(c_path.as_ptr());
        if dir.is_null() {
            return Some(false);
        }
        libc::closedir(dir);
    }

    Some(true)
}

/// Tauri command: checks whether the app has Full Disk Access (runs on a blocking task).
#[tauri::command]
pub async fn check_full_disk_access() -> bool {
    tauri::async_runtime::spawn_blocking(|| check_full_disk_access_sync().unwrap_or(false))
        .await
        .unwrap_or(false)
}
