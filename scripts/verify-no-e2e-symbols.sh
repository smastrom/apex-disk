#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Verify a built ApexDisk binary excludes the e2e command symbols.
#
# The `e2e` cargo feature exposes `set_e2e_trash_mode` and `reset_e2e_state`
# over IPC. The `set_e2e_trash_mode "success"` branch returns optimistic
# count/size without touching disk, so a release artifact accidentally built
# with `--features e2e` would silently report successful trashing while
# leaving files in place. This script is the last line of defense against
# shipping such a binary.
#
# Usage:
#   scripts/verify-no-e2e-symbols.sh <path-to-ApexDisk binary>
#
# Exits 0 if the binary is clean, 1 otherwise.

set -euo pipefail

BIN="${1:-}"

if [ -z "$BIN" ]; then
   echo "Usage: $0 <path-to-binary>" >&2
   exit 1
fi

if [ ! -f "$BIN" ]; then
   echo "Error: binary not found at $BIN" >&2
   exit 1
fi

FOUND=$(strings "$BIN" | grep -E '(set_e2e_trash_mode|reset_e2e_state)' || true)

if [ -n "$FOUND" ]; then
   echo "::error::Binary $BIN contains e2e command symbols — built with --features e2e?" >&2
   echo "$FOUND" >&2
   exit 1
fi

echo "Verified: $BIN excludes e2e command symbols."
