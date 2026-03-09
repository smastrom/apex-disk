![MacDiskTree](./src-tauri/icons/128x128.png)

# MacDiskTree

A macOS tool to easily identify and get rid of big, unused files and folders in seconds.

## Why?

Over time, your home folder quietly fills up with forgotten caches, old installers, duplicate images, leftover app data, and all sorts of files you didn't even know were there. macOS doesn't make it easy to figure out where all that space went.

MacDiskTree scans your entire user folder and presents everything as a navigable, size-sorted tree. You can drill into any directory, immediately spot what's taking up the most space, and clean it up — all from a single window.

## Features

- **Hyper-fast scanning** — Directory scanning distributes I/O across all available CPU cores for maximum throughput
- **Smart UX** — Easily spot waste with a size-sorted tree and last-modified dates. See exactly how much space you'll save as you select files.
- **Safe by design** — Files are moved to the Trash, never deleted directly. Reserved system folders are protected, and sensitive directories (like .ssh or .aws) are automatically skipped.
- **Optional Full Disk Access** — Works without FDA by default. Granting it allows you to skip repetitive macOS permission prompts.
- **Smooth UI** — Performance-minded UI with fluid animations, a clean design, and snappy navigation
- **10 languages** — Support for English, Italian, Spanish, French, Portuguese, German, Russian, Chinese, Japanese, and Arabic (with RTL support)
- **Themes** — Multiple color themes to choose from, with more on the way

## Installation

1. Download the latest `.dmg` from [Releases](https://github.com/smastrom/mac-disk-tree/releases)
2. Drag the app to your Applications folder

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

# Build unsigned binary
pnpm tauri:build:unsigned

# Build signed binary (required APPLE_SIGNING_IDENTITY, APPLE_ID, APPLE_PASSWORD, APPLE_TEAM_ID environment variables)
pnpm tauri:build
```

## Local Development

```bash
# Clone the repository
git clone https://github.com/smastrom/mac-disk-tree.git
cd mac-disk-tree

# Install dependencies
pnpm install

# Run the development server
pnpm tauri:dev
```

## License

[MIT](./LICENSE)
