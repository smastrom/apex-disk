<!--
ListItem

Purpose: Single row for folder or file. Selection circle, icon, name, item count (folders), size, nav chevron (folders).

Props: item (FolderInfo), selected (boolean), formatBytes (fn)

Example:
 <ListItem
   :item="folder"
   :selected="selectedPaths.has(folder.path)"
   :format-bytes="formatBytes"
   @select="toggleSelect"
   @navigate="goInto"
 />
-->

<script setup lang="ts">
import { PhFolder, PhFile, PhCaretRight, PhCircle, PhCheckCircle } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'

import type { FolderInfo } from '@/types/structures'

const { t } = useTranslations()

defineProps<{
   item: FolderInfo
   selected: boolean
   formatBytes: (bytes: number) => string
}>()

const emit = defineEmits<{
   (e: 'select'): void
   (e: 'navigate'): void
}>()
</script>

<template>
   <div
      class="ListItem-root"
      :class="{ 'ListItem-root--selected': selected, 'ListItem-root--folder': !item.is_file }"
      @click="!item.is_file && emit('navigate')"
   >
      <button
         type="button"
         class="ListItem-check"
         :class="{ 'ListItem-check--selected': selected }"
         :aria-pressed="selected"
         @click.stop="emit('select')"
      >
         <PhCircle v-if="!selected" :size="22" weight="regular" class="ListItem-checkEmpty" />
         <PhCheckCircle v-else :size="22" weight="fill" class="ListItem-checkFilled" />
      </button>
      <div class="ListItem-icon">
         <PhFolder v-if="!item.is_file" :size="28" weight="regular" />
         <PhFile v-else :size="28" weight="regular" />
      </div>
      <div class="ListItem-info">
         <span class="ListItem-name">{{ item.name }}</span>
         <span v-if="!item.is_file" class="ListItem-count">
            {{
               item.children.length === 1
                  ? t('ListItem', 'itemOne')
                  : t('ListItem', 'itemsCount', { count: item.children.length })
            }}
         </span>
      </div>
      <div class="ListItem-meta">
         <span class="ListItem-size">{{ formatBytes(item.size) }}</span>
         <PhCaretRight v-if="!item.is_file" :size="18" weight="regular" class="ListItem-chevron" />
      </div>
   </div>
</template>

<style scoped>
.ListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
   padding: var(--spacing-sm);
   min-height: 56px;
   margin: 4px var(--spacing-sm);
   border-radius: 8px;
   border: 1px solid transparent;
   transition:
      background 0.2s,
      border-color 0.2s;
}

.ListItem-root--folder {
   cursor: pointer;
}

.ListItem-root:hover {
   background: var(--color-accent-bg-hover);
   border-color: var(--color-accent-glow);
}

.ListItem-root--selected {
   background: var(--color-accent-bg);
   border-color: var(--color-accent-glow);
}

.ListItem-root--selected:hover {
   background: var(--color-accent-bg);
}

.ListItem-check {
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

.ListItem-check--selected .ListItem-checkFilled {
   color: var(--color-accent);
   filter: drop-shadow(0 0 4px var(--color-accent-glow));
}

.ListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ListItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 36px;
   height: 36px;
   border-radius: 8px;
   background: var(--color-surface);
   color: var(--color-accent);
}

.ListItem-info {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   gap: 4px;
}

.ListItem-name {
   font-size: 0.9375rem;
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ListItem-count {
   font-size: 0.8125rem;
   color: var(--color-text-muted);
}

.ListItem-meta {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.ListItem-size {
   font-size: 0.875rem;
   color: var(--color-text-muted);
}

.ListItem-chevron {
   color: var(--color-text-dim);
}
</style>
