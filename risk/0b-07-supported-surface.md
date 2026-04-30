# WU-0B-07 Supported-Surface Gate Review

**Severity:** LOW

## Result

`phase_0a_scaffold_commands()` still returns the single
`subscribe_workspace_events` command, the only new durable runtime
table is `graph_nodes`, and the prior-WU test updates are bounded to
migration-list and runtime-table-list assertions. No new Tauri
commands, UI panes, agent invocations, provider probes, optimizer
execution, or recovery execution are added.

## Verification

- `phase_0a_scaffold_commands()` still returns
  `["subscribe_workspace_events"]` (length 1). The new contract test
  `graphnode_wu_has_no_operator_visible_behavior_or_extra_tables`
  asserts the array equality via
  `assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"])`
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:382`). `git diff
  main -- src-tauri/src/lib.rs` returns empty, confirming
  `PHASE_0A_SCAFFOLD_COMMANDS` is byte-identical to `main`.
- `sqlite_master` enumeration in
  `graphnode_wu_has_no_operator_visible_behavior_or_extra_tables`
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:383-392`) asserts
  the durable table set is exactly
  `["graph_configurations", "graph_nodes", "graph_workspaces",
  "policy_sets", "schema_versions"]` — only `graph_nodes` is new. The
  migration `src-tauri/migrations/0b/07_graphnode.sql:1-99` creates
  exactly one durable table plus four indexes; no additional CREATE
  TABLE / CREATE TRIGGER / CREATE VIEW statements appear.
- Forbidden-token guard
  (`src-tauri/tests/wu_0b_07_graphnode_contract.rs:394-410`) asserts
  the implementation source contains none of `Command::new`,
  `std::process::Command`, `tokio::process`, `provider_probe`,
  `optimizer_execution`, `recovery_execution`, `tauri::command`,
  `generate_handler!`, or `invoke_handler`. Verified via direct
  inspection of `src-tauri/src/graphstore/graphnode.rs` and
  `src-tauri/src/contracts/graphnode.rs`.
- Contract-test boundary check at
  `src-tauri/tests/wu_0b_07_graphnode_contract.rs:418-447` pins the
  `agent_harness_lib::` import list to the predecessor surfaces plus
  this WU's contract and repo. Imports stay inside `contracts/*`
  surfaces except for `graphstore::policyset::PolicySetRepo`,
  `graphstore::graphconfiguration::GraphConfigurationRepo`,
  `graphstore::graphworkspace::GraphWorkspaceRepo`, and
  `graphstore::graphnode::GraphNodeRepo` — predecessor and own
  repository constructors needed to seed FK targets. This pattern is
  consistent with the WU-0B-04/05/06 contract tests.
- Prior-WU test updates audited against `git diff main`:
  - `src-tauri/tests/wu_0b_04_policyset_contract.rs:64`: changes
    `migrations_applied` assertion from `[1, 4, 5, 6]` to
    `[1, 4, 5, 6, 7]`. Single-line, no behavioral change.
  - `src-tauri/tests/wu_0b_05_graphconfiguration_contract.rs:79,334`:
    same `[1, 4, 5, 6, 7]` update; runtime-tables expected list adds
    `graph_nodes`. No behavioral change to GraphConfiguration
    assertions.
  - `src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:79,351`:
    same migration-list update and runtime-tables list adds
    `graph_nodes`. No behavioral change to GraphWorkspace
    assertions.
  - `src-tauri/tests/graphstore_fixture_contract.rs:94,104,163,
    240-241`: migration list `[1, 4, 5, 6, 7]`, runtime-tables list
    adds `graph_nodes`, migration-record count goes from `4` to `5`.
    All structural updates required by the new migration; no fixture
    behavior changes.
  - `src-tauri/tests/graphstore_migrations_contract.rs:576,581`:
    applied versions `[1, 4, 5, 6, 7]`, durable-tables list adds
    `graph_nodes`. Strict-equality assertion pattern preserved.
  - Fixture JSON updates
    (`product-strategy/contracts/fixtures/wu-0b-03/graphstore-fixture-shape.json`,
    `graphstore-pool-shape.json`, `no-side-effects-assertions.json`,
    `wu-0b-04/no-side-effects.json`) only add `7` to migration
    arrays and `graph_nodes` to allowed-runtime-tables arrays.
- `git diff main --stat` shows 11 modified tracked files plus the
  WU-0B-07 untracked additions (proposal, contract doc,
  fixtures/wu-0b-07/, migration `07_graphnode.sql`,
  `contracts/graphnode.rs`, `graphstore/graphnode.rs`,
  `tests/wu_0b_07_graphnode_contract.rs`). No unrelated files.
- `git diff main -- src-tauri/Cargo.toml` returns empty output (no
  new dependencies, no version bumps, no feature flags); `git diff
  main -- src-tauri/src/lib.rs` returns empty (command surface
  unchanged).

## Findings

No findings.
