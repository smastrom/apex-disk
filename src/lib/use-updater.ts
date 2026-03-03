import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

import { useTranslations } from '@/lib/use-translations'

export function useUpdater() {
   const { t } = useTranslations()

   const availableUpdate = ref<string | null>(null)

   let unlistenCheckUpdates: (() => void) | null = null

   onMounted(async () => {
      unlistenCheckUpdates = await listen('check-for-updates', onCheckForUpdates)

      // Silent check on startup
      try {
         const update = await check()
         availableUpdate.value = update?.version ?? null
      } catch {
         // Silent — don't bother the user if the check fails
      }
   })

   onUnmounted(() => {
      unlistenCheckUpdates?.()
   })

   async function onCheckForUpdates() {
      let update: Awaited<ReturnType<typeof check>>

      try {
         update = await check()
      } catch {
         update = null
      }

      availableUpdate.value = update?.version ?? null

      if (!update) {
         await invoke('show_message_dialog', {
            title: t('App', 'updateDialogTitle'),
            body: t('App', 'updateNone'),
            okLabel: 'OK',
         })
         return
      }

      const body = update.body ? `\n\n${update.body}` : ''

      const shouldDownload = await invoke<boolean>('show_ask_dialog', {
         title: t('App', 'updateAvailable'),
         body: t('App', 'updateVersion', { version: update.version }) + body,
         okLabel: t('App', 'updateDownload'),
         cancelLabel: t('App', 'updateLater'),
      })

      if (!shouldDownload) return

      await update.downloadAndInstall()

      const shouldRestart = await invoke<boolean>('show_ask_dialog', {
         title: t('App', 'updateDialogTitle'),
         body: t('App', 'updateRestart'),
         okLabel: t('App', 'updateRestartBtn'),
         cancelLabel: t('App', 'updateLater'),
      })

      if (shouldRestart) {
         await relaunch()
      }
   }

   return { availableUpdate }
}
