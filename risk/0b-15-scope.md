# WU-0B-15 — Scope Risk Review

**Gate:** Phase 8 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0B-15 boundary (the 13-field `AuditEvent` row, the `audit_events` durable table, the `AuditDecision` enum, the `ProviderStateRef` soft-FK marker, and the repository insert/get/list surface), and does every one of the 9 acceptance criteria get exercised?

## Findings

### WU-0B-15-SCOPE-F01 — All 9 acceptance criteria are exercised

| AC | Verified by |
| --- | --- |
| Schema, FKs, unique, JSON checks, indexes, RecordMeta | `audit_events_schema_contains_declared_columns_constraints_and_indexes` (`src-tauri/tests/wu_0b_15_auditevent_contract.rs:179-267`) — asserts the 22 expected column names, `UNIQUE(audit_event_id_value, audit_event_id_namespace)`, the three `json_valid(...)` checks (`input_refs`, `output_refs`, `record_actor`), the `schema_versions` FK, and via `PRAGMA foreign_key_list` the FKs to `graph_workspaces`, `policy_sets`, `graph_configurations`. The four named indexes (`idx_audit_events_workspace_id`, `idx_audit_events_policy_set_id`, `idx_audit_events_decision`, `idx_audit_events_created_at`) are asserted via `PRAGMA index_list`. |
| insert→get byte-equivalent round-trip | `insert_then_get_round_trips_every_audit_event_field_byte_equivalent` (`:269-296`) — uses `serde_json::to_string(&inserted)` and `serde_json::to_string(&fetched)` against the canonical `round-trip.json` fixture's serialized form for byte-equivalent comparison. The fixture covers every scalar, optional opaque ID, JSON array, enum, timestamp, and `RecordMeta` field. |
| Explicit transaction + rollback on FK / enum / routing / state-transition failure | `invalid_insert_paths_roll_back_without_partial_audit_rows` (`:298-322`) and `unknown_workspace_policy_or_configuration_rejects_without_partial_rows` (`:533-554`) — exercise FK miss (`UnknownRef`) and invariant rejection (`InvariantViolation`); `audit_event_row_count` confirms zero growth after each. The repo wraps each write in `pool.begin().await?` and invokes `transaction.rollback()` on every error branch (`src-tauri/src/graphstore/auditevent.rs:91-99, 109-112, 165-174`). |
| No operator-visible feature, UI pane, agent launch, optimizer, provider probe, recovery | `auditevent_wu_has_no_operator_visible_behavior_or_extra_tables` (`:324-362`) — asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` (length 1) and runtime tables `["audit_events", "evidence_artifacts", "graph_configurations", "graph_nodes", "graph_workspaces", "policy_sets", "schema_versions"]`; greps the impl source for the nine forbidden tokens (`Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler`). |
| Contract tests inside boundary | `auditevent_contract_tests_stay_inside_declared_wu_boundary` (`:364-391`) — asserts the exact list of 11 `use agent_harness_lib::` lines; all are this WU's contract module (`auditevent`), prelude/fixture infra, or the FK-target predecessors (`policyset`, `graphconfiguration`, `graphworkspace`) — exactly the dependencies the ticket declares. |
| Every `decision` variant (5) round-trips + unknown rejected | `every_decision_variant_round_trips_and_unknown_is_rejected` (`:393-445`) — `decision-variants.json` fixture asserts `len() == 5`; per variant exercises serde, SQLx encode/decode through a temp codec table, and a real `repo.insert_audit_event` with each variant. Unknown `"escalated"` rejected by both serde (`serde_json::from_value::<AuditDecision>` returns `Err`) and SQLx decode (`row.try_get::<AuditDecision, _>` returns `Err`). |
| Append-only invariant | `audit_events_repository_is_append_only_by_source_assertion` (`:447-467`) — greps the implementation source (lower-cased) for six forbidden patterns: `update audit_events`, `delete from audit_events`, `fn update_`, `fn delete_`, `pub async fn update_`, `pub async fn delete_`. None are present. The contract document at `product-strategy/contracts/wu-0b-15-auditevent.md:54-55` declares the invariant; the test enforces it by source grep. |
| Output-with-input + required policy/actor/decision/reason | `output_refs_require_input_refs_policy_actor_decision_and_reason` (`:469-531`) — exercises four typed fixtures (`output_refs_without_input_refs`, `output_refs_with_empty_policy_id`, `output_refs_with_empty_actor`, `output_refs_with_empty_reason_code`), each rejected `InvariantViolation` with row-count unchanged, plus a raw `INSERT ... decision NULL` that the SQL `NOT NULL` rejects. Repo enforces the rule pre-DB at `src-tauri/src/graphstore/auditevent.rs:339-358`; SQL `CHECK (json_array_length(output_refs) = 0 OR json_array_length(input_refs) > 0)` (`migrations/0b/15_auditevent.sql:46-49`) is the second layer. |
| FK rejection on unknown workspace/policy/configuration without partial writes | `unknown_workspace_policy_or_configuration_rejects_without_partial_rows` (`:533-554`) — three fixtures (`unknown_workspace`, `unknown_policy_set`, `unknown_configuration`), each rejected `UnknownRef` with row-count unchanged. The `PRAGMA foreign_keys = ON` runs before every transaction (`src-tauri/src/graphstore/auditevent.rs:87-90`) and `map_write_error` maps `FOREIGN KEY constraint failed` → `UnknownRef` (`:393-395`). |

### WU-0B-15-SCOPE-F02 — `AuditEvent` row carries every declared field

`src-tauri/src/graphstore/auditevent.rs:30-46` declares the 13-field struct exactly mirroring the contract row at `product-strategy/contracts/wu-0b-15-auditevent.md:7-23`: `audit_event_id`, `workspace_id`, `event_type`, `actor`, `policy_set_id`, optional `configuration_id`, optional `provider_state_id`, `input_refs`, `output_refs`, `decision`, `reason_code`, `created_at`, and `meta: RecordMeta`. `#[serde(deny_unknown_fields)]` (`:31`) hard-rejects extras at the wire boundary.

