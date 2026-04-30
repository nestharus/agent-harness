# WU-0B-16 — Supported-Surface Risk Review

**Gate:** Phase 8 supported surface.
**Severity:** LOW.

## Question

Does the implementation respect the Phase 0A scaffold-command invariant (`phase_0a_scaffold_commands()` length 1), add only the single declared durable table `budget_ledgers`, register no Tauri commands, and keep prior-WU runtime tables and migration ordering intact? Are the prior-WU contract test updates strictly bounded to migration list / runtime-table list bumps that flow naturally from adding migration 16?

## Findings

### WU-0B-16-SURFACE-F01 — `phase_0a_scaffold_commands()` length still 1

`src-tauri/src/lib.rs:17` keeps `const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` unchanged; `phase_0a_scaffold_commands()` returns the slice (`:31-33`); `registered_command_count()` returns `1` (`:35-37`). `git diff main -- src-tauri/src/lib.rs` is empty — this WU does not touch `lib.rs`. The contract test at `wu_0b_16_budgetledger_contract.rs:379` asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` live and passes. The same invariant is preserved by every prior-WU contract test (which still asserts the same shape).

### WU-0B-16-SURFACE-F02 — Only one new durable table: `budget_ledgers`

`src-tauri/migrations/0b/16_budgetledger.sql` declares exactly one `CREATE TABLE` (`budget_ledgers`) plus three `CREATE INDEX` statements (`idx_budget_ledgers_workspace_id`, `idx_budget_ledgers_scope_type`, `idx_budget_ledgers_budget_state`). No additional tables, no triggers, no views. The runtime-table assertion in the contract test (`:380-392`) lists the exact eight tables `[audit_events, budget_ledgers, evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, schema_versions]` — that's the prior seven plus this one in alphabetical order.

### WU-0B-16-SURFACE-F03 — No new Tauri commands; no allowlist creep; no new dependencies

- `git diff main -- src-tauri/tauri.conf.json` — empty.
- `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json` — empty.
- The implementation imports already-present `serde`, `sqlx`, the WU-0B-02 prelude (`OpaqueId`, `JsonField`, `RecordMeta`, `ActorRef`, `validate_record_meta`, `GraphStoreError`), and reuses `ProviderStateRef` from WU-0B-15 — no new crates, no new modules.
- The contract-test forbidden-token grep (`:395-410`) covers `Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer_execution`, `recovery_execution`, `tauri::command`, `generate_handler!`, `invoke_handler` against `src-tauri/src/graphstore/budgetledger.rs`. None present.

### WU-0B-16-SURFACE-F04 — Prior-WU test updates are bounded to migration-list / runtime-table bumps

`git diff main -- src-tauri/tests/` touches eight prior-WU test files; every diff is one of two structural shapes:

| File | Hunks | Shape |
| --- | --- | --- |
| `tests/graphstore_fixture_contract.rs` | 3 | (a) `migrations_applied` literal `[1,4,5,6,7,9,15]` → `[1,4,5,6,7,9,15,16]`; (b) runtime-table list adds `"budget_ledgers"` (alphabetical position 2); (c) post-reset `skipped_versions` mirror change; (d) `migration_records().len()` 7 → 8. |
| `tests/graphstore_migrations_contract.rs` | 1 | `applied_versions` literal bump; `durable_tables` adds `"budget_ledgers"`. |
| `tests/wu_0b_04_policyset_contract.rs` | 1 | `migrations_applied` literal bump only (test does not assert runtime tables). |
| `tests/wu_0b_05_graphconfiguration_contract.rs` | 2 | `migrations_applied` literal bump; runtime-table list adds `"budget_ledgers"`. |
| `tests/wu_0b_06_graphworkspace_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_07_graphnode_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_09_evidenceartifact_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_15_auditevent_contract.rs` | 2 | identical shape. |

`git diff main --stat` reports 44 insertions / 13 deletions across 13 files in total. No prior-WU test grew a new test, no test was deleted, no test was renamed, no behavioral assertion changed shape — every edit is a literal-list bump that mechanically follows from migration 16 being added to the shipped Phase 0B set. Behavior of every prior WU is unchanged (the same insert/get/list/round-trip/rollback assertions still execute against the same tables).

### WU-0B-16-SURFACE-F05 — Prior-WU implementation files are untouched

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs src-tauri/src/graphstore/auditevent.rs` returns empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs src-tauri/src/contracts/evidenceartifact.rs src-tauri/src/contracts/auditevent.rs` returns empty. The two `mod.rs` edits are single-line `pub mod budgetledger;` insertions in alphabetical order (`src-tauri/src/contracts/mod.rs`, `src-tauri/src/graphstore/mod.rs`) — wiring only.

### WU-0B-16-SURFACE-F06 — Migration ordering / Phase 0B set integrity

The shipped Phase 0B migration set is `[1, 4, 5, 6, 7, 9, 15, 16]` after this WU; this matches the migration filenames on disk (`src-tauri/migrations/0b/`: `04_policyset.sql`, `05_graphconfiguration.sql`, `06_graphworkspace.sql`, `07_graphnode.sql`, `09_evidenceartifact.sql`, `15_auditevent.sql`, `16_budgetledger.sql`). Version slots 8 and 10–14 are reserved for future WUs; the implementer correctly skips them rather than renumbering. `budget_ledgers.schema_version DEFAULT 16` (`migrations/0b/16_budgetledger.sql:50`) and `FOREIGN KEY(schema_version) REFERENCES schema_versions(version)` (`:64`) enforce that the row's `schema_version` matches the registered migration. The post-reset assertion `applied_versions = [], skipped_versions = [1,4,5,6,7,9,15,16]` (`product-strategy/contracts/fixtures/wu-0b-03/reset-preserves-migration-history.json`) confirms the harness sees all eight migrations as already-applied after reset and re-run.

### WU-0B-16-SURFACE-F07 — Contract test boundary list is the only enumerable harness-import surface

`budgetledger_contract_tests_stay_inside_declared_wu_boundary` (`tests/wu_0b_16_budgetledger_contract.rs:414-440`) asserts the exact 11 `use agent_harness_lib::` lines — all of which are this WU's own contract module, the prelude/fixture infra, the FK-target predecessor contracts (`policyset`, `graphconfiguration`, `graphworkspace`), or `phase_0a_scaffold_commands`. No imports of agents, optimizers, providers, recovery, evidenceartifact, graphnode, auditevent, or non-FK pre-WUs — those would be drift. **Justified.**

### WU-0B-16-SURFACE-F08 — No new top-level modules, no new commands directory, no new fixture directories outside `wu-0b-16/`

`git ls-files --others --exclude-standard` lists only the expected paths under `product-strategy/contracts/fixtures/wu-0b-16/` (nine fixtures: `budget-state-variants`, `canonical-row`, `counter-rejections`, `fk-rejections`, `policy-action-invariants`, `policy-action-variants`, `round-trip`, `scope-type-variants`, `unknown-variants`), `product-strategy/contracts/wu-0b-16-budgetledger.md`, `proposals/0b-16-wu-0b-16.md`, `src-tauri/migrations/0b/16_budgetledger.sql`, `src-tauri/src/contracts/budgetledger.rs`, `src-tauri/src/graphstore/budgetledger.rs`, and `src-tauri/tests/wu_0b_16_budgetledger_contract.rs`. No new `src-tauri/src/commands/` items, no new `src/contracts/` TS items, no new modules registered in `lib.rs`, no `provider_states` migration or table.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test` (full suite) | ✓ all suites pass; `wu_0b_16_budgetledger_contract` 10/10; prior-WU contract tests still 100% green |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |
| `git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs src-tauri/src/graphstore/auditevent.rs` | empty |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty |

## Verdict

**LOW.** The supported-surface promise holds: `phase_0a_scaffold_commands()` length is still 1 (verified live by the WU-0B-16 contract test and by every prior-WU contract test still asserting the same shape); the only new durable table is `budget_ledgers`; the runtime-table set grows by exactly one entry; no new Tauri commands or allowlist edits; no new dependencies; no `lib.rs` change. Prior-WU implementation files are byte-identical to `main`. The eight prior-WU test files were edited only to bump migration-list literals (`[1,4,5,6,7,9,15]` → `[1,4,5,6,7,9,15,16]`) and the runtime-table list (adds `"budget_ledgers"` in alphabetical position 2) — mechanical updates that follow from adding migration 16 to the shipped set, not behavioral changes. The reserved version slots 8 and 10–14 are correctly skipped, not renumbered.
