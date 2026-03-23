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
//! The only native dialogs remaining are for the **menu-initiated** manual check
//! flow: "No Updates Available" (when already up to date) and "Update Ready"
//! (offering Restart / Later after a successful download).
//!
//! All dialog strings are translated to match the current app language.

use std::sync::Mutex;

use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

use crate::constants;
use crate::native_dialog;
use crate::store;

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

// ── Dialog labels (used only by the menu-initiated flow) ────────────────────

/// Minimal dialog labels for the menu-initiated update flow.
struct UpdateLabels {
    up_to_date_title: &'static str,
    up_to_date_body_prefix: &'static str,
    ready_title: &'static str,
    ready_body: &'static str,
    restart_button: &'static str,
    later_button: &'static str,
    error_title: &'static str,
    error_body: &'static str,
    ok_button: &'static str,
}

fn labels_for(lang: &str) -> UpdateLabels {
    match lang {
        "it" => UpdateLabels {
            up_to_date_title: "Nessun aggiornamento disponibile",
            up_to_date_body_prefix: "Stai usando l'ultima versione (v",
            ready_title: "Aggiornamento pronto",
            ready_body: "L'aggiornamento è stato scaricato ed è pronto per l'installazione. Riavviare ora?",
            restart_button: "Riavvia",
            later_button: "Non ora",
            error_title: "Errore di aggiornamento",
            error_body: "Impossibile completare l'aggiornamento. Riprova più tardi.",
            ok_button: "OK",
        },
        "es" => UpdateLabels {
            up_to_date_title: "No hay actualizaciones disponibles",
            up_to_date_body_prefix: "Estás usando la última versión (v",
            ready_title: "Actualización lista",
            ready_body: "La actualización se ha descargado y está lista para instalar. ¿Reiniciar ahora?",
            restart_button: "Reiniciar",
            later_button: "Ahora no",
            error_title: "Error de actualización",
            error_body: "No se pudo completar la actualización. Inténtalo más tarde.",
            ok_button: "OK",
        },
        "fr" => UpdateLabels {
            up_to_date_title: "Aucune mise à jour disponible",
            up_to_date_body_prefix: "Vous utilisez la dernière version (v",
            ready_title: "Mise à jour prête",
            ready_body: "La mise à jour a été téléchargée et est prête à être installée. Redémarrer maintenant ?",
            restart_button: "Redémarrer",
            later_button: "Pas maintenant",
            error_title: "Erreur de mise à jour",
            error_body: "Impossible de terminer la mise à jour. Réessayez plus tard.",
            ok_button: "OK",
        },
        "pt" => UpdateLabels {
            up_to_date_title: "Nenhuma atualização disponível",
            up_to_date_body_prefix: "Está a utilizar a última versão (v",
            ready_title: "Atualização pronta",
            ready_body: "A atualização foi transferida e está pronta para instalar. Reiniciar agora?",
            restart_button: "Reiniciar",
            later_button: "Agora não",
            error_title: "Erro de atualização",
            error_body: "Não foi possível concluir a atualização. Tente novamente mais tarde.",
            ok_button: "OK",
        },
        "de" => UpdateLabels {
            up_to_date_title: "Keine Updates verfügbar",
            up_to_date_body_prefix: "Sie verwenden die neueste Version (v",
            ready_title: "Update bereit",
            ready_body: "Das Update wurde heruntergeladen und ist installationsbereit. Jetzt neu starten?",
            restart_button: "Neu starten",
            later_button: "Nicht jetzt",
            error_title: "Update-Fehler",
            error_body: "Das Update konnte nicht abgeschlossen werden. Bitte versuchen Sie es später erneut.",
            ok_button: "OK",
        },
        "ru" => UpdateLabels {
            up_to_date_title: "Обновления не найдены",
            up_to_date_body_prefix: "Вы используете последнюю версию (v",
            ready_title: "Обновление готово",
            ready_body: "Обновление загружено и готово к установке. Перезапустить сейчас?",
            restart_button: "Перезапустить",
            later_button: "Не сейчас",
            error_title: "Ошибка обновления",
            error_body: "Не удалось завершить обновление. Повторите попытку позже.",
            ok_button: "OK",
        },
        "zh" => UpdateLabels {
            up_to_date_title: "没有可用更新",
            up_to_date_body_prefix: "您正在使用最新版本 (v",
            ready_title: "更新已就绪",
            ready_body: "更新已下载，准备安装。立即重启？",
            restart_button: "重启",
            later_button: "以后再说",
            error_title: "更新错误",
            error_body: "无法完成更新，请稍后重试。",
            ok_button: "好",
        },
        "ja" => UpdateLabels {
            up_to_date_title: "アップデートはありません",
            up_to_date_body_prefix: "最新バージョン (v",
            ready_title: "アップデート準備完了",
            ready_body: "アップデートがダウンロードされ、インストールの準備ができました。今すぐ再起動しますか？",
            restart_button: "再起動",
            later_button: "後で",
            error_title: "アップデートエラー",
            error_body: "アップデートを完了できませんでした。後でもう一度お試しください。",
            ok_button: "OK",
        },
        "ar" => UpdateLabels {
            up_to_date_title: "لا تتوفر تحديثات",
            up_to_date_body_prefix: "أنت تستخدم أحدث إصدار (v",
            ready_title: "التحديث جاهز",
            ready_body: "تم تنزيل التحديث وهو جاهز للتثبيت. إعادة التشغيل الآن؟",
            restart_button: "إعادة التشغيل",
            later_button: "ليس الآن",
            error_title: "خطأ في التحديث",
            error_body: "تعذّر إكمال التحديث. يرجى المحاولة لاحقًا.",
            ok_button: "حسنًا",
        },
        // English (default)
        _ => UpdateLabels {
            up_to_date_title: "No Updates Available",
            up_to_date_body_prefix: "You're using the latest version (v",
            ready_title: "Update Ready",
            ready_body: "The update has been downloaded and is ready to install. Restart now?",
            restart_button: "Restart",
            later_button: "Not Now",
            error_title: "Update Error",
            error_body: "Could not complete the update. Please try again later.",
            ok_button: "OK",
        },
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Builds an updater instance with optional GitHub token authentication.
/// When `GITHUB_PAT` is set (e.g. in `.env`), adds an `Authorization` header
/// so the updater can fetch release info from private repositories.
fn build_updater(app: &tauri::AppHandle) -> Result<tauri_plugin_updater::Updater, String> {
    let mut builder = app.updater_builder();

    if let Ok(token) = std::env::var("GITHUB_PAT") {
        builder = builder
            .header("Authorization", format!("token {}", token))
            .map_err(|e| e.to_string())?;
    }

    builder.build().map_err(|e| e.to_string())
}

/// Reads the current app language from the settings store.
fn get_current_language(app: &tauri::AppHandle) -> String {
    store::get_setting_with_handle(app, "language".to_string())
        .ok()
        .flatten()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| constants::DEFAULT_LANGUAGE.to_string())
}

/// Updates the menu item text to the translated "Restart to Update (vX.Y.Z)".
fn update_menu_text_to_restart(app: &tauri::AppHandle, version: &str) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    if let Some(menu) = app.menu() {
        if let Some(tauri::menu::MenuItemKind::MenuItem(item)) =
            menu.get(constants::CHECK_FOR_UPDATES_MENU_ID)
        {
            let _ = item.set_text(format!("{} (v{})", menu_labels.restart_to_update, version));
        }
    }
}

