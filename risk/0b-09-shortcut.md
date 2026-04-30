# WU-0B-09 — Shortcut Risk Review

**Gate:** Phase 8 shortcut.
**Severity:** LOW.

## Question

Do the chosen implementation paths compromise WU-0B-09's purpose (real serde+SQLx enum codecs, real FK enforcement, real path-escape rejection, real content-hash invariant)? Are there stubbed branches, fake throws, swallowed errors, hidden TODOs, or string-prefix shortcuts that hide incomplete work?

## Shortcuts examined

### WU-0B-09-SHORTCUT-F01 — `impl_sqlite_text_enum!` is a real macro generating real Type/Encode/Decode

`src-tauri/src/graphstore/evidenceartifact.rs:523-552` defines `impl_sqlite_text_enum!($type_name)` which expands to genuine `Type<Sqlite>`, `Encode<'q, Sqlite>` (encoding via `self.as_str().to_string()`), and `Decode<'r, Sqlite>` (decoding via `Self::parse(&encoded)`) implementations. Invocations at `:554-556` apply the macro to `SourceType`, `ToolProtocol`, and `CaptureState`; `PrivilegeOrigin` reuses the codec from `graphnode.rs` via the import at `:12`. The four variant-round-trip tests each create a temp codec table, `INSERT` the variant via `.bind(variant)`, then `SELECT ... try_get::<Enum, _>("value")` (`tests/wu_0b_09_evidenceartifact_contract.rs:443-461, 498-516, 553-571, 611-629`) — those go through the real macro-generated codec, not a sentinel. Unknown values fail SQLx decode (`:478, 533, 591, 646`) because `Enum::parse` returns `GraphStoreError::InvalidEnum` which is mapped through `BoxDynError` in the `Decode` impl. **Justified.**

### WU-0B-09-SHORTCUT-F02 — `PRAGMA foreign_keys = ON` is a real pragma, executed before the transaction

`src-tauri/src/graphstore/evidenceartifact.rs:109-112` runs `sqlx::query("PRAGMA foreign_keys = ON").execute(&self.pool).await?` before `pool.begin()`. The FK target check (`workspace_storage_root` at `:313-327`) executes inside the transaction and joins to `graph_workspaces`; the FK miss `transaction-rollback-fk.json` fixture exercises this and is mapped to `GraphStoreError::UnknownRef` either by the explicit pre-check (`:137-143`) or by `map_write_error("FOREIGN KEY constraint failed")` (`:422-423`). **Real pragma, real FK enforcement.**

### WU-0B-09-SHORTCUT-F03 — `blob_ref` validation does real path canonicalization, not a string-prefix check

`validate_blob_ref_under_storage_root` (`:381-410`) does four checks, in order:
1. trim-empty rejection on both `blob_ref` and `storage_root` (`:385-387`).
2. Component walk: `Path::new(blob_ref).components()` rejects `Component::ParentDir`, `Component::RootDir`, `Component::Prefix(_)`, plus `relative.is_absolute()` (`:389-399`). This catches `..`, leading `/`, and Windows-style drive prefixes purely from path semantics.
3. `fs::canonicalize(storage_root)` resolves the storage root to a real absolute canonical path (`:401`); failure to canonicalize the root → `InvariantViolation`.
4. `fs::canonicalize(root.join(relative))` resolves the candidate path including following symlinks (`:402-404`); the final guard is `if !canonical_candidate.starts_with(&root)` (`:405-407`) — a comparison between *canonicalized* paths, so symlinks pointing outside the root are caught here, not at the textual layer.

The contract test at `:685-743` exercises all three failure shapes the contract document names (`product-strategy/contracts/wu-0b-09-evidenceartifact.md:50`): `../outside.txt`, an absolute path, and (on Unix) a real symlink whose target lives in `storage_root.parent().join("outside-evidence.txt")` — the symlink path's canonicalization escapes the workspace root and the `starts_with` check rejects it. **Real canonicalization, not a string-prefix shortcut.**

### WU-0B-09-SHORTCUT-F04 — Content-hash invariant is enforced in the repository before insert (defense-in-depth with the SQL CHECK)

`validate_evidence_artifact` (`:329-365`) calls `match record.capture_state` and rejects `Captured | Partial | Redacted | Quarantined` when `content_hash` is `None` (`:352-360`) returning `GraphStoreError::InvariantViolation`. The `Failed` arm falls through, but the next branch checks `record.blob_ref.trim().is_empty()` (`:343-350`) which catches the `failed_missing_hash_missing_failure_ref` fixture (`blob_ref: ""`). The SQL `CHECK (capture_state = 'failed' OR content_hash IS NOT NULL)` (`migrations/0b/09_evidenceartifact.sql:64-67`) is a second layer that catches the same invariant if the repo guard ever drifts — not a redundant stub, but a defense-in-depth pair. The contract test at `:650-682` confirms all three branches: `captured + null hash` rejected pre-DB; `failed + null hash + empty blob_ref` rejected pre-DB; `failed + null hash + valid blob_ref` accepted. **Real layered enforcement.**

### WU-0B-09-SHORTCUT-F05 — `failed`-without-failure-ref returns `InvariantViolation` (not a generic SqlxFailure)

