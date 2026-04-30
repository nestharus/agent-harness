use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::budgetledger::{
    BudgetLedger, BudgetScopeType, BudgetState, PolicyAction,
};
use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;
use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, GraphStoreSeedPlan,
};
use agent_harness_lib::contracts::graphstore_prelude::GraphStoreError;
use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;
use agent_harness_lib::contracts::policyset::PolicySet;
use agent_harness_lib::graphstore::budgetledger::BudgetLedgerRepo;
use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;
use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;
use agent_harness_lib::graphstore::policyset::PolicySetRepo;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Row, SqlitePool};

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-16";
const POLICYSET_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-04";
const GRAPHCONFIGURATION_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-05";
const GRAPHWORKSPACE_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-06";

#[derive(Debug, Deserialize)]
struct RowFixture {
    row: BudgetLedger,
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
struct ScopeTypeVariantsFixture {
    variants: Vec<BudgetScopeType>,
}

#[derive(Debug, Deserialize)]
struct BudgetStateVariantsFixture {
    variants: Vec<BudgetState>,
}

#[derive(Debug, Deserialize)]
struct PolicyActionVariantsFixture {
    variants: Vec<PolicyAction>,
}

#[derive(Debug, Deserialize)]
struct UnknownVariantsFixture {
    scope_type: Value,
    budget_state: Value,
    policy_action: Value,
}

#[derive(Debug, Deserialize)]
struct CounterRejectionsFixture {
    zero_counters: BudgetLedger,
    negative_values: Vec<NegativeCounterFixture>,
}

#[derive(Debug, Deserialize)]
struct NegativeCounterFixture {
    field: String,
    value: i64,
}

#[derive(Debug, Deserialize)]
struct PolicyActionInvariantsFixture {
    legal: Vec<PolicyActionPair>,
    invalid: Vec<PolicyActionPair>,
}

#[derive(Debug, Deserialize)]
struct PolicyActionPair {
    budget_state: BudgetState,
    policy_action: PolicyAction,
}

#[derive(Debug, Deserialize)]
struct FkRejectionsFixture {
    unknown_workspace: BudgetLedger,
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

async fn empty_budgetledger_fixture() -> (SqlitePool, BudgetLedgerRepo) {
    let fixture = create_graphstore_fixture(GraphStoreSeedPlan::default())
        .await
        .expect("empty graphstore fixture should apply migrations");
    assert_eq!(
        fixture.pool.migrations_applied,
        vec![1, 4, 5, 6, 7, 9, 15, 16]
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

    let repo = BudgetLedgerRepo::new(pool.clone());
    (pool, repo)
}

fn unique_ledger(mut row: BudgetLedger, suffix: &str) -> BudgetLedger {
    row.budget_ledger_id.value = format!("budget-ledger-{suffix}");
    row.scope_id.value = format!("budget-scope-{suffix}");
    row.cache_prefix_hash = Some(format!("cache-prefix-{suffix}"));
    row.meta.created_at = "2026-04-30T16:40:00Z".to_string();
    row.meta.updated_at = row.meta.created_at.clone();
    row
}

async fn budget_ledger_row_count(pool: &SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM budget_ledgers")
        .fetch_one(pool)
        .await
        .expect("budget_ledgers row count should be readable");
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

fn budgetledger_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("budgetledger.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn budgetledger_contract_test_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wu_0b_16_budgetledger_contract.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_error(error: GraphStoreError, expected: GraphStoreError) {
    assert_eq!(error, expected);
    assert_eq!(error.code(), expected.code());
}

#[tokio::test]
async fn budget_ledgers_schema_contains_declared_columns_constraints_and_indexes() {
    // Risk: budget table misses columns, FK, unique constraints, counter
    // checks, enum checks, policy matrix, indexes, or RecordMeta fields.
    // Level: particular-integration. Source: proposals/0b-16-wu-0b-16.md
    // test intent "Schema creation".
    let (pool, _repo) = empty_budgetledger_fixture().await;

    let sql: (String,) = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'budget_ledgers'",
    )
    .fetch_one(&pool)
    .await
    .expect("budget_ledgers DDL should exist");

    for column in [
        "budget_ledger_id_value",
        "budget_ledger_id_namespace",
        "workspace_id_value",
        "workspace_id_namespace",
        "scope_type",
        "scope_id_value",
        "scope_id_namespace",
        "input_tokens",
        "output_tokens",
        "cache_read_tokens",
        "cache_write_tokens",
        "latency_ms",
        "provider_cost_estimate",
        "provider_state_id_value",
        "provider_state_id_namespace",
        "cache_prefix_hash",
        "budget_state",
        "policy_action",
        "created_at",
        "updated_at",
        "actor",
        "record_policy_version",
        "schema_version",
    ] {
        assert!(
            sql.0.contains(column),
            "budget_ledgers DDL should contain {column}"
        );
    }
    assert!(sql
        .0
        .contains("UNIQUE(budget_ledger_id_value, budget_ledger_id_namespace)"));
    assert!(sql.0.contains("json_valid(actor)"));
    assert!(sql.0.contains("input_tokens >= 0"));
    assert!(sql
        .0
        .contains("budget_state = 'blocked' AND policy_action = 'block'"));
    assert!(sql
        .0
        .contains("FOREIGN KEY(schema_version) REFERENCES schema_versions(version)"));

    let fks = sqlx::query("PRAGMA foreign_key_list(budget_ledgers)")
        .fetch_all(&pool)
        .await
        .expect("foreign keys should be introspectable");
    for expected_table in ["schema_versions", "graph_workspaces"] {
        assert!(
            fks.iter()
                .any(|row| row.get::<String, _>("table") == expected_table),
            "budget_ledgers should reference {expected_table}"
        );
    }
    assert!(
        !fks.iter()
            .any(|row| row.get::<String, _>("table").contains("provider")),
        "provider_state_id must remain a soft ref in WU-0B-16"
    );

    let indexes = sqlx::query("PRAGMA index_list(budget_ledgers)")
        .fetch_all(&pool)
        .await
        .expect("indexes should be introspectable");
    let index_names = indexes
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();
    for expected in [
        "idx_budget_ledgers_workspace_id",
        "idx_budget_ledgers_scope_type",
        "idx_budget_ledgers_budget_state",
    ] {
        assert!(
            index_names.iter().any(|name| name == expected),
            "missing index {expected}"
        );
    }
}

#[tokio::test]
async fn insert_then_get_round_trips_every_budget_ledger_field_byte_equivalent() {
    // Risk: repository loses or rewrites scalar, enum, timestamp, optional,
    // counter, or opaque-ID fields. Level: particular-integration. Source:
    // proposals/0b-16-wu-0b-16.md test intent "Round-trip".
    let (_pool, repo) = empty_budgetledger_fixture().await;
    let fixture: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "round-trip.json");
    let expected = serde_json::to_string(&fixture.row).expect("fixture row should serialize");

    let inserted = repo
        .insert_budget_ledger(fixture.row.clone())
        .await
        .expect("valid budget ledger should insert");
    let fetched = repo
        .get_budget_ledger(&fixture.row.budget_ledger_id)
        .await
        .expect("budget ledger get should succeed")
        .expect("inserted budget ledger should be returned");

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
async fn invalid_insert_paths_roll_back_without_partial_budget_rows() {
    // Risk: FK, enum, routing, or state-transition validation failures leave
    // durable rows. Level: particular-integration. Source:
    // proposals/0b-16-wu-0b-16.md test intent "Transaction rollback".
    let (pool, repo) = empty_budgetledger_fixture().await;
    let before = budget_ledger_row_count(&pool).await;
    let fk_fixture: FkRejectionsFixture = read_fixture(CONTRACT_FIXTURE_DIR, "fk-rejections.json");
    let counter_fixture: CounterRejectionsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "counter-rejections.json");
    let mut blocked_warn = counter_fixture.zero_counters.clone();
    blocked_warn.budget_ledger_id.value = "budget-ledger-blocked-warn".to_string();
    blocked_warn.budget_state = BudgetState::Blocked;
    blocked_warn.policy_action = PolicyAction::Warn;

    let error = repo
        .insert_budget_ledger(fk_fixture.unknown_workspace)
        .await
        .expect_err("unknown workspace FK should fail");
    assert_error(error, GraphStoreError::UnknownRef);
    assert_eq!(budget_ledger_row_count(&pool).await, before);

    let error = repo
        .insert_budget_ledger(record_with_counter(
            counter_fixture.zero_counters.clone(),
            "input_tokens",
            -1,
        ))
        .await
        .expect_err("negative input tokens should fail");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(budget_ledger_row_count(&pool).await, before);

    let error = repo
        .insert_budget_ledger(blocked_warn)
        .await
        .expect_err("blocked plus warn should fail");
    assert_error(error, GraphStoreError::InvariantViolation);
    assert_eq!(budget_ledger_row_count(&pool).await, before);
}

#[tokio::test]
async fn budgetledger_wu_has_no_operator_visible_behavior_or_extra_tables() {
    // Risk: budget storage enables operator-visible behavior or extra tables.
    // Level: component structural. Source: proposals/0b-16-wu-0b-16.md test
    // intent "Side-effect absence".
    let (pool, _repo) = empty_budgetledger_fixture().await;

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
            "schema_versions"
        ]
    );

