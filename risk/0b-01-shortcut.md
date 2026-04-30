# WU-0B-01 Shortcut / Placeholder Gate

**Severity:** LOW

## Verdict

The migration runner is real: it parses `migrations_dir` with `fs::read_dir`, validates filenames, computes SHA-256 over exact file bytes via the `sha2` crate, applies SQL inside a `sqlx` transaction with `pool.begin()` / `transaction.commit()`, records `applied_at` from SQLite's own `strftime('%Y-%m-%dT%H:%M:%fZ','now')`, and times `execution_ms` from `Instant::now()`. Idempotency, checksum-mismatch detection, and out-of-order rejection are all implemented and asserted with snapshot-equality on the row set, not by string-matching error messages. There are no `unwrap()` on `Result`, no `panic!`, no `todo!`, no `unimplemented!`, and no TODO/FIXME/placeholder markers in the new source. Two findings, both nit-level.

## Per-Item Findings

### WU-0B-01-SHORTCUT-F01 — Real directory parse + real SHA-256 (PASS)

`load_migration_files` (`src-tauri/src/graphstore/migrations.rs:78-104`) actually iterates `fs::read_dir(migrations_dir)`, filters `.sql` files, parses `NNNN_<name>.sql` via `path.file_stem()` + `split_once('_')`, parses the prefix as `i64`, rejects non-positive versions, requires a non-empty migration name, and reads bytes via `fs::read`. Checksum (`checksum_bytes`, `:208-212`) uses `Sha256::new()` / `hasher.update(bytes)` / hex-format finalize. The fresh test asserts `row.checksum == checksum(&expected.sql)` where `checksum` is computed independently in the test (`src-tauri/tests/graphstore_migrations_contract.rs:137-141`) — i.e. the test verifies the runner's checksum value is the genuine SHA-256 of the file bytes, not an opaque token.

### WU-0B-01-SHORTCUT-F02 — Real `applied_at` and `execution_ms` (PASS)

`apply_migration` (`src-tauri/src/graphstore/migrations.rs:170-206`) records `applied_at` from `strftime('%Y-%m-%dT%H:%M:%fZ', 'now')` inside the same transaction that ran the DDL, and `execution_ms` from `started_at.elapsed().as_millis()` cast to `i64` (with `unwrap_or(i64::MAX)` only as a saturating fallback for the impossible >292M-year case). The fresh test asserts `!row.applied_at.trim().is_empty()` and `row.execution_ms >= 0` (`graphstore_migrations_contract.rs:189-192`) — both are real recorded values, not stubbed.

### WU-0B-01-SHORTCUT-F03 — Idempotency is asserted by row-set snapshot equality, not just by report shape (PASS)

`rerunning_identical_migrations_is_idempotent_and_does_not_rewrite_rows` (`graphstore_migrations_contract.rs:196-217`) snapshots `migration_records(&pool)` before the second run, then snapshots again after, and asserts `before == after`. Because `MigrationRecord` derives `PartialEq` over all seven fields including `applied_at` and `execution_ms`, this catches any silent rewrite — a runner that did `UPSERT ... applied_at = strftime(now)` would fail this test. The runner's iteration logic at `migrations.rs:32-42` also short-circuits on `applied.contains_key`, so no INSERT is even attempted for already-applied versions. PASS.

### WU-0B-01-SHORTCUT-F04 — Checksum-mismatch returns the typed error and leaves the schema unchanged (PASS)

`changed_applied_migration_returns_checksum_mismatch_without_rewriting_rows` (`graphstore_migrations_contract.rs:220-251`) applies the bootstrap + a `0002_mutable_contract_noop.sql`, snapshots the rows, mutates the file contents on disk, re-runs, and asserts `error == MigrationError::VersionChecksumMismatch` AND `before == after`. The runner check is at `migrations.rs:33-39`: after looking up an existing applied record, it compares both `migration_name` and `checksum` and returns the typed variant on mismatch *before* `apply_migration` is called — so no transaction is opened and no row can be rewritten. PASS.

### WU-0B-01-SHORTCUT-F05 — Out-of-order rejection happens before any later migration is applied (PASS)

Two complementary tests cover the two failure shapes:

- **Duplicate version in the directory:** `duplicate_versions_are_rejected_before_later_migrations_apply` (`graphstore_migrations_contract.rs:253-278`) ships three files including two with version `0001`. After `expect_err`, it asserts `!schema_versions_exists(&pool).await` — i.e. even the bootstrap `0001_schema_versions.sql` was not applied, because the duplicate is detected by the `BTreeSet::insert` collision in `load_migration_files` (`migrations.rs:96-101`) **before** the iteration loop calls `apply_migration` for anything.
- **Lower missing version added after a higher version is recorded:** `lower_missing_version_after_higher_applied_version_is_out_of_order` (`graphstore_migrations_contract.rs:281-313`) bootstraps v=1, manually inserts a synthetic v=3 row, then drops a v=2 file into the directory and re-runs. The runner's `max_applied_version = applied.keys().copied().max().unwrap_or(0)` (`migrations.rs:29`) plus the `if migration.version < max_applied_version { return Err(...) }` guard (`:44-46`) trips before `apply_migration` is called for v=2. The test asserts row snapshot is unchanged.

