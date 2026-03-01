//! Native application menu for the menu bar.
//!
//! Builds the app menu (About, Quit), Window (minimize, close), and Help
//! (release notes, license). Menu labels are localized based on the app's
//! language setting; the frontend calls `set_menu_language` to rebuild the menu
//! when the user changes language. Translations live in `menu_translations.rs`.

use crate::constants;
use crate::menu_translations;
use tauri::menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu};

/// App icon for the About dialog (128×128 PNG, embedded at compile time).
const APP_ICON: &[u8] = include_bytes!("../icons/128x128.png");

/// Builds the application menu for the given language. On macOS this becomes
/// the menu bar: app submenu, Window, Help.
pub fn build_app_menu(
    handle: &tauri::AppHandle,
    lang: &str,
) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    let labels = menu_translations::labels_for(lang);

    // ── App submenu ──
    let about_icon = tauri::image::Image::from_bytes(APP_ICON).ok();
    let about = PredefinedMenuItem::about(
        handle,
        None::<&str>,
        Some(AboutMetadata {
            name: Some(constants::APP_NAME.to_string()),
            version: Some(env!("CARGO_PKG_VERSION").into()),
            copyright: Some("MIT License".into()),
            credits: Some(constants::APP_CREDITS.into()),
            website: Some(constants::RELEASE_NOTES_URL.to_string()),
            website_label: Some(labels.website_label.to_string()),
            icon: about_icon,
            ..Default::default()
        }),
    )?;
    let sep = PredefinedMenuItem::separator(handle)?;
    let services = PredefinedMenuItem::services(handle, Some(labels.services))?;
    let sep2 = PredefinedMenuItem::separator(handle)?;
    let hide = PredefinedMenuItem::hide(handle, Some(labels.hide))?;
    let hide_others = PredefinedMenuItem::hide_others(handle, Some(labels.hide_others))?;
    let show_all = PredefinedMenuItem::show_all(handle, Some(labels.show_all))?;
    let sep3 = PredefinedMenuItem::separator(handle)?;
    let quit = PredefinedMenuItem::quit(handle, Some(labels.quit))?;
    let app_submenu = Submenu::with_items(
        handle,
        constants::APP_NAME,
        true,
        &[&about, &sep, &services, &sep2, &hide, &hide_others, &show_all, &sep3, &quit],
    )?;

    // ── Window submenu ──
    let minimize = PredefinedMenuItem::minimize(handle, Some(labels.minimize))?;
    let close_window = PredefinedMenuItem::close_window(handle, Some(labels.close_window))?;
    let window_submenu = Submenu::with_id_and_items(
        handle,
        tauri::menu::WINDOW_SUBMENU_ID,
        labels.window,
        true,
        &[&minimize, &close_window],
    )?;

    // ── Help submenu ──
    let release_notes = MenuItem::with_id(
        handle,
        constants::RELEASE_NOTES_MENU_ID,
        labels.release_notes,
        true,
        None::<&str>,
    )?;
    let read_license = MenuItem::with_id(
        handle,
        constants::LICENSE_MENU_ID,
        labels.license,
        true,
        None::<&str>,
    )?;
    let help_submenu = Submenu::with_id_and_items(
        handle,
        tauri::menu::HELP_SUBMENU_ID,
        labels.help,
        true,
        &[&release_notes, &read_license],
    )?;

    Menu::with_items(handle, &[&app_submenu, &window_submenu, &help_submenu])
}

/// Tauri command: rebuilds the application menu with the given language.
#[tauri::command]
pub fn set_menu_language(app: tauri::AppHandle, lang: String) -> Result<(), String> {
    let menu = build_app_menu(&app, &lang).map_err(|e| e.to_string())?;
    app.set_menu(menu).map_err(|e| e.to_string())?;
    Ok(())
}
