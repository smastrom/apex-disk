import { ref, onMounted, onUnmounted } from 'vue'

/**
 * Reactive composable that tracks the user's `prefers-reduced-motion` preference.
 * Returns a ref that updates live when the system setting changes.
 */

export function useReducedMotion() {
   const query = window.matchMedia('(prefers-reduced-motion: reduce)')
   const prefersReducedMotion = ref(query.matches)

   const onChange = (event: MediaQueryListEvent) => {
      prefersReducedMotion.value = event.matches
   }

   onMounted(() => {
      query.addEventListener('change', onChange)
   })

   onUnmounted(() => {
      query.removeEventListener('change', onChange)
   })

   return { prefersReducedMotion }
}
