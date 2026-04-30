# WU-0B-17 — Scope Risk Review

**Gate:** Phase 8 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0B-17 boundary (the 17-field `ProviderState` row, the `provider_states` durable table, the nine documented enums, the four-method repository surface including `mark_provider_state_stale`, the `secret_material_stored = false` invariant, and the credential-shape rejection guard), and does every one of the 16 acceptance criteria get exercised by a named test?

## Findings

### WU-0B-17-SCOPE-F01 — All 16 acceptance criteria are exercised

| AC | Verified by |
| --- | --- |
| Schema, FKs, unique, JSON CHECKs, indexes, RecordMeta | `provider_states_schema_contains_declared_columns_constraints_indexes_and_fks` (`src-tauri/tests/wu_0b_17_providerstate_contract.rs:190-285`) — asserts all 23 expected column names, `UNIQUE(provider_state_id_value, provider_state_id_namespace)`, `json_valid(sandbox_constraints)`, `json_valid(store_locations_checked)`, the SQL invariant `secret_material_stored = 0`; `PRAGMA foreign_key_list(provider_states)` confirms FKs to `schema_versions` and `graph_workspaces`; `PRAGMA foreign_key_list(audit_events)` and `PRAGMA foreign_key_list(budget_ledgers)` confirm both now FK their `provider_state_id` to `provider_states`; `PRAGMA index_list` asserts the five named indexes (`idx_provider_states_workspace_id`, `idx_provider_states_provider`, `idx_provider_states_cli`, `idx_provider_states_freshness`, `idx_provider_states_last_probe_at`). |
| insert→get byte-equivalent round-trip (every scalar, enum, JSON, timestamp, opaque ID) | `insert_then_get_round_trips_every_provider_state_field_byte_equivalent` (`:288-314`) — `serde_json::to_string(&fixture.row)` vs `serde_json::to_string(&inserted)` and `serde_json::to_string(&fetched)` against `round-trip.json`, exercising every scalar, all nine enums (using non-canonical variants like `expired`/`near_limit`/`rate_limited`/`degraded`/`wrong_version`/`manual`/`medium`), the JSON object/array fields, the bool, both opaque IDs, the timestamp, and `RecordMeta`. |
| Explicit transaction + rollback on FK / enum / routing / state-transition failure | `invalid_insert_paths_roll_back_without_partial_provider_rows` (`:317-342`) — exercises FK miss (`unknown-workspace` → `UnknownRef`) and empty-account `InvariantViolation`; the repo wraps every write in `pool.begin().await?` and explicitly invokes `rollback(transaction)` on every error branch (`src-tauri/src/graphstore/providerstate.rs:165-237, 282-345`). Row count is unchanged after each rejection. The mark-stale negative test (`:471-518`) extends rollback coverage to UPDATE failures (unknown-id, malformed reasons). |
| No operator-visible feature, UI pane, agent launch, optimizer, provider probe, recovery | `providerstate_wu_has_no_operator_visible_behavior_or_extra_tables` (`:345-384`) — asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` (length 1); runtime tables exactly `[audit_events, budget_ledgers, evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, provider_states, schema_versions]` (the prior eight plus `provider_states` in alphabetical order); greps the impl source for the nine forbidden tokens (`Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler`). |
| Contract tests written from this WU only | The 16 tests in `wu_0b_17_providerstate_contract.rs` import only this WU's contracts (`providerstate`), the predecessor FK-target contracts (`policyset`, `graphconfiguration`, `graphworkspace`), the prelude/fixture infra, and `phase_0a_scaffold_commands` — exactly the dependencies the ticket declares. No imports of agents, optimizers, providers, recovery, evidenceartifact, graphnode, auditevent, budgetledger. |
| Every `provider` variant (6) round-trips + unknown rejected | `every_provider_variant_round_trips_and_unknown_is_rejected` macro-expansion (`:649-658`) — `provider-variants.json` fixture asserts `len() == 6`; per variant exercises serde, SQLx encode/decode through a temp `provider_kind_codec` table, and a real `repo.insert_provider_state` with each variant. Unknown `"mistral"` rejected by both serde (`serde_json::from_value::<ProviderKind>` returns `Err`) and SQLx decode (`row.try_get::<ProviderKind, _>` returns `Err`). |
| Every `cli` variant (4) round-trips + unknown rejected | `every_cli_variant_round_trips_and_unknown_is_rejected` (`:659-668`) — `cli-variants.json` `len() == 4`; same pattern; unknown `"terminal"` rejected at both layers. |
| Every `auth_state` variant (5) round-trips + unknown rejected | `every_auth_state_variant_round_trips_and_unknown_is_rejected` (`:669-678`) — `auth-state-variants.json` `len() == 5`; unknown `"revoked"` rejected. |
| Every `billing_state` variant (5) round-trips + unknown rejected | `every_billing_state_variant_round_trips_and_unknown_is_rejected` (`:679-688`) — `billing-state-variants.json` `len() == 5`; unknown `"past_due"` rejected. |
| Every `quota_state` variant (4) round-trips + unknown rejected | `every_quota_state_variant_round_trips_and_unknown_is_rejected` (`:689-698`) — `quota-state-variants.json` `len() == 4`; unknown `"warming"` rejected. |
| Every `network_state` variant (5) round-trips + unknown rejected | `every_network_state_variant_round_trips_and_unknown_is_rejected` (`:699-708`) — `network-state-variants.json` `len() == 5`; unknown `"offline"` rejected. |
| Every `runtime_state` variant (6) round-trips + unknown rejected | `every_runtime_state_variant_round_trips_and_unknown_is_rejected` (`:709-718`) — `runtime-state-variants.json` `len() == 6`; unknown `"booting"` rejected. |
| Every `freshness` variant (4) round-trips + unknown rejected | `every_freshness_variant_round_trips_and_unknown_is_rejected` (`:719-728`) — `freshness-variants.json` `len() == 4`; unknown `"aged"` rejected. |
| Every `confidence` variant (3) round-trips + unknown rejected | `every_confidence_variant_round_trips_and_unknown_is_rejected` (`:729-738`) — `confidence-variants.json` `len() == 3`; unknown `"certain"` rejected. |
| `mark_provider_state_stale` mutates only documented columns; invalid inputs reject without partial writes | `mark_provider_state_stale_updates_only_documented_columns` (`:387-468`) — diffs the `serde_json::to_value` of before/after with `/freshness`, `/confidence`, `/meta/updated_at`, and `/meta/actor` removed: `assert_eq!(after_json, before_json)`. Asserts `after.freshness = Stale`, `after.confidence = Low`, `after.meta.actor.value = "provider-state-stale:auth_refresh"`, `after.meta.updated_at != before.meta.updated_at`. Raw SQL spot-check confirms `provider`, `cli`, `account_ref`, `auth_state`, `billing_state`, `quota_state`, `network_state`, `runtime_state`, `secret_material_stored`, `last_probe_at`, `created_at`, `record_policy_version` are byte-identical. The negative path `mark_provider_state_stale_rejects_unknown_id_and_malformed_reason_without_writes` (`:471-518`) covers: unknown provider state id → `UnknownRef`; five malformed reasons (empty, single space, `"Auth Refresh"` with capitals/space, `"../escape"`, `"has/slash"`) → `InvariantViolation`; row count and post-rejection `serde_json::to_string(&after)` is byte-identical to before. |
| `secret_material_stored = false`; true values and credential-looking payloads rejected before persistence | `secret_material_true_and_credential_looking_payloads_reject_before_persistence` (`:521-586`) — repo guard rejects `secret_material_true` fixture with `InvariantViolation`; raw direct `INSERT INTO provider_states ... secret_material_stored=1` is rejected by SQL (the SQL `CHECK (secret_material_stored = 0)` constraint at `migrations/0b/17_providerstate.sql:73` fires); four credential-shape payloads (`sk-` prefix in `account_ref`, `Bearer ` in `sandbox_constraints`, AWS access-key-shape `AKIAIOSFODNN7EXAMPLE` in `store_locations_checked`, JWT-shape `eyJ...` in `sandbox_constraints`) all reject `InvariantViolation` before persistence; row count unchanged after each. |

### WU-0B-17-SCOPE-F02 — `ProviderState` row carries every declared field

`src-tauri/src/graphstore/providerstate.rs:103-123` declares the 17-field struct exactly mirroring the contract row at `product-strategy/contracts/wu-0b-17-providerstate.md:7-26`: `provider_state_id`, `workspace_id`, `provider`, `cli`, `account_ref`, `auth_state`, `billing_state`, `quota_state`, `network_state`, `runtime_state`, `sandbox_constraints`, `store_locations_checked`, `secret_material_stored`, `freshness`, `confidence`, `last_probe_at`, plus `meta: RecordMeta`. `#[serde(deny_unknown_fields)]` (`:104`) hard-rejects extras at the wire boundary.

