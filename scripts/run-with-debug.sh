#!/usr/bin/env bash
# Run a packaged ApexDisk build with verbose logging (frontend + updater diagnostics).
# macOS does not pass custom env vars through `open -a`, so invoke the binary inside the .app.
#
# Usage:
#   ./scripts/run-with-debug.sh
#   ./scripts/run-with-debug.sh "/Applications/ApexDisk Nightly.app"

set -euo pipefail

APP_BUNDLE="${1:-/Applications/ApexDisk.app}"
PLIST="$APP_BUNDLE/Contents/Info.plist"

if [ ! -f "$PLIST" ]; then
   echo "Not an app bundle (missing Info.plist): $APP_BUNDLE" >&2
   exit 1
fi

EXEC_NAME=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST")
BINARY="$APP_BUNDLE/Contents/MacOS/$EXEC_NAME"

if [ ! -x "$BINARY" ]; then
   echo "Executable not found: $BINARY" >&2
   exit 1
fi

exec env APEX_DISK_DEBUG=1 "$BINARY"