    let source = budgetledger_source();
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
            "budgetledger source must not contain forbidden token {token:?}"
        );
    }
}

#[test]
fn budgetledger_contract_tests_stay_inside_declared_wu_boundary() {
    // Risk: tests inspect services outside the WU boundary. Level: component.
    // Source: WU-0B-16 acceptance criterion "Contract tests are written only
    // from this WU contract".
    let source = budgetledger_contract_test_source();
    let harness_imports = source
        .lines()
        .filter(|line| line.starts_with("use agent_harness_lib::"))
        .collect::<Vec<_>>();

    assert_eq!(
        harness_imports,
        vec![
            "use agent_harness_lib::contracts::budgetledger::{",
            "use agent_harness_lib::contracts::graphconfiguration::GraphConfiguration;",
            "use agent_harness_lib::contracts::graphstore_fixture::{",
            "use agent_harness_lib::contracts::graphstore_prelude::GraphStoreError;",
            "use agent_harness_lib::contracts::graphworkspace::GraphWorkspace;",
            "use agent_harness_lib::contracts::policyset::PolicySet;",
            "use agent_harness_lib::graphstore::budgetledger::BudgetLedgerRepo;",
            "use agent_harness_lib::graphstore::graphconfiguration::GraphConfigurationRepo;",
            "use agent_harness_lib::graphstore::graphworkspace::GraphWorkspaceRepo;",
            "use agent_harness_lib::graphstore::policyset::PolicySetRepo;",
            "use agent_harness_lib::phase_0a_scaffold_commands;",
        ]
    );
}

