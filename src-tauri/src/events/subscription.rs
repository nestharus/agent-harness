use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::ipc::Channel;
use tokio::sync::broadcast;

use crate::contracts::event_topic::EventTopic;
use crate::contracts::ipc_event::IpcEvent;

static NEXT_SUBSCRIPTION_COUNTER: AtomicU64 = AtomicU64::new(1);

pub type WorkspaceEventChannel = Channel<IpcEvent<Value>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionMetadata {
    pub subscription_id: String,
    pub workspace_id: String,
    pub topic: EventTopic,
}

pub struct Subscription {
    subscription_id: String,
    workspace_id: String,
    topic: EventTopic,
    channel: WorkspaceEventChannel,
    _receiver: broadcast::Receiver<IpcEvent<Value>>,
}

impl Subscription {
    pub fn new(
        workspace_id: String,
        topic: EventTopic,
        channel: WorkspaceEventChannel,
        receiver: broadcast::Receiver<IpcEvent<Value>>,
    ) -> Self {
        Self {
            subscription_id: next_subscription_id(),
            workspace_id,
            topic,
            channel,
            _receiver: receiver,
        }
    }

    pub fn subscription_id(&self) -> &str {
        &self.subscription_id
    }

    pub fn metadata(&self) -> SubscriptionMetadata {
        SubscriptionMetadata {
            subscription_id: self.subscription_id.clone(),
            workspace_id: self.workspace_id.clone(),
            topic: self.topic,
        }
    }
}

impl std::fmt::Debug for Subscription {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Subscription")
            .field("subscription_id", &self.subscription_id)
            .field("workspace_id", &self.workspace_id)
            .field("topic", &self.topic)
            .field("channel_id", &self.channel.id())
            .finish_non_exhaustive()
    }
}

fn next_subscription_id() -> String {
    let counter = NEXT_SUBSCRIPTION_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();

    format!("sub-{nanos}-{counter}")
}
