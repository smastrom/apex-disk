import { nextTick } from 'vue'

import { inject, type Ref } from 'vue'

import { SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'

const VIEW_TRANSITION_SUPPORTED =
   typeof document !== 'undefined' && 'startViewTransition' in document

function prefersReducedMotion(): boolean {
   if (typeof window === 'undefined' || !window.matchMedia) return false
   return window.matchMedia('(prefers-reduced-motion: reduce)').matches
}

/**
 * Wraps a DOM update in the View Transitions API when animations are enabled.
 * Falls back to immediate update when disabled or unsupported.
 * @param storeRef - Optional ref to the settings store; when provided (e.g. by the providing component), inject is skipped.
 */
export function useViewTransition(storeRef?: Ref<SettingsStore | null>) {
   const resolved = storeRef ?? inject<Ref<SettingsStore | null>>(SETTINGS_KEY)

   async function withTransition(update: () => void | Promise<void>): Promise<void> {
      const enabled = resolved?.value?.settings?.value?.enableAnimations ?? true
      const useNative = VIEW_TRANSITION_SUPPORTED && enabled && !prefersReducedMotion()

      if (useNative) {
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
