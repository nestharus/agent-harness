# WU-0B-06 Scope Gate Review

**Severity:** LOW

## Result

All eight ticket acceptance criteria are covered by `src-tauri/tests/wu_0b_06_graphworkspace_contract.rs`, the eight documented `GraphWorkspace` fields are present in struct, migration, and round-trip fixture, and the schema introspection asserts every declared column, FK, unique constraint, partial unique index, JSON `actor` check, and `RecordMeta` field. Mutator behavior matches the proposal: `set_active_configuration` mutates only `active_configuration_id` plus `updated_at` and `advance_graph_version` returns `OptimisticConflict` on stale `expected_version` without row mutation.

## Verification

- Acceptance criterion → test mapping:
  - AC1 schema columns/FKs/unique/indexes/RecordMeta →
    `graph_workspaces_schema_contains_declared_columns_constraints_and_indexes`
    (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:188`).
  - AC2 round-trip byte-equivalence →
    `insert_then_get_round_trips_every_graph_workspace_field_byte_equivalent`
    (`:293`).
  - AC3 transaction rollback on FK failure →
    `fk_failure_rolls_back_without_partial_workspace_rows` (`:322`).
  - AC4 no operator-visible behavior →
    `graphworkspace_wu_has_no_operator_visible_behavior` (`:340`).
  - AC5 contract-test boundary →
    `graphworkspace_contract_tests_stay_inside_declared_wu_boundary`
    (`:377`).
  - AC6 active-default uniqueness →
    `exactly_one_active_default_workspace_is_allowed` (`:407`).
  - AC7 `set_active_configuration` mutation surface →
    `set_active_configuration_updates_only_configuration_and_updated_at`
    (`:429`), backed by
    `assert_only_active_configuration_and_updated_at_changed`
    (`:144`).
  - AC8 `advance_graph_version` OCC →
    `advance_graph_version_uses_optimistic_concurrency` (`:480`),
    backed by `assert_only_graph_version_and_updated_at_changed`
    (`:163`).
- `GraphWorkspace` declares the eight fields named in
  `proposals/0b-06-wu-0b-06.md` (`workspace_id`, `schema_version`,
  `active_orchestrator_id`, `current_graph_version`, `storage_root`,
  `policy_set_id`, `active_configuration_id`, `is_active_default`) plus
  `meta: RecordMeta`
  (`src-tauri/src/graphstore/graphworkspace.rs:17-27`). Each field is
  introspected by the schema test column list
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:202-219`).
- Migration `src-tauri/migrations/0b/06_graphworkspace.sql:1-39` declares
  all sixteen columns, `UNIQUE(workspace_id_value,
  workspace_id_namespace)`, FK to `schema_versions(version)`, composite
  FKs to `policy_sets` and `graph_configurations`, three lookup indexes,
  and the partial `UNIQUE INDEX
  idx_graph_workspaces_one_active_default ... WHERE is_active_default =
  1`. The contract test asserts every FK via
  `PRAGMA foreign_key_list(graph_workspaces)`
  (`:233-262`) and asserts the partial-index `WHERE` clause via
  `sqlite_master`
  (`:283-289`).
- `set_active_configuration` only updates
  `active_configuration_id_value`, `active_configuration_id_namespace`,
  and `updated_at`
  (`src-tauri/src/graphstore/graphworkspace.rs:228-241`); the contract
  test asserts every other field, including `meta.created_at`,
  `meta.actor`, and `meta.policy_version`, is preserved
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:144-161`).
- `advance_graph_version` selects, compares to `expected_version`, then
  runs `UPDATE ... WHERE current_graph_version = ?` and treats
  `rows_affected() != 1` as `OptimisticConflict` without committing,
  while a stale read returns `OptimisticConflict` after rolling the
  transaction back
  (`src-tauri/src/graphstore/graphworkspace.rs:280-320`). The
  `advance-graph-version.json` fixture exercises the post-advance stale
  case (`expected_version=0`, `stale_expected_version=0`); the second
  call returns `OptimisticConflict` because
  `current_graph_version` is now `1`, and the test asserts the row's
  serialized form is unchanged after the failed call
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:498-513`).
- Active-default uniqueness is double-enforced (in-transaction count
  pre-check at
  `src-tauri/src/graphstore/graphworkspace.rs:90-99`, partial unique
  index at the schema layer); the contract test rejects the second
  default row with `InvariantViolation` and asserts the row count
  remains `1`
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:407-426`).
- Cargo workspace verification: `cargo test --tests` (all suites pass),
  `cargo clippy --all-targets --all-features -- -D warnings` (clean),
  `cargo fmt --check` (clean), `bun run typecheck` and `bun run test`
  (clean).

## Findings

### WU-0B-06-SCOPE-F01 (NIT)

`is_active_default` is documented in `proposals/0b-06-wu-0b-06.md` as the
eighth scalar required to satisfy AC6 even though the verbatim Contract
field list in the ticket only enumerates seven fields plus
`RecordMeta`. The proposal explicitly justifies the addition, and the
column is asserted by the schema test, partial unique index, and
round-trip fixture. Positive confirmation that the AC6-driven addition
is fully wired through schema, repository, fixture, and assertion
surfaces.
