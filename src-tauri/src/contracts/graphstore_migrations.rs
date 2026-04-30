use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, Hash, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum MigrationRollbackState {
    Applied,
    RollbackUnsupported,
    RollbackPlanned,
    RollbackFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(deny_unknown_fields)]
pub struct MigrationRecord {
    pub version: i64,
    pub migration_name: String,
    pub applied_at: String,
    pub checksum: String,
    pub execution_ms: i64,
    pub applied_by: String,
    pub rollback_state: MigrationRollbackState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationReport {
    pub applied_versions: Vec<i64>,
    pub skipped_versions: Vec<i64>,
    pub failed_version: Option<i64>,
    pub checksum_mismatches: Vec<i64>,
    pub rollback_state: MigrationRollbackState,
}

impl Default for MigrationReport {
    fn default() -> Self {
        Self {
            applied_versions: Vec::new(),
            skipped_versions: Vec::new(),
            failed_version: None,
            checksum_mismatches: Vec::new(),
            rollback_state: MigrationRollbackState::Applied,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationError {
    OutOfOrderVersion,
    VersionChecksumMismatch,
    VersionAlreadyAppliedDifferently,
    RollbackUnsupported,
    SqlxFailure,
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for MigrationError {}
