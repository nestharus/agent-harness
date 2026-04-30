# WU-0B-06 Shortcut Gate Review

**Severity:** LOW

## Result

The WU-0B-06 implementation does the real work end to end: foreign keys
are enforced via `PRAGMA foreign_keys = ON` per repository call,
optimistic concurrency in `advance_graph_version` runs through a real
`UPDATE ... WHERE current_graph_version = ?` with a `rows_affected()`
check, the partial unique index on `is_active_default = 1` is enforced
at the schema layer (and the repository also pre-checks), and
`set_active_configuration` validates the target configuration via a
SELECT before mutating. Append-only fields cannot be reached by any
UPDATE path. No `unwrap()` or `TODO`s appear in production paths.

## Verification

- `PRAGMA foreign_keys = ON` is set inside every repository call before
  the transaction opens
  (`src-tauri/src/graphstore/graphworkspace.rs:63-66`,
  `:192-195`,
  `:267-270`).
  Combined with the migration FKs at
  `src-tauri/migrations/0b/06_graphworkspace.sql:20-25`, FK targets are
  enforced at the database layer; the `transaction-rollback-fk.json`
  fixture exercises this end to end and the contract test asserts row
  count is unchanged after the rollback
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:322-337`).
- `advance_graph_version` performs an in-transaction SELECT, compares
  versions, then issues a real `UPDATE graph_workspaces SET
  current_graph_version = ?, updated_at = ? WHERE workspace_id_value =
  ? AND workspace_id_namespace = ? AND current_graph_version = ?`.
  Mismatched rows produce `rows_affected() != 1`, which rolls the
  transaction back and returns `GraphStoreError::OptimisticConflict`
  (`src-tauri/src/graphstore/graphworkspace.rs:262-326`). The contract
  test demonstrates the row is byte-identical after a stale call
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:498-513`).
- Active-default uniqueness is enforced both in the application layer
  (in-transaction `SELECT COUNT(*) FROM graph_workspaces WHERE
  is_active_default = 1`,
  `src-tauri/src/graphstore/graphworkspace.rs:90-99`) and at the
  schema layer (`CREATE UNIQUE INDEX
  idx_graph_workspaces_one_active_default ON
  graph_workspaces(is_active_default) WHERE is_active_default = 1`,
  `src-tauri/migrations/0b/06_graphworkspace.sql:36-38`). The contract
  test asserts the partial index `WHERE` clause via `sqlite_master`
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:283-289`) and
  exercises the rejection path
  (`:407-426`). The `map_write_error` fallback also recognizes the
  partial index name if the schema constraint trips before the
  application check
  (`src-tauri/src/graphstore/graphworkspace.rs:461-464`).
- `set_active_configuration` validates the configuration target via
  `SELECT COUNT(*) FROM graph_configurations WHERE
  configuration_id_value = ? AND configuration_id_namespace = ?` and
  rejects unknown IDs with `UnknownRef` *before* issuing the UPDATE
  (`src-tauri/src/graphstore/graphworkspace.rs:214-225`). It then
  updates only `active_configuration_id_value`,
  `active_configuration_id_namespace`, and `updated_at`
  (`:228-241`), and `assert_only_active_configuration_and_updated_at_changed`
  asserts every other field, including the entire `RecordMeta` minus
  `updated_at`, is preserved
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:144-161`).
- Append-only enforcement: the only `UPDATE graph_workspaces` statements
  in the implementation are at `:229` (set active configuration) and
  `:295` (advance graph version), so `storage_root`, `schema_version`,
  `policy_set_id_value/namespace`, `active_orchestrator_id_value/namespace`,
  `is_active_default`, `created_at`, `actor`, and
  `record_policy_version` have no mutator path. Verified via
  `Grep "UPDATE graph_workspaces"` returning exactly two hits.
- `Grep "unwrap\(\)|TODO|todo!|unimplemented!|panic!\("` against
  `src-tauri/src/graphstore/graphworkspace.rs` returns no matches.
- `cargo clippy --all-targets --all-features -- -D warnings` and
  `cargo fmt --check` are clean (turbo-cached); the full `cargo test`
  suite passes including the eight new contract tests.

## Findings

### WU-0B-06-SHORTCUT-F01 (NIT)

`next_contract_timestamp`
(`src-tauri/src/graphstore/graphworkspace.rs:477-559`) advances
`updated_at` by exactly one second using a hand-rolled date
incrementer rather than reading a real clock. This is a deterministic
shortcut chosen because the WU is contract-test-only and the contract
asserts `assert_ne!(after.meta.updated_at, before.meta.updated_at)`
without a clock requirement. Acceptable for Phase 0B; flag for the
phase that introduces a real clock so this helper can be replaced
rather than persisted as a parallel timestamp source.
