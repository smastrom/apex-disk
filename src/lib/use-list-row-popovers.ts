// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { computePosition, flip, offset, shift, type Placement } from '@floating-ui/dom'
import { onMounted, onUnmounted, type Ref } from 'vue'

const ENTER_DELAY = 400
const LEAVE_DELAY = 200
const VIEWPORT_PADDING = 16

type PopoverKind = 'name' | 'checkbox'
type DisabledTooltipKind = 'fda' | 'protected'

interface PopoverState {
   trigger: HTMLElement | null
   anchor: HTMLElement | null
   isOpen: boolean
   enterTimer: ReturnType<typeof setTimeout> | null
   leaveTimer: ReturnType<typeof setTimeout> | null
   cleanupScroll: (() => void) | null
}

function makeState(): PopoverState {
   return {
      trigger: null,
      anchor: null,
      isOpen: false,
      enterTimer: null,
      leaveTimer: null,
      cleanupScroll: null,
   }
}

/**
 * One name popover + one checkbox popover shared across all rows of a list,
 * driven by event delegation on the list root.
 *
 * Rows opt into a tooltip by exposing well-known classes/attributes:
 * - `.ScanResultsListItem-name` (truncation-only; text taken from textContent)
 * - `.ScanResultsListItem-check[data-disabled-tooltip="fda"|"protected"]`
 */
