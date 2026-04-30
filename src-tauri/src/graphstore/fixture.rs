use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::contracts::graphstore_migrations::MigrationError;
use crate::graphstore::migrations::run_migrations;
use crate::graphstore::prelude::{GraphStoreError, OpaqueId};
use crate::test_harness::temp_harness::{harness_app_state, temp_harness_state};

pub type GraphStoreRepoFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, GraphStoreError>> + 'a>>;

#[derive(Debug, Clone)]
pub struct GraphWorkspaceRef;

#[derive(Debug, Clone)]
pub struct GraphStoreFixtureRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphStoreSeedRef {
    pub wu: String,
    pub ref_type: String,
    pub value: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GraphStoreSeedPlan {
    pub future_refs: Vec<GraphStoreSeedRef>,
}

#[derive(Debug, Clone)]
pub struct GraphStorePool {
    pub sqlite_url: String,
    pub sqlite: SqlitePool,
    pub migrations_applied: Vec<i64>,
    pub workspace_root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct GraphStoreFixture {
    pub pool: GraphStorePool,
    pub workspace_id: Option<OpaqueId<GraphWorkspaceRef>>,
    pub graph_version: Option<i64>,
    pub created_refs: Vec<OpaqueId<GraphStoreFixtureRef>>,
}

pub trait GraphStoreRepo<T> {
    type IdMarker;

    fn insert<'a>(&'a self, record: T) -> GraphStoreRepoFuture<'a, T>;

    fn get<'a>(&'a self, id: &'a OpaqueId<Self::IdMarker>) -> GraphStoreRepoFuture<'a, Option<T>>;

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<T>>;
}

pub async fn create_graphstore_fixture(
    seed_plan: GraphStoreSeedPlan,
) -> Result<GraphStoreFixture, GraphStoreError> {
    validate_seed_plan(&seed_plan)?;

    let handle = temp_harness_state("empty", None, None, Vec::new())
        .await
        .map_err(|_| GraphStoreError::SqlxFailure)?;
    let app_state = harness_app_state(&handle).ok_or(GraphStoreError::SqlxFailure)?;
    let migration_report = run_migrations(&app_state.db, migrations_dir())
        .await
        .map_err(map_migration_error)?;
    let workspace_root = PathBuf::from(app_state.storage_layout.storage_root);
    let sqlite_url = sqlite_url(&app_state.storage_layout.database_path);

    Ok(GraphStoreFixture {
        pool: GraphStorePool {
            sqlite_url,
            sqlite: app_state.db,
            migrations_applied: migration_report.applied_versions,
            workspace_root,
        },
        workspace_id: None,
        graph_version: None,
        created_refs: Vec::new(),
    })
}

pub async fn reset_graphstore_fixture(fixture: &GraphStoreFixture) -> Result<(), GraphStoreError> {
    let _: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'schema_versions'",
    )
    .fetch_one(&fixture.pool.sqlite)
    .await
    .map_err(|_| GraphStoreError::SqlxFailure)?;

    Ok(())
}

pub fn validate_fixture_created_refs(fixture: &GraphStoreFixture) -> Result<(), GraphStoreError> {
    for created_ref in &fixture.created_refs {
        OpaqueId::<GraphStoreFixtureRef>::new(
            created_ref.value.clone(),
            created_ref.namespace.clone(),
        )?;
    }

    Ok(())
}

fn validate_seed_plan(seed_plan: &GraphStoreSeedPlan) -> Result<(), GraphStoreError> {
    if seed_plan.future_refs.is_empty() {
        Ok(())
    } else {
        Err(GraphStoreError::UnknownRef)
    }
}

fn migrations_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

fn sqlite_url(database_path: &str) -> String {
    format!("sqlite://{database_path}")
}

fn map_migration_error(error: MigrationError) -> GraphStoreError {
    match error {
        MigrationError::OutOfOrderVersion
        | MigrationError::VersionChecksumMismatch
        | MigrationError::VersionAlreadyAppliedDifferently
        | MigrationError::RollbackUnsupported
        | MigrationError::SqlxFailure => GraphStoreError::SqlxFailure,
    }
}
