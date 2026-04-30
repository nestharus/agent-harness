# WU-0B-17 ProviderState Contract

`ProviderState` is an inert, redacted local observation of provider, CLI, account, auth, billing, quota, network, runtime, sandbox, freshness, and confidence state. This WU owns only the GraphStore schema, typed Rust contract, repository insert/get/list boundary, stale-marking boundary, and fixtures.

## Row

```text
ProviderState {
  provider_state_id: OpaqueId<ProviderStateRef>,
  workspace_id: OpaqueId<GraphWorkspace>,
  provider: ProviderKind,
  cli: ProviderCli,
  account_ref: String,
  auth_state: AuthState,
  billing_state: BillingState,
  quota_state: QuotaState,
  network_state: NetworkState,
  runtime_state: RuntimeState,
  sandbox_constraints: JsonField<Value>,
  store_locations_checked: JsonField<Vec<String>>,
  secret_material_stored: bool,
  freshness: ProviderFreshness,
  confidence: ProviderConfidence,
  last_probe_at: Timestamp,
  meta: RecordMeta
}
```

`secret_material_stored` is always false. Repository validation rejects true values and credential-looking payloads before persistence; the database also has a `CHECK (secret_material_stored = 0)` constraint.

## Enums

- `provider`: `anthropic`, `openai`, `google`, `local_runtime`, `openai_compatible`, `other`
- `cli`: `claude`, `codex`, `opencode`, `agent_runner`
- `auth_state`: `present`, `missing`, `expired`, `invalid`, `unknown`
- `billing_state`: `healthy`, `near_limit`, `over_limit`, `payment_required`, `unknown`
- `quota_state`: `available`, `rate_limited`, `exhausted`, `unknown`
- `network_state`: `available`, `blocked_by_sandbox`, `blocked_by_host`, `degraded`, `unknown`
- `runtime_state`: `installed`, `missing`, `wrong_version`, `unreachable`, `not_applicable`, `unknown`
- `freshness`: `fresh`, `stale`, `probe_failed`, `manual`
- `confidence`: `high`, `medium`, `low`

Unknown enum values are rejected by serde and SQLx decode.

## Repository

- `insert_provider_state(ProviderState) -> Result<ProviderState, GraphStoreError>`
- `get_provider_state(&OpaqueId<ProviderStateRef>) -> Result<Option<ProviderState>, GraphStoreError>`
- `list_provider_states_by_workspace(&OpaqueId<GraphWorkspace>) -> Result<Vec<ProviderState>, GraphStoreError>`
- `mark_provider_state_stale(&OpaqueId<ProviderStateRef>, &str) -> Result<ProviderState, GraphStoreError>`

Writes run inside explicit transactions and roll back on FK, enum, duplicate-ID, malformed stale reason, unknown provider state, or invariant failure.

## Stale Marking

`mark_provider_state_stale` accepts a non-empty ASCII reason containing only lowercase letters, digits, `_`, `-`, `.`, or `:`. It updates only `freshness`, `confidence`, `meta.updated_at`, and `meta.actor`; all other columns must remain byte-equivalent.

The stale result is `freshness = stale`, `confidence = low`, and `meta.actor.value = provider-state-stale:{reason}`.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, recovery execution, or any durable table other than `provider_states`.
