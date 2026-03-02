![Mac User Lens](./icons/128x128@2x.png)

# Mac User Lens

A macOS tool to easily identify and get rid of big, unused files and folders in seconds.

## Why?

Over time, your home folder quietly fills up with forgotten caches, old installers, duplicate images, leftover app data, and all sorts of files you didn't even know were there. macOS doesn't make it easy to figure out where all that space went.

Mac User Lens scans your entire user folder and presents everything as a navigable, size-sorted tree. You can drill into any directory, immediately spot what's taking up the most space, and clean it up — all from a single window.

## Features

- **Parallel Rust I/O** — Directory scanning distributes I/O across all available CPU cores for maximum throughput
- **Smooth frontend** — Built with Vue 3 and fine-tuned for performance and fluid navigation powered by the native [View Transitions API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API)
- **Smart selection** — Three-state checkboxes with full parent-child logic, no double-counting of nested folders, and a clear preview of how much space you'll reclaim before deleting anything
- **Safe by design** — Critical macOS directories (Desktop, Documents, Library, etc.) are protected from deletion with path canonicalization to prevent bypass. Sensitive credential folders (`.ssh`, `.gnupg`, `.aws`, `.kube`) are completely excluded from scan results
- **10 languages** — English, Italian, Spanish, French, Portuguese, German, Russian, Chinese, Japanese, and Arabic (with RTL support)
- **Themes** — Multiple color themes to choose from, with more on the way

## Installation

### Homebrew

```bash
brew install --cask mac-user-lens
```

### Manual

1. Download the latest `.dmg` from [Releases](https://github.com/smastromattei/mac-user-lens/releases)
2. Drag the app to your Applications folder
3. Before opening, run this command in Terminal to bypass macOS Gatekeeper:

```bash
xattr -cr /Applications/Mac\ User\ Lens.app
```

> **Note** — The only official and trustworthy distribution of this app is the `.dmg` published in the [GitHub Releases](https://github.com/smastromattei/mac-user-lens/releases) page. Do not download it from any other source.

## Building from source

**Prerequisites:**

- [Xcode Command Line Tools](https://developer.apple.com/xcode/resources/) — `xcode-select --install`
- [Rust](https://www.rust-lang.org/tools/install) — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [Node.js](https://nodejs.org) >= 22
- [pnpm](https://pnpm.io) >= 10

```bash
# Clone the repository
git clone https://github.com/smastrom/mac-user-lens.git
cd mac-user-lens

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri:dev

# Add the universal macOS target
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# Build the universal macOS binary
pnpm tauri:build
```

## Credits

- [Phosphor Icons](https://phosphoricons.com) — UI icons
- [Hugeicons](https://hugeicons.com) — Mac User Lens app icon

## License

[MIT](./LICENSE) — Simone Mastromattei
