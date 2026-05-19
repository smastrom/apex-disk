<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
AnimatedAlertCircle

Purpose: Animated SVG alert-circle icon. The circle scales in (pop), then the exclamation mark draws itself.

Props: size (number, default 48)

Example:
 <AnimatedAlertCircle :size="48" />
-->

<script setup lang="ts">
withDefaults(defineProps<{ size?: number }>(), { size: 48 })
</script>

<template>
   <svg
      class="AnimatedAlertCircle-root"
      :width="size"
      :height="size"
      viewBox="0 0 256 256"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
   >
      <circle class="AnimatedAlertCircle-circle" cx="128" cy="128" r="120" fill="currentColor" />
      <line
         class="AnimatedAlertCircle-line"
         x1="128"
         y1="80"
         x2="128"
         y2="152"
         stroke="var(--color-bg, #fff)"
         stroke-width="18"
         stroke-linecap="round"
      />
      <circle
         class="AnimatedAlertCircle-dot"
         cx="128"
         cy="184"
         r="10"
         fill="var(--color-bg, #fff)"
      />
   </svg>
</template>

<style scoped>
.AnimatedAlertCircle-root {
   overflow: visible;
}

.AnimatedAlertCircle-circle {
   transform-origin: center;
   animation: AnimatedAlertCircle-pop 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) both;

   @media (prefers-reduced-motion: reduce) {
      animation-name: AnimatedAlertCircle-fade-in;
   }
}

.AnimatedAlertCircle-line {
   stroke-dasharray: 72;
   stroke-dashoffset: 72;
   animation: AnimatedAlertCircle-draw 0.35s ease-out 0.3s forwards;

   @media (prefers-reduced-motion: reduce) {
      animation: none;
      stroke-dashoffset: 0;
   }
}

.AnimatedAlertCircle-dot {
   opacity: 0;
   animation: AnimatedAlertCircle-fade-in 0.2s ease-out 0.55s forwards;

   @media (prefers-reduced-motion: reduce) {
      animation: none;
      opacity: 1;
   }
}

@keyframes AnimatedAlertCircle-pop {
   0% {
      transform: scale(0);
      opacity: 0;
   }
   100% {
      transform: scale(1);
      opacity: 1;
   }
}

@keyframes AnimatedAlertCircle-draw {
   to {
      stroke-dashoffset: 0;
   }
}

@keyframes AnimatedAlertCircle-fade-in {
   to {
      opacity: 1;
   }
}
</style>
