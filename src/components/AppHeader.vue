<!--
AppHeader

Purpose: Top bar of the app layout. Placeholder for future navigation/actions.

Props: none

Example:
 <AppHeader />
-->

<script setup lang="ts">
import { useTranslations } from '@/lib/use-translations'

const { t } = useTranslations()
</script>

<template>
   <header class="AppHeader-root" data-tauri-drag-region data-testid="app-header">
      <div class="AppHeader-inner">
         <h1 class="AppHeader-title" data-testid="app-title">{{ t('AppHeader', 'title') }}</h1>
      </div>
   </header>
</template>

<style scoped>
.AppHeader-root {
   height: var(--header-height);
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0 var(--spacing-md);
   background: var(--color-bg-elevated);
   flex-shrink: 0;
   border-bottom: 1px solid var(--color-bg);
   box-shadow: 0 1px 12px var(--color-bg);
}

.AppHeader-inner {
   display: flex;
   align-items: baseline;
   gap: var(--spacing-sm);
   pointer-events: none;
}

.AppHeader-title {
   font-size: 1.125rem;
   font-weight: 600;
   color: var(--color-text);
   margin: 0;
   text-shadow: var(--glow-text);
   pointer-events: none;
}

.AppHeader-version {
   font-size: 0.75rem;
   color: var(--color-text-muted);
   font-weight: 400;
   pointer-events: auto;
   transition: color 0.15s;
}

.AppHeader-version:hover {
   color: var(--color-text);
}
</style>
