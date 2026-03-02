import { onMounted, onUnmounted } from 'vue'

const FOCUS_RING_CLASS = 'focus-ring-keyboard'

function onKeyDown() {
   document.documentElement.classList.add(FOCUS_RING_CLASS)
}

function onPointerDown() {
   document.documentElement.classList.remove(FOCUS_RING_CLASS)
}

/**
 * Shows a CSS focus ring only after keyboard navigation. Adds `focus-ring-keyboard` to
 * `<html>` on keydown and removes it on pointer down, so click-focused elements stay ring-free.
 * Call once in the root component (App.vue).
 */
export function useFocusRing() {
   onMounted(() => {
      document.addEventListener('keydown', onKeyDown, true)
      document.addEventListener('mousedown', onPointerDown, true)
      document.addEventListener('touchstart', onPointerDown, true)
   })

   onUnmounted(() => {
      document.removeEventListener('keydown', onKeyDown, true)
      document.removeEventListener('mousedown', onPointerDown, true)
      document.removeEventListener('touchstart', onPointerDown, true)
   })
}
