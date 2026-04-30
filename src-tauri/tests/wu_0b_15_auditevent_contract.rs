use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::auditevent::{AuditDecision, AuditEvent};
use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::GraphStoreError;
use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::graphstore::auditevent::AuditEventRepo;
use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;
use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-15";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";
const GRAPHCONFIGURATION_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";
const GRAPHWORKSPACE_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-06";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: AuditEvent,
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
struct DecisionVariantsFixture {
    variants: Vec<AuditDecision>,
}

#[derive(Debug, Deserialize)]
struct UnknownDecisionFixture {
    decision: Value,
}

#[derive(Debug, Deserialize)]
struct OutputWithInputFixture {
    output_refs_without_input_refs: AuditEvent,
    output_refs_with_empty_policy_id: AuditEvent,
    output_refs_with_empty_actor: AuditEvent,
    output_refs_with_empty_reason_code: AuditEvent,
}

#[derive(Debug, Deserialize)]
struct FkRejectionsFixture {
    unknown_workspace: AuditEvent,
    unknown_policy_set: AuditEvent,
    unknown_configuration: AuditEvent,
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

async fn empty_auditevent_fixture() -> (SqlitePool, AuditEventRepo) {
    let fixture = create_graphstore_fixture(GraphStoreSeedPlan::default())
        .await
        .expect("empty graphstore fixture should apply migrations");
    assert_eq!(
        fixture.pool.migrations_applied,
        vec![1, 4, 5, 6, 7, 9, 15, 16, 17]
    );
    let storage_root = fixture.pool.workspace_root.clone();
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
    let mut workspace_fixture: GraphWorkspaceFixture =
        read_fixture(GRAPHWORKSPACE_FIXTURE_DIR, "round-trip.json");
    workspace_fixture.row.storage_root = storage_root.to_string_lossy().into_owned();
    workspace_repo
        .insert_graph_workspace(workspace_fixture.row)
        .await
        .expect("workspace FK target should insert");

    let repo = AuditEventRepo::new(pool.clone());
    (pool, repo)
}

fn unique_audit_event(mut row: AuditEvent, suffix: &str) -> AuditEvent {
    row.audit_event_id.value = format!("audit-event-{suffix}");
    row.event_type = format!("contract_event_{suffix}");
    row.reason_code = format!("policy.{suffix}");
    row.created_at = "2026-04-30T16:00:00Z".to_string();
    row.meta.created_at = row.created_at.clone();
    row.meta.updated_at = row.created_at.clone();
    row
}

async fn audit_event_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_events")
        .fetch_one(pool)
        .await
        .expect("audit_events row count should be readable");
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

fn auditevent_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("auditevent.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn auditevent_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_15_auditevent_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

#[tokio::test]
async fn audit_events_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: audit table misses columns, FKs, unique constraints, JSON checks,
    // indexes, or RecordMeta fields. Level: particular-integration. Source:
    // proposals/0b-15-wu-0b-15.md test intent "Schema creation".
    let (pool, _repo) = empty_auditevent_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'audit_events'",
    )
    .fetch_one(&pool)
    .await
    .expect("audit_events DDL should exist");

