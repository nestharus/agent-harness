use serde::{Deserialize, Serialize};

use crate::contracts::event_topic::EventTopic;
use crate::events::topic::parse_event_topic;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubscribeWorkspaceEventsArgs {
    pub workspace_id: String,
    pub topic: EventTopic,
    pub channel: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscribeWorkspaceEventsError {
    EmptyWorkspaceId,
    UnknownTopic,
    ChannelUnavailable,
    AppStateUnavailable,
}

pub fn parse_subscribe_workspace_events_topic(
    raw: &str,
) -> Result<EventTopic, SubscribeWorkspaceEventsError> {
    parse_event_topic(raw).map_err(|_| SubscribeWorkspaceEventsError::UnknownTopic)
}

impl std::fmt::Display for SubscribeWorkspaceEventsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for SubscribeWorkspaceEventsError {}