### WU-0B-15-SCOPE-F03 — Schema columns, constraints, and indexes match the contract

`src-tauri/migrations/0b/15_auditevent.sql` declares: 22 columns (the 13 row fields decomposed plus `row_id`, `record_created_at`, `record_updated_at`, `record_actor`, `record_policy_version`, `schema_version`), `UNIQUE(audit_event_id_value, audit_event_id_namespace)`, paired-null CHECKs for optional `configuration_id` and `provider_state_id`, JSON validity checks for `input_refs`, `output_refs`, and `record_actor`, decision CHECK over the five variants, an `output_refs > 0 implies input_refs > 0` CHECK, FKs to `graph_workspaces`, `policy_sets`, `graph_configurations` (composite-key) and `schema_versions`, and four named indexes. `provider_state_id` is intentionally schema-FK-less (the contract notes the WU-0B-17 target table does not exist yet — `product-strategy/contracts/wu-0b-15-auditevent.md:27`); validation as `OpaqueId` happens in `validate_audit_event` (`:330-335`). All assertions in `audit_events_schema_contains_declared_columns_constraints_and_indexes` pass.

### WU-0B-15-SCOPE-F04 — Repository surface matches the contract

The contract specifies three methods (`product-strategy/contracts/wu-0b-15-auditevent.md:41-43`); `AuditEventRepo` implements exactly those:
- `insert_audit_event` (`src-tauri/src/graphstore/auditevent.rs:83-175`)
- `get_audit_event` (`:177-190`)
- `list_audit_events_by_workspace` (`:192-221`)

Plus the `GraphStoreRepo<AuditEvent>` trait impl (`:224-250`) wires the same three methods through the WU-0B-03 harness trait. No `update_*` / `delete_*` methods exist — append-only is structural, not just gated by source-grep.

### WU-0B-15-SCOPE-F05 — `AuditDecision` enum surface is exactly the five contract variants

`src-tauri/src/graphstore/auditevent.rs:20-28` declares `Accepted | Rejected | Deferred | Quarantined | UserRequired` with `#[serde(rename_all = "snake_case")]`; `as_str` (`:408-417`) and `parse` (`:419-428`) cover the same five variants symmetrically. The `decision-variants.json` fixture lists all five (`product-strategy/contracts/fixtures/wu-0b-15/decision-variants.json`). Unknown `"escalated"` (`unknown-decision.json`) is rejected by both serde and SQLx decode paths.

### WU-0B-15-SCOPE-F06 — `ProviderStateRef` is a soft-FK marker, not a stub for future work

`src-tauri/src/graphstore/auditevent.rs:16-18` declares `pub struct ProviderStateRef;` as the `OpaqueId<T>` type marker. There is no `ProviderStateRefRepo`, no `provider_states` table, no `provider_state` migration. The contract document explicitly authorizes this soft reference and reserves the FK target for WU-0B-17 (`product-strategy/contracts/wu-0b-15-auditevent.md:27`). Validation happens at `validate_audit_event:330-335` via the `OpaqueId::<ProviderStateRef>::new` constructor — same trim/forbidden-char checks as every other opaque ID. The contract test exercises a soft-ref fixture value `"provider-state-soft-ref-round-trip"` in the canonical `round-trip.json`.

### WU-0B-15-SCOPE-F07 — Anti-scope honored

The proposal anti-scope (`proposals/0b-15-wu-0b-15.md:7`) bars provider state table/FK target, audit-trail UI, commands, agent launch, provider probe, optimizer execution, and recovery execution. Verified:
- No new Tauri commands, no UI panes, no event producers, no agent invocation, no provider probes, no optimizer execution, no recovery execution.
- `phase_0a_scaffold_commands()` still returns the single `["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:17, 31-33`) — verified live by the contract test (`:331`).
- The only new durable table is `audit_events`. The runtime-table set in the contract test is exactly `[audit_events, evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, schema_versions]`.
- No new commands, no new dependencies, no new top-level modules beyond the two `pub mod` declarations (`src-tauri/src/contracts/mod.rs`, `src-tauri/src/graphstore/mod.rs`).
- No `provider_states` table, no `ProviderStateRefRepo`, no `provider_state_id` index — the WU-0B-17 surface is genuinely deferred.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test` (full suite) | ✓ all suites pass; `wu_0b_15_auditevent_contract` 9/9 |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** All 9 acceptance criteria are exercised by named tests with real serde/SQLx round-trips, real FK exercise paths through `PRAGMA foreign_keys = ON`, and a source-grep enforcement of the append-only invariant. The 13-field row, the five-variant decision enum, the soft `ProviderStateRef` marker, the three-method repository, and the schema with FKs/uniques/indexes/CHECKs all match the contract. Anti-scope is honored end-to-end: the only new durable table is `audit_events`, no new Tauri commands are registered, `phase_0a_scaffold_commands()` length remains 1, and the WU-0B-17 provider-state FK target is genuinely deferred.
