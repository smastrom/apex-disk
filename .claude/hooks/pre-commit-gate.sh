#!/usr/bin/env sh
# Pre-commit gate.
#
# Blocks agent-initiated `git commit` and `git push` so they go through `/sync`,
# which enforces the docs sweep + test gates from
# .claude/rules/pre-commit-protocol.md.
#
# `/sync` opts in by creating .claude/.sync-active at its start and removing it
# at its end. When that marker is present, the hook lets the command through.
#
# Wired in .claude/settings.json as a PreToolUse hook on Bash.
set -eu

input=$(cat)
cmd=$(printf '%s' "$input" | python3 -c 'import sys,json;print(json.load(sys.stdin).get("tool_input",{}).get("command",""))' 2>/dev/null || printf '')

# Match `git commit` / `git push` only as an actual command invocation:
# preceded by start-of-string or a shell separator (whitespace, ; & | ( { ),
# and followed by whitespace or end-of-string. This avoids matching the
# substring inside echo/grep/JSON payloads.
if printf '%s' "$cmd" | grep -qE '(^|[[:space:];&|({])git[[:space:]]+(commit|push)([[:space:]]|$)'; then
   if [ -f .claude/.sync-active ]; then
      exit 0
   fi
   cat >&2 <<'MSG'
Pre-commit gate blocked this command.

This repo requires `/sync` for every commit + push (see
.claude/rules/pre-commit-protocol.md). `/sync` runs the docs sweep,
license-header check, typecheck, unit and e2e gates, commits in logical
groups, and pushes.

Run `/sync` instead. To bypass intentionally, ask the user first.
MSG
   exit 2
fi

exit 0
