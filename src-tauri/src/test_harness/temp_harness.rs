use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use crate::app_state::HarnessAppState;
use crate::contracts::backend_span_event::BackendSpanEvent;
use crate::contracts::event_topic::EventTopic;
use crate::contracts::harness_settings::{HarnessLogLevel, HarnessSettings};
use crate::contracts::ipc_event::IpcEvent;
use crate::contracts::local_storage_layout::LocalStorageLayout;
use crate::contracts::temp_harness::{TempHarnessError, TempHarnessHandle};
use crate::events::ipc_event::build_ipc_event;
use crate::storage::derive_local_storage_layout;

static NEXT_TEMP_HARNESS_COUNTER: AtomicU64 = AtomicU64::new(1);
static HARNESS_REGISTRY: OnceLock<Mutex<HashMap<String, StoredHarness>>> = OnceLock::new();

#[derive(Debug, Clone)]
struct SeedSpec {
    manifest_relative_path: &'static str,
}

#[derive(Debug, Clone)]
struct StoredHarness {
    app_state: HarnessAppState,
    runtime_events: Vec<IpcEvent<Value>>,
}

struct SeedMaterialization {
    settings: HarnessSettings,
    storage_layout: LocalStorageLayout,
    manifest_path: PathBuf,
}

pub async fn temp_harness_state(
    seed_name: &str,
    settings: Option<HarnessSettings>,
    storage_layout: Option<LocalStorageLayout>,
    runtime_event_fixtures: Vec<BackendSpanEvent>,
) -> Result<TempHarnessHandle, TempHarnessError> {
    let seed = seed_registry()
        .get(seed_name)
        .cloned()
        .ok_or(TempHarnessError::UnknownSeed)?;
    let materialized = materialize_seed(seed_name, &seed)?;
    if !materialized.manifest_path.exists() {
        return Err(TempHarnessError::FixtureManifestMissing);
    }

    let settings = settings.unwrap_or(materialized.settings);
    let storage_layout = storage_layout.unwrap_or(materialized.storage_layout);

    prepare_database_parent(&storage_layout)?;
    probe_sqlite_create_if_missing(&storage_layout.database_path).await?;

    let app_state =
        crate::app_state::init_harness_app_state(settings.clone(), storage_layout.clone())
            .await
            .map_err(|_| TempHarnessError::AppStateInitFailed)?;
    let runtime_events = runtime_fixture_envelopes(&settings.workspace_id, runtime_event_fixtures)?;
    let fixture_manifest_id = fixture_manifest_id(seed_name);

    let handle = TempHarnessHandle {
        workspace_id: settings.workspace_id,
        database_path: storage_layout.database_path,
        fixture_manifest_id,
        app_state_ready: true,
    };

    registry()
        .lock()
        .expect("temp harness registry should not be poisoned")
        .insert(
            handle.fixture_manifest_id.clone(),
            StoredHarness {
                app_state,
                runtime_events,
            },
        );

    Ok(handle)
}

pub fn harness_app_state(handle: &TempHarnessHandle) -> Option<HarnessAppState> {
    registry()
        .lock()
        .expect("temp harness registry should not be poisoned")
        .get(&handle.fixture_manifest_id)
        .map(|stored| stored.app_state.clone())
}

pub fn replay_recorded_runtime_events(
    handle: &TempHarnessHandle,
) -> Result<usize, TempHarnessError> {
    let stored = registry()
        .lock()
        .expect("temp harness registry should not be poisoned")
        .get(&handle.fixture_manifest_id)
        .cloned()
        .ok_or(TempHarnessError::AppStateInitFailed)?;

    let count = stored.runtime_events.len();
    for event in stored.runtime_events {
        let _ = stored.app_state.event_bus.publish(event);
    }

    Ok(count)
}

pub fn attempt_real_agents_spawn(_handle: &TempHarnessHandle) -> Result<(), TempHarnessError> {
    Err(TempHarnessError::RealAgentsInvocationAttempted)
}

fn seed_registry() -> HashMap<&'static str, SeedSpec> {
    HashMap::from([
        (
            "empty",
            SeedSpec {
                manifest_relative_path: "manifests/empty.json",
            },
        ),
        (
            "minimal-runtime",
            SeedSpec {
                manifest_relative_path: "manifests/minimal-runtime.json",
            },
        ),
        (
            "missing-manifest",
            SeedSpec {
                manifest_relative_path: "manifests/missing-manifest.json",
            },
        ),
    ])
}

fn materialize_seed(
    seed_name: &str,
    seed: &SeedSpec,
) -> Result<SeedMaterialization, TempHarnessError> {
    let temp_root = unique_temp_dir(seed_name);
    let storage_root = temp_root.join("storage");
    fs::create_dir_all(&storage_root).map_err(|_| TempHarnessError::DatabaseCreateFailed)?;

    let settings = HarnessSettings {
        workspace_id: format!("workspace-{seed_name}-{}", unique_suffix()),
        storage_root: storage_root.to_string_lossy().into_owned(),
        database_path: storage_root
            .join("harness.sqlite")
            .to_string_lossy()
            .into_owned(),
        agent_runner_bin: temp_root.join("fake-agents").to_string_lossy().into_owned(),
        log_level: HarnessLogLevel::Info,
        profile_name: Some(seed_name.to_string()),
    };
    let storage_layout =
        derive_local_storage_layout(&settings).map_err(|_| TempHarnessError::AppStateInitFailed)?;

    Ok(SeedMaterialization {
        settings,
        storage_layout,
        manifest_path: fixture_root().join(seed.manifest_relative_path),
    })
}

fn prepare_database_parent(storage_layout: &LocalStorageLayout) -> Result<(), TempHarnessError> {
    let database_path = Path::new(&storage_layout.database_path);
    let parent = database_path
        .parent()
        .ok_or(TempHarnessError::DatabaseCreateFailed)?;
    fs::create_dir_all(parent).map_err(|_| TempHarnessError::DatabaseCreateFailed)
}

async fn probe_sqlite_create_if_missing(database_path: &str) -> Result<(), TempHarnessError> {
    let options = SqliteConnectOptions::new()
        .filename(Path::new(database_path))
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .map_err(|_| TempHarnessError::DatabaseCreateFailed)?;
    pool.close().await;

    Ok(())
}

fn runtime_fixture_envelopes(
    workspace_id: &str,
    runtime_event_fixtures: Vec<BackendSpanEvent>,
) -> Result<Vec<IpcEvent<Value>>, TempHarnessError> {
    runtime_event_fixtures
        .into_iter()
        .map(|payload| {
            let trace_context_id = payload.trace_context.correlation_id.clone();
            let value =
                serde_json::to_value(payload).map_err(|_| TempHarnessError::AppStateInitFailed)?;
            build_ipc_event(
                workspace_id,
                EventTopic::Runtime,
                value,
                Some(&trace_context_id),
            )
            .map_err(|_| TempHarnessError::AppStateInitFailed)
        })
        .collect()
}

fn registry() -> &'static Mutex<HashMap<String, StoredHarness>> {
    HARNESS_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a repository parent")
        .join("product-strategy/contracts/fixtures/wu-0a-14a")
}

fn unique_temp_dir(seed_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "agent-harness-wu-0a-14a-{seed_name}-{}-{}",
        std::process::id(),
        unique_suffix()
    ))
}

fn fixture_manifest_id(seed_name: &str) -> String {
    format!("manifest-{seed_name}-{}", unique_suffix())
}

fn unique_suffix() -> String {
    let counter = NEXT_TEMP_HARNESS_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    format!("{nanos}-{counter}")
}
