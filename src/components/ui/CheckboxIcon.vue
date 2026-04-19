<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
CheckboxIcon

Purpose: Reusable checkbox icon that shows empty circle when unselected and filled checkmark when selected.

Props: selected (boolean), size (number?)

Example:
 <CheckboxIcon :selected="isSelected" :size="16" />
-->

<script setup lang="ts">
import { PhCircle, PhCheckCircle } from '@phosphor-icons/vue'

defineProps<{
   selected: boolean
   size?: number
}>()
</script>

<template>
   <PhCircle v-if="!selected" :size="size ?? 16" weight="regular" aria-hidden="true" />
   <PhCheckCircle v-else :size="size ?? 16" weight="fill" aria-hidden="true" />
</template>
