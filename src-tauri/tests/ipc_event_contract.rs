use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::event_topic::EventTopic;
use agent_harness_lib::contracts::ipc_event::{IpcEvent, IpcEventError};
use agent_harness_lib::events::ipc_event::{build_ipc_event, build_ipc_event_from_raw_topic};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-06";

#[derive(Debug, Deserialize)]
struct BuildInput {
    workspace_id: String,
    topic: Option<EventTopic>,
    raw_topic: Option<String>,
    payload: Option<Value>,
    trace_context_id: Option<String>,
    payload_fixture: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BuildSuccessCase {
    input: BuildInput,
    expected: Value,
}

#[derive(Debug, Deserialize)]
struct BuildErrorCase {
    name: String,
    input: BuildInput,
    expected_error: IpcEventError,
}

struct NonSerializablePayload;

impl Serialize for NonSerializablePayload {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(serde::ser::Error::custom(
            "contract fixture payload cannot serialize",
        ))
    }
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

#[test]
fn ipc_event_shape_round_trips_canonical_and_minimal_fixtures() {
    // Risk: Rust/TS contract drift. Level: particular-integration. Source:
    // proposal test-intent "Rust DTO shape and serde round-trip".
    for fixture_name in ["canonical-event.json", "minimal-event.json"] {
        let fixture = read_json(fixture_path(fixture_name));
        let event: IpcEvent<Value> =
            serde_json::from_value(fixture.clone()).expect("IpcEvent fixture must deserialize");

        assert_eq!(
            serde_json::to_value(&event).expect("IpcEvent must serialize"),
            fixture,
            "{fixture_name} should round-trip without field drift"
        );
    }

    assert!(
        serde_json::from_value::<IpcEvent<Value>>(serde_json::json!({
            "event_id": "event-contract-extra",
            "workspace_id": "workspace-alpha",
            "topic": "runtime",
            "payload": { "kind": "runtime_ping" },
            "created_at": "2026-04-29T00:00:00.000Z",
            "extra": true
        }))
        .is_err(),
        "unknown IpcEvent fields must be rejected"
    );
}

#[test]
fn build_ipc_event_generates_metadata_and_preserves_fixture_fields() {
    // Risk: builder drops envelope metadata or payload. Level: unit. Source:
    // proposal test-intent "Rust build success behavior".
    let case: BuildSuccessCase =
        serde_json::from_value(read_json(fixture_path("build-success.json")))
            .expect("build success fixture must deserialize");
    let topic = case
        .input
        .topic
        .expect("success fixture must include topic");
    let payload = case
        .input
        .payload
        .clone()
        .expect("success fixture must include payload");

    let event = build_ipc_event(
        &case.input.workspace_id,
        topic,
        payload,
        case.input.trace_context_id.as_deref(),
    )
    .expect("fixture should build an IpcEvent");
    let serialized = serde_json::to_value(&event).expect("IpcEvent must serialize");

    assert!(!event.event_id.is_empty(), "event_id must be non-empty");
    assert!(!event.created_at.is_empty(), "created_at must be non-empty");

    for key in ["workspace_id", "topic", "payload", "trace_context_id"] {
        assert_eq!(
            serialized.get(key),
            case.expected.get(key),
            "{key} must be preserved from the documented input"
        );
    }

    let round_tripped: IpcEvent<Value> =
        serde_json::from_value(serialized).expect("built IpcEvent must deserialize");
    assert_eq!(round_tripped.workspace_id, case.input.workspace_id);
    assert_eq!(round_tripped.topic, topic);
}

#[test]
fn build_ipc_event_errors_are_reachable_through_documented_inputs() {
    // Risk: unreachable or renamed error surface. Level: unit. Source:
    // proposal test-intent "Rust error reachability and taxonomy".
    let cases: Vec<BuildErrorCase> =
        serde_json::from_value(read_json(fixture_path("build-errors.json")))
            .expect("build error fixture must deserialize");

    for case in cases {
        let actual = match case.expected_error {
            IpcEventError::EmptyWorkspaceId => build_ipc_event(
                &case.input.workspace_id,
                case.input
                    .topic
                    .expect("empty-workspace fixture must include topic"),
                case.input
                    .payload
                    .clone()
                    .expect("empty-workspace fixture must include payload"),
                case.input.trace_context_id.as_deref(),
            )
            .map(|_| ()),
            IpcEventError::UnknownTopic => build_ipc_event_from_raw_topic(
                &case.input.workspace_id,
                case.input
                    .raw_topic
                    .as_deref()
                    .expect("unknown-topic fixture must include raw_topic"),
                case.input
                    .payload
                    .clone()
                    .expect("unknown-topic fixture must include payload"),
                case.input.trace_context_id.as_deref(),
            )
            .map(|_| ()),
            IpcEventError::PayloadSerializationFailed => {
                assert_eq!(
                    case.input.payload_fixture.as_deref(),
                    Some("non_serializable"),
                    "{} should use the documented non-serializable payload fixture",
                    case.name
                );
                build_ipc_event(
                    &case.input.workspace_id,
                    case.input
                        .topic
                        .expect("payload-error fixture must include topic"),
                    NonSerializablePayload,
                    case.input.trace_context_id.as_deref(),
                )
                .map(|_| ())
            }
            IpcEventError::EmptyEventId => panic!(
                "{} should be covered by error taxonomy round-trip, not normal builder input",
                case.name
            ),
        };

        assert_eq!(
            actual,
            Err(case.expected_error),
            "{} should reach its documented IpcEventError",
            case.name
        );
    }
}

#[test]
fn ipc_event_error_variants_round_trip_and_unknown_variant_is_rejected() {
    // Risk: Rust/TS error taxonomy drift. Level: particular-integration.
    // Source: WU acceptance criteria and contract error fixtures.
    let expected_names = vec![
        "EmptyEventId",
        "EmptyWorkspaceId",
        "UnknownTopic",
        "PayloadSerializationFailed",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("ipc-event-errors.json")))
            .expect("ipc event error fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: IpcEventError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<IpcEventError>(Value::String("OtherError".to_string())).is_err(),
        "unknown IpcEventError variants must be rejected"
    );
}
