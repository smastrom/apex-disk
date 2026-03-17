//! In-app update flow using tauri-plugin-updater with native macOS dialogs.
//!
//! Checks for updates against the GitHub releases endpoint configured in
//! `tauri.conf.json`. When triggered (from the menu bar or the Settings UI),
//! shows native NSAlert dialogs for confirm → download → restart.
//!
//! All dialog strings are translated to match the current app language.

use tauri_plugin_updater::UpdaterExt;

use crate::constants;
use crate::native_dialog;
use crate::store;

/// Dialog labels for the update flow, resolved from the current app language.
struct UpdateLabels {
    up_to_date_title: &'static str,
    up_to_date_body_prefix: &'static str,
    available_title: &'static str,
    available_body_prefix: &'static str,
    available_body_suffix: &'static str,
    download_button: &'static str,
    cancel_button: &'static str,
    installing_title: &'static str,
    installing_body: &'static str,
    restart_title: &'static str,
    restart_body: &'static str,
    restart_button: &'static str,
    error_title: &'static str,
    error_body: &'static str,
    ok_button: &'static str,
}

fn labels_for(lang: &str) -> UpdateLabels {
    match lang {
        "it" => UpdateLabels {
            up_to_date_title: "Nessun aggiornamento disponibile",
            up_to_date_body_prefix: "Stai usando l'ultima versione (v",
            available_title: "Aggiornamento disponibile",
            available_body_prefix: "La versione ",
            available_body_suffix: " è disponibile. Vuoi scaricarla e installarla ora?",
            download_button: "Scarica e installa",
            cancel_button: "Non ora",
            installing_title: "Aggiornamento in corso",
            installing_body: "Download e installazione in corso. L'app si riavvierà automaticamente…",
            restart_title: "Aggiornamento installato",
            restart_body: "L'aggiornamento è stato installato. L'app verrà riavviata.",
            restart_button: "Riavvia",
            error_title: "Errore di aggiornamento",
            error_body: "Impossibile completare l'aggiornamento. Riprova più tardi.",
            ok_button: "OK",
        },
        "es" => UpdateLabels {
            up_to_date_title: "No hay actualizaciones disponibles",
            up_to_date_body_prefix: "Estás usando la última versión (v",
            available_title: "Actualización disponible",
            available_body_prefix: "La versión ",
            available_body_suffix: " está disponible. ¿Deseas descargarla e instalarla ahora?",
            download_button: "Descargar e instalar",
            cancel_button: "Ahora no",
            installing_title: "Actualizando",
            installing_body: "Descargando e instalando. La app se reiniciará automáticamente…",
            restart_title: "Actualización instalada",
            restart_body: "La actualización se ha instalado. La app se reiniciará.",
            restart_button: "Reiniciar",
            error_title: "Error de actualización",
            error_body: "No se pudo completar la actualización. Inténtalo más tarde.",
            ok_button: "OK",
        },
        "fr" => UpdateLabels {
            up_to_date_title: "Aucune mise à jour disponible",
            up_to_date_body_prefix: "Vous utilisez la dernière version (v",
            available_title: "Mise à jour disponible",
            available_body_prefix: "La version ",
            available_body_suffix: " est disponible. Voulez-vous la télécharger et l'installer maintenant ?",
            download_button: "Télécharger et installer",
            cancel_button: "Pas maintenant",
            installing_title: "Mise à jour en cours",
            installing_body: "Téléchargement et installation en cours. L'app redémarrera automatiquement…",
            restart_title: "Mise à jour installée",
            restart_body: "La mise à jour a été installée. L'app va redémarrer.",
            restart_button: "Redémarrer",
            error_title: "Erreur de mise à jour",
            error_body: "Impossible de terminer la mise à jour. Réessayez plus tard.",
            ok_button: "OK",
        },
        "pt" => UpdateLabels {
            up_to_date_title: "Nenhuma atualização disponível",
            up_to_date_body_prefix: "Está a utilizar a última versão (v",
            available_title: "Atualização disponível",
            available_body_prefix: "A versão ",
            available_body_suffix: " está disponível. Deseja transferir e instalar agora?",
            download_button: "Transferir e instalar",
            cancel_button: "Agora não",
            installing_title: "A atualizar",
            installing_body: "A transferir e instalar. A app reiniciará automaticamente…",
            restart_title: "Atualização instalada",
            restart_body: "A atualização foi instalada. A app será reiniciada.",
            restart_button: "Reiniciar",
            error_title: "Erro de atualização",
            error_body: "Não foi possível concluir a atualização. Tente novamente mais tarde.",
            ok_button: "OK",
        },
        "de" => UpdateLabels {
            up_to_date_title: "Keine Updates verfügbar",
            up_to_date_body_prefix: "Sie verwenden die neueste Version (v",
            available_title: "Update verfügbar",
            available_body_prefix: "Version ",
            available_body_suffix: " ist verfügbar. Möchten Sie sie jetzt herunterladen und installieren?",
            download_button: "Laden und installieren",
            cancel_button: "Nicht jetzt",
            installing_title: "Aktualisierung läuft",
            installing_body: "Download und Installation laufen. Die App wird automatisch neu gestartet…",
            restart_title: "Update installiert",
            restart_body: "Das Update wurde installiert. Die App wird neu gestartet.",
            restart_button: "Neu starten",
            error_title: "Update-Fehler",
            error_body: "Das Update konnte nicht abgeschlossen werden. Bitte versuchen Sie es später erneut.",
            ok_button: "OK",
        },
        "ru" => UpdateLabels {
            up_to_date_title: "Обновления не найдены",
            up_to_date_body_prefix: "Вы используете последнюю версию (v",
            available_title: "Доступно обновление",
            available_body_prefix: "Доступна версия ",
            available_body_suffix: ". Скачать и установить сейчас?",
            download_button: "Скачать и установить",
            cancel_button: "Не сейчас",
            installing_title: "Обновление",
            installing_body: "Загрузка и установка. Приложение перезапустится автоматически…",
            restart_title: "Обновление установлено",
            restart_body: "Обновление установлено. Приложение будет перезапущено.",
            restart_button: "Перезапустить",
            error_title: "Ошибка обновления",
            error_body: "Не удалось завершить обновление. Повторите попытку позже.",
            ok_button: "OK",
        },
        "zh" => UpdateLabels {
            up_to_date_title: "没有可用更新",
            up_to_date_body_prefix: "您正在使用最新版本 (v",
            available_title: "有可用更新",
            available_body_prefix: "版本 ",
            available_body_suffix: " 可用。是否立即下载并安装？",
            download_button: "下载并安装",
            cancel_button: "以后再说",
            installing_title: "正在更新",
            installing_body: "正在下载并安装。应用将自动重启…",
            restart_title: "更新已安装",
            restart_body: "更新已安装。应用将重新启动。",
            restart_button: "重启",
            error_title: "更新错误",
            error_body: "无法完成更新，请稍后重试。",
            ok_button: "好",
        },
        "ja" => UpdateLabels {
            up_to_date_title: "アップデートはありません",
            up_to_date_body_prefix: "最新バージョン (v",
            available_title: "アップデートが利用可能です",
            available_body_prefix: "バージョン ",
            available_body_suffix: " が利用可能です。今すぐダウンロードしてインストールしますか？",
            download_button: "ダウンロードしてインストール",
            cancel_button: "後で",
            installing_title: "アップデート中",
            installing_body: "ダウンロードとインストール中です。アプリは自動的に再起動します…",
            restart_title: "アップデート完了",
            restart_body: "アップデートがインストールされました。アプリを再起動します。",
            restart_button: "再起動",
            error_title: "アップデートエラー",
            error_body: "アップデートを完了できませんでした。後でもう一度お試しください。",
            ok_button: "OK",
        },
        "ar" => UpdateLabels {
            up_to_date_title: "لا تتوفر تحديثات",
            up_to_date_body_prefix: "أنت تستخدم أحدث إصدار (v",
            available_title: "يتوفر تحديث",
            available_body_prefix: "الإصدار ",
            available_body_suffix: " متاح. هل تريد تنزيله وتثبيته الآن؟",
            download_button: "تنزيل وتثبيت",
            cancel_button: "ليس الآن",
            installing_title: "جارٍ التحديث",
            installing_body: "جارٍ التنزيل والتثبيت. سيُعاد تشغيل التطبيق تلقائيًا…",
            restart_title: "تم تثبيت التحديث",
            restart_body: "تم تثبيت التحديث. سيُعاد تشغيل التطبيق.",
            restart_button: "إعادة التشغيل",
            error_title: "خطأ في التحديث",
            error_body: "تعذّر إكمال التحديث. يرجى المحاولة لاحقًا.",
            ok_button: "حسنًا",
        },
        // English (default)
        _ => UpdateLabels {
            up_to_date_title: "No Updates Available",
            up_to_date_body_prefix: "You're using the latest version (v",
            available_title: "Update Available",
            available_body_prefix: "Version ",
            available_body_suffix: " is available. Would you like to download and install it now?",
            download_button: "Download & Install",
            cancel_button: "Not Now",
            installing_title: "Updating",
            installing_body: "Downloading and installing. The app will restart automatically…",
            restart_title: "Update Installed",
            restart_body: "The update has been installed. The app will restart.",
            restart_button: "Restart",
            error_title: "Update Error",
            error_body: "Could not complete the update. Please try again later.",
            ok_button: "OK",
        },
    }
}

