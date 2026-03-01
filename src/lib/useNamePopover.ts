import { ref, onUnmounted, type Ref } from 'vue'

/**
 * Manages a native popover for truncated text with debounced hover.
 *
 * - Only shows when the element's text is actually truncated (scrollWidth > clientWidth).
 * - 400 ms enter debounce, 200 ms leave debounce (macOS-style timing).
 * - Positions the popover just above the trigger element.
 */
export function useNamePopover() {
   const popoverRef: Ref<HTMLElement | null> = ref(null)
   const triggerRef: Ref<HTMLElement | null> = ref(null)
   const isOpen = ref(false)

   let enterTimer: ReturnType<typeof setTimeout> | null = null
   let leaveTimer: ReturnType<typeof setTimeout> | null = null
   let scrollCleanup: (() => void) | null = null

   const ENTER_DELAY = 400
   const LEAVE_DELAY = 200

   function isTruncated(): boolean {
      const el = triggerRef.value
      if (!el) return false
      return el.scrollWidth > el.clientWidth
   }

   function positionPopover() {
      const trigger = triggerRef.value
      const popover = popoverRef.value
      if (!trigger || !popover) return

      const rect = trigger.getBoundingClientRect()
      popover.style.left = `${rect.left}px`
      popover.style.top = `${rect.top - 4}px`
   }

   function show() {
      const popover = popoverRef.value
      if (!popover || isOpen.value) return
      positionPopover()
      popover.showPopover()
      isOpen.value = true
      addScrollListener()
   }

   function hide() {
      const popover = popoverRef.value
      if (!popover || !isOpen.value) return
      popover.hidePopover()
      isOpen.value = false
      removeScrollListener()
   }

   /** Immediately dismiss — no leave debounce. Used by scroll. */
   function dismiss() {
      clearTimers()
      const popover = popoverRef.value
      if (!popover || !isOpen.value) return
      popover.hidePopover()
      isOpen.value = false
      removeScrollListener()
   }

   function addScrollListener() {
      const el = triggerRef.value
      if (!el) return

      // Walk up to find the nearest scrollable ancestor
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
      if (!isTruncated()) return

      // Cancel any pending leave
      if (leaveTimer) {
         clearTimeout(leaveTimer)
         leaveTimer = null
      }

      enterTimer = setTimeout(show, ENTER_DELAY)
   }

   function onPointerLeave() {
      // Cancel any pending enter
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
      popoverRef,
      triggerRef,
      isOpen,
      onPointerEnter,
      onPointerLeave,
   }
}
