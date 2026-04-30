# WU-0B-09 — Supported-Surface Risk Review

**Gate:** Phase 8 supported surface.
**Severity:** LOW.

## Question

Does the implementation respect the Phase 0A scaffold-command invariant (`phase_0a_scaffold_commands()` length 1), add only the single declared durable table `evidence_artifacts`, register no Tauri commands, and keep prior-WU runtime tables and migration ordering intact? Are the prior-WU contract test updates strictly bounded to migration list / runtime-table list bumps that flow naturally from adding migration 9?

## Findings

### WU-0B-09-SURFACE-F01 — `phase_0a_scaffold_commands()` length still 1

`src-tauri/src/lib.rs:17` keeps `const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` unchanged; `phase_0a_scaffold_commands()` returns the slice (`:31-33`); `registered_command_count()` returns `1` (`:35-37`). `git diff main -- src-tauri/src/lib.rs` is empty — this WU does not touch `lib.rs`. The contract test at `wu_0b_09_evidenceartifact_contract.rs:365` asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` live and passes. The same invariant is preserved by every prior-WU contract test that was edited (see SURFACE-F04).

### WU-0B-09-SURFACE-F02 — Only one new durable table: `evidence_artifacts`

`src-tauri/migrations/0b/09_evidenceartifact.sql` declares exactly one `CREATE TABLE` (`evidence_artifacts`) plus three `CREATE INDEX` statements (`idx_evidence_artifacts_workspace_id`, `idx_evidence_artifacts_content_hash`, `idx_evidence_artifacts_capture_state`). No additional tables, no triggers, no views. The runtime-table assertion in the contract test (`:367-376`) lists the exact six tables `[evidence_artifacts, graph_configurations, graph_nodes, graph_workspaces, policy_sets, schema_versions]` — that's the prior five plus this one.

### WU-0B-09-SURFACE-F03 — No new Tauri commands; no allowlist creep; no new dependencies

- `git diff main -- src-tauri/tauri.conf.json` — empty.
- `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json` — empty.
- The implementation imports already-present `serde`, `sqlx`, and `std::fs` / `std::path` — no new crates.
- `grep` for `tauri::command|generate_handler|invoke_handler|app_handle\.emit|tauri::ipc::Channel` against `src-tauri/src/graphstore/evidenceartifact.rs` returns nothing (also checked by the contract test's forbidden-token list at `:379-394`).

### WU-0B-09-SURFACE-F04 — Prior-WU test updates are bounded to migration-list / runtime-table bumps

`git diff main -- src-tauri/tests/` touches six prior-WU test files; every diff is one of two structural shapes:

| File | Hunks | Shape |
| --- | --- | --- |
| `tests/graphstore_fixture_contract.rs` | 3 | (a) `migrations_applied` literal `[1,4,5,6,7]` → `[1,4,5,6,7,9]`; (b) runtime-table list now includes `"evidence_artifacts"`; (c) post-reset `skipped_versions` mirror change; (d) `migration_records().len()` 5 → 6. |
| `tests/graphstore_migrations_contract.rs` | 1 | `applied_versions` `[1,4,5,6,7]` → `[1,4,5,6,7,9]`; `durable_tables` adds `"evidence_artifacts"`. |
| `tests/wu_0b_04_policyset_contract.rs` | 1 | `migrations_applied` literal bump. |
| `tests/wu_0b_05_graphconfiguration_contract.rs` | 2 | `migrations_applied` literal bump; runtime-table list adds `"evidence_artifacts"`. |
| `tests/wu_0b_06_graphworkspace_contract.rs` | 2 | identical shape. |
| `tests/wu_0b_07_graphnode_contract.rs` | 2 | identical shape. |

`git diff main --stat` reports 21 insertions / 13 deletions across 13 files in total. No prior-WU test grew a new test, no test was deleted, no test was renamed, no behavioral assertion changed shape — every edit is a literal-list bump that mechanically follows from migration 9 being added to the shipped Phase 0B set. Behavior of every prior WU is unchanged (the same insert/get/list/round-trip/rollback assertions still execute against the same tables).

### WU-0B-09-SURFACE-F05 — Prior-WU implementation files are untouched

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs` returns empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs` returns empty. The two `mod.rs` edits are single-line `pub mod evidenceartifact;` insertions in alphabetical order (`src-tauri/src/contracts/mod.rs`, `src-tauri/src/graphstore/mod.rs`) — wiring only.

### WU-0B-09-SURFACE-F06 — Migration ordering / Phase 0B set integrity

The shipped Phase 0B migration set is `[1, 4, 5, 6, 7, 9]` after this WU; this matches the migration filenames on disk (`src-tauri/migrations/0b/`: `04_policyset.sql`, `05_graphconfiguration.sql`, `06_graphworkspace.sql`, `07_graphnode.sql`, `09_evidenceartifact.sql`). The version-8 slot is reserved for a future WU and the implementer correctly skips it rather than renumbering — `evidence_artifact.schema_version DEFAULT 9` (`migrations/0b/09_evidenceartifact.sql:62`) and the `FOREIGN KEY(schema_version) REFERENCES schema_versions(version)` enforce that the row's `schema_version` matches the registered migration. The post-reset assertion `applied_versions = [], skipped_versions = [1,4,5,6,7,9]` (`product-strategy/contracts/fixtures/wu-0b-03/reset-preserves-migration-history.json`) confirms the harness sees all six migrations as already-applied after a reset and re-run.

### WU-0B-09-SURFACE-F07 — Contract test boundary list is the only enumerable harness-import surface

`evidenceartifact_contract_tests_stay_inside_declared_wu_boundary` (`tests/wu_0b_09_evidenceartifact_contract.rs:398-424`) asserts the exact 11 `use agent_harness_lib::` lines — all of which are this WU's own contract module, the prelude/fixture infra, the FK-target predecessor contracts (`policyset`, `graphconfiguration`, `graphworkspace`), or `phase_0a_scaffold_commands`. No imports of agents, optimizers, providers, recovery, or pre-WU-0B-04 modules — those would be drift. **Justified.**

### WU-0B-09-SURFACE-F08 — No new top-level modules, no new commands directory, no new fixture directories outside `wu-0b-09/`

- `git ls-files --others --exclude-standard` lists only the 15 expected paths under `product-strategy/contracts/fixtures/wu-0b-09/`, `product-strategy/contracts/wu-0b-09-evidenceartifact.md`, `src-tauri/migrations/0b/09_evidenceartifact.sql`, `src-tauri/src/contracts/evidenceartifact.rs`, `src-tauri/src/graphstore/evidenceartifact.rs`, and `src-tauri/tests/wu_0b_09_evidenceartifact_contract.rs`.
- No new `src-tauri/src/commands/` items, no new `src/contracts/` TS items, no new modules registered in `lib.rs`.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ FULL TURBO cache hit |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 86/86 |
| `cargo test` (full suite) | ✓ all suites pass; `wu_0b_09_evidenceartifact_contract` 11/11; prior-WU contract tests still 100% green |
| `cargo fmt --check` | ✓ no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | ✓ no warnings |
| `git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs` | empty |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty |

## Verdict

**LOW.** The supported-surface promise holds: `phase_0a_scaffold_commands()` length is still 1 (verified live by the WU-0B-09 contract test and by every prior-WU contract test still asserting the same shape); the only new durable table is `evidence_artifacts`; the runtime-table set grows by exactly one entry; no new Tauri commands or allowlist edits; no new dependencies; no `lib.rs` change. Prior-WU implementation files are byte-identical to `main`. The six prior-WU test files were edited only to bump migration-list literals (`[1,4,5,6,7]` → `[1,4,5,6,7,9]`) and the runtime-table list (adds `"evidence_artifacts"` in alphabetical order) — mechanical updates that follow from adding migration 9 to the shipped set, not behavioral changes.
