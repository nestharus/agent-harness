use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::{Database, Sqlite, SqlitePool};

use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
use crate::graphstore::graphconfiguration::GraphConfiguration;
use crate::graphstore::graphworkspace::GraphWorkspace;
use crate::graphstore::policyset::PolicySet;
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderStateRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditDecision {
    Accepted,
    Rejected,
    Deferred,
    Quarantined,
    UserRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditEvent {
    pub audit_event_id: OpaqueId<AuditEvent>,
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub event_type: String,
    pub actor: String,
    pub policy_set_id: OpaqueId<PolicySet>,
    pub configuration_id: Option<OpaqueId<GraphConfiguration>>,
    pub provider_state_id: Option<OpaqueId<ProviderStateRef>>,
    pub input_refs: JsonField<Vec<String>>,
    pub output_refs: JsonField<Vec<String>>,
    pub decision: AuditDecision,
    pub reason_code: String,
    pub created_at: String,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct AuditEventRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct AuditEventRow {
    audit_event_id_value: String,
    audit_event_id_namespace: String,
    workspace_id_value: String,
    workspace_id_namespace: String,
    event_type: String,
    actor: String,
    policy_set_id_value: String,
    policy_set_id_namespace: String,
    configuration_id_value: Option<String>,
    configuration_id_namespace: Option<String>,
    provider_state_id_value: Option<String>,
    provider_state_id_namespace: Option<String>,
    input_refs: JsonField<Vec<String>>,
    output_refs: JsonField<Vec<String>>,
    decision: AuditDecision,
    reason_code: String,
    created_at: String,
    record_created_at: String,
    record_updated_at: String,
    record_actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl AuditEventRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_audit_event(
        &self,
        record: AuditEvent,
    ) -> Result<AuditEvent, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_audit_event(record) {
            Ok(record) => record,
            Err(error) => {
                rollback(transaction).await?;
                return Err(error);
            }
        };

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM audit_events
             WHERE audit_event_id_value = ? AND audit_event_id_namespace = ?",
        )
        .bind(&record.audit_event_id.value)
        .bind(&record.audit_event_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::DuplicateId);
        }

        let configuration_id_value = record.configuration_id.as_ref().map(|id| id.value.as_str());
        let configuration_id_namespace = record
            .configuration_id
            .as_ref()
            .map(|id| id.namespace.as_str());
        let provider_state_id_value = record
            .provider_state_id
            .as_ref()
            .map(|id| id.value.as_str());
        let provider_state_id_namespace = record
            .provider_state_id
            .as_ref()
            .map(|id| id.namespace.as_str());

        let insert_result = sqlx::query(
            "INSERT INTO audit_events
             (audit_event_id_value, audit_event_id_namespace,
              workspace_id_value, workspace_id_namespace,
              event_type, actor,
              policy_set_id_value, policy_set_id_namespace,
              configuration_id_value, configuration_id_namespace,
              provider_state_id_value, provider_state_id_namespace,
              input_refs, output_refs, decision, reason_code, created_at,
              record_created_at, record_updated_at, record_actor,
              record_policy_version, schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 15)",
        )
        .bind(&record.audit_event_id.value)
        .bind(&record.audit_event_id.namespace)
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(&record.event_type)
        .bind(&record.actor)
        .bind(&record.policy_set_id.value)
        .bind(&record.policy_set_id.namespace)
        .bind(configuration_id_value)
        .bind(configuration_id_namespace)
        .bind(provider_state_id_value)
        .bind(provider_state_id_namespace)
        .bind(&record.input_refs)
        .bind(&record.output_refs)
        .bind(record.decision)
        .bind(&record.reason_code)
        .bind(&record.created_at)
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

    pub async fn get_audit_event(
        &self,
        id: &OpaqueId<AuditEvent>,
    ) -> Result<Option<AuditEvent>, GraphStoreError> {
        OpaqueId::<AuditEvent>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, AuditEventRow>(SELECT_AUDIT_EVENT)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(AuditEvent::try_from).transpose()
    }

    pub async fn list_audit_events_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Vec<AuditEvent>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, AuditEventRow>(
            "SELECT audit_event_id_value, audit_event_id_namespace,
                    workspace_id_value, workspace_id_namespace,
                    event_type, actor,
                    policy_set_id_value, policy_set_id_namespace,
                    configuration_id_value, configuration_id_namespace,
                    provider_state_id_value, provider_state_id_namespace,
                    input_refs, output_refs, decision, reason_code, created_at,
                    record_created_at, record_updated_at, record_actor,
                    record_policy_version
             FROM audit_events
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(AuditEvent::try_from).collect()
    }
}

