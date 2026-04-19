import path from 'path'

import yaml from '@rollup/plugin-yaml'
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

const host = process.env.TAURI_DEV_HOST

// Safari version matching minimumSystemVersion in tauri.conf.json (macOS 10.15 = Safari 13)
// Version encoding: (major << 16) | (minor << 8) | patch
const safari13 = (13 << 16) | (0 << 8) | 0

export default defineConfig(() => ({
   root: 'src',
   plugins: [vue(), yaml()],
   resolve: {
      alias: {
         '@': path.resolve(__dirname, 'src'),
      },
   },

   css: {
      transformer: 'lightningcss' as const,
      lightningcss: {
         targets: {
            safari: safari13,
         },
      },
   },

   build: {
      target: 'safari13',
      cssMinify: 'lightningcss' as const,
      outDir: '../dist',
      emptyOutDir: true,
   },

   clearScreen: true,
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
         ignored: ['**/src-tauri/**'],
      },
   },
}))
