use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    Anthropic,
    Openai,
    Google,
    LocalRuntime,
    OpenaiCompatible,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCli {
    Claude,
    Codex,
    Opencode,
    AgentRunner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthState {
    Present,
    Missing,
    Expired,
    Invalid,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingState {
    Healthy,
    NearLimit,
    OverLimit,
    PaymentRequired,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderQuotaState {
    Available,
    RateLimited,
    Exhausted,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkState {
    Available,
    BlockedBySandbox,
    BlockedByHost,
    Degraded,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRuntimeState {
    Installed,
    Missing,
    WrongVersion,
    Unreachable,
    NotApplicable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFreshness {
    Fresh,
    Stale,
    ProbeFailed,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderState {
    pub provider_state_id: OpaqueId<ProviderStateRef>,
    pub workspace_id: OpaqueId<GraphWorkspace>,
    pub provider: ProviderKind,
    pub cli: ProviderCli,
    pub account_ref: String,
    pub auth_state: AuthState,
    pub billing_state: BillingState,
    pub quota_state: ProviderQuotaState,
    pub network_state: NetworkState,
    pub runtime_state: ProviderRuntimeState,
    pub sandbox_constraints: JsonField<Value>,
    pub store_locations_checked: JsonField<Vec<String>>,
    pub secret_material_stored: bool,
    pub freshness: ProviderFreshness,
    pub confidence: ProviderConfidence,
    pub last_probe_at: String,
    pub meta: RecordMeta,
}

#[derive(Debug, Clone)]
pub struct ProviderStateRepo {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct ProviderStateRow {
    provider_state_id_value: String,
    provider_state_id_namespace: String,
    workspace_id_value: String,
    workspace_id_namespace: String,
    provider: ProviderKind,
    cli: ProviderCli,
    account_ref: String,
    auth_state: AuthState,
    billing_state: BillingState,
    quota_state: ProviderQuotaState,
    network_state: NetworkState,
    runtime_state: ProviderRuntimeState,
    sandbox_constraints: JsonField<Value>,
    store_locations_checked: JsonField<Vec<String>>,
    secret_material_stored: bool,
    freshness: ProviderFreshness,
    confidence: ProviderConfidence,
    last_probe_at: String,
    created_at: String,
    updated_at: String,
    actor: JsonField<ActorRef>,
    record_policy_version: String,
}

impl ProviderStateRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_provider_state(
        &self,
        record: ProviderState,
    ) -> Result<ProviderState, GraphStoreError> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let record = match validate_provider_state(record) {
            Ok(record) => record,
            Err(error) => {
                rollback(transaction).await?;
                return Err(error);
            }
        };

        let existing: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM provider_states
             WHERE provider_state_id_value = ? AND provider_state_id_namespace = ?",
        )
        .bind(&record.provider_state_id.value)
        .bind(&record.provider_state_id.namespace)
        .fetch_one(&mut *transaction)
        .await?;
        if existing.0 != 0 {
            rollback(transaction).await?;
            return Err(GraphStoreError::DuplicateId);
        }

        let insert_result = sqlx::query(
            "INSERT INTO provider_states
             (provider_state_id_value, provider_state_id_namespace,
              workspace_id_value, workspace_id_namespace,
              provider, cli, account_ref, auth_state, billing_state, quota_state,
              network_state, runtime_state, sandbox_constraints, store_locations_checked,
              secret_material_stored, freshness, confidence, last_probe_at,
              created_at, updated_at, actor, record_policy_version, schema_version)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 17)",
        )
        .bind(&record.provider_state_id.value)
        .bind(&record.provider_state_id.namespace)
        .bind(&record.workspace_id.value)
        .bind(&record.workspace_id.namespace)
        .bind(record.provider)
        .bind(record.cli)
        .bind(&record.account_ref)
        .bind(record.auth_state)
        .bind(record.billing_state)
        .bind(record.quota_state)
        .bind(record.network_state)
        .bind(record.runtime_state)
        .bind(&record.sandbox_constraints)
        .bind(&record.store_locations_checked)
        .bind(record.secret_material_stored)
        .bind(record.freshness)
        .bind(record.confidence)
        .bind(&record.last_probe_at)
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

    pub async fn get_provider_state(
        &self,
        id: &OpaqueId<ProviderStateRef>,
    ) -> Result<Option<ProviderState>, GraphStoreError> {
        OpaqueId::<ProviderStateRef>::new(id.value.clone(), id.namespace.clone())?;

        let row = sqlx::query_as::<_, ProviderStateRow>(SELECT_PROVIDER_STATE)
            .bind(&id.value)
            .bind(&id.namespace)
            .fetch_optional(&self.pool)
            .await?;

        row.map(ProviderState::try_from).transpose()
    }

    pub async fn list_provider_states_by_workspace(
        &self,
        workspace_id: &OpaqueId<GraphWorkspace>,
    ) -> Result<Vec<ProviderState>, GraphStoreError> {
        OpaqueId::<GraphWorkspace>::new(
            workspace_id.value.clone(),
            workspace_id.namespace.clone(),
        )?;

        let rows = sqlx::query_as::<_, ProviderStateRow>(
            "SELECT provider_state_id_value, provider_state_id_namespace,
                    workspace_id_value, workspace_id_namespace,
                    provider, cli, account_ref, auth_state, billing_state, quota_state,
                    network_state, runtime_state, sandbox_constraints, store_locations_checked,
                    secret_material_stored, freshness, confidence, last_probe_at,
                    created_at, updated_at, actor, record_policy_version
             FROM provider_states
             WHERE workspace_id_value = ? AND workspace_id_namespace = ?
             ORDER BY row_id",
        )
        .bind(&workspace_id.value)
        .bind(&workspace_id.namespace)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(ProviderState::try_from).collect()
    }

    pub async fn mark_provider_state_stale(
        &self,
        provider_state_id: &OpaqueId<ProviderStateRef>,
        reason: &str,
    ) -> Result<ProviderState, GraphStoreError> {
        OpaqueId::<ProviderStateRef>::new(
            provider_state_id.value.clone(),
            provider_state_id.namespace.clone(),
        )?;
        validate_stale_reason(reason)?;
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await
            .map_err(GraphStoreError::from)?;
        let mut transaction = self.pool.begin().await?;

        let current = match select_provider_state(&mut transaction, provider_state_id).await? {
            Some(current) => current,
            None => {
                rollback(transaction).await?;
                return Err(GraphStoreError::UnknownRef);
            }
        };

        let updated_at = next_contract_timestamp(&current.meta.updated_at)?;
        let actor = ActorRef {
            value: format!("provider-state-stale:{reason}"),
        };
        let update_result = sqlx::query(
            "UPDATE provider_states
             SET freshness = ?,
                 confidence = ?,
                 updated_at = ?,
                 actor = ?
             WHERE provider_state_id_value = ?
               AND provider_state_id_namespace = ?",
        )
        .bind(ProviderFreshness::Stale)
        .bind(ProviderConfidence::Low)
        .bind(&updated_at)
        .bind(JsonField(actor))
        .bind(&provider_state_id.value)
        .bind(&provider_state_id.namespace)
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

        let updated = select_provider_state(&mut transaction, provider_state_id)
            .await?
            .ok_or(GraphStoreError::UnknownRef)?;
        transaction.commit().await.map_err(GraphStoreError::from)?;
        Ok(updated)
    }
}

impl GraphStoreRepo<ProviderState> for ProviderStateRepo {
    type IdMarker = ProviderStateRef;

    fn insert<'a>(&'a self, record: ProviderState) -> GraphStoreRepoFuture<'a, ProviderState> {
        Box::pin(async move { self.insert_provider_state(record).await })
    }

    fn get<'a>(
        &'a self,
        id: &'a OpaqueId<Self::IdMarker>,
    ) -> GraphStoreRepoFuture<'a, Option<ProviderState>> {
        Box::pin(async move { self.get_provider_state(id).await })
    }

    fn list_by_workspace<'a>(
        &'a self,
        workspace_id: &'a OpaqueId<GraphWorkspaceRef>,
    ) -> GraphStoreRepoFuture<'a, Vec<ProviderState>> {
        Box::pin(async move {
            let workspace_id = OpaqueId::<GraphWorkspace>::new(
                workspace_id.value.clone(),
                workspace_id.namespace.clone(),
            )?;
            self.list_provider_states_by_workspace(&workspace_id).await
        })
    }
}

