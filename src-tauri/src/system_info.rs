use crate::disk::get_volume_name;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize, Debug)]
pub struct SystemInfo {
    pub macos_version: String,
    pub hardware_model: String,
    pub cpu_info: String,
    pub memory_info: String,
    pub system_disk_name: String,
    pub system_disk_size: String,
    pub current_user: String,
}

/// Get macOS version using sw_vers command
fn get_macos_version() -> String {
    match Command::new("sw_vers").arg("-productVersion").output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        },
        Err(_) => "Unknown".to_string(),
    }
}

/// Get hardware model using system_profiler
fn get_hardware_model() -> String {
    match Command::new("system_profiler")
        .args(&["SPHardwareDataType", "-json"])
        .output()
    {
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(json_str) => {
                    // Parse JSON to get model name
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(data) =
                            json.get("SPHardwareDataType").and_then(|v| v.as_array())
                        {
                            if let Some(first) = data.first() {
                                if let Some(model) =
                                    first.get("machine_model").and_then(|v| v.as_str())
                                {
                                    return model.to_string();
                                }
                            }
                        }
                    }

                    // Fallback to ioreg if system_profiler fails
                    get_hardware_model_ioreg()
                }
                Err(_) => get_hardware_model_ioreg(),
            }
        }
        Err(_) => get_hardware_model_ioreg(),
    }
}

/// Fallback method using ioreg to get hardware model
fn get_hardware_model_ioreg() -> String {
    match Command::new("ioreg")
        .args(&["-c", "IOPlatformExpertDevice", "-d", "2"])
        .output()
    {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(ioreg_output) => {
                for line in ioreg_output.lines() {
                    if line.contains("model") {
                        if let Some(start) = line.find('"') {
                            if let Some(end) = line[start + 1..].find('"') {
                                return line[start + 1..start + 1 + end].to_string();
                            }
                        }
                    }
                }
                "Unknown".to_string()
            }
            Err(_) => "Unknown".to_string(),
        },
        Err(_) => "Unknown".to_string(),
    }
}

/// Get CPU information using sysctl
fn get_cpu_info() -> String {
    match Command::new("sysctl")
        .args(&["-n", "machdep.cpu.brand_string"])
        .output()
    {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        },
        Err(_) => "Unknown".to_string(),
    }
}

/// Get memory information using sysctl
fn get_memory_info() -> String {
    match Command::new("sysctl").args(&["-n", "hw.memsize"]).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => {
                if let Ok(bytes) = s.trim().parse::<u64>() {
                    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    if gb >= 1024.0 {
                        format!("{:.1} TB", gb / 1024.0)
                    } else {
                        format!("{:.0} GB", gb)
                    }
                } else {
                    "Unknown".to_string()
                }
            }
            Err(_) => "Unknown".to_string(),
        },
        Err(_) => "Unknown".to_string(),
    }
}

/// Get system disk name using existing disk utility function
fn get_system_disk_name() -> String {
    use std::path::Path;
    get_volume_name(Path::new("/"))
}

/// Get system disk size using system_profiler (for total capacity)
fn get_system_disk_size() -> String {
    match Command::new("system_profiler")
        .args(&["SPStorageDataType", "-json"])
        .output()
    {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(json_str) => {
                // Parse JSON to find the startup disk and get its total capacity
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    if let Some(storage_data) = data["SPStorageDataType"].as_array() {
                        for storage in storage_data {
                            // Look for the volume that is mounted at "/" (root)
                            if let Some(mount_point) = storage["mount_point"].as_str() {
                                if mount_point == "/" {
                                    // Get the total size in bytes
                                    if let Some(size_bytes) = storage["size_in_bytes"].as_u64() {
                                        return size_bytes.to_string();
                                    }
                                }
                            }
                        }
                    }
                }
                "Unknown".to_string()
            }
            Err(_) => "Unknown".to_string(),
        },
        Err(_) => "Unknown".to_string(),
    }
}

/// Get current user name using whoami
fn get_current_user() -> String {
    match Command::new("whoami").output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        },
        Err(_) => "Unknown".to_string(),
    }
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let system_info = SystemInfo {
        macos_version: get_macos_version(),
        hardware_model: get_hardware_model(),
        cpu_info: get_cpu_info(),
        memory_info: get_memory_info(),
        system_disk_name: get_system_disk_name(),
        system_disk_size: get_system_disk_size(),
        current_user: get_current_user(),
    };

    Ok(system_info)
}