    for column in [
        "audit_event_id_value",
        "audit_event_id_namespace",
        "workspace_id_value",
        "workspace_id_namespace",
        "event_type",
        "actor",
        "policy_set_id_value",
        "policy_set_id_namespace",
        "configuration_id_value",
        "configuration_id_namespace",
        "provider_state_id_value",
        "provider_state_id_namespace",
        "input_refs",
        "output_refs",
        "decision",
        "reason_code",
        "created_at",
        "record_created_at",
        "record_updated_at",
        "record_actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "audit_events DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(audit_event_id_value, audit_event_id_namespace)"));
    assert!(sql.0.contains("json_valid(input_refs)"));
    assert!(sql.0.contains("json_valid(output_refs)"));
    assert!(sql.0.contains("json_valid(record_actor)"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(audit_events)")
        .fetch_all(&pool)
        .await
        .expect("foreign keys should be introspectable");
    for expected_table in [
        "schema_versions",
        "graph_workspaces",
        "policy_sets",
        "graph_configurations",
        "provider_states",
    ] {
        assert!(
            fks.iter()
                .any(|row| row.get::<String, _>("table") == expected_table),
            "audit_events should reference {expected_table}"
        );
    }

    let indexes = sqlx::query("PRAGMA index_list(audit_events)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    for expected in [
        "idx_audit_events_workspace_id",
        "idx_audit_events_policy_set_id",
        "idx_audit_events_decision",
        "idx_audit_events_created_at",
    ] {
        assert!(
            index_names.iter().any(|name| name == expected),
            "missing index {expected}"
        );
    }
}

#[tokio::test]
async fn insert_then_get_round_trips_every_audit_event_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, enum, timestamp, optional,
    // JSON, or opaque-ID fields. Level: particular-integration. Source:
    // proposals/0b-15-wu-0b-15.md test intent "Round-trip".
    let (_pool, repo) = empty_auditevent_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_audit_event(fixture.row.clone())
        .await
        .expect("valid audit event should insert");
    let fetched = repo
        .get_audit_event(&fixture.row.audit_event_id)
        .await
        .expect("audit event get should succeed")
        .expect("inserted audit event should be returned");

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
async fn invalid_insert_paths_roll_back_without_partial_audit_rows() {
    // Risk: FK, enum, routing, or invariant failures leave durable rows. Level:
    // particular-integration. Source: proposals/0b-15-wu-0b-15.md test intent
    // "Transaction rollback".
    let (pool, repo) = empty_auditevent_fixture().await;
    let before = audit_event_row_count(&pool).await;
    let fk_fixture: FkRejectionsFixture = read_fixture(CONTRACT_FIXTURE_DIR, "fk-rejections.json");
    let invariant_fixture: OutputWithInputFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "output-with-input-invariants.json");

    let error = repo
        .insert_audit_event(fk_fixture.unknown_workspace)
        .await
        .expect_err("unknown workspace FK should fail");
    assert_error(error, GraphStoreError::UnknownRef);
    assert_eq!(audit_event_row_count(&pool).await, before);

    let error = repo
        .insert_audit_event(invariant_fixture.output_refs_without_input_refs)
        .await
        .expect_err("output refs without input refs should fail before insert");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(audit_event_row_count(&pool).await, before);
}

#[tokio::test]
async fn auditevent_wu_has_no_operator_visible_behavior_or_extra_tables() {
    // Risk: audit storage enables operator-visible behavior or extra tables.
    // Level: component structural. Source: proposals/0b-15-wu-0b-15.md test
    // intent "Side-effect absence".
    let (pool, _repo) = empty_auditevent_fixture().await;

    assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
    assert_eq!(
        sqlite_table_names(&pool).await,
        [
            "audit_events",
            "budget_ledgers",
            "evidence_artifacts",
            "graph_configurations",
            "graph_nodes",
            "graph_workspaces",
            "policy_sets",
            "provider_states",
            "schema_versions"
        ]
    );

    let source = auditevent_source();
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
            "auditevent source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn auditevent_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-15 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = auditevent_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::auditevent::{AuditDecision, AuditEvent};",
            "use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;",
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::GraphStoreError;",
            "use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;",
            "use agent_harness_lib::contracts::policyset::PolicySet;",
            "use agent_harness_lib::graphstore::auditevent::AuditEventRepo;",
            "use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;",
            "use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );
}

#[tokio::test]
async fn every_decision_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one decision variant is unreachable or fails serde/SQLx round-trip.
    // Level: unit/particular-integration. Source:
    // proposals/0b-15-wu-0b-15.md test intent "Decision variants".
    let (pool, repo) = empty_auditevent_fixture().await;
    let fixture: DecisionVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "decision-variants.json");
    assert_eq!(fixture.variants.len(), 5);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: AuditDecision =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS audit_decision_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM audit_decision_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO audit_decision_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM audit_decision_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: AuditDecision = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_audit_event(base.row.clone(), &format!("decision-{index}"));
        record.decision = variant;
        repo.insert_audit_event(record)
            .await
            .expect("decision variant should be reachable through insert");
    }

    let unknown: UnknownDecisionFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-decision.json");
    assert!(serde_json::from_value::<AuditDecision>(unknown.decision).is_err());
    let row = sqlx::query("SELECT 'escalated' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown decision row should select");
    assert!(row.try_get::<AuditDecision, _>("value").is_err());
}

