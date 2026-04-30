use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::fake_agents_fixture::{
    ChildAcceptanceState, FakeAgentsFixture, FakeAgentsFixtureError,
};
use agent_harness_lib::contracts::temp_harness::TempHarnessHandle;
use agent_harness_lib::test_harness::fake_agents::install_fake_agents_fixture;
use agent_harness_lib::test_harness::temp_harness::{harness_app_state, temp_harness_state};
use serde::Deserialize;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-15";
const REAL_AGENTS_PATH: &str = "/home/nes/.local/bin/agents";

#[derive(Debug, Deserialize)]
struct ErrorFixture {
    scenario_name: String,
    expected_error: FakeAgentsFixtureError,
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

async fn ready_temp_harness() -> TempHarnessHandle {
    temp_harness_state("empty", None, None, Vec::new())
        .await
        .expect("fake agents fixture tests need a ready temp harness")
}

fn fixture_install_root(handle: &TempHarnessHandle) -> PathBuf {
    let state = harness_app_state(handle).expect("handle should resolve a live app state");
    Path::new(&state.storage_layout.fixture_root).join(&handle.fixture_manifest_id)
}

#[test]
fn fake_agents_fixture_round_trips_stable_field_names() {
    // Risk: Rust DTO field drift. Level: particular-integration. Source:
    // WU-0A-15 proposal test-intent "DTO serde round-trip".
    let fixture = read_json(fixture_path("fixture-round-trip.json"));
    let parsed: FakeAgentsFixture =
        serde_json::from_value(fixture.clone()).expect("round-trip fixture must parse");

    assert_eq!(
        serde_json::to_value(&parsed).expect("FakeAgentsFixture must serialize"),
        fixture
    );

    let absent_optional = serde_json::json!({
        "bin_path": "/tmp/agent-harness-wu-0a-15/fake-agents-unknown",
        "scenario_name": "unknown-state-fixture",
        "argv_log_ref": "/tmp/agent-harness-wu-0a-15/argv.json",
        "exit_status": 0,
        "child_acceptance_state": "unknown"
    });
    let parsed: FakeAgentsFixture =
        serde_json::from_value(absent_optional.clone()).expect("optional fields may be absent");
    assert_eq!(
        serde_json::to_value(&parsed).expect("optional-absent fixture must serialize"),
        absent_optional
    );

    assert!(
        serde_json::from_value::<FakeAgentsFixture>(serde_json::json!({
            "bin_path": "/tmp/fake-agents",
            "scenario_name": "success",
            "argv_log_ref": "/tmp/argv.json",
            "exit_status": 0,
            "child_acceptance_state": "accepted",
            "extra": true
        }))
        .is_err(),
        "unknown FakeAgentsFixture fields must be rejected"
    );
}

#[test]
fn fake_agents_error_variants_round_trip_and_unknown_is_rejected() {
    // Risk: caller-visible error taxonomy drift. Level: unit. Source:
    // WU-0A-15 proposal test-intent "Error taxonomy".
    let expected_names = vec![
        "UnknownScenario",
        "FixtureInstallFailed",
        "ArgvLogMissing",
        "TranscriptRefMissing",
        "RealAgentsPathRejected",
        "InvalidChildAcceptanceState",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("fake-agents-fixture-errors.json")))
            .expect("error fixture must be a string array");
    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: FakeAgentsFixtureError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<FakeAgentsFixtureError>(Value::String("OtherError".to_string()))
            .is_err(),
        "unknown FakeAgentsFixtureError variants must be rejected"
    );
}

#[tokio::test]
async fn documented_child_acceptance_states_are_reachable() {
    // Risk: unreachable child acceptance states. Level: particular-integration.
    // Source: WU-0A-15 proposal test-intent "Child acceptance states".
    let cases = [
        ("unknown-state-fixture", ChildAcceptanceState::Unknown),
        ("success", ChildAcceptanceState::Accepted),
        ("rejected-fixture", ChildAcceptanceState::Rejected),
        ("timeout", ChildAcceptanceState::TimedOut),
        ("cancellation", ChildAcceptanceState::Ambiguous),
    ];

    for (scenario_name, expected_state) in cases {
        let handle = ready_temp_harness().await;
        let fixture = install_fake_agents_fixture(handle, scenario_name)
            .expect("documented child acceptance state scenario should install");
        assert_eq!(fixture.child_acceptance_state, expected_state);
    }
}

