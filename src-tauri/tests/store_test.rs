//! Tests for the settings store (defaults, merging, persistence) using a mock
//! Tauri app handle without runtime type mismatches.
//!
//! All tests acquire STORE_LOCK before running because the mock Tauri runtime
//! shares a single backing store, causing parallel tests to interfere.

use apex_disk_lib::constants;
use apex_disk_lib::store;

use serde_json::json;
use std::sync::Mutex;

use tauri_plugin_store::StoreExt;

/// Serialize store tests: the mock Tauri runtime shares a single backing store.
static STORE_LOCK: Mutex<()> = Mutex::new(());

/// Builds a mock Tauri app with the store plugin enabled.
fn create_app_with_store() -> tauri::App<tauri::test::MockRuntime> {
    let context = tauri::test::mock_context(tauri::test::noop_assets());

    tauri::test::mock_builder()
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(context)
        .expect("build mock app with store")
}

/// Default settings must match the constants used by the frontend.
#[test]
fn default_settings_match_constants() {
    let _lock = STORE_LOCK.lock().unwrap();
    let defaults = store::get_default_settings();

    assert_eq!(defaults["language"], json!(constants::DEFAULT_LANGUAGE));
    assert_eq!(defaults["themeColor"], json!(constants::DEFAULT_THEME));
    assert_eq!(defaults["showHiddenFiles"], json!(false));
    assert_eq!(defaults["showUnder1Kb"], json!(false));
    assert_eq!(defaults["showZeroByte"], json!(false));
    assert_eq!(defaults["autoUpdates"], json!(false));
}

/// initialize_store_with_handle must write defaults when the store is empty.
#[test]
fn initialize_store_writes_defaults_for_empty_store() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    // Clear the store to ensure it's empty
    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    let value = store_handle
        .get("app")
        .expect("settings value written by initialize_store");

    store_handle.close_resource();

    assert_eq!(value, store::get_default_settings());
}

/// get_settings_with_handle must merge existing settings with defaults, filling
/// in any missing keys while preserving existing values.
#[test]
fn get_settings_merges_existing_with_defaults() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    // Clear the store to ensure it's empty
    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let partial = json!({
        "language": constants::DEFAULT_LANGUAGE
    });

    store::set_settings_with_handle(&handle, partial).expect("set partial settings");

    let merged = store::get_settings_with_handle(&handle).expect("get merged settings");

    assert_eq!(merged["language"], json!(constants::DEFAULT_LANGUAGE));
    assert_eq!(merged["themeColor"], json!(constants::DEFAULT_THEME));
    assert_eq!(merged["showHiddenFiles"], json!(false));
    assert_eq!(merged["showUnder1Kb"], json!(false));
    assert_eq!(merged["showZeroByte"], json!(false));
    assert_eq!(merged["autoUpdates"], json!(false));
}

/// set_settings_with_handle and get_settings_with_handle must work together to
/// persist and retrieve settings values.
#[test]
fn set_and_get_settings_round_trip() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    // Clear the store to ensure it's empty
    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let custom = json!({
        "language": "it",
        "themeColor": constants::DEFAULT_THEME,
        "showHiddenFiles": true,
        "showUnder1Kb": true,
        "showZeroByte": true,
        "autoUpdates": false
    });

    store::set_settings_with_handle(&handle, custom.clone())
        .expect("write custom settings to store");

    let read_back = store::get_settings_with_handle(&handle).expect("read settings back");

    assert_eq!(read_back, custom);
}

/// update_setting_with_handle must update a single field without touching others.
#[test]
fn update_setting_preserves_other_fields() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    // Update just one field
    store::update_setting_with_handle(&handle, "language".to_string(), json!("it"))
        .expect("update language");

    let settings = store::get_settings_with_handle(&handle).expect("get settings");
    assert_eq!(
        settings["language"],
        json!("it"),
        "language should be updated"
    );
    assert_eq!(
        settings["themeColor"],
        json!(constants::DEFAULT_THEME),
        "themeColor should remain default"
    );
    assert_eq!(
        settings["showHiddenFiles"],
        json!(false),
        "showHiddenFiles should remain default"
    );
}

/// get_setting_with_handle must return a single field value.
#[test]
fn get_setting_returns_single_field() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let language = store::get_setting_with_handle(&handle, "language".to_string())
        .expect("get language setting");
    assert_eq!(language, Some(json!(constants::DEFAULT_LANGUAGE)));
}

/// get_setting_with_handle must return None for a non-existent key.
#[test]
fn get_setting_nonexistent_key_returns_none() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let result = store::get_setting_with_handle(&handle, "nonexistent".to_string())
        .expect("get nonexistent setting");
    assert_eq!(result, None);
}

/// set_settings_with_handle must reject non-object values.
#[test]
fn set_settings_rejects_non_object() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    let result = store::set_settings_with_handle(&handle, json!("not an object"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Settings must be an object");
}

/// update_setting_with_handle must reject unknown keys so typos or stale
/// frontend code can't silently write new fields into the persisted settings.
#[test]
fn update_setting_rejects_unknown_key() {
    let _lock = STORE_LOCK.lock().unwrap();
    let app = create_app_with_store();
    let handle = app.handle();

    let store_handle = handle
        .store(apex_disk_lib::SETTINGS_STORE_PATH)
        .expect("open settings store");
    store_handle.set("app", serde_json::json!({}));
    store_handle.save().expect("clear store");
    store_handle.close_resource();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let result =
        store::update_setting_with_handle(&handle, "bogusField".to_string(), json!("value"));
    assert!(result.is_err(), "unknown key must be rejected");
    let err = result.unwrap_err();
    assert!(
        err.contains("bogusField"),
        "error should mention the offending key, got: {err}"
    );

    // Persisted settings must be unchanged after a rejected update.
    let settings = store::get_settings_with_handle(&handle).expect("get settings");
    assert!(
        settings.get("bogusField").is_none(),
        "unknown key must not be persisted"
    );
}
