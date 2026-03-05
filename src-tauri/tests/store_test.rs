//! Tests for the settings store (defaults, merging, persistence) using a mock
//! Tauri app handle without runtime type mismatches.

use mac_disk_tree_lib::constants;
use mac_disk_tree_lib::store;

use serde_json::json;

use tauri_plugin_store::StoreExt;

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
    let defaults = store::get_default_settings();

    assert_eq!(defaults["language"], json!(constants::DEFAULT_LANGUAGE));
    assert_eq!(defaults["themeColor"], json!(constants::DEFAULT_THEME));
    assert_eq!(defaults["permanentlyDelete"], json!(false));
    assert_eq!(defaults["showHiddenFiles"], json!(false));
    assert_eq!(defaults["showUnder1Kb"], json!(false));
    assert_eq!(defaults["showZeroByte"], json!(false));
}

/// initialize_store_with_handle must write defaults when the store is empty.
#[test]
fn initialize_store_writes_defaults_for_empty_store() {
    let app = create_app_with_store();
    let handle = app.handle();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let store_handle = handle
        .store(mac_disk_tree_lib::SETTINGS_STORE_PATH)
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
    let app = create_app_with_store();
    let handle = app.handle();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let partial = json!({
        "language": constants::DEFAULT_LANGUAGE,
        "permanentlyDelete": true
    });

    store::set_settings_with_handle(&handle, partial).expect("set partial settings");

    let merged = store::get_settings_with_handle(&handle).expect("get merged settings");

    assert_eq!(merged["language"], json!(constants::DEFAULT_LANGUAGE));
    assert_eq!(merged["permanentlyDelete"], json!(true));
    assert_eq!(merged["themeColor"], json!(constants::DEFAULT_THEME));
    assert_eq!(merged["showHiddenFiles"], json!(false));
    assert_eq!(merged["showUnder1Kb"], json!(false));
    assert_eq!(merged["showZeroByte"], json!(false));
}

/// set_settings_with_handle and get_settings_with_handle must work together to
/// persist and retrieve settings values.
#[test]
fn set_and_get_settings_round_trip() {
    let app = create_app_with_store();
    let handle = app.handle();

    store::initialize_store_with_handle(&handle).expect("initialize store");

    let custom = json!({
        "language": "it",
        "themeColor": constants::DEFAULT_THEME,
        "permanentlyDelete": true,
        "showHiddenFiles": true,
        "showUnder1Kb": true,
        "showZeroByte": true
    });

    store::set_settings_with_handle(&handle, custom.clone())
        .expect("write custom settings to store");

    let read_back = store::get_settings_with_handle(&handle).expect("read settings back");

    assert_eq!(read_back, custom);
}
