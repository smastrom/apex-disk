// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

pub mod constants;
pub mod disk;
#[cfg(feature = "e2e")]
pub mod e2e_fixtures;
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
#[derive(serde::Deserialize, Clone)]
pub struct ScanOptions {
    #[serde(default)]
    pub show_hidden_files: bool,
    /// Include `.DS_Store` entries in scan results. Always excluded from
    /// `last_modified` calculations regardless (see `scan::is_system_file`).
    #[serde(default)]
    pub show_ds_store: bool,
    #[serde(default)]
    pub show_under_1kb: bool,
    #[serde(default)]
    pub show_zero_byte: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            show_ds_store: false,
            show_under_1kb: false,
            show_zero_byte: false,
        }
    }
}

fn is_zero_u32(n: &u32) -> bool {
    *n == 0
}

fn is_zero_u64(n: &u64) -> bool {
    *n == 0
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct FolderInfo {
    pub name: String,
    /// Absolute path. Used internally by the scanner (safe-folders, xattr) and
    /// by tests, but excluded from the IPC payload via `#[serde(skip)]`. The
    /// frontend reconstructs each node's path from `ScanResult.root` plus the
    /// chain of `name` values, halving the wire size of the tree.
    #[serde(skip)]
    pub path: String,
    pub size: u64,
    pub children: Vec<FolderInfo>,
    pub is_file: bool,
    pub is_protected: bool,
    pub is_fda_required: bool,
    pub last_modified: Option<i64>,
    /// True when the directory had more entries than the per-folder cap and at
    /// least one was dropped from `children`: files past `scan::MAX_FILES_PER_DIR`
    /// or subfolders past `scan::MAX_FOLDERS_PER_DIR`. Always false for files.
    pub truncated: bool,
    /// Number of files dropped from `children` because the directory had more
    /// than `scan::MAX_FILES_PER_DIR`. Bytes of dropped files still contribute
    /// to `size` (see [`reference/scan-trash-flow.md`]). Zero on files and on
    /// folders that weren't truncated; skipped on the wire when zero.
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub hidden_files_count: u32,
    /// Total bytes of the dropped files counted by `hidden_files_count`.
    #[serde(default, skip_serializing_if = "is_zero_u64")]
    pub hidden_files_size: u64,
    /// Number of subfolders dropped from `children` because the directory had
    /// more than `scan::MAX_FOLDERS_PER_DIR`. Bytes still contribute to `size`.
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub hidden_folders_count: u32,
    /// Total bytes of the dropped subfolders counted by `hidden_folders_count`.
    #[serde(default, skip_serializing_if = "is_zero_u64")]
    pub hidden_folders_size: u64,
}

/// Wire shape of `get_user_folders`. `root` is the home dir; the frontend uses
/// it to reconstruct each node's `path` (which is skipped on the wire).
#[derive(serde::Serialize)]
pub struct ScanResult {
    pub root: String,
    pub folders: Vec<FolderInfo>,
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build());

    #[cfg(feature = "e2e")]
    let builder = builder.plugin(tauri_plugin_webdriver::init());

    builder
        .manage(updater::UpdateState::default())
        .enable_macos_default_menu(false)
        .setup(|app| {
            store::initialize_store(&app.handle())?;

            // Initialize locale (handles first-load system detection).
            // This also sets the macOS locale and builds the menu via
            // resolve_app_language → set_app_locale → set_menu_language.
            locale::resolve_app_language(app.handle().clone())?;

            app.on_menu_event(|app, event| {
                use tauri_plugin_opener::OpenerExt;
                let id = event.id().as_ref();
                if id == constants::CHECK_FOR_UPDATES_MENU_ID {
                    updater::check_for_updates_from_menu(app);
                } else if id == constants::RELEASE_NOTES_MENU_ID {
                    let _ = app.opener().open_url(constants::RELEASE_NOTES_URL, None::<&str>);
                } else if id == constants::LICENSE_MENU_ID {
                    let _ = app.opener().open_url(constants::LICENSE_URL, None::<&str>);
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
        .invoke_handler({
            // Note: generate_handler! is a proc macro and cannot accept macro
            // invocations, so both blocks must list handlers explicitly.
            // When adding a new command, update BOTH blocks.
            #[cfg(not(feature = "e2e"))]
            {
                tauri::generate_handler![
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
                    log::log_error_message,
                    updater::check_for_updates_silent,
                    updater::check_for_updates_dialog,
                    updater::download_update,
                    updater::restart_app,
                    updater::set_update_menu_ready,
                    updater::set_update_menu_available,
                    updater::reset_update_menu
                ]
            }
            #[cfg(feature = "e2e")]
            {
                tauri::generate_handler![
                    disk::get_disk_usage,
                    scan::get_user_folders,
                    scan::cancel_scan,
                    trash::trash_paths,
                    trash::set_e2e_trash_mode,
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
                    store::reset_e2e_state,
                    system_info::get_system_info,
                    log::is_debug_mode,
                    log::log_message,
                    log::log_error_message,
                    updater::check_for_updates_silent,
                    updater::check_for_updates_dialog,
                    updater::download_update,
                    updater::restart_app,
                    updater::set_update_menu_ready,
                    updater::set_update_menu_available,
                    updater::reset_update_menu
                ]
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
