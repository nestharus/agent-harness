use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool};

use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
use crate::graphstore::graphconfiguration::GraphConfiguration;
use crate::graphstore::policyset::PolicySet;
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Orchestrator;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphWorkspace {
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub schema_version: i64,
    pub active_orchestrator_id: OpaqueId<Orchestrator>,
    pub current_graph_version: i64,
    pub storage_root: String,
    pub policy_set_id: OpaqueId<PolicySet>,
    pub active_configuration_id: OpaqueId<GraphConfiguration>,
    pub is_active_default: bool,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct GraphWorkspaceRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct GraphWorkspaceRow {
    workspace_id_value: String,
    workspace_id_namespace: String,
    schema_version: i64,
    active_orchestrator_id_value: String,
    active_orchestrator_id_namespace: String,
    current_graph_version: i64,
    storage_root: String,
    policy_set_id_value: String,
    policy_set_id_namespace: String,
    active_configuration_id_value: String,
    active_configuration_id_namespace: String,
    is_active_default: bool,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl GraphWorkspaceRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_graph_workspace(
        &self,
        record: GraphWorkspace,
    ) -> Result<GraphWorkspace, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_graph_workspace(record) {
            Ok(record) => record,
            Err(error) => {
                rollback(transaction).await?;
                return Err(error);
            }
        };

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM graph_workspaces
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?",
        )
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::DuplicateId);
        }

        if record.is_active_default {
            let active_defaults: (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM graph_workspaces WHERE is_active_default = 1")
                    .fetch_one(&mut *transaction)
                    .await?;
            if active_defaults.0 != 0 {
                rollback(transaction).await?;
                return Err(GraphStoreError::InvariantViolation);
            }
        }

        let insert_result = sqlx::query(
            "INSERT INTO graph_workspaces
             (workspace_id_value, workspace_id_namespace,
              schema_version,
              active_orchestrator_id_value, active_orchestrator_id_namespace,
              current_graph_version, storage_root,
              policy_set_id_value, policy_set_id_namespace,
              active_configuration_id_value, active_configuration_id_namespace,
              is_active_default,
              created_at, updated_at, actor, record_policy_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(record.schema_version)
        .bind(&record.active_orchestrator_id.value)
        .bind(&record.active_orchestrator_id.namespace)
        .bind(record.current_graph_version)
        .bind(&record.storage_root)
        .bind(&record.policy_set_id.value)
        .bind(&record.policy_set_id.namespace)
        .bind(&record.active_configuration_id.value)
        .bind(&record.active_configuration_id.namespace)
        .bind(record.is_active_default)
        .bind(&record.meta.created_at)
        .bind(&record.meta.updated_at)
        .bind(JsonField(record.meta.actor.clone()))
        .bind(&record.meta.policy_version)
        .execute(&mut *transaction)
        .await;

        match insert_result {
            Ok(_) => {
                transaction.commit().await.map_err(GraphStoreError::from)?;
                Ok(record)
            }
            Err(error) => {
                rollback(transaction).await?;
                Err(map_write_error(&error))
            }
        }
    }

    pub async fn get_graph_workspace(
        &self,
        id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Option<GraphWorkspace>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, GraphWorkspaceRow>(SELECT_GRAPH_WORKSPACE)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(GraphWorkspace::try_from).transpose()
    }

    pub async fn list_graph_workspaces_by_namespace(
        &self,
        namespace: &str,
    ) -> Result<Vec<GraphWorkspace>, GraphStoreError> {
        if namespace.trim().is_empty() {
            return Err(GraphStoreError::InvariantViolation);
        }

        let rows = sqlx::query_as::<_, GraphWorkspaceRow>(
            "SELECT workspace_id_value, workspace_id_namespace,
                    schema_version,
                    active_orchestrator_id_value, active_orchestrator_id_namespace,
                    current_graph_version, storage_root,
                    policy_set_id_value, policy_set_id_namespace,
                    active_configuration_id_value, active_configuration_id_namespace,
                    is_active_default,
                    created_at, updated_at, actor, record_policy_version
             FROM graph_workspaces
             WHERE workspace_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(GraphWorkspace::try_from).collect()
    }

    pub async fn set_active_configuration(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
        configuration_id: &OpaqueId<GraphConfiguration>,
    ) -> Result<GraphWorkspace, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;
        OpaqueId::<GraphConfiguration>::new(
            configuration_id.value.clone(),
            configuration_id.namespace.clone(),
        )?;

        let mut transaction = self.pool.begin().await?;
        let current = match select_workspace(&mut transaction, workspace_id).await? {
            Some(current) => current,
            None => {
                rollback(transaction).await?;
                return Err(GraphStoreError::UnknownRef);
            }
        };

        let configuration_exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM graph_configurations
             WHERE configuration_id_value = ? AND configuration_id_namespace = ?",
        )
        .bind(&configuration_id.value)
        .bind(&configuration_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if configuration_exists.0 == 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::UnknownRef);
        }

        let updated_at = next_contract_timestamp(&current.meta.updated_at)?;
        let update_result = sqlx::query(
            "UPDATE graph_workspaces
             SET active_configuration_id_value = ?,
                 active_configuration_id_namespace = ?,
                 updated_at = ?
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?",
        )
        .bind(&configuration_id.value)
        .bind(&configuration_id.namespace)
        .bind(&updated_at)
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .execute(&mut *transaction)
        .await;

        let update = match update_result {
            Ok(update) => update,
            Err(error) => {
                rollback(transaction).await?;
                return Err(map_write_error(&error));
            }
        };
        if update.rows_affected() != 1 {
            rollback(transaction).await?;
            return Err(GraphStoreError::UnknownRef);
        }

        let updated = select_workspace(&mut transaction, workspace_id)
            .await?
            .ok_or(GraphStoreError::UnknownRef)?;
        transaction.commit().await.map_err(GraphStoreError::from)?;
        Ok(updated)
    }

    pub async fn advance_graph_version(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
        expected_version: i64,
    ) -> Result<GraphWorkspace, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;
        if expected_version < 0 {
            return Err(GraphStoreError::InvalidTransition);
        }

        let mut transaction = self.pool.begin().await?;
        let current = match select_workspace(&mut transaction, workspace_id).await? {
            Some(current) => current,
            None => {
                rollback(transaction).await?;
                return Err(GraphStoreError::UnknownRef);
            }
        };
        if current.current_graph_version != expected_version {
            rollback(transaction).await?;
            return Err(GraphStoreError::OptimisticConflict);
        }

        let updated_at = next_contract_timestamp(&current.meta.updated_at)?;
        let next_version = current.current_graph_version + 1;
        let update_result = sqlx::query(
            "UPDATE graph_workspaces
             SET current_graph_version = ?,
                 updated_at = ?
             WHERE workspace_id_value = ?
               AND workspace_id_namespace = ?
               AND current_graph_version = ?",
        )
        .bind(next_version)
        .bind(&updated_at)
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .bind(expected_version)
        .execute(&mut *transaction)
        .await;

        let update = match update_result {
            Ok(update) => update,
            Err(error) => {
                rollback(transaction).await?;
                return Err(map_write_error(&error));
            }
        };
        if update.rows_affected() != 1 {
            rollback(transaction).await?;
            return Err(GraphStoreError::OptimisticConflict);
        }

        let updated = select_workspace(&mut transaction, workspace_id)
            .await?
            .ok_or(GraphStoreError::UnknownRef)?;
        transaction.commit().await.map_err(GraphStoreError::from)?;
        Ok(updated)
    }
}

