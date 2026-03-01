mod delete;
mod disk;
mod permissions;
mod safe_folders;
mod scan;

/// Options for the user folder scan. Passed from the frontend (settings).
#[derive(serde::Deserialize, Clone, Default)]
pub struct ScanOptions {
    #[serde(default)]
    pub show_hidden_files: bool,
    #[serde(default)]
    pub show_zero_byte_files: bool,
    #[serde(default)]
    pub show_zero_byte_folders: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub icon: Option<String>,
    pub children: Vec<FolderInfo>,
    pub is_file: bool,
    pub is_protected: bool,
}

#[tauri::command]
async fn get_user_folders(
    app: tauri::AppHandle,
    options: Option<ScanOptions>,
) -> Result<Vec<FolderInfo>, String> {
    let options = options.unwrap_or_default();
    tauri::async_runtime::spawn_blocking(move || {
        scan::get_user_folders_sync_with_progress(app, options)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn check_full_disk_access() -> bool {
    tauri::async_runtime::spawn_blocking(permissions::check_full_disk_access_sync)
        .await
        .unwrap_or(false)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            disk::get_disk_usage,
            get_user_folders,
            delete::delete_paths,
            check_full_disk_access,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
