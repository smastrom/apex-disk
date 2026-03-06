import { ref, onMounted, onUnmounted, type Ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

import { getDiskUsage } from './disk'
import { debounce } from './utils'

import type { DiskUsage } from '@/types/disk'

export interface UseDiskUsageReturn {
   diskUsage: Ref<DiskUsage | null>
}

export async function useDiskUsage(): Promise<UseDiskUsageReturn> {
   let unlisten = () => {}

   // Register lifecycle hooks before any await
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
         diskUsage.value = await getDiskUsage()
      } catch (err) {
         console.error('Failed to get disk usage:', err)
      }
   }

   const debouncedFetchUsage = debounce(setDiskUsage, 200)

   // Initial fetch after hooks are registered
   await setDiskUsage()

   return {
      diskUsage,
   }
}
