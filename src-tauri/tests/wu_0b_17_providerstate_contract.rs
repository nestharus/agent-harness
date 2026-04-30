use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::auditevent::ProviderStateRef;
use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::{GraphStoreError, OpaqueId};
use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::contracts::providerstate::{
    AuthState, BillingState, NetworkState, ProviderCli, ProviderConfidence, ProviderFreshness,
    ProviderKind, ProviderQuotaState, ProviderRuntimeState, ProviderState,
};
use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;
use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::graphstore::providerstate::ProviderStateRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-17";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";
const GRAPHCONFIGURATION_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";
const GRAPHWORKSPACE_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-06";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: ProviderState,
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
struct VariantsFixture<T> {
    variants: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct UnknownVariantsFixture {
    provider: Value,
    cli: Value,
    auth_state: Value,
    billing_state: Value,
    quota_state: Value,
    network_state: Value,
    runtime_state: Value,
    freshness: Value,
    confidence: Value,
}

#[derive(Debug, Deserialize)]
struct StaleMarkingFixture {
    reason: String,
    malformed_reasons: Vec<String>,
    unknown_provider_state_id: OpaqueId<ProviderStateRef>,
}

#[derive(Debug, Deserialize)]
struct SecretRejectionsFixture {
    secret_material_true: ProviderState,
    credential_payloads: Vec<CredentialPayloadFixture>,
}

#[derive(Debug, Deserialize)]
struct CredentialPayloadFixture {
    name: String,
    field: String,
    payload: String,
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

async fn empty_providerstate_fixture() -> (SqlitePool, ProviderStateRepo) {
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

    let repo = ProviderStateRepo::new(pool.clone());
    (pool, repo)
}

fn unique_provider_state(mut row: ProviderState, suffix: &str) -> ProviderState {
    row.provider_state_id.value = format!("provider-state-{suffix}");
    row.account_ref = format!("account:fixture:{suffix}");
    row.last_probe_at = "2026-04-30T17:30:00Z".to_string();
    row.meta.created_at = row.last_probe_at.clone();
    row.meta.updated_at = row.last_probe_at.clone();
    row
}

async fn provider_state_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM provider_states")
        .fetch_one(pool)
        .await
        .expect("provider_states row count should be readable");
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

fn providerstate_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("providerstate.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

#[tokio::test]
async fn provider_states_schema_contains_declared_columns_constraints_indexes_and_fks() {
    // Risk: ProviderState table misses declared columns, enum checks, JSON
    // checks, SQL secret-material invariant, indexes, or FK targets. Level:
    // particular-integration. Source: product-strategy/contracts/
    // wu-0b-17-providerstate.md.
    let (pool, _repo) = empty_providerstate_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'provider_states'",
    )
    .fetch_one(&pool)
    .await
    .expect("provider_states DDL should exist");

    for column in [
        "provider_state_id_value",
        "provider_state_id_namespace",
        "workspace_id_value",
        "workspace_id_namespace",
        "provider",
        "cli",
        "account_ref",
        "auth_state",
        "billing_state",
        "quota_state",
        "network_state",
        "runtime_state",
        "sandbox_constraints",
        "store_locations_checked",
        "secret_material_stored",
        "freshness",
        "confidence",
        "last_probe_at",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "provider_states DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(provider_state_id_value, provider_state_id_namespace)"));
    assert!(sql.0.contains("json_valid(sandbox_constraints)"));
    assert!(sql.0.contains("json_valid(store_locations_checked)"));
    assert!(sql.0.contains("secret_material_stored = 0"));

    let fks = sqlx::query("PRAGMA foreign_key_list(provider_states)")
        .fetch_all(&pool)
        .await
        .expect("provider_states foreign keys should be introspectable");
    for expected_table in ["schema_versions", "graph_workspaces"] {
        assert!(
            fks.iter()
                .any(|row| row.get::<String, _>("table") == expected_table),
            "provider_states should reference {expected_table}"
        );
    }

