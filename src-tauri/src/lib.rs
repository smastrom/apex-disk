mod constants;
mod delete;
mod disk;
mod menu;
mod menu_translations;
mod permissions;
mod safe_folders;
mod scan;

/// Options for the user folder scan. Passed from the frontend (settings).
#[derive(serde::Deserialize, Clone, Default)]
pub struct ScanOptions {
    #[serde(default)]
    pub show_hidden_files: bool,
    #[serde(default = "default_show_under_1kb")]
    pub show_under_1kb: bool,
    #[serde(default)]
    pub show_zero_byte: bool,
}

fn default_show_under_1kb() -> bool {
    true
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

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .enable_macos_default_menu(false)
        .menu(|handle| menu::build_app_menu(handle, "en"))
        .setup(|app| {
            app.on_menu_event(|_app, event| {
                let id = event.id().as_ref();
                if id == constants::RELEASE_NOTES_MENU_ID {
                    let _ = open::that(constants::RELEASE_NOTES_URL);
                } else if id == constants::LICENSE_MENU_ID {
                    let _ = open::that(constants::LICENSE_URL);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            disk::get_disk_usage,
            scan::get_user_folders,
            delete::delete_paths,
            permissions::check_full_disk_access,
            menu::set_menu_language,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
