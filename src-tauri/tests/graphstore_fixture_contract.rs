use std::fs;
use std::path::{Path, PathBuf};

use agent_harness_lib::contracts::graphstore_fixture::{
    create_graphstore_fixture, reset_graphstore_fixture, validate_fixture_created_refs,
    GraphStoreError, GraphStoreSeedPlan, GraphStoreSeedRef,
};
use agent_harness_lib::contracts::graphstore_migrations::MigrationRecord;
use agent_harness_lib::graphstore::migrations::run_migrations;
use agent_harness_lib::phase_0a_scaffold_commands;
use serde::Deserialize;
use sqlx::SqlitePool;

const CONTRACT_FIXTURE_DIR: &str = "product-strategy/contracts/fixtures/wu-0b-03";

#[derive(Debug, Deserialize)]
struct UnknownRefSeedFixture {
    future_refs: Vec<GraphStoreSeedRef>,
    expected_error: GraphStoreError,
    expected_code: String,
}

#[derive(Debug, Deserialize)]
struct NoSideEffectsFixture {
    forbidden_source_tokens: Vec<String>,
    allowed_migration_files: Vec<String>,
    allowed_runtime_tables: Vec<String>,
    phase_0a_scaffold_commands: Vec<String>,
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

fn migrations_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

fn graphstore_fixture_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graphstore")
        .join("fixture.rs");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

async fn migration_records(pool: &SqlitePool) -> Vec<MigrationRecord> {
    sqlx::query_as::<_, MigrationRecord>(
        "SELECT version, migration_name, applied_at, checksum, execution_ms, applied_by, rollback_state
         FROM schema_versions
         ORDER BY version",
    )
    .fetch_all(pool)
    .await
    .expect("schema_versions rows should be readable")
}

async fn sqlite_table_names(pool: &SqlitePool) -> Vec<String> {
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .fetch_all(pool)
            .await
            .expect("sqlite_master should be readable");
    rows.into_iter().map(|(name,)| name).collect()
}

#[tokio::test]
async fn empty_seed_plan_creates_isolated_schema_versions_fixture() {
    // Risk: fixture bypasses temp isolation or migration reporting. Level:
    // particular-integration. Source: proposals/0b-03-wu-0b-03.md test intent
    // "Empty fixture creation".
    let seed_plan: GraphStoreSeedPlan = read_fixture("empty-seed-plan.json");

    let fixture = create_graphstore_fixture(seed_plan)
        .await
        .expect("empty seed plan should create a fixture");

    assert_eq!(
        fixture.pool.migrations_applied,
        vec![1, 4, 5, 6, 7, 9, 15, 16, 17]
    );
    assert!(fixture.pool.sqlite_url.starts_with("sqlite://"));
    assert!(fixture.pool.workspace_root.ends_with("storage"));
    assert!(fixture.workspace_id.is_none());
    assert_eq!(fixture.graph_version, None);
    assert!(fixture.created_refs.is_empty());
    assert_eq!(
        sqlite_table_names(&fixture.pool.sqlite).await,
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
    assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
}

#[tokio::test]
async fn future_wu_seed_plan_returns_unknown_ref_before_database_creation() {
    // Risk: tests seed objects before their owning WU exists. Level: component.
    // Source: proposals/0b-03-wu-0b-03.md test intent "Future-WU seed rejection".
    let fixture: UnknownRefSeedFixture = read_fixture("unknown-ref-seed-plan.json");
    let seed_plan = GraphStoreSeedPlan {
        future_refs: fixture.future_refs,
    };

    let error = create_graphstore_fixture(seed_plan)
        .await
        .expect_err("future-WU refs must be rejected");

    assert_eq!(error, fixture.expected_error);
    assert_eq!(error.code(), fixture.expected_code);

    let source = graphstore_fixture_source();
    let validate_index = source
        .find("validate_seed_plan(&seed_plan)?")
        .expect("source should validate seed plan explicitly");
    let temp_harness_index = source
        .find("temp_harness_state(")
        .expect("source should call temp_harness_state for valid plans");
    assert!(
        validate_index < temp_harness_index,
        "seed validation must happen before temp harness creation"
    );
}

#[tokio::test]
async fn reset_preserves_schema_versions_history_exactly() {
    // Risk: reset destroys migration history. Level: particular-integration.
    // Source: proposals/0b-03-wu-0b-03.md test intent "Reset preservation".
    let seed_plan: GraphStoreSeedPlan = read_fixture("empty-seed-plan.json");
    let fixture = create_graphstore_fixture(seed_plan)
        .await
        .expect("empty seed plan should create a fixture");
    let before = migration_records(&fixture.pool.sqlite).await;

    reset_graphstore_fixture(&fixture)
        .await
        .expect("reset should succeed");

    let after = migration_records(&fixture.pool.sqlite).await;
    assert_eq!(after, before);

    let rerun = run_migrations(&fixture.pool.sqlite, migrations_dir())
        .await
        .expect("migrations should rerun idempotently after reset");
    assert_eq!(rerun.applied_versions, Vec::<i64>::new());
    assert_eq!(rerun.skipped_versions, vec![1, 4, 5, 6, 7, 9, 15, 16, 17]);
    assert_eq!(migration_records(&fixture.pool.sqlite).await, before);
}

#[tokio::test]
async fn fixture_created_refs_are_validated_through_opaque_id_hook() {
    // Risk: future seeded refs bypass WU-0B-02 invariants. Level:
    // unit/component. Source: proposals/0b-03-wu-0b-03.md test intent
    // "Opaque-ID hook".
    let seed_plan: GraphStoreSeedPlan = read_fixture("empty-seed-plan.json");
    let fixture = create_graphstore_fixture(seed_plan)
        .await
        .expect("empty seed plan should create a fixture");

    validate_fixture_created_refs(&fixture).expect("empty created ref set is valid");
}

#[tokio::test]
async fn fixture_builder_has_no_subprocess_provider_ui_optimizer_or_recovery_side_effects() {
    // Risk: fixture setup leaks subprocess, provider, UI, optimizer, recovery,
    // command, or migration side effects. Level: component structural. Source:
    // proposals/0b-03-wu-0b-03.md test intent "Side-effect absence".
    let fixture: NoSideEffectsFixture = read_fixture("no-side-effects-assertions.json");
    let source = graphstore_fixture_source();

    for token in fixture.forbidden_source_tokens {
        assert!(
            !source.contains(&token),
            "graphstore fixture source must not contain forbidden token {token:?}"
        );
    }

    let mut migration_files = fs::read_dir(migrations_dir())
        .expect("migrations directory should be readable")
        .map(|entry| {
            entry
                .expect("migration directory entry should be readable")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    migration_files.sort();
    assert_eq!(migration_files, fixture.allowed_migration_files);

    let seed_plan: GraphStoreSeedPlan = read_fixture("empty-seed-plan.json");
    let graphstore_fixture = create_graphstore_fixture(seed_plan)
        .await
        .expect("empty seed plan should create a fixture");
    assert_eq!(
        sqlite_table_names(&graphstore_fixture.pool.sqlite).await,
        fixture.allowed_runtime_tables
    );
    assert_eq!(
        phase_0a_scaffold_commands(),
        fixture
            .phase_0a_scaffold_commands
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn fixtures_created_in_same_test_have_independent_temp_paths() {
    // Risk: downstream tests share global database state. Level:
    // particular-integration. Source: proposals/0b-03-wu-0b-03.md test intent
    // "Independent fixtures".
    let first = create_graphstore_fixture(read_fixture("empty-seed-plan.json"))
        .await
        .expect("first fixture should create");
    let second = create_graphstore_fixture(read_fixture("empty-seed-plan.json"))
        .await
        .expect("second fixture should create");

    assert_ne!(first.pool.sqlite_url, second.pool.sqlite_url);
    assert_ne!(first.pool.workspace_root, second.pool.workspace_root);
    assert_eq!(migration_records(&first.pool.sqlite).await.len(), 9);
    assert_eq!(migration_records(&second.pool.sqlite).await.len(), 9);
}
