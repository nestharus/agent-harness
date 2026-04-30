use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;

use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

const SUMMARY_CONTRACT_POLICY: &str = "summary_contract";
const RENDER_POLICY: &str = "render";
const IDENTITY_POLICY: &str = "identity";
const PRIVILEGE_POLICY: &str = "privilege";
const TOOL_PROTOCOL_POLICY: &str = "tool_protocol";
const BUDGET_POLICY: &str = "budget";
const REVIEW_SAMPLING_POLICY: &str = "review_sampling";
const RECOVERY_POLICY: &str = "recovery";
const CONFIGURATION_POLICY: &str = "configuration";
const PROVIDER_POLICY: &str = "provider";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyVersionRef {
    pub policy: String,
    pub version: String,
    pub previous_version: Option<String>,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySet {
    pub policy_set_id: OpaqueId<PolicySet>,
    pub summary_contract_version: JsonField<PolicyVersionRef>,
    pub render_policy_version: JsonField<PolicyVersionRef>,
    pub identity_policy_version: JsonField<PolicyVersionRef>,
    pub privilege_policy_version: JsonField<PolicyVersionRef>,
    pub tool_protocol_policy_version: JsonField<PolicyVersionRef>,
    pub budget_policy_version: JsonField<PolicyVersionRef>,
    pub review_sampling_policy_version: JsonField<PolicyVersionRef>,
    pub recovery_policy_version: JsonField<PolicyVersionRef>,
    pub configuration_policy_version: JsonField<PolicyVersionRef>,
    pub provider_policy_version: JsonField<PolicyVersionRef>,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct PolicySetRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct PolicySetRow {
    policy_set_id_value: String,
    policy_set_id_namespace: String,
    summary_contract_version: JsonField<PolicyVersionRef>,
    render_policy_version: JsonField<PolicyVersionRef>,
    identity_policy_version: JsonField<PolicyVersionRef>,
    privilege_policy_version: JsonField<PolicyVersionRef>,
    tool_protocol_policy_version: JsonField<PolicyVersionRef>,
    budget_policy_version: JsonField<PolicyVersionRef>,
    review_sampling_policy_version: JsonField<PolicyVersionRef>,
    recovery_policy_version: JsonField<PolicyVersionRef>,
    configuration_policy_version: JsonField<PolicyVersionRef>,
    provider_policy_version: JsonField<PolicyVersionRef>,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl PolicySetRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_policy_set(&self, record: PolicySet) -> Result<PolicySet, GraphStoreError> {
        let mut transaction = self.pool.begin().await?;

        let record = match validate_policy_set(record) {
            Ok(record) => record,
            Err(error) => {
                transaction
                    .rollback()
                    .await
                    .map_err(GraphStoreError::from)?;
                return Err(error);
            }
        };

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM policy_sets
             WHERE policy_set_id_value = ? AND policy_set_id_namespace = ?",
        )
        .bind(&record.policy_set_id.value)
        .bind(&record.policy_set_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            transaction
                .rollback()
                .await
                .map_err(GraphStoreError::from)?;
            return Err(GraphStoreError::DuplicateId);
        }

        let insert_result = sqlx::query(
            "INSERT INTO policy_sets
             (policy_set_id_value, policy_set_id_namespace,
              summary_contract_version, render_policy_version, identity_policy_version,
              privilege_policy_version, tool_protocol_policy_version, budget_policy_version,
              review_sampling_policy_version, recovery_policy_version,
              configuration_policy_version, provider_policy_version,
              created_at, updated_at, actor, record_policy_version, schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 4)",
        )
        .bind(&record.policy_set_id.value)
        .bind(&record.policy_set_id.namespace)
        .bind(&record.summary_contract_version)
        .bind(&record.render_policy_version)
        .bind(&record.identity_policy_version)
        .bind(&record.privilege_policy_version)
        .bind(&record.tool_protocol_policy_version)
        .bind(&record.budget_policy_version)
        .bind(&record.review_sampling_policy_version)
        .bind(&record.recovery_policy_version)
        .bind(&record.configuration_policy_version)
        .bind(&record.provider_policy_version)
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
                transaction
                    .rollback()
                    .await
                    .map_err(GraphStoreError::from)?;
                if is_sqlite_unique_violation(&error) {
                    Err(GraphStoreError::DuplicateId)
                } else {
                    Err(GraphStoreError::SqlxFailure)
                }
            }
        }
    }

    pub async fn get_policy_set(
        &self,
        id: &OpaqueId<PolicySet>,
    ) -> Result<Option<PolicySet>, GraphStoreError> {
        OpaqueId::<PolicySet>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, PolicySetRow>(
            "SELECT policy_set_id_value, policy_set_id_namespace,
                    summary_contract_version, render_policy_version, identity_policy_version,
                    privilege_policy_version, tool_protocol_policy_version, budget_policy_version,
                    review_sampling_policy_version, recovery_policy_version,
                    configuration_policy_version, provider_policy_version,
                    created_at, updated_at, actor, record_policy_version
             FROM policy_sets
             WHERE policy_set_id_value = ? AND policy_set_id_namespace = ?",
        )
        .bind(&id.value)
        .bind(&id.namespace)
        .fetch_optional(&self.pool)
        .await?;

        row.map(PolicySet::try_from).transpose()
    }

    pub async fn list_policy_sets_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspaceRef>,
    ) -> Result<Vec<PolicySet>, GraphStoreError> {
        OpaqueId::<GraphWorkspaceRef>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, PolicySetRow>(
            "SELECT policy_set_id_value, policy_set_id_namespace,
                    summary_contract_version, render_policy_version, identity_policy_version,
                    privilege_policy_version, tool_protocol_policy_version, budget_policy_version,
                    review_sampling_policy_version, recovery_policy_version,
                    configuration_policy_version, provider_policy_version,
                    created_at, updated_at, actor, record_policy_version
             FROM policy_sets
             WHERE policy_set_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(&workspace_id.value)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(PolicySet::try_from).collect()
    }
}

