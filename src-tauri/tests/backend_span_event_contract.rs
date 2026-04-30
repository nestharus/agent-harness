use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::backend_span_event::{
    BackendSpanError, BackendSpanEvent, BackendSpanStatus,
};
use agent_harness_lib::contracts::event_topic::EventTopic;
use agent_harness_lib::contracts::ipc_event::IpcEvent;
use agent_harness_lib::contracts::trace_context::{TraceActor, TraceContext};
use agent_harness_lib::events::runtime_events::build_backend_span_runtime_event;
use agent_harness_lib::phase_0a_scaffold_commands;
use agent_harness_lib::tracing::backend_span_event::emit_backend_span;
use agent_harness_lib::tracing::trace_context::create_trace_context;
use serde::Deserialize;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-10";

#[derive(Debug, Deserialize)]
struct EmitInput {
    trace_context: TraceContext,
    span_name: String,
    started_at: String,
    completed_at: Option<String>,
    status: BackendSpanStatus,
    error_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HappyPathCase {
    input: EmitInput,
    expected: Value,
}

#[derive(Debug, Deserialize)]
struct ErrorCase {
    name: String,
    input: EmitInput,
    expected_error: BackendSpanError,
}

#[derive(Debug, Deserialize)]
struct TraceContextInvalidCase {
    name: String,
    input: EmitInput,
    expected_error: BackendSpanError,
    delegated_trace_context_error: String,
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

fn read_happy_path(name: &str) -> HappyPathCase {
    serde_json::from_value(read_json(fixture_path(name)))
        .unwrap_or_else(|error| panic!("{name} should match the happy-path fixture shape: {error}"))
}

fn real_trace_context() -> TraceContext {
    create_trace_context(
        "workspace-alpha",
        "backend",
        Some("invocation-contract"),
        None,
        None,
        None,
    )
    .expect("real TraceContext should be created through WU-0A-09 helper")
}

fn emit_from_input_with_trace(
    input: &EmitInput,
    trace_context: TraceContext,
) -> Result<BackendSpanEvent, BackendSpanError> {
    emit_backend_span(
        trace_context,
        &input.span_name,
        &input.started_at,
        input.completed_at.as_deref(),
        input.status,
        input.error_ref.as_deref(),
    )
}

#[test]
fn backend_span_event_round_trips_all_happy_path_fixtures() {
    // Risk: Rust/TS contract drift. Level: particular-integration. Source:
    // proposal test-intent "Rust DTO shape and serde round-trip".
    for fixture_name in [
        "started-happy-path.json",
        "completed-happy-path.json",
        "failed-happy-path.json",
    ] {
        let case = read_happy_path(fixture_name);
        let event: BackendSpanEvent = serde_json::from_value(case.expected.clone())
            .expect("BackendSpanEvent fixture must deserialize");

        assert_eq!(
            serde_json::to_value(&event).expect("BackendSpanEvent must serialize"),
            case.expected,
            "{fixture_name} should round-trip without field drift"
        );
    }

    let envelope_fixture = read_json(fixture_path("runtime-only-topic-event.json"));
    let envelope: IpcEvent<BackendSpanEvent> = serde_json::from_value(envelope_fixture.clone())
        .expect("runtime envelope must deserialize");
    assert_eq!(
        serde_json::to_value(&envelope).expect("runtime envelope must serialize"),
        envelope_fixture
    );

    assert!(
        serde_json::from_value::<BackendSpanEvent>(serde_json::json!({
            "span_event_id": "span-contract-extra",
            "trace_context": {
                "correlation_id": "trace-contract-extra",
                "workspace_id": "workspace-alpha",
                "actor": "backend"
            },
            "span_name": "runtime.bootstrap",
            "started_at": "1000000000",
            "status": "started",
            "extra": true
        }))
        .is_err(),
        "unknown BackendSpanEvent fields must be rejected"
    );
}

#[test]
fn backend_span_status_variants_round_trip_and_are_reachable_from_fixtures() {
    // Risk: status taxonomy drift. Level: particular-integration. Source:
    // proposal test-intent "Rust status taxonomy".
    let expected_names = vec!["started", "completed", "failed"];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("backend-span-statuses.json")))
            .expect("status fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let status: BackendSpanStatus =
            serde_json::from_value(Value::String(name.clone())).expect("status must parse");
        assert_eq!(
            serde_json::to_value(status).expect("status must serialize"),
            Value::String(name)
        );
    }

