<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<script setup lang="ts">
import AppLayout from './AppLayout.vue'
import AppLoadingScreen from './AppLoadingScreen.vue'

import { useSystem } from '@/lib/use-system'

const { isSystemLoaded, systemInfo, isFdaGranted, diskUsage } = useSystem()

import '@/assets/css/theme.css'
import '@/assets/css/global.css'
import '@/assets/css/reset.css'
import '@/assets/css/classes.css'
import '@/assets/css/animations.css'
import '@/assets/css/rtl.css'
</script>

<template>
   <AppLoadingScreen v-if="!isSystemLoaded" />
   <AppLayout v-else :systemInfo="systemInfo" :isFdaGranted="isFdaGranted" :diskUsage="diskUsage" />
</template>
