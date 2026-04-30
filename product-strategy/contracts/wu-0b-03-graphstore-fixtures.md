# WU-0B-03 Contract: GraphStore Repository Harness and Fixture Builder

## Surface

This WU owns a Rust test-harness fixture for GraphStore repository contract tests:

```text
GraphStorePool {
  sqlite_url: String,
  sqlite: sqlx::SqlitePool,
  migrations_applied: Vec<i64>,
  workspace_root: PathBuf
}

GraphStoreFixture {
  pool: GraphStorePool,
  workspace_id: Option<OpaqueId<GraphWorkspaceRef>>,
  graph_version: Option<i64>,
  created_refs: Vec<OpaqueId<GraphStoreFixtureRef>>
}

GraphStoreRepo<T> {
  insert(record) -> Result<T, GraphStoreError>
  get(id) -> Result<Option<T>, GraphStoreError>
  list_by_workspace(workspace_id) -> Result<Vec<T>, GraphStoreError>
}

create_graphstore_fixture(seed_plan) -> Result<GraphStoreFixture, GraphStoreError>
reset_graphstore_fixture(fixture) -> Result<(), GraphStoreError>
validate_fixture_created_refs(fixture) -> Result<(), GraphStoreError>
```

The fixture composes WU-0A-14a `temp_harness_state`, WU-0B-01 `run_migrations`, and WU-0B-02 `OpaqueId<T>` / `GraphStoreError`.

## Seed Plan

For WU-0B-03, the only accepted seed plan is empty:

```json
{}
```

A seed plan requesting any future-WU object returns `GraphStoreError::UnknownRef`. Rejection must happen before temp harness creation or migration execution so invalid input cannot leave partial database state.

Future WUs may add typed seed variants. They must not weaken current empty-seed behavior and must preserve the future-WU rejection rule for refs outside the current dependency graph.

## Create Behavior

`create_graphstore_fixture(empty_seed_plan)`:

1. Opens a fresh temp SQLite database through `temp_harness_state("empty", None, None, [])`.
2. Runs the shipped migrations directory through `run_migrations`.
3. Returns `migrations_applied == [1]` for a fresh fixture when only `0001_schema_versions.sql` exists.
4. Returns `workspace_id == None`, `graph_version == None`, and `created_refs == []` in WU-0B-03.
5. Creates no durable table except `schema_versions`.

Two fixtures created in the same process must have different temp roots and SQLite URLs.

## Reset Behavior

`reset_graphstore_fixture(fixture)` deletes WU-owned rows and preserves migration history exactly. WU-0B-03 owns no domain rows, so reset must leave `schema_versions` row count and row content unchanged. After reset, rerunning the shipped migration directory must skip version `1` rather than reapply or rewrite it.

## Repository Contract

`GraphStoreRepo<T>` is a typed trait boundary for downstream WUs. WU-0B-03 must not introduce a dynamic catch-all repository or domain table. Downstream WUs define their own typed implementations.

## No Side Effects

The fixture builder must not:

- call `Command::new`, `std::process::Command`, or `tokio::process`;
- invoke real `agents`;
- read or probe provider configuration;
- register Tauri commands or edit invoke handlers;
- write UI panes or render outputs;
- run optimizer or recovery code;
- create migrations beyond `0001_schema_versions.sql`;
- create Phase 0B domain tables.

## Fixtures

- `empty-seed-plan.json`
- `unknown-ref-seed-plan.json`
- `graphstore-pool-shape.json`
- `graphstore-fixture-shape.json`
- `reset-preserves-migration-history.json`
- `no-side-effects-assertions.json`
