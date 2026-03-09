//! Full Disk Access (FDA) for macOS.

/// Checks FDA using the `tauri-plugin-macos-permissions` plugin.
pub fn check_full_disk_access(app: &tauri::AppHandle) -> bool {
    tauri::async_runtime::block_on(
        tauri_plugin_macos_permissions::check_full_disk_access_permission(app.clone()),
    )
}
