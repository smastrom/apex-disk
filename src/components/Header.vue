<!--
Header

Purpose: Top bar of the app layout. Placeholder for future navigation/actions.

Props: none

Example:
 <Header />
-->

<script setup lang="ts"></script>

<template>
   <header class="Header-root">
      <h1 class="Header-title">Mac Disk Lens</h1>
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
}

.Header-title {
   font-size: 1.125rem;
   font-weight: 600;
   color: var(--color-text);
   margin: 0;
}
</style>
