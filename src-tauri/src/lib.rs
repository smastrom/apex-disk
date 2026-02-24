// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::Path;
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub icon: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct FolderScanProgress {
    current: usize,
    total: usize,
    folder: String,
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scanning: Option<String>,
}

fn get_user_folders_sync_with_progress(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    let user_dir = dirs::home_dir().ok_or("Unable to determine user directory")?;

    // Collect folder paths (skip symlinks only)
    let mut folder_paths: Vec<(std::path::PathBuf, String)> = Vec::new();
    for entry in
        std::fs::read_dir(&user_dir).map_err(|e| format!("Failed to read user directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        let is_symlink = entry.file_type().map(|ft| ft.is_symlink()).unwrap_or(false);
        if path.is_dir() && !is_symlink {
            let folder_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            folder_paths.push((path, folder_name));
        }
    }

    let total = folder_paths.len();
    let mut folders: Vec<FolderInfo> = Vec::with_capacity(total);

    // Sequential scan: ensures all folders are included
    for (current, (path, folder_name)) in folder_paths.into_iter().enumerate() {
        let _ = app.emit(
            "folder-scan-progress",
            &FolderScanProgress {
                current,
                total,
                folder: String::new(),
                size: 0,
                scanning: Some(folder_name.clone()),
            },
        );

        let size = calculate_folder_size(&path).unwrap_or(0);

        let _ = app.emit(
            "folder-scan-progress",
            &FolderScanProgress {
                current: current + 1,
                total,
                folder: folder_name.clone(),
                size,
                scanning: None,
            },
        );

        folders.push(FolderInfo {
            name: folder_name,
            path: path.to_string_lossy().to_string(),
            size,
            icon: None,
        });
    }

    folders.sort_by(|a, b| {
        a.size
            .cmp(&b.size)
            .reverse()
            .then_with(|| a.name.cmp(&b.name))
    });

    Ok(folders)
}

/// Runs the heavy disk I/O in a background thread so the UI stays responsive.
/// Emits `folder-scan-progress` events as each folder is scanned for real-time progress.
#[tauri::command]
async fn get_user_folders(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    tauri::async_runtime::spawn_blocking(move || get_user_folders_sync_with_progress(app))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

/// Recursive directory size using std::fs. Skips symlinks.
fn calculate_folder_size(path: &Path) -> Result<u64, String> {
    if std::fs::symlink_metadata(path)
        .map(|m| m.is_symlink())
        .unwrap_or(false)
    {
        return Ok(0);
    }

    if path.is_dir() {
        let mut size = 0u64;
        for entry in std::fs::read_dir(path)
            .map_err(|e| format!("Cannot read directory {}: {}", path.display(), e))?
        {
            let entry = entry.map_err(|e| format!("Cannot read entry: {}", e))?;
            let entry_path = entry.path();
            if std::fs::symlink_metadata(&entry_path)
                .map(|m| m.is_symlink())
                .unwrap_or(false)
            {
                continue;
            }
            if entry_path.is_dir() {
                size += calculate_folder_size(&entry_path).unwrap_or(0);
            } else if entry_path.is_file() {
                size += std::fs::metadata(&entry_path).map(|m| m.len()).unwrap_or(0);
            }
        }
        Ok(size)
    } else if path.is_file() {
        Ok(std::fs::metadata(path).map(|m| m.len()).unwrap_or(0))
    } else {
        Ok(0)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_user_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
