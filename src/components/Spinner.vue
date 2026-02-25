<!--
Spinner

Purpose: Indeterminate loading indicator using ring-resize style. Theme-aware stroke color.

Props: size (number?, default 24)

Example:
 <Spinner />
 <Spinner :size="20" />
-->

<script setup lang="ts">
defineProps<{
   size?: number
}>()
</script>

<template>
   <svg
      class="Spinner-root"
      :width="size ?? 24"
      :height="size ?? 24"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
   >
      <circle
         class="Spinner-circle"
         cx="12"
         cy="12"
         r="9.5"
         fill="none"
         stroke-width="3"
         stroke-linecap="round"
      />
   </svg>
</template>

<style scoped>
.Spinner-root {
   flex-shrink: 0;
   stroke: currentColor;
   color: var(--color-text-muted);
   transform-origin: center;
   animation: Spinner-rotate 2s linear infinite;
}

.Spinner-circle {
   animation: Spinner-dash 1.5s ease-in-out infinite;
}

@keyframes Spinner-rotate {
   to {
      transform: rotate(360deg);
   }
}

@keyframes Spinner-dash {
   0% {
      stroke-dasharray: 0 150;
      stroke-dashoffset: 0;
   }

   47.5% {
      stroke-dasharray: 42 150;
      stroke-dashoffset: -16;
   }

   95%,
   100% {
      stroke-dasharray: 42 150;
      stroke-dashoffset: -59;
   }
}
</style>