impl GraphStoreRepo<AuditEvent> for AuditEventRepo {
    type IdMarker = AuditEvent;

    fn insert<'a>(&'a self, record: AuditEvent) -> GraphStoreRepoFuture<'a, AuditEvent> {
        Box::pin(async move { self.insert_audit_event(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<AuditEvent>> {
        Box::pin(async move { self.get_audit_event(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<AuditEvent>> {
        Box::pin(async move {
            let workspace_id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            self.list_audit_events_by_workspace(&workspace_id).await
        })
    }
}

impl TryFrom<AuditEventRow> for AuditEvent {
    type Error = GraphStoreError;

    fn try_from(row: AuditEventRow) -> Result<Self, Self::Error> {
        let configuration_id = optional_opaque_id::<GraphConfiguration>(
            row.configuration_id_value,
            row.configuration_id_namespace,
        )?;
        let provider_state_id = optional_opaque_id::<ProviderStateRef>(
            row.provider_state_id_value,
            row.provider_state_id_namespace,
        )?;

        let record = AuditEvent {
            audit_event_id: OpaqueId::<AuditEvent>::new(
                row.audit_event_id_value,
                row.audit_event_id_namespace,
            )?,
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            event_type: row.event_type,
            actor: row.actor,
            policy_set_id: OpaqueId::<PolicySet>::new(
                row.policy_set_id_value,
                row.policy_set_id_namespace,
            )?,
            configuration_id,
            provider_state_id,
            input_refs: row.input_refs,
            output_refs: row.output_refs,
            decision: row.decision,
            reason_code: row.reason_code,
            created_at: row.created_at,
            meta: validate_record_meta(RecordMeta {
                created_at: row.record_created_at,
                updated_at: row.record_updated_at,
                actor: row.record_actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_audit_event(record)
    }
}

const SELECT_AUDIT_EVENT: &str = "SELECT audit_event_id_value, audit_event_id_namespace,
        workspace_id_value, workspace_id_namespace,
        event_type, actor,
        policy_set_id_value, policy_set_id_namespace,
        configuration_id_value, configuration_id_namespace,
        provider_state_id_value, provider_state_id_namespace,
        input_refs, output_refs, decision, reason_code, created_at,
        record_created_at, record_updated_at, record_actor,
        record_policy_version
 FROM audit_events
 WHERE audit_event_id_value = ? AND audit_event_id_namespace = ?";

fn validate_audit_event(record: AuditEvent) -> Result<AuditEvent, GraphStoreError> {
    OpaqueId::<AuditEvent>::new(
        record.audit_event_id.value.clone(),
        record.audit_event_id.namespace.clone(),
    )?;
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    OpaqueId::<PolicySet>::new(
        record.policy_set_id.value.clone(),
        record.policy_set_id.namespace.clone(),
    )?;
    if let Some(configuration_id) = &record.configuration_id {
        OpaqueId::<GraphConfiguration>::new(
            configuration_id.value.clone(),
            configuration_id.namespace.clone(),
        )?;
    }
    if let Some(provider_state_id) = &record.provider_state_id {
        OpaqueId::<ProviderStateRef>::new(
            provider_state_id.value.clone(),
            provider_state_id.namespace.clone(),
        )?;
    }
    validate_record_meta(record.meta.clone())?;
    validate_contract_timestamp(&record.created_at, &record.meta)?;

    if record.event_type.trim().is_empty()
        || record.actor.trim().is_empty()
        || record.reason_code.trim().is_empty()
        || record
            .input_refs
            .0
            .iter()
            .any(|value| value.trim().is_empty())
        || record
            .output_refs
            .0
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    if !record.output_refs.0.is_empty() && record.input_refs.0.is_empty() {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(record)
}

fn validate_contract_timestamp(value: &str, meta: &RecordMeta) -> Result<(), GraphStoreError> {
    validate_record_meta(RecordMeta {
        created_at: value.to_string(),
        updated_at: value.to_string(),
        actor: meta.actor.clone(),
        policy_version: meta.policy_version.clone(),
    })?;
    Ok(())
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
            if message.contains("UNIQUE constraint failed: audit_events.audit_event_id_value") {
                GraphStoreError::DuplicateId
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed: decision") {
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

impl AuditDecision {
    fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
            Self::Quarantined => "quarantined",
            Self::UserRequired => "user_required",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "deferred" => Ok(Self::Deferred),
            "quarantined" => Ok(Self::Quarantined),
            "user_required" => Ok(Self::UserRequired),
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

impl_sqlite_text_enum!(AuditDecision);
