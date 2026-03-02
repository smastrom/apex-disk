import { ref, onMounted, onUnmounted } from 'vue'

/**
 * Reactive composable that tracks the user's `prefers-reduced-motion` preference.
 * Returns a ref that updates live when the system setting changes.
 */
export function useReducedMotion() {
   const prefersReducedMotion = ref(false)

   onMounted(() => {
      const mq = window.matchMedia('(prefers-reduced-motion: reduce)')
      prefersReducedMotion.value = mq.matches
      const onChange = (e: MediaQueryListEvent) => {
         prefersReducedMotion.value = e.matches
      }
      mq.addEventListener('change', onChange)
      onUnmounted(() => mq.removeEventListener('change', onChange))
   })

   return prefersReducedMotion
}
