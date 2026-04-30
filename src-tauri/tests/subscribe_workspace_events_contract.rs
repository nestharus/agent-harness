use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use agent_harness_lib::app_state::init_harness_app_state;
use agent_harness_lib::commands::subscribe_workspace_events::{
    subscribe_workspace_events_with_optional_state, subscribe_workspace_events_with_state,
};
use agent_harness_lib::contracts::event_topic::EventTopic;
use agent_harness_lib::contracts::harness_settings::{HarnessLogLevel, HarnessSettings};
use agent_harness_lib::contracts::ipc_event::IpcEvent;
use agent_harness_lib::contracts::local_storage_layout::LocalStorageLayout;
use agent_harness_lib::contracts::subscribe_workspace_events::SubscribeWorkspaceEventsError;
use serde::Deserialize;
use serde_json::Value;
use tauri::ipc::{Channel, InvokeResponseBody};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-08";

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Deserialize)]
struct HappyPathCase {
    workspace_id: String,
    topic: String,
}

#[derive(Debug, Deserialize)]
struct ErrorCase {
    workspace_id: String,
    topic: String,
    expected_error: SubscribeWorkspaceEventsError,
}

#[derive(Debug, Deserialize)]
struct InertTopicCase {
    workspace_id: String,
    topic: String,
    observation_window_ms: u64,
    expected_domain_payload_count: usize,
}

#[derive(Debug, Deserialize)]
struct SubscriptionIdShapeCase {
    workspace_id: String,
    topic: String,
    call_count: usize,
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
        "agent-harness-wu-0a-08-{name}-{}-{counter}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("stale temp fixture directory should be removable");
    }
    fs::create_dir_all(&path).expect("temp fixture directory should be creatable");
    path
}

fn recording_channel() -> (Channel<IpcEvent<Value>>, Arc<Mutex<Vec<Value>>>) {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let recorded_for_channel = Arc::clone(&recorded);

    let channel = Channel::new(move |body| {
        let value = match body {
            InvokeResponseBody::Json(raw) => {
                serde_json::from_str(&raw).map_err(tauri::Error::Json)?
            }
            InvokeResponseBody::Raw(bytes) => Value::Array(
                bytes
                    .into_iter()
                    .map(|byte| Value::Number(byte.into()))
                    .collect(),
            ),
        };
        recorded_for_channel
            .lock()
            .expect("recording channel mutex should not be poisoned")
            .push(value);
        Ok(())
    });

    (channel, recorded)
}

async fn app_state_fixture(name: &str) -> (agent_harness_lib::app_state::HarnessAppState, PathBuf) {
    let temp_root = temp_fixture_root(name);
    let storage_root = temp_root.join("storage");
    fs::create_dir_all(&storage_root).expect("storage root should be creatable");

    let settings = HarnessSettings {
        workspace_id: "workspace-alpha".to_string(),
        storage_root: storage_root.to_string_lossy().into_owned(),
        database_path: storage_root
            .join("harness.sqlite")
            .to_string_lossy()
            .into_owned(),
        agent_runner_bin: temp_root.join("fake-agents").to_string_lossy().into_owned(),
        log_level: HarnessLogLevel::Info,
        profile_name: None,
    };
    let storage_layout = LocalStorageLayout {
        storage_root: settings.storage_root.clone(),
        database_path: settings.database_path.clone(),
        evidence_root: storage_root.join("evidence").to_string_lossy().into_owned(),
        fixture_root: storage_root.join("fixtures").to_string_lossy().into_owned(),
        log_root: storage_root.join("logs").to_string_lossy().into_owned(),
        temp_root: storage_root.join("tmp").to_string_lossy().into_owned(),
    };

    let state = init_harness_app_state(settings, storage_layout)
        .await
        .expect("app state fixture should initialize");
    (state, temp_root)
}

#[test]
fn command_registration_matches_phase_0a_scaffold_allowlist() {
    // Risk: value-slice command leakage. Level: particular-integration. Source:
    // WU-0A-08 proposal test-intent "Rust command registration and scaffold
    // invariant".
    assert_eq!(
        agent_harness_lib::phase_0a_scaffold_commands(),
        ["subscribe_workspace_events"]
    );
    assert_eq!(agent_harness_lib::registered_command_count(), 1);
}

