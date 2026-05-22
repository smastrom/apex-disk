#!/usr/bin/env sh
# Pre-commit gate.
#
# Blocks agent-initiated `git commit` and `git push` so they go through `/sync`,
# which enforces the docs sweep + test gates from
# .claude/rules/agent-commit-protocol.md.
#
# `/sync` opts in by creating .claude/.sync-active at its start and removing it
# at its end. When that marker is present, the hook lets the command through.
#
# Wired in .claude/settings.json (PreToolUse on Bash) and .cursor/hooks.json
# (beforeShellExecution).
set -eu

input=$(cat)
cmd=$(printf '%s' "$input" | python3 -c '
import sys, json
d = json.load(sys.stdin)
print(d.get("command") or d.get("tool_input", {}).get("command") or "")
' 2>/dev/null || printf '')

is_cursor=$(printf '%s' "$input" | python3 -c '
import sys, json
d = json.load(sys.stdin)
print("1" if isinstance(d.get("command"), str) else "0")
' 2>/dev/null || printf '0')

# Match `git commit` / `git push` only as an actual command invocation:
# preceded by start-of-string or a shell separator (whitespace, ; & | ( { ),
# and followed by whitespace or end-of-string. This avoids matching the
# substring inside echo/grep/JSON payloads.
if printf '%s' "$cmd" | grep -qE '(^|[[:space:];&|({])git[[:space:]]+(commit|push)([[:space:]]|$)'; then
   if [ -f .claude/.sync-active ]; then
      if [ "$is_cursor" = "1" ]; then
         printf '%s\n' '{"permission":"allow"}'
      fi
      exit 0
   fi
   if [ "$is_cursor" = "1" ]; then
      cat <<'EOF'
{"permission":"deny","user_message":"Pre-commit gate blocked this command.\n\nThis repo requires /sync for every agent commit + push (see .claude/rules/agent-commit-protocol.md). /sync groups commits, runs the doc/test sweep, typecheck, headers, and unit tests, then pushes. E2E runs in CI (/sync skips it locally).\n\nRun /sync instead. To bypass intentionally, ask the user first.","agent_message":"Pre-commit gate blocked git commit/push. Run /sync instead (see .claude/rules/agent-commit-protocol.md and reference/agent-workflow.md). To bypass intentionally, ask the user first."}
EOF
      exit 0
   fi
   cat >&2 <<'MSG'
Pre-commit gate blocked this command.

This repo requires `/sync` for every commit + push (see
.claude/rules/agent-commit-protocol.md). `/sync` groups commits, runs the
doc/test sweep, typecheck, headers, and unit tests, then pushes. E2E runs
in CI (`/sync` skips it locally).

Run `/sync` instead. To bypass intentionally, ask the user first.
MSG
   exit 2
fi

if [ "$is_cursor" = "1" ]; then
   printf '%s\n' '{"permission":"allow"}'
fi
exit 0
