<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
FilledCheckCircleIcon

Purpose: Static filled circle with a bold checkmark. Circle size matches Phosphor fill icons (MinusCircle, CheckCircle).

Props: size (number?)

Example:
 <FilledCheckCircleIcon :size="16" />
-->

<script setup lang="ts">
defineProps<{
   size?: number
}>()
</script>

<template>
   <svg
      :width="size ?? 16"
      :height="size ?? 16"
      viewBox="0 0 256 256"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
   >
      <circle cx="128" cy="128" r="104" fill="currentColor" />
      <path
         d="M91 130 L117 156 L169 104"
         stroke="var(--color-bg, #fff)"
         stroke-width="16"
         stroke-linecap="round"
         stroke-linejoin="round"
         fill="none"
      />
   </svg>
</template>
