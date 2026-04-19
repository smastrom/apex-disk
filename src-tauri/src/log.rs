//! Diagnostic logging shared with the Vue frontend (`src/lib/log.ts`).
//!
//! Scheme, streams, and channel names: see **`docs/LOGGING.md`**.
//!
//! - [`dev_rust_trace`] — stdout `[apex:rust:{channel}]` for IPC entry traces
//! - [`log_message`] / [`log_error_message`] — Tauri commands used by TS
//! - [`format_bytes_si`] — human-readable sizes for diagnostic lines (decimal SI, matches Vue `formatBytes`)

/// Human-readable byte string using **decimal (SI) units** (1 KB = 1000 B), matching `formatBytes` in `src/lib/format.ts`.
pub(crate) fn format_bytes_si(n: u64) -> String {
    if n == 0 {
        return "0 B".to_string();
    }

    const UNIT: f64 = 1000.0;
    let nf = n as f64;
    let idx = (nf.log10() / UNIT.log10()).floor() as usize;
    let idx = idx.min(4);
    let labels = ["B", "KB", "MB", "GB", "TB"];
    let divisor = UNIT.powi(idx as i32);
    let value = nf / divisor;

    format!("{value:.2} {}", labels[idx])
}

/// Returns true when the user launched with `APEX_DISK_DEBUG=1` or `true` (bug-report /
/// support diagnostics). Shared by the frontend logger and optional backend `println!`s.
pub(crate) fn is_apex_disk_debug() -> bool {
    std::env::var("APEX_DISK_DEBUG").map_or(false, |v| v == "1" || v == "true")
}

/// UTC time-of-day prefix matching the Vue logger (`HH:MM:SS.mmm`, no `chrono` dep).
pub(crate) fn format_diag_utc_time() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs_today = dur.as_secs() % 86400;
    let h = secs_today / 3600;
    let m = (secs_today % 3600) / 60;
    let s = secs_today % 60;
    let ms = dur.subsec_millis();
    format!("{h:02}:{m:02}:{s:02}.{ms:03}")
}

/// Entry-point traces for IPC handlers. **Debug builds** (`tauri dev`) or
/// **`APEX_DISK_DEBUG=1`** release — same rule as updater diagnostics.
pub(crate) fn dev_rust_trace(channel: &str, message: &str) {
    if !cfg!(debug_assertions) && !is_apex_disk_debug() {
        return;
    }

    println!(
        "[{}] [apex:rust:{channel}] {message}",
        format_diag_utc_time()
    );
}

/// Exposes [`is_apex_disk_debug`] to the frontend so `initLog()` can enable production logging.
#[tauri::command]
pub fn is_debug_mode() -> bool {
    is_apex_disk_debug()
}

/// Prints a diagnostic line to **stdout**. The message should already include an
/// `[apex:…]` prefix from the frontend (`[apex:vue:…]`).
#[tauri::command]
pub fn log_message(message: String) {
    println!("{message}");
}

/// Prints a diagnostic line to **stderr** (uncaught web / Vue errors mirrored from TS).
#[tauri::command]
pub fn log_error_message(message: String) {
    eprintln!("{message}");
}

#[cfg(test)]
mod format_bytes_si_tests {
    use super::format_bytes_si;

    #[test]
    fn zero_is_zero_b() {
        assert_eq!(format_bytes_si(0), "0 B");
    }

    #[test]
    fn uses_decimal_si_like_frontend() {
        assert_eq!(format_bytes_si(500), "500.00 B");
        assert_eq!(format_bytes_si(1_500), "1.50 KB");
        assert_eq!(format_bytes_si(1_500_000), "1.50 MB");
    }
}
