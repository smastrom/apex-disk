# Protected files

ApexDisk filters two classes of paths so a careless click can't break macOS or expose credentials. Both lists live in [`src-tauri/src/safe_folders.rs`](../src-tauri/src/safe_folders.rs) as `const &[&str]`. Edit them there; everything downstream is derived.

## Two lists, two purposes

| List                      | Examples                                                                                  | Threat model                                                                                                                                                                |
| ------------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PROTECTED_RELATIVE_PATHS` | `Library`, `Documents`, `Desktop`, `Dropbox`, `OneDrive`, `Pictures/Photos Library.photoslibrary` | The **folder itself** must survive. Removing it breaks system hooks, app registration, or triggers cascade-deletion on cloud-sync remotes. Contents may still be trashed.   |
| `SKIPPED_RELATIVE_PATHS`  | `.ssh`, `.gnupg`, `.aws`, `.kube`, `.password-store`, `Library/Keychains`, `Library/Messages/chat.db`, `.Trash` | Secret material: SSH keys, GPG keys, cloud tokens, password vaults, iMessage history, system keychains. The folder **and every descendant** is hidden from the scan and refused by the trash filter, so keys never surface in the UI. |

Comparison is **case-insensitive**, matching APFS's default case-insensitive volume layout (case-sensitive APFS volumes also work, since both sides are lowercased before comparison). Both functions take `home` as a parameter so tests can point them at a temp dir; production passes the canonicalized `dirs::home_dir()`.

## Where Rust checks

| Function                       | Match                    | Call sites                                                                                          |
| ------------------------------ | ------------------------ | --------------------------------------------------------------------------------------------------- |
| `is_path_skipped(path, home)`  | folder or any descendant | [`scan.rs`](../src-tauri/src/scan.rs) walker + user-folder seed; [`trash.rs`](../src-tauri/src/trash.rs) `filter_items` |
| `is_path_protected(path, home)` | exact folder only        | [`scan.rs`](../src-tauri/src/scan.rs) sets `FolderInfo.is_protected` for the UI; [`trash.rs`](../src-tauri/src/trash.rs) `filter_items` |

### Scan side

- Skipped paths are pruned at `read_dir` time, so credential trees are never traversed and never appear in `FolderInfo`.
- Protected paths are still walked; the `is_protected: true` flag on the resulting `FolderInfo` lets the UI lock the row from selection. The user can dive in and trash *contents*, but not the folder itself.

### Trash side (yes, enforced)

[`trash::filter_items`](../src-tauri/src/trash.rs) is the last line of defense. For every requested item it:

1. Calls `path.canonicalize()` to resolve symlinks.
2. Drops the item if `is_path_protected(canonical, home)` **or** `is_path_skipped(canonical, home)` returns `true`.
3. Survivors are returned as `FilteredItem { canonical, is_file, size }`, **carrying the canonical `PathBuf`** so the eventual `trash::delete` operates on the exact identity the checks approved.

Passing the canonical (not the original string) to `trash::delete` closes a symlink/rename TOCTOU window: between filter approval and the system call, the original path could be repointed or renamed; the canonical is the resolved target as observed at check time, so the deletion targets the same inode the filter saw.

`TrashResult.count` and `.size` reflect items actually trashed, so a frontend bug that lets a protected row through cannot remove it from disk; it just silently disappears from the batch.

The canonicalize step is load-bearing: without it, a path resolving through a symlink loses its home prefix and `is_path_protected` falls back to the out-of-home default (`true`), silently dropping legitimate items.

Canonicalize failures (broken symlinks, permission errors, missing files) are logged via `[apex:rust:trash]` with a sanitized basename before the item is dropped fail-closed, so a frontend that reports fewer trashed items than requested has a debuggable trail.

## Out-of-home asymmetry (defensive default)

A path that is **not** under `home`:

- `is_path_protected` returns `true`: refuse to trash anything we can't anchor.
- `is_path_skipped` returns `false`: skipping only ever excludes named home-relative paths.

The scanner never feeds out-of-home paths into these checks. The trash filter does, and the asymmetry is deliberate: fail closed for deletion, fail open for visibility.

## Adding a path

1. Append to `PROTECTED_RELATIVE_PATHS` or `SKIPPED_RELATIVE_PATHS` in `safe_folders.rs`. Use the on-disk casing; matching lowercases the input.
2. The coverage tests in [`src-tauri/tests/safe_folders_test.rs`](../src-tauri/tests/safe_folders_test.rs) (`is_path_skipped_exact_match_all`, `is_path_protected_exact_match_all`, `is_path_skipped_descendant_all`) iterate the constants, so a new entry is exercised automatically. Add a targeted assertion only if the path has unusual casing, depth, or descendant semantics worth pinning.
3. If the change adds a new *class* of sensitive path (not just a new entry), update the threat-model column above so the rationale stays current.
