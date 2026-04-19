// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { getCurrentWindow } from '@tauri-apps/api/window'
import { ref, onMounted, onUnmounted, type Ref } from 'vue'

import type { DiskUsage } from '@/types/disk'

import { getDiskUsage } from './disk'
import { formatBytes } from './format'
import { log } from './log'
import { debounce } from './utils'

export interface UseDiskUsageReturn {
   diskUsage: Ref<DiskUsage | null>
}

export async function useDiskUsage(): Promise<UseDiskUsageReturn> {
   let unlisten = () => {}

   // Register lifecycle hooks
   onMounted(async () => {
      unlisten = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
         if (focused) debouncedFetchUsage()
      })
   })

   onUnmounted(() => {
      unlisten()
   })

   const diskUsage = ref<DiskUsage | null>(null)

   async function setDiskUsage() {
      try {
         const u = await getDiskUsage()
         diskUsage.value = u
         const used = Math.max(0, u.total - u.free)

         log(
            'disk',
            `Disk: usage — ${u.volume_name} total=${formatBytes(u.total)} free=${formatBytes(u.free)} used=${formatBytes(used)} user=${u.user_name}`,
            { home_path: u.home_path }
         )
      } catch (err) {
         console.error('Failed to get disk usage:', err)
         const msg = err instanceof Error ? err.message : String(err)
         log('disk', `Disk: fetch failed — ${msg}`)
      }
   }

   const debouncedFetchUsage = debounce(setDiskUsage, 200)

   // Initial fetch after hooks are registered
   await setDiskUsage()

   return {
      diskUsage,
   }
}
