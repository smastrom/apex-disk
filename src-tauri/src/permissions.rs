//! Full Disk Access (FDA) for macOS.

pub fn check_full_disk_access_sync() -> bool {
    // Try to list ~/Library/Safari/ which is TCC-protected and requires FDA.
    // This directory always exists on macOS and listing it does NOT trigger
    // a permission prompt — it simply returns an error without FDA.
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return false,
    };
    std::fs::read_dir(home.join("Library/Safari")).is_ok() // TODO: To be fixed
}
