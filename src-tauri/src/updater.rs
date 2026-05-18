// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Seamless in-app update flow using tauri-plugin-updater.
//!
//! Checks for updates against the GitHub releases endpoint configured in
//! `tauri.conf.json`. The update experience follows the pattern used by modern
//! desktop apps (Claude, VS Code, Slack):
//!
//! 1. **Auto-check on app start** — the frontend calls `check_for_updates_silent`
//! 2. **Auto-download** — if an update is found, the frontend calls `download_update`
//!    to stage it silently (no dialogs)
//! 3. **Restart prompt** — the UI shows a "Restart to Update" button in Settings
//!    and the menu item text changes to "Restart to Update (vX.Y.Z)"
//! 4. **User restarts** — clicking the button or menu item calls `restart_app`
//!
//! Both the frontend and the menu-initiated flow avoid spamming users: most
//! failures either use native dialogs or reset UI state quietly. Optional
//! diagnostic lines for the update flow go to stdout in **debug builds** (`tauri dev`)
//! or when `APEX_DISK_DEBUG=1` in release builds.

use std::sync::Mutex;

use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

use crate::constants;
use crate::log;
use crate::native_dialog;
use crate::store;

/// Logs an update-related diagnostic to stdout (`[HH:MM:SS.mmm] [apex:rust:updater] …`).
fn log_update(message: &str) {
    let is_verbose = cfg!(debug_assertions) || log::is_apex_disk_debug();
    if !is_verbose {
        return;
    }

    println!(
        "[{}] [apex:rust:updater] {message}",
        log::format_diag_utc_time()
    );
}

// ── Shared state ────────────────────────────────────────────────────────────

/// Tracks whether an update has been downloaded and is ready to install.
/// Managed as Tauri state so both commands and the menu event handler can read it.
pub struct UpdateState {
    pub ready_version: Mutex<Option<String>>,
}

impl Default for UpdateState {
    fn default() -> Self {
        Self {
            ready_version: Mutex::new(None),
        }
    }
}

// ── Dialog labels (menu-initiated flow: "no updates" and "update ready") ────

struct UpdateDialogLabels {
    up_to_date_title: &'static str,
    up_to_date_body_prefix: &'static str,
    ready_title: &'static str,
    ready_body: &'static str,
    restart_button: &'static str,
    later_button: &'static str,
    ok_button: &'static str,
}

