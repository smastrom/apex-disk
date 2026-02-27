mod delete;
mod disk;
mod permissions;
mod safe_folders;
mod scan;

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
async fn get_user_folders(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    tauri::async_runtime::spawn_blocking(move || scan::get_user_folders_sync_with_progress(app))
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