impl GraphStoreRepo<GraphWorkspace> for GraphWorkspaceRepo {
    type IdMarker = GraphWorkspace;

    fn insert<'a>(&'a self, record: GraphWorkspace) -> GraphStoreRepoFuture<'a, GraphWorkspace> {
        Box::pin(async move { self.insert_graph_workspace(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<GraphWorkspace>> {
        Box::pin(async move { self.get_graph_workspace(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<GraphWorkspace>> {
        Box::pin(async move {
            let id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            match self.get_graph_workspace(&id).await? {
                Some(workspace) => Ok(vec![workspace]),
                None => Ok(Vec::new()),
            }
        })
    }
}

impl TryFrom<GraphWorkspaceRow> for GraphWorkspace {
    type Error = GraphStoreError;

    fn try_from(row: GraphWorkspaceRow) -> Result<Self, Self::Error> {
        let record = GraphWorkspace {
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            schema_version: row.schema_version,
            active_orchestrator_id: OpaqueId::<Orchestrator>::new(
                row.active_orchestrator_id_value,
                row.active_orchestrator_id_namespace,
            )?,
            current_graph_version: row.current_graph_version,
            storage_root: row.storage_root,
            policy_set_id: OpaqueId::<PolicySet>::new(
                row.policy_set_id_value,
                row.policy_set_id_namespace,
            )?,
            active_configuration_id: OpaqueId::<GraphConfiguration>::new(
                row.active_configuration_id_value,
                row.active_configuration_id_namespace,
            )?,
            is_active_default: row.is_active_default,
            meta: validate_record_meta(RecordMeta {
                created_at: row.created_at,
                updated_at: row.updated_at,
                actor: row.actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_graph_workspace(record)
    }
}

const SELECT_GRAPH_WORKSPACE: &str = "SELECT workspace_id_value, workspace_id_namespace,
        schema_version,
        active_orchestrator_id_value, active_orchestrator_id_namespace,
        current_graph_version, storage_root,
        policy_set_id_value, policy_set_id_namespace,
        active_configuration_id_value, active_configuration_id_namespace,
        is_active_default,
        created_at, updated_at, actor, record_policy_version
 FROM graph_workspaces
 WHERE workspace_id_value = ? AND workspace_id_namespace = ?";

async fn select_workspace(
    transaction: &mut sqlx::Transaction<'_, Sqlite>,
    id: &OpaqueId<GraphWorkspace>,
) -> Result<Option<GraphWorkspace>, GraphStoreError> {
    let row = sqlx::query_as::<_, GraphWorkspaceRow>(SELECT_GRAPH_WORKSPACE)
        .bind(&id.value)
        .bind(&id.namespace)
        .fetch_optional(&mut **transaction)
        .await?;

    row.map(GraphWorkspace::try_from).transpose()
}

fn validate_graph_workspace(record: GraphWorkspace) -> Result<GraphWorkspace, GraphStoreError> {
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    OpaqueId::<Orchestrator>::new(
        record.active_orchestrator_id.value.clone(),
        record.active_orchestrator_id.namespace.clone(),
    )?;
    OpaqueId::<PolicySet>::new(
        record.policy_set_id.value.clone(),
        record.policy_set_id.namespace.clone(),
    )?;
    OpaqueId::<GraphConfiguration>::new(
        record.active_configuration_id.value.clone(),
        record.active_configuration_id.namespace.clone(),
    )?;
    validate_record_meta(record.meta.clone())?;

    if record.schema_version <= 0
        || record.current_graph_version < 0
        || record.storage_root.trim().is_empty()
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(record)
}

async fn rollback(transaction: sqlx::Transaction<'_, Sqlite>) -> Result<(), GraphStoreError> {
    transaction.rollback().await.map_err(GraphStoreError::from)
}

fn map_write_error(error: &sqlx::Error) -> GraphStoreError {
    match error {
        sqlx::Error::Database(database_error) => {
            let message = database_error.message();
            if message.contains("UNIQUE constraint failed: graph_workspaces.workspace_id_value") {
                GraphStoreError::DuplicateId
            } else if message.contains("idx_graph_workspaces_one_active_default")
                || message.contains("UNIQUE constraint failed: graph_workspaces.is_active_default")
            {
                GraphStoreError::InvariantViolation
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed") {
                GraphStoreError::InvariantViolation
            } else {
                GraphStoreError::SqlxFailure
            }
        }
        _ => GraphStoreError::SqlxFailure,
    }
}

fn next_contract_timestamp(value: &str) -> Result<String, GraphStoreError> {
    let mut year = parse_digits(value, 0, 4)? as u16;
    let mut month = parse_digits(value, 5, 7)? as u8;
    let mut day = parse_digits(value, 8, 10)? as u8;
    let mut hour = parse_digits(value, 11, 13)? as u8;
    let mut minute = parse_digits(value, 14, 16)? as u8;
    let mut second = parse_digits(value, 17, 19)? as u8;

    if value.len() != 20
        || value.as_bytes()[4] != b'-'
        || value.as_bytes()[7] != b'-'
        || value.as_bytes()[10] != b'T'
        || value.as_bytes()[13] != b':'
        || value.as_bytes()[16] != b':'
        || value.as_bytes()[19] != b'Z'
        || !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    second += 1;
    if second <= 59 {
        return Ok(format_timestamp(year, month, day, hour, minute, second));
    }
    second = 0;
    minute += 1;
    if minute <= 59 {
        return Ok(format_timestamp(year, month, day, hour, minute, second));
    }
    minute = 0;
    hour += 1;
    if hour <= 23 {
        return Ok(format_timestamp(year, month, day, hour, minute, second));
    }
    hour = 0;
    day += 1;
    if day <= days_in_month(year, month) {
        return Ok(format_timestamp(year, month, day, hour, minute, second));
    }
    day = 1;
    month += 1;
    if month <= 12 {
        return Ok(format_timestamp(year, month, day, hour, minute, second));
    }
    month = 1;
    year += 1;
    Ok(format_timestamp(year, month, day, hour, minute, second))
}

fn parse_digits(value: &str, start: usize, end: usize) -> Result<u32, GraphStoreError> {
    let slice = value
        .get(start..end)
        .ok_or(GraphStoreError::InvariantViolation)?;
    if slice.is_empty() || !slice.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(GraphStoreError::InvariantViolation);
    }
    slice
        .parse::<u32>()
        .map_err(|_| GraphStoreError::InvariantViolation)
}

fn format_timestamp(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> String {
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
