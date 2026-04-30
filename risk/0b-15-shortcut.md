# WU-0B-15 — Shortcut Risk Review

**Gate:** Phase 8 shortcut.
**Severity:** LOW.

## Question

Do the chosen implementation paths compromise WU-0B-15's purpose (real serde+SQLx codec for `AuditDecision`, real FK enforcement, real append-only invariant, real output-with-input invariant)? Are there stubbed branches, fake throws, swallowed errors, hidden TODOs, or string-prefix shortcuts that hide incomplete work?

## Shortcuts examined

### WU-0B-15-SHORTCUT-F01 — `impl_sqlite_text_enum!` is a real macro generating real Type/Encode/Decode

`src-tauri/src/graphstore/auditevent.rs:431-460` defines `impl_sqlite_text_enum!($type_name)` which expands to genuine `Type<Sqlite>`, `Encode<'q, Sqlite>` (encoding via `self.as_str().to_string()`), and `Decode<'r, Sqlite>` (decoding via `Self::parse(&encoded)`) implementations. The macro is invoked at `:462` for `AuditDecision`. The variant-round-trip test (`tests/wu_0b_15_auditevent_contract.rs:393-445`) creates a temp `audit_decision_codec` table, `INSERT`s each variant via `.bind(variant)`, then `SELECT ... try_get::<AuditDecision, _>("value")` (`:418-428`) — these go through the real macro-generated codec, not a sentinel. Unknown `"escalated"` fails SQLx decode (`:443-444`) because `AuditDecision::parse` returns `GraphStoreError::InvalidEnum`, mapped through `BoxDynError` in the `Decode` impl. The same macro pattern is repeated across `graphnode.rs:715`, `graphconfiguration.rs:577`, and `evidenceartifact.rs:523` — established convention, not a one-off shortcut.

### WU-0B-15-SHORTCUT-F02 — `PRAGMA foreign_keys = ON` is a real pragma, executed before every transaction

`src-tauri/src/graphstore/auditevent.rs:87-90` runs `sqlx::query("PRAGMA foreign_keys = ON").execute(&self.pool).await?` before `pool.begin()`. The FK miss fixtures (`fk-rejections.json`: `unknown_workspace`, `unknown_policy_set`, `unknown_configuration`) are exercised through this path and mapped to `GraphStoreError::UnknownRef` by `map_write_error("FOREIGN KEY constraint failed")` (`:393-395`). The contract test verifies all three FK-miss shapes return `UnknownRef` and the row count is unchanged (`:533-554`). **Real pragma, real FK enforcement.**

### WU-0B-15-SHORTCUT-F03 — Append-only invariant is enforced by source-grep, not just by omission

`audit_events_repository_is_append_only_by_source_assertion` (`tests/wu_0b_15_auditevent_contract.rs:447-467`) lower-cases the impl source and asserts six forbidden patterns are absent: `update audit_events`, `delete from audit_events`, `fn update_`, `fn delete_`, `pub async fn update_`, `pub async fn delete_`. This catches both runtime SQL strings and Rust function declarations. Independent grep (`grep -rn "UPDATE audit_events\|DELETE FROM audit_events" src-tauri/src/graphstore/auditevent.rs src-tauri/migrations/0b/15_auditevent.sql`) returns zero matches across both source and migration. The `AuditEventRepo` (`src-tauri/src/graphstore/auditevent.rs:78-222`) declares only `new`, `insert_audit_event`, `get_audit_event`, and `list_audit_events_by_workspace` — append-only is structural, not just an assertion. **Justified.**

### WU-0B-15-SHORTCUT-F04 — Output-with-input invariant is enforced pre-DB and at the SQL CHECK layer (defense-in-depth)

