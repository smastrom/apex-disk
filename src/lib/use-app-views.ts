// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { onScopeDispose, ref } from 'vue'

import { log } from '@/lib/log'
import { isWebDriverSession } from '@/lib/utils'

const VIEW_ORDER = ['scan', 'settings', 'information'] as const

type View = (typeof VIEW_ORDER)[number]
type ViewState = 'active' | 'leaving' | 'entering' | 'hidden'

const TRANSITION_HALF_MS = 120
const TRANSITION_TOTAL_MS = TRANSITION_HALF_MS * 2

interface UseAppViewsOptions {
   onEnter?: Record<string, () => void>
   onLeave?: Record<string, (nextView: string) => void>
}

function viewIndex(view: string): number {
   const i = VIEW_ORDER.indexOf(view as View)

   return i >= 0 ? i : 0
}

function initialStates(active: View): Record<string, ViewState> {
   const states: Record<string, ViewState> = {}

   for (const v of VIEW_ORDER) states[v] = v === active ? 'active' : 'hidden'

   return states
}

/**
 * Coordinates app view changes, exposes lifecycle callbacks, and drives the
 * per-view data-state machine that powers the sequenced slide animation
 * (leave → enter, total 240ms). Replaces the previous outer KeepAlive +
 * Transition wrap, which crashed when patches landed on cached subtrees
 * mid-removal.
 */
export function useAppViews(options: UseAppViewsOptions = {}) {
   const activeView = ref<View>('scan')
   const viewStates = ref<Record<string, ViewState>>(initialStates('scan'))

   let settleTimer: ReturnType<typeof setTimeout> | null = null

   function clearTimers() {
      if (settleTimer) {
         clearTimeout(settleTimer)

         settleTimer = null
      }
   }

   function setActiveView(view: string) {
      if (view === activeView.value) return
      if (!VIEW_ORDER.includes(view as View)) return

      const prev = activeView.value
      const next = view as View

      options.onLeave?.[prev]?.(next)

      log('view', `App: shell ${prev} → ${next}`)

      const dir = viewIndex(next) > viewIndex(prev) ? 1 : -1

      document.documentElement.style.setProperty('--nav-direction', String(dir))

      activeView.value = next

      options.onEnter?.[next]?.()

      clearTimers()

      if (isWebDriverSession) {
         viewStates.value = initialStates(next)

         return
      }

      const transitional: Record<string, ViewState> = {}

      for (const v of VIEW_ORDER) {
         transitional[v] = v === prev ? 'leaving' : v === next ? 'entering' : 'hidden'
      }

      viewStates.value = transitional

      settleTimer = setTimeout(() => {
         viewStates.value = initialStates(next)
         settleTimer = null
      }, TRANSITION_TOTAL_MS)
   }

   onScopeDispose(clearTimers)

   return { activeView, viewStates, setActiveView }
}