The empty-`blob_ref` rejection at `validate_evidence_artifact:343-350` returns `GraphStoreError::InvariantViolation` directly — no `_ =>` catch-all, no error swallowing. The contract test asserts equality on the typed enum *and* on `error.code()` via `assert_error` (`:206-209, 671-672`). **Justified.**

### WU-0B-09-SHORTCUT-F06 — `PrivilegeOrigin` reuse is a real type re-export, not a duplicated definition

`src-tauri/src/graphstore/evidenceartifact.rs:12` imports the WU-0B-07 enum directly. `src-tauri/src/contracts/evidenceartifact.rs:4` re-exports it as `pub use crate::graphstore::graphnode::PrivilegeOrigin;` so contract consumers get a single source of truth. No `enum PrivilegeOrigin` declaration appears in either new file. The contract test at `:595-647` uses the *re-exported* `PrivilegeOrigin` from `agent_harness_lib::contracts::evidenceartifact` and inserts each variant through the real `EvidenceArtifactRepo` — the round-trip path goes through the existing WU-0B-07 codec, not a duplicated one. **Justified — reuse, not duplication.**

### WU-0B-09-SHORTCUT-F07 — No `unwrap()`, no `todo!()`, no `unimplemented!()`, no `TODO`/`FIXME` in the implementation

`grep` for `unwrap\(\)|todo!|unimplemented!|TODO|FIXME|XXX|HACK` against `src-tauri/src/graphstore/evidenceartifact.rs` and `src-tauri/src/contracts/evidenceartifact.rs` returns no matches. Error paths use `?` with explicit `GraphStoreError::from` mappings (`:111-112, 178-179`); the only `expect(...)` calls live in the test file (test-only seed/expectation helpers). **Justified.**

### WU-0B-09-SHORTCUT-F08 — No `_` catch-all swallowing in `map_write_error`

`map_write_error` (`:416-438`) cascades through specific `message.contains(...)` checks for `UNIQUE`, `FOREIGN KEY`, and the four enum-named CHECK constraints, then falls to a generic `CHECK constraint failed` → `InvariantViolation`, with the final arm being `SqlxFailure`. The first match is the duplicate-id message, which is also pre-checked at `:123-134` (the `SELECT COUNT(*)` returns `DuplicateId` before hitting the insert). The `_ => SqlxFailure` arm at `:436` is the typed sentinel for non-database errors (connection drops, encoding errors), not a swallowing catch-all over silent failures. **Justified.**

### WU-0B-09-SHORTCUT-F09 — `validate_contract_timestamp` rejects empty/malformed `captured_at`

`captured_at` is a contract-required `String` field (`:69`) but `RecordMeta` already validates created/updated timestamps. The implementer reuses `validate_record_meta` against a synthetic `RecordMeta` with `captured_at` substituted into both timestamp slots (`:367-375`), so any whitespace/empty/malformed `captured_at` is rejected by the same code path that validates the meta timestamps. This is a real reuse of the prior-WU validator, not a custom partial check. **Justified.**

### WU-0B-09-SHORTCUT-F10 — `TryFrom<EvidenceArtifactRow>` re-runs `validate_evidence_artifact` on read

`EvidenceArtifact::try_from(row)` at `:269-302` re-validates the row through `validate_evidence_artifact` after constructing it from columns. This means a row that was *somehow* inserted bypassing the repo (e.g., via a future repair script that touched the DB directly) is still rejected on read — the contract is enforced on both the write *and* read path. The byte-equivalent round-trip test passes because the canonical fixture obeys every invariant. **Justified — defense-in-depth, not a stub.**

## Hidden-incomplete-work check

- No `#[allow(dead_code)]`, no `#[ignore]`, no `#[cfg(test)]` gating of production logic, no commented-out blocks. The only `#[cfg(unix)]` block (`tests/wu_0b_09_evidenceartifact_contract.rs:713-742`) is a documented platform gate for the symlink test, which fails closed (test does nothing on Windows because `symlink` is unavailable) — this is acceptable for a Linux-targeted Phase 0B harness and the parent + absolute-path tests still run unconditionally.
- The four enum types use `#[serde(rename_all = "snake_case")]` with explicit `as_str` and `parse` methods — no `Default::default()` fall-through and no derive-only path that could silently accept variants.
- `cargo clippy --all-targets --all-features -- -D warnings` is clean.

## Verdict

**LOW.** Every shortcut examined is a real implementation, not a stub:
- The enum codecs go through a real macro generating real `Type`/`Encode`/`Decode`, exercised through a temp SQLx codec table for each variant.
- `PRAGMA foreign_keys = ON` runs before every `insert_evidence_artifact` and the FK miss is exercised through both a pre-check (`UnknownRef`) and the database-level CHECK error mapping.
- `blob_ref` validation uses `fs::canonicalize` on both root and candidate, and a real `starts_with` comparison on canonical paths — the symlink test confirms the path-walk component check alone wouldn't catch a symlink, but canonicalization does.
- The content-hash invariant is enforced both pre-DB (`validate_evidence_artifact`) and via SQL CHECK (defense-in-depth, not redundancy).
- `failed` without a failure payload ref returns `InvariantViolation` exactly as the contract requires.
- `PrivilegeOrigin` is reused via type re-export, not duplicated.
- No `unwrap()`, no `todo!`, no TODO markers, no swallowing `_` catch-all, no test-only feature flags. The `cfg(unix)` gate on the symlink test is the only platform-conditional block and is a documented constraint of the test (not the impl).
