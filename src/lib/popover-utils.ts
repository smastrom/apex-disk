// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { computePosition, flip, offset, shift, type Placement } from '@floating-ui/dom'

export const POPOVER_ENTER_DELAY = 400
export const POPOVER_LEAVE_DELAY = 200
export const POPOVER_VIEWPORT_PADDING = 16
export const POPOVER_DEFAULT_OFFSET = 8

export type PopoverTimerRef = ReturnType<typeof setTimeout> | null

export function isTextTruncated(el: HTMLElement): boolean {
   return el.scrollWidth > el.clientWidth
}

export function findScrollableAncestor(el: HTMLElement): HTMLElement | Document {
   let ancestor: HTMLElement | null = el.parentElement

   while (ancestor) {
      const { overflow, overflowY } = getComputedStyle(ancestor)

      if (/auto|scroll/.test(overflow + overflowY)) break

      ancestor = ancestor.parentElement
   }

   return ancestor ?? document
}

export async function positionPopover(
   anchor: HTMLElement,
   popover: HTMLElement,
   options: { placement: Placement; offsetPx?: number }
): Promise<void> {
   const offsetPx = options.offsetPx ?? POPOVER_DEFAULT_OFFSET

   popover.style.maxWidth = `${window.innerWidth - POPOVER_VIEWPORT_PADDING * 2}px`

   const { x, y } = await computePosition(anchor, popover, {
      placement: options.placement,
      middleware: [offset(offsetPx), flip(), shift({ padding: POPOVER_VIEWPORT_PADDING })],
   })

   popover.style.left = `${x}px`
   popover.style.top = `${y}px`
}

/** Dismiss on the next scroll of the nearest scrollable ancestor (or document). */
export function attachScrollDismiss(trigger: HTMLElement, onDismiss: () => void): () => void {
   const target = findScrollableAncestor(trigger)

   target.addEventListener('scroll', onDismiss, { passive: true, once: true })

   return () => target.removeEventListener('scroll', onDismiss)
}

export function clearPopoverTimer(timer: PopoverTimerRef): PopoverTimerRef {
   if (timer) clearTimeout(timer)

   return null
}

export function clearPopoverTimers(timers: {
   enterTimer: PopoverTimerRef
   leaveTimer: PopoverTimerRef
}): void {
   timers.enterTimer = clearPopoverTimer(timers.enterTimer)
   timers.leaveTimer = clearPopoverTimer(timers.leaveTimer)
}
