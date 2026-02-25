<!--
Header

Purpose: Top bar of the app layout. Placeholder for future navigation/actions.

Props: none

Example:
 <Header />
-->

<script setup lang="ts">
import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()
</script>

<template>
   <header class="Header-root" data-tauri-drag-region>
      <h1 class="Header-title">{{ t('Header', 'title') }}</h1>
   </header>
</template>

<style scoped>
.Header-root {
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

.Header-title {
   font-size: 1.125rem;
   font-weight: 600;
   color: var(--color-text);
   margin: 0;
   text-shadow: var(--glow-text);
   pointer-events: none;
}
</style>
