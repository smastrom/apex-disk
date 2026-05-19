// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Native macOS dialogs using NSAlert with the embedded app icon.
//!
//! Provides Tauri commands that show native OS dialogs displaying the same
//! app icon used in the About panel, unlike `tauri-plugin-dialog` which
//! shows a generic system icon.

use objc2::{AnyThread, MainThreadMarker};
use objc2_app_kit::{NSAlert, NSAlertStyle, NSImage};
use objc2_foundation::{NSData, NSString};

/// App icon for dialogs (128×128 PNG, same as the About dialog in menu.rs).
const APP_ICON: &[u8] = include_bytes!("../icons/128x128.png");

/// NSAlertFirstButtonReturn value.
const NS_ALERT_FIRST_BUTTON: isize = 1000;

/// Dispatches an NSAlert to the main thread and awaits the user response
/// without blocking an IPC worker thread for the lifetime of the modal.
pub(crate) async fn show_alert(
    app: &tauri::AppHandle,
    title: String,
    body: String,
    ok_label: String,
    cancel_label: Option<String>,
) -> Result<bool, String> {
    // Capacity-1 mpsc acts as an async oneshot — the main-thread closure sends once,
    // we await the receiver, and the channel drops at end of scope.
    let (tx, mut rx) = tauri::async_runtime::channel::<bool>(1);

    app.run_on_main_thread(move || {
        let mtm = MainThreadMarker::new().expect("run_on_main_thread guarantees main thread");

        let alert = NSAlert::new(mtm);
        alert.setMessageText(&NSString::from_str(&title));
        alert.setInformativeText(&NSString::from_str(&body));
        alert.setAlertStyle(NSAlertStyle::Informational);

        let data = NSData::with_bytes(APP_ICON);
        if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
            unsafe { alert.setIcon(Some(&image)) };
        }

        alert.addButtonWithTitle(&NSString::from_str(&ok_label));
        if let Some(ref label) = cancel_label {
            alert.addButtonWithTitle(&NSString::from_str(label));
        }

        let response = alert.runModal();
        let _ = tx.blocking_send(response == NS_ALERT_FIRST_BUTTON);
    })
    .map_err(|e| e.to_string())?;

    rx.recv().await.ok_or_else(|| "dialog channel closed".to_string())
}

/// Shows a native message dialog with the app icon and a single OK button.
#[tauri::command]
pub async fn show_message_dialog(
    app: tauri::AppHandle,
    title: String,
    body: String,
    ok_label: String,
) -> Result<(), String> {
    show_alert(&app, title, body, ok_label, None).await?;
    Ok(())
}

/// Shows a native ask dialog with the app icon and OK/Cancel buttons.
/// Returns `true` if the user clicked the primary (first) button.
#[tauri::command]
pub async fn show_ask_dialog(
    app: tauri::AppHandle,
    title: String,
    body: String,
    ok_label: String,
    cancel_label: String,
) -> Result<bool, String> {
    show_alert(&app, title, body, ok_label, Some(cancel_label)).await
}
