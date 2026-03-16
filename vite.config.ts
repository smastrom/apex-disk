import path from 'path'

import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

const host = process.env.TAURI_DEV_HOST

// Safari version matching minimumSystemVersion in tauri.conf.json (macOS 10.15 = Safari 13)
// Version encoding: (major << 16) | (minor << 8) | patch
const safari13 = (13 << 16) | (0 << 8) | 0

export default defineConfig(async () => ({
   root: 'src',
   plugins: [vue()],
   resolve: {
      alias: {
         '@': path.resolve(__dirname, 'src'),
      },
   },

   /* css: {
      transformer: 'lightningcss',
      lightningcss: {
         targets: {
            safari: safari13,
         },
      },
   }, */

   build: {
      // Target Safari 13 (macOS 10.15 Catalina WKWebView)
      target: 'safari13',
   },

   // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
   //
   // 1. prevent Vite from obscuring rust errors
   clearScreen: true,
   // 2. tauri expects a fixed port, fail if that port is not available
   server: {
      port: 1420,
      strictPort: true,
      host: host || false,
      hmr: host
         ? {
              protocol: 'ws',
              host,
              port: 1421,
           }
         : undefined,
      watch: {
         // 3. tell Vite to ignore watching `src-tauri`
         ignored: ['**/src-tauri/**'],
      },
   },
}))
