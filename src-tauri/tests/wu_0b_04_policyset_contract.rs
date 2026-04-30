use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};
use agent_harness_lib::contracts::policyset::{PolicySet, PolicyVersionRef};
use agent_harness_lib::graphstore::fixture::{GraphStoreRepo, GraphWorkspaceRef};
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";

#[derive(Debug, Deserialize)]
struct RoundTripFixture {
    row: PolicySet,
}

#[derive(Debug, Deserialize)]
struct RejectionMatrixFixture {
    expected_error: String,
    modes: Vec<String>,
    fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AppendOnlyFixture {
    rows: Vec<PolicySet>,
}

#[derive(Debug, Deserialize)]
struct NoSideEffectsFixture {
    phase_0a_scaffold_commands: Vec<String>,
    allowed_runtime_tables: Vec<String>,
    forbidden_source_tokens: Vec<String>,
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

async fn empty_policyset_fixture() -> (SqlitePool, PolicySetRepo) {
    let fixture = create_graphstore_fixture(GraphStoreSeedPlan::default())
        .await
        .expect("empty graphstore fixture should apply migrations");
    assert_eq!(
        fixture.pool.migrations_applied,
        vec![1, 4, 5, 6, 7, 9, 15, 16]
    );
    let pool = fixture.pool.sqlite;
    let repo = PolicySetRepo::new(pool.clone());
    (pool, repo)
}

async fn policyset_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM policy_sets")
        .fetch_one(pool)
        .await
        .expect("policy_sets row count should be readable");
    row.0
}

async fn sqlite_table_names(pool: &SqlitePool) -> Vec<String> {
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .fetch_all(pool)
            .await
            .expect("sqlite_master should be readable");
    rows.into_iter()
        .map(|(name,)| name)
        .filter(|name| !name.starts_with("sqlite_"))
        .collect()
}

fn policyset_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("policyset.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn policyset_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_04_policyset_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn canonical_policy_version(field: &str) -> PolicyVersionRef {
    let policy = match field {
        "summary_contract_version" => "summary_contract",
        "render_policy_version" => "render",
        "identity_policy_version" => "identity",
        "privilege_policy_version" => "privilege",
        "tool_protocol_policy_version" => "tool_protocol",
        "budget_policy_version" => "budget",
        "review_sampling_policy_version" => "review_sampling",
        "recovery_policy_version" => "recovery",
        "configuration_policy_version" => "configuration",
        "provider_policy_version" => "provider",
        _ => panic!("unexpected policy version field {field}"),
    };

    PolicyVersionRef {
        policy: policy.to_string(),
        version: "v1.1.0".to_string(),
        previous_version: Some("v1.0.0".to_string()),
        payload: serde_json::json!({ "fixture": field }),
    }
}

fn invalid_policy_version(field: &str, mode: &str) -> PolicyVersionRef {
    let mut value = canonical_policy_version(field);
    match mode {
        "empty" => value.version = " ".to_string(),
        "unknown" => value.policy = "unknown_policy".to_string(),
        "non_monotonic" => value.version = "v1.0.0".to_string(),
        "malformed" => value.version = "1.1".to_string(),
        _ => panic!("unexpected rejection mode {mode}"),
    }
    value
}

fn set_policy_version(record: &mut PolicySet, field: &str, value: PolicyVersionRef) {
    match field {
        "summary_contract_version" => record.summary_contract_version.0 = value,
        "render_policy_version" => record.render_policy_version.0 = value,
        "identity_policy_version" => record.identity_policy_version.0 = value,
        "privilege_policy_version" => record.privilege_policy_version.0 = value,
        "tool_protocol_policy_version" => record.tool_protocol_policy_version.0 = value,
        "budget_policy_version" => record.budget_policy_version.0 = value,
        "review_sampling_policy_version" => record.review_sampling_policy_version.0 = value,
        "recovery_policy_version" => record.recovery_policy_version.0 = value,
        "configuration_policy_version" => record.configuration_policy_version.0 = value,
        "provider_policy_version" => record.provider_policy_version.0 = value,
        _ => panic!("unexpected policy version field {field}"),
    }
}

fn unique_policy_set_id(base: &PolicySet, field: &str, mode: &str) -> PolicySet {
    let mut record = base.clone();
    record.policy_set_id.value = format!("policy-set-invalid-{field}-{mode}");
    record
}

fn assert_invariant_violation(error: GraphStoreError) {
    assert_eq!(error, GraphStoreError::InvariantViolation);
    assert_eq!(error.code(), "invariant_violation");
}

#[tokio::test]
async fn policy_sets_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: first durable domain table misses columns, JSON checks, FK, unique,
    // or indexes. Level: particular-integration. Source:
    // proposals/0b-04-wu-0b-04.md test intent "Schema creation".
    let (pool, _repo) = empty_policyset_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'policy_sets'",
    )
    .fetch_one(&pool)
    .await
    .expect("policy_sets DDL should exist");

    for column in [
        "policy_set_id_value",
        "policy_set_id_namespace",
        "summary_contract_version",
        "render_policy_version",
        "identity_policy_version",
        "privilege_policy_version",
        "tool_protocol_policy_version",
        "budget_policy_version",
        "review_sampling_policy_version",
        "recovery_policy_version",
        "configuration_policy_version",
        "provider_policy_version",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "policy_sets DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(policy_set_id_value, policy_set_id_namespace)"));
    assert!(sql.0.contains("json_valid(summary_contract_version)"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(policy_sets)")
        .fetch_all(&pool)
        .await
        .expect("foreign keys should be introspectable");
    assert!(
        fks.iter()
            .any(|row| row.get::<String, _>("table") == "schema_versions"
                && row.get::<String, _>("from") == "schema_version"
                && row.get::<String, _>("to") == "version"),
        "policy_sets should reference schema_versions(version)"
    );

    let indexes = sqlx::query("PRAGMA index_list(policy_sets)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    assert!(index_names
        .iter()
        .any(|name| name == "idx_policy_sets_policy_set_id"));
    assert!(index_names
        .iter()
        .any(|name| name == "idx_policy_sets_namespace"));
    assert!(
        indexes.iter().any(|row| row.get::<i64, _>("unique") == 1),
        "policy_sets should expose a unique policy_set_id index"
    );
}

#[tokio::test]
async fn insert_then_get_round_trips_every_policyset_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, JSON, opaque-ID, timestamp, or
    // metadata fields. Level: particular-integration. Source:
    // proposals/0b-04-wu-0b-04.md test intent "Round-trip".
    let (_pool, repo) = empty_policyset_fixture().await;
    let fixture: RoundTripFixture = read_fixture("round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_policy_set(fixture.row.clone())
        .await
        .expect("valid policy set should insert");
    let fetched = repo
        .get_policy_set(&fixture.row.policy_set_id)
        .await
        .expect("policy set get should succeed")
        .expect("inserted policy set should be returned");

    assert_eq!(
        serde_json::to_string(&inserted).expect("inserted row should serialize"),
        expected
    );
    assert_eq!(
        serde_json::to_string(&fetched).expect("fetched row should serialize"),
        expected
    );
}

#[tokio::test]
async fn invalid_insert_rolls_back_and_leaves_row_count_unchanged() {
    // Risk: invalid inserts leave durable rows. Level: particular-integration.
    // Source: proposals/0b-04-wu-0b-04.md test intent "Transaction rollback".
    let (pool, repo) = empty_policyset_fixture().await;
    let fixture: RoundTripFixture = read_fixture("round-trip.json");
    let mut invalid = fixture.row;
    invalid.summary_contract_version.0.version = "not-semver".to_string();

    let before = policyset_row_count(&pool).await;
    let error = repo
        .insert_policy_set(invalid)
        .await
        .expect_err("invalid policy set should fail");
    let after = policyset_row_count(&pool).await;

    assert_invariant_violation(error);
    assert_eq!(after, before);
}

#[tokio::test]
async fn every_policy_version_field_rejects_empty_unknown_non_monotonic_and_malformed() {
    // Risk: one policy-version field accepts empty, unknown, non-monotonic, or
    // malformed versions. Level: unit/particular-integration. Source:
    // proposals/0b-04-wu-0b-04.md test intent "Rejection matrix".
    let (_pool, repo) = empty_policyset_fixture().await;
    let fixture: RoundTripFixture = read_fixture("round-trip.json");
    let matrix: RejectionMatrixFixture = read_fixture("rejection-matrix.json");
    assert_eq!(matrix.expected_error, "InvariantViolation");

    for field in matrix.fields {
        for mode in &matrix.modes {
            let mut invalid = unique_policy_set_id(&fixture.row, &field, mode);
            set_policy_version(&mut invalid, &field, invalid_policy_version(&field, mode));

            let error = match repo.insert_policy_set(invalid).await {
                Ok(_) => panic!("invalid {field}/{mode} should not insert"),
                Err(error) => error,
            };
            assert_invariant_violation(error);
        }
    }
}

#[tokio::test]
async fn governance_update_creates_new_row_and_existing_rows_remain_addressable() {
    // Risk: policy updates overwrite historical rows. Level:
    // particular-integration. Source: proposals/0b-04-wu-0b-04.md test intent
    // "Append-only governance".
    let (pool, repo) = empty_policyset_fixture().await;
    let fixture: AppendOnlyFixture = read_fixture("append-only-governance.json");
    assert_eq!(fixture.rows.len(), 2);

    for row in fixture.rows.clone() {
        repo.insert_policy_set(row)
            .await
            .expect("append-only policy set row should insert");
    }

    assert_eq!(policyset_row_count(&pool).await, 2);
    for row in &fixture.rows {
        let fetched = repo
            .get_policy_set(&row.policy_set_id)
            .await
            .expect("policy set get should succeed")
            .expect("policy set row should remain addressable");
        assert_eq!(
            serde_json::to_string(&fetched).expect("fetched row should serialize"),
            serde_json::to_string(row).expect("fixture row should serialize")
        );
    }

    let workspace_id =
        OpaqueId::<GraphWorkspaceRef>::new("workspace-alpha", "workspace").expect("valid id");
    let listed = repo
        .list_by_workspace(&workspace_id)
        .await
        .expect("workspace/global list should succeed");
    assert_eq!(listed.len(), 2);
}

#[tokio::test]
async fn policyset_wu_has_no_operator_visible_behavior() {
    // Risk: governance storage enables operator-visible behavior. Level:
    // component structural. Source: proposals/0b-04-wu-0b-04.md test intent
    // "Side-effect absence".
    let fixture: NoSideEffectsFixture = read_fixture("no-side-effects.json");
    let (pool, _repo) = empty_policyset_fixture().await;

    assert_eq!(
        phase_0a_scaffold_commands(),
        fixture
            .phase_0a_scaffold_commands
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        sqlite_table_names(&pool).await,
        fixture.allowed_runtime_tables
    );

    let source = policyset_source();
    for token in fixture.forbidden_source_tokens {
        assert!(
            !source.contains(&token),
            "policyset source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn policyset_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-04 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = policyset_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};",
            "use agent_harness_lib::contracts::policyset::{PolicySet, PolicyVersionRef};",
            "use agent_harness_lib::graphstore::fixture::{GraphStoreRepo, GraphWorkspaceRef};",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );
}
