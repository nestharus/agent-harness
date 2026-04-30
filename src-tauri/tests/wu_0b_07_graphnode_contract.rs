use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;
use agent_harness_lib::contracts::graphnode::{
    GraphEdgeRef, GraphNode, GraphNodeKind, LifecycleState, NodeRevisionRef, PrivilegeOrigin,
    TrustState,
};
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};
use agent_harness_lib::contracts::graphworkspace::{GraphWorkspace, Orchestrator};
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;
use agent_harness_lib::graphstore::graphnode::GraphNodeRepo;
use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-07";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";
const GRAPHCONFIGURATION_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";
const GRAPHWORKSPACE_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-06";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: GraphNode,
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
struct GraphWorkspaceFixture {
    row: GraphWorkspace,
}

#[derive(Debug, Deserialize)]
struct KindVariantsFixture {
    variants: Vec<GraphNodeKind>,
}

#[derive(Debug, Deserialize)]
struct LifecycleStateVariantsFixture {
    variants: Vec<LifecycleState>,
}

#[derive(Debug, Deserialize)]
struct PrivilegeOriginVariantsFixture {
    variants: Vec<PrivilegeOrigin>,
}

#[derive(Debug, Deserialize)]
struct TrustStateVariantsFixture {
    variants: Vec<TrustState>,
}

#[derive(Debug, Deserialize)]
struct UnknownVariantsFixture {
    kind: Value,
    lifecycle_state: Value,
    privilege_origin: Value,
    trust_state: Value,
}

#[derive(Debug, Deserialize)]
struct TransitionMatrixFixture {
    valid: Vec<TransitionFixture>,
    invalid: Vec<TransitionFixture>,
    deleted_terminal_target: LifecycleState,
}

#[derive(Debug, Deserialize)]
struct TransitionFixture {
    from: LifecycleState,
    to: LifecycleState,
}

#[derive(Debug, Deserialize)]
struct RollbackFixture {
    unknown_workspace: GraphNode,
    unknown_current_revision: GraphNode,
    unknown_canonical_parent: GraphNode,
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

async fn empty_graphnode_fixture() -> (SqlitePool, GraphNodeRepo) {
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
    let configuration_fixture: GraphConfigurationFixture =
        read_fixture(GRAPHCONFIGURATION_FIXTURE_DIR, "round-trip.json");
    configuration_repo
        .insert_graph_configuration(configuration_fixture.row)
        .await
        .expect("configuration FK target should insert");

    let workspace_repo = GraphWorkspaceRepo::new(pool.clone());
    let workspace_fixture: GraphWorkspaceFixture =
        read_fixture(GRAPHWORKSPACE_FIXTURE_DIR, "round-trip.json");
    workspace_repo
        .insert_graph_workspace(workspace_fixture.row)
        .await
        .expect("workspace FK target should insert");

    let repo = GraphNodeRepo::new(pool.clone());
    (pool, repo)
}

async fn graph_node_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM graph_nodes")
        .fetch_one(pool)
        .await
        .expect("graph_nodes row count should be readable");
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

fn graphnode_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("graphnode.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn graphnode_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_07_graphnode_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn unique_node(mut row: GraphNode, suffix: &str) -> GraphNode {
    row.node_id.value = format!("graph-node-{suffix}");
    row.title = format!("Graph node {suffix}");
    row
}

fn node_with_state(mut row: GraphNode, suffix: &str, state: LifecycleState) -> GraphNode {
    row = unique_node(row, suffix);
    row.lifecycle_state = state;
    row.deleted_at = if state == LifecycleState::Deleted {
        Some("2026-04-30T13:30:00Z".to_string())
    } else {
        None
    };
    row
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

#[tokio::test]
async fn graph_nodes_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: node table misses columns, FK, unique constraints, JSON checks,
    // indexes, soft-ref columns, or RecordMeta fields. Level:
    // particular-integration. Source: proposals/0b-07-wu-0b-07.md test
    // intent "Schema creation".
    let (pool, _repo) = empty_graphnode_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'graph_nodes'",
    )
    .fetch_one(&pool)
    .await
    .expect("graph_nodes DDL should exist");

