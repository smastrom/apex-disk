import { nextTick } from 'vue'

const VIEW_TRANSITION_SUPPORTED =
   typeof document !== 'undefined' && 'startViewTransition' in document

/**
 * Wraps a DOM update in the View Transitions API when supported.
 * Falls back to immediate update when unsupported.
 * Reduced-motion users still get view transitions — CSS media queries
 * swap the slide keyframes for opacity-only fades.
 */
export function useViewTransition() {
   async function withTransition(update: () => void | Promise<void>): Promise<void> {
      if (VIEW_TRANSITION_SUPPORTED) {
         const transition = document.startViewTransition(async () => {
            await update()
            await nextTick()
         })
         await transition.finished
      } else {
         await update()
      }
   }

   return { withTransition }
}