impl TryFrom<ProviderStateRow> for ProviderState {
    type Error = GraphStoreError;

    fn try_from(row: ProviderStateRow) -> Result<Self, Self::Error> {
        let record = ProviderState {
            provider_state_id: OpaqueId::<ProviderStateRef>::new(
                row.provider_state_id_value,
                row.provider_state_id_namespace,
            )?,
            workspace_id: OpaqueId::<GraphWorkspace>::new(
                row.workspace_id_value,
                row.workspace_id_namespace,
            )?,
            provider: row.provider,
            cli: row.cli,
            account_ref: row.account_ref,
            auth_state: row.auth_state,
            billing_state: row.billing_state,
            quota_state: row.quota_state,
            network_state: row.network_state,
            runtime_state: row.runtime_state,
            sandbox_constraints: row.sandbox_constraints,
            store_locations_checked: row.store_locations_checked,
            secret_material_stored: row.secret_material_stored,
            freshness: row.freshness,
            confidence: row.confidence,
            last_probe_at: row.last_probe_at,
            meta: validate_record_meta(RecordMeta {
                created_at: row.created_at,
                updated_at: row.updated_at,
                actor: row.actor.0,
                policy_version: row.record_policy_version,
            })?,
        };

        validate_provider_state(record)
    }
}

