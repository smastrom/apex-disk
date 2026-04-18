<!--
AppHeader

Purpose: Top bar of the app layout. Placeholder for future navigation/actions.

Props: none

Example:
 <AppHeader />
-->

<script setup lang="ts">
import { APP_NAME } from '@/lib/constants'
</script>

<template>
   <header class="AppHeader-root" data-tauri-drag-region data-testid="app-header">
      <div class="AppHeader-inner">
         <h1 class="AppHeader-title" data-testid="app-title">{{ APP_NAME }}</h1>
      </div>
   </header>
</template>

<style scoped>
.AppHeader-root {
   position: relative;
   height: var(--header-height);
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0 var(--spacing-md);
   background: var(--color-chrome);
   -webkit-backdrop-filter: saturate(180%) blur(30px);
   backdrop-filter: saturate(180%) blur(30px);
   flex-shrink: 0;
   border-bottom: 1px solid var(--color-chrome-border);
   z-index: 2;
}

.AppHeader-inner {
   display: flex;
   align-items: baseline;
   gap: var(--spacing-sm);
   pointer-events: none;
}

.AppHeader-title {
   font-size: var(--font-size-xl);
   font-weight: 600;
   color: var(--color-text);
   margin: 0;
   text-shadow: var(--glow-text);
   pointer-events: none;
}
</style>