    for referencing_table in ["audit_events", "budget_ledgers"] {
        let fks = sqlx::query(&format!("PRAGMA foreign_key_list({referencing_table})"))
            .fetch_all(&pool)
            .await
            .expect("referencing-table foreign keys should be introspectable");
        assert!(
            fks.iter()
                .any(|row| row.get::<String, _>("table") == "provider_states"),
            "{referencing_table} should FK provider_state_id to provider_states"
        );
    }

    let indexes = sqlx::query("PRAGMA index_list(provider_states)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    for expected in [
        "idx_provider_states_workspace_id",
        "idx_provider_states_provider",
        "idx_provider_states_cli",
        "idx_provider_states_freshness",
        "idx_provider_states_last_probe_at",
    ] {
        assert!(
            index_names.iter().any(|name| name == expected),
            "missing index {expected}"
        );
    }
}

#[tokio::test]
async fn insert_then_get_round_trips_every_provider_state_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, enum, JSON, timestamp, bool,
    // or opaque-ID fields. Level: particular-integration. Source:
    // product-strategy/contracts/wu-0b-17-providerstate.md.
    let (_pool, repo) = empty_providerstate_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_provider_state(fixture.row.clone())
        .await
        .expect("valid provider state should insert");
    let fetched = repo
        .get_provider_state(&fixture.row.provider_state_id)
        .await
        .expect("provider state get should succeed")
        .expect("inserted provider state should be returned");

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
async fn invalid_insert_paths_roll_back_without_partial_provider_rows() {
    // Risk: FK, enum, routing, duplicate-ID, or invariant failures leave
    // durable rows. Level: particular-integration. Source:
    // product-strategy/contracts/wu-0b-17-providerstate.md.
    let (pool, repo) = empty_providerstate_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    let before = provider_state_row_count(&pool).await;

    let mut unknown_workspace = unique_provider_state(fixture.row.clone(), "unknown-workspace");
    unknown_workspace.workspace_id.value = "workspace-missing".to_string();
    let error = repo
        .insert_provider_state(unknown_workspace)
        .await
        .expect_err("unknown workspace FK should fail");
    assert_error(error, GraphStoreError::UnknownRef);
    assert_eq!(provider_state_row_count(&pool).await, before);

    let mut empty_account = unique_provider_state(fixture.row.clone(), "empty-account");
    empty_account.account_ref = " ".to_string();
    let error = repo
        .insert_provider_state(empty_account)
        .await
        .expect_err("empty account_ref should fail before insert");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(provider_state_row_count(&pool).await, before);
}

#[tokio::test]
async fn providerstate_wu_has_no_operator_visible_behavior_or_extra_tables() {
    // Risk: provider state storage enables operator-visible behavior or extra
    // tables. Level: component structural. Source: product-strategy/contracts/
    // wu-0b-17-providerstate.md.
    let (pool, _repo) = empty_providerstate_fixture().await;

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

    let source = providerstate_source();
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
            "providerstate source must not contain forbidden token {token:?}"
        );
    }
}

