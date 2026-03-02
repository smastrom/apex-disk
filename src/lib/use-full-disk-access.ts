import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

/** Checks macOS Full Disk Access permission on mount and exposes the result as a reactive ref. */
export function useFullDiskAccess() {
   const fdaGranted = ref(false)

   onMounted(async () => {
      try {
         fdaGranted.value = await invoke<boolean>('check_full_disk_access')
         console.log('[FDA] Result:', fdaGranted.value)
      } catch (err) {
         console.error('[FDA] invoke failed:', err)
      }
   })

   return { fdaGranted }
}
