use serde::{Deserialize, Serialize};

use crate::contracts::event_topic::EventTopic;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpcEvent<T> {
    pub event_id: String,
    pub workspace_id: String,
    pub topic: EventTopic,
    pub payload: T,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_context_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpcEventError {
    EmptyEventId,
    EmptyWorkspaceId,
    UnknownTopic,
    PayloadSerializationFailed,
}

impl std::fmt::Display for IpcEventError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for IpcEventError {}
