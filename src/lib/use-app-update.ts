import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { check } from '@tauri-apps/plugin-updater'

export function useAppUpdate({
   checkOnMount = true,
   listenToUpdates = true,
}: { checkOnMount?: boolean; listenToUpdates?: boolean } = {}) {
   const newAvailableVersion = ref<string | null>(null)
   const isChecking = ref(false)

   let unlistenCheckUpdates: (() => void) | null = null

   onMounted(async () => {
      if (listenToUpdates) {
         unlistenCheckUpdates = await listen('check-for-updates', onCheckForUpdates)
      }

      if (checkOnMount) {
         onCheckForUpdates()
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
      } catch (error) {
         console.error('Error checking for updates:', error)
         update = null
      } finally {
         isChecking.value = false
      }

      newAvailableVersion.value = update?.version ?? null
   }

   return { newAvailableVersion, isChecking, onCheckForUpdates }
}