    for (fixture_name, status) in [
        ("started-happy-path.json", BackendSpanStatus::Started),
        ("completed-happy-path.json", BackendSpanStatus::Completed),
        ("failed-happy-path.json", BackendSpanStatus::Failed),
    ] {
        let case = read_happy_path(fixture_name);
        assert_eq!(
            emit_from_input_with_trace(&case.input, real_trace_context())
                .expect("documented status fixture input should emit")
                .status,
            status,
            "{fixture_name} should reach its documented status"
        );
    }

    assert!(
        serde_json::from_value::<BackendSpanStatus>(Value::String("unknown".to_string())).is_err(),
        "unknown BackendSpanStatus strings must be rejected"
    );
}

#[test]
fn emit_backend_span_started_returns_started_without_completion_or_error() {
    // Risk: builder drops or invents status-specific fields. Level: unit.
    // Source: proposal test-intent "Rust builder success behavior".
    let case = read_happy_path("started-happy-path.json");
    let event = emit_from_input_with_trace(&case.input, real_trace_context())
        .expect("started backend span should emit");

    assert!(!event.span_event_id.is_empty());
    assert_eq!(event.status, BackendSpanStatus::Started);
    assert_eq!(event.span_name, case.input.span_name);
    assert_eq!(event.started_at, case.input.started_at);
    assert_eq!(event.completed_at, None);
    assert_eq!(event.error_ref, None);
}

#[test]
fn emit_backend_span_completed_preserves_ordered_timestamps() {
    // Risk: builder rejects valid completed spans or drops timestamps. Level:
    // unit. Source: proposal test-intent "Rust builder success behavior".
    let case = read_happy_path("completed-happy-path.json");
    let event = emit_from_input_with_trace(&case.input, real_trace_context())
        .expect("completed backend span should emit");

    assert!(!event.span_event_id.is_empty());
    assert_eq!(event.status, BackendSpanStatus::Completed);
    assert_eq!(event.started_at, case.input.started_at);
    assert_eq!(event.completed_at, case.input.completed_at);
    assert_eq!(event.error_ref, None);
}

#[test]
fn emit_backend_span_failed_preserves_error_ref() {
    // Risk: builder drops failure references. Level: unit. Source: proposal
    // test-intent "Rust builder success behavior".
    let case = read_happy_path("failed-happy-path.json");
    let event = emit_from_input_with_trace(&case.input, real_trace_context())
        .expect("failed backend span should emit");

    assert!(!event.span_event_id.is_empty());
    assert_eq!(event.status, BackendSpanStatus::Failed);
    assert_eq!(event.error_ref, case.input.error_ref);
    assert_eq!(event.completed_at, None);
}

#[test]
fn backend_span_error_variants_are_reachable_through_documented_inputs() {
    // Risk: unreachable or renamed error surface. Level: unit. Source:
    // proposal test-intent "Rust error reachability".
    for fixture_name in [
        "error-empty-span-name.json",
        "error-completed-before-started.json",
        "error-failed-without-error-ref.json",
    ] {
        let case: ErrorCase = serde_json::from_value(read_json(fixture_path(fixture_name)))
            .unwrap_or_else(|error| panic!("{fixture_name} should deserialize: {error}"));

        assert_eq!(
            emit_from_input_with_trace(&case.input, real_trace_context()),
            Err(case.expected_error),
            "{} should reach its documented BackendSpanError",
            case.name
        );
    }

    let invalid_trace_case: TraceContextInvalidCase =
        serde_json::from_value(read_json(fixture_path("error-trace-context-invalid.json")))
            .expect("invalid trace context fixture should deserialize");
    assert_eq!(
        invalid_trace_case.delegated_trace_context_error, "EmptyWorkspaceId",
        "fixture should document the WU-0A-09 delegated error shape"
    );
    assert_eq!(
        emit_from_input_with_trace(
            &invalid_trace_case.input,
            invalid_trace_case.input.trace_context.clone(),
        ),
        Err(invalid_trace_case.expected_error),
        "{} should reach TraceContextInvalid",
        invalid_trace_case.name
    );
}

