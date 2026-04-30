use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::harness_command::{
    parse_harness_command, HarnessCommand, HarnessCommandError,
};
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-07";

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

fn command_string(command: HarnessCommand) -> String {
    serde_json::from_value::<String>(serde_json::to_value(command).expect("command must serialize"))
        .expect("serialized command must be a string")
}

#[test]
fn harness_command_variants_round_trip_through_canonical_names() {
    // Risk: Rust command-name drift. Level: particular-integration. Source:
    // proposal test-intent "Rust command taxonomy and serde round-trip".
    let fixture_commands: Vec<String> =
        serde_json::from_value(read_json(fixture_path("command-names.json")))
            .expect("command names fixture must be a string array");
    let expected = [
        HarnessCommand::GetHarnessSettings,
        HarnessCommand::SubscribeWorkspaceEvents,
        HarnessCommand::PingRuntime,
    ];

    assert_eq!(
        expected
            .iter()
            .copied()
            .map(command_string)
            .collect::<Vec<_>>(),
        fixture_commands
    );

    for raw in fixture_commands {
        let command: HarnessCommand = serde_json::from_value(Value::String(raw.clone()))
            .expect("documented command must deserialize");
        assert_eq!(
            serde_json::to_value(command).expect("command must serialize"),
            Value::String(raw.clone())
        );
        assert_eq!(
            parse_harness_command(&raw),
            Ok(command),
            "{raw} should parse to its documented HarnessCommand"
        );
    }
}

#[test]
fn harness_command_rejects_alternate_and_unknown_names() {
    // Risk: alternate command names accidentally become supported. Level:
    // particular-integration. Source: proposal test-intent "Rust command
    // taxonomy and serde round-trip".
    let alternate_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("alternate-command-names.json")))
            .expect("alternate command names fixture must be a string array");

    for raw in alternate_names {
        assert_eq!(
            parse_harness_command(&raw),
            Err(HarnessCommandError::UnknownCommand),
            "{raw:?} should reach UnknownCommand"
        );
        assert!(
            serde_json::from_value::<HarnessCommand>(Value::String(raw.clone())).is_err(),
            "{raw:?} should fail serde deserialization"
        );
    }
}
