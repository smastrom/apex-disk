import { createApp } from 'vue'

import App from './components/App.vue'

/** Show focus ring only after keyboard navigation; remove the hint on pointer so click focus has no ring. */
const FOCUS_RING_CLASS = 'focus-ring-keyboard'

function onKeyDown() {
   document.documentElement.classList.add(FOCUS_RING_CLASS)
}

function onPointerDown() {
   document.documentElement.classList.remove(FOCUS_RING_CLASS)
}

document.addEventListener('keydown', onKeyDown, true)
document.addEventListener('mousedown', onPointerDown, true)
document.addEventListener('touchstart', onPointerDown, true)

createApp(App).mount('#app')
