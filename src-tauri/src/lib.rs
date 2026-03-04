pub mod constants;
pub mod delete;
pub mod disk;
mod menu;
pub mod menu_translations;
mod native_dialog;
mod permissions;
pub mod safe_folders;
pub mod scan;

use tauri::Emitter;

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
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build());

    #[cfg(any(target_os = "macos", windows, target_os = "linux"))]
    {
        builder = builder
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_process::init());
    }

    #[cfg(feature = "e2e")]
    {
        builder = builder.plugin(tauri_plugin_webdriver::init());
    }

    builder
        .enable_macos_default_menu(false)
        .menu(|handle| menu::build_app_menu(handle, "en"))
        .setup(|app| {
            app.on_menu_event(|app, event| {
                let id = event.id().as_ref();
                if id == constants::RELEASE_NOTES_MENU_ID {
                    let _ = open::that(constants::RELEASE_NOTES_URL);
                } else if id == constants::LICENSE_MENU_ID {
                    let _ = open::that(constants::LICENSE_URL);
                } else if id == constants::CHECK_UPDATES_MENU_ID {
                    let _ = app.emit("check-for-updates", ());
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
            native_dialog::show_message_dialog,
            native_dialog::show_ask_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
