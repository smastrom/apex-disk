<!--
ScanResultsDeleteListItem

Purpose: Compact list row for delete review. Checkbox (default on), icon, name, size.

Props: item (DeleteListItem), selected (boolean), formatBytes (fn)

Example:
 <ScanResultsDeleteListItem
   :item="entry"
   :selected="checkedSet.has(entry.path)"
   :formatBytes="formatBytes"
   @toggle="toggle(entry.path)"
 />
-->

<script setup lang="ts">
import { PhFolderSimple, PhFile, PhCircle, PhCheckCircle } from '@phosphor-icons/vue'

import type { DeleteListItem } from '@/types/structs'

defineProps<{
   item: DeleteListItem
   selected: boolean
   formatBytes: (bytes: number) => string
}>()

/** Delete list items are never protected; only hidden (name starts with dot) affects icon. */
function isHidden(item: DeleteListItem) {
   return item.name.startsWith('.')
}

const emit = defineEmits<{
   (e: 'toggle'): void
}>()
</script>

<template>
   <div class="ScanResultsDeleteListItem-root" @click="emit('toggle')">
      <button
         type="button"
         class="ScanResultsDeleteListItem-check"
         :class="{ 'ScanResultsDeleteListItem-check--selected': selected }"
         :aria-pressed="selected"
         @click.stop="emit('toggle')"
      >
         <PhCircle
            v-if="!selected"
            :size="16"
            weight="regular"
            class="ScanResultsDeleteListItem-checkEmpty"
            aria-hidden="true"
         />
         <PhCheckCircle
            v-else
            :size="16"
            weight="fill"
            class="ScanResultsDeleteListItem-checkFilled"
            aria-hidden="true"
         />
      </button>
      <div
         class="ScanResultsDeleteListItem-icon"
         :class="{ 'ScanResultsDeleteListItem-icon--hidden': isHidden(item) }"
      >
         <PhFolderSimple v-if="!item.is_file" :size="18" weight="regular" aria-hidden="true" />
         <PhFile v-else :size="18" weight="regular" aria-hidden="true" />
      </div>
      <span class="ScanResultsDeleteListItem-name">{{ item.name }}</span>
      <span class="ScanResultsDeleteListItem-size">{{ formatBytes(item.size) }}</span>
   </div>
</template>

<style scoped>
.ScanResultsDeleteListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-xs) var(--spacing-sm);
   min-height: 40px;
   cursor: pointer;
   transition: background 0.2s;

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanResultsDeleteListItem-check {
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

.ScanResultsDeleteListItem-check--selected .ScanResultsDeleteListItem-checkFilled {
   color: var(--color-accent);
}

.ScanResultsDeleteListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanResultsDeleteListItem-icon {
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

.ScanResultsDeleteListItem-icon--hidden {
   opacity: 0.5;
}

.ScanResultsDeleteListItem-name {
   flex: 1;
   min-width: 0;
   font-size: 0.8125rem;
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsDeleteListItem-size {
   flex-shrink: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
}
</style>
