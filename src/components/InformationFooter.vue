<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
InformationFooter

Purpose: Footer block in InformationView showing app name, version, author, and links to release notes, license, donate, and website.

Props: none

Example:
 <InformationFooter />
-->

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

import { formatYearRange } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

import {
   APP_CREDITS,
   APP_LICENSE,
   APP_NAME,
   APP_VERSION,
   DONATE_URL,
   LICENSE_URL,
   RELEASE_NOTES_URL,
   WEBSITE_URL,
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

async function openDonate() {
   try {
      await openUrl(DONATE_URL)
   } catch (err) {
      console.error('Failed to open donate page:', err)
   }
}

async function openWebsite() {
   try {
      await openUrl(WEBSITE_URL)
   } catch (err) {
      console.error('Failed to open website:', err)
   }
}
</script>

<template>
   <footer class="InformationFooter-root">
      <p class="InformationFooter-line">
         <span class="InformationFooter-name">{{ APP_NAME }}</span>
         <span class="InformationFooter-version">v{{ APP_VERSION }}</span>
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
         <button type="button" class="InformationFooter-link" @click="openWebsite">
            {{ t('InformationFooter', 'website') }}
         </button>
         <span class="InformationFooter-sep" aria-hidden="true">·</span>
         <button type="button" class="InformationFooter-link" @click="openDonate">
            {{ t('InformationFooter', 'donate') }}
         </button>
      </div>
      <p class="InformationFooter-line InformationFooter-credits">
         © {{ formatYearRange(RELEASE_YEAR, new Date().getFullYear()) }} {{ APP_CREDITS }}
         <span class="InformationFooter-sep" aria-hidden="true">·</span>
         {{ APP_LICENSE }}
      </p>
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
   font-size: var(--font-size-base);
   color: var(--color-text-muted);
}

.InformationFooter-name {
   font-weight: 600;
   color: var(--color-text);
}

.InformationFooter-version {
   margin-left: var(--spacing-xs);
   font-weight: var(--font-weight-min);
   color: var(--color-text-muted);
}

.InformationFooter-credits {
   margin-top: var(--spacing-xxs);
   font-size: var(--font-size-xs);
}

.InformationFooter-links {
   display: flex;
   align-items: center;
   justify-content: center;
   gap: var(--spacing-xs);
   margin-top: var(--spacing-xs);
   margin-bottom: var(--spacing-xl);
   flex-wrap: wrap;
   padding: 0 var(--spacing-xl);
}

.InformationFooter-link {
   padding: 0;
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-accent);
   background: none;
   border: none;
   cursor: pointer;
   transition: color 0.2s var(--ease-standard);
   white-space: nowrap;

   &:hover {
      color: var(--color-accent-hover);
   }
}

.InformationFooter-sep {
   color: var(--color-text-dim);
   font-weight: var(--font-weight-min);
   pointer-events: none;
}
</style>
