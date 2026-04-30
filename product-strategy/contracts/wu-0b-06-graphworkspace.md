# WU-0B-06 Contract: GraphWorkspace

## Surface

WU-0B-06 owns the inert durable GraphWorkspace schema and typed Rust repository:

```text
GraphWorkspace {
  workspace_id: OpaqueId<GraphWorkspace>,
  schema_version: i64,
  active_orchestrator_id: OpaqueId<Orchestrator>,
  current_graph_version: i64,
  storage_root: String,
  policy_set_id: OpaqueId<PolicySet>,
  active_configuration_id: OpaqueId<GraphConfiguration>,
  is_active_default: bool,
  meta: RecordMeta
}

GraphWorkspaceRepo {
  new(pool: SqlitePool)
  insert_graph_workspace(record) -> Result<GraphWorkspace, GraphStoreError>
  get_graph_workspace(id) -> Result<Option<GraphWorkspace>, GraphStoreError>
  list_graph_workspaces_by_namespace(namespace) -> Result<Vec<GraphWorkspace>, GraphStoreError>
  set_active_configuration(workspace_id, configuration_id) -> Result<GraphWorkspace, GraphStoreError>
  advance_graph_version(workspace_id, expected_version) -> Result<GraphWorkspace, GraphStoreError>
}
```

`schema_version` is a FK to `schema_versions(version)` and records the migration contract version used for the workspace row. `current_graph_version` starts at `0`. `active_orchestrator_id` is an opaque value/namespace pair only; this WU adds no orchestrator FK.

## Schema

The shipped migration path is `src-tauri/migrations/0b/06_graphworkspace.sql`. The migration runner must discover it recursively from `src-tauri/migrations`.

`graph_workspaces` must include:

- opaque ID columns for `workspace_id`, `active_orchestrator_id`, `policy_set_id`, and `active_configuration_id`
- scalar columns for `schema_version`, `current_graph_version`, `storage_root`, and `is_active_default`
- RecordMeta columns: `created_at`, `updated_at`, `actor`, `record_policy_version`
- `UNIQUE(workspace_id_value, workspace_id_namespace)`
- FK `schema_version -> schema_versions(version)`
- composite FK `policy_set_id -> policy_sets(policy_set_id_value, policy_set_id_namespace)`
- composite FK `active_configuration_id -> graph_configurations(configuration_id_value, configuration_id_namespace)`
- indexes on workspace lookup, policy set lookup, and active configuration lookup
- partial unique index enforcing at most one `is_active_default = 1` row

The migration must create no durable table other than `graph_workspaces`.

## Validation

Before insert, the repository validates:

- every opaque ID field
- `RecordMeta` through WU-0B-02 `validate_record_meta`
- `schema_version > 0`
- `current_graph_version >= 0`
- non-empty `storage_root`

Duplicate workspace IDs return `GraphStoreError::DuplicateId`. Missing policy, configuration, or schema-version FK targets return `GraphStoreError::UnknownRef`. A second active default workspace returns `GraphStoreError::InvariantViolation`.

## Mutators

`set_active_configuration(workspace_id, configuration_id)` runs inside an explicit transaction. It validates both IDs, verifies the target configuration exists, updates only `active_configuration_id` and `RecordMeta.updated_at`, and returns the updated row. Unknown workspace IDs or configuration IDs return `UnknownRef` without partial writes.

`advance_graph_version(workspace_id, expected_version)` runs inside an explicit transaction. It validates `expected_version >= 0`, reads the current version, compares it to `expected_version`, and updates only `current_graph_version` plus `RecordMeta.updated_at` when they match. Stale expected versions return `GraphStoreError::OptimisticConflict` without changing the row.

All other fields are append-only after insert.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, recovery execution, graph nodes, graph edges, or any domain table other than `graph_workspaces`.

## Fixtures

- `canonical-row.json`: complete canonical GraphWorkspace row.
- `round-trip.json`: row used for insert/get byte-equivalent assertions.
- `active-default-uniqueness.json`: two default rows; the first inserts and the second is rejected.
- `set-active-configuration.json`: base row, replacement configuration ID, and unknown configuration ID.
- `advance-graph-version.json`: base row, expected version, and stale version.
- `transaction-rollback-fk.json`: row whose policy/configuration refs do not exist.
