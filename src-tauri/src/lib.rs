pub mod app_state;
pub mod commands;
pub mod contracts;
pub mod events;
pub mod graphstore;
pub mod settings;
pub mod storage;
pub mod test_harness;
pub mod tracing;

use std::fs;

use app_state::{init_harness_app_state, HarnessAppState};
use contracts::harness_settings::{HarnessLogLevel, HarnessSettings};
use contracts::local_storage_layout::LocalStorageLayout;

const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"];

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(build_runtime_harness_app_state())
        .invoke_handler(tauri::generate_handler![
            commands::subscribe_workspace_events::subscribe_workspace_events
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

pub fn phase_0a_scaffold_commands() -> &'static [&'static str] {
    &PHASE_0A_SCAFFOLD_COMMANDS
}

pub fn registered_command_count() -> usize {
    phase_0a_scaffold_commands().len()
}

fn build_runtime_harness_app_state() -> HarnessAppState {
    let root = std::env::temp_dir().join("agent-harness-phase-0a-runtime");
    let storage_root = root.join("storage");
    fs::create_dir_all(&storage_root).expect("runtime storage root should be creatable");

    let agent_runner_bin = root.join("fake-agents");
    if !agent_runner_bin.exists() {
        fs::write(&agent_runner_bin, "#!/bin/sh\nexit 0\n")
            .expect("runtime fake agent runner should be writable");
    }

    let settings = HarnessSettings {
        workspace_id: "phase-0a-runtime".to_string(),
        storage_root: storage_root.to_string_lossy().into_owned(),
        database_path: storage_root
            .join("harness.sqlite")
            .to_string_lossy()
            .into_owned(),
        agent_runner_bin: agent_runner_bin.to_string_lossy().into_owned(),
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

    tauri::async_runtime::block_on(init_harness_app_state(settings, storage_layout))
        .expect("runtime HarnessAppState should initialize")
}

#[cfg(test)]
mod tests {
    use super::{phase_0a_scaffold_commands, registered_command_count};

    #[test]
    fn phase_0a_registers_no_value_slice_commands() {
        assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
        assert_eq!(
            registered_command_count(),
            phase_0a_scaffold_commands().len()
        );
    }
}