#[tokio::test]
async fn success_fixture_installs_under_temp_harness_root_and_materializes_refs() {
    // Risk: accidental real agents path use or success scenario drift. Level:
    // particular-integration. Source: WU-0A-15 proposal test-intent "Bin path
    // safety" and "Scenario behavior".
    let handle = ready_temp_harness().await;
    let expected_root = fixture_install_root(&handle);
    let fixture =
        install_fake_agents_fixture(handle, "success").expect("success scenario should install");

    let canonical_bin = fs::canonicalize(&fixture.bin_path).expect("fake agents bin should exist");
    let canonical_root =
        fs::canonicalize(expected_root).expect("fixture install root should exist");
    assert!(canonical_bin.starts_with(&canonical_root));

    if let Ok(real_agents) = fs::canonicalize(REAL_AGENTS_PATH) {
        assert_ne!(canonical_bin, real_agents);
    }
    assert_ne!(Path::new(&fixture.bin_path), Path::new(REAL_AGENTS_PATH));

    assert_eq!(fixture.scenario_name, "success");
    assert_eq!(fixture.exit_status, 0);
    assert_eq!(
        fixture.child_acceptance_state,
        ChildAcceptanceState::Accepted
    );
    assert_eq!(
        read_json(&fixture.argv_log_ref),
        serde_json::json!(["agents", "run", "--scenario", "success"])
    );

    let stdout_ref = fixture
        .stdout_ref
        .as_ref()
        .expect("success scenario declares stdout");
    let stderr_ref = fixture
        .stderr_ref
        .as_ref()
        .expect("success scenario declares stderr");
    assert!(Path::new(stdout_ref).exists());
    assert!(Path::new(stderr_ref).exists());
}

#[tokio::test]
async fn nonzero_cancellation_and_timeout_scenarios_materialize_contract_refs() {
    // Risk: scenario-specific fake fixture drift. Level:
    // particular-integration. Source: WU-0A-15 proposal test-intent "Scenario
    // behavior".
    let handle = ready_temp_harness().await;
    let nonzero = install_fake_agents_fixture(handle, "nonzero-exit")
        .expect("nonzero scenario should install");
    assert_eq!(nonzero.exit_status, 42);
    assert_eq!(
        nonzero.child_acceptance_state,
        ChildAcceptanceState::Rejected
    );
    assert!(Path::new(&nonzero.argv_log_ref).exists());
    assert!(Path::new(nonzero.stdin_ref.as_ref().expect("stdin ref")).exists());
    assert!(Path::new(nonzero.stdout_ref.as_ref().expect("stdout ref")).exists());
    assert!(Path::new(nonzero.stderr_ref.as_ref().expect("stderr ref")).exists());

    let handle = ready_temp_harness().await;
    let cancellation = install_fake_agents_fixture(handle, "cancellation")
        .expect("cancellation scenario should install");
    assert_eq!(
        cancellation.child_acceptance_state,
        ChildAcceptanceState::Ambiguous
    );
    assert_ne!(
        cancellation.child_acceptance_state,
        ChildAcceptanceState::Accepted,
        "cancellation must not simulate successful child acceptance"
    );
    assert!(Path::new(&cancellation.argv_log_ref).exists());

    let handle = ready_temp_harness().await;
    let timeout =
        install_fake_agents_fixture(handle, "timeout").expect("timeout scenario should install");
    assert_eq!(
        timeout.child_acceptance_state,
        ChildAcceptanceState::TimedOut
    );
    let stderr_ref = timeout
        .stderr_ref
        .as_ref()
        .expect("timeout scenario preserves stderr transcript ref");
    assert!(Path::new(stderr_ref).exists());
    assert!(fs::read_to_string(stderr_ref)
        .expect("timeout stderr transcript should be readable")
        .contains("timeout stderr transcript"));
}

#[tokio::test]
async fn documented_error_inputs_reach_every_fake_agents_error_variant() {
    // Risk: documented errors are unreachable or misclassified. Level:
    // unit/particular-integration. Source: WU-0A-15 proposal test-intent
    // "Error taxonomy".
    let cases = [
        "errors/error-unknown-scenario.json",
        "errors/error-fixture-install-failed.json",
        "errors/error-argv-log-missing.json",
        "errors/error-transcript-ref-missing.json",
        "errors/error-real-agents-path-rejected.json",
        "errors/error-invalid-child-acceptance-state.json",
    ];

    for fixture_name in cases {
        let error_fixture: ErrorFixture =
            serde_json::from_value(read_json(fixture_path(fixture_name)))
                .expect("error fixture must parse");
        let handle = ready_temp_harness().await;

        assert_eq!(
            install_fake_agents_fixture(handle, &error_fixture.scenario_name).map(|_| ()),
            Err(error_fixture.expected_error),
            "{fixture_name} should reach its documented error"
        );
    }
}

#[test]
fn fake_agents_fixture_adds_no_commands_and_no_process_execution() {
    // Risk: command leakage or subprocess execution in Phase 0A. Level:
    // particular-integration. Source: WU-0A-15 proposal test-intent "Scaffold
    // invariant" and anti-scope.
    assert_eq!(
        agent_harness_lib::phase_0a_scaffold_commands(),
        ["subscribe_workspace_events"]
    );
    assert_eq!(agent_harness_lib::registered_command_count(), 1);

    let source = fs::read_to_string(repo_root().join("src-tauri/src/test_harness/fake_agents.rs"))
        .expect("fake agents source should be readable after implementation");
    assert!(!source.contains("Command::new"));
    assert!(!source.contains("std::process::Command"));
}
