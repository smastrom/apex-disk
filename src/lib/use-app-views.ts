// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { ref } from 'vue'

import { log } from '@/lib/log'

const VIEW_ORDER = ['scan', 'settings', 'information'] as const

function viewIndex(view: string): number {
   const i = VIEW_ORDER.indexOf(view as (typeof VIEW_ORDER)[number])

   return i >= 0 ? i : 0
}

export function useAppViews() {
   const activeView = ref('scan')

   function setActiveView(view: string) {
      if (view === activeView.value) return

      log('view', `App: shell ${activeView.value} → ${view}`)

      const dir = viewIndex(view) > viewIndex(activeView.value) ? 1 : -1

      document.documentElement.style.setProperty('--nav-direction', String(dir))
      activeView.value = view
   }

   return { activeView, setActiveView }
}
