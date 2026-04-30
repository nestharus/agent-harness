# WU-0B-03 — Shortcut Gate

Reviewer: claude-opus
Branch: impl-wu-0b-03

## Verdict

LOW. `create_graphstore_fixture` composes the prior WUs without reimplementing them, the seed validator rejects future-WU refs strictly before any I/O so partial writes are impossible, reset preserves `schema_versions` exactly, and there are no `unwrap()` calls or TODOs in production code.

## Composition vs reimplementation

- Temp harness reuse: `temp_harness_state("empty", None, None, Vec::new())` and `harness_app_state(&handle)` (`fixture.rs:71-74`) reach the WU-0A-14a entrypoint directly. No re-creation of temp dirs, no separate `SqlitePool` opening, no shadow `LocalStorageLayout`. The pool's `workspace_root` and `sqlite_url` are derived from `app_state.storage_layout` (`fixture.rs:78-79`).
- Migration runner reuse: `run_migrations(&app_state.db, migrations_dir())` (`fixture.rs:75-77`) calls WU-0B-01 directly. `migrations_applied` comes from `migration_report.applied_versions` (`fixture.rs:85`) — there is no parallel migration tracker.
- Prelude reuse: `OpaqueId<T>` and `GraphStoreError` are imported from `crate::graphstore::prelude` (`fixture.rs:10`). `validate_fixture_created_refs` calls `OpaqueId::<GraphStoreFixtureRef>::new` (`fixture.rs:105-114`) so every ID flows through the WU-0B-02 invariant.

## Seed validation rolls back partial state

- `create_graphstore_fixture` calls `validate_seed_plan(&seed_plan)?` first (`fixture.rs:69`), before any temp harness or migration call. A non-empty `future_refs` returns `GraphStoreError::UnknownRef` (`fixture.rs:116-122`) without touching the filesystem or the database — so there is no partial state to roll back.
- The contract test enforces this order at the source level: `future_wu_seed_plan_returns_unknown_ref_before_database_creation` reads `fixture.rs` and asserts `validate_seed_plan(&seed_plan)?` appears before `temp_harness_state(` (`graphstore_fixture_contract.rs:122-133`). This locks the invariant against future drift.

## Reset preserves schema_versions exactly

- `reset_graphstore_fixture` runs only a `SELECT COUNT(*) FROM sqlite_master` against the fixture's pool (`fixture.rs:94-103`). It does not delete, truncate, drop, or update `schema_versions`. Because WU-0B-03 owns no Phase 0B domain rows, the function correctly is a no-op for data rows.
- `reset_preserves_schema_versions_history_exactly` (`graphstore_fixture_contract.rs:136-159`) reads the full `schema_versions` row set into `MigrationRecord` instances before and after reset and asserts equality, then reruns `run_migrations` and asserts `applied_versions == []` and `skipped_versions == [1]`, which would fail if the row's `checksum` or `migration_name` were rewritten.

## No `unwrap()` in production code, no TODOs

- `grep -nE "TODO|FIXME|unwrap\(\)" src-tauri/src/graphstore/fixture.rs` returns no matches. The integration test does use `unwrap_or_else` for fixture-file panics, which is appropriate for a test rig.
- Errors from `temp_harness_state`, `harness_app_state`, and `run_migrations` are mapped through `map_err` / `ok_or` / `map_migration_error` to `GraphStoreError` (`fixture.rs:71-77`, `:132-140`).

## Findings

- `WU-0B-03-SHORTCUT-F01` (NIT). `map_migration_error` (`fixture.rs:132-140`) collapses every `MigrationError` variant — `OutOfOrderVersion`, `VersionChecksumMismatch`, `VersionAlreadyAppliedDifferently`, `RollbackUnsupported`, `SqlxFailure` — into `GraphStoreError::SqlxFailure`. The current `GraphStoreError` enum has no migration-specific variants, so this is consistent with WU-0B-02. Future WUs may want a richer mapping; not actionable now.
- `WU-0B-03-SHORTCUT-F02` (NIT). `harness_app_state(&handle).ok_or(GraphStoreError::SqlxFailure)` (`fixture.rs:74`) reports the typed-state-erasure failure as a SQLx failure even though no SQLx call has been made. Acceptable given the limited error vocabulary; not actionable.
