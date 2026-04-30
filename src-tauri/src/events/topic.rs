use crate::contracts::event_topic::{EventTopic, EventTopicError};

pub const EVENT_TOPICS: [EventTopic; 10] = [
    EventTopic::Graph,
    EventTopic::Render,
    EventTopic::Provider,
    EventTopic::Optimizer,
    EventTopic::Worker,
    EventTopic::Question,
    EventTopic::Recovery,
    EventTopic::Budget,
    EventTopic::Audit,
    EventTopic::Runtime,
];

const PHASE_0A_LIVE_TOPICS: [EventTopic; 1] = [EventTopic::Runtime];

pub fn parse_event_topic(raw: &str) -> Result<EventTopic, EventTopicError> {
    match raw {
        "" => Err(EventTopicError::EmptyTopic),
        "graph" => Ok(EventTopic::Graph),
        "render" => Ok(EventTopic::Render),
        "provider" => Ok(EventTopic::Provider),
        "optimizer" => Ok(EventTopic::Optimizer),
        "worker" => Ok(EventTopic::Worker),
        "question" => Ok(EventTopic::Question),
        "recovery" => Ok(EventTopic::Recovery),
        "budget" => Ok(EventTopic::Budget),
        "audit" => Ok(EventTopic::Audit),
        "runtime" => Ok(EventTopic::Runtime),
        _ => Err(EventTopicError::UnknownTopic),
    }
}

pub fn phase_0a_live_topics() -> &'static [EventTopic] {
    &PHASE_0A_LIVE_TOPICS
}

pub fn is_phase_0a_live_topic(topic: EventTopic) -> bool {
    PHASE_0A_LIVE_TOPICS.contains(&topic)
}