/// Reads the current app language from the settings store.
fn get_current_language(app: &tauri::AppHandle) -> String {
    store::get_setting_with_handle(app, "language".to_string())
        .ok()
        .flatten()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| constants::DEFAULT_LANGUAGE.to_string())
}

/// Core update flow: check → confirm → download → restart.
/// Called from both the menu bar item and the frontend command.
async fn run_update_flow(app: tauri::AppHandle) -> Result<(), String> {
    let lang = get_current_language(&app);
    let labels = labels_for(&lang);

    let check_result = app
        .updater_builder()
        .build()
        .map_err(|e| e.to_string())?
        .check()
        .await;

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

    // Ask user whether to download and install
    let body = format!(
        "{}{}{}",
        labels.available_body_prefix, update.version, labels.available_body_suffix,
    );
    let confirmed = native_dialog::show_alert(
        &app,
        labels.available_title.to_string(),
        body,
        labels.download_button.to_string(),
        Some(labels.cancel_button.to_string()),
    )?;

    if !confirmed {
        return Ok(());
    }

    // Show a non-blocking "installing" dialog, then start the download
    native_dialog::show_alert(
        &app,
        labels.installing_title.to_string(),
        labels.installing_body.to_string(),
        labels.ok_button.to_string(),
        None,
    )?;

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

    // Prompt user to restart
    native_dialog::show_alert(
        &app,
        labels.restart_title.to_string(),
        labels.restart_body.to_string(),
        labels.restart_button.to_string(),
        None,
    )?;

    app.restart();
}

/// Silent update check — returns the available version string or `null`.
/// Used by the frontend on app start and for the inline SettingsView status.
/// Does not show any dialogs.
#[tauri::command]
pub async fn check_for_updates_silent(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let update = app
        .updater_builder()
        .build()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?;

    Ok(update.map(|u| u.version))
}

/// Full update flow with native dialogs: check → confirm → download → restart.
/// Triggered from the menu bar item and the Settings "Check for Updates" button.
#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<(), String> {
    run_update_flow(app).await
}

/// Spawns the update flow from a sync context (e.g. menu event handler).
pub fn check_for_updates_from_menu(app: &tauri::AppHandle) {
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = run_update_flow(handle).await {
            eprintln!("Update check failed: {e}");
        }
    });
}
