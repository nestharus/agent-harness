use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::contracts::harness_settings::{HarnessLogLevel, HarnessSettings, SettingsError};

const ENV_WORKSPACE_ID: &str = "HARNESS_WORKSPACE_ID";
const ENV_STORAGE_ROOT: &str = "HARNESS_STORAGE_ROOT";
const ENV_DATABASE_PATH: &str = "HARNESS_DATABASE_PATH";
const ENV_AGENT_RUNNER_BIN: &str = "HARNESS_AGENT_RUNNER_BIN";
const ENV_LOG_LEVEL: &str = "HARNESS_LOG_LEVEL";
const ENV_PROFILE_NAME: &str = "HARNESS_PROFILE_NAME";

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawHarnessSettings {
    workspace_id: Option<String>,
    storage_root: Option<String>,
    database_path: Option<String>,
    agent_runner_bin: Option<String>,
    log_level: Option<String>,
    profile_name: Option<String>,
}

pub fn load_harness_settings(
    cwd: impl AsRef<Path>,
    env_overrides: &HashMap<String, String>,
    config_file: Option<&str>,
) -> Result<HarnessSettings, SettingsError> {
    let cwd = cwd.as_ref();
    let mut raw = match config_file {
        Some(config_file) => read_config_file(cwd, config_file)?,
        None => RawHarnessSettings::default(),
    };

    apply_env_overrides(&mut raw, env_overrides);
    validate_settings(cwd, raw)
}

fn read_config_file(cwd: &Path, config_file: &str) -> Result<RawHarnessSettings, SettingsError> {
    let path = resolve_path(cwd, config_file);
    let content = fs::read_to_string(path).map_err(|_| SettingsError::ConfigFileUnreadable)?;
    serde_json::from_str(&content).map_err(|_| SettingsError::ConfigFileUnreadable)
}

fn apply_env_overrides(raw: &mut RawHarnessSettings, env_overrides: &HashMap<String, String>) {
    for (key, value) in env_overrides {
        match key.as_str() {
            ENV_WORKSPACE_ID => raw.workspace_id = Some(value.clone()),
            ENV_STORAGE_ROOT => raw.storage_root = Some(value.clone()),
            ENV_DATABASE_PATH => raw.database_path = Some(value.clone()),
            ENV_AGENT_RUNNER_BIN => raw.agent_runner_bin = Some(value.clone()),
            ENV_LOG_LEVEL => raw.log_level = Some(value.clone()),
            ENV_PROFILE_NAME => raw.profile_name = Some(value.clone()),
            _ => {}
        }
    }
}

fn validate_settings(
    cwd: &Path,
    raw: RawHarnessSettings,
) -> Result<HarnessSettings, SettingsError> {
    let workspace_id = required_string(raw.workspace_id, SettingsError::EmptyWorkspaceId)?;
    let storage_root = required_string(raw.storage_root, SettingsError::EmptyStorageRoot)?;
    let database_path = required_string(raw.database_path, SettingsError::EmptyDatabasePath)?;
    let agent_runner_bin =
        required_string(raw.agent_runner_bin, SettingsError::MissingAgentRunnerBin)?;

    let agent_runner_path = resolve_path(cwd, &agent_runner_bin);
    if !agent_runner_path.is_file() {
        return Err(SettingsError::MissingAgentRunnerBin);
    }

    let log_level = parse_log_level(raw.log_level.as_deref())?;

    Ok(HarnessSettings {
        workspace_id,
        storage_root,
        database_path,
        agent_runner_bin,
        log_level,
        profile_name: raw.profile_name,
    })
}

fn required_string(value: Option<String>, error: SettingsError) -> Result<String, SettingsError> {
    let value = value.unwrap_or_default();
    if value.trim().is_empty() {
        return Err(error);
    }
    Ok(value)
}

fn parse_log_level(value: Option<&str>) -> Result<HarnessLogLevel, SettingsError> {
    match value {
        Some("trace") => Ok(HarnessLogLevel::Trace),
        Some("debug") => Ok(HarnessLogLevel::Debug),
        Some("info") => Ok(HarnessLogLevel::Info),
        Some("warn") => Ok(HarnessLogLevel::Warn),
        Some("error") => Ok(HarnessLogLevel::Error),
        _ => Err(SettingsError::InvalidLogLevel),
    }
}

fn resolve_path(cwd: &Path, value: &str) -> PathBuf {
    let path = Path::new(value);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}
