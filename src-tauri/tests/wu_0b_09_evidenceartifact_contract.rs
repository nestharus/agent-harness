use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::evidenceartifact::{
    CaptureState, EvidenceArtifact, PrivilegeOrigin, SourceType, ToolProtocol,
};
use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::GraphStoreError;
use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::graphstore::evidenceartifact::EvidenceArtifactRepo;
use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;
use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-09";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";
const GRAPHCONFIGURATION_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";
const GRAPHWORKSPACE_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-06";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: EvidenceArtifact,
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
struct SourceTypeVariantsFixture {
    variants: Vec<SourceType>,
}

#[derive(Debug, Deserialize)]
struct ToolProtocolVariantsFixture {
    variants: Vec<ToolProtocol>,
}

#[derive(Debug, Deserialize)]
struct CaptureStateVariantsFixture {
    variants: Vec<CaptureState>,
}

#[derive(Debug, Deserialize)]
struct PrivilegeOriginVariantsFixture {
    variants: Vec<PrivilegeOrigin>,
}

#[derive(Debug, Deserialize)]
struct UnknownVariantsFixture {
    source_type: Value,
    tool_protocol: Value,
    capture_state: Value,
    privilege_origin: Value,
}

#[derive(Debug, Deserialize)]
struct ContentHashInvariantsFixture {
    captured_missing_hash: EvidenceArtifact,
    failed_missing_hash_missing_failure_ref: EvidenceArtifact,
    failed_missing_hash_with_failure_ref: EvidenceArtifact,
}

#[derive(Debug, Deserialize)]
struct PathEscapeAttemptsFixture {
    parent_path_blob_ref: String,
    absolute_blob_ref: String,
    symlink_blob_ref: String,
}

#[derive(Debug, Deserialize)]
struct RollbackFixture {
    unknown_workspace: EvidenceArtifact,
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

async fn empty_evidenceartifact_fixture() -> (SqlitePool, EvidenceArtifactRepo, PathBuf) {
    let fixture = create_graphstore_fixture(GraphStoreSeedPlan::default())
        .await
        .expect("empty graphstore fixture should apply migrations");
    assert_eq!(fixture.pool.migrations_applied, vec![1, 4, 5, 6, 7, 9]);
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

    let repo = EvidenceArtifactRepo::new(pool.clone());
    (pool, repo, storage_root)
}

fn create_blob(storage_root: &Path, blob_ref: &str) {
    let path = storage_root.join(blob_ref);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("failed to create {}: {error}", parent.display()));
    }
    fs::write(&path, format!("payload for {blob_ref}"))
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

fn unique_evidence(mut row: EvidenceArtifact, suffix: &str) -> EvidenceArtifact {
    row.evidence_id.value = format!("evidence-artifact-{suffix}");
    row.blob_ref = format!("evidence/{suffix}.txt");
    row.source_uri = format!("agent://contract/{suffix}");
    row.source_session_id = Some(format!("session-{suffix}"));
    row.correlation_key = Some(format!("corr-{suffix}"));
    row
}

async fn evidence_artifact_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM evidence_artifacts")
        .fetch_one(pool)
        .await
        .expect("evidence_artifacts row count should be readable");
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

