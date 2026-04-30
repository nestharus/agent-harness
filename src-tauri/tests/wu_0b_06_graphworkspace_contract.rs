use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};
use agent_harness_lib::contracts::graphworkspace::{GraphWorkspace, Orchestrator};
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;
use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-06";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";
const GRAPHCONFIGURATION_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: GraphWorkspace,
}

#[derive(Debug, Deserialize)]
struct PolicySetFixture {
    row: PolicySet,
}

#[derive(Debug, Deserialize)]
struct GraphConfigurationFixture {
    row: GraphConfiguration,
}

#[derive(Debug, Deserialize)]
struct ActiveDefaultFixture {
    rows: Vec<GraphWorkspace>,
}

#[derive(Debug, Deserialize)]
struct SetActiveConfigurationFixture {
    row: GraphWorkspace,
    replacement_configuration_id: OpaqueId<GraphConfiguration>,
    unknown_configuration_id: OpaqueId<GraphConfiguration>,
}

#[derive(Debug, Deserialize)]
struct AdvanceGraphVersionFixture {
    row: GraphWorkspace,
    expected_version: i64,
    stale_expected_version: i64,
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

async fn empty_graphworkspace_fixture() -> (SqlitePool, GraphWorkspaceRepo) {
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

    let configuration_repo = GraphConfigurationRepo::new(pool.clone());
    for name in ["round-trip.json", "canonical-row.json"] {
        let configuration_fixture: GraphConfigurationFixture =
            read_fixture(GRAPHCONFIGURATION_FIXTURE_DIR, name);
        configuration_repo
            .insert_graph_configuration(configuration_fixture.row)
            .await
            .expect("configuration FK target should insert");
    }

    let repo = GraphWorkspaceRepo::new(pool.clone());
    (pool, repo)
}

async fn graph_workspace_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM graph_workspaces")
        .fetch_one(pool)
        .await
        .expect("graph_workspaces row count should be readable");
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

fn graphworkspace_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("graphworkspace.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn graphworkspace_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_06_graphworkspace_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

fn assert_only_active_configuration_and_updated_at_changed(
    before: &GraphWorkspace,
    after: &GraphWorkspace,
    expected_configuration: &OpaqueId<GraphConfiguration>,
) {
    assert_eq!(after.workspace_id, before.workspace_id);
    assert_eq!(after.schema_version, before.schema_version);
    assert_eq!(after.active_orchestrator_id, before.active_orchestrator_id);
    assert_eq!(after.current_graph_version, before.current_graph_version);
    assert_eq!(after.storage_root, before.storage_root);
    assert_eq!(after.policy_set_id, before.policy_set_id);
    assert_eq!(after.active_configuration_id, *expected_configuration);
    assert_eq!(after.is_active_default, before.is_active_default);
    assert_eq!(after.meta.created_at, before.meta.created_at);
    assert_ne!(after.meta.updated_at, before.meta.updated_at);
    assert_eq!(after.meta.actor, before.meta.actor);
    assert_eq!(after.meta.policy_version, before.meta.policy_version);
}

fn assert_only_graph_version_and_updated_at_changed(
    before: &GraphWorkspace,
    after: &GraphWorkspace,
) {
    assert_eq!(after.workspace_id, before.workspace_id);
    assert_eq!(after.schema_version, before.schema_version);
    assert_eq!(after.active_orchestrator_id, before.active_orchestrator_id);
    assert_eq!(
        after.current_graph_version,
        before.current_graph_version + 1
    );
    assert_eq!(after.storage_root, before.storage_root);
    assert_eq!(after.policy_set_id, before.policy_set_id);
    assert_eq!(
        after.active_configuration_id,
        before.active_configuration_id
    );
    assert_eq!(after.is_active_default, before.is_active_default);
    assert_eq!(after.meta.created_at, before.meta.created_at);
    assert_ne!(after.meta.updated_at, before.meta.updated_at);
    assert_eq!(after.meta.actor, before.meta.actor);
    assert_eq!(after.meta.policy_version, before.meta.policy_version);
}

#[tokio::test]
async fn graph_workspaces_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: workspace table misses columns, FKs, unique constraints, indexes,
    // partial default invariant, or RecordMeta fields. Level:
    // particular-integration. Source: proposals/0b-06-wu-0b-06.md test
    // intent "Schema creation".
    let (pool, _repo) = empty_graphworkspace_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'graph_workspaces'",
    )
    .fetch_one(&pool)
    .await
    .expect("graph_workspaces DDL should exist");

