use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::{Database, Sqlite, SqlitePool};

use crate::graphstore::fixture::{GraphStoreRepo, GraphStoreRepoFuture, GraphWorkspaceRef};
use crate::graphstore::graphworkspace::GraphWorkspace;
use crate::graphstore::prelude::{
    validate_record_meta, ActorRef, GraphStoreError, JsonField, OpaqueId, RecordMeta,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeRevisionRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphEdgeRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeKind {
    Initiative,
    WorkUnit,
    Summary,
    Evidence,
    Question,
    Decision,
    Blocker,
    WorkerOutput,
    Recovery,
    PolicyNote,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Active,
    Packed,
    Unpacked,
    Blocked,
    Stale,
    Recovering,
    Archived,
    Quarantined,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivilegeOrigin {
    System,
    User,
    Tool,
    Model,
    Worker,
    Optimizer,
    Reviewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Trusted,
    Derived,
    Unverified,
    Suspect,
    PoisonQuarantined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphNode {
    pub node_id: OpaqueId<GraphNode>,
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub kind: GraphNodeKind,
    pub title: String,
    pub lifecycle_state: LifecycleState,
    pub current_revision_id: Option<OpaqueId<NodeRevisionRef>>,
    pub canonical_parent_edge_id: Option<OpaqueId<GraphEdgeRef>>,
    pub privilege_origin: PrivilegeOrigin,
    pub trust_state: TrustState,
    pub created_from_ref: Option<JsonField<Value>>,
    pub deleted_at: Option<String>,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct GraphNodeRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct GraphNodeRow {
    node_id_value: String,
    node_id_namespace: String,
    workspace_id_value: String,
    workspace_id_namespace: String,
    kind: GraphNodeKind,
    title: String,
    lifecycle_state: LifecycleState,
    current_revision_id_value: Option<String>,
    current_revision_id_namespace: Option<String>,
    canonical_parent_edge_id_value: Option<String>,
    canonical_parent_edge_id_namespace: Option<String>,
    privilege_origin: PrivilegeOrigin,
    trust_state: TrustState,
    created_from_ref: Option<JsonField<Value>>,
    deleted_at: Option<String>,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl GraphNodeRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_graph_node(&self, record: GraphNode) -> Result<GraphNode, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_graph_node(record) {
            Ok(record) => record,
            Err(error) => {
                rollback(transaction).await?;
                return Err(error);
            }
        };

        if record.current_revision_id.is_some() || record.canonical_parent_edge_id.is_some() {
            rollback(transaction).await?;
            return Err(GraphStoreError::UnknownRef);
        }

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM graph_nodes
             WHERE node_id_value = ? AND node_id_namespace = ?",
        )
        .bind(&record.node_id.value)
        .bind(&record.node_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::DuplicateId);
        }

        let insert_result = sqlx::query(
            "INSERT INTO graph_nodes
             (node_id_value, node_id_namespace,
              workspace_id_value, workspace_id_namespace,
              kind, title, lifecycle_state,
              current_revision_id_value, current_revision_id_namespace,
              canonical_parent_edge_id_value, canonical_parent_edge_id_namespace,
              privilege_origin, trust_state, created_from_ref, deleted_at,
              created_at, updated_at, actor, record_policy_version, schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 7)",
        )
        .bind(&record.node_id.value)
        .bind(&record.node_id.namespace)
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(record.kind)
        .bind(&record.title)
        .bind(record.lifecycle_state)
        .bind(
            record
                .current_revision_id
                .as_ref()
                .map(|id| id.value.as_str()),
        )
        .bind(
            record
                .current_revision_id
                .as_ref()
                .map(|id| id.namespace.as_str()),
        )
        .bind(
            record
                .canonical_parent_edge_id
                .as_ref()
                .map(|id| id.value.as_str()),
        )
        .bind(
            record
                .canonical_parent_edge_id
                .as_ref()
                .map(|id| id.namespace.as_str()),
        )
        .bind(record.privilege_origin)
        .bind(record.trust_state)
        .bind(&record.created_from_ref)
        .bind(&record.deleted_at)
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

    pub async fn get_graph_node(
        &self,
        id: &OpaqueId<GraphNode>,
    ) -> Result<Option<GraphNode>, GraphStoreError> {
        OpaqueId::<GraphNode>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, GraphNodeRow>(SELECT_GRAPH_NODE)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(GraphNode::try_from).transpose()
    }

    pub async fn list_graph_nodes_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Vec<GraphNode>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, GraphNodeRow>(
            "SELECT node_id_value, node_id_namespace,
                    workspace_id_value, workspace_id_namespace,
                    kind, title, lifecycle_state,
                    current_revision_id_value, current_revision_id_namespace,
                    canonical_parent_edge_id_value, canonical_parent_edge_id_namespace,
                    privilege_origin, trust_state, created_from_ref, deleted_at,
                    created_at, updated_at, actor, record_policy_version
             FROM graph_nodes
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(GraphNode::try_from).collect()
    }

    pub async fn transition_lifecycle(
        &self,
        node_id: &OpaqueId<GraphNode>,
        target_state: LifecycleState,
    ) -> Result<GraphNode, GraphStoreError> {
        OpaqueId::<GraphNode>::new(node_id.value.clone(), node_id.namespace.clone())?;
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let current = match select_node(&mut transaction, node_id).await? {
            Some(current) => current,
            None => {
                rollback(transaction).await?;
                return Err(GraphStoreError::UnknownRef);
            }
        };

        if !is_valid_transition(current.lifecycle_state, target_state) {
            rollback(transaction).await?;
            return Err(GraphStoreError::InvalidTransition);
        }

        let updated_at = next_contract_timestamp(&current.meta.updated_at)?;
        let update_result = sqlx::query(
            "UPDATE graph_nodes
             SET lifecycle_state = ?,
                 updated_at = ?
             WHERE node_id_value = ? AND node_id_namespace = ?",
        )
        .bind(target_state)
        .bind(&updated_at)
        .bind(&node_id.value)
        .bind(&node_id.namespace)
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

        let updated = select_node(&mut transaction, node_id)
            .await?
            .ok_or(GraphStoreError::UnknownRef)?;
        transaction.commit().await.map_err(GraphStoreError::from)?;
        Ok(updated)
    }
}

impl GraphStoreRepo<GraphNode> for GraphNodeRepo {
    type IdMarker = GraphNode;

    fn insert<'a>(&'a self, record: GraphNode) -> GraphStoreRepoFuture<'a, GraphNode> {
        Box::pin(async move { self.insert_graph_node(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<GraphNode>> {
        Box::pin(async move { self.get_graph_node(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<GraphNode>> {
        Box::pin(async move {
            let workspace_id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            self.list_graph_nodes_by_workspace(&workspace_id).await
        })
    }
}

impl TryFrom<GraphNodeRow> for GraphNode {
    type Error = GraphStoreError;

    fn try_from(row: GraphNodeRow) -> Result<Self, Self::Error> {
        let current_revision_id = match (
            row.current_revision_id_value,
            row.current_revision_id_namespace,
        ) {
            (Some(value), Some(namespace)) => {
                Some(OpaqueId::<NodeRevisionRef>::new(value, namespace)?)
            }
            (None, None) => None,
            _ => return Err(GraphStoreError::InvariantViolation),
        };
        let canonical_parent_edge_id = match (
            row.canonical_parent_edge_id_value,
            row.canonical_parent_edge_id_namespace,
        ) {
            (Some(value), Some(namespace)) => {
                Some(OpaqueId::<GraphEdgeRef>::new(value, namespace)?)
            }
            (None, None) => None,
            _ => return Err(GraphStoreError::InvariantViolation),
        };

        let record = GraphNode {
            node_id: OpaqueId::<GraphNode>::new(row.node_id_value, row.node_id_namespace)?,
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            kind: row.kind,
            title: row.title,
            lifecycle_state: row.lifecycle_state,
            current_revision_id,
            canonical_parent_edge_id,
            privilege_origin: row.privilege_origin,
            trust_state: row.trust_state,
            created_from_ref: row.created_from_ref,
            deleted_at: row.deleted_at,
            meta: validate_record_meta(RecordMeta {
                created_at: row.created_at,
                updated_at: row.updated_at,
                actor: row.actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_graph_node(record)
    }
}

const SELECT_GRAPH_NODE: &str = "SELECT node_id_value, node_id_namespace,
        workspace_id_value, workspace_id_namespace,
        kind, title, lifecycle_state,
        current_revision_id_value, current_revision_id_namespace,
        canonical_parent_edge_id_value, canonical_parent_edge_id_namespace,
        privilege_origin, trust_state, created_from_ref, deleted_at,
        created_at, updated_at, actor, record_policy_version
 FROM graph_nodes
 WHERE node_id_value = ? AND node_id_namespace = ?";

async fn select_node(
    transaction: &mut sqlx::Transaction<'_, Sqlite>,
    id: &OpaqueId<GraphNode>,
) -> Result<Option<GraphNode>, GraphStoreError> {
    let row = sqlx::query_as::<_, GraphNodeRow>(SELECT_GRAPH_NODE)
        .bind(&id.value)
        .bind(&id.namespace)
        .fetch_optional(&mut **transaction)
        .await?;

    row.map(GraphNode::try_from).transpose()
}

fn validate_graph_node(record: GraphNode) -> Result<GraphNode, GraphStoreError> {
    OpaqueId::<GraphNode>::new(
        record.node_id.value.clone(),
        record.node_id.namespace.clone(),
    )?;
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    if let Some(id) = &record.current_revision_id {
        OpaqueId::<NodeRevisionRef>::new(id.value.clone(), id.namespace.clone())?;
    }
    if let Some(id) = &record.canonical_parent_edge_id {
        OpaqueId::<GraphEdgeRef>::new(id.value.clone(), id.namespace.clone())?;
    }
    validate_record_meta(record.meta.clone())?;

    if record.title.trim().is_empty() {
        return Err(GraphStoreError::InvariantViolation);
    }

    match (record.lifecycle_state, &record.deleted_at) {
        (LifecycleState::Deleted, Some(deleted_at)) => {
            validate_record_meta(RecordMeta {
                created_at: record.meta.created_at.clone(),
                updated_at: deleted_at.clone(),
                actor: record.meta.actor.clone(),
                policy_version: record.meta.policy_version.clone(),
            })?;
        }
        (LifecycleState::Deleted, None) | (_, Some(_)) => {
            return Err(GraphStoreError::InvariantViolation);
        }
        (_, None) => {}
    }

    Ok(record)
}

fn is_valid_transition(source: LifecycleState, target: LifecycleState) -> bool {
    if source == LifecycleState::Deleted || target == LifecycleState::Deleted {
        return false;
    }

    matches!(
        (source, target),
        (LifecycleState::Active, LifecycleState::Packed)
            | (LifecycleState::Packed, LifecycleState::Unpacked)
            | (LifecycleState::Active, LifecycleState::Blocked)
            | (LifecycleState::Blocked, LifecycleState::Active)
            | (LifecycleState::Active, LifecycleState::Stale)
            | (LifecycleState::Stale, LifecycleState::Active)
            | (_, LifecycleState::Archived)
            | (_, LifecycleState::Quarantined)
            | (LifecycleState::Quarantined, LifecycleState::Recovering)
    )
}

async fn rollback(transaction: sqlx::Transaction<'_, Sqlite>) -> Result<(), GraphStoreError> {
    transaction.rollback().await.map_err(GraphStoreError::from)
}

fn map_write_error(error: &sqlx::Error) -> GraphStoreError {
    match error {
        sqlx::Error::Database(database_error) => {
            let message = database_error.message();
            if message.contains("UNIQUE constraint failed: graph_nodes.node_id_value") {
                GraphStoreError::DuplicateId
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed: kind")
                || message.contains("CHECK constraint failed: lifecycle_state")
                || message.contains("CHECK constraint failed: privilege_origin")
                || message.contains("CHECK constraint failed: trust_state")
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

impl GraphNodeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Initiative => "initiative",
            Self::WorkUnit => "work_unit",
            Self::Summary => "summary",
            Self::Evidence => "evidence",
            Self::Question => "question",
            Self::Decision => "decision",
            Self::Blocker => "blocker",
            Self::WorkerOutput => "worker_output",
            Self::Recovery => "recovery",
            Self::PolicyNote => "policy_note",
            Self::Archive => "archive",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "initiative" => Ok(Self::Initiative),
            "work_unit" => Ok(Self::WorkUnit),
            "summary" => Ok(Self::Summary),
            "evidence" => Ok(Self::Evidence),
            "question" => Ok(Self::Question),
            "decision" => Ok(Self::Decision),
            "blocker" => Ok(Self::Blocker),
            "worker_output" => Ok(Self::WorkerOutput),
            "recovery" => Ok(Self::Recovery),
            "policy_note" => Ok(Self::PolicyNote),
            "archive" => Ok(Self::Archive),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl LifecycleState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Packed => "packed",
            Self::Unpacked => "unpacked",
            Self::Blocked => "blocked",
            Self::Stale => "stale",
            Self::Recovering => "recovering",
            Self::Archived => "archived",
            Self::Quarantined => "quarantined",
            Self::Deleted => "deleted",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "active" => Ok(Self::Active),
            "packed" => Ok(Self::Packed),
            "unpacked" => Ok(Self::Unpacked),
            "blocked" => Ok(Self::Blocked),
            "stale" => Ok(Self::Stale),
            "recovering" => Ok(Self::Recovering),
            "archived" => Ok(Self::Archived),
            "quarantined" => Ok(Self::Quarantined),
            "deleted" => Ok(Self::Deleted),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl PrivilegeOrigin {
    fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Tool => "tool",
            Self::Model => "model",
            Self::Worker => "worker",
            Self::Optimizer => "optimizer",
            Self::Reviewer => "reviewer",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "system" => Ok(Self::System),
            "user" => Ok(Self::User),
            "tool" => Ok(Self::Tool),
            "model" => Ok(Self::Model),
            "worker" => Ok(Self::Worker),
            "optimizer" => Ok(Self::Optimizer),
            "reviewer" => Ok(Self::Reviewer),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl TrustState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Derived => "derived",
            Self::Unverified => "unverified",
            Self::Suspect => "suspect",
            Self::PoisonQuarantined => "poison_quarantined",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "trusted" => Ok(Self::Trusted),
            "derived" => Ok(Self::Derived),
            "unverified" => Ok(Self::Unverified),
            "suspect" => Ok(Self::Suspect),
            "poison_quarantined" => Ok(Self::PoisonQuarantined),
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

impl_sqlite_text_enum!(GraphNodeKind);
impl_sqlite_text_enum!(LifecycleState);
impl_sqlite_text_enum!(PrivilegeOrigin);
impl_sqlite_text_enum!(TrustState);
