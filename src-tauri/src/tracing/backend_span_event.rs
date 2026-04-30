use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::contracts::backend_span_event::{BackendSpanError, BackendSpanEvent, BackendSpanStatus};
use crate::contracts::trace_context::TraceContext;

static NEXT_SPAN_EVENT_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn emit_backend_span(
    trace_context: TraceContext,
    span_name: &str,
    started_at: &str,
    completed_at: Option<&str>,
    status: BackendSpanStatus,
    error_ref: Option<&str>,
) -> Result<BackendSpanEvent, BackendSpanError> {
    if span_name.is_empty() {
        return Err(BackendSpanError::EmptySpanName);
    }

    validate_trace_context(&trace_context)?;

    let (completed_at, error_ref) = match status {
        BackendSpanStatus::Started => (None, None),
        BackendSpanStatus::Completed => {
            let completed_at = completed_at.ok_or(BackendSpanError::CompletedBeforeStarted)?;
            if timestamp_before(completed_at, started_at) {
                return Err(BackendSpanError::CompletedBeforeStarted);
            }

            (Some(completed_at.to_string()), None)
        }
        BackendSpanStatus::Failed => {
            let error_ref = error_ref.ok_or(BackendSpanError::FailedWithoutErrorRef)?;
            (None, Some(error_ref.to_string()))
        }
    };

    Ok(BackendSpanEvent {
        span_event_id: generate_span_event_id(),
        trace_context,
        span_name: span_name.to_string(),
        started_at: started_at.to_string(),
        completed_at,
        status,
        error_ref,
    })
}

fn validate_trace_context(trace_context: &TraceContext) -> Result<(), BackendSpanError> {
    if trace_context.correlation_id.is_empty()
        || trace_context.workspace_id.is_empty()
        || (trace_context.parent_invocation_id.is_some() && trace_context.invocation_id.is_none())
    {
        return Err(BackendSpanError::TraceContextInvalid);
    }

    Ok(())
}

fn timestamp_before(candidate: &str, baseline: &str) -> bool {
    match (candidate.parse::<u128>(), baseline.parse::<u128>()) {
        (Ok(candidate), Ok(baseline)) => candidate < baseline,
        _ => candidate < baseline,
    }
}

fn generate_span_event_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let counter = NEXT_SPAN_EVENT_COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("span-{timestamp}-{counter}")
}
