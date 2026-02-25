<!--
SettingsView

Purpose: Settings screen with Language, Theme, Show hidden files, Show 0B files. macOS-style grouped list.

Props: none (uses inject for settings store)

Example:
 <SettingsView />
-->

<script setup lang="ts">
import { inject, computed, type Ref } from 'vue'

import { useTranslations } from '@/lib/useTranslations'
import { SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { AppSettings, Language, ThemeColor } from '@/types/settings'

const { t } = useTranslations()

const storeRef = inject<Ref<SettingsStore | null>>(SETTINGS_KEY)
const store = computed(() => storeRef?.value ?? null)
const settings = computed((): AppSettings | null => store.value?.settings.value ?? null)

const languageOptions = computed(() => [
   { value: 'en' as Language, label: t('SettingsView', 'languageEn') },
   { value: 'it' as Language, label: t('SettingsView', 'languageIt') },
])

const themeOptions = computed(() => [
   { value: 'oceanic' as ThemeColor, label: t('SettingsView', 'themeOceanic') },
   { value: 'light' as ThemeColor, label: t('SettingsView', 'themeLight') },
   { value: 'dark' as ThemeColor, label: t('SettingsView', 'themeDark') },
   { value: 'tokyo-night' as ThemeColor, label: t('SettingsView', 'themeTokyoNight') },
   { value: 'ayu-dark' as ThemeColor, label: t('SettingsView', 'themeAyuDark') },
   { value: 'ayu-mirage' as ThemeColor, label: t('SettingsView', 'themeAyuMirage') },
   { value: 'dracula' as ThemeColor, label: t('SettingsView', 'themeDracula') },
   { value: 'catppuccin' as ThemeColor, label: t('SettingsView', 'themeCatppuccin') },
   { value: 'gruvbox' as ThemeColor, label: t('SettingsView', 'themeGruvbox') },
   { value: 'nord' as ThemeColor, label: t('SettingsView', 'themeNord') },
   { value: 'solarized' as ThemeColor, label: t('SettingsView', 'themeSolarized') },
   { value: 'one-dark' as ThemeColor, label: t('SettingsView', 'themeOneDark') },
   { value: 'deep-purple' as ThemeColor, label: t('SettingsView', 'themeDeepPurple') },
   { value: 'kanagawa' as ThemeColor, label: t('SettingsView', 'themeKanagawa') },
])

function toggleHiddenFiles() {
   if (store.value && settings.value)
      store.value.setShowHiddenFiles(!settings.value.showHiddenFiles)
}

function toggleZeroByteFiles() {
   if (store.value && settings.value)
      store.value.setShowZeroByteFiles(!settings.value.showZeroByteFiles)
}

function toggleZeroByteFolders() {
   if (store.value && settings.value)
      store.value.setShowZeroByteFolders(!settings.value.showZeroByteFolders)
}

function toggleAnimations() {
   if (store.value && settings.value)
      store.value.setEnableAnimations(!settings.value.enableAnimations)
}
</script>

<template>
   <main class="SettingsView-root">
      <div v-if="!settings" class="SettingsView-loading">
         {{ t('SettingsView', 'loadingSettings') }}
      </div>
      <div v-else class="SettingsView-content">
         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'language') }}</span>
               <select
                  class="SettingsSelect"
                  :value="settings.language"
                  @change="store?.setLanguage(($event.target as HTMLSelectElement).value as Language)"
               >
                  <option v-for="opt in languageOptions" :key="opt.value" :value="opt.value">
                     {{ opt.label }}
                  </option>
               </select>
            </div>
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'themeColor') }}</span>
               <select
                  class="SettingsSelect"
                  :value="settings.themeColor"
                  @change="store?.setThemeColor(($event.target as HTMLSelectElement).value as ThemeColor)"
               >
                  <option v-for="opt in themeOptions" :key="opt.value" :value="opt.value">
                     {{ opt.label }}
                  </option>
               </select>
            </div>
         </section>

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'showHiddenFiles') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.showHiddenFiles }"
                  :aria-pressed="settings.showHiddenFiles"
                  @click="toggleHiddenFiles"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'showZeroByteFiles') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.showZeroByteFiles }"
                  :aria-pressed="settings.showZeroByteFiles"
                  @click="toggleZeroByteFiles"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'showZeroByteFolders') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.showZeroByteFolders }"
                  :aria-pressed="settings.showZeroByteFolders"
                  @click="toggleZeroByteFolders"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
         </section>

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'enableAnimations') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.enableAnimations }"
                  :aria-pressed="settings.enableAnimations"
                  @click="toggleAnimations"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
         </section>
      </div>
   </main>
</template>

<style scoped>
.SettingsView-root {
   flex: 1;
   overflow-y: auto;
   padding: var(--spacing-md);
   background: var(--color-bg);
}

.SettingsView-loading {
   display: flex;
   align-items: center;
   justify-content: center;
   padding: var(--spacing-xl);
   color: var(--color-text-muted);
}

.SettingsView-content {
   max-width: var(--content-max-width);
   margin: 0 auto;
}
</style>