    for column in [
        "node_id_value",
        "node_id_namespace",
        "workspace_id_value",
        "workspace_id_namespace",
        "kind",
        "title",
        "lifecycle_state",
        "current_revision_id_value",
        "current_revision_id_namespace",
        "canonical_parent_edge_id_value",
        "canonical_parent_edge_id_namespace",
        "privilege_origin",
        "trust_state",
        "created_from_ref",
        "deleted_at",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "graph_nodes DDL should contain {column}"
        );
    }
    assert!(sql.0.contains("UNIQUE(node_id_value, node_id_namespace)"));
    assert!(sql.0.contains("json_valid(actor)"));
    assert!(sql.0.contains("json_valid(created_from_ref)"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(graph_nodes)")
        .fetch_all(&pool)
        .await
        .expect("foreign keys should be introspectable");
    assert!(fks
        .iter()
        .any(|row| row.get::<String, _>("table") == "schema_versions"
            && row.get::<String, _>("from") == "schema_version"
            && row.get::<String, _>("to") == "version"));
    assert!(fks
        .iter()
        .any(|row| row.get::<String, _>("table") == "graph_workspaces"
            && row.get::<String, _>("from") == "workspace_id_value"
            && row.get::<String, _>("to") == "workspace_id_value"));
    assert!(fks
        .iter()
        .any(|row| row.get::<String, _>("table") == "graph_workspaces"
            && row.get::<String, _>("from") == "workspace_id_namespace"
            && row.get::<String, _>("to") == "workspace_id_namespace"));
    assert!(
        !fks.iter().any(|row| {
            row.get::<String, _>("from")
                .starts_with("current_revision_id")
                || row
                    .get::<String, _>("from")
                    .starts_with("canonical_parent_edge_id")
        }),
        "future-WU refs should be soft refs until their tables exist"
    );

    let indexes = sqlx::query("PRAGMA index_list(graph_nodes)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    for expected in [
        "idx_graph_nodes_node_id",
        "idx_graph_nodes_workspace_id",
        "idx_graph_nodes_lifecycle_state",
        "idx_graph_nodes_kind",
    ] {
        assert!(
            index_names.iter().any(|name| name == expected),
            "missing index {expected}"
        );
    }
    assert!(
        indexes.iter().any(|row| row.get::<i64, _>("unique") == 1),
        "graph_nodes should expose a unique node_id index"
    );
}

#[tokio::test]
async fn insert_then_get_round_trips_every_graph_node_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, enum, JSON, timestamp,
    // soft-ref, or opaque-ID fields. Level: particular-integration. Source:
    // proposals/0b-07-wu-0b-07.md test intent "Round-trip".
    let (_pool, repo) = empty_graphnode_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_graph_node(fixture.row.clone())
        .await
        .expect("valid graph node should insert");
    let fetched = repo
        .get_graph_node(&fixture.row.node_id)
        .await
        .expect("graph node get should succeed")
        .expect("inserted graph node should be returned");

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
async fn invalid_insert_paths_roll_back_without_partial_node_rows() {
    // Risk: FK, enum, routing, or future soft-ref validation failures leave
    // durable rows. Level: particular-integration. Source:
    // proposals/0b-07-wu-0b-07.md test intent "Transaction rollback".
    let (pool, repo) = empty_graphnode_fixture().await;
    let fixture: RollbackFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "transaction-rollback-fk.json");
    let before = graph_node_row_count(&pool).await;

    for row in [
        fixture.unknown_workspace,
        fixture.unknown_current_revision,
        fixture.unknown_canonical_parent,
    ] {
        let error = repo
            .insert_graph_node(row)
            .await
            .expect_err("unknown FK or soft ref should fail");
        assert_error(error, GraphStoreError::UnknownRef);
        assert_eq!(graph_node_row_count(&pool).await, before);
    }

    let mut invalid_routing: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    invalid_routing.row.title.clear();
    let error = repo
        .insert_graph_node(invalid_routing.row)
        .await
        .expect_err("invalid node title should fail before insert");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(graph_node_row_count(&pool).await, before);
}