    for column in [
        "workspace_id_value",
        "workspace_id_namespace",
        "schema_version",
        "active_orchestrator_id_value",
        "active_orchestrator_id_namespace",
        "current_graph_version",
        "storage_root",
        "policy_set_id_value",
        "policy_set_id_namespace",
        "active_configuration_id_value",
        "active_configuration_id_namespace",
        "is_active_default",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
    ] {
        assert!(
            sql.0.contains(column),
            "graph_workspaces DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(workspace_id_value, workspace_id_namespace)"));
    assert!(sql.0.contains("json_valid(actor)"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(graph_workspaces)")
        .fetch_all(&pool)
        .await
        .expect("foreign keys should be introspectable");
    assert!(fks
        .iter()
        .any(|row| row.get::<String, _>("table") == "schema_versions"
            && row.get::<String, _>("from") == "schema_version"
            && row.get::<String, _>("to") == "version"));
    for (table, field, target) in [
        ("policy_sets", "policy_set_id", "policy_set_id"),
        (
            "graph_configurations",
            "active_configuration_id",
            "configuration_id",
        ),
    ] {
        assert!(
            fks.iter().any(|row| row.get::<String, _>("table") == table
                && row.get::<String, _>("from") == format!("{field}_value")
                && row.get::<String, _>("to") == format!("{target}_value")),
            "{field} should reference {table} value"
        );
        assert!(
            fks.iter().any(|row| row.get::<String, _>("table") == table
                && row.get::<String, _>("from") == format!("{field}_namespace")
                && row.get::<String, _>("to") == format!("{target}_namespace")),
            "{field} should reference {table} namespace"
        );
    }

    let indexes = sqlx::query("PRAGMA index_list(graph_workspaces)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    for expected in [
        "idx_graph_workspaces_workspace_id",
        "idx_graph_workspaces_policy_set_id",
        "idx_graph_workspaces_active_configuration_id",
        "idx_graph_workspaces_one_active_default",
    ] {
        assert!(
            index_names.iter().any(|name| name == expected),
            "missing index {expected}"
        );
    }
    let partial_index: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'index' AND name = 'idx_graph_workspaces_one_active_default'",
    )
    .fetch_one(&pool)
    .await
    .expect("partial active-default index should exist");
    assert!(partial_index.0.contains("WHERE is_active_default = 1"));
}

#[tokio::test]
async fn insert_then_get_round_trips_every_graph_workspace_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, bool, opaque-ID, timestamp, or
    // metadata fields. Level: particular-integration. Source:
    // proposals/0b-06-wu-0b-06.md test intent "Round-trip".
    let (_pool, repo) = empty_graphworkspace_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_graph_workspace(fixture.row.clone())
        .await
        .expect("valid graph workspace should insert");
    let fetched = repo
        .get_graph_workspace(&fixture.row.workspace_id)
        .await
        .expect("graph workspace get should succeed")
        .expect("inserted graph workspace should be returned");

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
async fn fk_failure_rolls_back_without_partial_workspace_rows() {
    // Risk: FK failures leave durable workspace rows. Level:
    // particular-integration. Source: proposals/0b-06-wu-0b-06.md test intent
    // "Transaction rollback".
    let (pool, repo) = empty_graphworkspace_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "transaction-rollback-fk.json");
    let before = graph_workspace_row_count(&pool).await;

    let error = repo
        .insert_graph_workspace(fixture.row)
        .await
        .expect_err("missing FK targets should fail");

    assert_error(error, GraphStoreError::UnknownRef);
    assert_eq!(graph_workspace_row_count(&pool).await, before);
}

