import { APP_VERSION, LATEST_RELEASE_URL } from '@/lib/constants'
import { ref } from 'vue'

export function useAppUpdate() {
   const newAvailableVersion = ref<string | null>(null)
   const isChecking = ref(false)

   async function onCheckForUpdates() {
      try {
         isChecking.value = true

         const res = await fetch(LATEST_RELEASE_URL)
         if (!res.ok) return

         const data = await res.json()
         const latestTag: string = data.tag_name ?? ''
         const latestVersion = latestTag.replace(/^v/, '')

         if (latestVersion && latestVersion !== APP_VERSION) {
            newAvailableVersion.value = latestVersion
         } else {
            newAvailableVersion.value = null
         }
      } catch (error) {
         console.error('Error checking for updates:', error)
      } finally {
         isChecking.value = false
      }
   }

   return { newAvailableVersion, isChecking, onCheckForUpdates }
}