fn dialog_labels_for(lang: &str) -> UpdateDialogLabels {
    match lang {
        "it" => UpdateDialogLabels {
            up_to_date_title: "Nessun aggiornamento disponibile",
            up_to_date_body_prefix: "Stai usando l'ultima versione (v",
            ready_title: "Aggiornamento pronto",
            ready_body: "L'aggiornamento è stato scaricato ed è pronto per l'installazione. Riavviare ora?",
            restart_button: "Riavvia",
            later_button: "Non ora",
            ok_button: "OK",
        },
        "es" => UpdateDialogLabels {
            up_to_date_title: "No hay actualizaciones disponibles",
            up_to_date_body_prefix: "Estás usando la última versión (v",
            ready_title: "Actualización lista",
            ready_body: "La actualización se ha descargado y está lista para instalar. ¿Reiniciar ahora?",
            restart_button: "Reiniciar",
            later_button: "Ahora no",
            ok_button: "OK",
        },
        "fr" => UpdateDialogLabels {
            up_to_date_title: "Aucune mise à jour disponible",
            up_to_date_body_prefix: "Vous utilisez la dernière version (v",
            ready_title: "Mise à jour prête",
            ready_body: "La mise à jour a été téléchargée et est prête à être installée. Redémarrer maintenant ?",
            restart_button: "Redémarrer",
            later_button: "Pas maintenant",
            ok_button: "OK",
        },
        "pt" => UpdateDialogLabels {
            up_to_date_title: "Nenhuma atualização disponível",
            up_to_date_body_prefix: "Está a utilizar a última versão (v",
            ready_title: "Atualização pronta",
            ready_body: "A atualização foi transferida e está pronta para instalar. Reiniciar agora?",
            restart_button: "Reiniciar",
            later_button: "Agora não",
            ok_button: "OK",
        },
        "de" => UpdateDialogLabels {
            up_to_date_title: "Keine Updates verfügbar",
            up_to_date_body_prefix: "Sie verwenden die neueste Version (v",
            ready_title: "Update bereit",
            ready_body: "Das Update wurde heruntergeladen und ist installationsbereit. Jetzt neu starten?",
            restart_button: "Neu starten",
            later_button: "Nicht jetzt",
            ok_button: "OK",
        },
        "ru" => UpdateDialogLabels {
            up_to_date_title: "Обновления не найдены",
            up_to_date_body_prefix: "Вы используете последнюю версию (v",
            ready_title: "Обновление готово",
            ready_body: "Обновление загружено и готово к установке. Перезапустить сейчас?",
            restart_button: "Перезапустить",
            later_button: "Не сейчас",
            ok_button: "OK",
        },
        "zh" => UpdateDialogLabels {
            up_to_date_title: "没有可用更新",
            up_to_date_body_prefix: "您正在使用最新版本 (v",
            ready_title: "更新已就绪",
            ready_body: "更新已下载，准备安装。立即重启？",
            restart_button: "重启",
            later_button: "以后再说",
            ok_button: "好",
        },
        "ja" => UpdateDialogLabels {
            up_to_date_title: "アップデートはありません",
            up_to_date_body_prefix: "最新バージョン (v",
            ready_title: "アップデート準備完了",
            ready_body: "アップデートがダウンロードされ、インストールの準備ができました。今すぐ再起動しますか？",
            restart_button: "再起動",
            later_button: "後で",
            ok_button: "OK",
        },
        "ar" => UpdateDialogLabels {
            up_to_date_title: "لا تتوفر تحديثات",
            up_to_date_body_prefix: "أنت تستخدم أحدث إصدار (v",
            ready_title: "التحديث جاهز",
            ready_body: "تم تنزيل التحديث وهو جاهز للتثبيت. إعادة التشغيل الآن؟",
            restart_button: "إعادة التشغيل",
            later_button: "ليس الآن",
            ok_button: "حسنًا",
        },
        _ => UpdateDialogLabels {
            up_to_date_title: "No Updates Available",
            up_to_date_body_prefix: "You're using the latest version (v",
            ready_title: "Update Ready",
            ready_body: "The update has been downloaded and is ready to install. Restart now?",
            restart_button: "Restart",
            later_button: "Not Now",
            ok_button: "OK",
        },
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Builds an updater instance from the app handle.
fn build_updater(app: &tauri::AppHandle) -> Result<tauri_plugin_updater::Updater, String> {
    app.updater_builder().build().map_err(|e| e.to_string())
}

/// Reads the current app language from the settings store.
fn get_current_language(app: &tauri::AppHandle) -> String {
    store::get_setting_with_handle(app, "language".to_string())
        .ok()
        .flatten()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| constants::DEFAULT_LANGUAGE.to_string())
}

/// Reads a boolean settings key, defaulting to false on miss.
fn get_bool_setting(app: &tauri::AppHandle, key: &str) -> bool {
    store::get_setting_with_handle(app, key.to_string())
        .ok()
        .flatten()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

/// Reads the autoInstallUpdates setting (silent download after check).
/// Used to decide whether `download_update` should also flip the menu item
/// text to "Restart to Update", which only makes sense for the silent path.
fn get_auto_install_updates(app: &tauri::AppHandle) -> bool {
    get_bool_setting(app, "autoInstallUpdates")
}

/// Updates the "Check for Updates" menu item text and enabled state.
fn set_update_menu_item(app: &tauri::AppHandle, text: &str, enabled: bool) {
    if let Some(menu) = app.menu() {
        if let Some(tauri::menu::MenuItemKind::MenuItem(item)) =
            menu.get(constants::CHECK_FOR_UPDATES_MENU_ID)
        {
            let _ = item.set_text(text);
            let _ = item.set_enabled(enabled);
        }
    }
}

/// Updates the menu item text to the translated "Restart to Update (vX.Y.Z)".
fn update_menu_text_to_restart(app: &tauri::AppHandle, version: &str) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    set_update_menu_item(
        app,
        &format!("{} (v{})", menu_labels.restart_to_update, version),
        true,
    );
}

/// Resets the menu item text back to the translated "Check for Updates…".
fn update_menu_text_to_check(app: &tauri::AppHandle) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    set_update_menu_item(app, menu_labels.check_for_updates, true);
}

