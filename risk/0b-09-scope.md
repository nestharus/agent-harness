# WU-0B-09 — Scope Risk Review

**Gate:** Phase 8 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0B-09 boundary (the `EvidenceArtifact` row, `evidence_artifacts` durable table, four enums — `SourceType`, `ToolProtocol`, `CaptureState`, plus reused `PrivilegeOrigin` — repository insert/get/list, and supporting fixtures), and does every one of the 11 acceptance criteria get exercised?

## Findings

### WU-0B-09-SCOPE-F01 — All 11 acceptance criteria are exercised

| AC | Verified by |
| --- | --- |
| Schema, FK, unique, JSON, indexes, RecordMeta | `evidence_artifacts_schema_contains_declared_columns_constraints_and_indexes` (`src-tauri/tests/wu_0b_09_evidenceartifact_contract.rs:212-297`) — asserts 19 column names, `UNIQUE(evidence_id_value, evidence_id_namespace)`, `json_valid(actor)`, FK to `schema_versions(version)`, FK to `graph_workspaces` on both `workspace_id_value` + `workspace_id_namespace`, and the three indexes (`idx_evidence_artifacts_workspace_id`, `idx_evidence_artifacts_content_hash`, `idx_evidence_artifacts_capture_state`). |
| insert→get byte-equivalent round-trip | `insert_then_get_round_trips_every_evidence_artifact_field_byte_equivalent` (`:300-327`) — uses `serde_json::to_string(&inserted)` and `serde_json::to_string(&fetched)` against the canonical fixture's serialized form for byte-equivalent comparison. |
| Explicit transaction + rollback on FK / enum / routing / state-transition failure | `invalid_insert_paths_roll_back_without_partial_evidence_rows` (`:330-356`) — exercises FK miss (`UnknownRef`) and routing failure (empty `source_uri` → `InvariantViolation`); `evidence_artifact_row_count` confirms zero growth after each. The repo wraps the entire write in `pool.begin().await?` and invokes `transaction.rollback()` on every error branch (`src-tauri/src/graphstore/evidenceartifact.rs:113, 115-148, 412-414`). |
| No operator-visible feature, UI pane, agent launch, optimizer, provider probe, recovery | `evidenceartifact_wu_has_no_operator_visible_behavior_or_extra_tables` (`:359-395`) — asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` (length 1) and runtime tables `["evidence_artifacts", "graph_configurations", "graph_nodes", "graph_workspaces", "policy_sets", "schema_versions"]`; greps the impl source for nine forbidden tokens (`Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler`). |
| Contract tests inside boundary | `evidenceartifact_contract_tests_stay_inside_declared_wu_boundary` (`:398-424`) — asserts the exact list of 11 `use agent_harness_lib::` lines, all of which are this WU's contract module (`evidenceartifact`), prelude/fixture infra, or its FK predecessors (`policyset`, `graphconfiguration`, `graphworkspace`). |
| Every `source_type` variant (11) round-trips + unknown rejected | `every_source_type_variant_round_trips_and_unknown_is_rejected` (`:427-479`) — fixture asserts `len() == 11`; per variant exercises serde, SQLx encode/decode through a temp codec table, and a real repo insert; unknown `"screen_recording"` rejected by both serde and SQLx decode. |
| Every `tool_protocol` variant (6) round-trips + unknown rejected | `every_tool_protocol_variant_round_trips_and_unknown_is_rejected` (`:482-534`) — fixture asserts `len() == 6`; same shape; unknown `"browser"` rejected. |
| Every `capture_state` variant (5) round-trips + unknown rejected | `every_capture_state_variant_round_trips_and_unknown_is_rejected` (`:537-592`) — fixture asserts `len() == 5`; same shape; unknown `"pending"` rejected. |
| Every `privilege_origin` variant (7) round-trips + unknown rejected | `every_privilege_origin_variant_round_trips_and_unknown_is_rejected` (`:595-647`) — fixture asserts `len() == 7`; same shape; unknown `"administrator"` rejected. The variant set is reused from WU-0B-07 (see SCOPE-F03). |
| `content_hash` required for captured/partial/redacted/quarantined; failed may omit only with failure payload ref | `content_hash_is_required_except_failed_with_failure_payload_ref` (`:650-682`) — three branches: `captured_missing_hash` rejected `InvariantViolation`; `failed_missing_hash_missing_failure_ref` (empty `blob_ref`) rejected `InvariantViolation`; `failed_missing_hash_with_failure_ref` (non-empty `blob_ref`) inserts. Repo enforces the rule pre-DB at `src-tauri/src/graphstore/evidenceartifact.rs:352-362` and via the SQL `CHECK (capture_state = 'failed' OR content_hash IS NOT NULL)` (`src-tauri/migrations/0b/09_evidenceartifact.sql:64-67`). |
| Blob refs stay under workspace storage root; escapes rejected | `blob_refs_must_remain_under_workspace_storage_root` (`:685-743`) — three escape variants: `../outside.txt` parent path, absolute `/tmp/agent-harness-outside-evidence.txt`, and on Unix a real `symlink` whose target is under the storage-root parent. All three rejected `InvariantViolation`; row count unchanged. |

### WU-0B-09-SCOPE-F02 — `EvidenceArtifact` row carries every declared field

`src-tauri/src/graphstore/evidenceartifact.rs:55-71` declares the 13-field struct exactly mirroring the contract row at `product-strategy/contracts/wu-0b-09-evidenceartifact.md:7-23`: `evidence_id`, `workspace_id`, `source_type`, `source_uri`, optional `source_session_id`, `tool_protocol`, optional `correlation_key`, optional `content_hash`, `blob_ref`, `privilege_origin`, `capture_state`, `captured_at`, and `meta: RecordMeta`. `#[serde(deny_unknown_fields)]` (`:56`) hard-rejects extras at the wire boundary.

