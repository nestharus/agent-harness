use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::event_topic::{EventTopic, EventTopicError};
use agent_harness_lib::events::topic::{
    is_phase_0a_live_topic, parse_event_topic, phase_0a_live_topics, EVENT_TOPICS,
};
use serde::Deserialize;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-05";

#[derive(Debug, Deserialize)]
struct ParseSuccessCase {
    raw: String,
    topic: EventTopic,
}

#[derive(Debug, Deserialize)]
struct ParseErrorCase {
    raw: String,
    error: EventTopicError,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a repository parent")
        .to_path_buf()
}

fn fixture_path(name: &str) -> PathBuf {
    repo_root().join(CONTRACT_FIXTURE_DIR).join(name)
}

fn read_json(path: impl AsRef<Path>) -> Value {
    let content = fs::read_to_string(path.as_ref())
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.as_ref().display()));
    serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.as_ref().display()))
}

fn topic_strings(topics: &[EventTopic]) -> Vec<String> {
    topics
        .iter()
        .map(|topic| {
            serde_json::from_value::<String>(
                serde_json::to_value(topic).expect("topic must serialize"),
            )
            .expect("serialized topic must be a string")
        })
        .collect()
}

#[test]
fn event_topic_taxonomy_matches_canonical_fixture() {
    // Risk: Rust/TS contract drift. Level: particular-integration. Source:
    // proposal test-intent "Rust taxonomy and serde round-trip".
    let fixture_topics: Vec<String> =
        serde_json::from_value(read_json(fixture_path("event-topics.json")))
            .expect("event topics fixture must be a string array");

    assert_eq!(topic_strings(&EVENT_TOPICS), fixture_topics);

    for raw in fixture_topics {
        let topic: EventTopic = serde_json::from_value(Value::String(raw.clone()))
            .expect("documented topic must deserialize");
        assert_eq!(
            serde_json::to_value(topic).expect("topic must serialize"),
            Value::String(raw)
        );
    }

    assert!(
        serde_json::from_value::<EventTopic>(Value::String("unknown".to_string())).is_err(),
        "unknown EventTopic strings must be rejected"
    );
}

#[test]
fn parse_event_topic_accepts_documented_lowercase_topics() {
    // Risk: unknown subscription strings reach event routing. Level: unit.
    // Source: proposal test-intent "Rust parser success/error behavior".
    let cases: Vec<ParseSuccessCase> =
        serde_json::from_value(read_json(fixture_path("parse-success.json")))
            .expect("parse success fixture must deserialize");

    for case in cases {
        assert_eq!(
            parse_event_topic(&case.raw),
            Ok(case.topic),
            "{} should parse to its documented EventTopic",
            case.raw
        );
    }
}

#[test]
fn parse_event_topic_rejects_empty_and_unknown_topics() {
    // Risk: unknown subscription strings reach event routing. Level: unit.
    // Source: proposal test-intent "Rust parser success/error behavior".
    let cases: Vec<ParseErrorCase> =
        serde_json::from_value(read_json(fixture_path("parse-errors.json")))
            .expect("parse errors fixture must deserialize");

    for case in cases {
        assert_eq!(
            parse_event_topic(&case.raw),
            Err(case.error),
            "{:?} should reach its documented EventTopicError",
            case.raw
        );
    }
}

#[test]
fn phase_0a_keeps_placeholders_inert_and_runtime_live() {
    // Risk: Phase 0A accidentally exposes later-domain event payload producers.
    // Level: unit. Source: proposal test-intent "Rust Phase 0A live-topic gating".
    let inert_topics: Vec<EventTopic> =
        serde_json::from_value(read_json(fixture_path("inert-topics.json")))
            .expect("inert topics fixture must deserialize");
    let live_topics: Vec<EventTopic> =
        serde_json::from_value(read_json(fixture_path("phase-0a-live-topics.json")))
            .expect("live topics fixture must deserialize");

    assert_eq!(phase_0a_live_topics(), live_topics.as_slice());
    assert_eq!(live_topics, vec![EventTopic::Runtime]);

    for topic in inert_topics {
        assert!(
            EVENT_TOPICS.contains(&topic),
            "{topic:?} must remain in the documented taxonomy"
        );
        assert!(
            !is_phase_0a_live_topic(topic),
            "{topic:?} must stay inert during Phase 0A"
        );
    }

    assert!(is_phase_0a_live_topic(EventTopic::Runtime));
}
