use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::phase_0a_scaffold_commands;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-01";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a repository parent")
        .to_path_buf()
}

fn read_json(path: impl AsRef<Path>) -> Value {
    let content = fs::read_to_string(path.as_ref())
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.as_ref().display()));
    serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.as_ref().display()))
}

fn assert_json_subset(expected: &Value, actual: &Value, path: &str) {
    match (expected, actual) {
        (Value::Object(expected_map), Value::Object(actual_map)) => {
            for (key, expected_value) in expected_map {
                let next_path = format!("{path}.{key}");
                let actual_value = actual_map
                    .get(key)
                    .unwrap_or_else(|| panic!("missing key {next_path}"));
                assert_json_subset(expected_value, actual_value, &next_path);
            }
        }
        (Value::Array(expected_items), Value::Array(actual_items)) => {
            assert_eq!(expected_items, actual_items, "array mismatch at {path}");
        }
        _ => {
            assert_eq!(expected, actual, "value mismatch at {path}");
        }
    }
}

#[test]
fn workspace_manifests_match_phase_0a_contract_fixtures() {
    // Risk: workspace/task drift. Level: particular-integration. Source: proposal test-intent
    // "Contract fixture shape" and contract fixture files.
    let root = repo_root();
    let fixture_dir = root.join(CONTRACT_FIXTURE_DIR);

    let expected_package = read_json(fixture_dir.join("package.shape.json"));
    let actual_package = read_json(root.join("package.json"));
    assert_json_subset(&expected_package, &actual_package, "package.json");

    let expected_turbo = read_json(fixture_dir.join("turbo.shape.json"));
    let actual_turbo = read_json(root.join("turbo.json"));
    assert_json_subset(&expected_turbo, &actual_turbo, "turbo.json");

    let expected_tauri = read_json(fixture_dir.join("tauri.conf.shape.json"));
    let actual_tauri = read_json(root.join("src-tauri/tauri.conf.json"));
    assert_json_subset(&expected_tauri, &actual_tauri, "src-tauri/tauri.conf.json");
}

#[test]
fn cargo_manifest_declares_phase_0a_runtime_dependencies_without_sqlx_migrate_feature() {
    // Risk: backend scaffold drift or accidental schema ownership. Level:
    // particular-integration. Source: proposal assumptions A3 and A4.
    let root = repo_root();
    let cargo_toml = fs::read_to_string(root.join("src-tauri/Cargo.toml"))
        .expect("src-tauri/Cargo.toml must exist");
    let parsed: toml::Value = toml::from_str(&cargo_toml).expect("Cargo.toml must parse");
    let dependencies = parsed
        .get("dependencies")
        .and_then(toml::Value::as_table)
        .expect("Cargo.toml dependencies table must exist");

    assert_eq!(
        dependencies
            .get("tauri")
            .and_then(toml::Value::as_table)
            .and_then(|value| value.get("version"))
            .and_then(toml::Value::as_str),
        Some("2.10.3")
    );
    assert_eq!(
        dependencies
            .get("tokio")
            .and_then(toml::Value::as_table)
            .and_then(|value| value.get("version"))
            .and_then(toml::Value::as_str),
        Some("1.52.1")
    );

    let sqlx = dependencies
        .get("sqlx")
        .and_then(toml::Value::as_table)
        .expect("sqlx dependency must be declared as a table");
    assert_eq!(
        sqlx.get("version").and_then(toml::Value::as_str),
        Some("0.8.6")
    );
    assert_eq!(
        sqlx.get("default-features").and_then(toml::Value::as_bool),
        Some(false)
    );
    let features = sqlx
        .get("features")
        .and_then(toml::Value::as_array)
        .expect("sqlx features must be explicit");
    for required_feature in ["sqlite", "runtime-tokio", "macros"] {
        assert!(
            features
                .iter()
                .any(|feature| feature.as_str() == Some(required_feature)),
            "sqlx must enable {required_feature}"
        );
    }
    assert!(
        !features
            .iter()
            .any(|feature| feature.as_str() == Some("migrate")),
        "WU-0A-01 must not enable sqlx migration ownership"
    );

    let migrations_dir = root.join("src-tauri/migrations");
    if migrations_dir.exists() {
        let entries: Vec<String> = fs::read_dir(&migrations_dir)
            .expect("migrations placeholder must be readable")
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            entries,
            vec!["0001_schema_versions.sql", "0b"],
            "Phase 0B migrations may include the schema_versions bootstrap and bounded WU subdirectories"
        );
    }
}

#[test]
fn tauri_bootstrap_is_inert_and_command_free() {
    // Risk: value-slice command leakage. Level: particular-integration. Source:
    // WU-0A-08 scaffold command allowlist and Tauri bootstrap contract.
    let root = repo_root();
    let main_rs = root.join("src-tauri/src/main.rs");
    let lib_rs = root.join("src-tauri/src/lib.rs");

    assert!(main_rs.exists(), "main.rs must exist");
    assert!(lib_rs.exists(), "lib.rs must exist");

    let main_content = fs::read_to_string(main_rs).expect("main.rs must be readable");
    assert!(
        main_content.contains("agent_harness_lib::run()"),
        "main.rs must delegate to the library bootstrap"
    );

    let lib_content = fs::read_to_string(lib_rs).expect("lib.rs must be readable");
    assert!(
        lib_content.contains("pub fn run()"),
        "lib.rs must expose a run bootstrap"
    );
    assert!(
        lib_content.contains("registered_command_count() -> usize"),
        "lib.rs must expose command-count evidence for the inert scaffold"
    );

    let scaffold_commands = phase_0a_scaffold_commands();
    assert_eq!(scaffold_commands, ["subscribe_workspace_events"]);
    assert!(
        lib_content.contains("tauri::generate_handler!["),
        "Phase 0A must register its documented scaffold command handler"
    );
    assert!(
        lib_content.contains("commands::subscribe_workspace_events::subscribe_workspace_events"),
        "subscribe_workspace_events must be the registered scaffold command"
    );

    for forbidden_command in ["get_harness_settings", "ping_runtime"] {
        assert!(
            !lib_content.contains(&format!("commands::{forbidden_command}")),
            "Phase 0A scaffold must not register value-slice command {forbidden_command}"
        );
    }
    assert!(
        !lib_content.contains("generate_handler![]"),
        "Phase 0A scaffold command registration must not be empty"
    );
}
