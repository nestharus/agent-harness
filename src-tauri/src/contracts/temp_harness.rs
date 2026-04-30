use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TempHarnessHandle {
    pub workspace_id: String,
    pub database_path: String,
    pub fixture_manifest_id: String,
    pub app_state_ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TempHarnessError {
    UnknownSeed,
    DatabaseCreateFailed,
    AppStateInitFailed,
    RealAgentsInvocationAttempted,
    FixtureManifestMissing,
}

impl std::fmt::Display for TempHarnessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for TempHarnessError {}
