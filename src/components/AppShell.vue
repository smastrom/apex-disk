<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<script setup lang="ts">
import App from './App.vue'
import AppLoadingScreen from './AppLoadingScreen.vue'

import { useSystem } from '@/lib/use-system'

const { isSystemLoaded, systemInfo, isFdaGranted, diskUsage } = useSystem()
</script>

<template>
   <AppLoadingScreen v-if="!isSystemLoaded" />
   <App v-else :systemInfo="systemInfo" :isFdaGranted="isFdaGranted" :diskUsage="diskUsage" />
</template>
