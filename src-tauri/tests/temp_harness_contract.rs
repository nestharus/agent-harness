use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use agent_harness_lib::contracts::backend_span_event::BackendSpanEvent;
use agent_harness_lib::contracts::event_topic::EventTopic;
use agent_harness_lib::contracts::harness_settings::{HarnessLogLevel, HarnessSettings};
use agent_harness_lib::contracts::local_storage_layout::LocalStorageLayout;
use agent_harness_lib::contracts::temp_harness::{TempHarnessError, TempHarnessHandle};
use agent_harness_lib::test_harness::temp_harness::{
    attempt_real_agents_spawn, harness_app_state, replay_recorded_runtime_events,
    temp_harness_state,
};
use serde::Deserialize;
use serde_json::Value;
use tokio::time::timeout;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-14a";

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Deserialize)]
struct SeedRegistryFixture {
    happy_seeds: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HappySeedFixture {
    seed_name: String,
    expected_manifest_prefix: String,
    expected_app_state_ready: bool,
    expected_sqlite_tables: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ErrorFixture {
    seed_name: String,
    expected_error: TempHarnessError,
}

#[derive(Debug, Deserialize)]
struct RuntimeReplayFixture {
    seed_name: String,
    expected_replayed_event_count: usize,
    events: Vec<BackendSpanEvent>,
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
        "agent-harness-wu-0a-14a-{name}-{}-{counter}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("stale temp fixture directory should be removable");
    }
    fs::create_dir_all(&path).expect("temp fixture directory should be creatable");
    path
}

fn settings_and_layout(
    temp_root: &Path,
    workspace_id: &str,
    database_path: PathBuf,
) -> (HarnessSettings, LocalStorageLayout) {
    let storage_root = temp_root.join("storage");
    fs::create_dir_all(&storage_root).expect("storage root should be creatable");

    let settings = HarnessSettings {
        workspace_id: workspace_id.to_string(),
        storage_root: storage_root.to_string_lossy().into_owned(),
        database_path: database_path.to_string_lossy().into_owned(),
        agent_runner_bin: temp_root.join("fake-agents").to_string_lossy().into_owned(),
        log_level: HarnessLogLevel::Info,
        profile_name: None,
    };
    let layout = LocalStorageLayout {
        storage_root: settings.storage_root.clone(),
        database_path: settings.database_path.clone(),
        evidence_root: storage_root.join("evidence").to_string_lossy().into_owned(),
        fixture_root: storage_root.join("fixtures").to_string_lossy().into_owned(),
        log_root: storage_root.join("logs").to_string_lossy().into_owned(),
        temp_root: storage_root.join("tmp").to_string_lossy().into_owned(),
    };

    (settings, layout)
}

async fn sqlite_table_names(handle: &TempHarnessHandle) -> Vec<String> {
    let state = harness_app_state(handle).expect("successful handle should resolve app state");
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .fetch_all(&state.db)
            .await
            .expect("sqlite_master should be readable");

    rows.into_iter().map(|(name,)| name).collect()
}

#[test]
fn temp_harness_handle_round_trips_stable_field_names() {
    // Risk: Rust DTO field drift. Level: particular-integration. Source:
    // WU-0A-14a proposal test-intent "DTO serde round-trip".
    let fixture = read_json(fixture_path("handle-round-trip.json"));
    let handle: TempHarnessHandle =
        serde_json::from_value(fixture.clone()).expect("TempHarnessHandle fixture must parse");

    assert_eq!(
        serde_json::to_value(&handle).expect("TempHarnessHandle must serialize"),
        fixture
    );
    assert!(
        serde_json::from_value::<TempHarnessHandle>(serde_json::json!({
            "workspace_id": "workspace-contract-temp-harness",
            "database_path": "/tmp/agent-harness.sqlite",
            "fixture_manifest_id": "manifest-empty-contract",
            "app_state_ready": true,
            "extra": true
        }))
        .is_err(),
        "unknown TempHarnessHandle fields must be rejected"
    );
}

#[tokio::test]
async fn documented_seeds_create_ready_isolated_empty_sqlite_handles() {
    // Risk: seed registry drift or accidental GraphStore setup. Level:
    // particular-integration. Source: WU-0A-14a proposal test-intent "Seed
    // happy paths" and "SQLite isolation and no migrations".
    let registry: SeedRegistryFixture =
        serde_json::from_value(read_json(fixture_path("seed-registry.json")))
            .expect("seed registry fixture must parse");

    let happy_fixtures = ["happy-empty.json", "happy-minimal-runtime.json"];
    assert_eq!(registry.happy_seeds.len(), happy_fixtures.len());

    for fixture_name in happy_fixtures {
        let fixture: HappySeedFixture =
            serde_json::from_value(read_json(fixture_path(fixture_name)))
                .expect("happy seed fixture must parse");
        assert!(
            registry.happy_seeds.contains(&fixture.seed_name),
            "{fixture_name} must describe a documented happy seed"
        );

        let handle = temp_harness_state(&fixture.seed_name, None, None, Vec::new())
            .await
            .expect("documented seed should initialize temp harness state");

        assert!(!handle.workspace_id.is_empty());
        assert!(!handle.database_path.is_empty());
        assert!(Path::new(&handle.database_path).exists());
        assert!(handle
            .fixture_manifest_id
            .starts_with(&fixture.expected_manifest_prefix));
        assert_eq!(handle.app_state_ready, fixture.expected_app_state_ready);
        assert_eq!(
            sqlite_table_names(&handle).await,
            fixture.expected_sqlite_tables
        );
    }

    assert_eq!(
        agent_harness_lib::phase_0a_scaffold_commands(),
        ["subscribe_workspace_events"]
    );
    assert_eq!(agent_harness_lib::registered_command_count(), 1);
}

#[tokio::test]
async fn temp_harness_uses_live_app_state_and_replays_runtime_event_fixtures() {
    // Risk: bypassing WU-0A-04 app-state or starting a producer. Level:
    // particular-integration. Source: WU-0A-14a proposal test-intent
    // "App-state composition".
    let fixture: RuntimeReplayFixture =
        serde_json::from_value(read_json(fixture_path("runtime-event-replay.json")))
            .expect("runtime replay fixture must parse");

    let handle = temp_harness_state(&fixture.seed_name, None, None, fixture.events.clone())
        .await
        .expect("runtime fixture seed should initialize");
    let state = harness_app_state(&handle).expect("handle should resolve a live app state");
    let mut receiver = state.event_bus.subscribe();

    let replayed =
        replay_recorded_runtime_events(&handle).expect("recorded runtime fixtures should replay");
    assert_eq!(replayed, fixture.expected_replayed_event_count);

    let event = timeout(Duration::from_secs(1), receiver.recv())
        .await
        .expect("runtime fixture event should be broadcast")
        .expect("runtime fixture event should not lag");
    assert_eq!(event.topic, EventTopic::Runtime);
    assert_eq!(event.workspace_id, handle.workspace_id);
    assert_eq!(
        event.payload["span_event_id"],
        Value::String(fixture.events[0].span_event_id.clone())
    );
}

#[test]
fn temp_harness_error_variants_round_trip_and_unknown_is_rejected() {
    // Risk: caller-visible error taxonomy drift. Level: particular-integration.
    // Source: WU-0A-14a proposal test-intent "Error taxonomy".
    let expected_names = vec![
        "UnknownSeed",
        "DatabaseCreateFailed",
        "AppStateInitFailed",
        "RealAgentsInvocationAttempted",
        "FixtureManifestMissing",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("temp-harness-errors.json")))
            .expect("error fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: TempHarnessError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<TempHarnessError>(Value::String("OtherError".to_string()))
            .is_err(),
        "unknown TempHarnessError variants must be rejected"
    );
}

