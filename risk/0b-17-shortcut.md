# WU-0B-17 — Shortcut Risk Review

**Gate:** Phase 8 shortcut.
**Severity:** LOW.

## Question

Do the chosen implementation paths compromise WU-0B-17's purpose (real serde+SQLx codecs for nine enums, real FK enforcement, real two-layer `secret_material_stored = false` invariant, real credential-shape rejection, real `mark_provider_state_stale` UPDATE inside a transaction)? Are there stubbed branches, fake throws, swallowed errors, hidden TODOs, or string-prefix shortcuts that hide incomplete work?

## Shortcuts examined

### WU-0B-17-SHORTCUT-F01 — `impl_sqlite_text_enum!` is a real macro generating real Type/Encode/Decode for all nine enums

`src-tauri/src/graphstore/providerstate.rs:884-913` defines `impl_sqlite_text_enum!($type_name)` which expands to genuine `Type<Sqlite>` (using `<String as Type<Sqlite>>::type_info()`), `Encode<'q, Sqlite>` (encoding via `self.as_str().to_string()`), and `Decode<'r, Sqlite>` (decoding via `Self::parse(&encoded)` and lifting `GraphStoreError::InvalidEnum` into `BoxDynError`). The macro is invoked at `:915-923` for all nine enums:
- `ProviderKind`, `ProviderCli`, `AuthState`, `BillingState`, `ProviderQuotaState`, `NetworkState`, `ProviderRuntimeState`, `ProviderFreshness`, `ProviderConfidence`.

The nine variant-round-trip tests (`tests/wu_0b_17_providerstate_contract.rs:649-738`) each create a temp codec table per enum (`provider_kind_codec`, `provider_cli_codec`, `auth_state_codec`, `billing_state_codec`, `quota_state_codec`, `network_state_codec`, `runtime_state_codec`, `freshness_codec`, `confidence_codec`), `INSERT`s each variant via `.bind(variant)`, then `SELECT ... try_get::<Enum, _>("value")` — these go through the real macro-generated codec. Unknown values fail SQLx decode because per-enum `parse` returns `GraphStoreError::InvalidEnum` (`:703, 724, 747, 770, 791, 814, 839, 860, 879`), mapped through `BoxDynError`. The same macro pattern is established convention across `auditevent.rs`, `budgetledger.rs`, `graphnode.rs`, `graphconfiguration.rs` — not a one-off shortcut.

### WU-0B-17-SHORTCUT-F02 — `PRAGMA foreign_keys = ON` is a real pragma, executed before every transaction

`src-tauri/src/graphstore/providerstate.rs:165-168` runs `sqlx::query("PRAGMA foreign_keys = ON").execute(&self.pool).await?` before `pool.begin()` in `insert_provider_state`. Repeated at `:292-295` for `mark_provider_state_stale`. The FK miss fixture (`unknown-workspace`) is exercised through this path and mapped to `GraphStoreError::UnknownRef` by `map_write_error` matching `"FOREIGN KEY constraint failed"` (`:576-578`). The contract test (`:328-332`) verifies the FK miss returns `UnknownRef` with row count unchanged. **Real pragma, real FK enforcement.**

### WU-0B-17-SHORTCUT-F03 — `secret_material_stored = false` is enforced in two layers (defense-in-depth)

- **Repo layer** (`src-tauri/src/graphstore/providerstate.rs:459-461`): `validate_provider_state` rejects `record.secret_material_stored == true` with `GraphStoreError::InvariantViolation` *before* the SQL insert; failure rolls back the transaction.
- **SQL layer** (`migrations/0b/17_providerstate.sql:73`): `secret_material_stored INTEGER NOT NULL CHECK (secret_material_stored = 0)` — if a future repair script bypasses the repo, SQLite rejects the row.

The `secret_material_true_and_credential_looking_payloads_reject_before_persistence` test (`:521-586`) exercises both layers: `repo.insert_provider_state(secret_material_true)` rejects via the repo guard (`:531-535`); a direct `sqlx::query("INSERT INTO provider_states ... 1, 'fresh', 'high', ...")` rejects via the SQL CHECK (`:538-561`). Row count unchanged after each. **Real layered enforcement.**

### WU-0B-17-SHORTCUT-F04 — Credential-shape detection is real string analysis, not a stub

`src-tauri/src/graphstore/providerstate.rs:487-548` implements:
- `contains_credential_like_string` (`:496-503`): special-cases `Bearer ` whole-string prefix, then splits on whitespace + `"` + `'` and runs `is_credential_like_token` per segment.
- `is_credential_like_token` (`:505-519`): trims punctuation, checks `sk-` prefix, `Bearer ` prefix, AWS access-key shape, JWT shape, and generic secret token shape.
- `is_aws_access_key_shape` (`:521-527`): exact 20 chars, starts with `AKIA` or `ASIA`, all bytes uppercase or digit.
- `is_jwt_shape` (`:529-536`): exactly three `.`-separated parts, first part `eyJ`-prefixed, every part ≥10 chars and base64-url alphabet.
- `is_generic_secret_token_shape` (`:542-548`): ≥48 chars, base64-url alphabet, contains digit + uppercase + lowercase.
- `value_contains_credential_like_string` (`:487-494`): recurses into `Value::Array` and `Value::Object` so the JSON tree under `sandbox_constraints` is fully scanned.