#[tokio::test]
async fn subscribe_workspace_events_succeeds_for_every_event_topic() {
    // Risk: topic coverage drift. Level: particular-integration. Source:
    // WU-0A-08 proposal test-intent "Rust every-topic happy path".
    let cases: Vec<HappyPathCase> =
        serde_json::from_value(read_json(fixture_path("event-topic-happy-paths.json")))
            .expect("happy path fixture must deserialize");
    let (state, temp_root) = app_state_fixture("every-topic").await;

    for case in cases {
        let (channel, _recorded) = recording_channel();
        let subscription_id = subscribe_workspace_events_with_state(
            &state,
            case.workspace_id.clone(),
            case.topic.clone(),
            Some(channel),
        )
        .expect("every documented topic should subscribe");

        let metadata = state
            .event_bus
            .subscription_metadata(&subscription_id)
            .expect("subscription metadata should be retained");
        let expected_topic: EventTopic =
            serde_json::from_value(Value::String(case.topic)).expect("topic fixture parses");
        assert_eq!(metadata.workspace_id, case.workspace_id);
        assert_eq!(metadata.topic, expected_topic);
    }

    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn subscription_id_is_non_empty_unique_and_associated_with_request() {
    // Risk: unusable or untracked subscriptions. Level: particular-integration.
    // Source: WU-0A-08 proposal test-intent "Rust subscription id semantics".
    let case: SubscriptionIdShapeCase =
        serde_json::from_value(read_json(fixture_path("subscription-id-shape.json")))
            .expect("subscription id fixture must deserialize");
    let (state, temp_root) = app_state_fixture("subscription-id").await;

    let mut ids = Vec::new();
    for _ in 0..case.call_count {
        let (channel, _recorded) = recording_channel();
        let subscription_id = subscribe_workspace_events_with_state(
            &state,
            case.workspace_id.clone(),
            case.topic.clone(),
            Some(channel),
        )
        .expect("subscription should succeed");
        assert!(!subscription_id.is_empty());
        assert!(subscription_id.starts_with("sub-"));

        let metadata = state
            .event_bus
            .subscription_metadata(&subscription_id)
            .expect("subscription metadata should be retained");
        assert_eq!(metadata.workspace_id, case.workspace_id);
        assert_eq!(metadata.topic, EventTopic::Runtime);
        ids.push(subscription_id);
    }

    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), case.call_count);

    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn subscribe_workspace_events_reports_documented_error_variants() {
    // Risk: caller-visible error drift. Level: unit/particular-integration.
    // Source: WU-0A-08 proposal test-intent "Rust error taxonomy".
    let (state, temp_root) = app_state_fixture("errors").await;

    let empty_workspace: ErrorCase =
        serde_json::from_value(read_json(fixture_path("error-empty-workspace-id.json")))
            .expect("empty workspace fixture must deserialize");
    let (channel, _recorded) = recording_channel();
    assert_eq!(
        subscribe_workspace_events_with_state(
            &state,
            empty_workspace.workspace_id,
            empty_workspace.topic,
            Some(channel),
        ),
        Err(empty_workspace.expected_error)
    );

    let unknown_topic: ErrorCase =
        serde_json::from_value(read_json(fixture_path("error-unknown-topic.json")))
            .expect("unknown topic fixture must deserialize");
    let (channel, _recorded) = recording_channel();
    assert_eq!(
        subscribe_workspace_events_with_state(
            &state,
            unknown_topic.workspace_id,
            unknown_topic.topic,
            Some(channel),
        ),
        Err(unknown_topic.expected_error)
    );

    let channel_unavailable: ErrorCase =
        serde_json::from_value(read_json(fixture_path("error-channel-unavailable.json")))
            .expect("channel unavailable fixture must deserialize");
    assert_eq!(
        subscribe_workspace_events_with_state(
            &state,
            channel_unavailable.workspace_id,
            channel_unavailable.topic,
            None,
        ),
        Err(channel_unavailable.expected_error)
    );

    let app_state_unavailable: ErrorCase =
        serde_json::from_value(read_json(fixture_path("error-app-state-unavailable.json")))
            .expect("app state unavailable fixture must deserialize");
    let (channel, _recorded) = recording_channel();
    assert_eq!(
        subscribe_workspace_events_with_optional_state(
            None,
            app_state_unavailable.workspace_id,
            app_state_unavailable.topic,
            Some(channel),
        ),
        Err(app_state_unavailable.expected_error)
    );

    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}

#[tokio::test]
async fn inert_non_runtime_topic_subscription_records_zero_domain_payloads() {
    // Risk: Phase 0A accidentally starts value-slice event flow. Level:
    // particular-integration. Source: WU-0A-08 proposal test-intent "Rust
    // inert non-runtime topic".
    let case: InertTopicCase =
        serde_json::from_value(read_json(fixture_path("inert-non-runtime-topic.json")))
            .expect("inert topic fixture must deserialize");
    let (state, temp_root) = app_state_fixture("inert-topic").await;
    let (channel, recorded) = recording_channel();

    let subscription_id =
        subscribe_workspace_events_with_state(&state, case.workspace_id, case.topic, Some(channel))
            .expect("inert non-runtime topic subscription should succeed");
    assert!(
        state
            .event_bus
            .subscription_metadata(&subscription_id)
            .is_some(),
        "subscription should still be registered"
    );

    tokio::time::sleep(Duration::from_millis(case.observation_window_ms)).await;
    assert_eq!(
        recorded
            .lock()
            .expect("recording channel mutex should not be poisoned")
            .len(),
        case.expected_domain_payload_count
    );

    fs::remove_dir_all(temp_root).expect("temp fixture directory should clean up");
}
