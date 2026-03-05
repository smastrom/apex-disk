import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { check } from '@tauri-apps/plugin-updater'

export function useAppUpdate() {
   const newAvailableVersion = ref<string | null>(null)
   const isChecking = ref(false)

   let unlistenCheckUpdates: (() => void) | null = null

   onMounted(async () => {
      unlistenCheckUpdates = await listen('check-for-updates', onCheckForUpdates)

      // Silent check on startup
      try {
         const update = await check()
         newAvailableVersion.value = update?.version ?? null
      } catch (error) {
         // Silent — don't bother the user if the check fails
         console.warn('Error checking for updates:', error)
      }
   })

   onUnmounted(() => {
      unlistenCheckUpdates?.()
   })

   async function onCheckForUpdates() {
      let update: Awaited<ReturnType<typeof check>>

      try {
         isChecking.value = true
         update = await check()
      } catch {
         update = null
      } finally {
         isChecking.value = false
      }

      newAvailableVersion.value = update?.version ?? null
   }

   return { newAvailableVersion, isChecking, onCheckForUpdates }
}