export function useListRowPopovers(
   rootRef: Ref<HTMLElement | null>,
   namePopoverRef: Ref<HTMLElement | null>,
   checkboxPopoverRef: Ref<HTMLElement | null>,
   opts: { resolveCheckboxText: (kind: DisabledTooltipKind) => string }
) {
   const states: Record<PopoverKind, PopoverState> = {
      name: makeState(),
      checkbox: makeState(),
   }

   function popoverEl(kind: PopoverKind): HTMLElement | null {
      return (kind === 'name' ? namePopoverRef : checkboxPopoverRef).value
   }

   function placementFor(kind: PopoverKind): Placement {
      return kind === 'name' ? 'top' : 'top-start'
   }

   async function position(kind: PopoverKind) {
      const state = states[kind]
      const el = popoverEl(kind)

      if (!state.anchor || !el) return

      el.style.maxWidth = `${window.innerWidth - VIEWPORT_PADDING * 2}px`

      const { x, y } = await computePosition(state.anchor, el, {
         placement: placementFor(kind),
         middleware: [offset(8), flip(), shift({ padding: VIEWPORT_PADDING })],
      })

      el.style.left = `${x}px`
      el.style.top = `${y}px`
   }

   async function open(kind: PopoverKind, text: string) {
      const state = states[kind]
      const el = popoverEl(kind)

      if (!el) return

      el.textContent = text

      state.isOpen = true

      await position(kind)
      el.classList.add('is-open')
      addScrollListener(kind)
   }

   function close(kind: PopoverKind) {
      const state = states[kind]
      const el = popoverEl(kind)

      if (!el || !state.isOpen) return

      el.classList.remove('is-open')

      state.isOpen = false
      state.anchor = null
      state.trigger = null

      removeScrollListener(kind)
   }

   function addScrollListener(kind: PopoverKind) {
      const state = states[kind]

      if (!state.trigger) return

      let ancestor: HTMLElement | null = state.trigger.parentElement

      while (ancestor) {
         const { overflow, overflowY } = getComputedStyle(ancestor)

         if (/auto|scroll/.test(overflow + overflowY)) break

         ancestor = ancestor.parentElement
      }

      const target = ancestor ?? document

      const dismiss = () => {
         clearTimers(kind)
         close(kind)
      }

      target.addEventListener('scroll', dismiss, { passive: true, once: true })

      state.cleanupScroll = () => target.removeEventListener('scroll', dismiss)
   }

   function removeScrollListener(kind: PopoverKind) {
      const state = states[kind]

      state.cleanupScroll?.()
      state.cleanupScroll = null
   }

   function clearTimers(kind: PopoverKind) {
      const state = states[kind]

      if (state.enterTimer) {
         clearTimeout(state.enterTimer)

         state.enterTimer = null
      }
      if (state.leaveTimer) {
         clearTimeout(state.leaveTimer)

         state.leaveTimer = null
      }
   }

   interface Match {
      kind: PopoverKind
      trigger: HTMLElement
      anchor: HTMLElement
      text: string
   }

   function resolve(target: EventTarget | null): Match | null {
      if (!(target instanceof Element)) return null

      const nameEl = target.closest<HTMLElement>('.ScanResultsListItem-name')

      if (nameEl) {
         if (nameEl.scrollWidth <= nameEl.clientWidth) return null

         return {
            kind: 'name',
            trigger: nameEl,
            anchor: nameEl,
            text: nameEl.textContent?.trim() ?? '',
         }
      }

      const checkEl = target.closest<HTMLElement>('.ScanResultsListItem-check')

      if (checkEl) {
         const kind = checkEl.getAttribute('data-disabled-tooltip')

         if (kind !== 'fda' && kind !== 'protected') return null

         const icon = checkEl.querySelector<HTMLElement>('svg') ?? checkEl

         return {
            kind: 'checkbox',
            trigger: checkEl,
            anchor: icon,
            text: opts.resolveCheckboxText(kind),
         }
      }

      return null
   }

   function onPointerOver(e: PointerEvent) {
      const match = resolve(e.target)

      if (!match) return

      const state = states[match.kind]

      if (state.trigger === match.trigger) {
         if (state.leaveTimer) {
            clearTimeout(state.leaveTimer)

            state.leaveTimer = null
         }

         return
      }

      clearTimers(match.kind)

      state.trigger = match.trigger
      state.anchor = match.anchor

      if (state.isOpen) {
         const el = popoverEl(match.kind)

         if (el) el.textContent = match.text

         position(match.kind)
      } else {
         state.enterTimer = setTimeout(() => open(match.kind, match.text), ENTER_DELAY)
      }
   }

   function onPointerOut(e: PointerEvent) {
      const related = e.relatedTarget as Node | null

      for (const kind of ['name', 'checkbox'] as const) {
         const state = states[kind]

         if (!state.trigger) continue
         if (related && state.trigger.contains(related)) continue

         const el = popoverEl(kind)

         if (related && el && el.contains(related)) continue

         if (state.enterTimer) {
            clearTimeout(state.enterTimer)

            state.enterTimer = null
         }

         if (state.isOpen) {
            if (state.leaveTimer) clearTimeout(state.leaveTimer)

            state.leaveTimer = setTimeout(() => close(kind), LEAVE_DELAY)
         } else {
            state.trigger = null
            state.anchor = null
         }
      }
   }

   function onPopoverEnter(kind: PopoverKind) {
      const state = states[kind]

      if (state.leaveTimer) {
         clearTimeout(state.leaveTimer)

         state.leaveTimer = null
      }
   }

   function onPopoverLeave(kind: PopoverKind) {
      const state = states[kind]

      if (!state.isOpen) return

      if (state.leaveTimer) clearTimeout(state.leaveTimer)

      state.leaveTimer = setTimeout(() => close(kind), LEAVE_DELAY)
   }

   function dismissAll() {
      for (const kind of ['name', 'checkbox'] as const) {
         clearTimers(kind)
         close(kind)
      }
   }

   onMounted(() => {
      const root = rootRef.value

      if (!root) return

      root.addEventListener('pointerover', onPointerOver)
      root.addEventListener('pointerout', onPointerOut)
   })

   onUnmounted(() => {
      const root = rootRef.value

      root?.removeEventListener('pointerover', onPointerOver)
      root?.removeEventListener('pointerout', onPointerOut)

      dismissAll()
   })

   return {
      dismissAll,
      onNamePopoverEnter: () => onPopoverEnter('name'),
      onNamePopoverLeave: () => onPopoverLeave('name'),
      onCheckboxPopoverEnter: () => onPopoverEnter('checkbox'),
      onCheckboxPopoverLeave: () => onPopoverLeave('checkbox'),
   }
}