### WU-0B-17-SCOPE-F03 — Schema columns, constraints, and indexes match the contract

`src-tauri/migrations/0b/17_providerstate.sql:1-115` declares: 23 columns (the 17 row fields decomposed plus `row_id`, `created_at`, `updated_at`, `actor`, `record_policy_version`, `schema_version`); `UNIQUE(provider_state_id_value, provider_state_id_namespace)` (`:95`); per-enum CHECK constraints over the 6 + 4 + 5 + 5 + 4 + 5 + 6 + 4 + 3 documented variants (`:7-88`); `json_valid(sandbox_constraints)` and `json_valid(store_locations_checked)` JSON CHECKs (`:71-72`); `CHECK (json_type(store_locations_checked) = 'array')` (`:96`); the SQL secret-material invariant `secret_material_stored INTEGER NOT NULL CHECK (secret_material_stored = 0)` (`:73`); `CHECK (trim(account_ref) <> '')` (`:25`); FKs to `graph_workspaces` (`:97-98`) and `schema_versions` (`:99`); five named indexes (`:102-115`).

### WU-0B-17-SCOPE-F04 — Repository surface matches the contract

The contract specifies four methods (`product-strategy/contracts/wu-0b-17-providerstate.md:47-50`); `ProviderStateRepo` implements exactly those:
- `insert_provider_state` (`src-tauri/src/graphstore/providerstate.rs:161-237`)
- `get_provider_state` (`:239-252`)
- `list_provider_states_by_workspace` (`:254-280`)
- `mark_provider_state_stale` (`:282-345`)