#[test]
fn audit_events_repository_is_append_only_by_source_assertion() {
    // Risk: repository exposes UPDATE or DELETE paths for append-only audit
    // rows. Level: component structural. Source:
    // proposals/0b-15-wu-0b-15.md test intent "Append-only".
    let source = auditevent_source().to_ascii_lowercase();

    for forbidden in [
        "update audit_events",
        "delete from audit_events",
        "fn update_",
        "fn delete_",
        "pub async fn update_",
        "pub async fn delete_",
    ] {
        assert!(
            !source.contains(forbidden),
            "auditevent repository must not contain append-only violation {forbidden:?}"
        );
    }
}

#[tokio::test]
async fn output_refs_require_input_refs_policy_actor_decision_and_reason() {
    // Risk: output-producing audit rows omit input provenance or required
    // decision context. Level: particular-integration. Source:
    // proposals/0b-15-wu-0b-15.md test intent "Output-with-input invariant".
    let (pool, repo) = empty_auditevent_fixture().await;
    let fixture: OutputWithInputFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "output-with-input-invariants.json");
    let before = audit_event_row_count(&pool).await;

    for (name, record) in [
        (
            "output_refs_without_input_refs",
            fixture.output_refs_without_input_refs,
        ),
        (
            "output_refs_with_empty_policy_id",
            fixture.output_refs_with_empty_policy_id,
        ),
        (
            "output_refs_with_empty_actor",
            fixture.output_refs_with_empty_actor,
        ),
        (
            "output_refs_with_empty_reason_code",
            fixture.output_refs_with_empty_reason_code,
        ),
    ] {
        let error = match repo.insert_audit_event(record).await {
            Ok(_) => panic!("{name} should reject"),
            Err(error) => error,
        };
        assert_error(error, GraphStoreError::InvariantViolation);
        assert_eq!(audit_event_row_count(&pool).await, before);
    }

    let missing_decision_result = sqlx::query(
        "INSERT INTO audit_events
         (audit_event_id_value, audit_event_id_namespace,
          workspace_id_value, workspace_id_namespace,
          event_type, actor,
          policy_set_id_value, policy_set_id_namespace,
          input_refs, output_refs, decision, reason_code,
          created_at, record_created_at, record_updated_at, record_actor,
          record_policy_version, schema_version)
         VALUES
         ('audit-event-missing-decision', 'workspace-alpha',
          'workspace-alpha', 'workspace',
          'graph_node_state_changed', 'audit-author',
          'policy-set-round-trip-v1', 'workspace-alpha',
          '[\"graph-node-input-v1\"]', '[\"graph-node-output-v1\"]', NULL, 'policy.accepted',
          '2026-04-30T15:24:00Z', '2026-04-30T15:24:00Z',
          '2026-04-30T15:24:00Z', '{\"value\":\"audit-recorder\"}',
          'v1.0.0', 15)",
    )
    .execute(&pool)
    .await;
    assert!(
        missing_decision_result.is_err(),
        "missing decision raw insert should be rejected"
    );
    assert_eq!(audit_event_row_count(&pool).await, before);
}

#[tokio::test]
async fn unknown_workspace_policy_or_configuration_rejects_without_partial_rows() {
    // Risk: unknown predecessor refs insert orphaned audit rows. Level:
    // particular-integration. Source: proposals/0b-15-wu-0b-15.md test intent
    // "FK rejection".
    let (pool, repo) = empty_auditevent_fixture().await;
    let fixture: FkRejectionsFixture = read_fixture(CONTRACT_FIXTURE_DIR, "fk-rejections.json");
    let before = audit_event_row_count(&pool).await;

    for (name, record) in [
        ("unknown_workspace", fixture.unknown_workspace),
        ("unknown_policy_set", fixture.unknown_policy_set),
        ("unknown_configuration", fixture.unknown_configuration),
    ] {
        let error = match repo.insert_audit_event(record).await {
            Ok(_) => panic!("{name} should reject"),
            Err(error) => error,
        };
        assert_error(error, GraphStoreError::UnknownRef);
        assert_eq!(audit_event_row_count(&pool).await, before);
    }
}
