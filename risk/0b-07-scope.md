# WU-0B-07 Scope Gate Review

**Severity:** LOW

## Result

All eleven ticket acceptance criteria are exercised by
`src-tauri/tests/wu_0b_07_graphnode_contract.rs`. The twelve declared
`GraphNode` fields appear in the struct, migration, fixtures, and
schema-introspection assertions. Every enum variant matrix has the
correct cardinality (`GraphNodeKind` 11, `LifecycleState` 9,
`PrivilegeOrigin` 7, `TrustState` 5), each variant is reached through
serde, SQLx encode/decode, and a real `insert_graph_node` call, and
unknown values are rejected at both the serde and SQLx layers. The
lifecycle matrix covers the nine valid transitions named in the
contract, two invalid transitions, and the deleted-terminal case.
Unknown-workspace, unknown-current-revision, and
unknown-canonical-parent inserts each roll back without leaving
durable rows.

## Verification

- Acceptance criterion → test mapping:
  - AC1 schema columns/FKs/unique/JSON checks/indexes/RecordMeta →
    `graph_nodes_schema_contains_declared_columns_constraints_and_indexes`
    (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:210`).
  - AC2 round-trip byte-equivalence →
    `insert_then_get_round_trips_every_graph_node_field_byte_equivalent`
    (`:314`).
  - AC3 transactional rollback on FK/enum/routing/state failures →
    `invalid_insert_paths_roll_back_without_partial_node_rows`
    (`:343`), with the `transition_lifecycle` half of AC3 covered by
    `lifecycle_transition_matrix_is_enforced_and_deleted_is_terminal`
    (`:664`) which asserts the row is byte-identical after each
    rejected transition.
  - AC4 no operator-visible behavior →
    `graphnode_wu_has_no_operator_visible_behavior_or_extra_tables`
    (`:376`).
  - AC5 contract-test boundary →
    `graphnode_contract_tests_stay_inside_declared_wu_boundary`
    (`:414`).
  - AC6 every `kind` variant round-trips and unknown is rejected →
    `every_kind_variant_round_trips_and_unknown_is_rejected` (`:450`),
    asserting `fixture.variants.len() == 11` (`:456`).
  - AC7 every `lifecycle_state` variant round-trips and unknown is
    rejected → `every_lifecycle_state_variant_round_trips_and_unknown_is_rejected`
    (`:503`), asserting `fixture.variants.len() == 9` (`:510`).
  - AC8 every `privilege_origin` variant round-trips and unknown is
    rejected →
    `every_privilege_origin_variant_round_trips_and_unknown_is_rejected`
    (`:556`), asserting `fixture.variants.len() == 7` (`:563`).
  - AC9 every `trust_state` variant round-trips and unknown is
    rejected →
    `every_trust_state_variant_round_trips_and_unknown_is_rejected`
    (`:610`), asserting `fixture.variants.len() == 5` (`:617`).
  - AC10 lifecycle transition matrix coverage →
    `lifecycle_transition_matrix_is_enforced_and_deleted_is_terminal`
    (`:664`), which iterates the nine valid transitions, the two
    invalid examples, and the deleted-terminal case from
    `product-strategy/contracts/fixtures/wu-0b-07/transition-matrix.json`.
  - AC11 unknown workspace / current-revision / canonical-parent refs
    rolled back → `invalid_insert_paths_roll_back_without_partial_node_rows`
    iterates the three named rollback fixtures and asserts
    `graph_node_row_count` is unchanged for each
    (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:347-363`).
- The `GraphNode` struct declares the twelve fields enumerated in
  `product-strategy/contracts/wu-0b-07-graphnode.md` (`node_id`,
  `workspace_id`, `kind`, `title`, `lifecycle_state`,
  `current_revision_id`, `canonical_parent_edge_id`,
  `privilege_origin`, `trust_state`, `created_from_ref`, `deleted_at`,
  `meta`) (`src-tauri/src/graphstore/graphnode.rs:75-90`). The schema
  test column list at `src-tauri/tests/wu_0b_07_graphnode_contract.rs:224-245`
  introspects every column pair, including the `RecordMeta`
  triplet (`created_at`, `updated_at`, `actor`,
  `record_policy_version`) and `schema_version`.
