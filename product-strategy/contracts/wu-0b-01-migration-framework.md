# WU-0B-01 Contract: SQLite Migration Framework and schema_versions Tracking

## API

```text
run_migrations(pool: &SqlitePool, migrations_dir: impl AsRef<Path>) -> Result<MigrationReport, MigrationError>
rollback_migration(pool: &SqlitePool, version: i64) -> Result<(), MigrationError>
```

`run_migrations` reads `*.sql` files from `migrations_dir`, applies them in version order, and records each successful migration in `schema_versions`. `rollback_migration` is present only for the documented unsupported rollback behavior and must not delete or rewrite migration history.

## Migration File Contract

Files use `NNNN_<name>.sql` naming. The numeric prefix is parsed as `i64 version`; the suffix without `.sql` is `migration_name`.

Invalid, duplicate, non-positive, or non-monotonic versions fail with `MigrationError::OutOfOrderVersion` before any unapplied migration from that directory is executed.

## schema_versions

```sql
CREATE TABLE IF NOT EXISTS schema_versions (
  version INTEGER PRIMARY KEY,
  migration_name TEXT NOT NULL,
  applied_at TEXT NOT NULL,
  checksum TEXT NOT NULL,
  execution_ms INTEGER NOT NULL,
  applied_by TEXT NOT NULL,
  rollback_state TEXT NOT NULL CHECK (
    rollback_state IN (
      'applied',
      'rollback_unsupported',
      'rollback_planned',
      'rollback_failed'
    )
  )
);
```

`applied_at` is UTC timestamp text. `checksum` is SHA-256 over exact file contents. `applied_by` is `agent-harness-migrator`.

## DTOs

```text
MigrationRecord {
  version: i64,
  migration_name: String,
  applied_at: String,
  checksum: String,
  execution_ms: i64,
  applied_by: String,
  rollback_state: MigrationRollbackState
}

MigrationRollbackState =
  applied
  | rollback_unsupported
  | rollback_planned
  | rollback_failed

MigrationReport {
  applied_versions: Vec<i64>,
  skipped_versions: Vec<i64>,
  failed_version: Option<i64>,
  checksum_mismatches: Vec<i64>,
  rollback_state: MigrationRollbackState
}

MigrationError =
  OutOfOrderVersion
  | VersionChecksumMismatch
  | VersionAlreadyAppliedDifferently
  | RollbackUnsupported
  | SqlxFailure
```

Serde field names are snake_case. `MigrationRollbackState` serializes as its lowercase contract token. `MigrationError` serializes as the Rust variant name. Unknown rollback-state strings are rejected by serde and by SQL row decoding.

## Behavioral Requirements

1. Fresh databases apply all ordered migrations and record one `schema_versions` row per file.
2. Re-running identical files is idempotent and does not mutate existing rows.
3. Same version and name with a changed checksum returns `VersionChecksumMismatch` and leaves rows unchanged.
4. Same version with a different applied name returns `VersionAlreadyAppliedDifferently` and leaves rows unchanged.
5. Duplicate or out-of-order versions return `OutOfOrderVersion` before later files are applied.
6. Unsupported rollback returns `RollbackUnsupported` and never deletes or rewrites rows.
7. Applying the shipped WU-0B-01 migration directory creates no durable table except `schema_versions`.

## Fixtures

- `schema-versions-canonical-row.json`
- `migration-record-round-trip.json`
- `rollback-states.json`
- `migration-report-canonical.json`
- `migration-report-variants.json`
- `error-out-of-order-version.json`
- `error-version-checksum-mismatch.json`
- `error-version-already-applied-differently.json`
- `error-rollback-unsupported.json`
- `error-sqlx-failure.json`
- `fresh-db-applies-n-manifest.json`
