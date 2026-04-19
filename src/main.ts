import { APP_VERSION } from '@/lib/constants'
import { applyTheme, applyDirection } from '@/lib/dom'
import { initLog, log, registerDiagnosticHandlers } from '@/lib/log'
import { useSystemInfo } from '@/lib/use-system-info'
import { initTauriAppSettings } from '@/stores/app-settings'
import { createApp } from 'vue'

import AppShell from './components/AppShell.vue'

;(async () => {
   await initLog()

   const store = await initTauriAppSettings()

   applyTheme(store.getThemeColor())
   applyDirection(store.settings.value.language)

   // Log app environment for debug output
   try {
      const { systemInfo } = await useSystemInfo()

      if (systemInfo.value) {
         const { macos_version, cpu_info } = systemInfo.value
         log('app', `App: v${APP_VERSION} — macOS ${macos_version}, ${cpu_info}`)
      } else {
         log('app', `App: v${APP_VERSION}`)
      }
   } catch {
      log('app', `App: v${APP_VERSION}`)
   }

   const app = createApp(AppShell)

   registerDiagnosticHandlers(app)
   app.mount('#app')
})()
