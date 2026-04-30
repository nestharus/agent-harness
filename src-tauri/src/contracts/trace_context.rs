use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraceContext {
    pub correlation_id: String,
    pub workspace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_invocation_id: Option<String>,
    pub actor: TraceActor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_event_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TraceActor {
    User,
    Orchestrator,
    Worker,
    Optimizer,
    Reviewer,
    Backend,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceContextError {
    EmptyWorkspaceId,
    UnknownActor,
    ParentWithoutInvocation,
}

impl std::fmt::Display for TraceContextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for TraceContextError {}