fn evidenceartifact_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("evidenceartifact.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn evidenceartifact_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_09_evidenceartifact_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

#[tokio::test]
async fn evidence_artifacts_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: evidence table misses columns, FK, unique constraints, JSON checks,
    // indexes, or RecordMeta fields. Level: particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "Schema creation".
    let (pool, _repo, _storage_root) = empty_evidenceartifact_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'evidence_artifacts'",
    )
    .fetch_one(&pool)
    .await
    .expect("evidence_artifacts DDL should exist");

    for column in [
        "evidence_id_value",
        "evidence_id_namespace",
        "workspace_id_value",
        "workspace_id_namespace",
        "source_type",
        "source_uri",
        "source_session_id",
        "tool_protocol",
        "correlation_key",
        "content_hash",
        "blob_ref",
        "privilege_origin",
        "capture_state",
        "captured_at",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "evidence_artifacts DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(evidence_id_value, evidence_id_namespace)"));
    assert!(sql.0.contains("json_valid(actor)"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(evidence_artifacts)")
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

    let indexes = sqlx::query("PRAGMA index_list(evidence_artifacts)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    for expected in [
        "idx_evidence_artifacts_workspace_id",
        "idx_evidence_artifacts_content_hash",
        "idx_evidence_artifacts_capture_state",
    ] {
        assert!(
            index_names.iter().any(|name| name == expected),
            "missing index {expected}"
        );
    }
}

#[tokio::test]
async fn insert_then_get_round_trips_every_evidence_artifact_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, enum, timestamp, optional,
    // blob, hash, or opaque-ID fields. Level: particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "Round-trip".
    let (_pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    create_blob(&storage_root, &fixture.row.blob_ref);
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_evidence_artifact(fixture.row.clone())
        .await
        .expect("valid evidence artifact should insert");
    let fetched = repo
        .get_evidence_artifact(&fixture.row.evidence_id)
        .await
        .expect("evidence artifact get should succeed")
        .expect("inserted evidence artifact should be returned");

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
async fn invalid_insert_paths_roll_back_without_partial_evidence_rows() {
    // Risk: FK, enum, routing, or invariant failures leave durable rows. Level:
    // particular-integration. Source: proposals/0b-09-wu-0b-09.md test intent
    // "Transaction rollback".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let before = evidence_artifact_row_count(&pool).await;

    let fixture: RollbackFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "transaction-rollback-fk.json");
    create_blob(&storage_root, &fixture.unknown_workspace.blob_ref);
    let error = repo
        .insert_evidence_artifact(fixture.unknown_workspace)
        .await
        .expect_err("unknown workspace FK should fail");
    assert_error(error, GraphStoreError::UnknownRef);
    assert_eq!(evidence_artifact_row_count(&pool).await, before);

    let mut invalid_routing: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    invalid_routing.row.source_uri.clear();
    create_blob(&storage_root, &invalid_routing.row.blob_ref);
    let error = repo
        .insert_evidence_artifact(invalid_routing.row)
        .await
        .expect_err("invalid source uri should fail before insert");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(evidence_artifact_row_count(&pool).await, before);
}

#[tokio::test]
async fn evidenceartifact_wu_has_no_operator_visible_behavior_or_extra_tables() {
    // Risk: evidence storage enables operator-visible behavior or extra tables.
    // Level: component structural. Source: proposals/0b-09-wu-0b-09.md test
    // intent "Side-effect absence".
    let (pool, _repo, _storage_root) = empty_evidenceartifact_fixture().await;

    assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
    assert_eq!(
        sqlite_table_names(&pool).await,
        [
            "evidence_artifacts",
            "graph_configurations",
            "graph_nodes",
            "graph_workspaces",
            "policy_sets",
            "schema_versions"
        ]
    );

    let source = evidenceartifact_source();
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
            "evidenceartifact source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn evidenceartifact_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-09 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = evidenceartifact_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::evidenceartifact::{",
            "use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;",
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::GraphStoreError;",
            "use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;",
            "use agent_harness_lib::contracts::policyset::PolicySet;",
            "use agent_harness_lib::graphstore::evidenceartifact::EvidenceArtifactRepo;",
            "use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;",
            "use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );
}

#[tokio::test]
async fn every_source_type_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one source_type variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "SourceType variants".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let fixture: SourceTypeVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "source-type-variants.json");
    assert_eq!(fixture.variants.len(), 11);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: SourceType =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS source_type_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM source_type_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO source_type_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM source_type_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: SourceType = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_evidence(base.row.clone(), &format!("source-type-{index}"));
        record.source_type = variant;
        create_blob(&storage_root, &record.blob_ref);
        repo.insert_evidence_artifact(record)
            .await
            .expect("source_type variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<SourceType>(unknown.source_type).is_err());
    let row = sqlx::query("SELECT 'screen_recording' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown source_type row should select");
    assert!(row.try_get::<SourceType, _>("value").is_err());
}

#[tokio::test]
async fn every_tool_protocol_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one tool_protocol variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "ToolProtocol variants".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let fixture: ToolProtocolVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "tool-protocol-variants.json");
    assert_eq!(fixture.variants.len(), 6);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: ToolProtocol =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS tool_protocol_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM tool_protocol_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO tool_protocol_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM tool_protocol_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: ToolProtocol = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_evidence(base.row.clone(), &format!("tool-protocol-{index}"));
        record.tool_protocol = variant;
        create_blob(&storage_root, &record.blob_ref);
        repo.insert_evidence_artifact(record)
            .await
            .expect("tool_protocol variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<ToolProtocol>(unknown.tool_protocol).is_err());
    let row = sqlx::query("SELECT 'browser' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown tool_protocol row should select");
    assert!(row.try_get::<ToolProtocol, _>("value").is_err());
}

