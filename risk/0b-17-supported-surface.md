# WU-0B-17 — Supported-Surface Risk Review

**Gate:** Phase 8 supported surface.
**Severity:** LOW.

## Question

Does the implementation respect the Phase 0A scaffold-command invariant (`phase_0a_scaffold_commands()` length 1), add only the single declared durable table `provider_states`, register no Tauri commands, and keep prior-WU runtime tables and migration ordering intact? Are the prior-WU contract test updates strictly bounded to migration list / runtime-table / FK-target reflection bumps that flow naturally from adding migration 17? **Critically**: are the prior-shipped migration files (`15_auditevent.sql`, `16_budgetledger.sql`) byte-identical to `main` (preserving the WU-0B-01 checksum invariant)?

## Findings

### WU-0B-17-SURFACE-F01 — `phase_0a_scaffold_commands()` length still 1

`src-tauri/src/lib.rs:17` keeps `const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` unchanged; `phase_0a_scaffold_commands()` returns the slice (`:31-33`); `registered_command_count()` returns `1` (`:35-37`). `git diff main -- src-tauri/src/lib.rs` is empty — this WU does not touch `lib.rs`. The contract test at `wu_0b_17_providerstate_contract.rs:351` asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` live and passes. Every other prior-WU contract test still asserts the same shape after their migration-list bump, all green.

### WU-0B-17-SURFACE-F02 — Only one new durable table: `provider_states`

`src-tauri/migrations/0b/17_providerstate.sql:1-100` declares `provider_states` plus its five indexes. The remaining DDL (`:117-350`) recreates `audit_events` and `budget_ledgers` to add the new FK to `provider_states` — these are *not new tables*, they are the same prior tables being reshaped via the standard SQLite idiom for adding a foreign key to an existing column.

The runtime-table assertion in this WU's contract test (`:352-365`) lists exactly nine tables `[audit_events, budget_ledgers, evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, provider_states, schema_versions]` — that's the prior eight plus `provider_states` in alphabetical position 8. Every prior-WU contract test that asserts the runtime-table set has been updated to add `"provider_states"` in the same alphabetical position. No views, no triggers, no extra hidden tables.

### WU-0B-17-SURFACE-F03 — Prior-shipped migration files are BYTE-IDENTICAL to main (checksum invariant preserved)

This is the critical check. The implementer reported "upgraded `audit_events.provider_state_id` and `budget_ledgers.provider_state_id` to real nullable FKs targeting `provider_states`" — three mechanisms could deliver this, two of them dangerous:

| Path | Risk | Status |
| --- | --- | --- |
| (a) Modify `15_auditevent.sql` / `16_budgetledger.sql` in place | **HIGH** — breaks WU-0B-01 checksum invariant; existing deployments would mismatch on re-run | **Not taken.** `git diff main -- src-tauri/migrations/0b/15_auditevent.sql src-tauri/migrations/0b/16_budgetledger.sql` is empty. |
| (b) Use `ALTER TABLE ... ADD CONSTRAINT FOREIGN KEY` in `17_providerstate.sql` | Would have failed — SQLite does not support `ADD FK` on existing tables | Not applicable. |
| (c) Recreate the table via `RENAME → CREATE → INSERT...SELECT → DROP` inside `17_providerstate.sql` | Acceptable — the standard SQLite idiom; preserves checksum invariant on the *files*, all data is preserved or NULLed via `WHERE EXISTS` | **Taken.** `migrations/0b/17_providerstate.sql:117-217` for `audit_events`, `:231-341` for `budget_ledgers`. |

The recreated `audit_events` table at `migrations/0b/17_providerstate.sql:119-177` is byte-equivalent to the original `15_auditevent.sql:1-57` shape (same 23 columns, same `event_type`/`actor`/`reason_code` `trim() <> ''` CHECKs, same paired-null CHECKs for optional `configuration_id` and `provider_state_id`, same JSON `array`/`length` CHECKs, same `decision` enum CHECK, same FKs to `graph_workspaces` / `policy_sets` / `graph_configurations` / `schema_versions`, same `schema_version DEFAULT 15`) **plus** the new `FOREIGN KEY(provider_state_id_value, provider_state_id_namespace) REFERENCES provider_states(...)` (`:174-175`). The four named indexes are recreated identically (`:219-229`).

The recreated `budget_ledgers` table at `migrations/0b/17_providerstate.sql:233-299` is byte-equivalent to the original `16_budgetledger.sql:1-65` shape (same 24 columns, same per-counter `>= 0` CHECKs, same paired-null CHECK for optional `provider_state_id`, same `scope_type` / `budget_state` / `policy_action` enum CHECKs, same four-arm matrix CHECK, same FKs to `graph_workspaces` / `schema_versions`, same `schema_version DEFAULT 16`) **plus** the new `FOREIGN KEY(provider_state_id_value, provider_state_id_namespace) REFERENCES provider_states(...)` (`:296-297`). The three named indexes are recreated identically (`:343-350`).

The `INSERT ... SELECT` backfill clauses (`:188-215, 311-339`) preserve `row_id`, every original column, and use `CASE WHEN ... AND EXISTS (SELECT 1 FROM provider_states ...) THEN ... ELSE NULL END` to NULL out any previously-stored soft-ref values that don't have a corresponding `provider_states` row — preserving the new FK invariant on backfill. `DROP TABLE` of the renamed staging tables (`:217, 341`) leaves no extra runtime artifacts.

The contract test at `:253-263` asserts via `PRAGMA foreign_key_list(audit_events)` and `PRAGMA foreign_key_list(budget_ledgers)` that both now reference `provider_states`. The prior-WU contract tests at `wu_0b_15_auditevent_contract.rs:243` and `wu_0b_16_budgetledger_contract.rs:269` were updated to include `"provider_states"` in the expected FK target list (replacing the previous "must remain a soft ref in WU-0B-16" negative assertion). All green.

**Bottom line:** the migration-checksum invariant is intact. The implementer chose path (c) — the only safe one of the three.

### WU-0B-17-SURFACE-F04 — No new Tauri commands; no allowlist creep; no new dependencies

- `git diff main -- src-tauri/tauri.conf.json` — empty.
- `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json` — empty.
- The implementation imports already-present `serde`, `sqlx`, `serde_json`, the WU-0B-02 prelude (`OpaqueId`, `JsonField`, `RecordMeta`, `ActorRef`, `validate_record_meta`, `GraphStoreError`), and reuses `ProviderStateRef` from WU-0B-15 (`auditevent.rs:18`) — no new crates, no new modules.
- The contract-test forbidden-token grep (`:367-383`) covers `Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler` against `src-tauri/src/graphstore/providerstate.rs`. None present.

### WU-0B-17-SURFACE-F05 — Prior-WU contract test updates are bounded to migration-list / runtime-table / FK-target bumps

`git diff main -- src-tauri/tests/` touches eight prior-WU test files; every diff falls into one of three structural shapes:

| File | Hunks | Shape |
| --- | --- | --- |
| `tests/graphstore_fixture_contract.rs` | 4 | (a) `migrations_applied` literal `[1,4,5,6,7,9,15,16]` → `[1,4,5,6,7,9,15,16,17]`; (b) runtime-table list adds `"provider_states"` (alphabetical position 8); (c) post-reset `skipped_versions` mirror change; (d) `migration_records().len()` 8 → 9. |
| `tests/graphstore_migrations_contract.rs` | 1 | `applied_versions` literal bump; `durable_tables` adds `"provider_states"`. |
| `tests/wu_0b_04_policyset_contract.rs` | 1 | `migrations_applied` literal bump only. |
| `tests/wu_0b_05_graphconfiguration_contract.rs` | 2 | `migrations_applied` literal bump; runtime-table list adds `"provider_states"`. |
| `tests/wu_0b_06_graphworkspace_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_07_graphnode_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_09_evidenceartifact_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_15_auditevent_contract.rs` | 3 | (a) migration-list bump; (b) FK-target list grows from `[..., graph_configurations]` to `[..., graph_configurations, provider_states]` — this reflects the new real FK (was previously a soft ref, see SURFACE-F03); (c) runtime-table list adds `"provider_states"`. |
| `tests/wu_0b_16_budgetledger_contract.rs` | 3 | (a) migration-list bump; (b) FK-target list grows by `provider_states` AND removes the previous negative assertion `"provider_state_id must remain a soft ref in WU-0B-16"` — this is the natural inversion: the soft-ref guarantee is *bounded to WU-0B-16's slice*, and is correctly lifted in WU-0B-17 (the WU that introduces the FK target); (c) runtime-table list adds `"provider_states"`. |

`git diff main --stat` reports 33 insertions / 18 deletions across 13 prior-WU files in total. No prior-WU test grew a new test, no test was deleted, no test was renamed. The single non-mechanical update is the inversion of the WU-0B-16 soft-ref negative assertion, which is *required* by the introduction of `provider_states` as the FK target — leaving the negative assertion in place would have made `wu_0b_16_budgetledger_contract` fail the moment migration 17 is applied to its fixture pool.

### WU-0B-17-SURFACE-F06 — Prior-WU implementation files are untouched

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs src-tauri/src/graphstore/auditevent.rs src-tauri/src/graphstore/budgetledger.rs` returns empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs src-tauri/src/contracts/evidenceartifact.rs src-tauri/src/contracts/auditevent.rs src-tauri/src/contracts/budgetledger.rs` returns empty. The two `mod.rs` edits are single-line `pub mod providerstate;` insertions in alphabetical order (`src-tauri/src/contracts/mod.rs:19`, `src-tauri/src/graphstore/mod.rs:11`) — wiring only.

### WU-0B-17-SURFACE-F07 — Migration ordering / Phase 0B set integrity

The shipped Phase 0B migration set is `[1, 4, 5, 6, 7, 9, 15, 16, 17]` after this WU; this matches the migration filenames on disk (`src-tauri/migrations/0b/`: `04_policyset.sql`, `05_graphconfiguration.sql`, `06_graphworkspace.sql`, `07_graphnode.sql`, `09_evidenceartifact.sql`, `15_auditevent.sql`, `16_budgetledger.sql`, `17_providerstate.sql`). Version slots 8 and 10–14 are reserved for future WUs; the implementer correctly skips them rather than renumbering. `provider_states.schema_version DEFAULT 17` (`migrations/0b/17_providerstate.sql:94`) and `FOREIGN KEY(schema_version) REFERENCES schema_versions(version)` (`:99`) enforce the row's `schema_version` matches the registered migration. The post-reset assertion `applied_versions = [], skipped_versions = [1,4,5,6,7,9,15,16,17]` (`product-strategy/contracts/fixtures/wu-0b-03/reset-preserves-migration-history.json`) confirms the harness sees all nine migrations as already-applied after reset and re-run.

### WU-0B-17-SURFACE-F08 — `wu-0b-03` graphstore-fixture-shape sample fixture catches up by two migrations

`product-strategy/contracts/fixtures/wu-0b-03/graphstore-fixture-shape.json` and `graphstore-pool-shape.json` had `migrations_applied: [1, 4, 5, 6, 7, 9, 15]` on `main` — they were not bumped during the WU-0B-16 pass (a pre-existing inconsistency on `main` rather than fresh damage). The current diff updates them to `[1, 4, 5, 6, 7, 9, 15, 16, 17]`, fixing both the pre-existing missing 16 and the new 17. These two files are illustrative-shape sample fixtures (not asserted byte-by-byte by any test), so the late catch-up is bounded scope: the assertion-driven companion fixtures (`reset-preserves-migration-history.json`, `no-side-effects-assertions.json`) were already at the right cadence. **Acceptable.**

### WU-0B-17-SURFACE-F09 — No new top-level modules, no new commands directory, no new fixture directories outside `wu-0b-17/`

`git ls-files --others --exclude-standard` lists only the expected paths under `product-strategy/contracts/fixtures/wu-0b-17/` (14 fixtures: `auth-state-variants`, `billing-state-variants`, `canonical-row`, `cli-variants`, `confidence-variants`, `freshness-variants`, `network-state-variants`, `provider-variants`, `quota-state-variants`, `round-trip`, `runtime-state-variants`, `secret-rejections`, `stale-marking`, `unknown-variants`), `product-strategy/contracts/wu-0b-17-providerstate.md`, `src-tauri/migrations/0b/17_providerstate.sql`, `src-tauri/src/contracts/providerstate.rs`, `src-tauri/src/graphstore/providerstate.rs`, and `src-tauri/tests/wu_0b_17_providerstate_contract.rs`. No new `src-tauri/src/commands/` items, no new `src/contracts/` TS items, no new modules registered in `lib.rs`.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test --test wu_0b_17_providerstate_contract` | ✓ 16/16 |
| `cargo test --test wu_0b_15_auditevent_contract` | ✓ 9/9 |
| `cargo test --test wu_0b_16_budgetledger_contract` | ✓ 10/10 |
| `cargo test --test graphstore_fixture_contract` | ✓ all green |
| `cargo test --test graphstore_migrations_contract` | ✓ 12/12 |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |
| `git diff main -- src-tauri/migrations/0b/15_auditevent.sql src-tauri/migrations/0b/16_budgetledger.sql` | **empty** (checksum invariant intact) |
| `git diff main -- src-tauri/src/graphstore/auditevent.rs src-tauri/src/graphstore/budgetledger.rs` | **empty** |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty |

