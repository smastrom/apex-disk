#[cfg(test)]
mod tests {
    use apex_disk_lib::system_info::{get_system_info, SystemInfo};

    #[tokio::test]
    async fn test_get_system_info_returns_valid_structure() {
        // This test verifies that the system info command returns a valid structure
        // and doesn't panic. We don't test exact values since they vary by system.
        let result: Result<SystemInfo, String> = get_system_info().await;

        match result {
            Ok(system_info) => {
                // Verify all fields are present and non-empty (or "Unknown")
                assert!(!system_info.macos_version.is_empty());
                assert!(!system_info.hardware_model.is_empty());
                assert!(!system_info.cpu_info.is_empty());
                assert!(!system_info.memory_info.is_empty());
                assert!(!system_info.system_disk_name.is_empty());
                assert!(!system_info.system_disk_size.is_empty());
                assert!(!system_info.current_user.is_empty());

                // Verify that "Unknown" is used for failed lookups
                if system_info.macos_version == "Unknown" {
                    println!("Warning: macOS version detection failed");
                }
                if system_info.hardware_model == "Unknown" {
                    println!("Warning: hardware model detection failed");
                }
            }
            Err(e) => {
                panic!("System info command failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_system_info_multiple_calls() {
        // Verify that multiple calls work consistently
        let result1: Result<SystemInfo, String> = get_system_info().await;
        let result2: Result<SystemInfo, String> = get_system_info().await;

        assert!(result1.is_ok(), "First call failed: {:?}", result1);
        assert!(result2.is_ok(), "Second call failed: {:?}", result2);

        let info1 = result1.unwrap();
        let info2 = result2.unwrap();

        // Results should be consistent
        assert_eq!(info1.macos_version, info2.macos_version);
        assert_eq!(info1.hardware_model, info2.hardware_model);
        assert_eq!(info1.cpu_info, info2.cpu_info);
        assert_eq!(info1.memory_info, info2.memory_info);
        assert_eq!(info1.system_disk_name, info2.system_disk_name);
        assert_eq!(info1.system_disk_size, info2.system_disk_size);
        assert_eq!(info1.current_user, info2.current_user);
    }
}
