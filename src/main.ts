import { applyTheme, applyDirection } from '@/lib/document'
import { initLog } from '@/lib/log'
import { initTauriAppSettings } from '@/stores/app-settings'
import { createApp } from 'vue'

import AppShell from './components/AppShell.vue'

await initLog()
const store = await initTauriAppSettings()

applyTheme(store.getThemeColor())
applyDirection(store.settings.value.language)

const app = createApp(AppShell)
app.mount('#app')