#[tokio::test]
async fn documented_error_inputs_reach_every_temp_harness_error_variant() {
    // Risk: documented errors are unreachable or misclassified. Level:
    // unit/particular-integration. Source: WU-0A-14a proposal test-intent
    // "Error taxonomy" and "Real agents refusal".
    let unknown: ErrorFixture =
        serde_json::from_value(read_json(fixture_path("error-unknown-seed.json")))
            .expect("unknown-seed fixture must parse");
    assert_eq!(
        temp_harness_state(&unknown.seed_name, None, None, Vec::new())
            .await
            .map(|_| ()),
        Err(unknown.expected_error)
    );

    let db_failure: ErrorFixture =
        serde_json::from_value(read_json(fixture_path("error-database-create-failed.json")))
            .expect("database-create-failed fixture must parse");
    let temp_root = temp_fixture_root("database-create-failed");
    let database_path = temp_root.join("storage").join("harness.sqlite");
    fs::create_dir_all(&database_path).expect("database path directory should be creatable");
    let (settings, layout) = settings_and_layout(
        &temp_root,
        "workspace-database-create-failed",
        database_path,
    );
    assert_eq!(
        temp_harness_state(
            &db_failure.seed_name,
            Some(settings),
            Some(layout),
            Vec::new()
        )
        .await
        .map(|_| ()),
        Err(db_failure.expected_error)
    );

    let app_failure: ErrorFixture =
        serde_json::from_value(read_json(fixture_path("error-app-state-init-failed.json")))
            .expect("app-state-init-failed fixture must parse");
    let temp_root = temp_fixture_root("app-state-init-failed");
    let database_path = temp_root.join("storage").join("harness.sqlite");
    let (settings, layout) = settings_and_layout(&temp_root, "", database_path);
    assert_eq!(
        temp_harness_state(
            &app_failure.seed_name,
            Some(settings),
            Some(layout),
            Vec::new()
        )
        .await
        .map(|_| ()),
        Err(app_failure.expected_error)
    );

    let manifest_missing: ErrorFixture = serde_json::from_value(read_json(fixture_path(
        "error-fixture-manifest-missing.json",
    )))
    .expect("fixture-manifest-missing fixture must parse");
    assert_eq!(
        temp_harness_state(&manifest_missing.seed_name, None, None, Vec::new())
            .await
            .map(|_| ()),
        Err(manifest_missing.expected_error)
    );

    let real_agents: ErrorFixture = serde_json::from_value(read_json(fixture_path(
        "error-real-agents-invocation-attempted.json",
    )))
    .expect("real-agents fixture must parse");
    let handle = temp_harness_state(&real_agents.seed_name, None, None, Vec::new())
        .await
        .expect("real-agents refusal fixture needs a ready handle");
    assert_eq!(
        attempt_real_agents_spawn(&handle).map(|_| ()),
        Err(real_agents.expected_error)
    );
}

#[test]
fn temp_harness_source_contains_no_graphstore_migration_hookpoints() {
    // Risk: accidental GraphStore setup. Level: particular-integration.
    // Source: WU-0A-14a proposal test-intent "SQLite isolation and no
    // migrations".
    let source = fs::read_to_string(repo_root().join("src-tauri/src/test_harness/temp_harness.rs"))
        .expect("temp harness source should be readable");

    assert!(!source.contains("sqlx::migrate!"));
    assert!(!source.contains("MIGRATIONS_DIR"));
}