/// Resets the menu item text back to the translated "Check for Updates…".
fn update_menu_text_to_check(app: &tauri::AppHandle) {
    let lang = get_current_language(app);
    let menu_labels = crate::menu_translations::labels_for(&lang);
    if let Some(menu) = app.menu() {
        if let Some(tauri::menu::MenuItemKind::MenuItem(item)) =
            menu.get(constants::CHECK_FOR_UPDATES_MENU_ID)
        {
            let _ = item.set_text(menu_labels.check_for_updates);
        }
    }
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
    *state.ready_version.lock().unwrap() = Some(version.clone());

    // Update menu item text
    update_menu_text_to_restart(&app, &version);

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

/// Resets the menu item text back to "Check for Updates…".
/// Called when the language changes and the menu is rebuilt.
#[tauri::command]
pub fn reset_update_menu(app: tauri::AppHandle) -> Result<(), String> {
    update_menu_text_to_check(&app);
    Ok(())
}

// ── Menu-initiated flow ─────────────────────────────────────────────────────

/// Menu-initiated update flow: check → download → prompt restart.
/// Only shows native dialogs for "no updates" and "update ready" states.
/// No confirmation dialog before downloading — mirrors modern app behavior.
async fn run_menu_update_flow(app: tauri::AppHandle) -> Result<(), String> {
    let lang = get_current_language(&app);
    let labels = labels_for(&lang);

    let check_result = build_updater(&app)?.check().await;

    let update = match check_result {
        Err(e) => {
            native_dialog::show_alert(
                &app,
                labels.error_title.to_string(),
                format!("{}\n\n{}", labels.error_body, e),
                labels.ok_button.to_string(),
                None,
            )?;
            return Err(e.to_string());
        }
        Ok(u) => u,
    };

    let update = match update {
        Some(u) => u,
        None => {
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

    // Download silently — no confirmation dialog
    let version = update.version.clone();
    if let Err(e) = update
        .download_and_install(|_chunk, _total| {}, || {})
        .await
    {
        native_dialog::show_alert(
            &app,
            labels.error_title.to_string(),
            format!("{}\n\n{}", labels.error_body, e),
            labels.ok_button.to_string(),
            None,
        )?;
        return Err(e.to_string());
    }

    // Mark as ready
    let state = app.state::<UpdateState>();
    *state.ready_version.lock().unwrap() = Some(version.clone());
    update_menu_text_to_restart(&app, &version);

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

/// Spawns the menu-initiated update flow from a sync context.
/// If an update is already staged, restarts immediately.
pub fn check_for_updates_from_menu(app: &tauri::AppHandle) {
    // If update is already downloaded, restart immediately
    let state = app.state::<UpdateState>();
    if state.ready_version.lock().unwrap().is_some() {
        app.restart();
    }

    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = run_menu_update_flow(handle).await {
            eprintln!("Update check failed: {e}");
        }
    });
}
