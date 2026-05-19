// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Full Disk Access (FDA) check for macOS.
//!
//! Pure Rust port of <https://github.com/inket/FullDiskAccess>.

#[cfg(not(feature = "e2e"))]
use std::process::Command;

use crate::log;

/// Returns the macOS major version (e.g. 12 for Monterey, 14 for Sonoma).
#[cfg(not(feature = "e2e"))]
fn macos_major_version() -> Option<u32> {
    let output = Command::new("sw_vers").arg("-productVersion").output().ok()?;
    let version = String::from_utf8(output.stdout).ok()?;
    version.trim().split('.').next()?.parse().ok()
}

/// Probes a path for FDA. Returns:
/// - `Some(true)` when readable (FDA granted)
/// - `Some(false)` when `PermissionDenied` (FDA definitely missing)
/// - `None` when the result is inconclusive (path missing, other I/O error)
#[cfg(not(feature = "e2e"))]
fn fda_probe(path: &std::path::Path) -> Option<bool> {
    match std::fs::read_dir(path) {
        Ok(_) => Some(true),
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => Some(false),
            _ => None,
        },
    }
}

/// Checks whether the current process has Full Disk Access.
///
/// - Returns `false` if running inside an App Sandbox (FDA is never granted to sandboxed apps).
/// - On Monterey (12) and later, probes `~/Library/Containers/com.apple.stocks` first, then falls
///   back to `~/Library/Safari` if the primary probe is missing.
/// - On Catalina (10.15) through Big Sur (11), probes `~/Library/Safari`.
///
/// Missing probe folders are inconclusive (return `None`); only an explicit
/// `PermissionDenied` from the OS is treated as "FDA not granted". This avoids
/// reporting FDA missing when a perfectly granted user just happens to lack a
/// rarely-installed system folder.
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

    let primary = if major >= 12 {
        home.join("Library/Containers/com.apple.stocks")
    } else {
        home.join("Library/Safari")
    };

    if let Some(result) = fda_probe(&primary) {
        return result;
    }

    // Fallback probe — only reached when the primary probe path is missing.
    let fallback = home.join("Library/Safari");
    fda_probe(&fallback).unwrap_or(false)
}

/// E2E stub: reads `E2E_FDA` (`"true"` ⇒ granted) instead of probing the filesystem.
#[cfg(feature = "e2e")]
pub fn is_full_disk_access_granted() -> bool {
    std::env::var("E2E_FDA").map(|v| v == "true").unwrap_or(false)
}

/// Tauri command: checks whether the app has Full Disk Access.
#[tauri::command]
pub async fn check_full_disk_access() -> bool {
    log::dev_rust_trace("permissions", "check_full_disk_access");
    tauri::async_runtime::spawn_blocking(is_full_disk_access_granted).await.unwrap_or(false)
}
