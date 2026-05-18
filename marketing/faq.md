## Product

# What is ApexDisk?

A tool like ApexDisk is a must-have tool to get rid of useless files and leftovers, and make the most of your available storage.

It shows your entire Mac user folder as a size-sorted disk tree and lets you decide what to remove. Since navigation is driven by folder and file size, the heaviest items are easy to spot and to navigate.

# What does ApexDisk find that automatic Mac cleaners miss?

Automatic cleaners only know about a fixed set of cache folders, and the decisions they make are limited to what they were preprogrammed for.

ApexDisk is built for humans and surfaces everything they skip: the cache of a streaming app like Stremio, leftover installers from heavy apps like Xcode or Android Studio, Docker containers you stopped using, old node_modules from projects you no longer touch, or video files forgotten in obscure subfolders.

# How often should I use ApexDisk?

Use it when your disk is nearly full and you need to reclaim space fast, or run it periodically to catch what your automatic cleaner misses, especially on a busy Mac with many apps or heavy project folders. The size-sorted disk tree surfaces those leftovers in seconds.

## Installation

# How do I install ApexDisk on my Mac?

Download the latest .dmg from the [Releases](/releases) page or the [GitHub Releases](https://github.com/smastrom/apex-disk/releases) page, then drag the app to your Applications folder.

# How big is the ApexDisk installer?

ApexDisk is intentionally lightweight. The universal .dmg is around 10 MB to download, and the installed app takes roughly 20 MB on disk, so you are not giving up hundreds of megabytes just to reclaim them.

# Is ApexDisk safe to download and legitimate?

ApexDisk is Apple Notarized and code-signed, so macOS verifies its integrity and confirms it is free from known malware before launch. Every release is built and published directly from the official GitHub repository ([github.com/smastrom/apex-disk](https://github.com/smastrom/apex-disk)), so you can audit exactly what's in each build.

# Is ApexDisk really free? What's the catch?

There is no catch. ApexDisk is 100% free and open source under the GPL-3.0 license. There are no ads, no telemetry, no premium tiers, and no data collection.

## Features

# How fast is the disk tree scanning?

ApexDisk is built with Rust and uses a multi-core parallel I/O engine that distributes directory scanning across all available CPU cores, so the Mac disk tree is built far faster than tools written in slower languages. Even on Mac user folders with hundreds of thousands of files, scanning completes in seconds.

# Does ApexDisk need an internet connection?

No. Scanning your Mac user folder, building the disk tree, and moving files to Trash all happen locally, and your data never leaves your machine.

ApexDisk doesn't auto-check or auto-install updates: it only reaches the GitHub server when you explicitly decide to check for a new release, which is why firewall tools like Lulu or Little Snitch may show a permission prompt the first time you do. You can safely deny it and every feature of the disk tree viewer will keep working.

# Does ApexDisk require Full Disk Access?

No. ApexDisk works without Full Disk Access by default. When it encounters a protected folder, macOS shows its standard permission prompt so you can grant access case-by-case.

If you'd rather scan everything at once, you can optionally grant Full Disk Access in System Settings, but it is never required. It's also needed to delete certain system-managed app containers (like sandboxed app data in ~/Library/Containers) that macOS protects with special attributes.

# Why does ApexDisk only scan the user folder, and will it ever scan the full disk or external drives?

ApexDisk focuses exclusively on your Mac user folder because that is where virtually all recoverable space lives: caches, downloads, old projects, app data, and duplicate files. System folders live outside your home directory are managed by macOS.

Full-disk scanning and other volumes (such as external drives) may be considered in future releases, but they are not currently on the roadmap.

# Will ApexDisk ever delete my files permanently?

No. ApexDisk never deletes files directly, everything is moved to the Trash so you can always recover.

Reserved macOS system folders are protected from modification, and sensitive directories (like .ssh, .aws, and similar) are automatically excluded from scans. The app also shows you exactly how much space you will reclaim before you confirm.
