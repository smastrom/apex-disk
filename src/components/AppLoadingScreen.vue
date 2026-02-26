<!--
AppLoadingScreen

Purpose: Fullscreen loading screen with centered spinner, shown while the app is bootstrapping.

Props: none

Example:
 <AppLoadingScreen />
-->

<script setup lang="ts">
import Spinner from '@/components/Spinner.vue'
</script>

<template>
   <div class="AppLoadingScreen-root">
      <Spinner :size="40" />
   </div>
</template>

<style scoped>
.AppLoadingScreen-root {
   position: fixed;
   inset: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   background: var(--color-bg);
   z-index: 9999;
}
</style>
