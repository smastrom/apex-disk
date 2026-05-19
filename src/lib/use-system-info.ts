// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import type { SystemInfo } from '@/types/system-info'

import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

/** Loads system information on mount and exposes reactive refs with formatted data. */
export async function useSystemInfo() {
   const systemInfo = ref<SystemInfo | null>(null)

   async function loadSystemInfo() {
      try {
         systemInfo.value = await invoke<SystemInfo>('get_system_info')
      } catch (error) {
         console.error('Failed to load system info:', error)
      }
   }

   // Load system info
   await loadSystemInfo()

   return {
      systemInfo,
   }
}
