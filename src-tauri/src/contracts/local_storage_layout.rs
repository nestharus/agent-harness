use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalStorageLayout {
    pub storage_root: String,
    pub database_path: String,
    pub evidence_root: String,
    pub fixture_root: String,
    pub log_root: String,
    pub temp_root: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageLayoutError {
    StorageRootEscapesWorkspace,
    DatabasePathOutsideStorageRoot,
    EvidenceRootOutsideStorageRoot,
    FixtureRootOutsideStorageRoot,
    LogRootOutsideStorageRoot,
    TempRootOutsideStorageRoot,
}

impl std::fmt::Display for StorageLayoutError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for StorageLayoutError {}
