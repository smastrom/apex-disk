// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Native application menu bar (App, Window, Help submenus).
//!
//! All labels (native items included: About, Services, Hide, Show All, Quit,
//! Minimize, Close Window, plus custom items: Check for Updates, Release Notes,
//! License) are pulled from `menu_translations::labels_for(lang)` so the
//! menu bar follows the app language rather than the macOS system language.
//!
//! `set_menu_language` is called by `locale::set_app_locale` whenever the
//! language changes; the menu is rebuilt in place. macOS context menus (Look
//! Up, Translate, etc.) read `AppleLanguages` at launch and still require an
//! app restart to pick up a new language.

use tauri::menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu};

use crate::{constants, menu_translations};

/// App icon for the About dialog (128×128 PNG, embedded at compile time).
const APP_ICON: &[u8] = include_bytes!("../icons/128x128.png");

/// Sets the application menu language.
#[tauri::command]
pub fn set_menu_language(app: tauri::AppHandle, lang: String) -> Result<(), String> {
    let menu = build_app_menu(&app, &lang).map_err(|e| e.to_string())?;
    app.set_menu(menu).map_err(|e| e.to_string())?;
    Ok(())
}

/// Builds the application menu (app submenu, Window, Help) using labels for `lang`.
pub fn build_app_menu(
    handle: &tauri::AppHandle,
    lang: &str,
) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    let labels = menu_translations::labels_for(lang);

    // ── App submenu ──
    let about_icon = tauri::image::Image::from_bytes(APP_ICON).ok();
    let about = PredefinedMenuItem::about(
        handle,
        Some(labels.about),
        Some(AboutMetadata {
            name: Some(constants::APP_NAME.to_string()),
            version: Some(env!("CARGO_PKG_VERSION").into()),
            copyright: Some(constants::APP_LICENSE_ABOUT.into()),
            credits: Some(constants::APP_CREDITS.into()),
            website: Some(constants::RELEASE_NOTES_URL.to_string()),
            website_label: Some(labels.website_label.to_string()),
            icon: about_icon,
            ..Default::default()
        }),
    )?;
    let sep = PredefinedMenuItem::separator(handle)?;
    let check_for_updates = MenuItem::with_id(
        handle,
        constants::CHECK_FOR_UPDATES_MENU_ID,
        labels.check_for_updates,
        true,
        None::<&str>,
    )?;
    let sep_updates = PredefinedMenuItem::separator(handle)?;
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
            &check_for_updates,
            &sep_updates,
            &services,
            &sep2,
            &hide,
            &hide_others,
            &show_all,
            &sep3,
            &quit,
        ],
    )?;

    // ── Window submenu ──
    let minimize = MenuItem::with_id(handle, "minimize", labels.minimize, true, Some("cmd+m"))?;
    let close_window =
        MenuItem::with_id(handle, "close_window", labels.close_window, true, Some("cmd+w"))?;
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
    let read_license =
        MenuItem::with_id(handle, constants::LICENSE_MENU_ID, labels.license, true, None::<&str>)?;
    let help_submenu = Submenu::with_id_and_items(
        handle,
        tauri::menu::HELP_SUBMENU_ID,
        labels.help,
        true,
        &[&release_notes, &read_license],
    )?;

    Menu::with_items(handle, &[&app_submenu, &window_submenu, &help_submenu])
}
