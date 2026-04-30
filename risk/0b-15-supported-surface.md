# WU-0B-15 — Supported-Surface Risk Review

**Gate:** Phase 8 supported surface.
**Severity:** LOW.

## Question

Does the implementation respect the Phase 0A scaffold-command invariant (`phase_0a_scaffold_commands()` length 1), add only the single declared durable table `audit_events`, register no Tauri commands, and keep prior-WU runtime tables and migration ordering intact? Are the prior-WU contract test updates strictly bounded to migration list / runtime-table list bumps that flow naturally from adding migration 15?

## Findings

### WU-0B-15-SURFACE-F01 — `phase_0a_scaffold_commands()` length still 1

`src-tauri/src/lib.rs:17` keeps `const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` unchanged; `phase_0a_scaffold_commands()` returns the slice (`:31-33`); `registered_command_count()` returns `1` (`:35-37`). `git diff main -- src-tauri/src/lib.rs` is empty — this WU does not touch `lib.rs`. The contract test at `wu_0b_15_auditevent_contract.rs:331` asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` live and passes. The same invariant is preserved by every prior-WU contract test (which still asserts the same shape).

### WU-0B-15-SURFACE-F02 — Only one new durable table: `audit_events`

`src-tauri/migrations/0b/15_auditevent.sql` declares exactly one `CREATE TABLE` (`audit_events`) plus four `CREATE INDEX` statements (`idx_audit_events_workspace_id`, `idx_audit_events_policy_set_id`, `idx_audit_events_decision`, `idx_audit_events_created_at`). No additional tables, no triggers, no views. The runtime-table assertion in the contract test (`:332-343`) lists the exact seven tables `[audit_events, evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, schema_versions]` — that's the prior six plus this one.

### WU-0B-15-SURFACE-F03 — No new Tauri commands; no allowlist creep; no new dependencies

- `git diff main -- src-tauri/tauri.conf.json` — empty.
- `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json` — empty.
- The implementation imports already-present `serde`, `sqlx`, and the WU-0B-02 prelude (`OpaqueId`, `JsonField`, `RecordMeta`, `ActorRef`, `validate_record_meta`, `GraphStoreError`) — no new crates.
- `grep` for `tauri::command|generate_handler|invoke_handler|app_handle\.emit|tauri::ipc::Channel` against `src-tauri/src/graphstore/auditevent.rs` returns nothing (also checked by the contract test's forbidden-token list at `:346-360`).

### WU-0B-15-SURFACE-F04 — Prior-WU test updates are bounded to migration-list / runtime-table bumps

`git diff main -- src-tauri/tests/` touches seven prior-WU test files; every diff is one of two structural shapes:

| File | Hunks | Shape |
| --- | --- | --- |
| `tests/graphstore_fixture_contract.rs` | 3 | (a) `migrations_applied` literal `[1,4,5,6,7,9]` → `[1,4,5,6,7,9,15]`; (b) runtime-table list now includes `"audit_events"` (alphabetical first); (c) post-reset `skipped_versions` mirror change; (d) `migration_records().len()` 6 → 7. |
| `tests/graphstore_migrations_contract.rs` | 1 | `applied_versions` `[1,4,5,6,7,9]` → `[1,4,5,6,7,9,15]`; `durable_tables` adds `"audit_events"`. |
| `tests/wu_0b_04_policyset_contract.rs` | 1 | `migrations_applied` literal bump only. |
| `tests/wu_0b_05_graphconfiguration_contract.rs` | 2 | `migrations_applied` literal bump; runtime-table list adds `"audit_events"`. |
| `tests/wu_0b_06_graphworkspace_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_07_graphnode_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_09_evidenceartifact_contract.rs` | 2 | identical shape. |

`git diff main --stat` reports 23 insertions / 14 deletions across 14 files in total. No prior-WU test grew a new test, no test was deleted, no test was renamed, no behavioral assertion changed shape — every edit is a literal-list bump that mechanically follows from migration 15 being added to the shipped Phase 0B set. Behavior of every prior WU is unchanged (the same insert/get/list/round-trip/rollback assertions still execute against the same tables).

### WU-0B-15-SURFACE-F05 — Prior-WU implementation files are untouched

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs` returns empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs src-tauri/src/contracts/evidenceartifact.rs` returns empty. The two `mod.rs` edits are single-line `pub mod auditevent;` insertions in alphabetical order (`src-tauri/src/contracts/mod.rs:1`, `src-tauri/src/graphstore/mod.rs:1`) — wiring only.

### WU-0B-15-SURFACE-F06 — Migration ordering / Phase 0B set integrity

The shipped Phase 0B migration set is `[1, 4, 5, 6, 7, 9, 15]` after this WU; this matches the migration filenames on disk (`src-tauri/migrations/0b/`: `04_policyset.sql`, `05_graphconfiguration.sql`, `06_graphworkspace.sql`, `07_graphnode.sql`, `09_evidenceartifact.sql`, `15_auditevent.sql`). Version slots 8 and 10–14 are reserved for future WUs; the implementer correctly skips them rather than renumbering. `audit_events.schema_version DEFAULT 15` (`migrations/0b/15_auditevent.sql:32`) and `FOREIGN KEY(schema_version) REFERENCES schema_versions(version)` (`:56`) enforce that the row's `schema_version` matches the registered migration. The post-reset assertion `applied_versions = [], skipped_versions = [1,4,5,6,7,9,15]` (`product-strategy/contracts/fixtures/wu-0b-03/reset-preserves-migration-history.json`) confirms the harness sees all seven migrations as already-applied after reset and re-run.

### WU-0B-15-SURFACE-F07 — Contract test boundary list is the only enumerable harness-import surface

`auditevent_contract_tests_stay_inside_declared_wu_boundary` (`tests/wu_0b_15_auditevent_contract.rs:364-391`) asserts the exact 11 `use agent_harness_lib::` lines — all of which are this WU's own contract module, the prelude/fixture infra, the FK-target predecessor contracts (`policyset`, `graphconfiguration`, `graphworkspace`), or `phase_0a_scaffold_commands`. No imports of agents, optimizers, providers, recovery, evidenceartifact, graphnode, or non-FK pre-WUs — those would be drift. **Justified.**

### WU-0B-15-SURFACE-F08 — No new top-level modules, no new commands directory, no new fixture directories outside `wu-0b-15/`

- `git ls-files --others --exclude-standard` lists only the expected paths under `product-strategy/contracts/fixtures/wu-0b-15/` (six fixtures: `canonical-row`, `decision-variants`, `fk-rejections`, `output-with-input-invariants`, `round-trip`, `unknown-decision`), `product-strategy/contracts/wu-0b-15-auditevent.md`, `proposals/0b-15-wu-0b-15.md`, `src-tauri/migrations/0b/15_auditevent.sql`, `src-tauri/src/contracts/auditevent.rs`, `src-tauri/src/graphstore/auditevent.rs`, and `src-tauri/tests/wu_0b_15_auditevent_contract.rs`.
- No new `src-tauri/src/commands/` items, no new `src/contracts/` TS items, no new modules registered in `lib.rs`, no `provider_states` migration or table.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test` (full suite) | ✓ all suites pass; `wu_0b_15_auditevent_contract` 9/9; prior-WU contract tests still 100% green |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |
| `git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs` | empty |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty |
| `grep -rn "UPDATE audit_events\|DELETE FROM audit_events" src-tauri/src/graphstore/auditevent.rs src-tauri/migrations/0b/15_auditevent.sql` | empty |

## Verdict

**LOW.** The supported-surface promise holds: `phase_0a_scaffold_commands()` length is still 1 (verified live by the WU-0B-15 contract test and by every prior-WU contract test still asserting the same shape); the only new durable table is `audit_events`; the runtime-table set grows by exactly one entry; no new Tauri commands or allowlist edits; no new dependencies; no `lib.rs` change. Prior-WU implementation files are byte-identical to `main`. The seven prior-WU test files were edited only to bump migration-list literals (`[1,4,5,6,7,9]` → `[1,4,5,6,7,9,15]`) and the runtime-table list (adds `"audit_events"` in alphabetical first position) — mechanical updates that follow from adding migration 15 to the shipped set, not behavioral changes. The reserved version slots 8 and 10–14 are correctly skipped, not renumbered.