- Migration `src-tauri/migrations/0b/07_graphnode.sql:1-99` declares
  all twenty stored columns, the four enum `CHECK` constraints, the
  `json_valid(actor)` and `json_valid(created_from_ref)` JSON checks,
  `UNIQUE(node_id_value, node_id_namespace)`, the deleted/deleted_at
  invariant CHECK, the soft-ref pair-completeness CHECKs, and the four
  required indexes (`idx_graph_nodes_node_id`,
  `idx_graph_nodes_workspace_id`, `idx_graph_nodes_lifecycle_state`,
  `idx_graph_nodes_kind`). The schema test asserts each by name
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:251-310`).
- Variant fixtures match the contract verbatim:
  `product-strategy/contracts/fixtures/wu-0b-07/kind-variants.json`
  (11 strings),
  `lifecycle-state-variants.json` (9),
  `privilege-origin-variants.json` (7),
  `trust-state-variants.json` (5). Each variant test loops the
  fixture, calls `serde_json::from_value`, encodes/decodes through a
  temporary SQLite codec table, then inserts a unique row through
  `insert_graph_node`, satisfying "reachable through a documented
  fixture or input path."
- Unknown-variant fixture
  `product-strategy/contracts/fixtures/wu-0b-07/unknown-variants.json`
  supplies four out-of-range strings (`operator_panel`, `paused`,
  `administrator`, `trusted_by_default`); each variant test asserts
  serde rejection via `serde_json::from_value::<...>(...)` returning
  `Err` and SQLx rejection via a `SELECT '...' AS value` row whose
  `try_get::<_, Enum>` returns `Err`
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:492-499, 545-552,
  599-606, 653-660`). The implementation routes unknown enum strings
  through `<Enum>::parse` returning `GraphStoreError::InvalidEnum`
  (`src-tauri/src/graphstore/graphnode.rs:629, 660, 687, 710`).
- Lifecycle matrix fixture
  `product-strategy/contracts/fixtures/wu-0b-07/transition-matrix.json`
  enumerates the nine valid transitions with concrete from-states for
  the "any non-deleted state -> archived" and "any non-deleted state
  -> quarantined" cases (`recovering -> archived`,
  `unpacked -> quarantined`). Two invalid examples cover
  `packed -> active` and `recovering -> active`. The
  `deleted_terminal_target = "active"` exercises the deleted-as-source
  rejection.
- FK rejection on unknown `workspace_id` is asserted via the
  `unknown_workspace` rollback fixture and `assert_error(error,
  GraphStoreError::UnknownRef)` (`:355-362`); the implementation
  enables `PRAGMA foreign_keys = ON` before opening the transaction
  (`src-tauri/src/graphstore/graphnode.rs:126-129`) so the SQLite FK
  trips and `map_write_error` maps the database error to
  `UnknownRef`. The schema test additionally asserts the FK rows
  exposed by `PRAGMA foreign_key_list(graph_nodes)`
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:258-276`) and that
  no FK row exists for the `current_revision_id` or
  `canonical_parent_edge_id` columns (`:277-286`).
- Cargo workspace verification: `cargo test` (whole suite passes,
  including the ten new contract tests),
  `cargo clippy --all-targets -- -D warnings` (clean),
  `cargo fmt --check` (clean), `bun run typecheck` (clean),
  `bun run test` (86 tests pass), `bun run lint` (clean).

## Findings

### WU-0B-07-SCOPE-F01 (NIT)

The contract sentence "Repository writes run in explicit transactions
and roll back on FK, enum, soft-ref, routing, or state-transition
failure" lists five rejection causes. AC3 collapses
"routing" and "state-transition" into the same set of validations.
The contract test exercises FK (workspace), soft-ref (current
revision, canonical parent), routing/invariant (empty title), enum
(via the unknown-variants JSON / SQL row), and state-transition (via
the transition-matrix invalid + deleted-terminal cases). All five
contract surfaces are covered, but they are split across two
top-level tests rather than aggregated into a single rollback test;
a future scope reviewer should note the cross-test coverage to avoid
flagging it as a missing rollback case.

### WU-0B-07-SCOPE-F02 (NIT)

The two "any non-deleted state -> archived/quarantined" valid-matrix
entries are realized by exactly one source state each
(`recovering -> archived`, `unpacked -> quarantined`). The
implementation `is_valid_transition`
(`src-tauri/src/graphstore/graphnode.rs:481-482`) accepts every
non-deleted source as the wildcard branch
(`(_, LifecycleState::Archived)`, `(_, LifecycleState::Quarantined)`),
so the contract behavior holds for the eight other non-deleted
sources, but the contract fixture only sample-tests two of them.
Coverage of the wildcard remains AC-complete because the contract
language is "any non-deleted state -> archived/quarantined" rather
than "every"; documenting positive confirmation that the
implementation matches the wildcard branch by reading code is
sufficient for this gate.
