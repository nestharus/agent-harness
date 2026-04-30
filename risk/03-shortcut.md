# WU-0A-03 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-03's purpose (a contract-pinned `LocalStorageLayout` DTO + lexical derivation helper + every documented `StorageLayoutError` reachable from fixture-backed tests)? Are TODOs, stubbed branches, weak assertions, or deferred behaviors hiding incomplete work?

## Shortcuts examined

### S1 — Path containment is lexical, not canonical

`src-tauri/src/storage.rs:84-106` walks `Path::components()` and resolves `.`/`..` purely lexically. There is no `Path::canonicalize`, no `fs::read_link`, no symlink dereference. Justified: contract `product-strategy/contracts/wu-0a-03-local-storage-layout-dto.md:38` says explicitly "The containment check is lexical and does not create directories, canonicalize symlinks, open the database, or inspect the host filesystem." Proposal A2 (`proposals/03-wu-0a-03.md:39`) further records this is invalidated only by a revised contract requiring filesystem canonicalization. **Justified.**

### S2 — `normalize_workspace_path` returns `Err(())` and the caller maps to a path-specific variant

`storage.rs:67-75, 84-106`. The `Result<PathBuf, ()>` return is a typed indication of "parent traversal escaped". Each caller maps the unit error to its own path-specific `StorageLayoutError` variant (`storage.rs:32, 68`). This means a `database_path` that contains `..` escapes maps to `DatabasePathOutsideStorageRoot`, not `StorageRootEscapesWorkspace` — which is what the contract / fixtures require (`fixtures/wu-0a-03/derive-errors.json:23-31` and `invalid-layouts.json:5-15`). The unit error is intentional — the caller owns the variant choice. Loop-bound by `local_storage_layout_contract.rs:119-152`. **Justified.**

### S3 — Storage-root "workspace fixture boundary" is encoded as "no parent traversal escapes implicit root"

`storage.rs:84-106` rejects only relative paths that pop past their starting point (`if !normalized.pop() { return Err(()) }`). There is no anchored "workspace fixture root" path injected into the check. Justified: the contract clause (`…dto.md:28`) requires lexical validation that "relative storage roots must not use parent traversal to escape that boundary"; with no app-state filesystem root in Phase 0A (proposal A2 at `proposals/03-wu-0a-03.md:39`), the only lexical signal available is "the path's `..` count exceeds its preceding `Normal` count." The `derive-errors.json` "storage root escapes workspace fixture boundary" fixture (`fixtures/wu-0a-03/derive-errors.json:5-13`) drives this branch and asserts `StorageRootEscapesWorkspace`. **Justified.**

### S4 — `join_child_path` uses `to_string_lossy().into_owned()`

`storage.rs:77-82` produces a `String` for the derived child paths. On Linux with UTF-8 paths there is no lossy conversion; on other targets a non-UTF-8 component would be replaced with U+FFFD, but the contract DTO is JSON strings (`…dto.md:11-16`), so a non-UTF-8 child would already be unrepresentable. The fixtures and tests are UTF-8 throughout (`fixtures/wu-0a-03/canonical-layout.json:2-7`). **Justified.**

### S5 — `Display` for `StorageLayoutError` synthesizes the wire string via `serde_json`

`local_storage_layout.rs:24-29` serializes through `serde_json` and trims the surrounding quotes. Couples the human-readable form to the wire form so they cannot drift. Not a shortcut around correctness. **Justified.**

### S6 — TS parser uses an inline allowed-key list rather than zod / typia

`src/contracts/local-storage-layout.ts:37-65`. Hand-rolled field-by-field type checks plus an exact-key-set comparison (sorted). Mirrors the WU-0A-02 parser style and avoids new runtime deps; the proposal anti-scope intentionally avoids them. `invalid-layout-shapes.json` exercises missing-field, extra-field, and wrong-type rejection (`local-storage-layout.test.ts:39-47`). **Justified.**

### S7 — `validate_local_storage_layout` re-runs *all* containment checks even after `derive_local_storage_layout` constructed the child paths itself

`storage.rs:23, 28-61`. Derivation calls validation, so the derived `evidence_root` / `fixture_root` / `log_root` / `temp_root` are revalidated against `storage_root`. Looks redundant — but it is the same code path used to validate operator-supplied layouts (the `invalid-layouts.json` fixture tests). Sharing the check across both call sites is the correct way to keep derivation and configured-layout validation behaviorally identical. **Justified.**

### S8 — `derive_local_storage_layout` does not pre-validate `settings.storage_root` before joining child names

`storage.rs:14-23`. If `storage_root` is `../outside/storage`, the function still constructs child paths under it and calls `validate_local_storage_layout`, which then fails on `storage_root` itself with `StorageRootEscapesWorkspace`. The wasted child-path string allocations are negligible, and the test fixture asserts the expected error (`fixtures/wu-0a-03/derive-errors.json:5-13`). The alternative — pre-validating storage_root before derivation — would duplicate the lexical check. **Justified.**

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented!()`, or `todo!()` markers in `src-tauri/src/storage.rs`, `src-tauri/src/contracts/local_storage_layout.rs`, `src/contracts/local-storage-layout.ts`, `src-tauri/tests/local_storage_layout_contract.rs`, or `src/test/local-storage-layout.test.ts`.
- No commented-out match arms, no `#[allow(dead_code)]`, no `#[ignore]` Rust tests, no `it.skip(...)` / `it.todo(...)` in vitest.
- All six error variants are reachable from fixtures: `StorageRootEscapesWorkspace` and `DatabasePathOutsideStorageRoot` from `derive-errors.json`; `DatabasePathOutsideStorageRoot` again plus the four child-root variants from `invalid-layouts.json` (`local_storage_layout_contract.rs:119-152`).
- Round-trip and unknown-variant rejection: `local_storage_layout_contract.rs:154-188` covers Rust serde of all six variants plus negative `invalid-storage-layout-error.json`; `local-storage-layout.test.ts:49-76` covers the TS parser side, including a `// @ts-expect-error` compile-time check at line 62 that the union actually rejects `"UnknownStorageLayoutError"`.
- `parseLocalStorageLayout` and `parseStorageLayoutError` both throw rather than returning sentinels (`local-storage-layout.ts:21-35`), so test failures cannot mask via undefined returns.
- The Rust serde `deny_unknown_fields` attribute (`local_storage_layout.rs:4`) ensures an unknown DTO field would fail deserialization — the canonical fixture round-trip would catch any silent acceptance (`local_storage_layout_contract.rs:51-82`).

## Verdict

**LOW.** Every shortcut is either documented in the contract (lexical-only path checks, no canonicalization), justified by the proposal anti-scope/assumptions (no filesystem probes, no symlinks, no env reads), or compensated by negative fixtures (`derive-errors.json`, `invalid-layouts.json`, `invalid-layout-shapes.json`, `invalid-storage-layout-error.json`). No deferred work hidden in TODOs, no stubbed enum arms, no weak assertions: every error variant is reachable, every variant is round-tripped through both Rust serde and the TS parser, and the success path is asserted against an explicit expected-result fixture rather than a self-referential snapshot.
