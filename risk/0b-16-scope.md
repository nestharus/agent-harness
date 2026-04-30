# WU-0B-16 — Scope Risk Review

**Gate:** Phase 8 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0B-16 boundary (the 14-field `BudgetLedger` row, the `budget_ledgers` durable table, the `BudgetScopeType` / `BudgetState` / `PolicyAction` enums, the `BudgetScopeRef` typed marker, the `ProviderStateRef` soft-FK marker reused from WU-0B-15, the policy-action matrix, and the repository insert/get/list-by-workspace surface), and does every one of the 10 acceptance criteria get exercised by a named test?

## Findings

### WU-0B-16-SCOPE-F01 — All 10 acceptance criteria are exercised

| AC | Verified by |
| --- | --- |
| Schema, FKs, unique, indexes, RecordMeta | `budget_ledgers_schema_contains_declared_columns_constraints_and_indexes` (`src-tauri/tests/wu_0b_16_budgetledger_contract.rs:209-300`) — asserts the 23 expected column names, `UNIQUE(budget_ledger_id_value, budget_ledger_id_namespace)`, `json_valid(actor)`, `input_tokens >= 0`, the matrix CHECK fragment `budget_state = 'blocked' AND policy_action = 'block'`, the `schema_versions` FK; via `PRAGMA foreign_key_list` confirms FKs to `schema_versions` and `graph_workspaces` and explicitly asserts no `provider`-prefixed FK exists; via `PRAGMA index_list` asserts `idx_budget_ledgers_workspace_id`, `idx_budget_ledgers_scope_type`, `idx_budget_ledgers_budget_state`. |
| insert→get byte-equivalent round-trip | `insert_then_get_round_trips_every_budget_ledger_field_byte_equivalent` (`:303-329`) — `serde_json::to_string(&fixture.row)` vs `serde_json::to_string(&inserted)` and `serde_json::to_string(&fetched)`, exercising every scalar, counter, optional opaque ID, optional `cache_prefix_hash`, every enum, `RecordMeta`, and timestamp via `round-trip.json`. |
| Explicit transaction + rollback on FK / enum / routing / state-transition failure | `invalid_insert_paths_roll_back_without_partial_budget_rows` (`:332-370`) — exercises three failure shapes (FK miss `unknown_workspace`, negative `input_tokens`, `blocked + warn` matrix violation); each returns the typed `GraphStoreError` and `budget_ledger_row_count` is unchanged after each. The repo wraps each write in `pool.begin().await?` and explicitly invokes `rollback(transaction)` on every error branch (`src-tauri/src/graphstore/budgetledger.rs:114-122, 132-135, 184-193`). |
| No operator-visible feature, UI pane, agent launch, optimizer, provider probe, recovery | `budgetledger_wu_has_no_operator_visible_behavior_or_extra_tables` (`:373-411`) — asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` (length 1); runtime tables `["audit_events", "budget_ledgers", "evidence_artifacts", "graph_configurations", "graph_nodes", "graph_workspaces", "policy_sets", "schema_versions"]`; greps the impl source for the nine forbidden tokens (`Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler`). |
| Contract tests inside boundary | `budgetledger_contract_tests_stay_inside_declared_wu_boundary` (`:414-440`) — asserts the exact list of 11 `use agent_harness_lib::` lines; all are this WU's contract module (`budgetledger`), prelude/fixture infra, or the FK-target predecessors (`policyset`, `graphconfiguration`, `graphworkspace`) — exactly the dependencies the ticket declares. |
| Every `scope_type` variant (7) round-trips + unknown rejected | `every_scope_type_variant_round_trips_and_unknown_is_rejected` (`:443-496`) — `scope-type-variants.json` fixture asserts `len() == 7`; per variant exercises serde, SQLx encode/decode through a temp `budget_scope_type_codec` table, and a real `repo.insert_budget_ledger` with each variant. Unknown `"workspace_span"` rejected by both serde (`serde_json::from_value::<BudgetScopeType>` returns `Err`) and SQLx decode (`row.try_get::<BudgetScopeType, _>` returns `Err`). |
| Every `budget_state` variant (4) round-trips + unknown rejected | `every_budget_state_variant_round_trips_and_unknown_is_rejected` (`:499-556`) — `budget-state-variants.json` fixture asserts `len() == 4`; same codec-table + repo-insert + serde + decode pattern; the per-variant repo insert pairs each `budget_state` with a matrix-legal `policy_action` so the insert reaches the codec layer rather than tripping the matrix CHECK. Unknown `"critical"` rejected by both serde and SQLx decode paths. |
| Every `policy_action` variant (5) round-trips + unknown rejected | `every_policy_action_variant_round_trips_and_unknown_is_rejected` (`:559-615`) — `policy-action-variants.json` fixture asserts `len() == 5`; same codec-table + repo-insert + serde + decode pattern; per-variant insert pairs each `policy_action` with a matrix-legal `budget_state`. Unknown `"auto_escalate"` rejected by both serde and SQLx decode paths. |
| Counters reject negatives + accept zero | `counters_preserve_zero_and_reject_negative_values` (`:618-647`) — first inserts the all-zero `zero_counters` fixture and asserts row count grew by 1; then iterates the six `negative_values` fixtures (`input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`, `latency_ms`, `provider_cost_estimate`), each set to `-1`, expects `InvariantViolation`, and confirms row count unchanged after each rejection. |
| Policy-action matrix (blocked⇒block; within⇏require_user_approval) | `policy_action_matrix_accepts_only_documented_pairs` (`:650-680`) — exercises all 8 legal pairs from `policy-action-invariants.json` (within/none, within/warn, near_limit/warn, near_limit/narrow_scope, exceeded/{require_user_approval, block, narrow_scope}, blocked/block) — every one inserts; then exercises 5 invalid pairs (blocked/{none, warn, narrow_scope, require_user_approval}, within/require_user_approval) — every one rejects `InvariantViolation` with row count unchanged. The four `blocked + non-block` cases enforce the first named invariant; `within + require_user_approval` enforces the second. |

### WU-0B-16-SCOPE-F02 — `BudgetLedger` row carries every declared field

`src-tauri/src/graphstore/budgetledger.rs:50-68` declares the 14-field struct exactly mirroring the contract row at `product-strategy/contracts/wu-0b-16-budgetledger.md:8-25`: `budget_ledger_id`, `workspace_id`, `scope_type`, `scope_id`, `input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`, `latency_ms`, `provider_cost_estimate`, optional `provider_state_id`, optional `cache_prefix_hash`, `budget_state`, `policy_action`, plus `meta: RecordMeta`. `#[serde(deny_unknown_fields)]` (`:51`) hard-rejects extras at the wire boundary.

