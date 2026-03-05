![MacDiskTree](./src-tauri/icons/128x128.png)

# MacDiskTree

A macOS tool to easily identify and get rid of big, unused files and folders in seconds.

## Why?

Over time, your home folder quietly fills up with forgotten caches, old installers, duplicate images, leftover app data, and all sorts of files you didn't even know were there. macOS doesn't make it easy to figure out where all that space went.

MacDiskTree scans your entire user folder and presents everything as a navigable, size-sorted tree. You can drill into any directory, immediately spot what's taking up the most space, and clean it up — all from a single window.

## Features

- **Hyper-fast scanning** — Directory scanning distributes I/O across all available CPU cores for maximum throughput
- **Smooth UI** — Performance-minded interface with fluid animations, a clean design, and snappy navigation
- **Smart selection** — Three-state checkboxes with full parent-child logic and a real-time preview of how much space you'll reclaim
- **Safe by design** — Uses "Move to Trash" and protects reserved macOS folders (Desktop, Documents, Library, etc.) from root deletion. Sensitive credential folders (.ssh, .aws, etc.) are completely ignored
- **Optional Full Disk Access** — Works without extra permissions by default, but you can grant access to bypass system prompts or manage restricted folders
- **10 languages** — Support for English, Italian, Spanish, French, Portuguese, German, Russian, Chinese, Japanese, and Arabic (with RTL support)
- **Accessible** — Engineered for everyone with complete keyboard navigation and screen reader support
- **Themes** — Multiple color themes to choose from, with more on the way

> [!WARNING]  
> This app can delete files and folders. Use it carefully and review your selections before confirming deletion. The author does not take responsibility for any file loss.

## Preview

![MacDiskTree](./src/assets/images/app-cover.png)

## Installation

### Manual

1. Download the latest `.dmg` from [Releases](https://github.com/smastrom/mac-disk-tree/releases)
2. Drag the app to your Applications folder
3. Before opening, run this command in Terminal to bypass macOS Gatekeeper:

```bash
xattr -cr /Applications/MacDiskTree.app
```

> [!NOTE]
> **Only install from [GitHub Releases](https://github.com/smastrom/mac-disk-tree/releases).** Copies from other sites or app stores may be tampered with (malware, backdoors). There is no other official distribution channel.

## Building from source

**Prerequisites:**

- [Xcode Command Line Tools](https://developer.apple.com/xcode/resources/) — `xcode-select --install`
- [Rust](https://www.rust-lang.org/tools/install) — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [Node.js](https://nodejs.org) >= 22
- [pnpm](https://pnpm.io) >= 10

### Local build (no signing credentials)

```bash
# Clone the repository
git clone https://github.com/smastrom/mac-disk-tree.git
cd mac-disk-tree

# Install dependencies
pnpm install

# Add the universal macOS target
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# Build (skips updater artifacts so no signing key is required)
pnpm tauri:build

# Ad-hoc sign the .app so it runs with the correct bundle ID and entitlements
./scripts/codesign.sh
```

The built app is at `src-tauri/target/universal-apple-darwin/release/bundle/macos/MacDiskTree.app`. This build does not produce updater artifacts (no in-app update); it is for local use and development.

## License

[MIT](./LICENSE)
