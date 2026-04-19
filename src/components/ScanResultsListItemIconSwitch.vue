<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanResultsListItemIconSwitch

Purpose: Renders a single folder or file icon by type. Folders: PhFolderSimpleLock (FDA-required), PhFolderSimpleUser (protected) or PhFolderSimple. Files: by extension PhFileText, PhFileAudio, PhFileVideo, PhFileArchive, PhFileImage, or PhFile.

Props: item ({ name: string, is_file: boolean, is_protected?: boolean, is_fda_required?: boolean }), size (number?)

Example:
 <div class="ScanResultsListItem-icon" :class="{ 'ScanResultsListItem-icon--hidden': item.name.startsWith('.') }">
   <ScanResultsListItemIconSwitch :item="item" :size="28" />
 </div>
-->

<script setup lang="ts">
import {
   PhFolderSimple,
   PhFolderSimpleUser,
   PhFolderSimpleLock,
   PhFile,
   PhFileText,
   PhFileAudio,
   PhFileVideo,
   PhFileArchive,
   PhFileImage,
} from '@phosphor-icons/vue'

import {
   DOC_EXTENSIONS,
   AUDIO_EXTENSIONS,
   VIDEO_EXTENSIONS,
   ARCHIVE_EXTENSIONS,
   IMAGE_EXTENSIONS,
} from '@/lib/constants'
import { getExtension } from '@/lib/utils'

defineProps<{
   item: { name: string; is_file: boolean; is_protected?: boolean; is_fda_required?: boolean }
   size?: number
}>()

/** Returns the Phosphor file icon component for the given filename by extension. */
function fileIconComponent(name: string) {
   const ext = getExtension(name)

   if (DOC_EXTENSIONS.has(ext)) return PhFileText
   if (AUDIO_EXTENSIONS.has(ext)) return PhFileAudio
   if (VIDEO_EXTENSIONS.has(ext)) return PhFileVideo
   if (ARCHIVE_EXTENSIONS.has(ext)) return PhFileArchive
   if (IMAGE_EXTENSIONS.has(ext)) return PhFileImage
   return PhFile
}
</script>

<template>
   <PhFolderSimpleLock
      v-if="!item.is_file && item.is_fda_required"
      :size="size ?? 24"
      weight="regular"
      aria-hidden="true"
   />
   <PhFolderSimpleUser
      v-else-if="!item.is_file && item.is_protected"
      :size="size ?? 24"
      weight="regular"
      aria-hidden="true"
   />
   <PhFolderSimple
      v-else-if="!item.is_file"
      :size="size ?? 24"
      weight="regular"
      aria-hidden="true"
   />
   <component
      v-else
      :is="fileIconComponent(item.name)"
      :size="size ?? 24"
      weight="regular"
      aria-hidden="true"
   />
</template>