#[tokio::test]
async fn graphworkspace_wu_has_no_operator_visible_behavior() {
    // Risk: workspace storage enables operator-visible behavior. Level:
    // component structural. Source: proposals/0b-06-wu-0b-06.md test intent
    // "Side-effect absence".
    let (pool, _repo) = empty_graphworkspace_fixture().await;

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

    let source = graphworkspace_source();
    for token in [
        "Command::new",
        "std::process::Command",
        "tokio::process",
        "provider_probe",
        "optimizer",
        "recovery_execution",
        "tauri::command",
        "generate_handler!",
        "invoke_handler",
    ] {
        assert!(
            !source.contains(token),
            "graphworkspace source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn graphworkspace_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-06 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = graphworkspace_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;",
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};",
            "use agent_harness_lib::contracts::graphworkspace::{GraphWorkspace, Orchestrator};",
            "use agent_harness_lib::contracts::policyset::PolicySet;",
            "use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;",
            "use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );

    let _orchestrator_id = OpaqueId::<Orchestrator>::new("orchestrator-default", "orchestrator")
        .expect("orchestrator opaque id should validate");
}

#[tokio::test]
async fn exactly_one_active_default_workspace_is_allowed() {
    // Risk: multiple local roots are marked active default. Level:
    // particular-integration. Source: proposals/0b-06-wu-0b-06.md test intent
    // "Active default".
    let (pool, repo) = empty_graphworkspace_fixture().await;
    let fixture: ActiveDefaultFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "active-default-uniqueness.json");
    assert_eq!(fixture.rows.len(), 2);

    repo.insert_graph_workspace(fixture.rows[0].clone())
        .await
        .expect("first active default should insert");
    let error = repo
        .insert_graph_workspace(fixture.rows[1].clone())
        .await
        .expect_err("second active default should fail");

    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(graph_workspace_row_count(&pool).await, 1);
}

#[tokio::test]
async fn set_active_configuration_updates_only_configuration_and_updated_at() {
    // Risk: active-configuration switch mutates append-only workspace fields or
    // partially writes on invalid input. Level: particular-integration. Source:
    // proposals/0b-06-wu-0b-06.md test intent "Active configuration
    // mutation".
    let (_pool, repo) = empty_graphworkspace_fixture().await;
    let fixture: SetActiveConfigurationFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "set-active-configuration.json");
    let before = repo
        .insert_graph_workspace(fixture.row.clone())
        .await
        .expect("workspace should insert");

    let updated = repo
        .set_active_configuration(
            &fixture.row.workspace_id,
            &fixture.replacement_configuration_id,
        )
        .await
        .expect("known configuration should become active");
    assert_only_active_configuration_and_updated_at_changed(
        &before,
        &updated,
        &fixture.replacement_configuration_id,
    );
    let fetched = repo
        .get_graph_workspace(&fixture.row.workspace_id)
        .await
        .expect("workspace get should succeed")
        .expect("workspace should still exist");
    assert_eq!(fetched, updated);

    let stable_before_error =
        serde_json::to_string(&updated).expect("updated workspace should serialize");
    let error = repo
        .set_active_configuration(&fixture.row.workspace_id, &fixture.unknown_configuration_id)
        .await
        .expect_err("unknown configuration should fail");
    assert_error(error, GraphStoreError::UnknownRef);
    let after_error = repo
        .get_graph_workspace(&fixture.row.workspace_id)
        .await
        .expect("workspace get should succeed")
        .expect("workspace should still exist");
    assert_eq!(
        serde_json::to_string(&after_error).expect("workspace should serialize"),
        stable_before_error
    );
}

#[tokio::test]
async fn advance_graph_version_uses_optimistic_concurrency() {
    // Risk: graph version advances on stale expectations or leaves partial
    // writes. Level: particular-integration. Source:
    // proposals/0b-06-wu-0b-06.md test intent "Graph version OCC".
    let (_pool, repo) = empty_graphworkspace_fixture().await;
    let fixture: AdvanceGraphVersionFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "advance-graph-version.json");
    let before = repo
        .insert_graph_workspace(fixture.row.clone())
        .await
        .expect("workspace should insert");

    let advanced = repo
        .advance_graph_version(&fixture.row.workspace_id, fixture.expected_version)
        .await
        .expect("matching graph version should advance");
    assert_only_graph_version_and_updated_at_changed(&before, &advanced);

    let stable_before_stale =
        serde_json::to_string(&advanced).expect("advanced workspace should serialize");
    let error = repo
        .advance_graph_version(&fixture.row.workspace_id, fixture.stale_expected_version)
        .await
        .expect_err("stale expected version should fail");
    assert_error(error, GraphStoreError::OptimisticConflict);
    let after_stale = repo
        .get_graph_workspace(&fixture.row.workspace_id)
        .await
        .expect("workspace get should succeed")
        .expect("workspace should still exist");
    assert_eq!(
        serde_json::to_string(&after_stale).expect("workspace should serialize"),
        stable_before_stale
    );
}
