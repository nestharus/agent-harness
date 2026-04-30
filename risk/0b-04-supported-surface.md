# WU-0B-04 — Supported-Surface Gate

Reviewer: claude-opus
Branch: impl-wu-0b-04

## Verdict

LOW. No new Tauri commands are registered, `phase_0a_scaffold_commands()` still has length 1, the only new migration is the bounded `0b/04_policyset.sql`, `policy_sets` is the only new durable domain table, and the prior-WU contract-test assertions are tightened (not relaxed) so the `0b/` subdirectory entry is the only allowed addition.

## Tauri command count

- `phase_0a_scaffold_commands()` (`src-tauri/src/lib.rs:31-33`) still returns `&PHASE_0A_SCAFFOLD_COMMANDS` where `PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:17`). Length is 1.
- `policyset_wu_has_no_operator_visible_behavior` re-asserts this against `no-side-effects.json` (`wu_0b_04_policyset_contract.rs:365-372`).
- `tauri_bootstrap_is_inert_and_command_free` (`scaffold_contract.rs:139-187`) re-asserts the same single-command allowlist and the absence of value-slice command names.
- `git diff main -- src-tauri/src/lib.rs` returns empty — no Tauri command registration changes in this WU.

## Migration runner surface

- `cargo manifest declares phase_0a_runtime_dependencies_without_sqlx_migrate_feature` (`scaffold_contract.rs:64-137`) still asserts `sqlx` features are exactly `["sqlite", "runtime-tokio", "macros"]` and explicitly rejects the `migrate` feature. WU-0B-04 did not bump or relax this.
- The same test enumerates `src-tauri/migrations/` and asserts entries equal `["0001_schema_versions.sql", "0b"]` (`scaffold_contract.rs:130-136`). The update is bounded — only the `0b` subdirectory is permitted, and the bootstrap migration is preserved.
- `tauri_bootstrap_is_inert_and_command_free` rejects value-slice command registration tokens `get_harness_settings` and `ping_runtime` (`scaffold_contract.rs:177-182`) and rejects empty `generate_handler![]` (`:183-186`). Both still pass.

## Migration files

- `ls src-tauri/migrations/` → `0001_schema_versions.sql`, `0b/`. `ls src-tauri/migrations/0b/` → `04_policyset.sql`. WU-0B-04 added one migration file, in a bounded subdirectory.
- `fixture_builder_has_no_subprocess_provider_ui_optimizer_or_recovery_side_effects` (`graphstore_fixture_contract.rs:174-218`) compares the directory listing against `allowed_migration_files: ["0001_schema_versions.sql", "0b"]` (`product-strategy/contracts/fixtures/wu-0b-03/no-side-effects-assertions.json`). A drift would fail the test deterministically.

## Runtime tables

- `shipped_phase_0b_migrations_emit_schema_versions_and_policy_sets` (`graphstore_migrations_contract.rs:560-581`) runs the shipped tree and asserts durable tables == `["policy_sets", "schema_versions"]`. `policy_sets` is the only new table.
- `empty_seed_plan_creates_isolated_schema_versions_fixture` (`graphstore_fixture_contract.rs:83-105`) asserts `sqlite_table_names(...) == ["policy_sets", "schema_versions"]` and `migrations_applied == [1, 4]`.
- `policyset_wu_has_no_operator_visible_behavior` (`wu_0b_04_policyset_contract.rs:357-385`) re-asserts the same allowlist against `no-side-effects.json`.

## Subprocess / provider / UI / optimizer / recovery absence

- `forbidden_source_tokens` for the WU-0B-04 source (`product-strategy/contracts/fixtures/wu-0b-04/no-side-effects.json`): `Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer`, `recovery_execution`, `invoke_handler`. The contract test reads `policyset.rs` and asserts none are present (`wu_0b_04_policyset_contract.rs:378-384`).
- The WU-0B-03 forbidden-token test (`graphstore_fixture_contract.rs:182-187`) already covers `agents`, `provider`, `tauri::command`, `generate_handler!`, `recovery`, `CREATE TABLE`, etc. for `fixture.rs`. WU-0B-04 did not perturb that surface.
- `policyset.rs` imports only `serde`, `serde_json::Value`, `sqlx::SqlitePool`, the WU-0B-03 fixture trait (`GraphStoreRepo`, `GraphStoreRepoFuture`, `GraphWorkspaceRef`), and the WU-0B-02 prelude (`policyset.rs:1-8`). No Tauri, no `Command`, no provider modules.

## Bounded prior-WU assertion updates

- `scaffold_contract.rs:131-136`: changed `vec!["0001_schema_versions.sql"]` → `vec!["0001_schema_versions.sql", "0b"]`. Adds the new subdirectory entry; preserves bootstrap migration; preserves comment intent.
- `graphstore_fixture_contract.rs:94, 102-103, 157, 234-235`: updated the migration-applied list from `[1]` to `[1, 4]`, the runtime-table list from `["schema_versions"]` to `["policy_sets", "schema_versions"]`, and migration-record-count from 1 to 2. Each edit reflects the new shipped state without relaxing structure.
- `graphstore_migrations_contract.rs:281-307` adds the recursive-duplicate test; `:560-581` renames `shipped_wu_0b_01_migration_emits_only_schema_versions` → `shipped_phase_0b_migrations_emit_schema_versions_and_policy_sets` and updates the asserted durable tables to `["policy_sets", "schema_versions"]`. The version assertion is `[1, 4]` — strict equality, not subset.
- `product-strategy/contracts/fixtures/wu-0b-03/*.json` updated similarly and consistently.

## Findings

None at supported-surface severity. No `WU-0B-04-SUPPORTED-SURFACE-F<NN>` IDs assigned.