### WU-0B-09-SCOPE-F03 — `PrivilegeOrigin` is reused from WU-0B-07, not duplicated

`src-tauri/src/graphstore/evidenceartifact.rs:12` imports `crate::graphstore::graphnode::PrivilegeOrigin` directly. The contract module re-exports it (`src-tauri/src/contracts/evidenceartifact.rs:4`: `pub use crate::graphstore::graphnode::PrivilegeOrigin;`). No `enum PrivilegeOrigin` declaration appears in the new files. The contract document explicitly authorizes this reuse (`product-strategy/contracts/wu-0b-09-evidenceartifact.md:32`).

### WU-0B-09-SCOPE-F04 — Schema columns, constraints, and indexes match the contract

`src-tauri/migrations/0b/09_evidenceartifact.sql` declares: 19 columns (the 13 row fields decomposed plus `row_id`, `created_at`, `updated_at`, `actor`, `record_policy_version`, `schema_version`), the four enum-bound `CHECK` clauses with the exact variant lists from the contract enums, the `actor` JSON validity check (`json_valid(actor)`), `UNIQUE(evidence_id_value, evidence_id_namespace)`, the content-state `CHECK (capture_state = 'failed' OR content_hash IS NOT NULL)`, the `graph_workspaces` composite-key FK, the `schema_versions` FK, and three named indexes. All assertions in `evidence_artifacts_schema_contains_declared_columns_constraints_and_indexes` pass.

### WU-0B-09-SCOPE-F05 — Repository surface matches the contract

The contract specifies three methods (`product-strategy/contracts/wu-0b-09-evidenceartifact.md:38-40`); `EvidenceArtifactRepo` implements exactly those:
- `insert_evidence_artifact` (`src-tauri/src/graphstore/evidenceartifact.rs:105-191`)
- `get_evidence_artifact` (`:193-206`)
- `list_evidence_artifacts_by_workspace` (`:208-234`)

Plus the `GraphStoreRepo<EvidenceArtifact>` trait impl (`:237-267`) wires the same three methods through the harness trait.

### WU-0B-09-SCOPE-F06 — Anti-scope honored

The ticket bars later-phase value-slice behavior (`plans/tickets/phase-0b/WU-0B-09.md` Scope: "Keep the work inert beyond the declared GraphStore/schema/repository contract"). Verified:
- No new Tauri commands, no UI panes, no event producers, no agent invocation, no provider probes, no optimizer execution, no recovery execution.
- `phase_0a_scaffold_commands()` still returns the single `["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:17, 31-33`) — verified live by the contract test (`:365`).
- The only new durable table is `evidence_artifacts`. The runtime-table set in the contract test is exactly `[evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, schema_versions]`.
- No new commands, no new dependencies, no new top-level modules beyond the two `pub mod` declarations (`src-tauri/src/contracts/mod.rs`, `src-tauri/src/graphstore/mod.rs`).

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ FULL TURBO cache hit |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test` (full suite) | ✓ all suites pass; `wu_0b_09_evidenceartifact_contract` 11/11 |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** All 11 acceptance criteria are exercised by named tests with real serde/SQLx round-trips, real FK/CHECK exercise paths, real path canonicalization, and a real symlink test on Unix. The 13-field row, the four-enum surface (with `PrivilegeOrigin` correctly *reused* from WU-0B-07 by import + re-export, not redefined), the three-method repository, and the schema with FKs/uniques/indexes all match the contract. Anti-scope is honored end-to-end: the only new durable table is `evidence_artifacts`, no new Tauri commands are registered, and `phase_0a_scaffold_commands()` length remains 1.
