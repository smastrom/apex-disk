Run a full compatibility verification for ApexDisk against the declared minimum target (macOS 10.15 Catalina, Safari 13).

Follow the steps documented in `COMPATIBILITY.md` at the project root. For each section:

1. **Architecture** — Verify the release workflow builds a universal binary (Intel + Apple Silicon). Check that no `cfg(target_arch)` gates exist in Rust source.
2. **Minimum macOS version** — Confirm `minimumSystemVersion` in `src-tauri/tauri.conf.json` is `10.15`.
3. **CSS transpilation** — Verify `vite.config.ts` has lightningcss targeting Safari 13. Run `pnpm build` and check that CSS output contains no untranspiled nesting.
4. **JavaScript transpilation** — Verify `vite.config.ts` has `build.target: 'safari13'`. Run `pnpm build` and check output for untranspiled syntax (optional chaining, nullish coalescing).
5. **Rust / Native APIs** — Audit all `objc2` usage in `src-tauri/src/` and verify every Foundation/AppKit symbol is available on macOS 10.13 or earlier.
6. **Tauri 2 requirements** — Confirm Tauri 2's WKWebView hard requirement of macOS 10.15 is met.
7. **Progressive enhancement** — List any CSS/Web API features that only work on newer macOS and confirm they degrade gracefully (behind `@supports`, no core functionality broken).

After completing all checks, update `COMPATIBILITY.md` with any findings that differ from the current content. If everything checks out, say so.
