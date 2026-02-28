<!--
ScanResultsListItem

Purpose: Single row for folder or file. Selection circle, icon, name, item count (folders), size, nav chevron (folders).

Props: item (FolderInfo), selected (boolean), someSelected (boolean?), selectable (boolean?), formatBytes (fn)

Example:
 <ScanResultsListItem
   :item="folder"
   :selected="selectedPaths.has(folder.path)"
   :someSelected="someSelectedPaths.has(folder.path)"
   :selectable="!folder.is_protected"
   :formatBytes="formatBytes"
   @select="toggleSelect"
   @navigate="goInto"
 />
-->

<script setup lang="ts">
import {
   PhFolder,
   PhFile,
   PhCaretRight,
   PhCircle,
   PhCheckCircle,
   PhMinusCircle,
} from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'

import type { FolderInfo } from '@/types/structures'

defineProps<{
   item: FolderInfo
   selected: boolean
   someSelected?: boolean
   selectable?: boolean
   formatBytes: (bytes: number) => string
}>()

const emit = defineEmits<{
   (e: 'select'): void
   (e: 'navigate'): void
}>()

const { t } = useTranslations()
</script>

<template>
   <div
      class="ScanResultsListItem-root"
      :class="{
         'ScanResultsListItem-root--selected': selected,
         'ScanResultsListItem-root--folder': !item.is_file,
      }"
      @click="!item.is_file && emit('navigate')"
   >
      <button
         type="button"
         class="ScanResultsListItem-check"
         :class="{
            'ScanResultsListItem-check--selected': selected,
            'ScanResultsListItem-check--some-selected': !selected && someSelected,
            'ScanResultsListItem-check--disabled': !selectable,
         }"
         :aria-pressed="selected || someSelected"
         :disabled="!selectable"
         :aria-disabled="!selectable"
         @click.stop="selectable && emit('select')"
      >
         <PhCircle
            v-if="!selected && !someSelected"
            :size="22"
            weight="regular"
            class="ScanResultsListItem-checkEmpty"
            aria-hidden="true"
         />
         <PhMinusCircle
            v-else-if="someSelected"
            :size="22"
            weight="fill"
            class="ScanResultsListItem-checkPartial"
            aria-hidden="true"
         />
         <PhCheckCircle
            v-else
            :size="22"
            weight="fill"
            class="ScanResultsListItem-checkFilled"
            aria-hidden="true"
         />
      </button>
      <div class="ScanResultsListItem-icon">
         <PhFolder v-if="!item.is_file" :size="28" weight="regular" aria-hidden="true" />
         <PhFile v-else :size="28" weight="regular" aria-hidden="true" />
      </div>
      <div class="ScanResultsListItem-info">
         <span class="ScanResultsListItem-name">{{ item.name }}</span>
         <span v-if="!item.is_file" class="ScanResultsListItem-count">
            {{
               item.children.length === 1
                  ? t('ScanResultsListItem', 'itemOne')
                  : t('ScanResultsListItem', 'itemsCount', { count: item.children.length })
            }}
         </span>
      </div>
      <div class="ScanResultsListItem-meta">
         <span class="ScanResultsListItem-size">{{ formatBytes(item.size) }}</span>
         <PhCaretRight
            v-if="!item.is_file"
            :size="18"
            weight="regular"
            class="ScanResultsListItem-chevron"
            aria-hidden="true"
         />
      </div>
   </div>
</template>

<style scoped>
.ScanResultsListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
   padding: var(--spacing-sm);
   height: 64px;
   min-height: 64px;
   margin: calc(var(--spacing-sm) / 2) var(--spacing-sm);
   border-radius: 8px;
   box-sizing: border-box;
   transition: background 0.2s;

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanResultsListItem-root--folder {
   cursor: pointer;
}

.ScanResultsListItem-root--selected {
   background: var(--color-accent-bg);

   &:hover {
      background: var(--color-accent-bg);
   }
}

.ScanResultsListItem-check {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0;
   background: none;
   border: none;
   cursor: pointer;
   color: var(--color-text-dim);
   transition: filter 0.2s;
}

.ScanResultsListItem-check--selected .ScanResultsListItem-checkFilled {
   color: var(--color-accent);
   filter: drop-shadow(0 0 4px var(--color-accent-glow));
}

.ScanResultsListItem-check--some-selected .ScanResultsListItem-checkPartial {
   color: var(--color-accent);
}

.ScanResultsListItem-check--disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

.ScanResultsListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanResultsListItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 24px;
   height: 24px;
   color: var(--color-accent);
}

.ScanResultsListItem-info {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   gap: var(--spacing-xxs);
}

.ScanResultsListItem-name {
   font-size: 0.9375rem;
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsListItem-count {
   font-size: 0.8125rem;
   color: var(--color-text-muted);
}

.ScanResultsListItem-meta {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.ScanResultsListItem-size {
   font-size: 0.875rem;
   color: var(--color-text-muted);
}

.ScanResultsListItem-chevron {
   color: var(--color-text-dim);
}
</style>
