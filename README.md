![ApexDisk](./src-tauri/icons/128x128.png)

# ApexDisk for Mac

macOS tool to easily identify and get rid of big, unused files and folders in seconds.

![ApexDisk Cover](./src/assets/images/apex-disk-cover.png)

## What is ApexDisk?

Automatic cleaners only know about a fixed set of cache folders, and the decisions they make are limited to what they were preprogrammed for.

ApexDisk finds and surfaces everything else they skip: caches left behind by apps you've uninstalled or lesser-known ones no cleaner looks for, leftover installers and SDKs, stopped Docker containers, and other junk you never knew was eating your disk.

It scans your user folder and lays it out as a size-sorted tree, so the heaviest items show up first. Drill into any folder, select what you don't need, and send it all to the Trash from a single window.

Visit the [ApexDisk Website](https://apexdisk.app) for more information.

## Features

- **Hyper-fast scanning:** Multi-core Rust engine builds the disk tree in seconds
- **Safe by default:** Files move to Trash, system folders stay protected, sensitive directories skipped automatically
- **Built to navigate:** Size-sorted tree with last-modified dates puts the heaviest folders first
- **Optional Full Disk Access:** Works without it by default, prompts only when needed
- **10 languages, 8 color themes:** Including Chinese, Japanese, and Arabic

## Installation

Download the latest `.dmg` (~5MB) from [Releases](https://github.com/smastrom/apex-disk/releases) and drag the app to your Applications folder.

## Building from source

**Prerequisites:**

- [Xcode Command Line Tools](https://developer.apple.com/xcode/resources/): `xcode-select --install`
- [Rust](https://www.rust-lang.org/tools/install): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [Node.js](https://nodejs.org) >= 22
- [pnpm](https://pnpm.io) >= 10

```bash
# Clone the repository
git clone https://github.com/smastrom/apex-disk.git
cd apex-disk

# Install dependencies
pnpm i

# Add target architectures, use `universal-apple-darwin` for a universal binary
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# Build unsigned binary (requires `xattr -cr /path/to/app.app` after building)
pnpm tauri:build:unsigned

# Build signed binary (requires Apple Developer ID and signing credentials)
pnpm tauri:build
```

## Local Development

```bash
# Clone the repository
git clone https://github.com/smastrom/apex-disk.git
cd apex-disk

# Install dependencies
pnpm i

# Run the development server
pnpm tauri:dev
```

## Support ApexDisk

Enjoying ApexDisk? Support the author with a donation:

- [PayPal](https://www.paypal.com/donate/?hosted_button_id=93WKXA68W9WQJ)
- [Buy Me a Coffee](https://buymeacoffee.com/smastrom)
- [Crypto (NOWPayments)](https://nowpayments.io/donation/smastrom) (BTC, ETH, USDT, etc.)

## Contributing

Pull requests and bug reports are welcome. Open PRs against the **`development`** branch (the repo default). Day-to-day work lands there; `main` is the release line and is updated when a version ships.

## License

Copyright (C) 2026 Simone Mastromattei. This project is licensed under the [GNU General Public License v3.0](./LICENSE) (GPL-3.0).
