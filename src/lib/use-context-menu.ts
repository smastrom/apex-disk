import { onMounted, onUnmounted } from 'vue'

/**
 * Disables the native right-click context menu in production builds.
 * No-ops in development so DevTools inspection works normally.
 */
export function disableNativeContextMenu() {
   if (import.meta.env.DEV) return

   function preventDefault(event: Event) {
      event.preventDefault()
   }

   onMounted(() => {
      document.addEventListener('contextmenu', preventDefault)
   })

   onUnmounted(() => {
      document.removeEventListener('contextmenu', preventDefault)
   })
}
