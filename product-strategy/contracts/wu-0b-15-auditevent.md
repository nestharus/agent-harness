# WU-0B-15 AuditEvent Contract

`AuditEvent` is an inert append-only local record of decisions and state changes. This WU owns only the GraphStore schema, typed Rust contract, repository insert/get/list boundary, and fixtures.

## Row

```text
AuditEvent {
  audit_event_id: OpaqueId<AuditEvent>,
  workspace_id: OpaqueId<GraphWorkspace>,
  event_type: String,
  actor: String,
  policy_set_id: OpaqueId<PolicySet>,
  configuration_id: Option<OpaqueId<GraphConfiguration>>,
  provider_state_id: Option<OpaqueId<ProviderStateRef>>,
  input_refs: JsonField<Vec<String>>,
  output_refs: JsonField<Vec<String>>,
  decision: AuditDecision,
  reason_code: String,
  created_at: Timestamp,
  meta: RecordMeta
}
```

`event_type` is a non-empty open-enum string. This WU does not freeze the event taxonomy.

`provider_state_id` is a soft reference until WU-0B-17 creates the provider-state target. It is validated as an opaque ID when present and has no FK in this WU.

## Decision Enum

- `accepted`
- `rejected`
- `deferred`
- `quarantined`
- `user_required`

Unknown decision values are rejected by serde and SQLx decode.

## Repository

- `insert_audit_event(AuditEvent) -> Result<AuditEvent, GraphStoreError>`
- `get_audit_event(&OpaqueId<AuditEvent>) -> Result<Option<AuditEvent>, GraphStoreError>`
- `list_audit_events_by_workspace(&OpaqueId<GraphWorkspace>) -> Result<Vec<AuditEvent>, GraphStoreError>`

Writes run inside explicit transactions and roll back on FK, enum, duplicate-ID, or invariant failure.

## Validation

Before insert, the repository validates every opaque ID, optional configuration/provider IDs when present, `RecordMeta`, `created_at`, non-empty `event_type`, non-empty `actor`, non-empty `reason_code`, and non-empty string refs inside `input_refs`/`output_refs`.

If `output_refs` is non-empty, `input_refs` must be non-empty. Because `policy_set_id` and `decision` are required typed fields, missing policy/decision cases are rejected either by opaque-ID validation, serde, or SQLite NOT NULL/CHECK constraints.

## Append Only

Audit events are append-only. The repository has insert and read paths only and must not contain `UPDATE audit_events` or `DELETE FROM audit_events`.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, recovery execution, or any durable table other than `audit_events`.
