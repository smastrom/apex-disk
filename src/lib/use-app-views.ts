import { ref, type ShallowRef } from 'vue'

import { useViewTransition } from '@/lib/use-view-transition'

const VIEW_ORDER = ['scan', 'settings', 'information'] as const

function viewIndex(view: string): number {
   const i = VIEW_ORDER.indexOf(view as (typeof VIEW_ORDER)[number])

   return i >= 0 ? i : 0
}

export function useAppViews(mainContentRef: Readonly<ShallowRef<HTMLElement | null>>) {
   const { withTransition } = useViewTransition()

   const activeView = ref('scan')

   async function setActiveView(view: string) {
      if (view === activeView.value) return

      const el = mainContentRef.value
      const dir = viewIndex(view) > viewIndex(activeView.value) ? 1 : -1

      document.documentElement.style.setProperty('--nav-direction', String(dir))
      el?.style.setProperty('view-transition-name', 'app-view')
      await withTransition(async () => {
         activeView.value = view
      })
      el?.style.removeProperty('view-transition-name')
   }

   return { activeView, setActiveView }
}
