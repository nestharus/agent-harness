# WU-0B-16 — Shortcut Risk Review

**Gate:** Phase 8 shortcut.
**Severity:** LOW.

## Question

Do the chosen implementation paths compromise WU-0B-16's purpose (real serde+SQLx codecs for the three enums, real FK enforcement, real non-negative-counter enforcement, real two-layer policy-action matrix)? Are there stubbed branches, fake throws, swallowed errors, hidden TODOs, or string-prefix shortcuts that hide incomplete work?

## Shortcuts examined

### WU-0B-16-SHORTCUT-F01 — `impl_sqlite_text_enum!` is a real macro generating real Type/Encode/Decode for all three enums

`src-tauri/src/graphstore/budgetledger.rs:487-516` defines `impl_sqlite_text_enum!($type_name)` which expands to genuine `Type<Sqlite>`, `Encode<'q, Sqlite>` (encoding via `self.as_str().to_string()`), and `Decode<'r, Sqlite>` (decoding via `Self::parse(&encoded)`). The macro is invoked at `:518-520` for `BudgetScopeType`, `BudgetState`, and `PolicyAction`. The three variant-round-trip tests (`tests/wu_0b_16_budgetledger_contract.rs:443-615`) each create a temp codec table per enum (`budget_scope_type_codec`, `budget_state_codec`, `policy_action_codec`), `INSERT`s each variant via `.bind(variant)`, then `SELECT ... try_get::<Enum, _>("value")` — these go through the real macro-generated codec. Unknown values fail SQLx decode because the per-enum `parse` returns `GraphStoreError::InvalidEnum` (`:438, 459, 482`), mapped through `BoxDynError` in the `Decode` impl. The same macro pattern is repeated across `auditevent.rs:431-462`, `graphnode.rs:715`, `graphconfiguration.rs:577` — established convention, not a one-off shortcut.

### WU-0B-16-SHORTCUT-F02 — `PRAGMA foreign_keys = ON` is a real pragma, executed before every transaction

`src-tauri/src/graphstore/budgetledger.rs:110-113` runs `sqlx::query("PRAGMA foreign_keys = ON").execute(&self.pool).await?` before `pool.begin()`. The FK miss fixture (`fk-rejections.json: unknown_workspace`) is exercised through this path and mapped to `GraphStoreError::UnknownRef` by `map_write_error` matching `"FOREIGN KEY constraint failed"` (`:399-400`). The contract test (`:346-351`) verifies the FK miss returns `UnknownRef` and the row count is unchanged. **Real pragma, real FK enforcement.**

### WU-0B-16-SHORTCUT-F03 — Non-negative counters are enforced in two layers (defense-in-depth)

- **Repo layer** (`src-tauri/src/graphstore/budgetledger.rs:344-352`): `validate_budget_ledger` rejects any of the six counters (`input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`, `latency_ms`, `provider_cost_estimate`) being `< 0` with `InvariantViolation` *before* the SQL insert; failure rolls back the transaction.
- **SQL layer** (`migrations/0b/16_budgetledger.sql:20-25`): each counter column has a per-column `CHECK (col >= 0)`; if a future repair script bypasses the repo, SQLite rejects the row. `map_write_error` (`:406-407`) maps the generic `CHECK constraint failed` (non-enum CHECK) to `InvariantViolation`.

The `counters_preserve_zero_and_reject_negative_values` test (`tests/wu_0b_16_budgetledger_contract.rs:618-647`) exercises the all-zero positive case (`zero_counters` from `counter-rejections.json`) and the six negative cases, each set to `-1`. Zero is preserved; negatives reject `InvariantViolation`. **Real layered enforcement; zero is correctly distinguished from negative.**

### WU-0B-16-SHORTCUT-F04 — Policy-action matrix is enforced in two layers (defense-in-depth)

- **Repo layer** (`policy_action_allowed`, `:361-376`): four-arm `match` covering `Within → {None, Warn}`, `NearLimit → {Warn, NarrowScope}`, `Exceeded → {RequireUserApproval, Block, NarrowScope}`, `Blocked → Block`. `validate_budget_ledger` (`:354-356`) calls it pre-insert and rolls back on failure with `InvariantViolation`.
- **SQL layer** (`migrations/0b/16_budgetledger.sql:56-61`): identical four-arm disjunction; if the repo guard ever drifts, SQLite rejects the row.

The `policy_action_matrix_accepts_only_documented_pairs` test exercises all 8 legal cells and 5 illegal cells (4 `blocked` + non-block + 1 `within + require_user_approval`), confirming both invariants from the contract: `blocked ⇒ block`, `within ⇏ require_user_approval`. The full 12-cell negative space is not exhaustively enumerated in fixtures, but both enforcement layers cover the entire space and the named contract invariants are exercised. **Real layered enforcement.**

### WU-0B-16-SHORTCUT-F05 — `validate_budget_ledger` runs on every read via `TryFrom<BudgetLedgerRow>`

`src-tauri/src/graphstore/budgetledger.rs:270-310` constructs a `BudgetLedger` from columns and then re-runs `validate_budget_ledger(record)` (`:308`) before returning. A row that was *somehow* inserted bypassing the repo (e.g., via a future repair script) is still rejected on read — counters, opaque IDs, `RecordMeta`, and the policy-action matrix are all re-validated. The byte-equivalent round-trip test (`:303-329`) passes because the canonical fixture obeys every invariant. The `optional_opaque_id` helper (`:378-387`) rejects half-populated provider-state IDs (one of value/namespace null but not the other) with `InvariantViolation` — paired with the SQL paired-null CHECK (`migrations/0b/16_budgetledger.sql:52-55`) for defense-in-depth.