#[tokio::test]
async fn every_capture_state_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one capture_state variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "CaptureState variants".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let fixture: CaptureStateVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "capture-state-variants.json");
    assert_eq!(fixture.variants.len(), 5);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: CaptureState =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS capture_state_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM capture_state_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO capture_state_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM capture_state_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: CaptureState = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_evidence(base.row.clone(), &format!("capture-state-{index}"));
        record.capture_state = variant;
        if variant == CaptureState::Failed {
            record.content_hash = None;
        }
        create_blob(&storage_root, &record.blob_ref);
        repo.insert_evidence_artifact(record)
            .await
            .expect("capture_state variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<CaptureState>(unknown.capture_state).is_err());
    let row = sqlx::query("SELECT 'pending' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown capture_state row should select");
    assert!(row.try_get::<CaptureState, _>("value").is_err());
}

#[tokio::test]
async fn every_privilege_origin_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one privilege_origin variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "PrivilegeOrigin variants".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
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

        let mut record = unique_evidence(base.row.clone(), &format!("privilege-{index}"));
        record.privilege_origin = variant;
        create_blob(&storage_root, &record.blob_ref);
        repo.insert_evidence_artifact(record)
            .await
            .expect("privilege_origin variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<PrivilegeOrigin>(unknown.privilege_origin).is_err());
    let row = sqlx::query("SELECT 'administrator' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown privilege_origin row should select");
    assert!(row.try_get::<PrivilegeOrigin, _>("value").is_err());
}

#[tokio::test]
async fn content_hash_is_required_except_failed_with_failure_payload_ref() {
    // Risk: capture-state/content-hash invariant accepts missing content or
    // rejects valid failure payload records. Level: particular-integration.
    // Source: proposals/0b-09-wu-0b-09.md test intent "Content hash invariant".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let fixture: ContentHashInvariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "content-hash-invariants.json");
    let before = evidence_artifact_row_count(&pool).await;

    create_blob(&storage_root, &fixture.captured_missing_hash.blob_ref);
    let error = repo
        .insert_evidence_artifact(fixture.captured_missing_hash)
        .await
        .expect_err("captured evidence without hash should fail");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(evidence_artifact_row_count(&pool).await, before);

    let error = repo
        .insert_evidence_artifact(fixture.failed_missing_hash_missing_failure_ref)
        .await
        .expect_err("failed evidence without hash or failure payload ref should fail");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(evidence_artifact_row_count(&pool).await, before);

    create_blob(
        &storage_root,
        &fixture.failed_missing_hash_with_failure_ref.blob_ref,
    );
    repo.insert_evidence_artifact(fixture.failed_missing_hash_with_failure_ref)
        .await
        .expect("failed evidence with failure payload ref should insert");
    assert_eq!(evidence_artifact_row_count(&pool).await, before + 1);
}

#[tokio::test]
async fn blob_refs_must_remain_under_workspace_storage_root() {
    // Risk: blob refs escape workspace storage via parent paths, absolute
    // paths, or symlinks. Level: particular-integration. Source:
    // proposals/0b-09-wu-0b-09.md test intent "Blob path guard".
    let (pool, repo, storage_root) = empty_evidenceartifact_fixture().await;
    let attempts: PathEscapeAttemptsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "path-escape-attempts.json");
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    let before = evidence_artifact_row_count(&pool).await;

    let mut parent_path = unique_evidence(base.row.clone(), "path-parent");
    parent_path.blob_ref = attempts.parent_path_blob_ref;
    let error = repo
        .insert_evidence_artifact(parent_path)
        .await
        .expect_err("parent path blob ref should fail");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(evidence_artifact_row_count(&pool).await, before);

    let mut absolute_path = unique_evidence(base.row.clone(), "path-absolute");
    absolute_path.blob_ref = attempts.absolute_blob_ref;
    let error = repo
        .insert_evidence_artifact(absolute_path)
        .await
        .expect_err("absolute blob ref should fail");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(evidence_artifact_row_count(&pool).await, before);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let outside = storage_root
            .parent()
            .expect("storage root should have a parent")
            .join("outside-evidence.txt");
        fs::write(&outside, "outside payload")
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", outside.display()));
        let symlink_path = storage_root.join(&attempts.symlink_blob_ref);
        fs::create_dir_all(
            symlink_path
                .parent()
                .expect("symlink fixture should have a parent"),
        )
        .expect("symlink parent should be creatable");
        symlink(&outside, &symlink_path).unwrap_or_else(|error| {
            panic!("failed to symlink {}: {error}", symlink_path.display())
        });

        let mut symlink_escape = unique_evidence(base.row.clone(), "path-symlink");
        symlink_escape.blob_ref = attempts.symlink_blob_ref;
        let error = repo
            .insert_evidence_artifact(symlink_escape)
            .await
            .expect_err("symlink escape blob ref should fail");
        assert_error(error, GraphStoreError::InvariantViolation);
        assert_eq!(evidence_artifact_row_count(&pool).await, before);
    }
}
