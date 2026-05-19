// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

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
   const name = isWindows ? 'apex-disk.exe' : 'apex-disk'

   return path.join(base, name)
}

function getPnpmCommand(): { command: string; args: string[] } {
   const npmExecPath = process.env.npm_execpath

   if (npmExecPath) {
      return { command: process.execPath, args: [npmExecPath] }
   }

   return { command: 'pnpm', args: [] }
}

function runE2eBuild(): void {
   const pnpm = getPnpmCommand()
   const buildResult = spawnSync(
      pnpm.command,
      [...pnpm.args, 'tauri', 'build', '--debug', '--no-bundle', '--', '--features', 'e2e'],
      {
         cwd: rootDir,
         stdio: 'inherit',
      }
   )

   if (buildResult.error) {
      throw buildResult.error
   }

   if (buildResult.status !== 0) {
      throw new Error(`E2E: tauri build failed with exit code ${buildResult.status}`)
   }
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
      try {
         runE2eBuild()
      } catch (err) {
         console.error(err)
         process.exit(1)
      }
   },
   beforeSession() {
      exitClean = false

      const appPath = getAppPath()

      appProcess = spawn(appPath, [], {
         cwd: path.join(rootDir, 'src-tauri'),
         stdio: ['ignore', 'pipe', 'pipe'],
         detached: process.platform !== 'win32',
         env: {
            ...process.env,
            E2E_FDA: process.env.E2E_FDA || 'false',
         },
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
   process.once('exit', fn)
   process.once('SIGINT', () => {
      fn()
      process.exit(130)
   })
   process.once('SIGTERM', () => {
      fn()
      process.exit(143)
   })
   process.once('SIGHUP', () => {
      fn()
      process.exit(129)
   })
}

onShutdown(closeApp)