#[tokio::test]
async fn mark_provider_state_stale_updates_only_documented_columns() {
    // Risk: stale marking mutates provider/account/runtime/probe fields or
    // leaves undocumented partial writes. Level: particular-integration.
    // Source: product-strategy/contracts/wu-0b-17-providerstate.md.
    let (pool, repo) = empty_providerstate_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    let stale_fixture: StaleMarkingFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "stale-marking.json");

    repo.insert_provider_state(fixture.row.clone())
        .await
        .expect("valid provider state should insert");
    let before = repo
        .get_provider_state(&fixture.row.provider_state_id)
        .await
        .expect("get before stale should succeed")
        .expect("provider state should exist before stale");

    let after = repo
        .mark_provider_state_stale(&fixture.row.provider_state_id, &stale_fixture.reason)
        .await
        .expect("valid stale marking should succeed");

    assert_eq!(after.freshness, ProviderFreshness::Stale);
    assert_eq!(after.confidence, ProviderConfidence::Low);
    assert_ne!(after.meta.updated_at, before.meta.updated_at);
    assert_eq!(
        after.meta.actor.value,
        "provider-state-stale:auth_refresh".to_string()
    );

    let mut before_json = serde_json::to_value(before).expect("before should serialize");
    let mut after_json = serde_json::to_value(after.clone()).expect("after should serialize");
    for pointer in [
        "/freshness",
        "/confidence",
        "/meta/updated_at",
        "/meta/actor",
    ] {
        before_json
            .pointer_mut(pointer)
            .expect("before pointer should exist")
            .take();
        after_json
            .pointer_mut(pointer)
            .expect("after pointer should exist")
            .take();
    }
    assert_eq!(
        after_json, before_json,
        "mark_provider_state_stale must mutate only freshness, confidence, updated_at, and actor"
    );

    let raw = sqlx::query(
        "SELECT provider, cli, account_ref, auth_state, billing_state, quota_state,
                network_state, runtime_state, sandbox_constraints, store_locations_checked,
                secret_material_stored, last_probe_at, created_at, record_policy_version
         FROM provider_states
         WHERE provider_state_id_value = ? AND provider_state_id_namespace = ?",
    )
    .bind(&fixture.row.provider_state_id.value)
    .bind(&fixture.row.provider_state_id.namespace)
    .fetch_one(&pool)
    .await
    .expect("raw provider state should be readable");
    assert_eq!(raw.get::<String, _>("provider"), "openai");
    assert_eq!(raw.get::<String, _>("cli"), "codex");
    assert_eq!(
        raw.get::<String, _>("account_ref"),
        "account:openai:team-alpha"
    );
    assert_eq!(raw.get::<String, _>("auth_state"), "present");
    assert_eq!(raw.get::<String, _>("billing_state"), "healthy");
    assert_eq!(raw.get::<String, _>("quota_state"), "available");
    assert_eq!(raw.get::<String, _>("network_state"), "available");
    assert_eq!(raw.get::<String, _>("runtime_state"), "installed");
    assert_eq!(raw.get::<i64, _>("secret_material_stored"), 0);
    assert_eq!(
        raw.get::<String, _>("last_probe_at"),
        "2026-04-30T17:00:00Z"
    );
}

#[tokio::test]
async fn mark_provider_state_stale_rejects_unknown_id_and_malformed_reason_without_writes() {
    // Risk: stale marking silently creates rows or mutates rows on invalid
    // input. Level: particular-integration. Source:
    // product-strategy/contracts/wu-0b-17-providerstate.md.
    let (pool, repo) = empty_providerstate_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    let stale_fixture: StaleMarkingFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "stale-marking.json");

    repo.insert_provider_state(fixture.row.clone())
        .await
        .expect("valid provider state should insert");
    let before = repo
        .get_provider_state(&fixture.row.provider_state_id)
        .await
        .expect("get before rejection should succeed")
        .expect("provider state should exist before rejection");
    let before_count = provider_state_row_count(&pool).await;

    let error = repo
        .mark_provider_state_stale(
            &stale_fixture.unknown_provider_state_id,
            &stale_fixture.reason,
        )
        .await
        .expect_err("unknown provider state should reject");
    assert_error(error, GraphStoreError::UnknownRef);
    assert_eq!(provider_state_row_count(&pool).await, before_count);

    for reason in stale_fixture.malformed_reasons {
        let error = repo
            .mark_provider_state_stale(&fixture.row.provider_state_id, &reason)
            .await
            .expect_err("malformed stale reason should reject");
        assert_error(error, GraphStoreError::InvariantViolation);
        assert_eq!(provider_state_row_count(&pool).await, before_count);
    }

    let after = repo
        .get_provider_state(&fixture.row.provider_state_id)
        .await
        .expect("get after rejection should succeed")
        .expect("provider state should still exist");
    assert_eq!(
        serde_json::to_string(&after).expect("after row should serialize"),
        serde_json::to_string(&before).expect("before row should serialize")
    );
}