#[tokio::test]
async fn every_scope_type_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one scope_type variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-16-wu-0b-16.md test intent "Scope type variants".
    let (pool, repo) = empty_budgetledger_fixture().await;
    let fixture: ScopeTypeVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "scope-type-variants.json");
    assert_eq!(fixture.variants.len(), 7);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: BudgetScopeType =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query(
            "CREATE TEMP TABLE IF NOT EXISTS budget_scope_type_codec (value TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .expect("codec table should create");
        sqlx::query("DELETE FROM budget_scope_type_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO budget_scope_type_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM budget_scope_type_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: BudgetScopeType = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_ledger(base.row.clone(), &format!("scope-type-{index}"));
        record.scope_type = variant;
        repo.insert_budget_ledger(record)
            .await
            .expect("scope_type variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<BudgetScopeType>(unknown.scope_type).is_err());
    let row = sqlx::query("SELECT 'workspace_span' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown scope_type row should select");
    assert!(row.try_get::<BudgetScopeType, _>("value").is_err());
}

#[tokio::test]
async fn every_budget_state_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one budget_state variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-16-wu-0b-16.md test intent "Budget state variants".
    let (pool, repo) = empty_budgetledger_fixture().await;
    let fixture: BudgetStateVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "budget-state-variants.json");
    assert_eq!(fixture.variants.len(), 4);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: BudgetState =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS budget_state_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM budget_state_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO budget_state_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM budget_state_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: BudgetState = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_ledger(base.row.clone(), &format!("budget-state-{index}"));
        record.budget_state = variant;
        record.policy_action = match variant {
            BudgetState::Within => PolicyAction::None,
            BudgetState::NearLimit => PolicyAction::Warn,
            BudgetState::Exceeded => PolicyAction::RequireUserApproval,
            BudgetState::Blocked => PolicyAction::Block,
        };
        repo.insert_budget_ledger(record)
            .await
            .expect("budget_state variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<BudgetState>(unknown.budget_state).is_err());
    let row = sqlx::query("SELECT 'critical' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown budget_state row should select");
    assert!(row.try_get::<BudgetState, _>("value").is_err());
}

#[tokio::test]
async fn every_policy_action_variant_round_trips_and_unknown_is_rejected() {
    // Risk: one policy_action variant is unreachable or fails serde/SQLx
    // round-trip. Level: unit/particular-integration. Source:
    // proposals/0b-16-wu-0b-16.md test intent "Policy action variants".
    let (pool, repo) = empty_budgetledger_fixture().await;
    let fixture: PolicyActionVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "policy-action-variants.json");
    assert_eq!(fixture.variants.len(), 5);
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, variant) in fixture.variants.into_iter().enumerate() {
        let serialized = serde_json::to_value(variant).expect("variant should serialize");
        let decoded: PolicyAction =
            serde_json::from_value(serialized).expect("variant should deserialize");
        assert_eq!(decoded, variant);

        sqlx::query("CREATE TEMP TABLE IF NOT EXISTS policy_action_codec (value TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("codec table should create");
        sqlx::query("DELETE FROM policy_action_codec")
            .execute(&pool)
            .await
            .expect("codec table should clear");
        sqlx::query("INSERT INTO policy_action_codec (value) VALUES (?)")
            .bind(variant)
            .execute(&pool)
            .await
            .expect("variant should encode through SQLx");
        let row = sqlx::query("SELECT value FROM policy_action_codec")
            .fetch_one(&pool)
            .await
            .expect("variant row should select");
        let from_sql: PolicyAction = row.try_get("value").expect("variant should decode");
        assert_eq!(from_sql, variant);

        let mut record = unique_ledger(base.row.clone(), &format!("policy-action-{index}"));
        record.policy_action = variant;
        record.budget_state = match variant {
            PolicyAction::None | PolicyAction::Warn => BudgetState::Within,
            PolicyAction::NarrowScope | PolicyAction::RequireUserApproval => BudgetState::Exceeded,
            PolicyAction::Block => BudgetState::Blocked,
        };
        repo.insert_budget_ledger(record)
            .await
            .expect("policy_action variant should be reachable through insert");
    }

    let unknown: UnknownVariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "unknown-variants.json");
    assert!(serde_json::from_value::<PolicyAction>(unknown.policy_action).is_err());
    let row = sqlx::query("SELECT 'auto_escalate' AS value")
        .fetch_one(&pool)
        .await
        .expect("unknown policy_action row should select");
    assert!(row.try_get::<PolicyAction, _>("value").is_err());
}

