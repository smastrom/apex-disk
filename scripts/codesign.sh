#!/usr/bin/env bash
# Ad-hoc codesign the macOS .app bundle with entitlements and hardened runtime.
#
# The preferred approach is to set APPLE_SIGNING_IDENTITY="-" before running
# `tauri build` so Tauri signs the .app BEFORE packaging into DMG and .tar.gz.
# Both `pnpm tauri:build:release` and CI already do this.
#
# This script is a fallback for when someone runs `pnpm tauri:build` directly
# without the env var. It only signs the loose .app (not the DMG or .tar.gz).

set -euo pipefail

BUNDLE_DIR="src-tauri/target/universal-apple-darwin/release/bundle/macos"
APP="$BUNDLE_DIR/MacDiskTree.app"

if [ ! -d "$APP" ]; then
  echo "App bundle not found at $APP"
  exit 1
fi

ENTITLEMENTS="src-tauri/Entitlements.plist"

echo "Ad-hoc signing $APP with entitlements and hardened runtime..."
codesign --force --deep --sign - \
  --entitlements "$ENTITLEMENTS" \
  --options runtime \
  "$APP"

echo "Done. Verifying..."
codesign -dvvv "$APP" 2>&1 | grep -E "Identifier|flags|Info.plist|Runtime"
