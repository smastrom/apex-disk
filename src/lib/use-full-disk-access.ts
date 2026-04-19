// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

/** Checks macOS Full Disk Access permission on mount and exposes the result as a reactive ref. */
export async function useFullDiskAccess() {
   const isFdaGranted = ref(false)

   try {
      isFdaGranted.value = await invoke<boolean>('check_full_disk_access')
   } catch (err) {
      console.error('[FDA] invoke failed:', err)
   }

   return { isFdaGranted }
}
