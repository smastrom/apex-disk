<!--
ScanResultsListItemIconSwitch

Purpose: Renders a single folder or file icon by type. Folders: PhFolderSimpleUser (protected) or PhFolderSimple. Files: by extension PhFileText, PhFileAudio, PhFileVideo, PhFileArchive, PhFileImage, or PhFile.

Props: item ({ name: string, is_file: boolean, is_protected?: boolean }), size (number?)

Example:
 <div class="ScanResultsListItem-icon" :class="{ 'ScanResultsListItem-icon--hidden': item.name.startsWith('.') }">
   <ScanResultsListItemIconSwitch :item="item" :size="28" />
 </div>
-->

<script setup lang="ts">
import {
   PhFolderSimple,
   PhFolderSimpleUser,
   PhFile,
   PhFileText,
   PhFileAudio,
   PhFileVideo,
   PhFileArchive,
   PhFileImage,
} from '@phosphor-icons/vue'

defineProps<{
   item: { name: string; is_file: boolean; is_protected?: boolean }
   size?: number
}>()

const DOC_EXTENSIONS = new Set([
   'pdf',
   'doc',
   'docx',
   'txt',
   'md',
   'rtf',
   'odt',
   'pages',
   'numbers',
   'key',
   'ppt',
   'pptx',
   'xls',
   'xlsx',
   'csv',
   'tex',
   'docm',
   'dotx',
   'dotm',
   'xlsm',
   'pptm',
   'ods',
   'odg',
   'odp',
   'odb',
   'odc',
   'odm',
   'odf',
])
const AUDIO_EXTENSIONS = new Set([
   'mp3',
   'wav',
   'aac',
   'flac',
   'm4a',
   'ogg',
   'wma',
   'aiff',
   'aif',
   'ape',
   'alac',
])
const VIDEO_EXTENSIONS = new Set([
   'mp4',
   'mov',
   'avi',
   'mkv',
   'webm',
   'wmv',
   'm4v',
   'flv',
   'mpg',
   'mpeg',
   '3gp',
])
const ARCHIVE_EXTENSIONS = new Set([
   'zip',
   'tar',
   'gz',
   'rar',
   '7z',
   'dmg',
   'bz2',
   'xz',
   'z',
   'lz',
   'lzma',
   'tgz',
   'tbz',
   'txz',
])
const IMAGE_EXTENSIONS = new Set([
   'jpg',
   'jpeg',
   'png',
   'gif',
   'webp',
   'svg',
   'bmp',
   'ico',
   'heic',
   'heif',
   'tiff',
   'tif',
])

/** Extracts lowercase extension from filename (e.g. "file.PDF" → "pdf"). */
function getExtension(name: string): string {
   const lastDot = name.lastIndexOf('.')
   if (lastDot === -1) return ''

   return name.slice(lastDot + 1).toLowerCase()
}

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
   <PhFolderSimpleUser
      v-if="!item.is_file && item.is_protected"
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
