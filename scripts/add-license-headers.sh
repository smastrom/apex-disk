#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Simone Mastromattei

# Add SPDX license + copyright headers to first-party source files in-place.
# Idempotent: files already carrying an SPDX tag are left untouched.
#
# Usage:
#   ./scripts/add-license-headers.sh          # write headers
#   ./scripts/add-license-headers.sh --check  # exit 1 if any file is missing a header (for CI / pre-commit)

set -euo pipefail

SPDX="SPDX-License-Identifier: GPL-3.0-or-later"
COPYRIGHT="Copyright (C) 2026 Simone Mastromattei"

ROOTS=(src src-tauri/src e2e tests scripts)

CHECK_ONLY=0
if [ "${1:-}" = "--check" ]; then
   CHECK_ONLY=1
fi

missing=0
updated=0

write_header() {
   # $1 = output stream target (file path, opened for append).
   # $2 = extension.
   case "$2" in
      ts|tsx|rs) printf '// %s\n// %s\n\n' "$SPDX" "$COPYRIGHT" >> "$1" ;;
      vue)       printf '<!-- %s -->\n<!-- %s -->\n\n' "$SPDX" "$COPYRIGHT" >> "$1" ;;
      sh)        printf '# %s\n# %s\n\n' "$SPDX" "$COPYRIGHT" >> "$1" ;;
   esac
}

apply_header() {
   local file=$1 ext=$2
   # First ~5 lines already have the SPDX tag → skip.
   if head -n 5 "$file" | grep -q "SPDX-License-Identifier"; then
      return 0
   fi

   if [ "$CHECK_ONLY" -eq 1 ]; then
      echo "missing header: $file"
      missing=$((missing + 1))
      return 0
   fi

   local tmp
   tmp=$(mktemp)

   # Shell scripts: preserve the shebang on line 1, insert header after it.
   if [ "$ext" = "sh" ] && head -n 1 "$file" | grep -q '^#!'; then
      head -n 1 "$file" > "$tmp"
      write_header "$tmp" "$ext"
      tail -n +2 "$file" >> "$tmp"
   else
      : > "$tmp"
      write_header "$tmp" "$ext"
      cat "$file" >> "$tmp"
   fi

   # Preserve the original file's mode; `mv` would carry mktemp's 0600 over.
   chmod --reference="$file" "$tmp" 2>/dev/null || chmod "$(stat -f '%A' "$file" 2>/dev/null || stat -c '%a' "$file")" "$tmp"
   mv "$tmp" "$file"
   updated=$((updated + 1))
   echo "added header: $file"
}

for root in "${ROOTS[@]}"; do
   [ -d "$root" ] || continue
   while IFS= read -r -d '' file; do
      case "$file" in
         *.ts)  apply_header "$file" ts ;;
         *.tsx) apply_header "$file" tsx ;;
         *.vue) apply_header "$file" vue ;;
         *.rs)  apply_header "$file" rs ;;
         *.sh)  apply_header "$file" sh ;;
      esac
   done < <(find "$root" -type f \( -name '*.ts' -o -name '*.tsx' -o -name '*.vue' -o -name '*.rs' -o -name '*.sh' \) -print0)
done

if [ "$CHECK_ONLY" -eq 1 ]; then
   if [ "$missing" -gt 0 ]; then
      echo "$missing file(s) missing SPDX header. Run: pnpm headers" >&2
      exit 1
   fi
   echo "all source files carry an SPDX header"
else
   echo "done ($updated file(s) updated)"
fi