`validate_audit_event` (`src-tauri/src/graphstore/auditevent.rs:311-361`) calls `OpaqueId::new` for every required ID (which catches empty `policy_set_id` via `validate_opaque_id_parts`'s trim-empty check at `prelude.rs:163-166`), then explicitly trim-checks `event_type`, `actor`, `reason_code`, and every entry inside `input_refs` / `output_refs` (`:339-354`). The output-with-input rule itself is `if !output_refs.is_empty() && input_refs.is_empty() { InvariantViolation }` (`:356-358`). The SQL `CHECK (json_array_length(output_refs) = 0 OR json_array_length(input_refs) > 0)` (`migrations/0b/15_auditevent.sql:46-49`) is the second layer that catches the same invariant if the repo guard ever drifts. The contract test at `:469-531` confirms all four typed branches (`output_refs_without_input_refs`, `output_refs_with_empty_policy_id`, `output_refs_with_empty_actor`, `output_refs_with_empty_reason_code`) return `InvariantViolation`, plus a raw `INSERT ... decision NULL` rejected by SQL `NOT NULL`. **Real layered enforcement.**

### WU-0B-15-SHORTCUT-F05 — `validate_audit_event` runs on every read via `TryFrom<AuditEventRow>`

`src-tauri/src/graphstore/auditevent.rs:252-297` constructs an `AuditEvent` from columns and then re-runs `validate_audit_event(record)` (`:295`) before returning. This means a row that was *somehow* inserted bypassing the repo (e.g., via a future repair script that touched the DB directly) is still rejected on read — the contract is enforced on both the write *and* read path. The byte-equivalent round-trip test passes because the canonical fixture obeys every invariant. The `optional_opaque_id` helper (`:373-382`) rejects half-populated configuration/provider IDs (one of value/namespace null but not the other) with `InvariantViolation` — paired with the SQL paired-null CHECKs (`migrations/0b/15_auditevent.sql:34-41`) for defense-in-depth. **Justified.**

### WU-0B-15-SHORTCUT-F06 — `validate_contract_timestamp` reuses `validate_record_meta` for `created_at`

`created_at` is a contract-required `String` field (`src-tauri/src/graphstore/auditevent.rs:44`) but `RecordMeta` already validates created/updated timestamps. The implementer reuses `validate_record_meta` against a synthetic `RecordMeta` with `created_at` substituted into both timestamp slots (`:363-371`), so any whitespace/empty/malformed `created_at` is rejected by the same code path that validates the meta timestamps. This is a real reuse of the WU-0B-02 prelude validator, not a custom partial check. **Justified.**

### WU-0B-15-SHORTCUT-F07 — `DuplicateId` pre-check is a real `SELECT COUNT(*)`, not a swallowing fall-through

`insert_audit_event` (`src-tauri/src/graphstore/auditevent.rs:101-112`) issues a real `SELECT COUNT(*) FROM audit_events WHERE audit_event_id_value = ? AND audit_event_id_namespace = ?` inside the open transaction; on `count != 0` it rolls back and returns `GraphStoreError::DuplicateId`. The `map_write_error` cascade also handles `UNIQUE constraint failed` via the SQLite error text (`:391-393`) — defense-in-depth, not a redundant stub. **Justified.**

### WU-0B-15-SHORTCUT-F08 — No `unwrap()`, no `todo!()`, no `unimplemented!()`, no `TODO`/`FIXME` in the implementation

`grep -E "unwrap\(\)|todo!|unimplemented!|TODO|FIXME|XXX|HACK"` against `src-tauri/src/graphstore/auditevent.rs` and `src-tauri/src/contracts/auditevent.rs` returns no matches. Error paths use `?` with explicit `GraphStoreError::from` mappings (`:88-89, 97-98, 167, 384-386`); the only `expect(...)` calls live in the test file (test-only seed/expectation helpers). **Justified.**

### WU-0B-15-SHORTCUT-F09 — No `_` catch-all swallowing in `map_write_error`

`map_write_error` (`src-tauri/src/graphstore/auditevent.rs:388-406`) cascades through specific `message.contains(...)` checks for the duplicate-id UNIQUE message, `FOREIGN KEY constraint failed`, the `decision` CHECK, and a generic `CHECK constraint failed` → `InvariantViolation`, with the final arm being `SqlxFailure` for non-database errors (connection drops, encoding errors). The `_ => SqlxFailure` arm at `:404` is the typed sentinel for non-database errors, not a swallowing catch-all over silent failures. The duplicate-ID UNIQUE arm is also pre-checked at `:101-112` (the `SELECT COUNT(*)` returns `DuplicateId` before hitting the insert). **Justified.**

### WU-0B-15-SHORTCUT-F10 — `ProviderStateRef` is a typed marker, not a stub for future work

`src-tauri/src/graphstore/auditevent.rs:16-18` declares `pub struct ProviderStateRef;` as a unit-marker type used solely as the `T` parameter on `OpaqueId<ProviderStateRef>`. The contract permits this as a soft reference until WU-0B-17 (`product-strategy/contracts/wu-0b-15-auditevent.md:27`). There is no `ProviderStateRefRepo`, no incomplete repository surface, no `todo!()`, and no `panic!()` — the type is genuinely just an opaque-ID namespace tag. Validation goes through `OpaqueId::<ProviderStateRef>::new` like every other ID (`:330-335, 260-263`). **Justified — typed soft reference, not a stub.**

## Hidden-incomplete-work check

- No `#[allow(dead_code)]`, no `#[ignore]`, no `#[cfg(test)]` gating of production logic, no commented-out blocks.
- The `AuditDecision` enum uses `#[serde(rename_all = "snake_case")]` with explicit `as_str` and `parse` methods — no `Default::default()` fall-through and no derive-only path that could silently accept variants.
- `cargo clippy --all-targets --all-features -- -D warnings` is clean.
- `cargo fmt --check` is clean.

## Verdict

**LOW.** Every shortcut examined is a real implementation, not a stub:
- The `AuditDecision` codec goes through a real macro generating real `Type`/`Encode`/`Decode`, exercised through a temp SQLx codec table for each of the five variants.
- `PRAGMA foreign_keys = ON` runs before every `insert_audit_event` and the FK miss is exercised through the database-level error mapping (`UnknownRef`).
- The append-only invariant is enforced by both the structural absence of `update_`/`delete_` methods and a source-grep test.
- The output-with-input invariant is enforced both pre-DB (`validate_audit_event`) and via SQL CHECK (defense-in-depth, not redundancy).
- `validate_audit_event` runs on both write and read (`TryFrom<AuditEventRow>`) — defense-in-depth across the full repository surface.
- `ProviderStateRef` is a typed soft-FK marker reserved for WU-0B-17, not a stub. No `unwrap()`, no `todo!`, no TODO markers, no swallowing `_` catch-all.
