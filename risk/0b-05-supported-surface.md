# WU-0B-05 Supported-Surface Gate Review

**Severity:** LOW

## Result

The supported surface is unchanged outside the declared GraphConfiguration
storage. `phase_0a_scaffold_commands()` still returns exactly
`["subscribe_workspace_events"]`. The migration runner is unchanged. The only
new durable table is `graph_configurations`. Prior-WU test updates are bounded
to migration version list and runtime tables list — no behavioral change to
PolicySet semantics.

## Verification

- `src-tauri/src/lib.rs:17` keeps
  `const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"];`.
  The `no_operator_visible_behavior_or_extra_tables_are_enabled` test asserts
  `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`.
- No new Tauri command, `tauri::command`, `generate_handler!`, or
  `invoke_handler` reference is added — the side-effect-absence test forbids
  these tokens in `graphconfiguration.rs` and passes.
- The migration runner (`graphstore/migrations.rs`) is not modified.
  `ls src-tauri/migrations/0b/` shows only `04_policyset.sql` and
  `05_graphconfiguration.sql`.
- `shipped_phase_0b_migrations_emit_schema_versions_policy_sets_and_graph_configurations`
  asserts the durable table list is exactly
  `["graph_configurations", "policy_sets", "schema_versions"]` after
  `report.applied_versions == [1, 4, 5]`.
- Prior-WU test updates are strictly bounded:
  - `wu_0b_04_policyset_contract.rs`: single line change from
    `vec![1, 4]` to `vec![1, 4, 5]` in the migrations-applied assertion.
    Nothing else in the file is touched, including the seven PolicySet
    behavioral tests, which all still pass.
  - `graphstore_fixture_contract.rs`: bumps the migrations-applied vec to
    `[1, 4, 5]`, adds `"graph_configurations"` to the durable table list, and
    bumps record-count expectations from 2 to 3.
  - `graphstore_migrations_contract.rs`: renames one shipped-Phase-0B test to
    include `_and_graph_configurations`, bumps its applied versions and table
    list. No existing assertion is loosened.
  - Fixture JSON updates under `wu-0b-03/` and `wu-0b-04/` mirror the new
    runtime tables and applied versions only.
- All 9 WU-0B-05 contract tests, 7 WU-0B-04 PolicySet tests, 12 graphstore
  migration tests, and the broader cargo test suite (~80+ tests) pass.
  `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` are
  clean. `bun run lint` and `bun run typecheck` are clean.

## Findings

### WU-0B-05-SUPPORTED-SURFACE-F01 (LOW)

None. Supported-surface boundary is preserved end-to-end.