#[tokio::test]
async fn graphnode_wu_has_no_operator_visible_behavior_or_extra_tables() {
    // Risk: node storage enables operator-visible behavior. Level: component
    // structural. Source: proposals/0b-07-wu-0b-07.md test intent
    // "Side-effect absence".
    let (pool, _repo) = empty_graphnode_fixture().await;

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

    let source = graphnode_source();
    for token in [
        "Command::new",
        "std::process::Command",
        "tokio::process",
        "provider_probe",
        "optimizer_execution",
        "recovery_execution",
        "tauri::command",
        "generate_handler!",
        "invoke_handler",
    ] {
        assert!(
            !source.contains(token),
            "graphnode source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn graphnode_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-07 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = graphnode_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;",
            "use agent_harness_lib::contracts::graphnode::{",
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};",
            "use agent_harness_lib::contracts::graphworkspace::{GraphWorkspace, Orchestrator};",
            "use agent_harness_lib::contracts::policyset::PolicySet;",
            "use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;",
            "use agent_harness_lib::graphstore::graphnode::GraphNodeRepo;",
            "use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );

    let _orchestrator_id = OpaqueId::<Orchestrator>::new("orchestrator-default", "orchestrator")
        .expect("orchestrator opaque id should validate");
    let _revision_id = OpaqueId::<NodeRevisionRef>::new("node-revision-ref", "revision")
        .expect("revision opaque id should validate");
    let _edge_id =
        OpaqueId::<GraphEdgeRef>::new("graph-edge-ref", "edge").expect("edge id validates");
}

#[tokio::test]
async fn every_kind_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one node kind variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-07-wu-0b-07.md test intent "Kind variants".
    let (pool, repo) = empty_graphnode_fixture().await;
    let fixture: KindVariantsFixture = read_fixture(CONTRACT_FIXTURE_DIR, "kind-variants.json");
    assert_eq!(fixture.variants.len(), 11);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: GraphNodeKind =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS kind_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM kind_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO kind_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM kind_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: GraphNodeKind = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_node(base.row.clone(), &format!("kind-{index}"));
        record.kind = variant;
        repo.insert_graph_node(record)
            .await
            .expect("kind variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<GraphNodeKind>(unknown.kind).is_err());
    let row = sqlx::query("SELECT 'operator_panel' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown kind row should select");
    assert!(row.try_get::<GraphNodeKind, _>("value").is_err());
}

#[tokio::test]
async fn every_lifecycle_state_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one lifecycle state variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-07-wu-0b-07.md test intent "Lifecycle variants".
    let (pool, repo) = empty_graphnode_fixture().await;
    let fixture: LifecycleStateVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "lifecycle-state-variants.json");
    assert_eq!(fixture.variants.len(), 9);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: LifecycleState =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS lifecycle_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM lifecycle_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO lifecycle_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM lifecycle_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: LifecycleState = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let record = node_with_state(base.row.clone(), &format!("lifecycle-{index}"), variant);
        repo.insert_graph_node(record)
            .await
            .expect("lifecycle variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<LifecycleState>(unknown.lifecycle_state).is_err());
    let row = sqlx::query("SELECT 'paused' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown lifecycle row should select");
    assert!(row.try_get::<LifecycleState, _>("value").is_err());
}

#[tokio::test]
async fn every_privilege_origin_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one privilege origin variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-07-wu-0b-07.md test intent "Privilege variants".
    let (pool, repo) = empty_graphnode_fixture().await;
    let fixture: PrivilegeOriginVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "privilege-origin-variants.json");
    assert_eq!(fixture.variants.len(), 7);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: PrivilegeOrigin =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS privilege_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM privilege_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO privilege_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM privilege_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: PrivilegeOrigin = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_node(base.row.clone(), &format!("privilege-{index}"));
        record.privilege_origin = variant;
        repo.insert_graph_node(record)
            .await
            .expect("privilege variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<PrivilegeOrigin>(unknown.privilege_origin).is_err());
    let row = sqlx::query("SELECT 'administrator' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown privilege row should select");
    assert!(row.try_get::<PrivilegeOrigin, _>("value").is_err());
}