### WU-0B-16-SHORTCUT-F06 — `DuplicateId` pre-check is a real `SELECT COUNT(*)`, not a swallowing fall-through

`insert_budget_ledger` (`src-tauri/src/graphstore/budgetledger.rs:124-135`) issues a real `SELECT COUNT(*) FROM budget_ledgers WHERE budget_ledger_id_value = ? AND budget_ledger_id_namespace = ?` inside the open transaction; on `count != 0` it rolls back and returns `GraphStoreError::DuplicateId`. The `map_write_error` cascade also handles the SQLite UNIQUE error message at `:397-398` (`UNIQUE constraint failed: budget_ledgers.budget_ledger_id_value`) — defense-in-depth, not a redundant stub. **Justified.**

### WU-0B-16-SHORTCUT-F07 — `ProviderStateRef` is a typed marker reused from WU-0B-15, not a stub

`src-tauri/src/graphstore/budgetledger.rs:8` imports `ProviderStateRef` from `auditevent` and uses it as the type tag for `OpaqueId<ProviderStateRef>` (`:63`). No duplicate marker is defined, no `provider_states` table is created, no FK is wired. The contract document permits this as a soft reference until WU-0B-17 (`product-strategy/contracts/wu-0b-16-budgetledger.md:27`). Validation goes through `OpaqueId::<ProviderStateRef>::new` (`:336-341`) and the `optional_opaque_id` helper (`:274-277, 378-387`) — same trim/forbidden-char checks as every other opaque ID. The contract test exercises the soft-ref value `"provider-state-soft-ref-round-trip"` in the canonical `round-trip.json`. **Typed soft reference, not a stub.**

### WU-0B-16-SHORTCUT-F08 — No `unwrap()`, no `todo!()`, no `unimplemented!()`, no `TODO`/`FIXME` in the implementation

`grep -E "unwrap\(\)|todo!|unimplemented!|TODO|FIXME|XXX|HACK"` against `src-tauri/src/graphstore/budgetledger.rs` and `src-tauri/src/contracts/budgetledger.rs` returns no matches. Error paths use `?` with explicit `GraphStoreError::from` mappings (`:113, 119-121, 132-134, 186, 190-192, 390`); the only `expect(...)` calls live in the test file (test-only fixture/seed expectations). **Justified.**

### WU-0B-16-SHORTCUT-F09 — `map_write_error` cascade is specific, not a swallowing catch-all

`map_write_error` (`src-tauri/src/graphstore/budgetledger.rs:393-414`) cascades through specific message matches:
- `UNIQUE constraint failed: budget_ledgers.budget_ledger_id_value` → `DuplicateId` (`:397-398`)
- `FOREIGN KEY constraint failed` → `UnknownRef` (`:399-400`)
- `CHECK constraint failed: scope_type` / `: budget_state` / `: policy_action` → `InvalidEnum` (`:401-405`)
- generic `CHECK constraint failed` → `InvariantViolation` (`:406-407`) — covers the matrix CHECK, the counter CHECKs, and the paired-null provider-state CHECK
- final `_ => SqlxFailure` (`:408-409, 411-412`) is the typed sentinel for non-database errors (connection drops, encoding errors), not a swallowing fallback for silent failures. The duplicate-ID UNIQUE arm is also pre-checked at `:124-135` (the `SELECT COUNT(*)` returns `DuplicateId` before hitting the insert).

### WU-0B-16-SHORTCUT-F10 — `BudgetScopeRef` is a typed marker, not a deferred-work stub

`src-tauri/src/graphstore/budgetledger.rs:15-17` declares `pub struct BudgetScopeRef;` with `Serialize`/`Deserialize`/`deny_unknown_fields` — used solely as a type marker for `OpaqueId<BudgetScopeRef>`. There is no `BudgetScopeRefRepo`, no `budget_scopes` table, no incomplete repository surface, no `panic!()`, and no `todo!()`. Scope IDs are plain opaque IDs validated by `OpaqueId::<BudgetScopeRef>::new` (`:332-335, 289`), and the contract intentionally treats the scope ID as a free-form opaque reference (the seven `scope_type` variants name what kind of scope, but the WU does not own per-scope-type FK targets).

## Hidden-incomplete-work check

- No `#[allow(dead_code)]`, no `#[ignore]`, no `#[cfg(test)]` gating of production logic, no commented-out blocks.
- The three enums use `#[serde(rename_all = "snake_case")]` with explicit `as_str` and `parse` methods — no `Default::default()` fall-through and no derive-only path that could silently accept variants.
- `cargo clippy --all-targets --all-features -- -D warnings` is clean.
- `cargo fmt --check` is clean.

## Verdict

**LOW.** Every shortcut examined is a real implementation, not a stub:
- The `BudgetScopeType` / `BudgetState` / `PolicyAction` codecs go through a real macro generating real `Type`/`Encode`/`Decode`, exercised through three temp SQLx codec tables for each variant.
- `PRAGMA foreign_keys = ON` runs before every `insert_budget_ledger` and the FK miss is exercised through the database-level error mapping (`UnknownRef`).
- Non-negative counters are enforced both by `validate_budget_ledger` pre-DB and by per-column SQL `CHECK (col >= 0)`; zero is correctly preserved.
- The policy-action matrix is enforced both by `policy_action_allowed` pre-DB and by the SQL four-arm `CHECK` — defense-in-depth, real layering.
- `validate_budget_ledger` runs on both write *and* read (`TryFrom<BudgetLedgerRow>`) — defense-in-depth across the full repository surface.
- `ProviderStateRef` is reused from WU-0B-15 as a typed soft-FK marker; `BudgetScopeRef` is a typed marker. Neither is a stub. No `unwrap()`, no `todo!`, no TODO markers, no swallowing `_` catch-all.
