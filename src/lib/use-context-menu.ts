// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { onMounted, onUnmounted } from 'vue'

/**
 * Suppresses the WKWebView UI surface that has no place in a native macOS app:
 * the right-click context menu (whose only item is "Reload") and the reload
 * keystrokes (cmd-R, cmd-Shift-R, F5).
 */
export function disableNativeContextMenu() {
   if (import.meta.env.DEV) return

   function preventContextMenu(event: Event) {
      // Let the native menu through when there's something to act on (selected
      // text → Copy/Look Up, editable field → Cut/Copy/Paste). Without this,
      // we'd block Copy on `.ScanResultsListItem-name` selections.
      const selection = window.getSelection()
      const hasSelection = !!selection && !selection.isCollapsed && selection.toString().length > 0
      const target = event.target as HTMLElement | null
      const isEditable =
         !!target &&
         (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable)

      if (hasSelection || isEditable) return

      event.preventDefault()
   }

   function preventReloadShortcut(event: KeyboardEvent) {
      const cmdOrCtrl = event.metaKey || event.ctrlKey
      const isReloadKey = (cmdOrCtrl && event.key.toLowerCase() === 'r') || event.key === 'F5'

      if (isReloadKey) event.preventDefault()
   }

   onMounted(() => {
      document.addEventListener('contextmenu', preventContextMenu)
      document.addEventListener('keydown', preventReloadShortcut, true)
   })

   onUnmounted(() => {
      document.removeEventListener('contextmenu', preventContextMenu)
      document.removeEventListener('keydown', preventReloadShortcut, true)
   })
}
