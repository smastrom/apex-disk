//! Full Disk Access (FDA) check for macOS.
//!
//! Pure Rust port of <https://github.com/inket/FullDiskAccess>.

#[cfg(not(feature = "e2e"))]
use std::process::Command;

use crate::log;

/// Returns the macOS major version (e.g. 12 for Monterey, 14 for Sonoma).
#[cfg(not(feature = "e2e"))]
fn macos_major_version() -> Option<u32> {
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;
    let version = String::from_utf8(output.stdout).ok()?;
    version.trim().split('.').next()?.parse().ok()
}

/// Checks whether the current process has Full Disk Access.
///
/// - Returns `false` if running inside an App Sandbox (FDA is never granted to sandboxed apps).
/// - On Monterey (12) and later, probes `~/Library/Containers/com.apple.stocks`.
/// - On Catalina (10.15) through Big Sur (11), probes `~/Library/Safari`.
///
/// In e2e mode, reads `E2E_FDA` env var instead of probing the filesystem.
#[cfg(not(feature = "e2e"))]
pub fn is_full_disk_access_granted() -> bool {
    // Sandboxed apps cannot have FDA.
    if std::env::var_os("APP_SANDBOX_CONTAINER_ID").is_some() {
        return false;
    }

    let Some(home) = dirs::home_dir() else {
        return false;
    };

    let major = macos_major_version().unwrap_or(0);

    // Monterey (12) and later: com.apple.stocks container is TCC-protected.
    // Catalina–Big Sur: Safari directory is TCC-protected.
    let probe_dir = if major >= 12 {
        home.join("Library/Containers/com.apple.stocks")
    } else {
        home.join("Library/Safari")
    };

    std::fs::read_dir(probe_dir).is_ok()
}

#[cfg(feature = "e2e")]
pub fn is_full_disk_access_granted() -> bool {
    std::env::var("E2E_FDA")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Tauri command: checks whether the app has Full Disk Access.
#[tauri::command]
pub async fn check_full_disk_access() -> bool {
    log::dev_rust_trace("permissions", "check_full_disk_access");
    tauri::async_runtime::spawn_blocking(is_full_disk_access_granted)
        .await
        .unwrap_or(false)
}
