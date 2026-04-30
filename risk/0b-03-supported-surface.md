# WU-0B-03 — Supported-Surface Gate

Reviewer: claude-opus
Branch: impl-wu-0b-03

## Verdict

LOW. No new Tauri commands are registered, no migration files are added, no Phase 0B domain tables are created, and the fixture builder is free of subprocess, provider, UI, optimizer, and recovery side effects.

## Tauri command count

- `phase_0a_scaffold_commands()` (`src-tauri/src/lib.rs:31-33`) still returns `&PHASE_0A_SCAFFOLD_COMMANDS` where `PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:17`). The lib.rs invoke handler is unchanged.
- `empty_seed_plan_creates_isolated_schema_versions_fixture` asserts `phase_0a_scaffold_commands()` returns `["subscribe_workspace_events"]` (`graphstore_fixture_contract.rs:104`).
- `fixture_builder_has_no_subprocess_provider_ui_optimizer_or_recovery_side_effects` re-asserts the same single-element list against `no-side-effects-assertions.json` (`graphstore_fixture_contract.rs:210-217`).

## Migration directory

- `ls src-tauri/migrations/` shows a single file: `0001_schema_versions.sql`. WU-0B-03 added no migration files.
- The contract test enumerates the directory and compares against `allowed_migration_files: ["0001_schema_versions.sql"]` (`graphstore_fixture_contract.rs:188-200`, `no-side-effects-assertions.json`). A drift would fail the test deterministically.

## Runtime tables

- `allowed_runtime_tables: ["schema_versions"]` and the contract test asserts `sqlite_table_names(&pool.sqlite).await == ["schema_versions"]` after fixture creation (`graphstore_fixture_contract.rs:206-209`). No Phase 0B domain table is created at fixture build time.

## Subprocess / provider / UI / optimizer / recovery absence

- `grep -rn "Command::new\|std::process::" src-tauri/src/graphstore/fixture.rs` returns no matches.
- The contract test's `forbidden_source_tokens` set (`no-side-effects-assertions.json`) covers `Command::new`, `std::process::Command`, `tokio::process`, `agents`, `provider`, `optimizer`, `recovery`, `tauri::command`, `generate_handler!`, `invoke_handler`, `CREATE TABLE`. The test reads `fixture.rs` and asserts none are present (`graphstore_fixture_contract.rs:182-187`).
- `fixture.rs` imports only `serde`, `sqlx::SqlitePool`, the WU-0B-01 contract's `MigrationError`, the WU-0B-01 runner `run_migrations`, the WU-0B-02 prelude (`GraphStoreError`, `OpaqueId`), and the WU-0A-14a temp harness API (`fixture.rs:1-11`). No Tauri, no `Command`, no provider modules.

## Findings

None at supported-surface severity. No `WU-0B-03-SUPPORTED-SURFACE-F<NN>` IDs assigned.
