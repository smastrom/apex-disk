//! Native application menu for the menu bar.
//!
//! ## Multilanguage Behavior
//!
//! All menu items are now translated to follow the app language setting:
//!
//! **All menu items** (About, Services, Hide, Show All, Quit, Minimize, Close Window, Release Notes, License):
//! - Use translated labels from `menu_translations::labels_for(lang)`
//! - These items follow the APP language setting, not system language
//! - When user changes app language, all items update immediately via `set_menu_language`
//! - Examples: "About", "Hide", "Quit" will appear in Italian when app language is set to Italian, regardless of system language
//!
//! ## Language Sync Flow
//!
//! 1. App startup: `resolve_app_language_inner` detects system language or uses stored preference
//! 2. `set_app_locale` sets macOS `AppleLanguages` (for context menus) and calls `set_menu_language`
//! 3. `set_menu_language` rebuilds menu with translations for current app language
//! 4. User changes language: Frontend calls `set_app_locale` → menu updates immediately
//! 5. Context menus (Look Up, Translate) require app restart to pick up new `AppleLanguages`
//!
//! Builds the app menu (About, Quit), Window (minimize, close), and Help
//! (release notes, license).

use crate::constants;
use crate::menu_translations;
use tauri::menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu};

/// App icon for the About dialog (128×128 PNG, embedded at compile time).
const APP_ICON: &[u8] = include_bytes!("../icons/128x128.png");

/// Sets the application menu language.
#[tauri::command]
pub fn set_menu_language(app: tauri::AppHandle, lang: String) -> Result<(), String> {
    let menu = build_app_menu(&app, &lang).map_err(|e| e.to_string())?;
    app.set_menu(menu).map_err(|e| e.to_string())?;
    Ok(())
}

/// Builds the application menu. On macOS this becomes
/// the menu bar: app submenu, Window, Help. All items follow app language.
pub fn build_app_menu(
    handle: &tauri::AppHandle,
    lang: &str,
) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    let labels = menu_translations::labels_for(lang);

    // ── App submenu ── (all items now follow app language)
    let about_icon = tauri::image::Image::from_bytes(APP_ICON).ok();
    let about = PredefinedMenuItem::about(
        handle,
        Some(labels.about),
        Some(AboutMetadata {
            name: Some(constants::APP_NAME.to_string()),
            version: Some(env!("CARGO_PKG_VERSION").into()),
            copyright: Some("GNU General Public License v3.0".into()),
            credits: Some(constants::APP_CREDITS.into()),
            website: Some(constants::RELEASE_NOTES_URL.to_string()),
            website_label: Some(labels.website_label.to_string()),
            icon: about_icon,
            ..Default::default()
        }),
    )?;
    let sep = PredefinedMenuItem::separator(handle)?;
    // Use translated labels to follow app language setting
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
        &[
            &about,
            &sep,
            &services,
            &sep2,
            &hide,
            &hide_others,
            &show_all,
            &sep3,
            &quit,
        ],
    )?;

    // ── Window submenu ── (custom items need translation)
    let minimize = MenuItem::with_id(handle, "minimize", labels.minimize, true, Some("cmd+m"))?;
    let close_window = MenuItem::with_id(
        handle,
        "close_window",
        labels.close_window,
        true,
        Some("cmd+w"),
    )?;
    let window_submenu = Submenu::with_id_and_items(
        handle,
        tauri::menu::WINDOW_SUBMENU_ID,
        labels.window,
        true,
        &[&minimize, &close_window],
    )?;

    // ── Help submenu ── (custom items need translation)
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
