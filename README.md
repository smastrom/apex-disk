![Mac User Lens](./src-tauri/icons/128x128.png)

# Mac User Lens

Tool for macOS to easily identify and get rid of big, unused files and folders in seconds.

<!-- ![Mac User Lens](./public/screenshot-2.png) -->

<br />

## Development

### 1. Dependencies

- **macOS** (the app targets macOS only)
- **Node.js** and **pnpm** (>= 10)
- **Rust** (stable toolchain)

### 2. Dev server

Install dependencies and start the dev server (opens the Tauri app with hot-reload):

```bash
pnpm install
pnpm tauri dev
```

### 3. Release build

For a universal build (Intel + Apple Silicon), install both macOS targets:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

Build the app for release (universal macOS binary + DMG):

```bash
pnpm tauri:build
```

Outputs are in `src-tauri/target/universal-apple-darwin/release/bundle/` (`.app` and `.dmg`).
