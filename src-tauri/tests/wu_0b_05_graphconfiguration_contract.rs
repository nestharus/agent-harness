use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::graphconfiguration::{
    EffectiveValueSource, GraphConfiguration, ValidationState,
};
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::graphstore::graphconfiguration::{GraphConfigurationRepo, GraphWorkspace};
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: GraphConfiguration,
}

#[derive(Debug, Deserialize)]
struct PolicySetFixture {
    row: PolicySet,
}

#[derive(Debug, Deserialize)]
struct SourceVariantsFixture {
    variants: Vec<EffectiveValueSource>,
}

#[derive(Debug, Deserialize)]
struct ValidationVariantsFixture {
    variants: Vec<ValidationState>,
}

#[derive(Debug, Deserialize)]
struct UnknownSourceFixture {
    source: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct UnknownValidationStateFixture {
    validation_state: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AppendOnlyFixture {
    rows: Vec<GraphConfiguration>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a repository parent")
        .to_path_buf()
}

fn fixture_path(dir: &str, name: &str) -> PathBuf {
    repo_root().join(dir).join(name)
}

fn read_fixture<T: for<'de> Deserialize<'de>>(dir: &str, name: &str) -> T {
    let path = fixture_path(dir, name);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("fixture {} must parse: {error}", path.display()))
}

async fn empty_graphconfiguration_fixture() -> (SqlitePool, GraphConfigurationRepo) {
    let fixture = create_graphstore_fixture(GraphStoreSeedPlan::default())
        .await
        .expect("empty graphstore fixture should apply migrations");
    assert_eq!(fixture.pool.migrations_applied, vec![1, 4, 5, 6, 7, 9, 15]);
    let pool = fixture.pool.sqlite;
    let policy_repo = PolicySetRepo::new(pool.clone());
    let policy_fixture: PolicySetFixture = read_fixture(POLICYSET_FIXTURE_DIR, "round-trip.json");
    policy_repo
        .insert_policy_set(policy_fixture.row)
        .await
        .expect("policy set FK target should insert");
    let repo = GraphConfigurationRepo::new(pool.clone());
    (pool, repo)
}

async fn graph_configuration_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM graph_configurations")
        .fetch_one(pool)
        .await
        .expect("graph_configurations row count should be readable");
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

fn graphconfiguration_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("graphconfiguration.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn graphconfiguration_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_05_graphconfiguration_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn unique_configuration(mut row: GraphConfiguration, suffix: &str) -> GraphConfiguration {
    row.configuration_id.value = format!("graph-configuration-{suffix}");
    row
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

#[tokio::test]
async fn graph_configurations_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: configuration table misses columns, JSON checks, policy FKs, unique
    // constraint, indexes, or RecordMeta fields. Level: particular-integration.
    // Source: proposals/0b-05-wu-0b-05.md test intent "Schema creation".
    let (pool, _repo) = empty_graphconfiguration_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'graph_configurations'",
    )
    .fetch_one(&pool)
    .await
    .expect("graph_configurations DDL should exist");

