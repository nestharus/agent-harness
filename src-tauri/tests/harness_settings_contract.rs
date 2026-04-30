use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::harness_settings::{
    HarnessLogLevel, HarnessSettings, SettingsError,
};
use agent_harness_lib::settings::load_harness_settings;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-02";

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

fn read_env_overrides(value: &Value) -> HashMap<String, String> {
    serde_json::from_value(value.clone()).expect("env_overrides fixture must be a string map")
}

#[test]
fn harness_settings_shape_matches_canonical_fixture() {
    // Risk: Rust/TS contract drift. Level: particular-integration. Source:
    // proposal test-intent "Rust DTO shape and serde round-trip".
    let canonical = read_json(fixture_path("canonical-settings.json"));
    let settings: HarnessSettings =
        serde_json::from_value(canonical.clone()).expect("canonical settings must deserialize");

    assert_eq!(
        serde_json::to_value(&settings).expect("settings must serialize"),
        canonical
    );

    let fields = canonical
        .as_object()
        .expect("canonical settings must be an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(
        fields,
        vec![
            "agent_runner_bin",
            "database_path",
            "log_level",
            "profile_name",
            "storage_root",
            "workspace_id"
        ]
    );
}

#[test]
fn load_harness_settings_merges_config_file_and_env_overrides() {
    // Risk: merge behavior ambiguity. Level: unit/particular-integration.
    // Source: proposal test-intent "Rust loader success".
    let root = repo_root();
    let env_overrides = read_env_overrides(&read_json(fixture_path("env-overrides.json")));
    let loaded = load_harness_settings(
        &root,
        &env_overrides,
        Some("product-strategy/contracts/fixtures/wu-0a-02/config-input.json"),
    )
    .expect("fixture settings should load");
    let expected: HarnessSettings =
        serde_json::from_value(read_json(fixture_path("env-overridden-settings.json")))
            .expect("expected settings fixture must deserialize");

    assert_eq!(loaded, expected);
}

#[test]
fn settings_error_variants_are_reachable_through_documented_inputs() {
    // Risk: unreachable or renamed error surface. Level: unit. Source:
    // proposal test-intent "Rust error reachability".
    let root = repo_root();

    for fixture_name in [
        "error-empty-workspace-id.json",
        "error-empty-storage-root.json",
        "error-empty-database-path.json",
        "error-missing-agent-runner-bin-empty.json",
        "error-missing-agent-runner-bin-unresolved.json",
        "error-invalid-log-level.json",
        "error-config-file-unreadable.json",
    ] {
        let case = read_json(fixture_path(fixture_name));
        let env_overrides = read_env_overrides(
            case.get("env_overrides")
                .expect("case fixture must include env_overrides"),
        );
        let config_file = case.get("config_file").and_then(Value::as_str);
        let expected_error: SettingsError = serde_json::from_value(
            case.get("expected_error")
                .expect("case fixture must include expected_error")
                .clone(),
        )
        .expect("expected error fixture must deserialize");

        let result = load_harness_settings(&root, &env_overrides, config_file);
        assert_eq!(
            result,
            Err(expected_error),
            "{fixture_name} should reach its documented SettingsError"
        );
    }
}

#[test]
fn log_level_union_is_exactly_documented_set() {
    // Risk: widened log-level type. Level: unit/typecheck. Source: documented
    // log-level union and proposal test-intent.
    for (raw, expected) in [
        ("trace", HarnessLogLevel::Trace),
        ("debug", HarnessLogLevel::Debug),
        ("info", HarnessLogLevel::Info),
        ("warn", HarnessLogLevel::Warn),
        ("error", HarnessLogLevel::Error),
    ] {
        let parsed: HarnessLogLevel =
            serde_json::from_value(Value::String(raw.to_string())).expect("log level must parse");
        assert_eq!(parsed, expected);
        assert_eq!(
            serde_json::to_value(parsed).expect("log level must serialize"),
            Value::String(raw.to_string())
        );
    }

    assert!(
        serde_json::from_value::<HarnessLogLevel>(Value::String("verbose".to_string())).is_err(),
        "unknown log levels must be rejected"
    );
}

#[test]
fn settings_error_variants_round_trip_and_unknown_variant_is_rejected() {
    // Risk: Rust/TS error taxonomy drift. Level: particular-integration.
    // Source: WU acceptance criteria and contract error fixtures.
    let expected_names = vec![
        "EmptyWorkspaceId",
        "EmptyStorageRoot",
        "EmptyDatabasePath",
        "MissingAgentRunnerBin",
        "InvalidLogLevel",
        "ConfigFileUnreadable",
    ];
    let fixture = read_json(fixture_path("settings-errors.json"));
    let fixture_names: Vec<String> =
        serde_json::from_value(fixture).expect("settings errors fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: SettingsError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<SettingsError>(read_json(fixture_path(
            "invalid-settings-error.json",
        )))
        .is_err(),
        "unknown SettingsError variants must be rejected"
    );
}