#[tokio::test]
async fn secret_material_true_and_credential_looking_payloads_reject_before_persistence() {
    // Risk: ProviderState stores secret material or credential-shaped payloads.
    // Level: particular-integration. Source: product-strategy/contracts/
    // wu-0b-17-providerstate.md.
    let (pool, repo) = empty_providerstate_fixture().await;
    let fixture: SecretRejectionsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "secret-rejections.json");
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");
    let before = provider_state_row_count(&pool).await;

    let error = repo
        .insert_provider_state(fixture.secret_material_true)
        .await
        .expect_err("secret_material_stored=true should reject before insert");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(provider_state_row_count(&pool).await, before);

    let raw_true_result = sqlx::query(
        "INSERT INTO provider_states
         (provider_state_id_value, provider_state_id_namespace,
          workspace_id_value, workspace_id_namespace,
          provider, cli, account_ref, auth_state, billing_state, quota_state,
          network_state, runtime_state, sandbox_constraints, store_locations_checked,
          secret_material_stored, freshness, confidence, last_probe_at,
          created_at, updated_at, actor, record_policy_version, schema_version)
         VALUES
         ('provider-state-raw-secret-true', 'provider-state',
          'workspace-alpha', 'workspace',
          'openai', 'codex', 'account:openai:team-alpha', 'present', 'healthy', 'available',
          'available', 'installed', '{\"network\":\"enabled\"}', '[\"codex_config\"]',
          1, 'fresh', 'high', '2026-04-30T17:11:00Z',
          '2026-04-30T17:11:00Z', '2026-04-30T17:11:00Z',
          '{\"value\":\"provider-state-recorder\"}', 'v1.0.0', 17)",
    )
    .execute(&pool)
    .await;
    assert!(
        raw_true_result.is_err(),
        "SQL CHECK should reject secret_material_stored=true"
    );
    assert_eq!(provider_state_row_count(&pool).await, before);

    for payload in fixture.credential_payloads {
        let mut record = unique_provider_state(base.row.clone(), &payload.name);
        match payload.field.as_str() {
            "account_ref" => record.account_ref = payload.payload,
            "sandbox_constraints" => {
                record.sandbox_constraints =
                    agent_harness_lib::contracts::providerstate::JsonField(json!({
                        "observed": payload.payload
                    }));
            }
            "store_locations_checked" => {
                record.store_locations_checked =
                    agent_harness_lib::contracts::providerstate::JsonField(vec![payload.payload]);
            }
            other => panic!("unknown credential payload field {other}"),
        }
        let error = repo
            .insert_provider_state(record)
            .await
            .expect_err("credential-shaped payload should reject before insert");
        assert_error(error, GraphStoreError::InvariantViolation);
        assert_eq!(provider_state_row_count(&pool).await, before);
    }
}

