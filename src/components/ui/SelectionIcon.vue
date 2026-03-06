<!--
SelectionIcon

Purpose: Reusable selection icon with three states: unselected (circle), partially selected (minus), fully selected (checkmark).

Props: state ('empty' | 'partial' | 'selected'), size (number?)

Example:
 <SelectionIcon state="partial" :size="16" />
-->

<script setup lang="ts">
import { PhCircle, PhMinusCircle, PhCheckCircle } from '@phosphor-icons/vue'

defineProps<{
   state: 'empty' | 'partial' | 'selected'
   size?: number
}>()
</script>

<template>
   <PhCircle v-if="state === 'empty'" :size="size ?? 16" weight="regular" aria-hidden="true" />
   <PhMinusCircle
      v-else-if="state === 'partial'"
      :size="size ?? 16"
      weight="regular"
      aria-hidden="true"
   />
   <PhCheckCircle v-else :size="size ?? 16" weight="regular" aria-hidden="true" />
</template>
