import { spawn, spawnSync, type ChildProcess } from 'child_process'
import path from 'node:path'
import { fileURLToPath } from 'url'

const __dirname = fileURLToPath(new URL('.', import.meta.url))
const rootDir = path.resolve(__dirname, '..')

let appProcess: ChildProcess | null = null
let exitClean = false

function getAppPath(): string {
   const base = path.join(rootDir, 'src-tauri', 'target', 'debug')
   const isWindows = process.platform === 'win32'
   const name = isWindows ? 'mac-disk-tree.exe' : 'mac-disk-tree'
   return path.join(base, name)
}

export const config = {
   hostname: '127.0.0.1',
   port: 4445,
   path: '/',
   specs: ['./specs/**/*.spec.ts'],
   maxInstances: 1,
   capabilities: [
      {
         maxInstances: 1,
         browserName: 'chrome',
      },
   ],
   reporters: ['spec'],
   framework: 'mocha',
   mochaOpts: {
      ui: 'bdd',
      timeout: 60000,
   },
   onPrepare() {
      spawnSync('pnpm', ['tauri', 'build', '--debug', '--no-bundle', '--', '--features', 'e2e'], {
         cwd: rootDir,
         stdio: 'inherit',
         shell: true,
      })
   },
   beforeSession() {
      const appPath = getAppPath()
      appProcess = spawn(appPath, [], {
         cwd: path.join(rootDir, 'src-tauri'),
         stdio: ['ignore', 'pipe', 'pipe'],
         detached: process.platform !== 'win32',
      })

      appProcess.on('error', (err) => {
         console.error('E2E: failed to start app:', err)
         process.exit(1)
      })

      appProcess.on('exit', (code) => {
         if (!exitClean) {
            console.error('E2E: app exited with code', code)
            process.exit(1)
         }
      })

      return new Promise((resolve) => setTimeout(resolve, 3000))
   },
   afterSession() {
      closeApp()
   },
   onComplete() {
      closeApp()
   },
}

function closeApp(): void {
   exitClean = true

   if (appProcess) {
      try {
         if (appProcess.pid && process.platform !== 'win32') {
            process.kill(-appProcess.pid, 'SIGTERM')
         } else {
            appProcess.kill('SIGTERM')
         }
      } catch (_) {}

      appProcess = null
   }
}

function onShutdown(fn: () => void): void {
   const cleanup = () => {
      try {
         fn()
      } finally {
         process.exit()
      }
   }

   process.on('exit', cleanup)
   process.on('SIGINT', cleanup)
   process.on('SIGTERM', cleanup)
   process.on('SIGHUP', cleanup)
}

onShutdown(closeApp)
