use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootstrapError {
    SettingsInvalid,
    StorageLayoutInvalid,
    DatabaseOpenFailed,
    EventBusInitFailed,
    TraceInitFailed,
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        formatter.write_str(value.trim_matches('"'))
    }
}

impl std::error::Error for BootstrapError {}
