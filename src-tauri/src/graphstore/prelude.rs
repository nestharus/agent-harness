use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::{Database, Sqlite};

pub type Timestamp = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActorRef {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordMeta {
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub actor: ActorRef,
    pub policy_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct OpaqueId<T> {
    pub value: String,
    pub namespace: String,
    #[serde(skip)]
    marker: PhantomData<fn() -> T>,
}

impl<T> PartialEq for OpaqueId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.namespace == other.namespace
    }
}

impl<T> Eq for OpaqueId<T> {}

impl<T> Hash for OpaqueId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.namespace.hash(state);
    }
}

impl<T> OpaqueId<T> {
    pub fn new(
        value: impl Into<String>,
        namespace: impl Into<String>,
    ) -> Result<Self, GraphStoreError> {
        let value = value.into();
        let namespace = namespace.into();
        validate_opaque_id_parts(&value, &namespace)?;

        Ok(Self {
            value,
            namespace,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonField<T>(pub T);

impl<T> Type<Sqlite> for JsonField<T> {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <String as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &<Sqlite as Database>::TypeInfo) -> bool {
        <String as Type<Sqlite>>::compatible(ty)
    }
}

impl<'q, T> Encode<'q, Sqlite> for JsonField<T>
where
    T: Serialize,
{
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        let encoded = serde_json::to_string(&self.0)?;
        <String as Encode<Sqlite>>::encode(encoded, buf)
    }
}

impl<'r, T> Decode<'r, Sqlite> for JsonField<T>
where
    T: DeserializeOwned,
{
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let encoded = <String as Decode<Sqlite>>::decode(value)?;
        let decoded = serde_json::from_str(&encoded)?;
        Ok(Self(decoded))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphStoreError {
    UnknownRef,
    DuplicateId,
    InvalidEnum,
    InvalidTransition,
    InvariantViolation,
    OptimisticConflict,
    SqlxFailure,
}

impl GraphStoreError {
    pub fn code(self) -> &'static str {
        match self {
            Self::UnknownRef => "unknown_ref",
            Self::DuplicateId => "duplicate_id",
            Self::InvalidEnum => "invalid_enum",
            Self::InvalidTransition => "invalid_transition",
            Self::InvariantViolation => "invariant_violation",
            Self::OptimisticConflict => "optimistic_conflict",
            Self::SqlxFailure => "sqlx_failure",
        }
    }
}

impl std::fmt::Display for GraphStoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for GraphStoreError {}

impl From<sqlx::Error> for GraphStoreError {
    fn from(_: sqlx::Error) -> Self {
        Self::SqlxFailure
    }
}

pub fn validate_record_meta(meta: RecordMeta) -> Result<RecordMeta, GraphStoreError> {
    let created_at =
        parse_contract_timestamp(&meta.created_at).ok_or(GraphStoreError::InvariantViolation)?;
    let updated_at =
        parse_contract_timestamp(&meta.updated_at).ok_or(GraphStoreError::InvariantViolation)?;

    if updated_at < created_at
        || meta.actor.value.trim().is_empty()
        || meta.policy_version.trim().is_empty()
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(meta)
}

fn validate_opaque_id_parts(value: &str, namespace: &str) -> Result<(), GraphStoreError> {
    if value.trim().is_empty() || namespace.trim().is_empty() {
        return Err(GraphStoreError::InvariantViolation);
    }

    if has_content_hash_prefix(value)
        || has_forbidden_id_char(value)
        || has_forbidden_id_char(namespace)
        || has_parent_path_encoding(value)
        || has_parent_path_encoding(namespace)
    {
        return Err(GraphStoreError::InvariantViolation);
    }

    Ok(())
}

fn has_content_hash_prefix(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    ["sha256:", "sha512:", "blake3:", "md5:"]
        .iter()
        .any(|prefix| value.starts_with(prefix))
}

fn has_forbidden_id_char(value: &str) -> bool {
    value.contains('/') || value.contains('\\') || value.contains(':')
}

fn has_parent_path_encoding(value: &str) -> bool {
    matches!(value, "." | "..")
        || value.contains("../")
        || value.contains("..\\")
        || value.contains('/')
        || value.contains('\\')
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TimestampParts {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

fn parse_contract_timestamp(value: &str) -> Option<TimestampParts> {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return None;
    }

    let year = parse_digits(value, 0, 4)? as u16;
    let month = parse_digits(value, 5, 7)? as u8;
    let day = parse_digits(value, 8, 10)? as u8;
    let hour = parse_digits(value, 11, 13)? as u8;
    let minute = parse_digits(value, 14, 16)? as u8;
    let second = parse_digits(value, 17, 19)? as u8;

    if !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }

    Some(TimestampParts {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}

fn parse_digits(value: &str, start: usize, end: usize) -> Option<u32> {
    let slice = value.get(start..end)?;
    if !slice.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    slice.parse().ok()
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
