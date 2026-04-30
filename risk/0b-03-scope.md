# WU-0B-03 — Scope Gate

Reviewer: claude-opus
Branch: impl-wu-0b-03
Inputs: ticket `WU-0B-03.md`, `proposals/0b-03-wu-0b-03.md`, `product-strategy/contracts/wu-0b-03-graphstore-fixtures.md`, `src-tauri/src/graphstore/fixture.rs`, `src-tauri/src/contracts/graphstore_fixture.rs`, `src-tauri/tests/graphstore_fixture_contract.rs`.

## Verdict

LOW. All six acceptance criteria are exercised by the contract test, and the contract surface matches the ticket and the converged contract document.

## Acceptance criteria coverage

- AC1 — empty seed creates isolated temp DB, reports applied migrations; invalid seed returns documented variant without partial writes.
  - Exercised by `empty_seed_plan_creates_isolated_schema_versions_fixture` and `future_wu_seed_plan_returns_unknown_ref_before_database_creation` (`graphstore_fixture_contract.rs:83-105`, `:107-134`). Latter test asserts source-order: `validate_seed_plan(&seed_plan)?` precedes `temp_harness_state(` so an invalid plan never reaches the temp harness.
- AC2 — reset clears WU-owned rows and preserves migration history exactly; documented invalid case returns documented variant without partial writes.
  - Exercised by `reset_preserves_schema_versions_history_exactly` (`graphstore_fixture_contract.rs:136-159`). Reads `schema_versions` rows before and after reset and asserts byte-for-byte equality, plus rerunning `run_migrations` reports `applied_versions == []` and `skipped_versions == [1]`. WU-0B-03 owns no domain rows, so the no-op reset is consistent with the contract.
- AC3 — seed plans for future-WU refs return `GraphStoreError::UnknownRef`.
  - `validate_seed_plan` in `fixture.rs:116-122` returns `UnknownRef` whenever `future_refs` is non-empty; test verifies the variant and its `code()`.
- AC4 — no provider, subprocess, UI, optimizer, recovery, or render side effects.
  - `fixture_builder_has_no_subprocess_provider_ui_optimizer_or_recovery_side_effects` (`graphstore_fixture_contract.rs:174-218`) greps `fixture.rs` for `Command::new`, `std::process::Command`, `tokio::process`, `agents`, `provider`, `optimizer`, `recovery`, `tauri::command`, `generate_handler!`, `invoke_handler`, `CREATE TABLE`. None are present.
- AC5 — every fixture-created ID passes the WU-0B-02 opaque ID invariant.
  - `validate_fixture_created_refs` (`fixture.rs:105-114`) reconstructs each `OpaqueId<GraphStoreFixtureRef>` through `OpaqueId::new`, which routes through `validate_opaque_id_parts`. Test `fixture_created_refs_are_validated_through_opaque_id_hook` asserts the empty case is accepted.
- AC6 — downstream WUs can run contract tests independently against a fresh fixture.
  - `fixtures_created_in_same_test_have_independent_temp_paths` (`graphstore_fixture_contract.rs:220-236`) creates two fixtures back to back, asserts `sqlite_url` and `workspace_root` differ, and confirms each pool has its own one-row `schema_versions` history.

## Contract surface match

- `GraphStorePool { sqlite_url, sqlite, migrations_applied, workspace_root }` (`fixture.rs:38-43`) matches `wu-0b-03-graphstore-fixtures.md:8-13`.
- `GraphStoreFixture { pool, workspace_id?, graph_version?, created_refs }` (`fixture.rs:46-51`) matches the contract.
- `GraphStoreRepo<T>` (`fixture.rs:53-64`) is a trait with `insert / get / list_by_workspace`. No catch-all dynamic repository; downstream WUs implement per-record types.
- `create_graphstore_fixture(seed_plan) -> Result<GraphStoreFixture, GraphStoreError>` (`fixture.rs:66-92`) and `reset_graphstore_fixture(fixture) -> Result<(), GraphStoreError>` (`fixture.rs:94-103`) signatures match.
- `validate_fixture_created_refs` is added to support AC5 and is documented in the contract surface (`wu-0b-03-graphstore-fixtures.md:30`).
- Marker types `GraphWorkspaceRef` and `GraphStoreFixtureRef` (`fixture.rs:17-20`) carry phantom data for the typed `OpaqueId<T>` parameters.

## Findings

None at scope severity. No `WU-0B-03-SCOPE-F<NN>` IDs assigned.
