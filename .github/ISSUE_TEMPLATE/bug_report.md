---
name: Bug Report
about: Something isn't working as expected
title: ''
labels: bug
assignees: ''
---

**What happened?**
A clear description of the issue.

**Steps to reproduce**

1. …
2. …

**Expected behavior**
What you expected to happen instead.

**Environment**

- macOS version:
- ApexDisk version:
- Chip (Apple Silicon / Intel):

**Debug log (optional)**
If the issue is reproducible, relaunch the app from Terminal with debug logging enabled (GUI `open -a` does not pass custom environment variables—run the binary inside the bundle):

```bash
APEX_DISK_DEBUG=1 /Applications/ApexDisk.app/Contents/MacOS/ApexDisk
# or, from the repo: ./scripts/run-with-debug.sh
```

Prefix reference: **`docs/LOGGING.md`**. In the log paste: `[apex:vue:…]` = intentional TS logs; `[apex:vue/…]` = uncaught web errors (stderr); `[apex:rust:…]` = Rust IPC / updater. Unprefixed lines may be Tauri/Wry.

Paste the relevant log output here.

**Screenshots (optional)**
If applicable, add screenshots to help explain the issue.
