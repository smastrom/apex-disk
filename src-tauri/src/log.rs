/// Returns true when the `APEX_DISK_DEBUG` env var is set to "1" or "true".
/// Checked once at app startup so the frontend knows whether to enable logging.
#[tauri::command]
pub fn is_debug_mode() -> bool {
    std::env::var("APEX_DISK_DEBUG").map_or(false, |v| v == "1" || v == "true")
}

/// Prints a log line to stdout so users can follow activity from the terminal.
#[tauri::command]
pub fn log_message(message: String) {
    println!("{message}");
}
