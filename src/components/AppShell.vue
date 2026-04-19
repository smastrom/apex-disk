<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<script setup lang="ts">
import App from './App.vue'
import AppLoadingScreen from './AppLoadingScreen.vue'
</script>

<template>
   <Suspense>
      <App />
      <template #fallback>
         <AppLoadingScreen />
      </template>
   </Suspense>
</template>