### WU-0B-16-SCOPE-F03 — Schema columns, constraints, and indexes match the contract

`src-tauri/migrations/0b/16_budgetledger.sql` declares: 23 columns (the 14 row fields decomposed plus `row_id`, `created_at`, `updated_at`, `actor`, `record_policy_version`, `schema_version`), `UNIQUE(budget_ledger_id_value, budget_ledger_id_namespace)`, paired-null CHECK for optional `provider_state_id` (`:52-55`), per-counter `CHECK (col >= 0)` (`:20-25`), three enum CHECKs over the 7+4+5 documented variants (`:7-17, 29-36, 37-45`), the four-arm policy-action matrix CHECK (`:56-61`), FKs to `graph_workspaces` (`:62-63`) and `schema_versions` (`:64`), and three named indexes (`:67-74`). `provider_state_id` is intentionally schema-FK-less (the contract notes the WU-0B-17 target table does not exist yet — `product-strategy/contracts/wu-0b-16-budgetledger.md:27`); validation as `OpaqueId` happens in `validate_budget_ledger` (`:336-341`). All assertions in `budget_ledgers_schema_contains_declared_columns_constraints_and_indexes` pass.

### WU-0B-16-SCOPE-F04 — Repository surface matches the contract

The contract specifies three methods (`product-strategy/contracts/wu-0b-16-budgetledger.md:69-71`); `BudgetLedgerRepo` implements exactly those:
- `insert_budget_ledger` (`src-tauri/src/graphstore/budgetledger.rs:106-194`)
- `get_budget_ledger` (`:196-209`)
- `list_budget_ledgers_by_workspace` (`:211-239`)

Plus the `GraphStoreRepo<BudgetLedger>` trait impl (`:242-268`) wires the same three methods through the WU-0B-03 harness trait.

### WU-0B-16-SCOPE-F05 — Three enum surfaces are exactly the contract variants

