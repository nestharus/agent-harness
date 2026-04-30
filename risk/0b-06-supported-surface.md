# WU-0B-06 Supported-Surface Gate Review

**Severity:** LOW

## Result

`phase_0a_scaffold_commands()` still returns the single
`subscribe_workspace_events` command, the only new durable runtime
table is `graph_workspaces`, and the prior-WU test updates are bounded
to migration-list and runtime-table-list assertions. No new Tauri
commands, UI panes, agent invocations, provider probes, optimizer
execution, or recovery execution are added.

## Verification

- `PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]`
  (`src-tauri/src/lib.rs:17`); `lib.rs` `tests::phase_0a_registers_no_value_slice_commands`
  (`:78-85`) and the new
  `graphworkspace_wu_has_no_operator_visible_behavior`
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:346`) both
  assert the array equality.
- `sqlite_master` enumeration in
  `graphworkspace_wu_has_no_operator_visible_behavior`
  (`:347-355`) asserts the durable table set is exactly
  `["graph_configurations", "graph_workspaces", "policy_sets",
  "schema_versions"]` — only `graph_workspaces` is new. The migration
  itself (`src-tauri/migrations/0b/06_graphworkspace.sql:1-39`) creates
  exactly one durable table plus four indexes.
- Forbidden-token guard
  (`src-tauri/tests/wu_0b_06_graphworkspace_contract.rs:357-373`)
  asserts the implementation source contains none of `Command::new`,
  `std::process::Command`, `tokio::process`, `provider_probe`,
  `optimizer`, `recovery_execution`, `tauri::command`,
  `generate_handler!`, or `invoke_handler`. Verified via direct
  inspection of `src-tauri/src/graphstore/graphworkspace.rs`.
- Contract-test boundary check at `:381-400` pins the
  `agent_harness_lib::` import list to the predecessor surfaces plus
  this WU's contract and repo. Imports stay inside contracts/* surfaces
  except for `graphstore::policyset::PolicySetRepo`,
  `graphstore::graphconfiguration::GraphConfigurationRepo`, and
  `graphstore::graphworkspace::GraphWorkspaceRepo` — predecessor and
  own repository constructors needed to seed FK targets. This pattern
  is consistent with the WU-0B-05 contract test.
- Prior-WU test updates audited against
  `git diff main`:
  - `src-tauri/tests/wu_0b_04_policyset_contract.rs:64`: changes
    `migrations_applied` assertion from `[1, 4, 5]` to `[1, 4, 5, 6]`.
    Single-line, no behavioral change.
  - `src-tauri/tests/wu_0b_05_graphconfiguration_contract.rs:79`: same
    `[1, 4, 5, 6]` update; lines 332-336 add `graph_workspaces` to the
    runtime-tables expected list. No behavioral change to PolicySet or
    GraphConfiguration assertions.
  - `src-tauri/tests/graphstore_fixture_contract.rs:94,102-106,162,239-240`:
    migration list `[1, 4, 5, 6]`, runtime-tables list adds
    `graph_workspaces`, migration-record count goes from `3` to `4`.
    All structural updates required by the new migration; no fixture
    behavior changes.
  - `src-tauri/tests/graphstore_migrations_contract.rs:561,576,580`:
    test renamed from `..._policy_sets_and_graph_configurations` to
    `..._phase_0b_tables`, applied versions `[1, 4, 5, 6]`,
    durable-tables list adds `graph_workspaces`. The rename is a
    documentation update; assertions are still the strict-equality
    pattern documented for this gate.
  - Fixture JSON updates
    (`product-strategy/contracts/fixtures/wu-0b-03/graphstore-fixture-shape.json`,
    `graphstore-pool-shape.json`,
    `no-side-effects-assertions.json`,
    `wu-0b-04/no-side-effects.json`) only add `6` to migration arrays
    and `graph_workspaces` to allowed-runtime-tables arrays.
- `git diff main --stat` shows 12 modified tracked files plus the
  WU-0B-06 untracked additions (proposal, contract doc,
  fixtures/wu-0b-06/, migration `06_graphworkspace.sql`,
  `contracts/graphworkspace.rs`, `graphstore/graphworkspace.rs`,
  `tests/wu_0b_06_graphworkspace_contract.rs`). No unrelated files.

## Findings

No findings.