`validate_provider_state` applies the guard to `account_ref`, the entire JSON tree under `sandbox_constraints`, and every entry in `store_locations_checked` (`:463-472`). The contract test exercises all four documented shapes (`sk-test-redacted-but-secret-shaped`, `Bearer redacted-token-shape`, `AKIAIOSFODNN7EXAMPLE`, `eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZ2VudCJ9.redactedSignature`) at `:563-585`; each rejects `InvariantViolation` before persistence. **Real shape detection, not a stub.**

### WU-0B-17-SHORTCUT-F05 — `mark_provider_state_stale` uses a real `UPDATE` inside an explicit transaction

`src-tauri/src/graphstore/providerstate.rs:282-345`:
1. Validates the opaque ID via `OpaqueId::<ProviderStateRef>::new` (`:287-290`).
2. Validates the reason via `validate_stale_reason` (`:291`, definition at `:550-563`: ASCII lowercase + digits + `_-.:`, ≤80 chars, no `..`).
3. Runs `PRAGMA foreign_keys = ON` (`:292-295`).
4. Opens transaction via `pool.begin()` (`:296`).
5. `select_provider_state` inside the transaction (`:298`) — fail-fast `UnknownRef` if the row is absent.
6. Computes `next_contract_timestamp` (`:306`, definition at `:599-651`: real second-by-second roll-over with leap-year aware `days_in_month`).
7. Issues a real SQL `UPDATE provider_states SET freshness = ?, confidence = ?, updated_at = ?, actor = ? WHERE provider_state_id_value = ? AND provider_state_id_namespace = ?` (`:310-326`).
8. Asserts `rows_affected() == 1` and rolls back if not (`:335-338`).
9. Re-selects the updated row inside the transaction (`:340`) and commits (`:343`).

The `mark_provider_state_stale_updates_only_documented_columns` test (`:387-468`) verifies the SET clause is bounded by JSON-pointer-zeroed before/after equality and a raw-SQL spot-check across 13 unrelated columns. **Real UPDATE, real transaction, real bounded SET clause.**

### WU-0B-17-SHORTCUT-F06 — `validate_provider_state` runs on every read via `TryFrom<ProviderStateRow>`

`src-tauri/src/graphstore/providerstate.rs:376-413` constructs a `ProviderState` from columns via `TryFrom<ProviderStateRow>` and then re-runs `validate_provider_state(record)` (`:411`) before returning. A row that was *somehow* inserted bypassing the repo (e.g., via a future repair script) is still rejected on read — `secret_material_stored` invariant, credential-shape guard, opaque-ID validators, RecordMeta validation, and the contract-timestamp validator all fire on read. The byte-equivalent round-trip test (`:288-314`) passes because the canonical fixture obeys every invariant.

### WU-0B-17-SHORTCUT-F07 — `DuplicateId` pre-check is a real `SELECT COUNT(*)`, not a swallowing fall-through

`insert_provider_state` (`src-tauri/src/graphstore/providerstate.rs:179-190`) issues a real `SELECT COUNT(*) FROM provider_states WHERE provider_state_id_value = ? AND provider_state_id_namespace = ?` inside the open transaction; on `count != 0` it rolls back and returns `GraphStoreError::DuplicateId`. The `map_write_error` cascade also handles the SQLite UNIQUE error message (`:573-575`: `UNIQUE constraint failed: provider_states.provider_state_id_value`) — defense-in-depth, not a redundant stub.

### WU-0B-17-SHORTCUT-F08 — `validate_stale_reason` is a real shape check, not a permissive accept

`src-tauri/src/graphstore/providerstate.rs:550-563`:
```
empty || len > 80 || any byte not in [a-z0-9_\-.:] || contains ".."  →  InvariantViolation
```
Five malformed reasons in `stale-marking.json` (empty, single space, `"Auth Refresh"` with capitals/space, `"../escape"`, `"has/slash"`) are exercised by the negative test (`:500-507`); each rejects `InvariantViolation`. The path-traversal `..` check is a deliberate hardening (the `actor` field is later persisted as JSON in `meta.actor.value`).

### WU-0B-17-SHORTCUT-F09 — `next_contract_timestamp` is a real arithmetic, not `Utc::now()`

