use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::harness_settings::HarnessSettings;
use agent_harness_lib::contracts::local_storage_layout::{LocalStorageLayout, StorageLayoutError};
use agent_harness_lib::storage::{derive_local_storage_layout, validate_local_storage_layout};
use serde::Deserialize;
use serde_json::Value;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0a-03";

#[derive(Debug, Deserialize)]
struct DeriveSuccessCase {
    settings: HarnessSettings,
    expected: LocalStorageLayout,
}

#[derive(Debug, Deserialize)]
struct DeriveErrorCase {
    name: String,
    settings: HarnessSettings,
    expected_error: StorageLayoutError,
}

#[derive(Debug, Deserialize)]
struct InvalidLayoutCase {
    name: String,
    layout: LocalStorageLayout,
    expected_error: StorageLayoutError,
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

#[test]
fn local_storage_layout_shape_matches_canonical_fixture() {
    // Risk: Rust/TS contract drift. Level: particular-integration. Source:
    // proposal test-intent "Rust DTO shape and serde round-trip".
    let canonical = read_json(fixture_path("canonical-layout.json"));
    let layout: LocalStorageLayout =
        serde_json::from_value(canonical.clone()).expect("canonical layout must deserialize");

    assert_eq!(
        serde_json::to_value(&layout).expect("layout must serialize"),
        canonical
    );

    let mut fields = canonical
        .as_object()
        .expect("canonical layout must be an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    fields.sort();
    assert_eq!(
        fields,
        vec![
            "database_path",
            "evidence_root",
            "fixture_root",
            "log_root",
            "storage_root",
            "temp_root"
        ]
    );
}

#[test]
fn derive_layout_preserves_database_path_and_derives_distinct_child_roots() {
    // Risk: broken path derivation. Level: unit. Source: proposal test-intent
    // "Rust derivation success".
    let case: DeriveSuccessCase =
        serde_json::from_value(read_json(fixture_path("derive-success.json")))
            .expect("derive success fixture must deserialize");

    let layout = derive_local_storage_layout(&case.settings)
        .expect("documented settings should derive a storage layout");

    assert_eq!(layout, case.expected);
    assert_eq!(layout.storage_root, case.settings.storage_root);
    assert_eq!(layout.database_path, case.settings.database_path);

    let child_roots = [
        &layout.evidence_root,
        &layout.fixture_root,
        &layout.log_root,
        &layout.temp_root,
    ];
    assert_eq!(
        child_roots.iter().copied().collect::<HashSet<_>>().len(),
        child_roots.len(),
        "derived child roots must be distinct"
    );
    for child_root in child_roots {
        assert!(
            Path::new(child_root).starts_with(&layout.storage_root),
            "{child_root} must be under storage root"
        );
    }
}

#[test]
fn derive_layout_rejects_documented_settings_errors() {
    // Risk: unreachable or renamed error surface. Level: unit. Source:
    // proposal test-intent "Rust error reachability".
    let cases: Vec<DeriveErrorCase> =
        serde_json::from_value(read_json(fixture_path("derive-errors.json")))
            .expect("derive error fixture must deserialize");

    for case in cases {
        assert_eq!(
            derive_local_storage_layout(&case.settings),
            Err(case.expected_error),
            "{} should reach its documented StorageLayoutError",
            case.name
        );
    }
}

#[test]
fn configured_layout_validation_rejects_paths_outside_storage_root() {
    // Risk: unreachable child-root containment errors. Level: unit. Source:
    // proposal test-intent "Rust error reachability".
    let cases: Vec<InvalidLayoutCase> =
        serde_json::from_value(read_json(fixture_path("invalid-layouts.json")))
            .expect("invalid layout fixture must deserialize");

    for case in cases {
        assert_eq!(
            validate_local_storage_layout(&case.layout),
            Err(case.expected_error),
            "{} should reach its documented StorageLayoutError",
            case.name
        );
    }
}

#[test]
fn storage_layout_error_variants_round_trip_and_unknown_variant_is_rejected() {
    // Risk: Rust/TS error taxonomy drift. Level: particular-integration.
    // Source: WU acceptance criteria and contract error fixtures.
    let expected_names = vec![
        "StorageRootEscapesWorkspace",
        "DatabasePathOutsideStorageRoot",
        "EvidenceRootOutsideStorageRoot",
        "FixtureRootOutsideStorageRoot",
        "LogRootOutsideStorageRoot",
        "TempRootOutsideStorageRoot",
    ];
    let fixture_names: Vec<String> =
        serde_json::from_value(read_json(fixture_path("storage-layout-errors.json")))
            .expect("storage layout errors fixture must be a string array");

    assert_eq!(fixture_names, expected_names);

    for name in fixture_names {
        let error: StorageLayoutError =
            serde_json::from_value(Value::String(name.clone())).expect("error variant must parse");
        assert_eq!(
            serde_json::to_value(error).expect("error variant must serialize"),
            Value::String(name)
        );
    }

    assert!(
        serde_json::from_value::<StorageLayoutError>(read_json(fixture_path(
            "invalid-storage-layout-error.json",
        )))
        .is_err(),
        "unknown StorageLayoutError variants must be rejected"
    );
}
