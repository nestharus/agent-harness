use serde_json::Value;
use tauri::ipc::Channel;
use tauri::State;

use crate::app_state::HarnessAppState;
use crate::contracts::ipc_event::IpcEvent;
use crate::contracts::subscribe_workspace_events::{
    parse_subscribe_workspace_events_topic, SubscribeWorkspaceEventsError,
};
use crate::events::subscription::WorkspaceEventChannel;

#[tauri::command]
pub fn subscribe_workspace_events(
    workspace_id: String,
    topic: String,
    channel: Channel<IpcEvent<Value>>,
    state: State<'_, HarnessAppState>,
) -> Result<String, SubscribeWorkspaceEventsError> {
    subscribe_workspace_events_with_optional_state(Some(&state), workspace_id, topic, Some(channel))
}

pub fn subscribe_workspace_events_with_state(
    state: &HarnessAppState,
    workspace_id: impl Into<String>,
    topic: impl AsRef<str>,
    channel: Option<WorkspaceEventChannel>,
) -> Result<String, SubscribeWorkspaceEventsError> {
    subscribe_workspace_events_with_optional_state(Some(state), workspace_id, topic, channel)
}

pub fn subscribe_workspace_events_with_optional_state(
    state: Option<&HarnessAppState>,
    workspace_id: impl Into<String>,
    topic: impl AsRef<str>,
    channel: Option<WorkspaceEventChannel>,
) -> Result<String, SubscribeWorkspaceEventsError> {
    let state = state.ok_or(SubscribeWorkspaceEventsError::AppStateUnavailable)?;
    let workspace_id = workspace_id.into();

    if workspace_id.trim().is_empty() {
        return Err(SubscribeWorkspaceEventsError::EmptyWorkspaceId);
    }

    let topic = parse_subscribe_workspace_events_topic(topic.as_ref())?;
    let channel = channel.ok_or(SubscribeWorkspaceEventsError::ChannelUnavailable)?;

    Ok(state
        .event_bus
        .register_subscription(workspace_id, topic, channel))
}
