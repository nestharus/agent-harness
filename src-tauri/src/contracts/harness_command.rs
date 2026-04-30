use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessCommand {
    GetHarnessSettings,
    SubscribeWorkspaceEvents,
    PingRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarnessCommandError {
    UnknownCommand,
}

pub fn parse_harness_command(raw: &str) -> Result<HarnessCommand, HarnessCommandError> {
    match raw {
        "get_harness_settings" => Ok(HarnessCommand::GetHarnessSettings),
        "subscribe_workspace_events" => Ok(HarnessCommand::SubscribeWorkspaceEvents),
        "ping_runtime" => Ok(HarnessCommand::PingRuntime),
        _ => Err(HarnessCommandError::UnknownCommand),
    }
}

impl std::fmt::Display for HarnessCommandError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for HarnessCommandError {}
