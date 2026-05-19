<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<template>
   <div class="AppLoadingScreen-root" role="status" aria-label="Loading">
      <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
         <circle cx="12" cy="12" r="9.5" fill="none" stroke-width="3" stroke-linecap="round" />
      </svg>
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

   & svg {
      width: 40px;
      height: 40px;
      stroke: var(--color-text-muted);
      transform-origin: center;
      animation: spinner-rotate 2s linear infinite;

      @media (prefers-reduced-motion: reduce) {
         animation-name: spinner-pulse;
      }
   }

   & circle {
      animation: spinner-dash 1.5s ease-in-out infinite;

      @media (prefers-reduced-motion: reduce) {
         animation: none;
         stroke-dasharray: 42 150;
      }
   }
}

@keyframes spinner-rotate {
   to {
      transform: rotate(360deg);
   }
}

@keyframes spinner-dash {
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

@keyframes spinner-pulse {
   0%,
   100% {
      opacity: 1;
   }
   50% {
      opacity: 0.4;
   }
}
</style>
