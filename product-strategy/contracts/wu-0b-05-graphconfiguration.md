# WU-0B-05 Contract: GraphConfiguration

## Surface

WU-0B-05 owns the inert durable GraphConfiguration schema and typed Rust repository:

```text
EffectiveValueSource = system_required | default | inherited | user_configured | recovered

EffectiveValueSourceEntry {
  field_name: String,
  source: EffectiveValueSource,
  source_ref: String
}

ValidationState =
  valid | valid_with_warnings | invalid_schema | invalid_provider_route |
  invalid_budget | invalid_index | needs_user_attention

GraphConfiguration {
  configuration_id: OpaqueId<GraphConfiguration>,
  workspace_id: OpaqueId<GraphWorkspace>,
  configuration_version: i64,
  schema_profile: String,
  summary_contract_template_ids: JsonField<Vec<String>>,
  optimizer_policy_ref: OpaqueId<PolicySet>,
  render_policy_ref: OpaqueId<PolicySet>,
  memory_policy_ref: OpaqueId<PolicySet>,
  provider_routing_policy_ref: OpaqueId<PolicySet>,
  capability_fingerprint_policy_ref: OpaqueId<PolicySet>,
  effective_value_sources: JsonField<Vec<EffectiveValueSourceEntry>>,
  created_from_configuration_id: Option<OpaqueId<GraphConfiguration>>,
  validation_state: ValidationState,
  meta: RecordMeta
}

GraphConfigurationRepo {
  new(pool: SqlitePool)
  insert_graph_configuration(record) -> Result<GraphConfiguration, GraphStoreError>
  get_graph_configuration(id) -> Result<Option<GraphConfiguration>, GraphStoreError>
  list_graph_configurations_by_workspace(workspace_id) -> Result<Vec<GraphConfiguration>, GraphStoreError>
}
```

`GraphConfigurationRepo` also implements WU-0B-03 `GraphStoreRepo<GraphConfiguration>`.

`GraphWorkspace` is a marker type only in this WU. `workspace_id` is stored as opaque value/namespace text and has no FK target until WU-0B-06.

## Schema

The shipped migration path is `src-tauri/migrations/0b/05_graphconfiguration.sql`. The migration runner must discover it recursively from `src-tauri/migrations`.

`graph_configurations` must include:

- opaque ID columns for `configuration_id` and `workspace_id`
- scalar columns for `configuration_version`, `schema_profile`, and `validation_state`
- JSON columns for `summary_contract_template_ids`, `effective_value_sources`, and RecordMeta `actor`
- opaque value/namespace columns for each policy ref
- nullable opaque value/namespace columns for `created_from_configuration_id`
- RecordMeta columns: `created_at`, `updated_at`, `actor`, `record_policy_version`
- `schema_version` with a FK to `schema_versions(version)`
- `UNIQUE(configuration_id_value, configuration_id_namespace)`
- composite FKs from every policy ref to `policy_sets(policy_set_id_value, policy_set_id_namespace)`
- indexes on configuration lookup, workspace listing, and revision parent lookup

The migration must create no durable table other than `graph_configurations`.

## Validation

Before insert, the repository validates:

- every opaque ID field
- `RecordMeta` through WU-0B-02 `validate_record_meta`
- `configuration_version > 0`
- non-empty `schema_profile`
- non-empty `summary_contract_template_ids` with no empty template id
- `effective_value_sources` contains entries for the effective configuration fields documented in the proposal
- every source entry has non-empty `field_name` and non-empty `source_ref`

Empty or missing source attribution returns `GraphStoreError::InvariantViolation`.

## Enums

Every `EffectiveValueSource` variant must round-trip through serde and SQLx:

- `system_required`
- `default`
- `inherited`
- `user_configured`
- `recovered`

Every `ValidationState` variant must round-trip through serde and SQLx:

- `valid`
- `valid_with_warnings`
- `invalid_schema`
- `invalid_provider_route`
- `invalid_budget`
- `invalid_index`
- `needs_user_attention`

Unknown enum strings are rejected by serde/SQLx decode and documented by fixtures.

## Transactions and Revisions

`insert_graph_configuration` must use an explicit SQLite transaction. Any validation, duplicate ID, FK, enum, or revision-transition failure rolls back and leaves the row count unchanged.

Rows are append-only. There is no update or upsert method. `created_from_configuration_id` may be null for a first revision. When present, it must reference an existing graph configuration in the same workspace whose `created_at` is smaller than the child row. Missing parent returns `UnknownRef`; wrong workspace or non-older parent returns `InvalidTransition`.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, recovery execution, or any domain table other than `graph_configurations`.

## Fixtures

- `canonical-row.json`: complete canonical GraphConfiguration row.
- `round-trip.json`: row used for insert/get byte-equivalent assertions.
- `effective-value-source-variants.json`: one row per source variant.
- `validation-state-variants.json`: one row per validation-state variant.
- `unknown-effective-value-source.json`: unknown source rejection fixture.
- `unknown-validation-state.json`: unknown validation-state rejection fixture.
- `append-only-revision.json`: first and second revisions in one workspace.
- `source-attribution-rejection.json`: source entry with missing `source_ref`.
- `transaction-rollback-fk.json`: row whose policy refs do not exist in `policy_sets`.