    for column in [
        "configuration_id_value",
        "configuration_id_namespace",
        "workspace_id_value",
        "workspace_id_namespace",
        "configuration_version",
        "schema_profile",
        "summary_contract_template_ids",
        "optimizer_policy_ref_value",
        "optimizer_policy_ref_namespace",
        "render_policy_ref_value",
        "render_policy_ref_namespace",
        "memory_policy_ref_value",
        "memory_policy_ref_namespace",
        "provider_routing_policy_ref_value",
        "provider_routing_policy_ref_namespace",
        "capability_fingerprint_policy_ref_value",
        "capability_fingerprint_policy_ref_namespace",
        "effective_value_sources",
        "created_from_configuration_id_value",
        "created_from_configuration_id_namespace",
        "validation_state",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "graph_configurations DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(configuration_id_value, configuration_id_namespace)"));
    assert!(sql.0.contains("json_valid(summary_contract_template_ids)"));
    assert!(sql.0.contains("json_valid(effective_value_sources)"));
    assert!(sql.0.contains("json_valid(actor)"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(graph_configurations)")
        .fetch_all(&pool)
        .await
        .expect("foreign keys should be introspectable");
    for field in [
        "optimizer_policy_ref",
        "render_policy_ref",
        "memory_policy_ref",
        "provider_routing_policy_ref",
        "capability_fingerprint_policy_ref",
    ] {
        assert!(
            fks.iter().any(|row| {
                row.get::<String, _>("table") == "policy_sets"
                    && row.get::<String, _>("from") == format!("{field}_value")
                    && row.get::<String, _>("to") == "policy_set_id_value"
            }),
            "{field} should reference policy_sets(policy_set_id_value)"
        );
        assert!(
            fks.iter().any(|row| {
                row.get::<String, _>("table") == "policy_sets"
                    && row.get::<String, _>("from") == format!("{field}_namespace")
                    && row.get::<String, _>("to") == "policy_set_id_namespace"
            }),
            "{field} should reference policy_sets(policy_set_id_namespace)"
        );
    }

    let indexes = sqlx::query("PRAGMA index_list(graph_configurations)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    assert!(index_names
        .iter()
        .any(|name| name == "idx_graph_configurations_configuration_id"));
    assert!(index_names
        .iter()
        .any(|name| name == "idx_graph_configurations_workspace_id"));
    assert!(index_names
        .iter()
        .any(|name| name == "idx_graph_configurations_created_from"));
    assert!(
        indexes.iter().any(|row| row.get::<i64, _>("unique") == 1),
        "graph_configurations should expose a unique configuration_id index"
    );
}

#[tokio::test]
async fn insert_then_get_round_trips_every_graph_configuration_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, enum, JSON, timestamp, or
    // opaque-ID fields. Level: particular-integration. Source:
    // proposals/0b-05-wu-0b-05.md test intent "Round-trip".
    let (_pool, repo) = empty_graphconfiguration_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_graph_configuration(fixture.row.clone())
        .await
        .expect("valid graph configuration should insert");
    let fetched = repo
        .get_graph_configuration(&fixture.row.configuration_id)
        .await
        .expect("graph configuration get should succeed")
        .expect("inserted graph configuration should be returned");

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
async fn invalid_insert_paths_roll_back_without_partial_rows() {
    // Risk: FK, enum, source-attribution, or revision-transition failures leave
    // durable rows. Level: particular-integration. Source:
    // proposals/0b-05-wu-0b-05.md test intent "Transaction rollback".
    let (pool, repo) = empty_graphconfiguration_fixture().await;
    let before = graph_configuration_row_count(&pool).await;

    let missing_policy: RowFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "transaction-rollback-fk.json");
    let fk_error = repo
        .insert_graph_configuration(missing_policy.row)
        .await
        .expect_err("missing policy refs should fail");
    assert_error(fk_error, GraphStoreError::UnknownRef);
    assert_eq!(graph_configuration_row_count(&pool).await, before);

    let missing_source: RowFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "source-attribution-rejection.json");
    let source_error = repo
        .insert_graph_configuration(missing_source.row)
        .await
        .expect_err("missing source attribution should fail");
    assert_error(source_error, GraphStoreError::InvariantViolation);
    assert_eq!(graph_configuration_row_count(&pool).await, before);

    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let mut bad_parent = unique_configuration(fixture.row, "bad-parent-transition");
    bad_parent.created_from_configuration_id =
        Some(OpaqueId::new("missing-parent", "configuration").expect("valid parent id"));
    let transition_error = repo
        .insert_graph_configuration(bad_parent)
        .await
        .expect_err("missing revision parent should fail");
    assert_error(transition_error, GraphStoreError::UnknownRef);
    assert_eq!(graph_configuration_row_count(&pool).await, before);

    let unknown_state: UnknownValidationStateFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-validation-state.json");
    assert!(
        serde_json::from_value::<ValidationState>(unknown_state.validation_state).is_err(),
        "unknown validation-state enum should be rejected before insert"
    );
    assert_eq!(graph_configuration_row_count(&pool).await, before);
}

#[tokio::test]
async fn no_operator_visible_behavior_or_extra_tables_are_enabled() {
    // Risk: configuration storage enables operator-visible behavior. Level:
    // component structural. Source: proposals/0b-05-wu-0b-05.md test intent
    // "Side-effect absence".
    let (pool, _repo) = empty_graphconfiguration_fixture().await;

    assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
    assert_eq!(
        sqlite_table_names(&pool).await,
        [
            "audit_events",
            "evidence_artifacts",
            "graph_configurations",
            "graph_nodes",
            "graph_workspaces",
            "policy_sets",
            "schema_versions"
        ]
    );

    let source = graphconfiguration_source();
    for token in [
        "Command::new",
        "std::process::Command",
        "tokio::process",
        "agents",
        "tauri::command",
        "generate_handler!",
        "invoke_handler",
    ] {
        assert!(
            !source.contains(token),
            "graphconfiguration source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn graphconfiguration_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-05 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = graphconfiguration_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::graphconfiguration::{",
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};",
            "use agent_harness_lib::contracts::policyset::PolicySet;",
            "use agent_harness_lib::graphstore::graphconfiguration::{GraphConfigurationRepo, GraphWorkspace};",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );
}

#[tokio::test]
async fn every_effective_value_source_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one attribution source variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-05-wu-0b-05.md test intent "Effective source variants".
    let (pool, repo) = empty_graphconfiguration_fixture().await;
    let fixture: SourceVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "effective-value-source-variants.json");
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: EffectiveValueSource =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS source_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM source_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO source_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM source_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: EffectiveValueSource = row
            .try_get("value")
            .expect("variant should decode through SQLx");
        assert_eq!(from_sql, variant);

        let mut record = unique_configuration(base.row.clone(), &format!("source-{index}"));
        record.effective_value_sources.0[0].source = variant;
        repo.insert_graph_configuration(record)
            .await
            .expect("source variant should be reachable through insert");
    }

    let unknown: UnknownSourceFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-effective-value-source.json");
    assert!(
        serde_json::from_value::<EffectiveValueSource>(unknown.source).is_err(),
        "unknown source enum should be rejected by serde"
    );
    let row = sqlx::query("SELECT 'operator_override' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown source row should select");
    assert!(
        row.try_get::<EffectiveValueSource, _>("value").is_err(),
        "unknown source enum should be rejected by SQLx"
    );
}

