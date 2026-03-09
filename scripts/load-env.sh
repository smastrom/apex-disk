#!/usr/bin/env bash
# Load .env file and validate that all required Apple signing variables are set.
# Usage: source scripts/load-env.sh

set -euo pipefail

ENV_FILE=".env"

if [ ! -f "$ENV_FILE" ]; then
  echo "Error: $ENV_FILE not found. Create it with your Apple signing credentials." >&2
  exit 1
fi

set -a
. "./$ENV_FILE"
set +a

REQUIRED_VARS=(APPLE_SIGNING_IDENTITY APPLE_ID APPLE_PASSWORD APPLE_TEAM_ID)
MISSING=()

for var in "${REQUIRED_VARS[@]}"; do
  if [ -z "${!var:-}" ]; then
    MISSING+=("$var")
  fi
done

if [ ${#MISSING[@]} -gt 0 ]; then
  echo "Error: Missing required env vars in $ENV_FILE:" >&2
  for var in "${MISSING[@]}"; do
    echo "  - $var" >&2
  done
  exit 1
fi