#[test]
fn backend_span_error_variants_round_trip_and_unknown_variant_is_rejected() {
    // Risk: Rust/TS error taxonomy drift. Level: particular-integration.
    // Source: WU acceptance criteria and contract error fixtures.
    let expected_names = vec![
        "EmptySpanName",
        "CompletedBeforeStarted",
        "FailedWithoutErrorRef",
        "TraceContextInvalid",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("backend-span-errors.json")))
            .expect("backend span error fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: BackendSpanError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<BackendSpanError>(Value::String("OtherError".to_string()))
            .is_err(),
        "unknown BackendSpanError variants must be rejected"
    );
}

#[test]
fn backend_span_runtime_event_uses_runtime_topic_only_and_adds_no_command() {
    // Risk: Phase 0A accidentally creates later-domain payloads. Level:
    // particular-integration/static. Source: proposal test-intent
    // "Runtime-only topic binding".
    let case = read_happy_path("completed-happy-path.json");
    let span = emit_from_input_with_trace(&case.input, real_trace_context())
        .expect("completed backend span should emit");
    let envelope = build_backend_span_runtime_event(
        "workspace-alpha",
        span.clone(),
        Some(&span.trace_context.correlation_id),
    )
    .expect("runtime backend span envelope should build");

    assert_eq!(envelope.topic, EventTopic::Runtime);
    assert_eq!(envelope.payload, span);
    assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);

    let fixture_envelope: IpcEvent<BackendSpanEvent> =
        serde_json::from_value(read_json(fixture_path("runtime-only-topic-event.json")))
            .expect("runtime-only fixture must deserialize");
    assert_eq!(fixture_envelope.topic, EventTopic::Runtime);
}

#[test]
fn wu_0a_10_source_references_no_later_domain_event_topic_variants() {
    // Risk: Phase 0A accidentally creates later-domain payloads. Level:
    // static. Source: proposal test-intent "Runtime-only topic binding".
    let source_files = [
        "src-tauri/src/tracing/backend_span_event.rs",
        "src-tauri/src/events/runtime_events.rs",
        "src-tauri/src/contracts/backend_span_event.rs",
    ];
    let mut joined_source = String::new();
    for relative_path in source_files {
        joined_source.push_str(
            &fs::read_to_string(repo_root().join(relative_path))
                .unwrap_or_else(|error| panic!("failed to read {relative_path}: {error}")),
        );
        joined_source.push('\n');
    }

    assert!(
        joined_source.contains("EventTopic::Runtime"),
        "WU-0A-10 source should contain the canonical runtime topic binding"
    );

    for forbidden in [
        "EventTopic::Graph",
        "EventTopic::Render",
        "EventTopic::Provider",
        "EventTopic::Optimizer",
        "EventTopic::Worker",
        "EventTopic::Question",
        "EventTopic::Recovery",
        "EventTopic::Budget",
        "EventTopic::Audit",
    ] {
        assert!(
            !joined_source.contains(forbidden),
            "WU-0A-10 source must not reference {forbidden}"
        );
    }
}

#[test]
fn invalid_trace_context_fixture_preserves_typed_actor_surface() {
    // Risk: TraceContextInvalid is tested with an impossible actor value rather
    // than a real TraceContext instance. Level: unit. Source: proposal
    // assumption A3 and TraceContext nesting design.
    let invalid_trace_case: TraceContextInvalidCase =
        serde_json::from_value(read_json(fixture_path("error-trace-context-invalid.json")))
            .expect("invalid trace context fixture should deserialize");

    assert_eq!(
        invalid_trace_case.input.trace_context.actor,
        TraceActor::Backend
    );
}
