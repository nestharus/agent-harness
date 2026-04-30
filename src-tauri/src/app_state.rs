use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::contracts::event_topic::EventTopic;
use crate::contracts::harness_app_state::BootstrapError;
use crate::contracts::harness_settings::HarnessSettings;
use crate::contracts::ipc_event::IpcEvent;
use crate::contracts::local_storage_layout::LocalStorageLayout;
use crate::contracts::trace_context::{TraceContext, TraceContextError};
use crate::events::subscription::{Subscription, SubscriptionMetadata, WorkspaceEventChannel};
use crate::storage::validate_local_storage_layout;
use crate::tracing::trace_context::create_trace_context;

const EVENT_BUS_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub struct HarnessAppState {
    pub settings: HarnessSettings,
    pub storage_layout: LocalStorageLayout,
    pub event_bus: EventBusHandle,
    pub db: SqlitePool,
    pub trace_context_factory: TraceContextFactory,
}

#[derive(Clone)]
pub struct EventBusHandle {
    sender: broadcast::Sender<IpcEvent<Value>>,
    subscriptions: Arc<Mutex<HashMap<String, Subscription>>>,
}

impl EventBusHandle {
    pub fn new(sender: broadcast::Sender<IpcEvent<Value>>) -> Self {
        Self {
            sender,
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<IpcEvent<Value>> {
        self.sender.subscribe()
    }

    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    pub fn register_subscription(
        &self,
        workspace_id: String,
        topic: EventTopic,
        channel: WorkspaceEventChannel,
    ) -> String {
        let receiver = self.subscribe();
        let subscription = Subscription::new(workspace_id, topic, channel, receiver);
        let subscription_id = subscription.subscription_id().to_string();

        self.subscriptions
            .lock()
            .expect("event-bus subscription registry should not be poisoned")
            .insert(subscription_id.clone(), subscription);

        subscription_id
    }

    pub fn subscription_metadata(&self, subscription_id: &str) -> Option<SubscriptionMetadata> {
        self.subscriptions
            .lock()
            .expect("event-bus subscription registry should not be poisoned")
            .get(subscription_id)
            .map(Subscription::metadata)
    }

    pub fn subscription_count(&self) -> usize {
        self.subscriptions
            .lock()
            .expect("event-bus subscription registry should not be poisoned")
            .len()
    }
}

impl std::fmt::Debug for EventBusHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EventBusHandle")
            .field("receiver_count", &self.receiver_count())
            .field("subscription_count", &self.subscription_count())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct TraceContextFactory {
    workspace_id: String,
}

impl TraceContextFactory {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }

    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub fn create(
        &self,
        actor: &str,
        invocation_id: Option<&str>,
        parent_invocation_id: Option<&str>,
        graph_ref: Option<&str>,
        audit_event_ref: Option<&str>,
    ) -> Result<TraceContext, TraceContextError> {
        create_trace_context(
            &self.workspace_id,
            actor,
            invocation_id,
            parent_invocation_id,
            graph_ref,
            audit_event_ref,
        )
    }
}

pub trait EventBusFactory {
    fn create_event_bus(&self) -> Result<EventBusHandle, BootstrapError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultEventBusFactory;

impl EventBusFactory for DefaultEventBusFactory {
    fn create_event_bus(&self) -> Result<EventBusHandle, BootstrapError> {
        let (sender, _receiver) = broadcast::channel(EVENT_BUS_CAPACITY);
        Ok(EventBusHandle::new(sender))
    }
}

pub trait TraceContextFactoryInitializer {
    fn create_trace_context_factory(
        &self,
        workspace_id: &str,
    ) -> Result<TraceContextFactory, BootstrapError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultTraceContextFactoryInitializer;

impl TraceContextFactoryInitializer for DefaultTraceContextFactoryInitializer {
    fn create_trace_context_factory(
        &self,
        workspace_id: &str,
    ) -> Result<TraceContextFactory, BootstrapError> {
        if workspace_id.trim().is_empty() {
            return Err(BootstrapError::TraceInitFailed);
        }

        Ok(TraceContextFactory::new(workspace_id))
    }
}

pub async fn init_harness_app_state(
    settings: HarnessSettings,
    storage_layout: LocalStorageLayout,
) -> Result<HarnessAppState, BootstrapError> {
    init_harness_app_state_with_factories(
        settings,
        storage_layout,
        &DefaultEventBusFactory,
        &DefaultTraceContextFactoryInitializer,
    )
    .await
}

pub async fn init_harness_app_state_with_factories(
    settings: HarnessSettings,
    storage_layout: LocalStorageLayout,
    event_bus_factory: &impl EventBusFactory,
    trace_context_factory_initializer: &impl TraceContextFactoryInitializer,
) -> Result<HarnessAppState, BootstrapError> {
    validate_settings(&settings)?;
    validate_storage_layout_pair(&settings, &storage_layout)?;

    let db = open_sqlite_pool(&storage_layout.database_path).await?;
    let event_bus = event_bus_factory
        .create_event_bus()
        .map_err(|_| BootstrapError::EventBusInitFailed)?;
    let trace_context_factory = trace_context_factory_initializer
        .create_trace_context_factory(&settings.workspace_id)
        .map_err(|_| BootstrapError::TraceInitFailed)?;

    Ok(HarnessAppState {
        settings,
        storage_layout,
        event_bus,
        db,
        trace_context_factory,
    })
}

fn validate_settings(settings: &HarnessSettings) -> Result<(), BootstrapError> {
    for value in [
        &settings.workspace_id,
        &settings.storage_root,
        &settings.database_path,
        &settings.agent_runner_bin,
    ] {
        if value.trim().is_empty() {
            return Err(BootstrapError::SettingsInvalid);
        }
    }

    Ok(())
}

fn validate_storage_layout_pair(
    settings: &HarnessSettings,
    storage_layout: &LocalStorageLayout,
) -> Result<(), BootstrapError> {
    if settings.storage_root != storage_layout.storage_root {
        return Err(BootstrapError::StorageLayoutInvalid);
    }

    if settings.database_path != storage_layout.database_path {
        return Err(BootstrapError::StorageLayoutInvalid);
    }

    validate_local_storage_layout(storage_layout).map_err(|_| BootstrapError::StorageLayoutInvalid)
}

async fn open_sqlite_pool(database_path: &str) -> Result<SqlitePool, BootstrapError> {
    let options = SqliteConnectOptions::new()
        .filename(Path::new(database_path))
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .map_err(|_| BootstrapError::DatabaseOpenFailed)
}
