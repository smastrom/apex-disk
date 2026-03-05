//! Disk usage and volume name for the home directory.
//!
//! Uses statvfs for usage stats; volume name from diskutil.

use nix::sys::statvfs;
use std::path::{Path, PathBuf};

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

#[tauri::command]
pub async fn get_disk_usage() -> Result<DiskUsage, String> {
    let home: PathBuf = dirs::home_dir().ok_or("Unable to determine home directory")?;

    // Spawn blocking operations on a separate thread to avoid starving the async runtime.
    // We can return the DiskUsage struct directly from the closure to keep things clean.
    tokio::task::spawn_blocking(move || {
        let vfs =
            statvfs::statvfs(&home).map_err(|e| format!("Failed to get disk stats: {}", e))?;

        let block_size = vfs.fragment_size() as u64;
        let total = vfs.blocks() as u64 * block_size;
        let free = vfs.blocks_available() as u64 * block_size;

        let user_name = home
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| std::env::var("USER").unwrap_or_else(|_| "User".to_string()));

        let volume_name = get_volume_name(&home);
        let home_path = home.to_string_lossy().into_owned();

        Ok(DiskUsage {
            total,
            free,
            volume_name,
            user_name,
            home_path,
        })
    })
    .await
    .map_err(|e| format!("Task failed to join: {}", e))?
}
