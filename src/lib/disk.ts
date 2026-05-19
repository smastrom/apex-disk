// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import type { DiskUsage } from '@/types/disk'

import { invoke } from '@tauri-apps/api/core'

/** Loads current disk usage for the active user home volume. */
export async function getDiskUsage(): Promise<DiskUsage> {
   return invoke<DiskUsage>('get_disk_usage')
}
