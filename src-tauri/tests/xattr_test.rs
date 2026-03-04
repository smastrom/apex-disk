//! Tests for `xattr` module.

use mac_disk_tree_lib::xattr;

#[test]
fn test_container_manager_detection() {
    // Test with a known container path if it exists
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };

    let containers_path = home.join("Library/Containers");
    if !containers_path.exists() {
        return;
    }

    // Read first container directory
    let entries = match std::fs::read_dir(&containers_path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            let path = entry.path();
            let has_attr = xattr::has_container_manager_attribute(&path);
            // Most containers should have this attribute
            println!("{}: {}", path.display(), has_attr);
            break;
        }
    }
}

#[test]
fn test_regular_folder_no_attribute() {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return,
    };

    // Desktop should not have container manager attributes
    let desktop = home.join("Desktop");
    if desktop.exists() {
        assert!(!xattr::has_container_manager_attribute(&desktop));
    }
}
