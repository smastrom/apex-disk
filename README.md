# Mac User Lens

Mac User Lens is a tool that helps you easily identify and get rid of big, unused files and folders in seconds.

## Getting started

### Dependencies

- **Node.js** and **pnpm** (>= 10)
- **Rust** (stable toolchain)
- **macOS** (the app targets macOS only)

For a universal build (Intel + Apple Silicon), install both macOS targets:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

### Development

Install dependencies and start the dev server (opens the Tauri app with hot-reload):

```bash
pnpm install
pnpm tauri dev
```

### Build

Build the app for release (universal macOS binary + DMG):

```bash
pnpm tauri:build
```

Outputs are in `src-tauri/target/universal-apple-darwin/release/bundle/` (`.app` and `.dmg`).