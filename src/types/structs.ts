// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/** Folder/file info from Tauri scan. Matches Rust struct field names (snake_case). */
export interface FolderInfo {
   name: string
   path: string
   size: number
   icon?: string
   children: FolderInfo[]
   is_file: boolean
   is_protected: boolean
   is_fda_required: boolean
   last_modified?: number
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