/// Sets the menu item text to "Checking for Updates…" and disables it.
fn update_menu_text_to_checking(app: &tauri::AppHandle) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    set_update_menu_item(app, menu_labels.checking_for_updates, false);
}

/// Sets the menu item text to "Downloading Update…" and disables it.
fn update_menu_text_to_downloading(app: &tauri::AppHandle) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    set_update_menu_item(app, menu_labels.downloading_update, false);
}

/// Sets the menu item text to "Update to vX.Y.Z" — used when a silent check
/// finds an available update but auto-install is off (so nothing was staged).
fn update_menu_text_to_available(app: &tauri::AppHandle, version: &str) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    set_update_menu_item(
        app,
        &format!("{} v{}", menu_labels.update_to_version, version),
        true,
    );
}

// ── Frontend commands ───────────────────────────────────────────────────────

/// Silent update check — returns the available version string or `null`.
/// Used by the frontend on app start and for the inline SettingsView status.
/// Does not show any dialogs.
#[tauri::command]
pub async fn check_for_updates_silent(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let update = build_updater(&app)?
        .check()
        .await
        .map_err(|e| e.to_string())?;

    Ok(update.map(|u| u.version))
}

/// Downloads and stages the update silently (no dialogs).
/// After success, marks the update as ready in shared state and updates the
/// menu item text to "Restart to Update (vX.Y.Z)".
#[tauri::command]
pub async fn download_update(app: tauri::AppHandle) -> Result<String, String> {
    let update = build_updater(&app)?
        .check()
        .await
        .map_err(|e| e.to_string())?;

    let update = update.ok_or_else(|| "No update available".to_string())?;
    let version = update.version.clone();

    update
        .download_and_install(|_chunk, _total| {}, || {})
        .await
        .map_err(|e| e.to_string())?;

    // Mark as ready in shared state
    let state = app.state::<UpdateState>();
    *state
        .ready_version
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(version.clone());

    // Only update menu item text when auto-install is enabled — the menu
    // reflecting "Restart to Update" implies the app staged the update silently.
    if get_auto_install_updates(&app) {
        update_menu_text_to_restart(&app, &version);
    }

    Ok(version)
}

/// Restarts the app to apply the staged update.
#[tauri::command]
pub fn restart_app(app: tauri::AppHandle) {
    app.restart();
}

/// Updates the menu item text to "Restart to Update (vX.Y.Z)".
/// Called from the frontend after a successful download.
#[tauri::command]
pub fn set_update_menu_ready(app: tauri::AppHandle, version: String) -> Result<(), String> {
    update_menu_text_to_restart(&app, &version);
    Ok(())
}

/// Updates the menu item text to "Update to vX.Y.Z".
/// Called from the frontend after a silent check finds an available update
/// but auto-install is off (so the update isn't staged yet).
#[tauri::command]
pub fn set_update_menu_available(app: tauri::AppHandle, version: String) -> Result<(), String> {
    update_menu_text_to_available(&app, &version);
    Ok(())
}

/// Resets the menu item text back to "Check for Updates…".
/// Called when the language changes and the menu is rebuilt.
#[tauri::command]
pub fn reset_update_menu(app: tauri::AppHandle) -> Result<(), String> {
    update_menu_text_to_check(&app);
    Ok(())
}

