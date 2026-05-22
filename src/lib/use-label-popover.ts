// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import type { Placement } from '@floating-ui/dom'

import { ref, onUnmounted, type Ref } from 'vue'

import {
   attachScrollDismiss,
   clearPopoverTimer,
   isTextTruncated,
   POPOVER_DEFAULT_OFFSET,
   POPOVER_ENTER_DELAY,
   POPOVER_LEAVE_DELAY,
   positionPopover,
   type PopoverTimerRef,
} from '@/lib/popover-utils'

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

   let enterTimer: PopoverTimerRef = null
   let leaveTimer: PopoverTimerRef = null
   let scrollCleanup: (() => void) | null = null

   const placement = options.placement ?? 'top'
   const offsetPx = options.offset ?? POPOVER_DEFAULT_OFFSET
   const alwaysShow = options.alwaysShow ?? false
   const anchorRef = options.anchorRef ?? triggerRef

   function shouldShow(): boolean {
      const el = triggerRef.value

      if (!el) return false
      if (alwaysShow) return true

      return isTextTruncated(el)
   }

   async function position() {
      const anchor = anchorRef.value
      const popover = popoverRef.value

      if (!anchor || !popover) return

      await positionPopover(anchor, popover, { placement, offsetPx })
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

      scrollCleanup = attachScrollDismiss(el, dismiss)
   }

   function removeScrollListener() {
      scrollCleanup?.()
      scrollCleanup = null
   }

   function clearTimers() {
      enterTimer = clearPopoverTimer(enterTimer)
      leaveTimer = clearPopoverTimer(leaveTimer)
   }

   function onPointerEnter() {
      if (!shouldShow()) return

      leaveTimer = clearPopoverTimer(leaveTimer)

      enterTimer = setTimeout(show, POPOVER_ENTER_DELAY)
   }

   function onPointerLeave() {
      enterTimer = clearPopoverTimer(enterTimer)

      if (isOpen.value) {
         leaveTimer = setTimeout(hide, POPOVER_LEAVE_DELAY)
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
