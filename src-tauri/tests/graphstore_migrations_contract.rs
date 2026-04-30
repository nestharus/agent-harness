use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use agent_harness_lib::contracts::graphstore_migrations::{
    MigrationError, MigrationRecord, MigrationReport, MigrationRollbackState,
};
use agent_harness_lib::graphstore::migrations::{rollback_migration, run_migrations};
use agent_harness_lib::test_harness::temp_harness::{harness_app_state, temp_harness_state};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-01";
const MIGRATOR_ACTOR: &str = "agent-harness-migrator";

static NEXT_MIGRATION_DIR: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Deserialize)]
struct FreshManifestFixture {
    applied_by: String,
    expected_rollback_state: MigrationRollbackState,
    migrations: Vec<MigrationFixture>,
}

#[derive(Debug, Deserialize)]
struct MigrationFixture {
    version: Option<i64>,
    migration_name: Option<String>,
    filename: String,
    sql: String,
}

#[derive(Debug, Deserialize)]
struct ErrorFixture {
    expected_error: MigrationError,
    migrations: Option<Vec<MigrationFixture>>,
    initial_sql: Option<String>,
    mutated_sql: Option<String>,
    conflicting_filename: Option<String>,
    conflicting_sql: Option<String>,
    version: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct RollbackStatesFixture {
    valid: Vec<MigrationRollbackState>,
    invalid: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a repository parent")
        .to_path_buf()
}

fn fixture_path(name: &str) -> PathBuf {
    repo_root().join(CONTRACT_FIXTURE_DIR).join(name)
}

fn read_json(path: impl AsRef<Path>) -> Value {
    let content = fs::read_to_string(path.as_ref())
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.as_ref().display()));
    serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.as_ref().display()))
}

fn read_fixture<T: for<'de> Deserialize<'de>>(name: &str) -> T {
    serde_json::from_value(read_json(fixture_path(name)))
        .unwrap_or_else(|error| panic!("fixture {name} must parse: {error}"))
}

fn migration_dir(name: &str) -> PathBuf {
    let counter = NEXT_MIGRATION_DIR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "agent-harness-wu-0b-01-{name}-{}-{counter}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("stale migration fixture directory should be removable");
    }
    fs::create_dir_all(&path).expect("migration fixture directory should be creatable");
    path
}

fn write_migration(dir: &Path, filename: &str, sql: &str) {
    fs::write(dir.join(filename), sql).expect("migration fixture file should be writable");
}

fn write_migrations(dir: &Path, migrations: &[MigrationFixture]) {
    for migration in migrations {
        write_migration(dir, &migration.filename, &migration.sql);
    }
}

async fn temp_pool() -> SqlitePool {
    let handle = temp_harness_state("empty", None, None, Vec::new())
        .await
        .expect("empty temp harness should initialize");
    harness_app_state(&handle)
        .expect("temp harness handle should resolve app state")
        .db
}

async fn migration_records(pool: &SqlitePool) -> Vec<MigrationRecord> {
    sqlx::query_as::<_, MigrationRecord>(
        "SELECT version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state
         FROM schema_versions
         ORDER BY version",
    )
    .fetch_all(pool)
    .await
    .expect("schema_versions rows should be readable")
}

async fn sqlite_table_names(pool: &SqlitePool) -> Vec<String> {
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .fetch_all(pool)
            .await
            .expect("sqlite_master should be readable");
    rows.into_iter().map(|(name,)| name).collect()
}

async fn schema_versions_exists(pool: &SqlitePool) -> bool {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'schema_versions'",
    )
    .fetch_one(pool)
    .await
    .expect("sqlite_master should be readable");
    count.0 == 1
}

