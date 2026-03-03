import { invoke } from '@tauri-apps/api/core'

import type { DiskUsage } from '@/types/disk'

/** Loads current disk usage for the active user home volume. */
export async function getDiskUsage(): Promise<DiskUsage> {
   return invoke<DiskUsage>('get_disk_usage')
}