- `BudgetScopeType` (`:19-29`): `Workspace | Initiative | OrchestratorTurn | WorkerRun | OptimizerPass | ReviewerPass | Render` — seven variants matching `wu-0b-16-budgetledger.md:31-37`. `as_str` (`:417-427`) and `parse` (`:429-440`) cover the same seven symmetrically.
- `BudgetState` (`:31-38`): `Within | NearLimit | Exceeded | Blocked` — four variants matching `:41-44`. `as_str`/`parse` symmetric (`:444-461`).
- `PolicyAction` (`:40-48`): `None | Warn | NarrowScope | RequireUserApproval | Block` — five variants matching `:48-52`. `as_str`/`parse` symmetric (`:465-484`).

`#[serde(rename_all = "snake_case")]` on each enum produces the contract's wire form. The variant fixtures (`scope-type-variants.json`, `budget-state-variants.json`, `policy-action-variants.json`) list exactly the contract sets, and the unknown-variants fixture supplies one rejected token per enum.

### WU-0B-16-SCOPE-F06 — Policy-action matrix is enforced in two layers

The matrix from `wu-0b-16-budgetledger.md:58-63` is implemented identically by:
- `policy_action_allowed` (`src-tauri/src/graphstore/budgetledger.rs:361-376`): `Within → {None, Warn}`, `NearLimit → {Warn, NarrowScope}`, `Exceeded → {RequireUserApproval, Block, NarrowScope}`, `Blocked → Block`.
- SQL `CHECK` (`migrations/0b/16_budgetledger.sql:56-61`) with the same four-arm disjunction.

The repo invokes `policy_action_allowed` inside `validate_budget_ledger` (`:354-356`) before the insert; failure returns `InvariantViolation` and rolls back. The contract test exercises all 8 legal pairs (every cell in the matrix) and the 5 invariant-named-pairs negative cases.

### WU-0B-16-SCOPE-F07 — `BudgetScopeRef` is a typed marker; `ProviderStateRef` is reused soft-FK marker

`BudgetScopeRef` (`src-tauri/src/graphstore/budgetledger.rs:15-17`) is a unit-marker type, used solely as `T` on `OpaqueId<BudgetScopeRef>`; no `BudgetScopeRefRepo`, no `budget_scopes` table — scope IDs validate as opaque-ID strings via `OpaqueId::<BudgetScopeRef>::new` (`:332-335`). `ProviderStateRef` is imported from WU-0B-15 (`auditevent.rs:16-18`) at `:8` and reused as the same soft-FK marker — no duplicate definition, no provider-state table, no provider-state FK. The contract document explicitly authorizes the `provider_state_id` soft reference until WU-0B-17 (`product-strategy/contracts/wu-0b-16-budgetledger.md:27`).

### WU-0B-16-SCOPE-F08 — Anti-scope honored

The proposal (`proposals/0b-16-wu-0b-16.md`) and contract anti-scope (`wu-0b-16-budgetledger.md:81-83`) bar Tauri commands, UI panes, agent invocation, provider probes, optimizer execution, recovery execution, and any durable table other than `budget_ledgers`. Verified:
- No new Tauri commands, no UI panes, no event producers, no agent invocation, no provider probes, no optimizer execution, no recovery execution.
- `phase_0a_scaffold_commands()` still returns the single `["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:17, 31-33`) — verified live by the contract test (`:379`).
- The only new durable table is `budget_ledgers`. The runtime-table set in the contract test is exactly `[audit_events, budget_ledgers, evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, schema_versions]`.
- No `provider_states` table, no `BudgetScopeRefRepo`, no `provider_state_id` index — the WU-0B-17 surface is genuinely deferred.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test` (full suite) | ✓ all suites pass; `wu_0b_16_budgetledger_contract` 10/10 |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** All 10 acceptance criteria are exercised by named tests with real serde/SQLx round-trips, real FK exercise paths through `PRAGMA foreign_keys = ON`, real defense-in-depth across `validate_budget_ledger` and SQL `CHECK`. The 14-field row, the 7+4+5-variant enum surface, the `BudgetScopeRef` typed marker, the `ProviderStateRef` soft-FK marker reused from WU-0B-15, the three-method repository, and the schema with FKs/uniques/indexes/CHECKs all match the contract. Anti-scope is honored end-to-end: the only new durable table is `budget_ledgers`, no new Tauri commands are registered, `phase_0a_scaffold_commands()` length remains 1, and the WU-0B-17 provider-state FK target is genuinely deferred.
