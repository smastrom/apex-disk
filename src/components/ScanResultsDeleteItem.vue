<!--
ScanResultsDeleteItem

Purpose: Compact list row for delete review. Checkbox (default on), icon, name, size.

Props: item (DeleteListItem), selected (boolean), formatBytes (fn)

Example:
 <ScanResultsDeleteItem
   :item="entry"
   :selected="checkedSet.has(entry.path)"
   :formatBytes="formatBytes"
   @toggle="toggle(entry.path)"
 />
-->

<script setup lang="ts">
import { PhFolder, PhFile, PhCircle, PhCheckCircle } from '@phosphor-icons/vue'

import type { DeleteListItem } from '@/types/structures'

defineProps<{
   item: DeleteListItem
   selected: boolean
   formatBytes: (bytes: number) => string
}>()

const emit = defineEmits<{
   (e: 'toggle'): void
}>()
</script>

<template>
   <div class="ScanResultsDeleteItem-root" @click="emit('toggle')">
      <button
         type="button"
         class="ScanResultsDeleteItem-check"
         :class="{ 'ScanResultsDeleteItem-check--selected': selected }"
         :aria-pressed="selected"
         @click.stop="emit('toggle')"
      >
         <PhCircle v-if="!selected" :size="16" weight="regular" class="ScanResultsDeleteItem-checkEmpty" />
         <PhCheckCircle v-else :size="16" weight="fill" class="ScanResultsDeleteItem-checkFilled" />
      </button>
      <div class="ScanResultsDeleteItem-icon">
         <PhFolder v-if="!item.is_file" :size="18" weight="regular" />
         <PhFile v-else :size="18" weight="regular" />
      </div>
      <span class="ScanResultsDeleteItem-name">{{ item.name }}</span>
      <span class="ScanResultsDeleteItem-size">{{ formatBytes(item.size) }}</span>
   </div>
</template>

<style scoped>
.ScanResultsDeleteItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-xs) var(--spacing-sm);
   min-height: 40px;
   border-radius: 6px;
   cursor: pointer;
   transition: background 0.2s;

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanResultsDeleteItem-check {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0;
   background: none;
   border: none;
   cursor: pointer;
   color: var(--color-text-dim);
}

.ScanResultsDeleteItem-check--selected .ScanResultsDeleteItem-checkFilled {
   color: var(--color-accent);
}

.ScanResultsDeleteItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanResultsDeleteItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 24px;
   height: 24px;
   border-radius: 4px;
   background: var(--color-surface);
   color: var(--color-accent);
}

.ScanResultsDeleteItem-name {
   flex: 1;
   min-width: 0;
   font-size: 0.8125rem;
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsDeleteItem-size {
   flex-shrink: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
}
</style>
