use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::trace_context::{TraceActor, TraceContext, TraceContextError};
use agent_harness_lib::tracing::trace_context::{create_trace_context, parse_trace_actor};
use serde::Deserialize;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-09";

#[derive(Debug, Deserialize)]
struct CreateInput {
    workspace_id: String,
    actor: String,
    invocation_id: Option<String>,
    parent_invocation_id: Option<String>,
    graph_ref: Option<String>,
    audit_event_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateSuccessCase {
    input: CreateInput,
    expected: Value,
}

#[derive(Debug, Deserialize)]
struct CreateErrorCase {
    name: String,
    input: CreateInput,
    expected_error: TraceContextError,
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

fn create_from_input(input: &CreateInput) -> Result<TraceContext, TraceContextError> {
    create_trace_context(
        &input.workspace_id,
        &input.actor,
        input.invocation_id.as_deref(),
        input.parent_invocation_id.as_deref(),
        input.graph_ref.as_deref(),
        input.audit_event_ref.as_deref(),
    )
}

#[test]
fn trace_context_shape_matches_canonical_and_minimal_fixtures() {
    // Risk: Rust/TS contract drift. Level: particular-integration. Source:
    // proposal test-intent "Rust DTO shape and serde round-trip".
    for fixture_name in ["canonical-context.json", "minimal-context.json"] {
        let fixture = read_json(fixture_path(fixture_name));
        let context: TraceContext =
            serde_json::from_value(fixture.clone()).expect("TraceContext fixture must deserialize");

        assert_eq!(
            serde_json::to_value(&context).expect("TraceContext must serialize"),
            fixture,
            "{fixture_name} should round-trip without field drift"
        );
    }

    assert!(
        serde_json::from_value::<TraceContext>(serde_json::json!({
            "correlation_id": "trace-contract-correlation",
            "workspace_id": "workspace-alpha",
            "actor": "user",
            "extra": true
        }))
        .is_err(),
        "unknown TraceContext fields must be rejected"
    );
}

#[test]
fn trace_actor_taxonomy_matches_canonical_fixture() {
    // Risk: actor taxonomy drift. Level: particular-integration. Source:
    // proposal test-intent "Rust actor taxonomy".
    let fixture_actors: Vec<String> =
        serde_json::from_value(read_json(fixture_path("trace-actors.json")))
            .expect("actor fixture must be a string array");

    for raw in fixture_actors {
        let actor: TraceActor = serde_json::from_value(Value::String(raw.clone()))
            .expect("documented actor must deserialize");
        assert_eq!(
            serde_json::to_value(actor).expect("actor must serialize"),
            Value::String(raw.clone())
        );
        assert_eq!(parse_trace_actor(&raw), Ok(actor));
    }

    assert_eq!(
        parse_trace_actor("unknown"),
        Err(TraceContextError::UnknownActor)
    );
    assert!(
        serde_json::from_value::<TraceActor>(Value::String("unknown".to_string())).is_err(),
        "unknown TraceActor strings must be rejected by serde"
    );
}

#[test]
fn create_trace_context_minimal_input_generates_required_context() {
    // Risk: creation helper drops required fields. Level: unit. Source:
    // proposal test-intent "Rust create success behavior".
    let case: CreateSuccessCase =
        serde_json::from_value(read_json(fixture_path("create-success-minimal.json")))
            .expect("minimal success fixture must deserialize");

    let context = create_from_input(&case.input).expect("minimal TraceContext should be created");
    let serialized = serde_json::to_value(&context).expect("TraceContext must serialize");

    assert!(
        !context.correlation_id.is_empty(),
        "correlation_id must be generated and non-empty"
    );
    assert_eq!(
        serialized.get("workspace_id"),
        case.expected.get("workspace_id")
    );
    assert_eq!(serialized.get("actor"), case.expected.get("actor"));
    assert!(serialized.get("invocation_id").is_none());
    assert!(serialized.get("parent_invocation_id").is_none());
    assert!(serialized.get("graph_ref").is_none());
    assert!(serialized.get("audit_event_ref").is_none());
}

#[test]
fn create_trace_context_full_input_preserves_lineage_and_references() {
    // Risk: creation helper drops optional lineage or references. Level: unit.
    // Source: proposal test-intent "Rust create success behavior".
    let case: CreateSuccessCase =
        serde_json::from_value(read_json(fixture_path("create-success-full.json")))
            .expect("full success fixture must deserialize");

    let context = create_from_input(&case.input).expect("full TraceContext should be created");
    let serialized = serde_json::to_value(&context).expect("TraceContext must serialize");

    assert!(
        !context.correlation_id.is_empty(),
        "correlation_id must be generated and non-empty"
    );
    for key in [
        "workspace_id",
        "actor",
        "invocation_id",
        "parent_invocation_id",
        "graph_ref",
        "audit_event_ref",
    ] {
        assert_eq!(
            serialized.get(key),
            case.expected.get(key),
            "{key} must be preserved from the documented input"
        );
    }
}

#[test]
fn trace_context_error_variants_are_reachable_through_create_inputs() {
    // Risk: unreachable or renamed error surface. Level: unit. Source:
    // proposal test-intent "Rust error reachability".
    let cases: Vec<CreateErrorCase> =
        serde_json::from_value(read_json(fixture_path("create-errors.json")))
            .expect("create error fixture must deserialize");

    for case in cases {
        assert_eq!(
            create_from_input(&case.input),
            Err(case.expected_error),
            "{} should reach its documented TraceContextError",
            case.name
        );
    }
}

#[test]
fn trace_context_error_variants_round_trip_and_unknown_variant_is_rejected() {
    // Risk: Rust/TS error taxonomy drift. Level: particular-integration.
    // Source: WU acceptance criteria and contract error fixtures.
    let expected_names = vec![
        "EmptyWorkspaceId",
        "UnknownActor",
        "ParentWithoutInvocation",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("trace-context-errors.json")))
            .expect("trace context error fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: TraceContextError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<TraceContextError>(Value::String("OtherError".to_string()))
            .is_err(),
        "unknown TraceContextError variants must be rejected"
    );
}