#[tokio::test]
async fn every_validation_state_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one validation-state variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-05-wu-0b-05.md test intent "Validation state variants".
    let (pool, repo) = empty_graphconfiguration_fixture().await;
    let fixture: ValidationVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "validation-state-variants.json");
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: ValidationState =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS state_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM state_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO state_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM state_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: ValidationState = row
            .try_get("value")
            .expect("variant should decode through SQLx");
        assert_eq!(from_sql, variant);

        let mut record = unique_configuration(base.row.clone(), &format!("state-{index}"));
        record.validation_state = variant;
        repo.insert_graph_configuration(record)
            .await
            .expect("validation state variant should be reachable through insert");
    }

    let unknown: UnknownValidationStateFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-validation-state.json");
    assert!(
        serde_json::from_value::<ValidationState>(unknown.validation_state).is_err(),
        "unknown validation-state enum should be rejected by serde"
    );
    let row = sqlx::query("SELECT 'valid_after_probe' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown validation-state row should select");
    assert!(
        row.try_get::<ValidationState, _>("value").is_err(),
        "unknown validation-state enum should be rejected by SQLx"
    );
}

#[tokio::test]
async fn created_from_configuration_id_is_null_or_older_same_workspace_revision() {
    // Risk: configuration revisions overwrite history or link to the wrong
    // parent. Level: particular-integration. Source:
    // proposals/0b-05-wu-0b-05.md test intent "Append-only revisions".
    let (pool, repo) = empty_graphconfiguration_fixture().await;
    let fixture: AppendOnlyFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "append-only-revision.json");
    assert_eq!(fixture.rows.len(), 2);

    repo.insert_graph_configuration(fixture.rows[0].clone())
        .await
        .expect("first revision with null parent should insert");
    repo.insert_graph_configuration(fixture.rows[1].clone())
        .await
        .expect("second revision with older same-workspace parent should insert");

    assert_eq!(graph_configuration_row_count(&pool).await, 2);
    let workspace =
        OpaqueId::<GraphWorkspace>::new("workspace-alpha", "workspace").expect("valid workspace");
    let listed = repo
        .list_graph_configurations_by_workspace(&workspace)
        .await
        .expect("workspace list should succeed");
    assert_eq!(listed.len(), 2);

    let mut different_workspace = unique_configuration(fixture.rows[1].clone(), "wrong-workspace");
    different_workspace.workspace_id =
        OpaqueId::new("workspace-beta", "workspace").expect("valid workspace");
    different_workspace.meta.created_at = "2026-04-30T12:20:00Z".to_string();
    different_workspace.meta.updated_at = "2026-04-30T12:20:00Z".to_string();
    let error = repo
        .insert_graph_configuration(different_workspace)
        .await
        .expect_err("different-workspace parent should fail");
    assert_error(error, GraphStoreError::InvalidTransition);

    let mut non_older = unique_configuration(fixture.rows[1].clone(), "non-older-parent");
    non_older.meta.created_at = "2026-04-30T12:10:00Z".to_string();
    non_older.meta.updated_at = "2026-04-30T12:10:00Z".to_string();
    let error = repo
        .insert_graph_configuration(non_older)
        .await
        .expect_err("non-older parent should fail");
    assert_error(error, GraphStoreError::InvalidTransition);

    let mut missing_parent = unique_configuration(fixture.rows[1].clone(), "missing-parent");
    missing_parent.created_from_configuration_id =
        Some(OpaqueId::new("missing-parent", "configuration").expect("valid id"));
    missing_parent.meta.created_at = "2026-04-30T12:30:00Z".to_string();
    missing_parent.meta.updated_at = "2026-04-30T12:30:00Z".to_string();
    let error = repo
        .insert_graph_configuration(missing_parent)
        .await
        .expect_err("missing parent should fail");
    assert_error(error, GraphStoreError::UnknownRef);

    assert_eq!(graph_configuration_row_count(&pool).await, 2);
}

#[tokio::test]
async fn effective_value_source_missing_source_ref_returns_invariant_violation() {
    // Risk: effective values lack provenance. Level: particular-integration.
    // Source: proposals/0b-05-wu-0b-05.md test intent "Source attribution".
    let (pool, repo) = empty_graphconfiguration_fixture().await;
    let fixture: RowFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "source-attribution-rejection.json");
    let before = graph_configuration_row_count(&pool).await;

    let error = repo
        .insert_graph_configuration(fixture.row)
        .await
        .expect_err("missing source_ref should fail");

    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(graph_configuration_row_count(&pool).await, before);
}
