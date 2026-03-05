import { ref, onMounted, onUnmounted, watch, type Ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

import { getDiskUsage } from './disk'
import { debounce } from './utils'

import type { DiskUsage } from '@/types/disk'

export interface UseDiskUsageOptions {
   refreshKey?: Ref<number | undefined>
}

export interface UseDiskUsageReturn {
   usage: Ref<DiskUsage | null>
}

export async function useDiskUsage(options: UseDiskUsageOptions = {}): Promise<UseDiskUsageReturn> {
   const { refreshKey } = options

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

   const usage = ref<DiskUsage | null>(null)

   async function setDiskUsage() {
      try {
         usage.value = await getDiskUsage()
      } catch (err) {
         console.error('Failed to get disk usage:', err)
      }
   }

   const debouncedFetchUsage = debounce(setDiskUsage, 200)

   // Initial fetch after hooks are registered
   await setDiskUsage()

   // Watch refreshKey changes to trigger refresh
   if (refreshKey) {
      watch(
         () => refreshKey.value,
         () => {
            if (refreshKey.value !== undefined) {
               setDiskUsage()
            }
         }
      )
   }

   return {
      usage,
   }
}
