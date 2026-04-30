# WU-0B-05 Scope Gate Review

**Severity:** LOW

## Result

All 9 acceptance criteria are exercised by the contract test. The 14 documented
GraphConfiguration fields, 5 policy-set FKs, JSON CHECK columns, three indexes,
unique constraint, and `RecordMeta` columns are all present in the migration and
introspected by the schema test.

## Verification

- Migration `src-tauri/migrations/0b/05_graphconfiguration.sql` declares all 26
  expected columns, `UNIQUE(configuration_id_value, configuration_id_namespace)`,
  five composite policy FKs to `policy_sets(policy_set_id_value,
  policy_set_id_namespace)`, `json_valid(...)` checks for
  `summary_contract_template_ids`, `effective_value_sources`, and `actor`,
  CHECK on `validation_state` enum, and the three required indexes
  (`idx_graph_configurations_configuration_id`,
  `idx_graph_configurations_workspace_id`,
  `idx_graph_configurations_created_from`).
- `graph_configurations_schema_contains_declared_columns_constraints_and_indexes`
  introspects DDL, `PRAGMA foreign_key_list`, and `PRAGMA index_list`.
- `insert_then_get_round_trips_every_graph_configuration_field_byte_equivalent`
  asserts JSON-string equality of the inserted record vs the fetched record using
  the `round-trip.json` fixture, covering every scalar, enum, JSON, timestamp,
  and opaque-ID field.
- All five `EffectiveValueSource` variants and all seven `ValidationState`
  variants are reached by `every_effective_value_source_variant_round_trips_*`
  and `every_validation_state_variant_round_trips_*`, including a real `INSERT`
  through the repository per variant. Unknown serde and SQLx decode paths are
  rejected (`unknown-effective-value-source.json`,
  `unknown-validation-state.json`).
- `created_from_configuration_id_is_null_or_older_same_workspace_revision`
  inserts a null-parent first revision, an older same-workspace second
  revision, then exercises `InvalidTransition` for different-workspace and
  non-older-`created_at` parents and `UnknownRef` for a missing parent. Row
  count remains at 2 after all rejections.
- `effective_value_source_missing_source_ref_returns_invariant_violation`
  exercises the source-attribution rejection path against the
  `source-attribution-rejection.json` fixture.

## Findings

### WU-0B-05-SCOPE-F01 (NIT)

`capability_fingerprint_policy_ref` is declared in the proposal as the fifth
policy ref and is present in struct, migration, FK list, and round-trip
fixture. No issue — this is a positive confirmation that the explicit reviewer
ask was met.
