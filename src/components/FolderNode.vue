<script setup lang="ts">
import { computed } from 'vue'

interface FolderInfo {
   name: string
   path: string
   size: number
   icon?: string
   children: FolderInfo[]
   is_file: boolean
}

const props = defineProps<{
   folder: FolderInfo
   depth?: number
   expandedPaths: Set<string>
   formatBytes: (n: number) => string
   toggleExpand: (path: string) => void
}>()

const depth = computed(() => props.depth ?? 0)
const isExpanded = computed(() => props.expandedPaths.has(props.folder.path))
const hasChildren = computed(() => !props.folder.is_file && props.folder.children.length > 0)
</script>

<template>
   <li class="folder-item">
      <div
         class="folder-row"
         :class="{
            expandable: hasChildren,
            nested: depth > 0,
            'is-file': folder.is_file,
         }"
         :style="{ paddingLeft: (depth * 12 + 4) + 'px' }"
         @click="hasChildren ? toggleExpand(folder.path) : null"
      >
         <span class="folder-arrow" v-if="hasChildren">
            {{ isExpanded ? '▼' : '▶' }}
         </span>
         <span class="folder-arrow-placeholder" v-else></span>
         <span class="folder-icon">{{ folder.is_file ? '📄' : '📁' }}</span>
         <span class="folder-name" :title="folder.path">{{ folder.name }}</span>
         <span class="folder-size">{{ formatBytes(folder.size) }}</span>
      </div>
      <ul v-if="hasChildren && isExpanded" class="folder-children">
         <FolderNode
            v-for="child in folder.children"
            :key="child.path"
            :folder="child"
            :depth="depth + 1"
            :expanded-paths="expandedPaths"
            :format-bytes="formatBytes"
            :toggle-expand="toggleExpand"
         />
      </ul>
   </li>
</template>
