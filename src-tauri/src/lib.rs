pub mod constants;
pub mod disk;
pub mod locale;
pub mod log;
pub mod menu;
pub mod menu_translations;
pub mod native_dialog;
pub mod permissions;
pub mod safe_folders;
pub mod scan;
pub mod store;
pub mod system_info;
pub mod trash;
pub mod updater;
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
    pub last_modified: Option<i64>,
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build());

    #[cfg(feature = "e2e")]
    let builder = builder.plugin(tauri_plugin_webdriver::init());

    builder
        .manage(updater::UpdateState::default())
        .enable_macos_default_menu(false)
        .setup(|app| {
            // Initialize store with defaults
            store::initialize_store(&app.handle())?;

            // Initialize locale (handles first-load system detection).
            // This also sets the macOS locale and builds the menu via
            // resolve_app_language_inner → set_app_locale → set_menu_language.
            locale::resolve_app_language_inner(app.handle().clone())?;

            app.on_menu_event(|app, event| {
                let id = event.id().as_ref();
                if id == constants::CHECK_FOR_UPDATES_MENU_ID {
                    updater::check_for_updates_from_menu(app);
                } else if id == constants::RELEASE_NOTES_MENU_ID {
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
            trash::trash_paths,
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
            system_info::get_system_info,
            log::is_debug_mode,
            log::log_message,
            updater::check_for_updates_silent,
            updater::download_update,
            updater::restart_app,
            updater::set_update_menu_ready,
            updater::reset_update_menu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
