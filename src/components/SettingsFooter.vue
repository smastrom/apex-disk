<!--
SettingsFooter

Purpose: Footer block in Settings showing app name, version, author, and links to release notes and license.

Props: none

Example:
 <SettingsFooter />
-->

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

import { useTranslations } from '@/lib/useTranslations'

import { APP_CREDITS, APP_NAME, AUTHOR_URL, LICENSE_URL, RELEASE_NOTES_URL } from '@/lib/constants'

import Package from '../../package.json'

const { t } = useTranslations()

async function openReleaseNotes() {
   try {
      await openUrl(RELEASE_NOTES_URL)
   } catch (err) {
      console.error('Failed to open release notes:', err)
   }
}

async function openLicense() {
   try {
      await openUrl(LICENSE_URL)
   } catch (err) {
      console.error('Failed to open license:', err)
   }
}

async function openAuthorUrl() {
   try {
      await openUrl(AUTHOR_URL)
   } catch (err) {
      console.error('Failed to open author URL:', err)
   }
}
</script>

<template>
   <footer class="SettingsFooter-root">
      <p class="SettingsFooter-line">
         <span class="SettingsFooter-name">{{ APP_NAME }}</span>
         <span class="SettingsFooter-version">v{{ Package.version }}</span>
      </p>
      <p class="SettingsFooter-line SettingsFooter-credits">
         <button type="button" class="SettingsFooter-creditsBtn" @click="openAuthorUrl">
            {{ APP_CREDITS }}
         </button>
      </p>
      <div class="SettingsFooter-links">
         <button type="button" class="SettingsFooter-link" @click="openReleaseNotes">
            {{ t('SettingsFooter', 'releaseNotes') }}
         </button>
         <span class="SettingsFooter-sep" aria-hidden="true">·</span>
         <button type="button" class="SettingsFooter-link" @click="openLicense">
            {{ t('SettingsFooter', 'license') }}
         </button>
      </div>
   </footer>
</template>

<style scoped>
.SettingsFooter-root {
   margin-top: var(--spacing-xl);
   padding-top: var(--spacing-lg);
   border-top: 1px solid var(--color-surface);
   text-align: center;
}

.SettingsFooter-line {
   margin: 0;
   font-size: 0.8125rem;
   color: var(--color-text-muted);
}

.SettingsFooter-name {
   font-weight: 600;
   color: var(--color-text);
}

.SettingsFooter-version {
   margin-left: var(--spacing-xs);
   font-weight: 400;
   color: var(--color-text-muted);
}

.SettingsFooter-credits {
   margin-top: var(--spacing-xxs);
}

.SettingsFooter-creditsBtn {
   padding: 0;
   font-size: inherit;
   font-weight: inherit;
   color: inherit;
   background: none;
   border: none;
   cursor: pointer;
   transition: color 0.2s;

   &:hover {
      color: var(--color-accent);
   }
}

.SettingsFooter-links {
   display: flex;
   align-items: center;
   justify-content: center;
   gap: var(--spacing-xs);
   margin-top: var(--spacing-sm);
}

.SettingsFooter-link {
   padding: 0;
   font-size: 0.8125rem;
   font-weight: 500;
   color: var(--color-accent);
   background: none;
   border: none;
   cursor: pointer;
   transition: color 0.2s;

   &:hover {
      color: var(--color-accent-hover);
   }
}

.SettingsFooter-sep {
   color: var(--color-text-dim);
   font-weight: 400;
   pointer-events: none;
}
</style>
