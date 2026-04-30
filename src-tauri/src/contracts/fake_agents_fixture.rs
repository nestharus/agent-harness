use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FakeAgentsFixture {
    pub bin_path: String,
    pub scenario_name: String,
    pub argv_log_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr_ref: Option<String>,
    pub exit_status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oulipoly_invocation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_invocation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub child_acceptance_state: ChildAcceptanceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChildAcceptanceState {
    Unknown,
    Accepted,
    Rejected,
    TimedOut,
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FakeAgentsFixtureError {
    UnknownScenario,
    FixtureInstallFailed,
    ArgvLogMissing,
    TranscriptRefMissing,
    RealAgentsPathRejected,
    InvalidChildAcceptanceState,
}

impl std::fmt::Display for FakeAgentsFixtureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for FakeAgentsFixtureError {}
