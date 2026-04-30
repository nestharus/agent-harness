use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventTopic {
    Graph,
    Render,
    Provider,
    Optimizer,
    Worker,
    Question,
    Recovery,
    Budget,
    Audit,
    Runtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventTopicError {
    UnknownTopic,
    EmptyTopic,
}

impl std::fmt::Display for EventTopicError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for EventTopicError {}
