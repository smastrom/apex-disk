<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
AppViewAnnouncer

Purpose: Screen-reader live region that announces the current view name when the user switches tabs.

Props: activeView (string)

Example:
 <AppViewAnnouncer :activeView="activeView" />
-->

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslations } from '@/lib/use-translations'

const props = defineProps<{
   activeView: string
}>()

const { t } = useTranslations()

const viewAnnouncement = computed(() => {
   const labels: Record<string, string> = {
      scan: t('AppFooter', 'scan'),
      settings: t('AppFooter', 'settings'),
      information: t('AppFooter', 'information'),
   }

   return labels[props.activeView] ?? ''
})
</script>

<template>
   <div aria-live="polite" aria-atomic="true" class="sr-only">{{ viewAnnouncement }}</div>
</template>
