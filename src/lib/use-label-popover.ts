// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { computePosition, flip, offset, shift, type Placement } from '@floating-ui/dom'
import { ref, onUnmounted, type Ref } from 'vue'

const ENTER_DELAY = 400
const LEAVE_DELAY = 200
const VIEWPORT_PADDING = 16

/** Wrap the popover element in `<Teleport to="body">` so ancestor `overflow` does not clip it. */
export function useLabelPopover(
   triggerRef: Ref<HTMLElement | null>,
   popoverRef: Ref<HTMLElement | null>,
   options: {
      placement?: Placement
      offset?: number
      alwaysShow?: boolean
      /** Element used to compute popover position; defaults to triggerRef. Split when the
       * trigger's hit area is larger than the visual anchor (e.g. a row-tall checkbox button). */
      anchorRef?: Ref<HTMLElement | null>
   } = {}
) {
   const isOpen = ref(false)

   let enterTimer: ReturnType<typeof setTimeout> | null = null
   let leaveTimer: ReturnType<typeof setTimeout> | null = null
   let scrollCleanup: (() => void) | null = null

   const placement = options.placement ?? 'top'
   const offsetPx = options.offset ?? 8
   const alwaysShow = options.alwaysShow ?? false
   const anchorRef = options.anchorRef ?? triggerRef

   function shouldShow(): boolean {
      const el = triggerRef.value

      if (!el) return false
      if (alwaysShow) return true

      return el.scrollWidth > el.clientWidth
   }

   async function position() {
      const anchor = anchorRef.value
      const popover = popoverRef.value

      if (!anchor || !popover) return

      popover.style.maxWidth = `${window.innerWidth - VIEWPORT_PADDING * 2}px`

      const { x, y } = await computePosition(anchor, popover, {
         placement,
         middleware: [offset(offsetPx), flip(), shift({ padding: VIEWPORT_PADDING })],
      })

      popover.style.left = `${x}px`
      popover.style.top = `${y}px`
   }

   async function show() {
      const popover = popoverRef.value

      if (!popover || isOpen.value) return

      isOpen.value = true
      await position()
      popover.classList.add('is-open')
      addScrollListener()
   }

   function hide() {
      const popover = popoverRef.value

      if (!popover || !isOpen.value) return

      popover.classList.remove('is-open')
      isOpen.value = false
      removeScrollListener()
   }

   function dismiss() {
      clearTimers()
      hide()
   }

   function addScrollListener() {
      const el = triggerRef.value

      if (!el) return

      let ancestor: HTMLElement | null = el.parentElement

      while (ancestor) {
         const { overflow, overflowY } = getComputedStyle(ancestor)

         if (/auto|scroll/.test(overflow + overflowY)) break

         ancestor = ancestor.parentElement
      }

      const target = ancestor ?? document

      target.addEventListener('scroll', dismiss, { passive: true, once: true })
      scrollCleanup = () => target.removeEventListener('scroll', dismiss)
   }

   function removeScrollListener() {
      scrollCleanup?.()
      scrollCleanup = null
   }

   function clearTimers() {
      if (enterTimer) {
         clearTimeout(enterTimer)
         enterTimer = null
      }
      if (leaveTimer) {
         clearTimeout(leaveTimer)
         leaveTimer = null
      }
   }

   function onPointerEnter() {
      if (!shouldShow()) return

      if (leaveTimer) {
         clearTimeout(leaveTimer)
         leaveTimer = null
      }

      enterTimer = setTimeout(show, ENTER_DELAY)
   }

   function onPointerLeave() {
      if (enterTimer) {
         clearTimeout(enterTimer)
         enterTimer = null
      }
      if (isOpen.value) {
         leaveTimer = setTimeout(hide, LEAVE_DELAY)
      }
   }

   onUnmounted(() => {
      clearTimers()
      removeScrollListener()
   })

   return {
      isOpen,
      onPointerEnter,
      onPointerLeave,
   }
}