impl GraphStoreRepo<PolicySet> for PolicySetRepo {
    type IdMarker = PolicySet;

    fn insert<'a>(&'a self, record: PolicySet) -> GraphStoreRepoFuture<'a, PolicySet> {
        Box::pin(async move { self.insert_policy_set(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<PolicySet>> {
        Box::pin(async move { self.get_policy_set(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<PolicySet>> {
        Box::pin(async move { self.list_policy_sets_by_workspace(workspace_id).await })
    }
}

impl TryFrom<PolicySetRow> for PolicySet {
    type Error = GraphStoreError;

    fn try_from(row: PolicySetRow) -> Result<Self, Self::Error> {
        let policy_set_id =
            OpaqueId::<PolicySet>::new(row.policy_set_id_value, row.policy_set_id_namespace)?;
        let meta = validate_record_meta(RecordMeta {
            created_at: row.created_at,
            updated_at: row.updated_at,
            actor: row.actor.0,
            policy_version: row.record_policy_version,
        })?;

        let record = PolicySet {
            policy_set_id,
            summary_contract_version: row.summary_contract_version,
            render_policy_version: row.render_policy_version,
            identity_policy_version: row.identity_policy_version,
            privilege_policy_version: row.privilege_policy_version,
            tool_protocol_policy_version: row.tool_protocol_policy_version,
            budget_policy_version: row.budget_policy_version,
            review_sampling_policy_version: row.review_sampling_policy_version,
            recovery_policy_version: row.recovery_policy_version,
            configuration_policy_version: row.configuration_policy_version,
            provider_policy_version: row.provider_policy_version,
            meta,
        };

        validate_policy_set(record)
    }
}

fn validate_policy_set(record: PolicySet) -> Result<PolicySet, GraphStoreError> {
    OpaqueId::<PolicySet>::new(
        record.policy_set_id.value.clone(),
        record.policy_set_id.namespace.clone(),
    )?;
    validate_record_meta(record.meta.clone())?;
    validate_policy_version(&record.summary_contract_version.0, SUMMARY_CONTRACT_POLICY)?;
    validate_policy_version(&record.render_policy_version.0, RENDER_POLICY)?;
    validate_policy_version(&record.identity_policy_version.0, IDENTITY_POLICY)?;
    validate_policy_version(&record.privilege_policy_version.0, PRIVILEGE_POLICY)?;
    validate_policy_version(&record.tool_protocol_policy_version.0, TOOL_PROTOCOL_POLICY)?;
    validate_policy_version(&record.budget_policy_version.0, BUDGET_POLICY)?;
    validate_policy_version(
        &record.review_sampling_policy_version.0,
        REVIEW_SAMPLING_POLICY,
    )?;
    validate_policy_version(&record.recovery_policy_version.0, RECOVERY_POLICY)?;
    validate_policy_version(&record.configuration_policy_version.0, CONFIGURATION_POLICY)?;
    validate_policy_version(&record.provider_policy_version.0, PROVIDER_POLICY)?;

    Ok(record)
}

fn validate_policy_version(
    value: &PolicyVersionRef,
    expected_policy: &str,
) -> Result<(), GraphStoreError> {
    if value.policy.trim().is_empty() || value.version.trim().is_empty() {
        return Err(GraphStoreError::InvariantViolation);
    }
    if value.policy != expected_policy {
        return Err(GraphStoreError::InvariantViolation);
    }

    let version = parse_policy_semver(&value.version)?;
    if let Some(previous_version) = &value.previous_version {
        if previous_version.trim().is_empty() {
            return Err(GraphStoreError::InvariantViolation);
        }
        let previous = parse_policy_semver(previous_version)?;
        if version <= previous {
            return Err(GraphStoreError::InvariantViolation);
        }
    }

    Ok(())
}

fn parse_policy_semver(value: &str) -> Result<(u64, u64, u64), GraphStoreError> {
    let version = value
        .strip_prefix('v')
        .ok_or(GraphStoreError::InvariantViolation)?;
    let mut parts = version.split('.');
    let major = parse_semver_part(parts.next())?;
    let minor = parse_semver_part(parts.next())?;
    let patch = parse_semver_part(parts.next())?;
    if parts.next().is_some() {
        return Err(GraphStoreError::InvariantViolation);
    }
    Ok((major, minor, patch))
}

fn parse_semver_part(value: Option<&str>) -> Result<u64, GraphStoreError> {
    let value = value.ok_or(GraphStoreError::InvariantViolation)?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(GraphStoreError::InvariantViolation);
    }
    value
        .parse::<u64>()
        .map_err(|_| GraphStoreError::InvariantViolation)
}

fn is_sqlite_unique_violation(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Database(database_error) => database_error
            .message()
            .contains("UNIQUE constraint failed: policy_sets.policy_set_id_value"),
        _ => false,
    }
}
