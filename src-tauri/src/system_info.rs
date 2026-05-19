// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

use std::process::Command;

use serde::Serialize;

use crate::{disk::get_volume_name, log};

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

/// Runs a command and returns trimmed stdout, or `None` on failure.
fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn get_macos_version() -> String {
    run_command("sw_vers", &["-productVersion"]).unwrap_or_else(|| "Unknown".to_string())
}

fn get_hardware_model_from_json(json_str: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(json_str).ok()?;
    json.get("SPHardwareDataType")?
        .as_array()?
        .first()?
        .get("machine_model")?
        .as_str()
        .map(String::from)
}

fn get_hardware_model() -> String {
    if let Some(json_str) = run_command("system_profiler", &["SPHardwareDataType", "-json"]) {
        if let Some(model) = get_hardware_model_from_json(&json_str) {
            return model;
        }
    }
    get_hardware_model_ioreg()
}

/// Fallback method using ioreg to get hardware model
fn get_hardware_model_ioreg() -> String {
    run_command("ioreg", &["-c", "IOPlatformExpertDevice", "-d", "2"])
        .and_then(|output| {
            output.lines().find(|line| line.contains("model")).and_then(|line| {
                let start = line.find('"')? + 1;
                let end = start + line[start..].find('"')?;
                Some(line[start..end].to_string())
            })
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_cpu_info() -> String {
    run_command("sysctl", &["-n", "machdep.cpu.brand_string"])
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_memory_info() -> String {
    run_command("sysctl", &["-n", "hw.memsize"])
        .and_then(|s| s.parse::<u64>().ok())
        .map(|bytes| {
            let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            if gb >= 1024.0 {
                format!("{:.1} TB", gb / 1024.0)
            } else {
                format!("{:.0} GB", gb)
            }
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_system_disk_name() -> String {
    use std::path::Path;
    get_volume_name(Path::new("/"))
}

fn get_system_disk_size_from_json(json_str: &str) -> Option<String> {
    let data: serde_json::Value = serde_json::from_str(json_str).ok()?;
    data["SPStorageDataType"]
        .as_array()?
        .iter()
        .find(|s| s["mount_point"].as_str() == Some("/"))
        .and_then(|s| s["size_in_bytes"].as_u64())
        .map(|size| size.to_string())
}

fn get_system_disk_size() -> String {
    run_command("system_profiler", &["SPStorageDataType", "-json"])
        .and_then(|json_str| get_system_disk_size_from_json(&json_str))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_current_user() -> String {
    run_command("whoami", &[]).unwrap_or_else(|| "Unknown".to_string())
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    log::dev_rust_trace("app", "get_system_info");
    tauri::async_runtime::spawn_blocking(|| {
        // Parallelize the two slow system_profiler calls with rayon.
        // The other commands (sw_vers, sysctl, whoami) are fast (<10ms).
        let (
            (hardware_model, system_disk_size),
            (macos_version, cpu_info, memory_info, system_disk_name, current_user),
        ) = rayon::join(
            || rayon::join(get_hardware_model, get_system_disk_size),
            || {
                let macos_version = get_macos_version();
                let cpu_info = get_cpu_info();
                let memory_info = get_memory_info();
                let system_disk_name = get_system_disk_name();
                let current_user = get_current_user();
                (macos_version, cpu_info, memory_info, system_disk_name, current_user)
            },
        );

        Ok(SystemInfo {
            macos_version,
            hardware_model,
            cpu_info,
            memory_info,
            system_disk_name,
            system_disk_size,
            current_user,
        })
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
