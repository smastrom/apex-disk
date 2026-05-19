// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! App language detection, persistence, and macOS locale sync.
//!
//! On first launch, detects the system language and persists it as the app
//! language. On later launches, reads the stored preference. Setting the
//! language also updates the macOS `AppleLanguages` default (so native
//! context menus like Look Up / Translate follow the app) and rebuilds the
//! menu bar via `menu::set_menu_language`.

#[cfg(target_os = "macos")]
use objc2_foundation::{NSArray, NSString, NSUserDefaults};
use serde_json::json;
use sys_locale::get_locale;
use tauri_plugin_store::StoreExt;

use crate::{APP_LANGUAGE_INITIALIZED_KEY, SETTINGS_STORE_PATH};

fn to_supported_language(language: &str) -> String {
    match language {
        "en" | "it" | "es" | "fr" | "pt" | "de" | "ru" | "zh" | "ja" | "ar" => language.to_string(),
        _ => "en".to_string(),
    }
}

/// Gets the system language and maps it to app language codes.
/// Returns the language code that matches our supported languages.
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn get_system_language() -> String {
    match get_locale() {
        Some(locale) => {
            // Extract primary language from locale (e.g., "it-IT" -> "it")
            let primary = locale.split(['-', '_']).next().unwrap_or(&locale).to_lowercase();

            to_supported_language(primary.as_str())
        },
        None => "en".to_string(), // fallback to English
    }
}

/// Resolves the app language during setup or via command.
#[tauri::command]
pub fn resolve_app_language(app: tauri::AppHandle) -> Result<String, String> {
    let store = app.store(SETTINGS_STORE_PATH).map_err(|e| e.to_string())?;

    let initialized =
        store.get(APP_LANGUAGE_INITIALIZED_KEY).and_then(|value| value.as_bool()).unwrap_or(false);

    let current_settings = store.get("app").unwrap_or_else(|| json!({}));
    let stored_language = current_settings
        .as_object()
        .and_then(|obj| obj.get("language"))
        .and_then(|value| value.as_str())
        .map(to_supported_language);

    let resolved_language = if initialized {
        stored_language.unwrap_or_else(|| "en".to_string())
    } else {
        let detected = get_system_language();
        let mut next_settings =
            if current_settings.is_object() { current_settings } else { json!({}) };

        if let Some(obj) = next_settings.as_object_mut() {
            obj.insert("language".to_string(), json!(&detected));
        }

        store.set("app", next_settings);
        store.set(APP_LANGUAGE_INITIALIZED_KEY, json!(true));
        store.save().map_err(|e| e.to_string())?;

        detected
    };

    // Close resource as per best practices
    store.close_resource();

    set_app_locale(app.clone(), resolved_language.clone())?;

    Ok(resolved_language)
}

/// Sets the application locale to match the selected language.
/// This affects macOS context menus (Look Up, Translate, etc.).
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn set_app_locale(app: tauri::AppHandle, language: String) -> Result<(), String> {
    // Map app language codes to macOS locale identifiers
    let locale_id = match language.as_str() {
        "it" => "it_IT",
        "es" => "es_ES",
        "fr" => "fr_FR",
        "pt" => "pt_PT",
        "de" => "de_DE",
        "ru" => "ru_RU",
        "zh" => "zh_CN",
        "ja" => "ja_JP",
        "ar" => "ar_AE",
        _ => "en_US", // fallback to English
    };

    // Get the current NSUserDefaults
    let defaults = NSUserDefaults::standardUserDefaults();

    // Create NSString for the locale
    let locale_key = NSString::from_str("AppleLanguages");
    let locale_ns = NSString::from_str(locale_id);
    let locale_value = NSArray::from_slice(&[&*locale_ns]);

    // Set the locale preference
    unsafe {
        defaults.setObject_forKey(Some(&locale_value), &locale_key);
        defaults.synchronize();
    }

    // Update menu to match the new app language
    crate::menu::set_menu_language(app, language)?;

    Ok(())
}

/// No-op for non-macOS platforms.
#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn set_app_locale(_app: tauri::AppHandle, _language: String) -> Result<(), String> {
    Ok(())
}

/// No-op for getting system language on non-macOS platforms (uses frontend fallback).
#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn get_system_language() -> String {
    "en".to_string()
}