PASS — out-of-order rejection is structural, not "best-effort".

### WU-0B-01-SHORTCUT-F06 — Rollback variants round-trip and unknown is rejected (PASS)

`rollback_states_round_trip_through_serde_and_sqlx_and_unknown_is_rejected` (`graphstore_migrations_contract.rs:380-439`) iterates all four valid variants from `rollback-states.json`, asserts `serde_json::to_value` → `serde_json::from_value` round-trips, then performs a real sqlx INSERT and a real sqlx FromRow SELECT for each variant. The test then attempts both `serde_json::from_value::<MigrationRollbackState>(Value::String("rollback_unknown"))` and `sqlx::query_as::<_, (MigrationRollbackState,)>("SELECT ?")` with the invalid token, and asserts both fail. The contract's `#[sqlx(type_name = "TEXT", rename_all = "snake_case")]` and `#[serde(rename_all = "snake_case")]` derives are what enforce the rejection — no permissive fallback. PASS.

### WU-0B-01-SHORTCUT-F07 — No `unwrap()` / `panic!` / `todo!` in production paths (PASS)

`grep -n 'unwrap|panic!|todo!'` against `src-tauri/src/graphstore/migrations.rs` returns exactly two hits, both `unwrap_or(...)` infallible defaults:

- `migrations.rs:29 — applied.keys().copied().max().unwrap_or(0)` — empty `HashMap.keys().max()` legitimately returns `None`; defaulting to 0 is the documented "no applied migrations yet" meaning.
- `migrations.rs:185 — i64::try_from(started_at.elapsed().as_millis()).unwrap_or(i64::MAX)` — saturating cast for the impossible-in-practice >2^63-millisecond window.

Both `MigrationError`-returning paths use `?` over `Result` and `map_err(|_| MigrationError::SqlxFailure)` for IO/SQL failures. There are no `Result::unwrap` calls on production paths. PASS.

### WU-0B-01-SHORTCUT-F08 — No TODO / FIXME / placeholder markers (PASS)

`grep -i 'TODO|FIXME|todo!|unimplemented!|placeholder'` over `src-tauri/src/graphstore/` returns no matches.

### WU-0B-01-SHORTCUT-N01 — Nit: error info from `fs` and SQL errors is collapsed via `map_err(|_| MigrationError::SqlxFailure)` (NIT)

`migrations.rs:81, 82, 125, 126, 151, 165, 178, 183, 200, 205` all collapse the underlying `sqlx::Error` / `io::Error` / `FromUtf8Error` into the bare `MigrationError::SqlxFailure` discriminant, dropping the cause string. The contract enumerates only five error variants and contains no `IoError` or `ParseError`, so widening the taxonomy is out of scope here, but a bare `SqlxFailure` discards information that would be useful in production triage. The proposal explicitly accepts this trade-off in §"Anti-Scope" / hand-rolled justification, and the contract round-trip asserts `MigrationError` serializes only as the variant name — so adding a `source: String` field would itself be a contract change. Flagging as NIT only; no action required for WU-0B-01. Future WUs that consume this runner may want to enrich the error type when the contract evolves.

### WU-0B-01-SHORTCUT-N02 — Nit: `rollback_migration` performs an unused SELECT before returning `RollbackUnsupported` (NIT)

`rollback_migration` (`migrations.rs:68-76`) executes `SELECT version FROM schema_versions WHERE version = ?` and discards the result, then returns `RollbackUnsupported`. The SELECT is presumably there so that a missing `schema_versions` table surfaces as `SqlxFailure` rather than `RollbackUnsupported`, but the test (`unsupported_rollback_returns_exact_error_and_preserves_history`) only exercises the path *after* the bootstrap has run — it never asserts the table-missing branch. So the SELECT is currently load-bearing only for an untested corner. Either keep the SELECT and add a fixture for that branch, or drop it. Flagging as NIT — the variant is documented as unsupported and the rest of the contract is honoured.

## Conclusion

Severity **LOW**. The runner is genuine, idempotent, checksum-mismatch detection and out-of-order rejection both fire before any later migration is applied, and the assertions verify real recorded values rather than placeholder fields. Two nits, neither of which crosses the MEDIUM threshold (no stub error path; the runner's logic is real).
