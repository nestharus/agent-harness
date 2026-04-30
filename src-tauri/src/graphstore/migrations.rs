use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::contracts::graphstore_migrations::{
    MigrationError, MigrationRecord, MigrationReport, MigrationRollbackState,
};

const MIGRATOR_ACTOR: &str = "agent-harness-migrator";

#[derive(Debug, Clone)]
struct MigrationFile {
    version: i64,
    migration_name: String,
    sql: String,
    checksum: String,
}

pub async fn run_migrations(
    pool: &SqlitePool,
    migrations_dir: impl AsRef<Path>,
) -> Result<MigrationReport, MigrationError> {
    let migrations = load_migration_files(migrations_dir.as_ref())?;
    let mut applied = load_applied_records(pool).await?;
    let mut max_applied_version = applied.keys().copied().max().unwrap_or(0);
    let mut report = MigrationReport::default();

    for migration in migrations {
        if let Some(record) = applied.get(&migration.version) {
            if record.migration_name != migration.migration_name {
                return Err(MigrationError::VersionAlreadyAppliedDifferently);
            }
            if record.checksum != migration.checksum {
                return Err(MigrationError::VersionChecksumMismatch);
            }
            report.skipped_versions.push(migration.version);
            continue;
        }

        if migration.version < max_applied_version {
            return Err(MigrationError::OutOfOrderVersion);
        }

        apply_migration(pool, &migration).await?;
        max_applied_version = max_applied_version.max(migration.version);
        report.applied_versions.push(migration.version);
        applied.insert(
            migration.version,
            MigrationRecord {
                version: migration.version,
                migration_name: migration.migration_name,
                applied_at: String::new(),
                checksum: migration.checksum,
                execution_ms: 0,
                applied_by: MIGRATOR_ACTOR.to_string(),
                rollback_state: MigrationRollbackState::Applied,
            },
        );
    }

    Ok(report)
}

pub async fn rollback_migration(pool: &SqlitePool, version: i64) -> Result<(), MigrationError> {
    let _: Option<(i64,)> = sqlx::query_as("SELECT version FROM schema_versions WHERE version = ?")
        .bind(version)
        .fetch_optional(pool)
        .await
        .map_err(|_| MigrationError::SqlxFailure)?;

    Err(MigrationError::RollbackUnsupported)
}

fn load_migration_files(migrations_dir: &Path) -> Result<Vec<MigrationFile>, MigrationError> {
    let mut files = Vec::new();

    for entry in fs::read_dir(migrations_dir).map_err(|_| MigrationError::SqlxFailure)? {
        let entry = entry.map_err(|_| MigrationError::SqlxFailure)?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("sql") {
            continue;
        }
        files.push(parse_migration_file(path)?);
    }

    files.sort_by(|left, right| {
        left.version
            .cmp(&right.version)
            .then_with(|| left.migration_name.cmp(&right.migration_name))
    });

    let mut seen_versions = BTreeSet::new();
    for file in &files {
        if !seen_versions.insert(file.version) {
            return Err(MigrationError::OutOfOrderVersion);
        }
    }

    Ok(files)
}

fn parse_migration_file(path: PathBuf) -> Result<MigrationFile, MigrationError> {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or(MigrationError::OutOfOrderVersion)?;
    let (version_text, migration_name) = stem
        .split_once('_')
        .ok_or(MigrationError::OutOfOrderVersion)?;
    if migration_name.trim().is_empty() {
        return Err(MigrationError::OutOfOrderVersion);
    }

    let version = version_text
        .parse::<i64>()
        .map_err(|_| MigrationError::OutOfOrderVersion)?;
    if version <= 0 {
        return Err(MigrationError::OutOfOrderVersion);
    }

    let bytes = fs::read(&path).map_err(|_| MigrationError::SqlxFailure)?;
    let sql = String::from_utf8(bytes.clone()).map_err(|_| MigrationError::SqlxFailure)?;
    let checksum = checksum_bytes(&bytes);

    Ok(MigrationFile {
        version,
        migration_name: migration_name.to_string(),
        sql,
        checksum,
    })
}

async fn load_applied_records(
    pool: &SqlitePool,
) -> Result<HashMap<i64, MigrationRecord>, MigrationError> {
    if !schema_versions_exists(pool).await? {
        return Ok(HashMap::new());
    }

    let records = sqlx::query_as::<_, MigrationRecord>(
        "SELECT version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state
         FROM schema_versions
         ORDER BY version",
    )
    .fetch_all(pool)
    .await
    .map_err(|_| MigrationError::SqlxFailure)?;

    Ok(records
        .into_iter()
        .map(|record| (record.version, record))
        .collect())
}

async fn schema_versions_exists(pool: &SqlitePool) -> Result<bool, MigrationError> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'schema_versions'",
    )
    .fetch_one(pool)
    .await
    .map_err(|_| MigrationError::SqlxFailure)?;

    Ok(row.0 == 1)
}

async fn apply_migration(
    pool: &SqlitePool,
    migration: &MigrationFile,
) -> Result<(), MigrationError> {
    let started_at = Instant::now();
    let mut transaction = pool
        .begin()
        .await
        .map_err(|_| MigrationError::SqlxFailure)?;

    sqlx::raw_sql(&migration.sql)
        .execute(&mut *transaction)
        .await
        .map_err(|_| MigrationError::SqlxFailure)?;

    let execution_ms = i64::try_from(started_at.elapsed().as_millis()).unwrap_or(i64::MAX);

    sqlx::query(
        "INSERT INTO schema_versions
         (version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state)
         VALUES (?, ?, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), ?, ?, ?, ?)",
    )
    .bind(migration.version)
    .bind(&migration.migration_name)
    .bind(&migration.checksum)
    .bind(execution_ms)
    .bind(MIGRATOR_ACTOR)
    .bind(MigrationRollbackState::Applied)
    .execute(&mut *transaction)
    .await
    .map_err(|_| MigrationError::SqlxFailure)?;

    transaction
        .commit()
        .await
        .map_err(|_| MigrationError::SqlxFailure)
}

fn checksum_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