// ── Native dialog flow ──────────────────────────────────────────────────────

/// Native dialog update flow: check → download → prompt restart.
/// Shows native dialogs for "no updates" and "update ready" states.
/// Used by both the menu click and the frontend button when auto-updates is OFF.
///
/// When `update_menu` is true, the menu item text is updated to reflect
/// the current state (checking, downloading, restart). When false, the menu
/// item stays as "Check for Updates…".
async fn run_dialog_update_flow(app: tauri::AppHandle, update_menu: bool) -> Result<(), String> {
    let lang = get_current_language(&app);
    let labels = dialog_labels_for(&lang);

    log_update("Dialog: checking for updates…");

    if update_menu {
        update_menu_text_to_checking(&app);
    }

    let check_result = build_updater(&app)?.check().await;

    let update = match check_result {
        Err(e) => {
            log_update(&format!("Dialog: update check failed: {e}"));
            if update_menu {
                update_menu_text_to_check(&app);
            }
            // Show "no updates" dialog even on failure (error is already logged)
            let body = format!(
                "{}{})",
                labels.up_to_date_body_prefix,
                constants::APP_VERSION,
            );
            native_dialog::show_alert(
                &app,
                labels.up_to_date_title.to_string(),
                body,
                labels.ok_button.to_string(),
                None,
            )?;
            return Ok(());
        }
        Ok(u) => u,
    };

    let update = match update {
        Some(u) => u,
        None => {
            log_update("Dialog: no updates available");
            if update_menu {
                update_menu_text_to_check(&app);
            }
            let body = format!(
                "{}{})",
                labels.up_to_date_body_prefix,
                constants::APP_VERSION,
            );
            native_dialog::show_alert(
                &app,
                labels.up_to_date_title.to_string(),
                body,
                labels.ok_button.to_string(),
                None,
            )?;
            return Ok(());
        }
    };

    let version = update.version.clone();
    log_update(&format!("Dialog: downloading update v{version}…"));
    if update_menu {
        update_menu_text_to_downloading(&app);
    }

    if let Err(e) = update
        .download_and_install(|_chunk, _total| {}, || {})
        .await
    {
        log_update(&format!("Dialog: update download failed: {e}"));
        if update_menu {
            update_menu_text_to_check(&app);
        }
        return Ok(());
    }

    log_update(&format!("Dialog: update v{version} ready to install"));
    let state = app.state::<UpdateState>();
    *state
        .ready_version
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(version.clone());

    if update_menu {
        update_menu_text_to_restart(&app, &version);
    }

    // Prompt: Restart now or Later?
    let confirmed = native_dialog::show_alert(
        &app,
        labels.ready_title.to_string(),
        labels.ready_body.to_string(),
        labels.restart_button.to_string(),
        Some(labels.later_button.to_string()),
    )?;

    if confirmed {
        app.restart();
    }

    Ok(())
}

/// Frontend command: triggers the native dialog update flow.
/// Used by the Settings button when auto-updates is disabled.
#[tauri::command]
pub async fn check_for_updates_dialog(app: tauri::AppHandle) -> Result<(), String> {
    run_dialog_update_flow(app, false).await
}

/// Spawns the menu-initiated update flow from a sync context.
///
/// - Update already staged (any source) → restart immediately.
/// - Otherwise → run dialog flow with menu text updates (the menu is the
///   entrypoint, so it reflects checking/downloading/restart state).
pub fn check_for_updates_from_menu(app: &tauri::AppHandle) {
    // If update is already downloaded, restart immediately.
    let state = app.state::<UpdateState>();
    let staged = state
        .ready_version
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .is_some();

    if staged {
        app.restart();
    }

    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = run_dialog_update_flow(handle, true).await {
            log_update(&format!("Menu: update flow error: {e}"));
        }
    });
}
