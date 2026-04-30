use crate::contracts::backend_span_event::BackendSpanEvent;
use crate::contracts::event_topic::EventTopic;
use crate::contracts::ipc_event::{IpcEvent, IpcEventError};
use crate::events::ipc_event::build_ipc_event;

pub fn build_backend_span_runtime_event(
    workspace_id: &str,
    payload: BackendSpanEvent,
    trace_context_id: Option<&str>,
) -> Result<IpcEvent<BackendSpanEvent>, IpcEventError> {
    build_ipc_event(workspace_id, EventTopic::Runtime, payload, trace_context_id)
}
