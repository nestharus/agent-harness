use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::contracts::event_topic::EventTopic;
use crate::contracts::ipc_event::{IpcEvent, IpcEventError};
use crate::events::topic::parse_event_topic;

static NEXT_EVENT_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn build_ipc_event<T>(
    workspace_id: &str,
    topic: EventTopic,
    payload: T,
    trace_context_id: Option<&str>,
) -> Result<IpcEvent<T>, IpcEventError>
where
    T: Serialize,
{
    build_ipc_event_with_metadata(
        generate_event_id(),
        workspace_id,
        topic,
        payload,
        generate_created_at(),
        trace_context_id,
    )
}

pub fn build_ipc_event_from_raw_topic<T>(
    workspace_id: &str,
    raw_topic: &str,
    payload: T,
    trace_context_id: Option<&str>,
) -> Result<IpcEvent<T>, IpcEventError>
where
    T: Serialize,
{
    let topic = parse_event_topic(raw_topic).map_err(|_| IpcEventError::UnknownTopic)?;
    build_ipc_event(workspace_id, topic, payload, trace_context_id)
}

fn build_ipc_event_with_metadata<T>(
    event_id: String,
    workspace_id: &str,
    topic: EventTopic,
    payload: T,
    created_at: String,
    trace_context_id: Option<&str>,
) -> Result<IpcEvent<T>, IpcEventError>
where
    T: Serialize,
{
    if event_id.is_empty() {
        return Err(IpcEventError::EmptyEventId);
    }

    if workspace_id.is_empty() {
        return Err(IpcEventError::EmptyWorkspaceId);
    }

    serde_json::to_value(&payload).map_err(|_| IpcEventError::PayloadSerializationFailed)?;

    Ok(IpcEvent {
        event_id,
        workspace_id: workspace_id.to_string(),
        topic,
        payload,
        created_at,
        trace_context_id: trace_context_id.map(ToString::to_string),
    })
}

fn generate_event_id() -> String {
    let timestamp = unix_epoch_nanos();
    let counter = NEXT_EVENT_COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("event-{timestamp}-{counter}")
}

fn generate_created_at() -> String {
    unix_epoch_nanos().to_string()
}

fn unix_epoch_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

#[cfg(test)]
mod tests {
    use super::build_ipc_event_with_metadata;
    use crate::contracts::event_topic::EventTopic;
    use crate::contracts::ipc_event::IpcEventError;

    #[test]
    fn empty_generated_event_id_is_reported() {
        let result = build_ipc_event_with_metadata(
            String::new(),
            "workspace-alpha",
            EventTopic::Runtime,
            serde_json::json!({ "kind": "runtime_ping" }),
            "123".to_string(),
            None,
        );

        assert_eq!(result.map(|_| ()), Err(IpcEventError::EmptyEventId));
    }
}
