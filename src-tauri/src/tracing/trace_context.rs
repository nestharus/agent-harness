use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::contracts::trace_context::{TraceActor, TraceContext, TraceContextError};

static NEXT_CORRELATION_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn parse_trace_actor(raw: &str) -> Result<TraceActor, TraceContextError> {
    match raw {
        "user" => Ok(TraceActor::User),
        "orchestrator" => Ok(TraceActor::Orchestrator),
        "worker" => Ok(TraceActor::Worker),
        "optimizer" => Ok(TraceActor::Optimizer),
        "reviewer" => Ok(TraceActor::Reviewer),
        "backend" => Ok(TraceActor::Backend),
        "system" => Ok(TraceActor::System),
        _ => Err(TraceContextError::UnknownActor),
    }
}

pub fn create_trace_context(
    workspace_id: &str,
    actor: &str,
    invocation_id: Option<&str>,
    parent_invocation_id: Option<&str>,
    graph_ref: Option<&str>,
    audit_event_ref: Option<&str>,
) -> Result<TraceContext, TraceContextError> {
    if workspace_id.is_empty() {
        return Err(TraceContextError::EmptyWorkspaceId);
    }

    let actor = parse_trace_actor(actor)?;

    if parent_invocation_id.is_some() && invocation_id.is_none() {
        return Err(TraceContextError::ParentWithoutInvocation);
    }

    Ok(TraceContext {
        correlation_id: generate_correlation_id(),
        workspace_id: workspace_id.to_string(),
        invocation_id: invocation_id.map(ToString::to_string),
        parent_invocation_id: parent_invocation_id.map(ToString::to_string),
        actor,
        graph_ref: graph_ref.map(ToString::to_string),
        audit_event_ref: audit_event_ref.map(ToString::to_string),
    })
}

fn generate_correlation_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let counter = NEXT_CORRELATION_COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("trace-{timestamp}-{counter}")
}
