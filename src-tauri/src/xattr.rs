// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

//! Extended attributes detection for macOS.
//!
//! Detects folders with `com.apple.containermanager.*` attributes that require
//! Full Disk Access (FDA) to delete. These are sandboxed app containers managed
//! by the system (e.g., ~/Library/Containers/com.docker.docker).

use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path, sync::LazyLock};

/// Fixed attribute name probed by [`has_container_manager_attribute`]; built once.
static CONTAINER_MANAGER_ATTR: LazyLock<CString> =
    LazyLock::new(|| CString::new("com.apple.containermanager.identifier").unwrap());

/// Returns true if the path has `com.apple.containermanager.identifier` extended attribute.
/// This attribute indicates a system-managed container that requires FDA to delete.
pub fn has_container_manager_attribute(path: &Path) -> bool {
    let path_cstr = match CString::new(path.as_os_str().as_bytes()) {
        Ok(c) => c,
        Err(_) => return false,
    };

    // SAFETY: getxattr is called with valid C strings and a zero-sized value buffer.
    // It returns the attribute size (>0) when present, or -1 otherwise.
    unsafe {
        let size = libc::getxattr(
            path_cstr.as_ptr(),
            CONTAINER_MANAGER_ATTR.as_ptr(),
            std::ptr::null_mut(),
            0,
            0,
            0,
        );
        size > 0
    }
}
