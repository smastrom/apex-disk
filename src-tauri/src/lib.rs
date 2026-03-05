pub mod constants;
pub mod delete;
pub mod disk;
pub mod locale;
pub mod menu;
pub mod menu_translations;
pub mod native_dialog;
pub mod permissions;
pub mod safe_folders;
pub mod scan;
pub mod store;
pub mod system_info;
pub mod xattr;

use tauri::Manager;

pub const SETTINGS_STORE_PATH: &str = "settings.json";

pub const APP_LANGUAGE_INITIALIZED_KEY: &str = "appLanguageInitialized";

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
    pub is_fda_required: bool,
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
        .setup(|app| {
            // Initialize store with defaults
            store::initialize_store(&app.handle())?;

            // Initialize locale (handles first-load system detection)
            let resolved_lang = locale::resolve_app_language_inner(app.handle().clone())?;

            // Set macOS locale first so native menu items can pick it up
            locale::set_app_locale(app.handle().clone(), resolved_lang.clone())?;

            // Build and set the menu AFTER locale is updated
            menu::set_menu_language(app.handle().clone(), resolved_lang)?;

            app.on_menu_event(|app, event| {
                let id = event.id().as_ref();
                if id == constants::RELEASE_NOTES_MENU_ID {
                    let _ = open::that(constants::RELEASE_NOTES_URL);
                } else if id == constants::LICENSE_MENU_ID {
                    let _ = open::that(constants::LICENSE_URL);
                } else if id == "minimize" {
                    if let Some(window) = app.webview_windows().values().next() {
                        let _ = window.minimize();
                    }
                } else if id == "close_window" {
                    if let Some(window) = app.webview_windows().values().next() {
                        let _ = window.close();
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            disk::get_disk_usage,
            scan::get_user_folders,
            scan::cancel_scan,
            delete::delete_paths,
            permissions::check_full_disk_access,
            native_dialog::show_message_dialog,
            native_dialog::show_ask_dialog,
            locale::set_app_locale,
            locale::get_system_language,
            locale::resolve_app_language,
            menu::set_menu_language,
            store::get_settings,
            store::set_settings,
            store::get_setting,
            store::update_setting,
            system_info::get_system_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
