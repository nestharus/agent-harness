use serde::{Deserialize, Serialize};

use crate::contracts::trace_context::TraceContext;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackendSpanEvent {
    pub span_event_id: String,
    pub trace_context: TraceContext,
    pub span_name: String,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    pub status: BackendSpanStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendSpanStatus {
    Started,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendSpanError {
    EmptySpanName,
    CompletedBeforeStarted,
    FailedWithoutErrorRef,
    TraceContextInvalid,
}

impl std::fmt::Display for BackendSpanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for BackendSpanError {}
