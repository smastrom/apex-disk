// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/** Folder/file info from Tauri scan. Matches Rust struct field names (snake_case).
 *  `path` is not on the wire; `useScanner` hydrates it from `ScanResult.root` after IPC. */
export interface FolderInfo {
   name: string
   path: string
   size: number
   children: FolderInfo[]
   is_file: boolean
   is_protected: boolean
   is_fda_required: boolean
   last_modified?: number
   /** True when Rust dropped at least one file from this folder because it exceeded
    * the per-folder file cap, or at least one subfolder beyond the folder cap.
    * Always false for files. */
   truncated: boolean
   /** Files dropped by the per-folder file cap. Bytes still contribute to `size`.
    * Rust skips these on the wire when zero, so they arrive as `undefined`. */
   hidden_files_count?: number
   hidden_files_size?: number
   /** Subfolders dropped by the per-folder folder cap. Bytes still contribute to `size`. */
   hidden_folders_count?: number
   hidden_folders_size?: number
}

/** Wire shape of `get_user_folders`. `root` is the home dir; children paths
 *  are reconstructed in JS to keep the IPC payload small. */
export interface ScanResult {
   root: string
   folders: FolderInfo[]
}

/** Single item in the trash review list (flattened from selection). */
export interface TrashListItem {
   path: string
   name: string
   size: number
   is_file: boolean
}

/** Permission status for a single TCC-protected user folder. */
export interface FolderPermissionStatus {
   folder: string
   accessible: boolean
}

/** Scan progress event payload from Tauri. */
export interface ScanProgress {
   current: number
   total: number
   folder: string
   size: number
   scanned_size_total: number
   completed_size: number
   scanning?: string
}