Plus the `GraphStoreRepo<ProviderState>` trait impl (`:348-374`) wires insert/get/list through the WU-0B-03 harness trait.

### WU-0B-17-SCOPE-F05 — Nine enum surfaces are exactly the contract variants

- `ProviderKind` (`:16-25`): `Anthropic | Openai | Google | LocalRuntime | OpenaiCompatible | Other` — six variants matching `wu-0b-17-providerstate.md:33`.
- `ProviderCli` (`:27-34`): `Claude | Codex | Opencode | AgentRunner` — four variants matching `:34`.
- `AuthState` (`:36-44`): `Present | Missing | Expired | Invalid | Unknown` — five variants matching `:35`.
- `BillingState` (`:46-54`): `Healthy | NearLimit | OverLimit | PaymentRequired | Unknown` — five variants matching `:36`.
- `ProviderQuotaState` (`:56-63`): `Available | RateLimited | Exhausted | Unknown` — four variants matching `:37`.
- `NetworkState` (`:65-73`): `Available | BlockedBySandbox | BlockedByHost | Degraded | Unknown` — five variants matching `:38`.
- `ProviderRuntimeState` (`:75-84`): `Installed | Missing | WrongVersion | Unreachable | NotApplicable | Unknown` — six variants matching `:39`.
- `ProviderFreshness` (`:86-93`): `Fresh | Stale | ProbeFailed | Manual` — four variants matching `:40`.
- `ProviderConfidence` (`:95-101`): `High | Medium | Low` — three variants matching `:41`.

`#[serde(rename_all = "snake_case")]` on each enum produces the contract wire form; per-enum `as_str` (`:683-882`) and `parse` are symmetric. The variant fixtures list exactly the contract sets, the unknown-variants fixture supplies one rejected token per enum, and every variant is reachable through `repo.insert_provider_state` in the macro-expanded test (the `record.$field = variant` loop in `enum_round_trip_test!` at `:600-635`).

### WU-0B-17-SCOPE-F06 — `secret_material_stored = false` invariant is enforced in two layers

- **Repo layer** (`validate_provider_state`, `:459-461`): `if record.secret_material_stored { return Err(InvariantViolation); }` — fires *before* the SQL insert; failure rolls back the transaction.
- **SQL layer** (`migrations/0b/17_providerstate.sql:73`): `secret_material_stored INTEGER NOT NULL CHECK (secret_material_stored = 0)` — if the repo guard ever drifts, SQLite rejects the row.

