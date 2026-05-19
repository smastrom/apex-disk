//! Tests for `system_info::get_system_info`.
//!
//! Runs the actual command against the host system (which is always macOS for
//! ApexDisk). Asserts that every field in `SystemInfo` is populated, either
//! with a real value from the underlying `sw_vers`/`system_profiler`/`sysctl`
//! call, or with the literal string `"Unknown"` when a lookup fails. Also
//! verifies that repeated calls return consistent results.

use apex_disk_lib::system_info::{get_system_info, SystemInfo};

#[tokio::test]
async fn get_system_info_returns_valid_structure() {
    let result: Result<SystemInfo, String> = get_system_info().await;

    match result {
        Ok(system_info) => {
            assert!(!system_info.macos_version.is_empty());
            assert!(!system_info.hardware_model.is_empty());
            assert!(!system_info.cpu_info.is_empty());
            assert!(!system_info.memory_info.is_empty());
            assert!(!system_info.system_disk_name.is_empty());
            assert!(!system_info.system_disk_size.is_empty());
            assert!(!system_info.current_user.is_empty());
        },
        Err(e) => panic!("System info command failed: {}", e),
    }
}

#[tokio::test]
async fn get_system_info_multiple_calls_are_consistent() {
    let info1 = get_system_info().await.expect("first call");
    let info2 = get_system_info().await.expect("second call");

    assert_eq!(info1.macos_version, info2.macos_version);
    assert_eq!(info1.hardware_model, info2.hardware_model);
    assert_eq!(info1.cpu_info, info2.cpu_info);
    assert_eq!(info1.memory_info, info2.memory_info);
    assert_eq!(info1.system_disk_name, info2.system_disk_name);
    assert_eq!(info1.system_disk_size, info2.system_disk_size);
    assert_eq!(info1.current_user, info2.current_user);
}
