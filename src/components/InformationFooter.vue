<!--
InformationFooter

Purpose: Footer block in InformationView showing app name, version, author, and links to release notes, license, and repository.

Props: none

Example:
 <InformationFooter />
-->

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

import { useTranslations } from '@/lib/use-translations'

import {
   APP_CREDITS,
   APP_NAME,
   APP_VERSION,
   AUTHOR_URL,
   LICENSE_URL,
   RELEASE_NOTES_URL,
   REPOSITORY_URL,
   DONATE_URL,
   RELEASE_YEAR,
} from '@/lib/constants'

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

async function openRepository() {
   try {
      await openUrl(REPOSITORY_URL)
   } catch (err) {
      console.error('Failed to open repository:', err)
   }
}

async function openDonate() {
   try {
      await openUrl(DONATE_URL)
   } catch (err) {
      console.error('Failed to open donate URL:', err)
   }
}
</script>

<template>
   <footer class="InformationFooter-root">
      <p class="InformationFooter-line">
         <span class="InformationFooter-name">{{ APP_NAME }}</span>
         <span class="InformationFooter-version">v{{ APP_VERSION }}</span>
      </p>
      <p class="InformationFooter-line InformationFooter-credits">
         <button type="button" class="InformationFooter-creditsBtn" @click="openAuthorUrl">
            {{ RELEASE_YEAR }} - {{ APP_CREDITS }}
         </button>
      </p>
      <div class="InformationFooter-links">
         <button type="button" class="InformationFooter-link" @click="openReleaseNotes">
            {{ t('InformationFooter', 'releaseNotes') }}
         </button>
         <span class="InformationFooter-sep" aria-hidden="true">·</span>
         <button type="button" class="InformationFooter-link" @click="openLicense">
            {{ t('InformationFooter', 'license') }}
         </button>
         <span class="InformationFooter-sep" aria-hidden="true">·</span>
         <button type="button" class="InformationFooter-link" @click="openRepository">
            {{ t('InformationFooter', 'repository') }}
         </button>
         <span class="InformationFooter-sep" aria-hidden="true">·</span>
         <button type="button" class="InformationFooter-link" @click="openDonate">
            {{ t('InformationFooter', 'donate') }}
         </button>
      </div>
   </footer>
</template>

<style scoped>
.InformationFooter-root {
   margin-top: var(--spacing-xl);
   padding-top: var(--spacing-lg);
   border-top: 1px solid var(--color-surface);
   text-align: center;
}

.InformationFooter-line {
   margin: 0;
   font-size: 0.8125rem;
   color: var(--color-text-muted);
}

.InformationFooter-name {
   font-weight: 600;
   color: var(--color-text);
}

.InformationFooter-version {
   margin-left: var(--spacing-xs);
   font-weight: 400;
   color: var(--color-text-muted);
}

.InformationFooter-credits {
   margin-top: var(--spacing-xxs);
}

.InformationFooter-creditsBtn {
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

.InformationFooter-links {
   display: flex;
   align-items: center;
   justify-content: center;
   gap: var(--spacing-xs);
   margin-top: var(--spacing-md);
}

.InformationFooter-link {
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

.InformationFooter-sep {
   color: var(--color-text-dim);
   font-weight: 400;
   pointer-events: none;
}
</style>
