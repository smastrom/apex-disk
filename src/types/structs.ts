/** Folder/file info from Tauri scan. Matches Rust struct field names (snake_case). */
export interface FolderInfo {
   name: string
   path: string
   size: number
   icon?: string
   children: FolderInfo[]
   is_file: boolean
   is_protected: boolean
}

/** Single item in the delete review list (flattened from selection). */
export interface DeleteListItem {
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
   scanning?: string
}
