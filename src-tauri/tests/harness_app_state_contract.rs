use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use agent_harness_lib::app_state::{
    init_harness_app_state, init_harness_app_state_with_factories, DefaultEventBusFactory,
    DefaultTraceContextFactoryInitializer, EventBusFactory, EventBusHandle, TraceContextFactory,
    TraceContextFactoryInitializer,
};
use agent_harness_lib::contracts::harness_app_state::BootstrapError;
use agent_harness_lib::contracts::harness_settings::HarnessSettings;
use agent_harness_lib::contracts::local_storage_layout::LocalStorageLayout;
use serde::Deserialize;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-04";

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Deserialize)]
struct AppStateCase {
    settings: HarnessSettings,
    storage_layout: LocalStorageLayout,
    expected_error: Option<BootstrapError>,
    database_path_is_directory: Option<bool>,
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

fn temp_fixture_root(name: &str) -> PathBuf {
    let counter = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "agent-harness-wu-0a-04-{name}-{}-{counter}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("stale temp fixture directory should be removable");
    }
    fs::create_dir_all(&path).expect("temp fixture directory should be creatable");
    path
}

fn materialize_tokens(value: Value, temp_root: &Path) -> Value {
    match value {
        Value::String(raw) => {
            Value::String(raw.replace("__TEMP_ROOT__", &temp_root.to_string_lossy()))
        }
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| materialize_tokens(value, temp_root))
                .collect(),
        ),
        Value::Object(entries) => Value::Object(
            entries
                .into_iter()
                .map(|(key, value)| (key, materialize_tokens(value, temp_root)))
                .collect(),
        ),
        other => other,
    }
}

fn read_case(fixture_name: &str, temp_root: &Path) -> AppStateCase {
    let value = materialize_tokens(read_json(fixture_path(fixture_name)), temp_root);
    serde_json::from_value(value).expect("app-state fixture must deserialize")
}

fn prepare_filesystem(case: &AppStateCase) {
    fs::create_dir_all(&case.storage_layout.storage_root)
        .expect("storage root should be creatable for contract fixture");
    fs::create_dir_all(
        Path::new(&case.settings.agent_runner_bin)
            .parent()
            .expect("agent runner fixture has a parent directory"),
    )
    .expect("agent runner bin parent should be creatable");
    fs::write(&case.settings.agent_runner_bin, "#!/bin/sh\nexit 0\n")
        .expect("fake agent runner fixture should be writable");

    if case.database_path_is_directory.unwrap_or(false) {
        fs::create_dir_all(&case.storage_layout.database_path)
            .expect("database path directory should be creatable");
    }
}

