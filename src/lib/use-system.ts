// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import type { DiskUsage } from '@/types/disk'
import type { SystemInfo } from '@/types/system-info'

import { ref, watch } from 'vue'

import { useDiskUsage } from './use-disk-usage'
import { useFullDiskAccess } from './use-full-disk-access'
import { useSystemInfo } from './use-system-info'

/**
 * Bootstraps macOS system data (info, Full Disk Access, disk usage) and exposes
 * a readiness flag plus the forwarded values. Call from a component's synchronous
 * setup so lifecycle hooks inside the inner composables bind correctly.
 */
export function useSystem() {
   const isSystemLoaded = ref(false)
   const systemInfo = ref<SystemInfo | null>(null)
   const isFdaGranted = ref(false)
   const diskUsage = ref<DiskUsage | null>(null)

   Promise.all([useSystemInfo(), useFullDiskAccess(), useDiskUsage()]).then(([s, f, d]) => {
      watch(s.systemInfo, (v) => (systemInfo.value = v), { immediate: true })
      watch(f.isFdaGranted, (v) => (isFdaGranted.value = v), { immediate: true })
      watch(d.diskUsage, (v) => (diskUsage.value = v), { immediate: true })

      isSystemLoaded.value = true
   })

   return {
      isSystemLoaded,
      systemInfo,
      isFdaGranted,
      diskUsage,
   }
}