`src-tauri/src/graphstore/providerstate.rs:599-651` implements deterministic second-by-second roll-over with field-by-field carry: second → minute → hour → day → month → year. `days_in_month` (`:669-677`) handles leap years via `is_leap_year` (`:679-681`). Format is the contract `YYYY-MM-DDTHH:MM:SSZ` shape (length 20). The function rejects malformed inputs (wrong length, wrong separator bytes, out-of-range fields) with `InvariantViolation` (`:607-622`). This produces a `> meta.updated_at` timestamp without depending on wall-clock time, which makes the `assert_ne!(after.meta.updated_at, before.meta.updated_at)` test deterministic.

### WU-0B-17-SHORTCUT-F10 — No `unwrap()`, no `todo!()`, no `unimplemented!()`, no `TODO`/`FIXME` in the implementation

`grep -E "unwrap\(\)|todo!|unimplemented!|TODO|FIXME|XXX|HACK"` against `src-tauri/src/graphstore/providerstate.rs` and `src-tauri/src/contracts/providerstate.rs` returns no matches. Error paths use `?` with explicit `GraphStoreError::from` mappings (`:168, 295, 343, 567`); the only `expect(...)` calls live in the test file (test-only fixture/seed expectations).

### WU-0B-17-SHORTCUT-F11 — `map_write_error` cascade is specific, not a swallowing catch-all

`map_write_error` (`src-tauri/src/graphstore/providerstate.rs:569-597`) cascades through specific message matches:
- `UNIQUE constraint failed: provider_states.provider_state_id_value` → `DuplicateId` (`:573-575`)
- `FOREIGN KEY constraint failed` → `UnknownRef` (`:576-578`)
- per-enum `CHECK constraint failed: provider`/`cli`/`auth_state`/`billing_state`/`quota_state`/`network_state`/`runtime_state`/`freshness`/`confidence` → `InvalidEnum` (`:578-587`)
- generic `CHECK constraint failed` → `InvariantViolation` (`:589-590`) — covers `secret_material_stored = 0`, `trim(account_ref) <> ''`, `json_valid(...)`, and `json_type(store_locations_checked) = 'array'`
- final `_ => SqlxFailure` (`:591-595`) is the typed sentinel for non-database errors, not a swallowing fallback.

### WU-0B-17-SHORTCUT-F12 — Backfill `INSERT...SELECT...WHERE EXISTS` is real, not a placeholder

`src-tauri/migrations/0b/17_providerstate.sql:179-215, 301-339` recreates `audit_events` and `budget_ledgers` to add the new FK to `provider_states` (the standard SQLite idiom for adding a FK to an existing column). The `INSERT ... SELECT` clause uses `CASE WHEN ... AND EXISTS (SELECT 1 FROM provider_states WHERE ...) THEN ... ELSE NULL END` to NULL out any previously-stored soft-ref values that don't have a corresponding `provider_states` row, preserving the new FK invariant on backfill. This is real data-preserving migration logic, not a placeholder. `DROP TABLE` of the renamed staging tables (`:217, 341`) and re-creation of all four indexes (`:219-229, 343-350`) follow.

## Hidden-incomplete-work check

- No `#[allow(dead_code)]`, no `#[ignore]`, no `#[cfg(test)]` gating of production logic, no commented-out blocks.
- The nine enums use `#[serde(rename_all = "snake_case")]` with explicit `as_str` and `parse` methods — no `Default::default()` fall-through and no derive-only path that could silently accept variants.
- `cargo clippy --all-targets --all-features -- -D warnings` is clean.
- `cargo fmt --check` is clean.

## Verdict

**LOW.** Every shortcut examined is a real implementation, not a stub:
- The nine enum codecs go through a real macro generating real `Type`/`Encode`/`Decode`, exercised through nine temp SQLx codec tables for each variant.
- `PRAGMA foreign_keys = ON` runs before every `insert_provider_state` and every `mark_provider_state_stale`; FK miss is exercised through the database-level error mapping (`UnknownRef`).
- `secret_material_stored = false` is enforced both by `validate_provider_state` pre-DB and by SQL `CHECK (secret_material_stored = 0)`; both paths are exercised by the contract test.
- Credential-shape detection is a real string analyzer covering `sk-`, `Bearer `, AWS access-key shape, JWT shape, and generic ≥48-char high-entropy tokens; the JSON tree under `sandbox_constraints` is recursively scanned.
- `mark_provider_state_stale` uses a real `UPDATE` with an explicit four-column SET clause inside a real transaction, with `rows_affected() == 1` rollback guarding and a real timestamp roll-over arithmetic (no `Utc::now()` dependency).
- `validate_provider_state` runs on both write *and* read (`TryFrom<ProviderStateRow>`) — defense-in-depth across the full repository surface.
- The `audit_events` / `budget_ledgers` table recreation backfills via `WHERE EXISTS`, NULLing pre-existing soft-ref values that have no real provider state — real data-preserving migration.
- No `unwrap()`, no `todo!`, no TODO markers, no swallowing `_` catch-all.