#[tokio::test]
async fn init_harness_app_state_creates_all_handles_and_preserves_paths() {
    // Risk: container wiring drops or mutates owned subsystem handles. Level:
    // particular-integration. Source: proposal test-intent "Happy-path
    // app-state initialization".
    let temp_root = temp_fixture_root("happy");
    let case = read_case("happy-path.json", &temp_root);
    prepare_filesystem(&case);

    let state = init_harness_app_state(case.settings.clone(), case.storage_layout.clone())
        .await
        .expect("happy path app state should initialize");

    assert_eq!(state.settings.workspace_id, case.settings.workspace_id);
    assert_eq!(
        state.storage_layout.storage_root,
        case.storage_layout.storage_root
    );
    assert_eq!(
        state.storage_layout.database_path,
        case.storage_layout.database_path
    );

    let _receiver = state.event_bus.subscribe();
    let trace_context = state
        .trace_context_factory
        .create("backend", None, None, None, None)
        .expect("trace context factory should create backend traces");
    assert_eq!(trace_context.workspace_id, state.settings.workspace_id);

    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&state.db)
        .await
        .expect("SQLite pool should execute a simple query");
    assert_eq!(row.0, 1);

    assert_eq!(agent_harness_lib::registered_command_count(), 0);

    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn init_harness_app_state_rejects_invalid_settings() {
    // Risk: invalid settings reach runtime handles. Level: unit. Source:
    // proposal test-intent "Settings validation error".
    let temp_root = temp_fixture_root("settings-invalid");
    let case = read_case("error-settings-invalid.json", &temp_root);
    prepare_filesystem(&case);
    let expected_error = expected_error(&case);

    let result = init_harness_app_state(case.settings, case.storage_layout).await;

    assert_eq!(result.map(|_| ()), Err(expected_error));
    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn init_harness_app_state_rejects_invalid_storage_layout() {
    // Risk: invalid layout reaches SQLite. Level: unit. Source:
    // proposal test-intent "Storage layout validation error".
    let temp_root = temp_fixture_root("layout-invalid");
    let case = read_case("error-storage-layout-invalid.json", &temp_root);
    prepare_filesystem(&case);
    let expected_error = expected_error(&case);

    let result = init_harness_app_state(case.settings, case.storage_layout).await;

    assert_eq!(result.map(|_| ()), Err(expected_error));
    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn init_harness_app_state_reports_database_open_failure() {
    // Risk: database failure is hidden or misclassified. Level:
    // particular-integration. Source: proposal test-intent "SQLite open error".
    let temp_root = temp_fixture_root("database-open-failed");
    let case = read_case("error-database-open-failed.json", &temp_root);
    prepare_filesystem(&case);
    let expected_error = expected_error(&case);

    let result = init_harness_app_state(case.settings, case.storage_layout).await;

    assert_eq!(result.map(|_| ()), Err(expected_error));
    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn injectable_event_bus_factory_failure_is_reported() {
    // Risk: test-only event seam cannot prove documented taxonomy. Level: unit.
    // Source: proposal test-intent "Event bus factory error".
    let temp_root = temp_fixture_root("event-bus-init-failed");
    let case = read_case("error-event-bus-init-failed.json", &temp_root);
    prepare_filesystem(&case);
    let expected_error = expected_error(&case);

    let result = init_harness_app_state_with_factories(
        case.settings,
        case.storage_layout,
        &FailingEventBusFactory,
        &DefaultTraceContextFactoryInitializer,
    )
    .await;

    assert_eq!(result.map(|_| ()), Err(expected_error));
    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn injectable_trace_factory_failure_is_reported() {
    // Risk: test-only trace seam cannot prove documented taxonomy. Level: unit.
    // Source: proposal test-intent "Trace factory error".
    let temp_root = temp_fixture_root("trace-init-failed");
    let case = read_case("error-trace-init-failed.json", &temp_root);
    prepare_filesystem(&case);
    let expected_error = expected_error(&case);

    let result = init_harness_app_state_with_factories(
        case.settings,
        case.storage_layout,
        &DefaultEventBusFactory,
        &FailingTraceContextFactoryInitializer,
    )
    .await;

    assert_eq!(result.map(|_| ()), Err(expected_error));
    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[test]
fn bootstrap_error_variants_round_trip_and_unknown_variant_is_rejected() {
    // Risk: Rust error taxonomy drift. Level: particular-integration. Source:
    // WU acceptance criteria and contract error fixtures.
    let expected_names = vec![
        "SettingsInvalid",
        "StorageLayoutInvalid",
        "DatabaseOpenFailed",
        "EventBusInitFailed",
        "TraceInitFailed",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("bootstrap-errors.json")))
            .expect("bootstrap errors fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: BootstrapError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<BootstrapError>(read_json(fixture_path(
            "invalid-bootstrap-error.json",
        )))
        .is_err(),
        "unknown BootstrapError variants must be rejected"
    );
}

struct FailingEventBusFactory;

impl EventBusFactory for FailingEventBusFactory {
    fn create_event_bus(&self) -> Result<EventBusHandle, BootstrapError> {
        Err(BootstrapError::EventBusInitFailed)
    }
}

fn expected_error(case: &AppStateCase) -> BootstrapError {
    case.expected_error
        .expect("error fixture must include expected_error")
}

struct FailingTraceContextFactoryInitializer;

impl TraceContextFactoryInitializer for FailingTraceContextFactoryInitializer {
    fn create_trace_context_factory(
        &self,
        _workspace_id: &str,
    ) -> Result<TraceContextFactory, BootstrapError> {
        Err(BootstrapError::TraceInitFailed)
    }
}
