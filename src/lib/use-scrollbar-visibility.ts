// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { onActivated, onBeforeUnmount, onDeactivated, onMounted, type Ref } from 'vue'

type Mode = 'scroll-and-hover' | 'hover-only'

interface Options {
   hideDelayMs?: number
   gutterPx?: number
}

const HIDE_DELAY_DEFAULT_MS = 1000
const GUTTER_DEFAULT_PX = 10

/**
 * Toggles `data-scrollbar-active="true"` on the scroll element so the CSS
 * in global.css can paint the webkit thumb. The element starts transparent
 * and only becomes visible while this attribute is set.
 *
 * scroll-and-hover: appears during scrolling and while the cursor sits over
 * the gutter band; hides `hideDelayMs` after the last scroll unless the
 * gutter is still hovered.
 *
 * hover-only: appears only while the cursor sits over the gutter band.
 */
export function useScrollbarVisibility(
   elementRef: Readonly<Ref<HTMLElement | null>>,
   mode: Mode,
   options: Options = {}
) {
   const hideDelayMs = options.hideDelayMs ?? HIDE_DELAY_DEFAULT_MS
   const gutterPx = options.gutterPx ?? GUTTER_DEFAULT_PX

   let hideTimer: ReturnType<typeof setTimeout> | null = null
   let isOverGutter = false
   let isActive = false

   function setActive(active: boolean) {
      const el = elementRef.value

      if (!el) return

      if (active === isActive) return

      isActive = active

      if (active) {
         el.setAttribute('data-scrollbar-active', 'true')
      } else {
         el.removeAttribute('data-scrollbar-active')
      }
   }

   function clearHideTimer() {
      if (hideTimer !== null) {
         clearTimeout(hideTimer)

         hideTimer = null
      }
   }

   function scheduleHide() {
      clearHideTimer()

      hideTimer = setTimeout(() => {
         hideTimer = null

         if (!isOverGutter) setActive(false)
      }, hideDelayMs)
   }

   function onScroll() {
      if (mode !== 'scroll-and-hover') return

      setActive(true)
      scheduleHide()
   }

   function onMouseMove(e: MouseEvent) {
      const el = elementRef.value

      if (!el) return

      const rect = el.getBoundingClientRect()
      const isRtl = getComputedStyle(el).direction === 'rtl'
      const distFromEdge = isRtl ? e.clientX - rect.left : rect.right - e.clientX
      const wasOverGutter = isOverGutter

      isOverGutter = distFromEdge >= 0 && distFromEdge <= gutterPx

      if (isOverGutter && !wasOverGutter) {
         clearHideTimer()
         setActive(true)
      } else if (!isOverGutter && wasOverGutter) {
         if (mode === 'hover-only') setActive(false)
         else scheduleHide()
      }
   }

   function onMouseLeave() {
      isOverGutter = false

      if (mode === 'hover-only') setActive(false)
      else scheduleHide()
   }

   function reset() {
      clearHideTimer()

      isOverGutter = false

      setActive(false)
   }

   onMounted(() => {
      const el = elementRef.value

      if (!el) return

      el.addEventListener('scroll', onScroll, { passive: true })
      el.addEventListener('mousemove', onMouseMove)
      el.addEventListener('mouseleave', onMouseLeave)
   })

   onBeforeUnmount(() => {
      const el = elementRef.value

      if (el) {
         el.removeEventListener('scroll', onScroll)
         el.removeEventListener('mousemove', onMouseMove)
         el.removeEventListener('mouseleave', onMouseLeave)
      }

      clearHideTimer()
   })

   onActivated(reset)
   onDeactivated(reset)

   return { reset }
}
