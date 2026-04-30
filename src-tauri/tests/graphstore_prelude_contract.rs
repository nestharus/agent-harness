use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::graphstore_prelude::{
    validate_record_meta, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use serde_json::Value;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, Sqlite, Type};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-02";

#[derive(Debug)]
struct ContractOpaqueMarker;

#[derive(Debug, Deserialize)]
struct JsonRoundTripFixture {
    cases: Vec<JsonRoundTripCase>,
}

#[derive(Debug, Deserialize)]
struct JsonRoundTripCase {
    name: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
struct ErrorFixtureIndex {
    errors: Vec<ErrorFixtureEntry>,
}

#[derive(Debug, Deserialize)]
struct ErrorFixtureEntry {
    fixture: String,
    variant: GraphStoreError,
    code: String,
}

#[derive(Debug, Deserialize)]
struct ErrorFixture {
    variant: GraphStoreError,
    code: String,
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

fn read_fixture<T: for<'de> Deserialize<'de>>(name: &str) -> T {
    let path = fixture_path(name);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("fixture {} must parse: {error}", path.display()))
}

fn assert_invariant_violation(error: GraphStoreError) {
    assert_eq!(error, GraphStoreError::InvariantViolation);
    assert_eq!(error.code(), "invariant_violation");
}

fn assert_error_matches_fixture(error: GraphStoreError, fixture: &ErrorFixture) {
    assert_eq!(error, fixture.variant);
    assert_eq!(error.code(), fixture.code);
}

#[test]
fn record_meta_validation_preserves_canonical_and_rejects_invalid_inputs() {
    // Risk: later rows accept invalid shared metadata. Level: unit/particular-integration.
    // Source: WU-0B-02 proposal test intent "RecordMeta validation".
    let canonical: RecordMeta = read_fixture("record-meta-canonical.json");
    let normalized = validate_record_meta(canonical.clone()).expect("canonical meta validates");
    assert_eq!(normalized, canonical);

    for fixture in [
        "record-meta-invalid-updated-before-created.json",
        "record-meta-invalid-empty-actor.json",
        "record-meta-invalid-empty-policy-version.json",
    ] {
        let meta: RecordMeta = read_fixture(fixture);
        let error = validate_record_meta(meta).expect_err("invalid metadata is rejected");
        assert_invariant_violation(error);
    }
}

#[test]
fn opaque_ids_preserve_valid_values_and_reject_documented_invalid_patterns() {
    // Risk: stable IDs become paths or hashes. Level: unit. Source: WU-0B-02 proposal
    // test intent "OpaqueId rejection".
    let valid: OpaqueId<ContractOpaqueMarker> = read_fixture("opaque-id-valid.json");
    let constructed =
        OpaqueId::<ContractOpaqueMarker>::new(valid.value.clone(), valid.namespace.clone())
            .expect("valid opaque id should construct");
    assert_eq!(constructed, valid);

    for fixture in [
        "opaque-id-invalid-path-separator.json",
        "opaque-id-invalid-content-hash-prefix.json",
        "opaque-id-invalid-empty-namespace.json",
        "opaque-id-invalid-parent-path.json",
    ] {
        let id: OpaqueId<ContractOpaqueMarker> = read_fixture(fixture);
        let error = OpaqueId::<ContractOpaqueMarker>::new(id.value, id.namespace)
            .expect_err("invalid opaque id should be rejected");
        assert_invariant_violation(error);
    }
}

#[test]
fn every_graphstore_error_variant_has_a_stable_machine_code_fixture() {
    // Risk: caller-visible error drift. Level: unit. Source: WU-0B-02 proposal test
    // intent "Error taxonomy".
    let index: ErrorFixtureIndex = read_fixture("graphstore-errors.json");
    assert_eq!(index.errors.len(), 7);

    for entry in index.errors {
        let fixture: ErrorFixture = read_fixture(&entry.fixture);
        assert_eq!(fixture.variant, entry.variant);
        assert_eq!(fixture.code, entry.code);
        assert_error_matches_fixture(entry.variant, &fixture);
    }

    assert_error_matches_fixture(
        GraphStoreError::UnknownRef,
        &read_fixture("error-unknown-ref.json"),
    );
    assert_error_matches_fixture(
        GraphStoreError::DuplicateId,
        &read_fixture("error-duplicate-id.json"),
    );
    assert_error_matches_fixture(
        GraphStoreError::InvalidEnum,
        &read_fixture("error-invalid-enum.json"),
    );
    assert_error_matches_fixture(
        GraphStoreError::InvalidTransition,
        &read_fixture("error-invalid-transition.json"),
    );
    assert_error_matches_fixture(
        GraphStoreError::InvariantViolation,
        &read_fixture("error-invariant-violation.json"),
    );
    assert_error_matches_fixture(
        GraphStoreError::OptimisticConflict,
        &read_fixture("error-optimistic-conflict.json"),
    );
    assert_error_matches_fixture(
        GraphStoreError::SqlxFailure,
        &read_fixture("error-sqlx-failure.json"),
    );
}

#[tokio::test]
async fn jsonfield_round_trips_documented_json_shapes_through_sqlite_text() {
    // Risk: lossy JSON stringification or missing SQLx codec. Level:
    // particular-integration. Source: WU-0B-02 proposal test intent "JsonField SQLx
    // round-trip".
    let fixture: JsonRoundTripFixture = read_fixture("jsonfield-round-trips.json");
    assert_eq!(fixture.cases.len(), 6);
    let jsonfield_type = <JsonField<Value> as Type<Sqlite>>::type_info();
    assert!(<JsonField<Value> as Type<Sqlite>>::compatible(
        &jsonfield_type
    ));

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory sqlite pool should connect");
    sqlx::query("CREATE TABLE jsonfield_contract (name TEXT PRIMARY KEY, body TEXT NOT NULL)")
        .execute(&pool)
        .await
        .expect("contract table should be creatable");

    for case in &fixture.cases {
        sqlx::query("INSERT INTO jsonfield_contract (name, body) VALUES (?, ?)")
            .bind(&case.name)
            .bind(JsonField(case.value.clone()))
            .execute(&pool)
            .await
            .expect("json fixture should insert through JsonField Encode");
    }

    for case in fixture.cases {
        let row = sqlx::query("SELECT body FROM jsonfield_contract WHERE name = ?")
            .bind(&case.name)
            .fetch_one(&pool)
            .await
            .expect("json fixture should be readable");
        let decoded: JsonField<Value> = row
            .try_get("body")
            .expect("json fixture should decode through JsonField Decode");
        assert_eq!(
            decoded.0, case.value,
            "round-trip mismatch for {}",
            case.name
        );
    }
}

#[tokio::test]
async fn invalid_timestamp_order_is_rejected_before_insert() {
    // Risk: invalid historical rows. Level: unit/particular-integration. Source:
    // WU-0B-02 proposal test intent "Timestamp ordering".
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory sqlite pool should connect");
    sqlx::query(
        "CREATE TABLE record_meta_attempts (
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            actor TEXT NOT NULL,
            policy_version TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("test table should be creatable");

    let invalid: RecordMeta = read_fixture("record-meta-invalid-updated-before-created.json");
    let error = validate_record_meta(invalid.clone()).expect_err("invalid order is rejected");
    assert_invariant_violation(error);

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM record_meta_attempts")
        .fetch_one(&pool)
        .await
        .expect("test table should be readable");
    assert_eq!(count.0, 0);
}

#[tokio::test]
async fn prelude_exports_no_durable_domain_table_or_value_slice_command() {
    // Risk: type prelude leaks schema or commands. Level: particular-integration/static.
    // Source: WU-0B-02 proposal test intent "Domain-table absence and command stability".
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory sqlite pool should connect");
    let fixture: JsonRoundTripFixture = read_fixture("jsonfield-round-trips.json");

    sqlx::query("CREATE TEMP TABLE jsonfield_temp (body TEXT NOT NULL)")
        .execute(&pool)
        .await
        .expect("temp table should be creatable for codec exercise");
    sqlx::query("INSERT INTO jsonfield_temp (body) VALUES (?)")
        .bind(JsonField(fixture.cases[0].value.clone()))
        .execute(&pool)
        .await
        .expect("codec exercise should insert into temp table");

    let tables: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .fetch_all(&pool)
            .await
            .expect("sqlite_master should be readable");
    assert_eq!(tables, Vec::<(String,)>::new());
    assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
}
