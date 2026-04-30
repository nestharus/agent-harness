# WU-0B-07 Shortcut Gate Review

**Severity:** LOW

## Result

The WU-0B-07 implementation does the real work end to end: a single
`impl_sqlite_text_enum!` macro provides the SQLx `Type`/`Encode`/
`Decode` triple for all four enums, `PRAGMA foreign_keys = ON` is
issued before every transaction, `transition_lifecycle` performs a
real source-state SELECT before issuing a real `UPDATE` and re-SELECT
inside the same transaction, the `deleted` source state is rejected
via an explicit early return in `is_valid_transition`, and the
soft-reference columns (`current_revision_id`,
`canonical_parent_edge_id`) are rejected with `UnknownRef` on insert
because their target tables (`node_revisions`, `graph_edges`) do not
yet exist. No `unwrap()`, `todo!()`, `unimplemented!()`, `TODO`, or
`FIXME` appear in the production paths.

## Verification

- `impl_sqlite_text_enum!` is a real macro
  (`src-tauri/src/graphstore/graphnode.rs:715-744`) implementing
  `Type<Sqlite>`, `Encode<'q, Sqlite>`, and `Decode<'r, Sqlite>` for
  the named type. It is invoked four times — once per enum
  (`:746-749`) — and the encode path serializes `as_str()` while the
  decode path delegates to `<Enum>::parse`, which returns
  `GraphStoreError::InvalidEnum` on unknown strings. Manual `Encode`
  /`Decode` impls verified for `GraphNodeKind`, `LifecycleState`,
  `PrivilegeOrigin`, and `TrustState` (each has both an `as_str` and a
  `parse` arm covering every documented variant; verified via
  `:599-713`).
- `PRAGMA foreign_keys = ON` is set inside both write entry points
  before the transaction opens (`src-tauri/src/graphstore/graphnode.rs:126-129`
  for `insert_graph_node`, `:273-276` for `transition_lifecycle`).
  Combined with the migration FK at
  `src-tauri/migrations/0b/07_graphnode.sql:83-84`, an unknown
  `workspace_id` triggers `FOREIGN KEY constraint failed`, which
  `map_write_error` maps to `GraphStoreError::UnknownRef`
  (`src-tauri/src/graphstore/graphnode.rs:497-498`), satisfying AC11.
- `transition_lifecycle` selects the current row inside the
  transaction via `select_node`
  (`src-tauri/src/graphstore/graphnode.rs:279`), validates the source
  state against the contract matrix
  (`is_valid_transition`, `:287`), issues a real
  `UPDATE graph_nodes SET lifecycle_state = ?, updated_at = ?` keyed
  on the node ID (`:293-304`), checks `rows_affected() == 1`
  (`:313-316`), then re-selects the updated row before commit
  (`:318-321`). On any failure the transaction is rolled back via
  `rollback(transaction).await?`. The valid- and invalid-transition
  loops in the contract test confirm both branches; the invalid loop
  asserts `serde_json::to_string(&after) == before`
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:715-723`).
- Deleted-as-source is rejected by the explicit early return
  `if source == LifecycleState::Deleted || target == LifecycleState::Deleted
  { return false; }` (`src-tauri/src/graphstore/graphnode.rs:469-471`),
  ahead of the `matches!` matrix. The deleted-terminal test inserts a
  `LifecycleState::Deleted` node (with `deleted_at` set so the
  `validate_graph_node` invariant holds) and asserts
  `transition_lifecycle(... , Active)` returns
  `GraphStoreError::InvalidTransition` while the row remains
  byte-identical
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:726-748`).
- Future-WU soft references (`current_revision_id`,
  `canonical_parent_edge_id`) are stored as nullable column pairs
  with no FK target — the migration's
  `PRAGMA foreign_key_list(graph_nodes)` exposes only the
  `graph_workspaces` and `schema_versions` FKs
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:258-286` asserts
  this absence). On insert, `insert_graph_node` rejects any non-null
  soft ref with `GraphStoreError::UnknownRef` before issuing the SQL
  statement (`src-tauri/src/graphstore/graphnode.rs:140-143`), and
  the rollback fixture's `unknown_current_revision` and
  `unknown_canonical_parent` cases verify both paths
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:347-363`). The
  proposal explicitly documents this as the
  "rejected as `UnknownRef` until those tables exist"
  shortcut (`proposals/0b-07-wu-0b-07.md:20`), and the contract
  states "WU-0B-07 rejects non-null values as `UnknownRef` until those
  tables exist"
  (`product-strategy/contracts/wu-0b-07-graphnode.md:24`).
  WU-0B-08 (`GraphEdge`) and WU-0B-12 (`NodeRevision`) own the future
  conversion to real FKs.
- The pair-completeness CHECK constraints
  (`src-tauri/migrations/0b/07_graphnode.sql:68-78`) reject half-set
  soft refs at the schema layer; `TryFrom<GraphNodeRow>` also defends
  against half-set state by returning `InvariantViolation`
  (`src-tauri/src/graphstore/graphnode.rs:366, 376`).
- `Grep "unwrap\b|todo!\(|unimplemented!\(|TODO|FIXME"` against
  `src-tauri/src/graphstore/graphnode.rs`,
  `src-tauri/src/contracts/graphnode.rs`, and
  `src-tauri/migrations/0b/07_graphnode.sql` returns no matches.
  `expect(...)` only appears inside the contract test, which is
  acceptable test-side code.
- `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`,
  and `cargo test` all clean; `bun run typecheck`, `bun run lint`, and
  `bun run test` all clean.

## Findings

### WU-0B-07-SHORTCUT-F01 (NIT)

`next_contract_timestamp`
(`src-tauri/src/graphstore/graphnode.rs:515-567`) is a hand-rolled
deterministic timestamp incrementer that advances `updated_at` by one
second using calendar arithmetic rather than reading a real clock.
This shortcut is consistent with the equivalent helper introduced in
WU-0B-06 (`graphworkspace.rs`); it satisfies the contract test
assertion `assert_ne!(updated.meta.updated_at, record.meta.updated_at)`
(`src-tauri/tests/wu_0b_07_graphnode_contract.rs:690`) without
introducing a global clock dependency before the phase that defines
one. Acceptable for Phase 0B; flag for the phase that introduces a
real clock so this duplicated helper can be unified rather than
persisted as a parallel timestamp source.

### WU-0B-07-SHORTCUT-F02 (NIT)

`insert_graph_node` performs an in-application
`SELECT COUNT(*) FROM graph_nodes WHERE node_id_value = ? AND
node_id_namespace = ?` duplicate-id pre-check
(`src-tauri/src/graphstore/graphnode.rs:145-156`) ahead of the SQL
INSERT. The SQLite layer would also reject the duplicate via
`UNIQUE(node_id_value, node_id_namespace)` and `map_write_error`
correctly maps that to `GraphStoreError::DuplicateId`
(`:495-496`). The pre-check is redundant but not incorrect; it
mirrors the WU-0B-06 active-default pattern and is consistent with
the prior WUs' "double-enforce" stance. No fix required.
