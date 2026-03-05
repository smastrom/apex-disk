//! Centralized store management for app settings and persistence.
//!
//! This module handles all store operations including initialization, defaults,
//! migration, and provides a clean API for other modules to interact with
//! persistent settings.

use tauri::AppHandle;
use tauri::Runtime;

use serde_json::json;

use tauri_plugin_store::StoreExt;

use crate::constants;

/// Default settings for the application.
/// This is the single source of truth for all default values.
pub fn get_default_settings() -> serde_json::Value {
    json!({
        "language": constants::DEFAULT_LANGUAGE,
        "themeColor": constants::DEFAULT_THEME,
        "permanentlyDelete": false,
        "showHiddenFiles": false,
        "showUnder1Kb": false,
        "showZeroByte": false,
    })
}

/// Initializes the store with proper defaults if needed for any runtime.
pub fn initialize_store_with_handle<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), String> {
    let store = app
        .store(crate::SETTINGS_STORE_PATH)
        .map_err(|e| e.to_string())?;

    // Initialize with defaults if store is empty or corrupted
    let current = store.get("app").unwrap_or_else(|| json!({}));

    if !current.is_object() {
        let defaults = get_default_settings();
        store.set("app", defaults);
        store.save().map_err(|e| e.to_string())?;
    }

    // Close resource as per best practices
    store.close_resource();
    Ok(())
}

/// Initializes the store with proper defaults if needed.
pub fn initialize_store(app: &AppHandle) -> Result<(), String> {
    initialize_store_with_handle(app)
}

/// Gets current settings from the store, applying defaults for missing fields for any runtime.
pub fn get_settings_with_handle<R: Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<serde_json::Value, String> {
    let store = app
        .store(crate::SETTINGS_STORE_PATH)
        .map_err(|e| e.to_string())?;

    let current = store.get("app").unwrap_or_else(|| get_default_settings());

    // Ensure all required fields exist with defaults
    let defaults = get_default_settings();
    let merged = if current.is_object() {
        let mut merged = current.clone();
        if let (Some(current_obj), Some(defaults_obj)) =
            (merged.as_object_mut(), defaults.as_object())
        {
            for (key, default_value) in defaults_obj {
                if !current_obj.contains_key(key) {
                    current_obj.insert(key.clone(), default_value.clone());
                }
            }
        }
        merged
    } else {
        defaults
    };

    // Close resource as per best practices
    store.close_resource();

    Ok(merged)
}

/// Gets current settings from the store, applying defaults for missing fields.
#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<serde_json::Value, String> {
    get_settings_with_handle(&app)
}

/// Updates settings in the store for any runtime.
pub fn set_settings_with_handle<R: Runtime>(
    app: &tauri::AppHandle<R>,
    settings: serde_json::Value,
) -> Result<(), String> {
    let store = app
        .store(crate::SETTINGS_STORE_PATH)
        .map_err(|e| e.to_string())?;

    if !settings.is_object() {
        return Err("Settings must be an object".to_string());
    }

    store.set("app", settings);
    store.save().map_err(|e| e.to_string())?;
    store.close_resource();

    Ok(())
}

/// Updates settings in the store.
#[tauri::command]
pub fn set_settings(app: AppHandle, settings: serde_json::Value) -> Result<(), String> {
    set_settings_with_handle(&app, settings)
}

/// Updates a specific setting field for any runtime.
pub fn update_setting_with_handle<R: Runtime>(
    app: &tauri::AppHandle<R>,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let store = app
        .store(crate::SETTINGS_STORE_PATH)
        .map_err(|e| e.to_string())?;

    let mut settings = store.get("app").unwrap_or_else(|| get_default_settings());

    if let Some(obj) = settings.as_object_mut() {
        obj.insert(key, value);
    } else {
        return Err("Settings is not an object".to_string());
    }

    store.set("app", settings);
    store.save().map_err(|e| e.to_string())?;
    store.close_resource();

    Ok(())
}

/// Updates a specific setting field.
#[tauri::command]
pub fn update_setting(app: AppHandle, key: String, value: serde_json::Value) -> Result<(), String> {
    update_setting_with_handle(&app, key, value)
}

/// Gets a specific setting field for any runtime.
pub fn get_setting_with_handle<R: Runtime>(
    app: &tauri::AppHandle<R>,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    let store = app
        .store(crate::SETTINGS_STORE_PATH)
        .map_err(|e| e.to_string())?;

    let settings = store.get("app").unwrap_or_else(|| get_default_settings());

    let result = if let Some(obj) = settings.as_object() {
        obj.get(&key).cloned()
    } else {
        None
    };

    // Close resource as per best practices
    store.close_resource();

    Ok(result)
}

/// Gets a specific setting field.
#[tauri::command]
pub fn get_setting(app: AppHandle, key: String) -> Result<Option<serde_json::Value>, String> {
    get_setting_with_handle(&app, key)
}