macro_rules! enum_round_trip_test {
    ($test_name:ident, $enum_ty:ty, $fixture:literal, $field:ident, $codec_table:literal, $unknown_field:ident, $unknown_sql:literal, $expected_count:expr) => {
        #[tokio::test]
        async fn $test_name() {
            // Risk: one documented enum variant is unreachable or fails
            // serde/SQLx round-trip. Level: unit/particular-integration.
            // Source: product-strategy/contracts/wu-0b-17-providerstate.md.
            let (pool, repo) = empty_providerstate_fixture().await;
            let fixture: VariantsFixture<$enum_ty> = read_fixture(CONTRACT_FIXTURE_DIR, $fixture);
            assert_eq!(fixture.variants.len(), $expected_count);
            let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

            for (index, variant) in fixture.variants.into_iter().enumerate() {
                let serialized = serde_json::to_value(variant).expect("variant should serialize");
                let decoded: $enum_ty =
                    serde_json::from_value(serialized).expect("variant should deserialize");
                assert_eq!(decoded, variant);

                sqlx::query(&format!(
                    "CREATE TEMP TABLE IF NOT EXISTS {} (value TEXT NOT NULL)",
                    $codec_table
                ))
                .execute(&pool)
                .await
                .expect("codec table should create");
                sqlx::query(&format!("DELETE FROM {}", $codec_table))
                    .execute(&pool)
                    .await
                    .expect("codec table should clear");
                sqlx::query(&format!("INSERT INTO {} (value) VALUES (?)", $codec_table))
                    .bind(variant)
                    .execute(&pool)
                    .await
                    .expect("variant should encode through SQLx");
                let row = sqlx::query(&format!("SELECT value FROM {}", $codec_table))
                    .fetch_one(&pool)
                    .await
                    .expect("variant row should select");
                let from_sql: $enum_ty = row.try_get("value").expect("variant should decode");
                assert_eq!(from_sql, variant);

                let mut record =
                    unique_provider_state(base.row.clone(), &format!("{}-{index}", $codec_table));
                record.$field = variant;
                repo.insert_provider_state(record)
                    .await
                    .expect("enum variant should be reachable through insert");
            }

            let unknown: UnknownVariantsFixture =
                read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
            assert!(serde_json::from_value::<$enum_ty>(unknown.$unknown_field).is_err());
            let row = sqlx::query(&format!("SELECT '{}' AS value", $unknown_sql))
                .fetch_one(&pool)
                .await
                .expect("unknown enum row should select");
            assert!(row.try_get::<$enum_ty, _>("value").is_err());
        }
    };
}

enum_round_trip_test!(
    every_provider_variant_round_trips_and_unknown_is_rejected,
    ProviderKind,
    "provider-variants.json",
    provider,
    "provider_kind_codec",
    provider,
    "mistral",
    6
);
enum_round_trip_test!(
    every_cli_variant_round_trips_and_unknown_is_rejected,
    ProviderCli,
    "cli-variants.json",
    cli,
    "provider_cli_codec",
    cli,
    "terminal",
    4
);
enum_round_trip_test!(
    every_auth_state_variant_round_trips_and_unknown_is_rejected,
    AuthState,
    "auth-state-variants.json",
    auth_state,
    "auth_state_codec",
    auth_state,
    "revoked",
    5
);
enum_round_trip_test!(
    every_billing_state_variant_round_trips_and_unknown_is_rejected,
    BillingState,
    "billing-state-variants.json",
    billing_state,
    "billing_state_codec",
    billing_state,
    "past_due",
    5
);
enum_round_trip_test!(
    every_quota_state_variant_round_trips_and_unknown_is_rejected,
    ProviderQuotaState,
    "quota-state-variants.json",
    quota_state,
    "quota_state_codec",
    quota_state,
    "warming",
    4
);
enum_round_trip_test!(
    every_network_state_variant_round_trips_and_unknown_is_rejected,
    NetworkState,
    "network-state-variants.json",
    network_state,
    "network_state_codec",
    network_state,
    "offline",
    5
);
enum_round_trip_test!(
    every_runtime_state_variant_round_trips_and_unknown_is_rejected,
    ProviderRuntimeState,
    "runtime-state-variants.json",
    runtime_state,
    "runtime_state_codec",
    runtime_state,
    "booting",
    6
);
enum_round_trip_test!(
    every_freshness_variant_round_trips_and_unknown_is_rejected,
    ProviderFreshness,
    "freshness-variants.json",
    freshness,
    "freshness_codec",
    freshness,
    "aged",
    4
);
enum_round_trip_test!(
    every_confidence_variant_round_trips_and_unknown_is_rejected,
    ProviderConfidence,
    "confidence-variants.json",
    confidence,
    "confidence_codec",
    confidence,
    "certain",
    3
);
