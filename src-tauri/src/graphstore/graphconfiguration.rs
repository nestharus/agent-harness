use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::{Database, Sqlite, SqlitePool};

use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
pub use crate::graphstore::graphworkspace::GraphWorkspace;
use crate::graphstore::policyset::PolicySet;
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

const ATTRIBUTED_FIELDS: &[&str] = &[
    "configuration_version",
    "schema_profile",
    "summary_contract_template_ids",
    "optimizer_policy_ref",
    "render_policy_ref",
    "memory_policy_ref",
    "provider_routing_policy_ref",
    "capability_fingerprint_policy_ref",
    "validation_state",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveValueSource {
    SystemRequired,
    Default,
    Inherited,
    UserConfigured,
    Recovered,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectiveValueSourceEntry {
    pub field_name: String,
    pub source: EffectiveValueSource,
    #[serde(default)]
    pub source_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationState {
    Valid,
    ValidWithWarnings,
    InvalidSchema,
    InvalidProviderRoute,
    InvalidBudget,
    InvalidIndex,
    NeedsUserAttention,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphConfiguration {
    pub configuration_id: OpaqueId<GraphConfiguration>,
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub configuration_version: i64,
    pub schema_profile: String,
    pub summary_contract_template_ids: JsonField<Vec<String>>,
    pub optimizer_policy_ref: OpaqueId<PolicySet>,
    pub render_policy_ref: OpaqueId<PolicySet>,
    pub memory_policy_ref: OpaqueId<PolicySet>,
    pub provider_routing_policy_ref: OpaqueId<PolicySet>,
    pub capability_fingerprint_policy_ref: OpaqueId<PolicySet>,
    pub effective_value_sources: JsonField<Vec<EffectiveValueSourceEntry>>,
    pub created_from_configuration_id: Option<OpaqueId<GraphConfiguration>>,
    pub validation_state: ValidationState,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct GraphConfigurationRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct GraphConfigurationRow {
    configuration_id_value: String,
    configuration_id_namespace: String,
    workspace_id_value: String,
    workspace_id_namespace: String,
    configuration_version: i64,
    schema_profile: String,
    summary_contract_template_ids: JsonField<Vec<String>>,
    optimizer_policy_ref_value: String,
    optimizer_policy_ref_namespace: String,
    render_policy_ref_value: String,
    render_policy_ref_namespace: String,
    memory_policy_ref_value: String,
    memory_policy_ref_namespace: String,
    provider_routing_policy_ref_value: String,
    provider_routing_policy_ref_namespace: String,
    capability_fingerprint_policy_ref_value: String,
    capability_fingerprint_policy_ref_namespace: String,
    effective_value_sources: JsonField<Vec<EffectiveValueSourceEntry>>,
    created_from_configuration_id_value: Option<String>,
    created_from_configuration_id_namespace: Option<String>,
    validation_state: ValidationState,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

#[derive(Debug, sqlx::FromRow)]
struct RevisionParentRow {
    workspace_id_value: String,
    workspace_id_namespace: String,
    created_at: String,
}

impl GraphConfigurationRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_graph_configuration(
        &self,
        record: GraphConfiguration,
    ) -> Result<GraphConfiguration, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_graph_configuration(record) {
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
            "SELECT COUNT(*) FROM graph_configurations
             WHERE configuration_id_value = ? AND configuration_id_namespace = ?",
        )
        .bind(&record.configuration_id.value)
        .bind(&record.configuration_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            transaction
                .rollback()
                .await
                .map_err(GraphStoreError::from)?;
            return Err(GraphStoreError::DuplicateId);
        }

        if let Err(error) = self
            .validate_revision_parent(&mut transaction, &record)
            .await
        {
            transaction
                .rollback()
                .await
                .map_err(GraphStoreError::from)?;
            return Err(error);
        }

        let created_from_value = record
            .created_from_configuration_id
            .as_ref()
            .map(|id| id.value.as_str());
        let created_from_namespace = record
            .created_from_configuration_id
            .as_ref()
            .map(|id| id.namespace.as_str());

        let insert_result = sqlx::query(
            "INSERT INTO graph_configurations
             (configuration_id_value, configuration_id_namespace,
              workspace_id_value, workspace_id_namespace,
              configuration_version, schema_profile, summary_contract_template_ids,
              optimizer_policy_ref_value, optimizer_policy_ref_namespace,
              render_policy_ref_value, render_policy_ref_namespace,
              memory_policy_ref_value, memory_policy_ref_namespace,
              provider_routing_policy_ref_value, provider_routing_policy_ref_namespace,
              capability_fingerprint_policy_ref_value,
              capability_fingerprint_policy_ref_namespace,
              effective_value_sources,
              created_from_configuration_id_value,
              created_from_configuration_id_namespace,
              validation_state,
              created_at, updated_at, actor, record_policy_version, schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 5)",
        )
        .bind(&record.configuration_id.value)
        .bind(&record.configuration_id.namespace)
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(record.configuration_version)
        .bind(&record.schema_profile)
        .bind(&record.summary_contract_template_ids)
        .bind(&record.optimizer_policy_ref.value)
        .bind(&record.optimizer_policy_ref.namespace)
        .bind(&record.render_policy_ref.value)
        .bind(&record.render_policy_ref.namespace)
        .bind(&record.memory_policy_ref.value)
        .bind(&record.memory_policy_ref.namespace)
        .bind(&record.provider_routing_policy_ref.value)
        .bind(&record.provider_routing_policy_ref.namespace)
        .bind(&record.capability_fingerprint_policy_ref.value)
        .bind(&record.capability_fingerprint_policy_ref.namespace)
        .bind(&record.effective_value_sources)
        .bind(created_from_value)
        .bind(created_from_namespace)
        .bind(record.validation_state)
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
                Err(map_insert_error(&error))
            }
        }
    }

    pub async fn get_graph_configuration(
        &self,
        id: &OpaqueId<GraphConfiguration>,
    ) -> Result<Option<GraphConfiguration>, GraphStoreError> {
        OpaqueId::<GraphConfiguration>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, GraphConfigurationRow>(SELECT_GRAPH_CONFIGURATION)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(GraphConfiguration::try_from).transpose()
    }

    pub async fn list_graph_configurations_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Vec<GraphConfiguration>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, GraphConfigurationRow>(
            "SELECT configuration_id_value, configuration_id_namespace,
                    workspace_id_value, workspace_id_namespace,
                    configuration_version, schema_profile,
                    summary_contract_template_ids,
                    optimizer_policy_ref_value, optimizer_policy_ref_namespace,
                    render_policy_ref_value, render_policy_ref_namespace,
                    memory_policy_ref_value, memory_policy_ref_namespace,
                    provider_routing_policy_ref_value,
                    provider_routing_policy_ref_namespace,
                    capability_fingerprint_policy_ref_value,
                    capability_fingerprint_policy_ref_namespace,
                    effective_value_sources,
                    created_from_configuration_id_value,
                    created_from_configuration_id_namespace,
                    validation_state,
                    created_at, updated_at, actor, record_policy_version
             FROM graph_configurations
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?
             ORDER BY configuration_version, row_id",
        )
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(GraphConfiguration::try_from).collect()
    }

    async fn validate_revision_parent(
        &self,
        transaction: &mut sqlx::Transaction<'_, Sqlite>,
        record: &GraphConfiguration,
    ) -> Result<(), GraphStoreError> {
        let Some(parent_id) = &record.created_from_configuration_id else {
            return Ok(());
        };

        let parent = sqlx::query_as::<_, RevisionParentRow>(
            "SELECT workspace_id_value, workspace_id_namespace, created_at
             FROM graph_configurations
             WHERE configuration_id_value = ? AND configuration_id_namespace = ?",
        )
        .bind(&parent_id.value)
        .bind(&parent_id.namespace)
        .fetch_optional(&mut **transaction)
        .await?;

        let parent = parent.ok_or(GraphStoreError::UnknownRef)?;
        if parent.workspace_id_value != record.workspace_id.value
            || parent.workspace_id_namespace != record.workspace_id.namespace
            || parent.created_at >= record.meta.created_at
        {
            return Err(GraphStoreError::InvalidTransition);
        }

        Ok(())
    }
}

impl GraphStoreRepo<GraphConfiguration> for GraphConfigurationRepo {
    type IdMarker = GraphConfiguration;

    fn insert<'a>(
        &'a self,
        record: GraphConfiguration,
    ) -> GraphStoreRepoFuture<'a, GraphConfiguration> {
        Box::pin(async move { self.insert_graph_configuration(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<GraphConfiguration>> {
        Box::pin(async move { self.get_graph_configuration(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<GraphConfiguration>> {
        Box::pin(async move {
            let workspace_id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            self.list_graph_configurations_by_workspace(&workspace_id)
                .await
        })
    }
}

impl TryFrom<GraphConfigurationRow> for GraphConfiguration {
    type Error = GraphStoreError;

    fn try_from(row: GraphConfigurationRow) -> Result<Self, Self::Error> {
        let created_from_configuration_id = match (
            row.created_from_configuration_id_value,
            row.created_from_configuration_id_namespace,
        ) {
            (Some(value), Some(namespace)) => {
                Some(OpaqueId::<GraphConfiguration>::new(value, namespace)?)
            }
            (None, None) => None,
            _ => return Err(GraphStoreError::InvariantViolation),
        };

        let record = GraphConfiguration {
            configuration_id: OpaqueId::<GraphConfiguration>::new(
                row.configuration_id_value,
                row.configuration_id_namespace,
            )?,
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            configuration_version: row.configuration_version,
            schema_profile: row.schema_profile,
            summary_contract_template_ids: row.summary_contract_template_ids,
            optimizer_policy_ref: OpaqueId::<PolicySet>::new(
                row.optimizer_policy_ref_value,
                row.optimizer_policy_ref_namespace,
            )?,
            render_policy_ref: OpaqueId::<PolicySet>::new(
                row.render_policy_ref_value,
                row.render_policy_ref_namespace,
            )?,
            memory_policy_ref: OpaqueId::<PolicySet>::new(
                row.memory_policy_ref_value,
                row.memory_policy_ref_namespace,
            )?,
            provider_routing_policy_ref: OpaqueId::<PolicySet>::new(
                row.provider_routing_policy_ref_value,
                row.provider_routing_policy_ref_namespace,
            )?,
            capability_fingerprint_policy_ref: OpaqueId::<PolicySet>::new(
                row.capability_fingerprint_policy_ref_value,
                row.capability_fingerprint_policy_ref_namespace,
            )?,
            effective_value_sources: row.effective_value_sources,
            created_from_configuration_id,
            validation_state: row.validation_state,
            meta: validate_record_meta(RecordMeta {
                created_at: row.created_at,
                updated_at: row.updated_at,
                actor: row.actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_graph_configuration(record)
    }
}

const SELECT_GRAPH_CONFIGURATION: &str =
    "SELECT configuration_id_value, configuration_id_namespace,
        workspace_id_value, workspace_id_namespace,
        configuration_version, schema_profile,
        summary_contract_template_ids,
        optimizer_policy_ref_value, optimizer_policy_ref_namespace,
        render_policy_ref_value, render_policy_ref_namespace,
        memory_policy_ref_value, memory_policy_ref_namespace,
        provider_routing_policy_ref_value, provider_routing_policy_ref_namespace,
        capability_fingerprint_policy_ref_value,
        capability_fingerprint_policy_ref_namespace,
        effective_value_sources,
        created_from_configuration_id_value,
        created_from_configuration_id_namespace,
        validation_state,
        created_at, updated_at, actor, record_policy_version
 FROM graph_configurations
 WHERE configuration_id_value = ? AND configuration_id_namespace = ?";

fn validate_graph_configuration(
    record: GraphConfiguration,
) -> Result<GraphConfiguration, GraphStoreError> {
    OpaqueId::<GraphConfiguration>::new(
        record.configuration_id.value.clone(),
        record.configuration_id.namespace.clone(),
    )?;
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    validate_policy_ref(&record.optimizer_policy_ref)?;
    validate_policy_ref(&record.render_policy_ref)?;
    validate_policy_ref(&record.memory_policy_ref)?;
    validate_policy_ref(&record.provider_routing_policy_ref)?;
    validate_policy_ref(&record.capability_fingerprint_policy_ref)?;
    if let Some(parent_id) = &record.created_from_configuration_id {
        OpaqueId::<GraphConfiguration>::new(parent_id.value.clone(), parent_id.namespace.clone())?;
    }
    validate_record_meta(record.meta.clone())?;

    if record.configuration_version <= 0
        || record.schema_profile.trim().is_empty()
        || record.summary_contract_template_ids.0.is_empty()
        || record
            .summary_contract_template_ids
            .0
            .iter()
            .any(|id| id.trim().is_empty())
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    validate_effective_value_sources(&record.effective_value_sources.0)?;

    Ok(record)
}

fn validate_policy_ref(id: &OpaqueId<PolicySet>) -> Result<(), GraphStoreError> {
    OpaqueId::<PolicySet>::new(id.value.clone(), id.namespace.clone()).map(|_| ())
}

fn validate_effective_value_sources(
    entries: &[EffectiveValueSourceEntry],
) -> Result<(), GraphStoreError> {
    if entries.is_empty() {
        return Err(GraphStoreError::InvariantViolation);
    }

    let mut present_fields = HashSet::new();
    for entry in entries {
        if entry.field_name.trim().is_empty() || entry.source_ref.trim().is_empty() {
            return Err(GraphStoreError::InvariantViolation);
        }
        present_fields.insert(entry.field_name.as_str());
    }

    if ATTRIBUTED_FIELDS
        .iter()
        .any(|field| !present_fields.contains(field))
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(())
}

fn map_insert_error(error: &sqlx::Error) -> GraphStoreError {
    match error {
        sqlx::Error::Database(database_error) => {
            let message = database_error.message();
            if message
                .contains("UNIQUE constraint failed: graph_configurations.configuration_id_value")
            {
                GraphStoreError::DuplicateId
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed: validation_state") {
                GraphStoreError::InvalidEnum
            } else {
                GraphStoreError::SqlxFailure
            }
        }
        _ => GraphStoreError::SqlxFailure,
    }
}

impl EffectiveValueSource {
    fn as_str(self) -> &'static str {
        match self {
            Self::SystemRequired => "system_required",
            Self::Default => "default",
            Self::Inherited => "inherited",
            Self::UserConfigured => "user_configured",
            Self::Recovered => "recovered",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "system_required" => Ok(Self::SystemRequired),
            "default" => Ok(Self::Default),
            "inherited" => Ok(Self::Inherited),
            "user_configured" => Ok(Self::UserConfigured),
            "recovered" => Ok(Self::Recovered),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ValidationState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::ValidWithWarnings => "valid_with_warnings",
            Self::InvalidSchema => "invalid_schema",
            Self::InvalidProviderRoute => "invalid_provider_route",
            Self::InvalidBudget => "invalid_budget",
            Self::InvalidIndex => "invalid_index",
            Self::NeedsUserAttention => "needs_user_attention",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "valid" => Ok(Self::Valid),
            "valid_with_warnings" => Ok(Self::ValidWithWarnings),
            "invalid_schema" => Ok(Self::InvalidSchema),
            "invalid_provider_route" => Ok(Self::InvalidProviderRoute),
            "invalid_budget" => Ok(Self::InvalidBudget),
            "invalid_index" => Ok(Self::InvalidIndex),
            "needs_user_attention" => Ok(Self::NeedsUserAttention),
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

impl_sqlite_text_enum!(EffectiveValueSource);
impl_sqlite_text_enum!(ValidationState);
