use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::{Database, Sqlite, SqlitePool};

use crate::graphstore::auditevent::ProviderStateRef;
use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
use crate::graphstore::graphworkspace::GraphWorkspace;
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetScopeRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetScopeType {
    Workspace,
    Initiative,
    OrchestratorTurn,
    WorkerRun,
    OptimizerPass,
    ReviewerPass,
    Render,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetState {
    Within,
    NearLimit,
    Exceeded,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    None,
    Warn,
    NarrowScope,
    RequireUserApproval,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetLedger {
    pub budget_ledger_id: OpaqueId<BudgetLedger>,
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub scope_type: BudgetScopeType,
    pub scope_id: OpaqueId<BudgetScopeRef>,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cache_read_tokens: i64,
    pub cache_write_tokens: i64,
    pub latency_ms: i64,
    pub provider_cost_estimate: i64,
    pub provider_state_id: Option<OpaqueId<ProviderStateRef>>,
    pub cache_prefix_hash: Option<String>,
    pub budget_state: BudgetState,
    pub policy_action: PolicyAction,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct BudgetLedgerRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct BudgetLedgerRow {
    budget_ledger_id_value: String,
    budget_ledger_id_namespace: String,
    workspace_id_value: String,
    workspace_id_namespace: String,
    scope_type: BudgetScopeType,
    scope_id_value: String,
    scope_id_namespace: String,
    input_tokens: i64,
    output_tokens: i64,
    cache_read_tokens: i64,
    cache_write_tokens: i64,
    latency_ms: i64,
    provider_cost_estimate: i64,
    provider_state_id_value: Option<String>,
    provider_state_id_namespace: Option<String>,
    cache_prefix_hash: Option<String>,
    budget_state: BudgetState,
    policy_action: PolicyAction,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl BudgetLedgerRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_budget_ledger(
        &self,
        record: BudgetLedger,
    ) -> Result<BudgetLedger, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_budget_ledger(record) {
            Ok(record) => record,
            Err(error) => {
                rollback(transaction).await?;
                return Err(error);
            }
        };

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM budget_ledgers
             WHERE budget_ledger_id_value = ? AND budget_ledger_id_namespace = ?",
        )
        .bind(&record.budget_ledger_id.value)
        .bind(&record.budget_ledger_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::DuplicateId);
        }

        let provider_state_id_value = record
            .provider_state_id
            .as_ref()
            .map(|id| id.value.as_str());
        let provider_state_id_namespace = record
            .provider_state_id
            .as_ref()
            .map(|id| id.namespace.as_str());

        let insert_result = sqlx::query(
            "INSERT INTO budget_ledgers
             (budget_ledger_id_value, budget_ledger_id_namespace,
              workspace_id_value, workspace_id_namespace,
              scope_type, scope_id_value, scope_id_namespace,
              input_tokens, output_tokens, cache_read_tokens,
              cache_write_tokens, latency_ms, provider_cost_estimate,
              provider_state_id_value, provider_state_id_namespace,
              cache_prefix_hash, budget_state, policy_action,
              created_at, updated_at, actor, record_policy_version,
              schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 16)",
        )
        .bind(&record.budget_ledger_id.value)
        .bind(&record.budget_ledger_id.namespace)
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(record.scope_type)
        .bind(&record.scope_id.value)
        .bind(&record.scope_id.namespace)
        .bind(record.input_tokens)
        .bind(record.output_tokens)
        .bind(record.cache_read_tokens)
        .bind(record.cache_write_tokens)
        .bind(record.latency_ms)
        .bind(record.provider_cost_estimate)
        .bind(provider_state_id_value)
        .bind(provider_state_id_namespace)
        .bind(&record.cache_prefix_hash)
        .bind(record.budget_state)
        .bind(record.policy_action)
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

    pub async fn get_budget_ledger(
        &self,
        id: &OpaqueId<BudgetLedger>,
    ) -> Result<Option<BudgetLedger>, GraphStoreError> {
        OpaqueId::<BudgetLedger>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, BudgetLedgerRow>(SELECT_BUDGET_LEDGER)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(BudgetLedger::try_from).transpose()
    }

    pub async fn list_budget_ledgers_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Vec<BudgetLedger>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, BudgetLedgerRow>(
            "SELECT budget_ledger_id_value, budget_ledger_id_namespace,
                    workspace_id_value, workspace_id_namespace,
                    scope_type, scope_id_value, scope_id_namespace,
                    input_tokens, output_tokens, cache_read_tokens,
                    cache_write_tokens, latency_ms, provider_cost_estimate,
                    provider_state_id_value, provider_state_id_namespace,
                    cache_prefix_hash, budget_state, policy_action,
                    created_at, updated_at, actor, record_policy_version
             FROM budget_ledgers
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(BudgetLedger::try_from).collect()
    }
}

impl GraphStoreRepo<BudgetLedger> for BudgetLedgerRepo {
    type IdMarker = BudgetLedger;

    fn insert<'a>(&'a self, record: BudgetLedger) -> GraphStoreRepoFuture<'a, BudgetLedger> {
        Box::pin(async move { self.insert_budget_ledger(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<BudgetLedger>> {
        Box::pin(async move { self.get_budget_ledger(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<BudgetLedger>> {
        Box::pin(async move {
            let workspace_id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            self.list_budget_ledgers_by_workspace(&workspace_id).await
        })
    }
}

impl TryFrom<BudgetLedgerRow> for BudgetLedger {
    type Error = GraphStoreError;

    fn try_from(row: BudgetLedgerRow) -> Result<Self, Self::Error> {
        let provider_state_id = optional_opaque_id::<ProviderStateRef>(
            row.provider_state_id_value,
            row.provider_state_id_namespace,
        )?;

        let record = BudgetLedger {
            budget_ledger_id: OpaqueId::<BudgetLedger>::new(
                row.budget_ledger_id_value,
                row.budget_ledger_id_namespace,
            )?,
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            scope_type: row.scope_type,
            scope_id: OpaqueId::<BudgetScopeRef>::new(row.scope_id_value, row.scope_id_namespace)?,
            input_tokens: row.input_tokens,
            output_tokens: row.output_tokens,
            cache_read_tokens: row.cache_read_tokens,
            cache_write_tokens: row.cache_write_tokens,
            latency_ms: row.latency_ms,
            provider_cost_estimate: row.provider_cost_estimate,
            provider_state_id,
            cache_prefix_hash: row.cache_prefix_hash,
            budget_state: row.budget_state,
            policy_action: row.policy_action,
            meta: validate_record_meta(RecordMeta {
                created_at: row.created_at,
                updated_at: row.updated_at,
                actor: row.actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_budget_ledger(record)
    }
}

const SELECT_BUDGET_LEDGER: &str = "SELECT budget_ledger_id_value, budget_ledger_id_namespace,
        workspace_id_value, workspace_id_namespace,
        scope_type, scope_id_value, scope_id_namespace,
        input_tokens, output_tokens, cache_read_tokens,
        cache_write_tokens, latency_ms, provider_cost_estimate,
        provider_state_id_value, provider_state_id_namespace,
        cache_prefix_hash, budget_state, policy_action,
        created_at, updated_at, actor, record_policy_version
 FROM budget_ledgers
 WHERE budget_ledger_id_value = ? AND budget_ledger_id_namespace = ?";

fn validate_budget_ledger(record: BudgetLedger) -> Result<BudgetLedger, GraphStoreError> {
    OpaqueId::<BudgetLedger>::new(
        record.budget_ledger_id.value.clone(),
        record.budget_ledger_id.namespace.clone(),
    )?;
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    OpaqueId::<BudgetScopeRef>::new(
        record.scope_id.value.clone(),
        record.scope_id.namespace.clone(),
    )?;
    if let Some(provider_state_id) = &record.provider_state_id {
        OpaqueId::<ProviderStateRef>::new(
            provider_state_id.value.clone(),
            provider_state_id.namespace.clone(),
        )?;
    }
    validate_record_meta(record.meta.clone())?;

    if record.input_tokens < 0
        || record.output_tokens < 0
        || record.cache_read_tokens < 0
        || record.cache_write_tokens < 0
        || record.latency_ms < 0
        || record.provider_cost_estimate < 0
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    if !policy_action_allowed(record.budget_state, record.policy_action) {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(record)
}

fn policy_action_allowed(budget_state: BudgetState, policy_action: PolicyAction) -> bool {
    match budget_state {
        BudgetState::Within => matches!(policy_action, PolicyAction::None | PolicyAction::Warn),
        BudgetState::NearLimit => {
            matches!(
                policy_action,
                PolicyAction::Warn | PolicyAction::NarrowScope
            )
        }
        BudgetState::Exceeded => matches!(
            policy_action,
            PolicyAction::RequireUserApproval | PolicyAction::Block | PolicyAction::NarrowScope
        ),
        BudgetState::Blocked => policy_action == PolicyAction::Block,
    }
}

fn optional_opaque_id<T>(
    value: Option<String>,
    namespace: Option<String>,
) -> Result<Option<OpaqueId<T>>, GraphStoreError> {
    match (value, namespace) {
        (Some(value), Some(namespace)) => OpaqueId::<T>::new(value, namespace).map(Some),
        (None, None) => Ok(None),
        _ => Err(GraphStoreError::InvariantViolation),
    }
}

async fn rollback(transaction: sqlx::Transaction<'_, Sqlite>) -> Result<(), GraphStoreError> {
    transaction.rollback().await.map_err(GraphStoreError::from)
}

fn map_write_error(error: &sqlx::Error) -> GraphStoreError {
    match error {
        sqlx::Error::Database(database_error) => {
            let message = database_error.message();
            if message.contains("UNIQUE constraint failed: budget_ledgers.budget_ledger_id_value") {
                GraphStoreError::DuplicateId
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed: scope_type")
                || message.contains("CHECK constraint failed: budget_state")
                || message.contains("CHECK constraint failed: policy_action")
            {
                GraphStoreError::InvalidEnum
            } else if message.contains("CHECK constraint failed") {
                GraphStoreError::InvariantViolation
            } else {
                GraphStoreError::SqlxFailure
            }
        }
        _ => GraphStoreError::SqlxFailure,
    }
}

impl BudgetScopeType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Initiative => "initiative",
            Self::OrchestratorTurn => "orchestrator_turn",
            Self::WorkerRun => "worker_run",
            Self::OptimizerPass => "optimizer_pass",
            Self::ReviewerPass => "reviewer_pass",
            Self::Render => "render",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "workspace" => Ok(Self::Workspace),
            "initiative" => Ok(Self::Initiative),
            "orchestrator_turn" => Ok(Self::OrchestratorTurn),
            "worker_run" => Ok(Self::WorkerRun),
            "optimizer_pass" => Ok(Self::OptimizerPass),
            "reviewer_pass" => Ok(Self::ReviewerPass),
            "render" => Ok(Self::Render),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl BudgetState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Within => "within",
            Self::NearLimit => "near_limit",
            Self::Exceeded => "exceeded",
            Self::Blocked => "blocked",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "within" => Ok(Self::Within),
            "near_limit" => Ok(Self::NearLimit),
            "exceeded" => Ok(Self::Exceeded),
            "blocked" => Ok(Self::Blocked),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl PolicyAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Warn => "warn",
            Self::NarrowScope => "narrow_scope",
            Self::RequireUserApproval => "require_user_approval",
            Self::Block => "block",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "none" => Ok(Self::None),
            "warn" => Ok(Self::Warn),
            "narrow_scope" => Ok(Self::NarrowScope),
            "require_user_approval" => Ok(Self::RequireUserApproval),
            "block" => Ok(Self::Block),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

macro_rules! impl_sqlite_text_enum {
    ($type_name:ty) => {
        impl Type<Sqlite> for $type_name {
            fn type_info() -> <Sqlite as Database>::TypeInfo {
                <String as Type<Sqlite>>::type_info()
            }

            fn compatible(ty: &<Sqlite as Database>::TypeInfo) -> bool {
                <String as Type<Sqlite>>::compatible(ty)
            }
        }

        impl<'q> Encode<'q, Sqlite> for $type_name {
            fn encode_by_ref(
                &self,
                buf: &mut <Sqlite as Database>::ArgumentBuffer<'q>,
            ) -> Result<IsNull, BoxDynError> {
                let encoded = self.as_str().to_string();
                <String as Encode<Sqlite>>::encode(encoded, buf)
            }
        }

        impl<'r> Decode<'r, Sqlite> for $type_name {
            fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
                let encoded = <String as Decode<Sqlite>>::decode(value)?;
                Self::parse(&encoded).map_err(|error| error.into())
            }
        }
    };
}

impl_sqlite_text_enum!(BudgetScopeType);
impl_sqlite_text_enum!(BudgetState);
impl_sqlite_text_enum!(PolicyAction);
