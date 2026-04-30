use std::fs;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::{Database, Sqlite, SqlitePool};

use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
use crate::graphstore::graphnode::PrivilegeOrigin;
use crate::graphstore::graphworkspace::GraphWorkspace;
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    CliTranscript,
    ToolCall,
    ToolResult,
    FileRead,
    FilePatch,
    CommandOutput,
    UserMessage,
    WorkerOutput,
    OptimizerPrompt,
    ReviewerOutput,
    ExternalDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolProtocol {
    Claude,
    Codex,
    Opencode,
    Mcp,
    Shell,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureState {
    Captured,
    Partial,
    Failed,
    Redacted,
    Quarantined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceArtifact {
    pub evidence_id: OpaqueId<EvidenceArtifact>,
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub source_type: SourceType,
    pub source_uri: String,
    pub source_session_id: Option<String>,
    pub tool_protocol: ToolProtocol,
    pub correlation_key: Option<String>,
    pub content_hash: Option<String>,
    pub blob_ref: String,
    pub privilege_origin: PrivilegeOrigin,
    pub capture_state: CaptureState,
    pub captured_at: String,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct EvidenceArtifactRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct EvidenceArtifactRow {
    evidence_id_value: String,
    evidence_id_namespace: String,
    workspace_id_value: String,
    workspace_id_namespace: String,
    source_type: SourceType,
    source_uri: String,
    source_session_id: Option<String>,
    tool_protocol: ToolProtocol,
    correlation_key: Option<String>,
    content_hash: Option<String>,
    blob_ref: String,
    privilege_origin: PrivilegeOrigin,
    capture_state: CaptureState,
    captured_at: String,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl EvidenceArtifactRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_evidence_artifact(
        &self,
        record: EvidenceArtifact,
    ) -> Result<EvidenceArtifact, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_evidence_artifact(record) {
            Ok(record) => record,
            Err(error) => {
                rollback(transaction).await?;
                return Err(error);
            }
        };

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM evidence_artifacts
             WHERE evidence_id_value = ? AND evidence_id_namespace = ?",
        )
        .bind(&record.evidence_id.value)
        .bind(&record.evidence_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::DuplicateId);
        }

        let storage_root =
            match workspace_storage_root(&mut transaction, &record.workspace_id).await? {
                Some(storage_root) => storage_root,
                None => {
                    rollback(transaction).await?;
                    return Err(GraphStoreError::UnknownRef);
                }
            };

        if let Err(error) = validate_blob_ref_under_storage_root(&record.blob_ref, &storage_root) {
            rollback(transaction).await?;
            return Err(error);
        }

        let insert_result = sqlx::query(
            "INSERT INTO evidence_artifacts
             (evidence_id_value, evidence_id_namespace,
              workspace_id_value, workspace_id_namespace,
              source_type, source_uri, source_session_id, tool_protocol,
              correlation_key, content_hash, blob_ref, privilege_origin,
              capture_state, captured_at,
              created_at, updated_at, actor, record_policy_version, schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 9)",
        )
        .bind(&record.evidence_id.value)
        .bind(&record.evidence_id.namespace)
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(record.source_type)
        .bind(&record.source_uri)
        .bind(&record.source_session_id)
        .bind(record.tool_protocol)
        .bind(&record.correlation_key)
        .bind(&record.content_hash)
        .bind(&record.blob_ref)
        .bind(record.privilege_origin)
        .bind(record.capture_state)
        .bind(&record.captured_at)
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

    pub async fn get_evidence_artifact(
        &self,
        id: &OpaqueId<EvidenceArtifact>,
    ) -> Result<Option<EvidenceArtifact>, GraphStoreError> {
        OpaqueId::<EvidenceArtifact>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, EvidenceArtifactRow>(SELECT_EVIDENCE_ARTIFACT)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(EvidenceArtifact::try_from).transpose()
    }

    pub async fn list_evidence_artifacts_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Vec<EvidenceArtifact>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, EvidenceArtifactRow>(
            "SELECT evidence_id_value, evidence_id_namespace,
                    workspace_id_value, workspace_id_namespace,
                    source_type, source_uri, source_session_id, tool_protocol,
                    correlation_key, content_hash, blob_ref, privilege_origin,
                    capture_state, captured_at,
                    created_at, updated_at, actor, record_policy_version
             FROM evidence_artifacts
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(EvidenceArtifact::try_from).collect()
    }
}

impl GraphStoreRepo<EvidenceArtifact> for EvidenceArtifactRepo {
    type IdMarker = EvidenceArtifact;

    fn insert<'a>(
        &'a self,
        record: EvidenceArtifact,
    ) -> GraphStoreRepoFuture<'a, EvidenceArtifact> {
        Box::pin(async move { self.insert_evidence_artifact(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<EvidenceArtifact>> {
        Box::pin(async move { self.get_evidence_artifact(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<EvidenceArtifact>> {
        Box::pin(async move {
            let workspace_id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            self.list_evidence_artifacts_by_workspace(&workspace_id)
                .await
        })
    }
}

impl TryFrom<EvidenceArtifactRow> for EvidenceArtifact {
    type Error = GraphStoreError;

    fn try_from(row: EvidenceArtifactRow) -> Result<Self, Self::Error> {
        let record = EvidenceArtifact {
            evidence_id: OpaqueId::<EvidenceArtifact>::new(
                row.evidence_id_value,
                row.evidence_id_namespace,
            )?,
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            source_type: row.source_type,
            source_uri: row.source_uri,
            source_session_id: row.source_session_id,
            tool_protocol: row.tool_protocol,
            correlation_key: row.correlation_key,
            content_hash: row.content_hash,
            blob_ref: row.blob_ref,
            privilege_origin: row.privilege_origin,
            capture_state: row.capture_state,
            captured_at: row.captured_at,
            meta: validate_record_meta(RecordMeta {
                created_at: row.created_at,
                updated_at: row.updated_at,
                actor: row.actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_evidence_artifact(record)
    }
}

const SELECT_EVIDENCE_ARTIFACT: &str = "SELECT evidence_id_value, evidence_id_namespace,
        workspace_id_value, workspace_id_namespace,
        source_type, source_uri, source_session_id, tool_protocol,
        correlation_key, content_hash, blob_ref, privilege_origin,
        capture_state, captured_at,
        created_at, updated_at, actor, record_policy_version
 FROM evidence_artifacts
 WHERE evidence_id_value = ? AND evidence_id_namespace = ?";

async fn workspace_storage_root(
    transaction: &mut sqlx::Transaction<'_, Sqlite>,
    workspace_id: &OpaqueId<GraphWorkspace>,
) -> Result<Option<String>, GraphStoreError> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT storage_root FROM graph_workspaces
         WHERE workspace_id_value = ? AND workspace_id_namespace = ?",
    )
    .bind(&workspace_id.value)
    .bind(&workspace_id.namespace)
    .fetch_optional(&mut **transaction)
    .await?;

    Ok(row.map(|(storage_root,)| storage_root))
}

fn validate_evidence_artifact(
    record: EvidenceArtifact,
) -> Result<EvidenceArtifact, GraphStoreError> {
    OpaqueId::<EvidenceArtifact>::new(
        record.evidence_id.value.clone(),
        record.evidence_id.namespace.clone(),
    )?;
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    validate_record_meta(record.meta.clone())?;
    validate_contract_timestamp(&record.captured_at, &record.meta)?;

    if record.source_uri.trim().is_empty()
        || record.blob_ref.trim().is_empty()
        || optional_string_is_empty(&record.source_session_id)
        || optional_string_is_empty(&record.correlation_key)
        || optional_string_is_empty(&record.content_hash)
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    match record.capture_state {
        CaptureState::Captured
        | CaptureState::Partial
        | CaptureState::Redacted
        | CaptureState::Quarantined => {
            if record.content_hash.is_none() {
                return Err(GraphStoreError::InvariantViolation);
            }
        }
        CaptureState::Failed => {}
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

fn optional_string_is_empty(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|value| value.trim().is_empty())
}

fn validate_blob_ref_under_storage_root(
    blob_ref: &str,
    storage_root: &str,
) -> Result<(), GraphStoreError> {
    if blob_ref.trim().is_empty() || storage_root.trim().is_empty() {
        return Err(GraphStoreError::InvariantViolation);
    }

    let relative = Path::new(blob_ref);
    if relative.is_absolute()
        || relative.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    let root = fs::canonicalize(storage_root).map_err(|_| GraphStoreError::InvariantViolation)?;
    let candidate = root.join(relative);
    let canonical_candidate =
        fs::canonicalize(candidate).map_err(|_| GraphStoreError::InvariantViolation)?;
    if !canonical_candidate.starts_with(&root) {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(())
}

async fn rollback(transaction: sqlx::Transaction<'_, Sqlite>) -> Result<(), GraphStoreError> {
    transaction.rollback().await.map_err(GraphStoreError::from)
}

fn map_write_error(error: &sqlx::Error) -> GraphStoreError {
    match error {
        sqlx::Error::Database(database_error) => {
            let message = database_error.message();
            if message.contains("UNIQUE constraint failed: evidence_artifacts.evidence_id_value") {
                GraphStoreError::DuplicateId
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed: source_type")
                || message.contains("CHECK constraint failed: tool_protocol")
                || message.contains("CHECK constraint failed: privilege_origin")
                || message.contains("CHECK constraint failed: capture_state")
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

impl SourceType {
    fn as_str(self) -> &'static str {
        match self {
            Self::CliTranscript => "cli_transcript",
            Self::ToolCall => "tool_call",
            Self::ToolResult => "tool_result",
            Self::FileRead => "file_read",
            Self::FilePatch => "file_patch",
            Self::CommandOutput => "command_output",
            Self::UserMessage => "user_message",
            Self::WorkerOutput => "worker_output",
            Self::OptimizerPrompt => "optimizer_prompt",
            Self::ReviewerOutput => "reviewer_output",
            Self::ExternalDocument => "external_document",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "cli_transcript" => Ok(Self::CliTranscript),
            "tool_call" => Ok(Self::ToolCall),
            "tool_result" => Ok(Self::ToolResult),
            "file_read" => Ok(Self::FileRead),
            "file_patch" => Ok(Self::FilePatch),
            "command_output" => Ok(Self::CommandOutput),
            "user_message" => Ok(Self::UserMessage),
            "worker_output" => Ok(Self::WorkerOutput),
            "optimizer_prompt" => Ok(Self::OptimizerPrompt),
            "reviewer_output" => Ok(Self::ReviewerOutput),
            "external_document" => Ok(Self::ExternalDocument),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ToolProtocol {
    fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Codex => "codex",
            Self::Opencode => "opencode",
            Self::Mcp => "mcp",
            Self::Shell => "shell",
            Self::None => "none",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "claude" => Ok(Self::Claude),
            "codex" => Ok(Self::Codex),
            "opencode" => Ok(Self::Opencode),
            "mcp" => Ok(Self::Mcp),
            "shell" => Ok(Self::Shell),
            "none" => Ok(Self::None),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl CaptureState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::Redacted => "redacted",
            Self::Quarantined => "quarantined",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "captured" => Ok(Self::Captured),
            "partial" => Ok(Self::Partial),
            "failed" => Ok(Self::Failed),
            "redacted" => Ok(Self::Redacted),
            "quarantined" => Ok(Self::Quarantined),
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

impl_sqlite_text_enum!(SourceType);
impl_sqlite_text_enum!(ToolProtocol);
impl_sqlite_text_enum!(CaptureState);
