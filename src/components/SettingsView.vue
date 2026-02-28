<!--
SettingsView

Purpose: Settings screen with Language, Theme, Show hidden files, Show 0B files, and Permissions. macOS-style grouped list.

Props: fdaGranted (boolean)

Example:
 <SettingsView :fdaGranted="fdaGranted" />
-->

<script setup lang="ts">
import { PhCaretDown, PhCheckCircle, PhXCircle, PhWrench as PhGearSix } from '@phosphor-icons/vue'

import { inject, computed, type Ref } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'

import { useTranslations } from '@/lib/useTranslations'

import { SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { AppSettings, Language, ThemeColor } from '@/types/settings'

defineProps<{
   fdaGranted: boolean
}>()

const { t } = useTranslations()

const storeRef = inject<Ref<SettingsStore | null>>(SETTINGS_KEY)
const store = computed(() => storeRef?.value ?? null)
const settings = computed((): AppSettings | null => store.value?.settings.value ?? null)

const languageOptions = computed(() => [
   { value: 'en' as Language, label: t('SettingsView', 'languageEn') },
   { value: 'it' as Language, label: t('SettingsView', 'languageIt') },
])

const themeOptions = computed(() => [
   { value: 'mac-user-lens' as ThemeColor, label: t('SettingsView', 'themeMacUserLens') },
   { value: 'ayu' as ThemeColor, label: t('SettingsView', 'themeAyu') },
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

async function openSystemSettings() {
   await openUrl('x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles')
}
</script>

<template>
   <main class="SettingsView-root">
      <div v-if="!settings" class="SettingsView-loading">
         {{ t('SettingsView', 'loadingSettings') }}
      </div>
      <div v-else class="SettingsView-content">
         <!-- FDA -->

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'fdaLabel') }}</span>
               <span class="SettingsView-fdaStatus" :class="fdaGranted ? 'is-ok' : 'is-denied'">
                  <PhCheckCircle v-if="fdaGranted" :size="13" weight="fill" aria-hidden="true" />
                  <PhXCircle v-else :size="13" weight="fill" aria-hidden="true" />
                  {{
                     fdaGranted ? t('SettingsView', 'fdaGranted') : t('SettingsView', 'fdaMissing')
                  }}
               </span>
            </div>
            <template v-if="!fdaGranted">
               <p class="SettingsView-fdaDesc">
                  {{ t('SettingsView', 'fdaDesc') }}
               </p>

               <div class="SettingsView-fdaControls">
                  <button type="button" class="SettingsView-fdaBtn" @click="openSystemSettings">
                     <PhGearSix :size="13" weight="fill" aria-hidden="true" />
                     {{ t('SettingsView', 'fdaOpenSettings') }}
                  </button>
               </div>
            </template>
         </section>

         <!-- App Settings -->

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'language') }}</span>
               <div class="SettingsView-selectWrap">
                  <select
                     class="SettingsSelect"
                     :value="settings.language"
                     @change="
                        store?.setLanguage(($event.target as HTMLSelectElement).value as Language)
                     "
                  >
                     <option v-for="opt in languageOptions" :key="opt.value" :value="opt.value">
                        {{ opt.label }}
                     </option>
                  </select>
                  <PhCaretDown
                     :size="14"
                     weight="regular"
                     class="SettingsView-selectChevron"
                     aria-hidden="true"
                  />
               </div>
            </div>
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'themeColor') }}</span>
               <div class="SettingsView-selectWrap">
                  <select
                     class="SettingsSelect"
                     :value="settings.themeColor"
                     @change="
                        store?.setThemeColor(
                           ($event.target as HTMLSelectElement).value as ThemeColor
                        )
                     "
                  >
                     <option v-for="opt in themeOptions" :key="opt.value" :value="opt.value">
                        {{ opt.label }}
                     </option>
                  </select>
                  <PhCaretDown
                     :size="14"
                     weight="regular"
                     class="SettingsView-selectChevron"
                     aria-hidden="true"
                  />
               </div>
            </div>
         </section>

         <!-- Results -->

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
               <span class="SettingsGroup-label">{{
                  t('SettingsView', 'showZeroByteFolders')
               }}</span>
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

         <!-- Animations -->

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

.SettingsView-selectWrap {
   position: relative;
   display: inline-block;

   .SettingsSelect {
      background-image: none;
   }
}

.SettingsView-selectChevron {
   position: absolute;
   right: 10px;
   top: 50%;
   transform: translateY(-50%);
   pointer-events: none;
   color: var(--color-text-muted);
}

/* FDA row */
.SettingsView-fdaStatus {
   display: flex;
   align-items: center;
   gap: 5px;
   font-size: 0.8125rem;
   font-weight: 500;
}

.SettingsView-fdaStatus.is-ok {
   color: var(--color-success, #22c55e);
}

.SettingsView-fdaStatus.is-denied {
   color: var(--color-danger, #ef4444);
}

.SettingsView-fdaControls {
   display: flex;
   justify-content: space-between;
   gap: 6px;
   padding: 0 var(--spacing-lg);
   margin-bottom: var(--spacing-md);
}

.SettingsView-fdaBtn {
   display: inline-flex;
   align-items: center;
   gap: 4px;
   padding: 4px 10px;
   border: 1px solid var(--color-border);
   border-radius: 5px;
   background: var(--color-bg);
   color: var(--color-text);
   font-size: 0.75rem;
   font-weight: 500;
   cursor: pointer;
   transition: opacity 0.15s ease;
}

.SettingsView-fdaBtn:hover:not(:disabled) {
   opacity: 0.75;
}

.SettingsView-fdaBtn:disabled {
   opacity: 0.45;
   cursor: default;
}

.SettingsView-fdaDesc {
   margin: 0;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: 0.75rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}
</style>