fn checksum(sql: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sql.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn schema_versions_migration() -> MigrationFixture {
    let manifest: FreshManifestFixture = read_fixture("fresh-db-applies-n-manifest.json");
    manifest
        .migrations
        .into_iter()
        .find(|migration| migration.version == Some(1))
        .expect("fresh manifest must include schema_versions migration")
}

#[tokio::test]
async fn fresh_temp_sqlite_applies_ordered_migrations_and_records_schema_versions() {
    // Risk: bootstrap or recording drift. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Fresh ordered apply".
    let fixture: FreshManifestFixture = read_fixture("fresh-db-applies-n-manifest.json");
    let dir = migration_dir("fresh");
    write_migrations(&dir, &fixture.migrations);
    let pool = temp_pool().await;

    let report = run_migrations(&pool, &dir)
        .await
        .expect("fresh migrations should apply");

    let expected_versions: Vec<i64> = fixture
        .migrations
        .iter()
        .map(|migration| migration.version.expect("fixture versions are required"))
        .collect();
    assert_eq!(report.applied_versions, expected_versions);
    assert_eq!(report.skipped_versions, Vec::<i64>::new());
    assert_eq!(report.failed_version, None);
    assert_eq!(report.checksum_mismatches, Vec::<i64>::new());
    assert_eq!(report.rollback_state, fixture.expected_rollback_state);

    let rows = migration_records(&pool).await;
    assert_eq!(rows.len(), fixture.migrations.len());
    for (row, expected) in rows.iter().zip(fixture.migrations.iter()) {
        assert_eq!(row.version, expected.version.expect("fixture version"));
        assert_eq!(
            row.migration_name.as_str(),
            expected
                .migration_name
                .as_ref()
                .expect("fixture migration name")
                .as_str()
        );
        assert_eq!(row.checksum, checksum(&expected.sql));
        assert!(row.execution_ms >= 0);
        assert_eq!(row.applied_by, fixture.applied_by);
        assert_eq!(row.rollback_state, fixture.expected_rollback_state);
        assert!(!row.applied_at.trim().is_empty());
    }
}

#[tokio::test]
async fn rerunning_identical_migrations_is_idempotent_and_does_not_rewrite_rows() {
    // Risk: migration history rewrite. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Idempotent rerun".
    let fixture: FreshManifestFixture = read_fixture("fresh-db-applies-n-manifest.json");
    let dir = migration_dir("idempotent");
    write_migrations(&dir, &fixture.migrations);
    let pool = temp_pool().await;

    run_migrations(&pool, &dir)
        .await
        .expect("first run should apply");
    let before = migration_records(&pool).await;
    let report = run_migrations(&pool, &dir)
        .await
        .expect("second run should skip");
    let after = migration_records(&pool).await;

    assert_eq!(report.applied_versions, Vec::<i64>::new());
    assert_eq!(report.skipped_versions, vec![1, 2, 3]);
    assert_eq!(before, after);
}

#[tokio::test]
async fn changed_applied_migration_returns_checksum_mismatch_without_rewriting_rows() {
    // Risk: silent drift. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Checksum mismatch".
    let fixture: ErrorFixture = read_fixture("error-version-checksum-mismatch.json");
    let dir = migration_dir("checksum");
    let bootstrap = schema_versions_migration();
    write_migration(&dir, &bootstrap.filename, &bootstrap.sql);
    write_migration(
        &dir,
        "0002_mutable_contract_noop.sql",
        fixture.initial_sql.as_deref().expect("initial SQL fixture"),
    );
    let pool = temp_pool().await;
    run_migrations(&pool, &dir)
        .await
        .expect("initial migrations should apply");
    let before = migration_records(&pool).await;

    write_migration(
        &dir,
        "0002_mutable_contract_noop.sql",
        fixture.mutated_sql.as_deref().expect("mutated SQL fixture"),
    );
    let error = run_migrations(&pool, &dir)
        .await
        .expect_err("mutated migration should be rejected");
    let after = migration_records(&pool).await;

    assert_eq!(error, fixture.expected_error);
    assert_eq!(error, MigrationError::VersionChecksumMismatch);
    assert_eq!(before, after);
}

#[tokio::test]
async fn duplicate_versions_are_rejected_before_later_migrations_apply() {
    // Risk: ambiguous version history. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Duplicate/out-of-order rejection".
    let fixture: ErrorFixture = read_fixture("error-out-of-order-version.json");
    let dir = migration_dir("duplicate");
    write_migrations(
        &dir,
        fixture
            .migrations
            .as_ref()
            .expect("duplicate fixture migrations"),
    );
    let pool = temp_pool().await;

    let error = run_migrations(&pool, &dir)
        .await
        .expect_err("duplicate versions should be rejected");

    assert_eq!(error, fixture.expected_error);
    assert_eq!(error, MigrationError::OutOfOrderVersion);
    assert!(
        !schema_versions_exists(&pool).await,
        "validation must fail before bootstrap migration applies"
    );
}

#[tokio::test]
async fn recursive_migration_walk_rejects_duplicate_versions_across_subdirectories() {
    // Risk: recursive migration discovery weakens WU-0B-01 version uniqueness.
    // Level: particular-integration. Source: WU-0B-04 migration directory
    // decision, option 1.
    let pool = temp_pool().await;
    let dir = migration_dir("recursive-duplicate");
    let nested = dir.join("0b");
    fs::create_dir_all(&nested).expect("nested migration directory should be creatable");
    let bootstrap = schema_versions_migration();

    write_migration(&dir, &bootstrap.filename, &bootstrap.sql);
    write_migration(
        &nested,
        "0001_duplicate_schema_versions.sql",
        "CREATE TABLE should_not_apply (id INTEGER PRIMARY KEY);\n",
    );

    let error = run_migrations(&pool, &dir)
        .await
        .expect_err("duplicate versions across the recursive tree should be rejected");

    assert_eq!(error, MigrationError::OutOfOrderVersion);
    assert!(
        !schema_versions_exists(&pool).await,
        "duplicate recursive manifests must not partially apply"
    );
}

#[tokio::test]
async fn lower_missing_version_after_higher_applied_version_is_out_of_order() {
    // Risk: ambiguous version history. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Duplicate/out-of-order rejection".
    let dir = migration_dir("stale-lower");
    let bootstrap = schema_versions_migration();
    write_migration(&dir, &bootstrap.filename, &bootstrap.sql);
    let pool = temp_pool().await;
    run_migrations(&pool, &dir)
        .await
        .expect("bootstrap should apply first");

    sqlx::query(
        "INSERT INTO schema_versions
         (version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state)
         VALUES (3, 'synthetic_higher_version', '2026-04-30T00:00:00.000Z', ?, 0, ?, ?)",
    )
    .bind(format!("{:064x}", 3))
    .bind(MIGRATOR_ACTOR)
    .bind(MigrationRollbackState::Applied)
    .execute(&pool)
    .await
    .expect("synthetic higher version should insert");

    let before = migration_records(&pool).await;
    write_migration(&dir, "0002_late_missing_version.sql", "SELECT 22;\n");
    let error = run_migrations(&pool, &dir)
        .await
        .expect_err("lower missing version after a higher applied version should be rejected");
    let after = migration_records(&pool).await;

    assert_eq!(error, MigrationError::OutOfOrderVersion);
    assert_eq!(before, after);
}

#[tokio::test]
async fn already_applied_version_with_different_name_is_rejected_exactly() {
    // Risk: ambiguous version history. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Duplicate/out-of-order rejection".
    let fixture: ErrorFixture = read_fixture("error-version-already-applied-differently.json");
    let dir = migration_dir("already-different");
    let bootstrap = schema_versions_migration();
    write_migration(&dir, &bootstrap.filename, &bootstrap.sql);
    let pool = temp_pool().await;
    run_migrations(&pool, &dir)
        .await
        .expect("bootstrap migration should apply");
    let before = migration_records(&pool).await;

    fs::remove_file(dir.join(&bootstrap.filename)).expect("bootstrap fixture should be removable");
    write_migration(
        &dir,
        fixture
            .conflicting_filename
            .as_deref()
            .expect("conflicting filename fixture"),
        fixture
            .conflicting_sql
            .as_deref()
            .expect("conflicting sql fixture"),
    );
    let error = run_migrations(&pool, &dir)
        .await
        .expect_err("same version with a different name should be rejected");
    let after = migration_records(&pool).await;

    assert_eq!(error, fixture.expected_error);
    assert_eq!(error, MigrationError::VersionAlreadyAppliedDifferently);
    assert_eq!(before, after);
}

#[tokio::test]
async fn unsupported_rollback_returns_exact_error_and_preserves_history() {
    // Risk: destructive history changes. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Unsupported rollback".
    let fixture: ErrorFixture = read_fixture("error-rollback-unsupported.json");
    let dir = migration_dir("rollback");
    let bootstrap = schema_versions_migration();
    write_migration(&dir, &bootstrap.filename, &bootstrap.sql);
    let pool = temp_pool().await;
    run_migrations(&pool, &dir)
        .await
        .expect("bootstrap migration should apply");
    let before = migration_records(&pool).await;

    let error = rollback_migration(
        &pool,
        fixture
            .version
            .expect("rollback fixture must name a version"),
    )
    .await
    .expect_err("WU-0B-01 rollback should be unsupported");
    let after = migration_records(&pool).await;

    assert_eq!(error, fixture.expected_error);
    assert_eq!(error, MigrationError::RollbackUnsupported);
    assert_eq!(before, after);
}

#[tokio::test]
async fn rollback_states_round_trip_through_serde_and_sqlx_and_unknown_is_rejected() {
    // Risk: enum drift or permissive decoding. Level: particular-integration.
    // Source: WU-0B-01 proposal test intent "Rollback state round-trip".
    let fixture: RollbackStatesFixture = read_fixture("rollback-states.json");
    let dir = migration_dir("states");
    let bootstrap = schema_versions_migration();
    write_migration(&dir, &bootstrap.filename, &bootstrap.sql);
    let pool = temp_pool().await;
    run_migrations(&pool, &dir)
        .await
        .expect("bootstrap migration should apply");

    for (index, state) in fixture.valid.iter().copied().enumerate() {
        let value = serde_json::to_value(state).expect("rollback state must serialize");
        let parsed: MigrationRollbackState =
            serde_json::from_value(value).expect("rollback state must deserialize");
        assert_eq!(parsed, state);

        let version = 100 + index as i64;
        sqlx::query(
            "INSERT INTO schema_versions
             (version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state)
             VALUES (?, ?, '2026-04-30T00:00:00.000Z', ?, 0, ?, ?)",
        )
        .bind(version)
        .bind(format!("state_{version}"))
        .bind(format!("{version:064x}"))
        .bind(MIGRATOR_ACTOR)
        .bind(state)
        .execute(&pool)
        .await
        .expect("valid rollback state should insert through sqlx");

        let row: MigrationRecord = sqlx::query_as(
            "SELECT version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state
             FROM schema_versions
             WHERE version = ?",
        )
        .bind(version)
        .fetch_one(&pool)
        .await
        .expect("valid rollback state should decode through sqlx");
        assert_eq!(row.rollback_state, state);
    }

    assert!(
        serde_json::from_value::<MigrationRollbackState>(Value::String(fixture.invalid.clone()))
            .is_err(),
        "unknown rollback states must be rejected by serde"
    );
    let sqlx_error = sqlx::query_as::<_, (MigrationRollbackState,)>("SELECT ?")
        .bind(fixture.invalid)
        .fetch_one(&pool)
        .await;
    assert!(
        sqlx_error.is_err(),
        "unknown rollback states must be rejected by sqlx decoding"
    );
}

#[tokio::test]
async fn sqlx_failure_variant_is_produced_by_named_fixture() {
    // Risk: caller-visible error drift. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Error taxonomy".
    let fixture: ErrorFixture = read_fixture("error-sqlx-failure.json");
    let dir = migration_dir("sqlx-failure");
    write_migrations(
        &dir,
        fixture
            .migrations
            .as_ref()
            .expect("sqlx failure fixture migrations"),
    );
    let pool = temp_pool().await;

    let error = run_migrations(&pool, &dir)
        .await
        .expect_err("invalid migration SQL should fail");

    assert_eq!(error, fixture.expected_error);
    assert_eq!(error, MigrationError::SqlxFailure);
}

#[test]
fn migration_record_report_and_error_fixtures_round_trip_stable_shapes() {
    // Risk: Rust contract drift. Level: particular-integration. Source:
    // WU-0B-01 proposal test intent "Error taxonomy" and DTO fixture shape.
    let record_fixture = read_json(fixture_path("migration-record-round-trip.json"));
    let record: MigrationRecord =
        serde_json::from_value(record_fixture.clone()).expect("MigrationRecord fixture must parse");
    assert_eq!(
        serde_json::to_value(record).expect("MigrationRecord must serialize"),
        record_fixture
    );
    let canonical_row_fixture = read_json(fixture_path("schema-versions-canonical-row.json"));
    let canonical_row: MigrationRecord = serde_json::from_value(canonical_row_fixture.clone())
        .expect("schema_versions canonical row fixture must parse");
    assert_eq!(
        serde_json::to_value(canonical_row).expect("schema_versions row must serialize"),
        canonical_row_fixture
    );

    let report_fixture = read_json(fixture_path("migration-report-canonical.json"));
    let report: MigrationReport =
        serde_json::from_value(report_fixture.clone()).expect("MigrationReport fixture must parse");
    assert_eq!(
        serde_json::to_value(report).expect("MigrationReport must serialize"),
        report_fixture
    );

    let variant_reports: Vec<MigrationReport> =
        serde_json::from_value(read_json(fixture_path("migration-report-variants.json")))
            .expect("MigrationReport variants fixture must parse");
    assert_eq!(variant_reports.len(), 4);
    assert_eq!(
        variant_reports
            .iter()
            .map(|report| report.rollback_state)
            .collect::<Vec<_>>(),
        vec![
            MigrationRollbackState::Applied,
            MigrationRollbackState::RollbackUnsupported,
            MigrationRollbackState::RollbackPlanned,
            MigrationRollbackState::RollbackFailed,
        ]
    );

    let fixture_names = [
        "error-out-of-order-version.json",
        "error-version-checksum-mismatch.json",
        "error-version-already-applied-differently.json",
        "error-rollback-unsupported.json",
        "error-sqlx-failure.json",
    ];
    let expected_errors: Vec<MigrationError> = fixture_names
        .into_iter()
        .map(|name| read_fixture::<ErrorFixture>(name).expected_error)
        .collect();
    assert_eq!(
        expected_errors,
        vec![
            MigrationError::OutOfOrderVersion,
            MigrationError::VersionChecksumMismatch,
            MigrationError::VersionAlreadyAppliedDifferently,
            MigrationError::RollbackUnsupported,
            MigrationError::SqlxFailure,
        ]
    );
}

#[tokio::test]
async fn shipped_phase_0b_migrations_emit_phase_0b_tables() {
    // Risk: table ownership leakage. Level: particular-integration. Source:
    // WU-0B-01 migration ownership plus WU-0B-04 first domain table contract.
    let pool = temp_pool().await;
    let migrations_dir = repo_root().join("src-tauri/migrations");

    let report = run_migrations(&pool, migrations_dir)
        .await
        .expect("shipped Phase 0B migrations should apply");
    let tables = sqlite_table_names(&pool).await;
    let durable_tables: Vec<String> = tables
        .into_iter()
        .filter(|table| !table.starts_with("sqlite_"))
        .collect();

    assert_eq!(report.applied_versions, vec![1, 4, 5, 6, 7, 9, 15]);
    assert_eq!(
        durable_tables,
        vec![
            "audit_events".to_string(),
            "evidence_artifacts".to_string(),
            "graph_configurations".to_string(),
            "graph_nodes".to_string(),
            "graph_workspaces".to_string(),
            "policy_sets".to_string(),
            "schema_versions".to_string()
        ]
    );
}