## Verdict

**LOW.** The supported-surface promise holds:
- `phase_0a_scaffold_commands()` length is still 1 (verified live by this WU's contract test and by every prior-WU contract test still asserting the same shape).
- The only new *durable table* is `provider_states`; the recreated `audit_events` / `budget_ledgers` are byte-equivalent to their original shape plus the new FK — no new runtime artifacts beyond `provider_states`.
- **Critically**: `git diff main -- src-tauri/migrations/0b/15_auditevent.sql src-tauri/migrations/0b/16_budgetledger.sql` is empty. The implementer chose the only safe of three possible mechanisms (the SQLite `RENAME → CREATE → INSERT...SELECT → DROP` table-recreation idiom inside `17_providerstate.sql`), preserving the WU-0B-01 migration-checksum invariant.
- No new Tauri commands or allowlist edits; no new dependencies; no `lib.rs` change.
- Prior-WU implementation files (Rust `graphstore/*.rs` and `contracts/*.rs`) are byte-identical to `main`.
- Prior-WU contract test files were edited only to bump migration-list literals, add `"provider_states"` to runtime-table assertions in alphabetical position, add `"provider_states"` to the FK-target list for `audit_events` / `budget_ledgers`, and remove the WU-0B-16 negative `"must remain a soft ref"` assertion that had been bounded to its slice — mechanical updates required by adding migration 17 to the shipped set, not behavioral changes.
- Reserved version slots 8 and 10–14 are correctly skipped, not renumbered.