#[tokio::test]
async fn counters_preserve_zero_and_reject_negative_values() {
    // Risk: token, latency, or provider cost counters accept negative values
    // or reject valid zero measurements. Level: particular-integration.
    // Source: proposals/0b-16-wu-0b-16.md test intent "Counter
    // validation".
    let (pool, repo) = empty_budgetledger_fixture().await;
    let fixture: CounterRejectionsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "counter-rejections.json");
    let before = budget_ledger_row_count(&pool).await;

    repo.insert_budget_ledger(fixture.zero_counters.clone())
        .await
        .expect("zero counters should be valid measured values");
    assert_eq!(budget_ledger_row_count(&pool).await, before + 1);

    for negative in fixture.negative_values {
        let mut record = unique_ledger(
            fixture.zero_counters.clone(),
            &format!("negative-{}", negative.field),
        );
        record = record_with_counter(record, &negative.field, negative.value);
        let count_before_rejection = budget_ledger_row_count(&pool).await;
        let error = match repo.insert_budget_ledger(record).await {
            Ok(_) => panic!("negative {} should reject", negative.field),
            Err(error) => error,
        };
        assert_error(error, GraphStoreError::InvariantViolation);
        assert_eq!(budget_ledger_row_count(&pool).await, count_before_rejection);
    }
}

#[tokio::test]
async fn policy_action_matrix_accepts_only_documented_pairs() {
    // Risk: blocked rows avoid block, within rows require user approval, or
    // legal matrix pairs are rejected. Level: particular-integration. Source:
    // proposals/0b-16-wu-0b-16.md test intent "Policy-action invariant".
    let (pool, repo) = empty_budgetledger_fixture().await;
    let fixture: PolicyActionInvariantsFixture =
        read_fixture(CONTRACT_FIXTURE_DIR, "policy-action-invariants.json");
    let base: RowFixture = read_fixture(CONTRACT_FIXTURE_DIR, "canonical-row.json");

    for (index, pair) in fixture.legal.into_iter().enumerate() {
        let mut record = unique_ledger(base.row.clone(), &format!("legal-policy-action-{index}"));
        record.budget_state = pair.budget_state;
        record.policy_action = pair.policy_action;
        repo.insert_budget_ledger(record)
            .await
            .expect("legal policy-action pair should insert");
    }

    let before_invalid = budget_ledger_row_count(&pool).await;
    for (index, pair) in fixture.invalid.into_iter().enumerate() {
        let mut record = unique_ledger(base.row.clone(), &format!("invalid-policy-action-{index}"));
        record.budget_state = pair.budget_state;
        record.policy_action = pair.policy_action;
        let error = repo
            .insert_budget_ledger(record)
            .await
            .expect_err("invalid policy-action pair should reject");
        assert_error(error, GraphStoreError::InvariantViolation);
        assert_eq!(budget_ledger_row_count(&pool).await, before_invalid);
    }
}

fn record_with_counter(mut record: BudgetLedger, field: &str, value: i64) -> BudgetLedger {
    match field {
        "input_tokens" => record.input_tokens = value,
        "output_tokens" => record.output_tokens = value,
        "cache_read_tokens" => record.cache_read_tokens = value,
        "cache_write_tokens" => record.cache_write_tokens = value,
        "latency_ms" => record.latency_ms = value,
        "provider_cost_estimate" => record.provider_cost_estimate = value,
        other => panic!("unknown counter field {other}"),
    }
    record
}
