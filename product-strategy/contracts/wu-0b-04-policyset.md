# WU-0B-04 Contract: PolicySet

## Surface

WU-0B-04 owns the inert durable PolicySet schema and typed Rust repository:

```text
PolicyVersionRef {
  policy: String,
  version: String,
  previous_version: Option<String>,
  payload: serde_json::Value
}

PolicySet {
  policy_set_id: OpaqueId<PolicySet>,
  summary_contract_version: JsonField<PolicyVersionRef>,
  render_policy_version: JsonField<PolicyVersionRef>,
  identity_policy_version: JsonField<PolicyVersionRef>,
  privilege_policy_version: JsonField<PolicyVersionRef>,
  tool_protocol_policy_version: JsonField<PolicyVersionRef>,
  budget_policy_version: JsonField<PolicyVersionRef>,
  review_sampling_policy_version: JsonField<PolicyVersionRef>,
  recovery_policy_version: JsonField<PolicyVersionRef>,
  configuration_policy_version: JsonField<PolicyVersionRef>,
  provider_policy_version: JsonField<PolicyVersionRef>,
  meta: RecordMeta
}

PolicySetRepo {
  new(pool: SqlitePool)
  insert_policy_set(record) -> Result<PolicySet, GraphStoreError>
  get_policy_set(id) -> Result<Option<PolicySet>, GraphStoreError>
  list_policy_sets_by_workspace(workspace_id) -> Result<Vec<PolicySet>, GraphStoreError>
}
```

`PolicySetRepo` also implements WU-0B-03 `GraphStoreRepo<PolicySet>`.

## Schema

The shipped migration path is `src-tauri/migrations/0b/04_policyset.sql`. The migration runner must discover it recursively from `src-tauri/migrations`.

`policy_sets` must include:

- opaque ID columns: `policy_set_id_value`, `policy_set_id_namespace`
- ten JSON policy-version columns named after the Rust fields
- RecordMeta columns: `created_at`, `updated_at`, `actor`, `record_policy_version`
- `schema_version` with a FK to `schema_versions(version)`
- `UNIQUE(policy_set_id_value, policy_set_id_namespace)`
- an index on `(policy_set_id_value, policy_set_id_namespace)`
- an index suitable for namespace/workspace listing

## Validation

Every policy-version field rejects these modes with `GraphStoreError::InvariantViolation`:

- `empty`: empty or whitespace `policy` or `version`
- `unknown`: `policy` does not match the expected field token
- `malformed`: `version` or `previous_version` is not `vMAJOR.MINOR.PATCH`
- `non_monotonic`: `previous_version` is present and `version <= previous_version`

Expected tokens:

| Field | token |
| --- | --- |
| `summary_contract_version` | `summary_contract` |
| `render_policy_version` | `render` |
| `identity_policy_version` | `identity` |
| `privilege_policy_version` | `privilege` |
| `tool_protocol_policy_version` | `tool_protocol` |
| `budget_policy_version` | `budget` |
| `review_sampling_policy_version` | `review_sampling` |
| `recovery_policy_version` | `recovery` |
| `configuration_policy_version` | `configuration` |
| `provider_policy_version` | `provider` |

`RecordMeta` is validated through WU-0B-02 `validate_record_meta`.

## Transactions and Immutability

`insert_policy_set` must use an explicit SQLite transaction. Any validation, duplicate ID, FK, enum, routing, or state-transition failure rolls back and leaves the policy row count unchanged.

Governance updates are append-only. A later governance bundle uses a new `policy_set_id`; existing rows are not updated and remain retrievable by ID.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, reviewer sampling, recovery execution, or any domain table other than `policy_sets`.

## Fixtures

- `canonical-row.json`: complete canonical PolicySet row.
- `round-trip.json`: row used for insert/get byte-equivalent assertions.
- `rejection-matrix.json`: every field appears under every rejection mode.
- `append-only-governance.json`: two rows in the same namespace with increasing policy versions.
- `no-side-effects.json`: structural and runtime absence assertions.
