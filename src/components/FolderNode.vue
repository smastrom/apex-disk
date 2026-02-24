<script setup lang="ts">
import { computed } from 'vue'

interface FolderInfo {
   name: string
   path: string
   size: number
   icon?: string
   children: FolderInfo[]
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
</script>

<template>
   <li class="folder-item">
      <div
         class="folder-row"
         :class="{ expandable: folder.children.length > 0, nested: depth > 0 }"
         :style="{ paddingLeft: (depth * 12 + 4) + 'px' }"
         @click="folder.children.length ? toggleExpand(folder.path) : null"
      >
         <span class="folder-arrow" v-if="folder.children.length">
            {{ isExpanded ? '▼' : '▶' }}
         </span>
         <span class="folder-arrow-placeholder" v-else></span>
         <span class="folder-name" :title="folder.path">{{ folder.name }}</span>
         <span class="folder-size">{{ formatBytes(folder.size) }}</span>
      </div>
      <ul v-if="folder.children.length && isExpanded" class="folder-children">
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
