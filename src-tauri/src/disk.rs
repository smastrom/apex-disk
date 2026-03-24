//! Disk usage and volume name for the home directory.
//!
//! Uses the macOS NSURL API for disk stats (to match Finder); volume name from diskutil.
//! With debug builds or `APEX_DISK_DEBUG`, [`crate::log::dev_rust_trace`] on channel **`disk`**
//! logs **`Disk: usage — …`** after a successful read. See **`LOGGING.md`**.

use objc2_foundation::{
    NSArray, NSNumber, NSString, NSURLVolumeAvailableCapacityForImportantUsageKey,
    NSURLVolumeTotalCapacityKey, NSURL,
};
use std::path::{Path, PathBuf};

use crate::log;

/// Parses the volume name from diskutil info stdout. Used by get_volume_name and by tests.
pub fn parse_volume_name(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        if let Some(name) = line.trim_start().strip_prefix("Volume Name:") {
            let name = name.trim();
            // Cleanly strip the " - Data" suffix if it exists
            let display = name.strip_suffix(" - Data").unwrap_or(name);
            return Some(display.to_string());
        }
    }
    None
}

pub fn get_volume_name(path: &Path) -> String {
    use std::process::Command;

    // A simpler, more readable way to build our fallback paths
    let mut paths = vec![];
    if let Some(p) = path.to_str() {
        paths.push(p);
    }
    paths.extend(["/System/Volumes/Data", "/"]);

    for p in paths {
        if p.is_empty() {
            continue;
        }

        if let Ok(o) = Command::new("/usr/sbin/diskutil")
            .args(["info", p])
            .output()
        {
            if o.status.success() {
                let stdout = String::from_utf8_lossy(&o.stdout);
                if let Some(name) = parse_volume_name(&stdout) {
                    return name;
                }
            }
        }
    }
    "Startup Disk".to_string()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DiskUsage {
    pub total: u64,
    pub free: u64,
    pub volume_name: String,
    pub user_name: String,
    pub home_path: String,
}

/// Gets total and free disk space using the macOS NSURL resource values API.
/// Uses `volumeAvailableCapacityForImportantUsage` which includes purgeable space,
/// matching the value shown by macOS Finder.
fn get_disk_capacity(path: &Path) -> Result<(u64, u64), String> {
    let path_str = path.to_string_lossy();
    let ns_path = NSString::from_str(&path_str);
    let url = NSURL::fileURLWithPath(&ns_path);

    // SAFETY: These are well-known Apple framework constants, always valid at runtime.
    let (total_key, free_key) = unsafe {
        (
            NSURLVolumeTotalCapacityKey,
            NSURLVolumeAvailableCapacityForImportantUsageKey,
        )
    };

    let keys = NSArray::from_slice(&[total_key, free_key]);

    let values = url
        .resourceValuesForKeys_error(&keys)
        .map_err(|e| format!("Failed to get disk resource values: {}", e))?;

    let total = values
        .objectForKey(total_key)
        .and_then(|v| {
            v.downcast_ref::<NSNumber>()
                .map(|n| u64::try_from(n.longLongValue()).unwrap_or(0))
        })
        .ok_or("Failed to read total capacity")?;

    let free = values
        .objectForKey(free_key)
        .and_then(|v| {
            v.downcast_ref::<NSNumber>()
                .map(|n| u64::try_from(n.longLongValue()).unwrap_or(0))
        })
        .ok_or("Failed to read available capacity")?;

    Ok((total, free))
}

#[tauri::command]
pub async fn get_disk_usage() -> Result<DiskUsage, String> {
    log::dev_rust_trace("disk", "get_disk_usage");
    let home: PathBuf = dirs::home_dir().ok_or("Unable to determine home directory")?;

    // Spawn blocking operations on a separate thread to avoid starving the async runtime.
    tokio::task::spawn_blocking(move || {
        let (total, free) = get_disk_capacity(&home)?;

        let user_name = home
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| std::env::var("USER").unwrap_or_else(|_| "User".to_string()));

        let volume_name = get_volume_name(&home);
        let home_path = home.to_string_lossy().into_owned();

        let disk = DiskUsage {
            total,
            free,
            volume_name,
            user_name,
            home_path,
        };

        let used = disk.total.saturating_sub(disk.free);
        log::dev_rust_trace(
            "disk",
            &format!(
                "Disk: usage — volume={} total={} free={} used={} user={} home={}",
                disk.volume_name,
                log::format_bytes_si(disk.total),
                log::format_bytes_si(disk.free),
                log::format_bytes_si(used),
                disk.user_name,
                disk.home_path,
            ),
        );

        Ok(disk)
    })
    .await
    .map_err(|e| format!("Task failed to join: {}", e))?
}
