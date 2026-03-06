![MacDiskTree](./src-tauri/icons/128x128.png)

# MacDiskTree

A macOS tool to easily identify and get rid of big, unused files and folders in seconds.

## Why?

Over time, your home folder quietly fills up with forgotten caches, old installers, duplicate images, leftover app data, and all sorts of files you didn't even know were there. macOS doesn't make it easy to figure out where all that space went.

MacDiskTree scans your entire user folder and presents everything as a navigable, size-sorted tree. You can drill into any directory, immediately spot what's taking up the most space, and clean it up — all from a single window.

## Features

- **Hyper-fast scanning** — Directory scanning distributes I/O across all available CPU cores for maximum throughput
- **Smooth UI** — Performance-minded interface with fluid animations, a clean design, and snappy navigation
- **Smart UX** — Intelligent results to easily identify real trash with a navigable tree with full parent-child logic, and a real-time preview of how much space you'll reclaim
- **Safe by design** — Moves to Trash and protects reserved macOS folders (Desktop, Documents, Library, etc.) from root deletion. Sensitive folders (.ssh, .aws, etc.) are completely skipped.
- **Optional Full Disk Access** — Works without FDA by default, but you can grant access to bypass macOS permission prompts
- **10 languages** — Support for English, Italian, Spanish, French, Portuguese, German, Russian, Chinese, Japanese, and Arabic (with RTL support)
- **Accessible** — Engineered for everyone with complete keyboard navigation and screen reader support
- **Themes** — Multiple color themes to choose from, with more on the way

## Preview

![MacDiskTree](./src/assets/images/mac-disk-tree-screenshots.png)

## Installation

1. Download the latest `.dmg` from [Releases](https://github.com/smastrom/mac-disk-tree/releases)
2. Drag the app to your Applications folder
3. Before opening, run this command in Terminal to bypass macOS Gatekeeper:

```bash
xattr -cr /Applications/MacDiskTree.app
```

> Running the command is required; without it the app won't work.

> [!NOTE]
> **Only install from [GitHub Releases](https://github.com/smastrom/mac-disk-tree/releases).** Copies from unknown sources may be tampered. There is no other official distribution channel.

## Building from source

**Prerequisites:**

- [Xcode Command Line Tools](https://developer.apple.com/xcode/resources/) — `xcode-select --install`
- [Rust](https://www.rust-lang.org/tools/install) — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [Node.js](https://nodejs.org) >= 22
- [pnpm](https://pnpm.io) >= 10

```bash
# Clone the repository
git clone https://github.com/smastrom/mac-disk-tree.git
cd mac-disk-tree

# Install dependencies
pnpm install

# Add the universal macOS target
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# Build (skips updater artifacts so no signing key is required).
# The app is ad-hoc signed during the build so it runs with the correct bundle ID and entitlements.
pnpm tauri:build
```

## License

[MIT](./LICENSE)
