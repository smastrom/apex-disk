# Security Policy

## Reporting a vulnerability

**Please do not open public GitHub issues for security-sensitive bugs.**

If you believe you've found a vulnerability in ApexDisk (anything that could compromise user data, bypass macOS sandboxing or entitlements, escalate privileges, abuse the in-app updater, or tamper with signed builds), email:

**smastrom@proton.me**

Include:

- A description of the issue and its impact
- Steps to reproduce (or a proof-of-concept)
- The ApexDisk version and macOS version you tested on
- Whether Full Disk Access was granted
- Your contact preference for follow-up

You'll get an acknowledgement within **72 hours**. Fixes are typically shipped in the next patch release; coordinated disclosure timing can be agreed on a case-by-case basis.

## Supported versions

Only the latest stable release receives security fixes. Beta builds are not covered.

| Version        | Supported |
| -------------- | --------- |
| Latest stable  | ✅        |
| Beta           | ❌        |
| Older releases | ❌        |

## Scope

In scope:

- The ApexDisk application (Rust backend + webview frontend)
- The in-app updater flow and update artifact signing (see `reference/updates.md`)
- Release artifacts published on GitHub Releases
- Interaction with macOS Trash, protected folders, and Full Disk Access

Out of scope:

- Vulnerabilities in third-party dependencies already tracked upstream (please report those to the respective project; you can still notify me so I can pin/patch)
- Social-engineering of the maintainer or release infrastructure
- Issues that require a pre-compromised macOS account

## Verifying release authenticity

Stable releases are signed with an Apple Developer ID and notarized by Apple. Updater artifacts are signed with the project's updater key. If Gatekeeper or the in-app updater reports a signature mismatch, treat the build as untrusted and email the address above.
