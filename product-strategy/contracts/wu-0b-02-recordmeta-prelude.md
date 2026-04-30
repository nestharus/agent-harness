# WU-0B-02 RecordMeta Prelude Contract

## Surface

This contract defines the Phase 0B shared GraphStore prelude only:

- `RecordMeta { created_at: Timestamp, updated_at: Timestamp, actor: ActorRef, policy_version: String }`
- `ActorRef { value: String }`
- `OpaqueId<T> { value: String, namespace: String }`
- `JsonField<T>` as a SQLite `TEXT` serde/sqlx codec
- `GraphStoreError`
- `validate_record_meta(meta) -> Result<RecordMeta, GraphStoreError>`

It defines no Tauri command, no value-slice service operation, no repository CRUD, and no durable domain table or migration.

## RecordMeta

`Timestamp` is a normalized UTC RFC3339 string ending in `Z`, using the shape `YYYY-MM-DDTHH:MM:SSZ`. `validate_record_meta` preserves valid caller strings and returns the same `RecordMeta`.

Invalid metadata returns `GraphStoreError::InvariantViolation` before any insert or other write:

- `updated_at < created_at`
- `actor.value` empty or whitespace-only
- `policy_version` empty or whitespace-only
- malformed timestamp strings

Fixtures:

- `record-meta-canonical.json`
- `record-meta-invalid-updated-before-created.json`
- `record-meta-invalid-empty-actor.json`
- `record-meta-invalid-empty-policy-version.json`

## OpaqueId

`OpaqueId<T>::new(value, namespace)` validates both fields. The phantom `T` is compile-time only and is not serialized.

The exact rejection set is:

- empty or whitespace-only `namespace`
- empty or whitespace-only `value`
- `/`, `\`, or `:` in `value` or `namespace`
- known content-hash prefixes in `value`: `sha256:`, `sha512:`, `blake3:`, or `md5:`
- parent path encodings such as `.`, `..`, `parent/child`, `../child`, or `parent\child`

Fixtures:

- `opaque-id-valid.json`
- `opaque-id-invalid-path-separator.json`
- `opaque-id-invalid-content-hash-prefix.json`
- `opaque-id-invalid-empty-namespace.json`
- `opaque-id-invalid-parent-path.json`

## JsonField

`JsonField<T>` serializes `T` with `serde_json::to_string` and decodes with `serde_json::from_str`. It implements SQLx SQLite `Type`, `Encode`, and `Decode` for `TEXT`, preserving JSON values as JSON rather than display strings.

Round-trip fixture cases:

- array
- map
- nested ref
- empty array
- null optional
- enum payload

Fixture: `jsonfield-round-trips.json`

## GraphStoreError

`GraphStoreError` has exactly seven stable variants and machine-readable codes:

| Variant | code() |
| --- | --- |
| `UnknownRef` | `unknown_ref` |
| `DuplicateId` | `duplicate_id` |
| `InvalidEnum` | `invalid_enum` |
| `InvalidTransition` | `invalid_transition` |
| `InvariantViolation` | `invariant_violation` |
| `OptimisticConflict` | `optimistic_conflict` |
| `SqlxFailure` | `sqlx_failure` |

Fixtures:

- `error-unknown-ref.json`
- `error-duplicate-id.json`
- `error-invalid-enum.json`
- `error-invalid-transition.json`
- `error-invariant-violation.json`
- `error-optimistic-conflict.json`
- `error-sqlx-failure.json`
- `graphstore-errors.json`

## Test Intent Handoff

- Validate canonical `RecordMeta` unchanged and invalid metadata as `InvariantViolation`.
- Reject each invalid opaque-ID pattern as `InvariantViolation`.
- Match every error variant structurally and assert exact `code()`.
- Round-trip every JSON fixture through a real SQLite `TEXT` column using `JsonField<T>`.
- Assert timestamp ordering is rejected before any insert attempt.
- Assert no durable domain table appears and `phase_0a_scaffold_commands()` remains `["subscribe_workspace_events"]`.