#[tokio::test]
async fn every_trust_state_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one trust state variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-07-wu-0b-07.md test intent "Trust variants".
    let (pool, repo) = empty_graphnode_fixture().await;
    let fixture: TrustStateVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "trust-state-variants.json");
    assert_eq!(fixture.variants.len(), 5);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: TrustState =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS trust_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM trust_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO trust_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM trust_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: TrustState = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_node(base.row.clone(), &format!("trust-{index}"));
        record.trust_state = variant;
        repo.insert_graph_node(record)
            .await
            .expect("trust variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<TrustState>(unknown.trust_state).is_err());
    let row = sqlx::query("SELECT 'trusted_by_default' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown trust row should select");
    assert!(row.try_get::<TrustState, _>("value").is_err());
}

#[tokio::test]
async fn lifecycle_transition_matrix_is_enforced_and_deleted_is_terminal() {
    // Risk: lifecycle state machine accepts illegal transitions, omits valid
    // contract transitions, or allows deleted nodes to re-enter active states.
    // Level: particular-integration. Source: proposals/0b-07-wu-0b-07.md
    // test intent "Lifecycle transition matrix".
    let (_pool, repo) = empty_graphnode_fixture().await;
    let fixture: TransitionMatrixFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "transition-matrix.json");
    assert_eq!(fixture.valid.len(), 9);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, transition) in fixture.valid.into_iter().enumerate() {
        let record = node_with_state(
            base.row.clone(),
            &format!("transition-valid-{index}"),
            transition.from,
        );
        repo.insert_graph_node(record.clone())
            .await
            .expect("transition source should insert");

        let updated = repo
            .transition_lifecycle(&record.node_id, transition.to)
            .await
            .expect("valid transition should apply");
        assert_eq!(updated.lifecycle_state, transition.to);
        assert_ne!(updated.meta.updated_at, record.meta.updated_at);
        let fetched = repo
            .get_graph_node(&record.node_id)
            .await
            .expect("node get should succeed")
            .expect("transitioned node should exist");
        assert_eq!(fetched.lifecycle_state, transition.to);
    }

    for (index, transition) in fixture.invalid.into_iter().enumerate() {
        let record = node_with_state(
            base.row.clone(),
            &format!("transition-invalid-{index}"),
            transition.from,
        );
        repo.insert_graph_node(record.clone())
            .await
            .expect("invalid-transition source should insert");
        let before = serde_json::to_string(&record).expect("source node should serialize");

        let error = repo
            .transition_lifecycle(&record.node_id, transition.to)
            .await
            .expect_err("invalid transition should fail");
        assert_error(error, GraphStoreError::InvalidTransition);
        let after = repo
            .get_graph_node(&record.node_id)
            .await
            .expect("node get should succeed")
            .expect("node should remain present");
        assert_eq!(
            serde_json::to_string(&after).expect("node should serialize"),
            before
        );
    }

    let deleted = node_with_state(
        base.row,
        "transition-deleted-terminal",
        LifecycleState::Deleted,
    );
    repo.insert_graph_node(deleted.clone())
        .await
        .expect("deleted source should insert");
    let before = serde_json::to_string(&deleted).expect("deleted node should serialize");
    let error = repo
        .transition_lifecycle(&deleted.node_id, fixture.deleted_terminal_target)
        .await
        .expect_err("deleted must be terminal");
    assert_error(error, GraphStoreError::InvalidTransition);
    let after = repo
        .get_graph_node(&deleted.node_id)
        .await
        .expect("node get should succeed")
        .expect("deleted node should remain present");
    assert_eq!(
        serde_json::to_string(&after).expect("node should serialize"),
        before
    );
}
