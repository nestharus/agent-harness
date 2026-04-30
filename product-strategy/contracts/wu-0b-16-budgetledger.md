# WU-0B-16 BudgetLedger Contract

`BudgetLedger` is an inert local record of token, latency, provider-cost, cache, and policy-action measurements. This WU owns only the GraphStore schema, typed Rust contract, repository insert/get/list boundary, and fixtures.

## Row

```text
BudgetLedger {
  budget_ledger_id: OpaqueId<BudgetLedger>,
  workspace_id: OpaqueId<GraphWorkspace>,
  scope_type: BudgetScopeType,
  scope_id: OpaqueId<BudgetScopeRef>,
  input_tokens: i64,
  output_tokens: i64,
  cache_read_tokens: i64,
  cache_write_tokens: i64,
  latency_ms: i64,
  provider_cost_estimate: i64,
  provider_state_id: Option<OpaqueId<ProviderStateRef>>,
  cache_prefix_hash: Option<String>,
  budget_state: BudgetState,
  policy_action: PolicyAction,
  meta: RecordMeta
}
```

`provider_state_id` is a soft reference until WU-0B-17 creates the provider-state target. It is validated as an opaque ID when present and has no FK in this WU.

## Scope Type Enum

- `workspace`
- `initiative`
- `orchestrator_turn`
- `worker_run`
- `optimizer_pass`
- `reviewer_pass`
- `render`

## Budget State Enum

- `within`
- `near_limit`
- `exceeded`
- `blocked`

## Policy Action Enum

- `none`
- `warn`
- `narrow_scope`
- `require_user_approval`
- `block`

Unknown enum values are rejected by serde and SQLx decode.

## Policy-Action Matrix

| budget_state | allowed policy_action values |
| --- | --- |
| `within` | `none`, `warn` |
| `near_limit` | `warn`, `narrow_scope` |
| `exceeded` | `require_user_approval`, `block`, `narrow_scope` |
| `blocked` | `block` |

The matrix means `blocked` always requires `block`, and `within` cannot require user approval.

## Repository

- `insert_budget_ledger(BudgetLedger) -> Result<BudgetLedger, GraphStoreError>`
- `get_budget_ledger(&OpaqueId<BudgetLedger>) -> Result<Option<BudgetLedger>, GraphStoreError>`
- `list_budget_ledgers_by_workspace(&OpaqueId<GraphWorkspace>) -> Result<Vec<BudgetLedger>, GraphStoreError>`

Writes run inside explicit transactions and roll back on FK, enum, duplicate-ID, or invariant failure.

## Validation

Before insert, the repository validates every opaque ID, optional provider-state ID when present, `RecordMeta`, non-negative counters, and the policy-action matrix.

`input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`, `latency_ms`, and `provider_cost_estimate` preserve zero as a valid measured value and reject negative values.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, recovery execution, or any durable table other than `budget_ledgers`.
