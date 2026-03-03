/** Disk usage payload returned by the Tauri backend. */
export interface DiskUsage {
   total: number
   free: number
   volume_name: string
   user_name: string
   home_path: string
}