const SELECT_PROVIDER_STATE: &str = "SELECT provider_state_id_value, provider_state_id_namespace,
        workspace_id_value, workspace_id_namespace,
        provider, cli, account_ref, auth_state, billing_state, quota_state,
        network_state, runtime_state, sandbox_constraints, store_locations_checked,
        secret_material_stored, freshness, confidence, last_probe_at,
        created_at, updated_at, actor, record_policy_version
 FROM provider_states
 WHERE provider_state_id_value = ? AND provider_state_id_namespace = ?";

async fn select_provider_state(
    transaction: &mut sqlx::Transaction<'_, Sqlite>,
    id: &OpaqueId<ProviderStateRef>,
) -> Result<Option<ProviderState>, GraphStoreError> {
    let row = sqlx::query_as::<_, ProviderStateRow>(SELECT_PROVIDER_STATE)
        .bind(&id.value)
        .bind(&id.namespace)
        .fetch_optional(&mut **transaction)
        .await?;

    row.map(ProviderState::try_from).transpose()
}

fn validate_provider_state(record: ProviderState) -> Result<ProviderState, GraphStoreError> {
    OpaqueId::<ProviderStateRef>::new(
        record.provider_state_id.value.clone(),
        record.provider_state_id.namespace.clone(),
    )?;
    OpaqueId::<GraphWorkspace>::new(
        record.workspace_id.value.clone(),
        record.workspace_id.namespace.clone(),
    )?;
    validate_record_meta(record.meta.clone())?;
    validate_contract_timestamp(&record.last_probe_at, &record.meta)?;

    if record.account_ref.trim().is_empty()
        || record
            .store_locations_checked
            .0
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    if record.secret_material_stored {
        return Err(GraphStoreError::InvariantViolation);
    }

    if contains_credential_like_string(&record.account_ref)
        || value_contains_credential_like_string(&record.sandbox_constraints.0)
        || record
            .store_locations_checked
            .0
            .iter()
            .any(|value| contains_credential_like_string(value))
    {
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

fn value_contains_credential_like_string(value: &Value) -> bool {
    match value {
        Value::String(value) => contains_credential_like_string(value),
        Value::Array(values) => values.iter().any(value_contains_credential_like_string),
        Value::Object(values) => values.values().any(value_contains_credential_like_string),
        Value::Null | Value::Bool(_) | Value::Number(_) => false,
    }
}

fn contains_credential_like_string(value: &str) -> bool {
    if value.starts_with("Bearer ") {
        return true;
    }
    value
        .split(|character: char| character.is_whitespace() || matches!(character, '"' | '\''))
        .any(is_credential_like_token)
}

fn is_credential_like_token(token: &str) -> bool {
    let token = token.trim_matches(|character: char| {
        matches!(
            character,
            ',' | ';' | ')' | '(' | '[' | ']' | '{' | '}' | '"' | '\''
        )
    });
    if token.starts_with("sk-") || token.starts_with("Bearer ") {
        return true;
    }
    if is_aws_access_key_shape(token) || is_jwt_shape(token) {
        return true;
    }
    is_generic_secret_token_shape(token)
}

fn is_aws_access_key_shape(token: &str) -> bool {
    token.len() == 20
        && (token.starts_with("AKIA") || token.starts_with("ASIA"))
        && token
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
}

fn is_jwt_shape(token: &str) -> bool {
    let parts = token.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts[0].starts_with("eyJ")
        && parts
            .iter()
            .all(|part| part.len() >= 10 && part.bytes().all(is_base64_url_byte))
}

fn is_base64_url_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')
}

fn is_generic_secret_token_shape(token: &str) -> bool {
    token.len() >= 48
        && token.bytes().all(is_base64_url_byte)
        && token.bytes().any(|byte| byte.is_ascii_digit())
        && token.bytes().any(|byte| byte.is_ascii_uppercase())
        && token.bytes().any(|byte| byte.is_ascii_lowercase())
}

fn validate_stale_reason(reason: &str) -> Result<(), GraphStoreError> {
    if reason.is_empty()
        || reason.len() > 80
        || reason.bytes().any(|byte| {
            !(byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'_' | b'-' | b'.' | b':'))
        })
        || reason.contains("..")
    {
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
            if message.contains("UNIQUE constraint failed: provider_states.provider_state_id_value")
            {
                GraphStoreError::DuplicateId
            } else if message.contains("FOREIGN KEY constraint failed") {
                GraphStoreError::UnknownRef
            } else if message.contains("CHECK constraint failed: provider")
                || message.contains("CHECK constraint failed: cli")
                || message.contains("CHECK constraint failed: auth_state")
                || message.contains("CHECK constraint failed: billing_state")
                || message.contains("CHECK constraint failed: quota_state")
                || message.contains("CHECK constraint failed: network_state")
                || message.contains("CHECK constraint failed: runtime_state")
                || message.contains("CHECK constraint failed: freshness")
                || message.contains("CHECK constraint failed: confidence")
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

impl ProviderKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Anthropic => "anthropic",
            Self::Openai => "openai",
            Self::Google => "google",
            Self::LocalRuntime => "local_runtime",
            Self::OpenaiCompatible => "openai_compatible",
            Self::Other => "other",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "anthropic" => Ok(Self::Anthropic),
            "openai" => Ok(Self::Openai),
            "google" => Ok(Self::Google),
            "local_runtime" => Ok(Self::LocalRuntime),
            "openai_compatible" => Ok(Self::OpenaiCompatible),
            "other" => Ok(Self::Other),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ProviderCli {
    fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Codex => "codex",
            Self::Opencode => "opencode",
            Self::AgentRunner => "agent_runner",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "claude" => Ok(Self::Claude),
            "codex" => Ok(Self::Codex),
            "opencode" => Ok(Self::Opencode),
            "agent_runner" => Ok(Self::AgentRunner),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl AuthState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
            Self::Expired => "expired",
            Self::Invalid => "invalid",
            Self::Unknown => "unknown",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "present" => Ok(Self::Present),
            "missing" => Ok(Self::Missing),
            "expired" => Ok(Self::Expired),
            "invalid" => Ok(Self::Invalid),
            "unknown" => Ok(Self::Unknown),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl BillingState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::NearLimit => "near_limit",
            Self::OverLimit => "over_limit",
            Self::PaymentRequired => "payment_required",
            Self::Unknown => "unknown",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "healthy" => Ok(Self::Healthy),
            "near_limit" => Ok(Self::NearLimit),
            "over_limit" => Ok(Self::OverLimit),
            "payment_required" => Ok(Self::PaymentRequired),
            "unknown" => Ok(Self::Unknown),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ProviderQuotaState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RateLimited => "rate_limited",
            Self::Exhausted => "exhausted",
            Self::Unknown => "unknown",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "available" => Ok(Self::Available),
            "rate_limited" => Ok(Self::RateLimited),
            "exhausted" => Ok(Self::Exhausted),
            "unknown" => Ok(Self::Unknown),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl NetworkState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::BlockedBySandbox => "blocked_by_sandbox",
            Self::BlockedByHost => "blocked_by_host",
            Self::Degraded => "degraded",
            Self::Unknown => "unknown",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "available" => Ok(Self::Available),
            "blocked_by_sandbox" => Ok(Self::BlockedBySandbox),
            "blocked_by_host" => Ok(Self::BlockedByHost),
            "degraded" => Ok(Self::Degraded),
            "unknown" => Ok(Self::Unknown),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ProviderRuntimeState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Missing => "missing",
            Self::WrongVersion => "wrong_version",
            Self::Unreachable => "unreachable",
            Self::NotApplicable => "not_applicable",
            Self::Unknown => "unknown",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "installed" => Ok(Self::Installed),
            "missing" => Ok(Self::Missing),
            "wrong_version" => Ok(Self::WrongVersion),
            "unreachable" => Ok(Self::Unreachable),
            "not_applicable" => Ok(Self::NotApplicable),
            "unknown" => Ok(Self::Unknown),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ProviderFreshness {
    fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::ProbeFailed => "probe_failed",
            Self::Manual => "manual",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "fresh" => Ok(Self::Fresh),
            "stale" => Ok(Self::Stale),
            "probe_failed" => Ok(Self::ProbeFailed),
            "manual" => Ok(Self::Manual),
            _ => Err(GraphStoreError::InvalidEnum),
        }
    }
}

impl ProviderConfidence {
    fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    fn parse(value: &str) -> Result<Self, GraphStoreError> {
        match value {
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
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

impl_sqlite_text_enum!(ProviderKind);
impl_sqlite_text_enum!(ProviderCli);
impl_sqlite_text_enum!(AuthState);
impl_sqlite_text_enum!(BillingState);
impl_sqlite_text_enum!(ProviderQuotaState);
impl_sqlite_text_enum!(NetworkState);
impl_sqlite_text_enum!(ProviderRuntimeState);
impl_sqlite_text_enum!(ProviderFreshness);
impl_sqlite_text_enum!(ProviderConfidence);
