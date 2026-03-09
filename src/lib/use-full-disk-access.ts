import { checkFullDiskAccessPermission } from 'tauri-plugin-macos-permissions-api'
import { ref } from 'vue'

/** Checks macOS Full Disk Access permission on mount and exposes the result as a reactive ref. */
export async function useFullDiskAccess() {
   const isFdaGranted = ref(false)

   try {
      isFdaGranted.value = await checkFullDiskAccessPermission()
   } catch (err) {
      console.error('[FDA] check failed:', err)
   }

   return { isFdaGranted }
}