The contract test exercises both layers: `repo.insert_provider_state(secret_material_true_fixture)` rejects via the repo guard; a direct `sqlx::query("INSERT INTO provider_states ... 1, ...")` rejects via the SQL CHECK (`:538-561`).

### WU-0B-17-SCOPE-F07 — Credential-shape rejection guard

`contains_credential_like_string` / `value_contains_credential_like_string` (`src-tauri/src/graphstore/providerstate.rs:487-548`) uses real string-shape detection (no stub):
- `sk-` prefix
- `Bearer ` prefix (and per-token after split)
- AWS access-key shape: 20 chars, starts with `AKIA` or `ASIA`, all uppercase + digit
- JWT shape: three `.`-separated base64-url parts with `eyJ` header prefix and ≥10-char parts
- Generic high-entropy ≥48-char base64-url tokens with mixed alpha/digit/case

Applied in `validate_provider_state` (`:463-472`) to `account_ref`, the JSON tree under `sandbox_constraints`, and every entry in `store_locations_checked`. The contract test exercises all four shapes (`:563-585`); each rejects `InvariantViolation` before persistence.

### WU-0B-17-SCOPE-F08 — `mark_provider_state_stale` is bounded to documented columns

`src-tauri/src/graphstore/providerstate.rs:282-345` opens a transaction, validates the reason via `validate_stale_reason` (`:550-563`: ASCII lowercase + digits + `_-.:`, ≤80 chars, no `..`), looks up the current row, computes `next_contract_timestamp` for `updated_at` (`:599-651`: real second-by-second roll-over honoring leap years), constructs the new actor as `"provider-state-stale:{reason}"`, and issues a SQL `UPDATE provider_states SET freshness = ?, confidence = ?, updated_at = ?, actor = ? WHERE provider_state_id_value = ? AND provider_state_id_namespace = ?`. Only those four columns appear in the SET clause. The contract test diff-asserts byte equality of the JSON-pointer-zeroed before/after (`:418-438`) and raw-SQL spot-checks 13 unrelated columns (`:440-468`). The negative path verifies unknown ID → `UnknownRef`, malformed reasons → `InvariantViolation`, all without partial writes.

### WU-0B-17-SCOPE-F09 — Anti-scope honored

The contract anti-scope (`wu-0b-17-providerstate.md:60-62`) bars Tauri commands, UI panes, agent invocation, provider probes, optimizer execution, recovery execution, and any durable table other than `provider_states`. Verified:
- No new Tauri commands, no UI panes, no event producers, no agent invocation, no provider probes, no optimizer execution, no recovery execution.
- `phase_0a_scaffold_commands()` still returns `["subscribe_workspace_events"]` (length 1) — `git diff main -- src-tauri/src/lib.rs` is empty.
- The only new durable table is `provider_states`. The runtime-table set asserted by the contract test is exactly the prior eight plus `provider_states`.
- The forbidden-token grep (`:367-383`) covers `Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler` against `src-tauri/src/graphstore/providerstate.rs`. None present.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test --test wu_0b_17_providerstate_contract` | ✓ 16/16 |
| `cargo test --test wu_0b_15_auditevent_contract --test wu_0b_16_budgetledger_contract --test graphstore_fixture_contract --test graphstore_migrations_contract` | ✓ all green |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** All 16 acceptance criteria are exercised by named tests with real serde/SQLx round-trips, real FK exercise paths through `PRAGMA foreign_keys = ON`, real defense-in-depth across `validate_provider_state` and SQL `CHECK`. The 17-field row, the 6+4+5+5+4+5+6+4+3-variant enum surface, the four-method repository, and the schema with FKs/uniques/JSON CHECKs/indexes all match the contract. `mark_provider_state_stale` is bounded by both the JSON-pointer-zeroed before/after equality assertion and a raw-SQL spot-check across 13 unrelated columns. `secret_material_stored = false` is enforced at both repo and SQL layers; credential-shape rejection covers `sk-`, `Bearer `, AWS access-key shape, JWT shape, and generic high-entropy tokens across `account_ref`, the JSON tree under `sandbox_constraints`, and every `store_locations_checked` entry. Anti-scope is honored end-to-end.
