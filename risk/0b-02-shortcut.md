# WU-0B-02 Shortcut / Placeholder Gate

**Severity:** LOW

## Verdict

The codec is real: `JsonField<T>` implements `sqlx::Type<Sqlite>`, `Encode<'q, Sqlite>`, and `Decode<'r, Sqlite>` directly, the contract test inserts and selects through a real `sqlite::memory:` pool using `bind(JsonField(...))` and `row.try_get::<JsonField<Value>>(...)` — both paths force the SQLx codec to fire end-to-end (encode → SQLite TEXT → decode), not just `serde_json::to_string` / `from_str` against a string. Every `GraphStoreError::code()` is a stable snake_case `&'static str` from a total `match`. `validate_record_meta` does a real timestamp parse and a structured order comparison (`updated_at < created_at` over typed `TimestampParts`), not a string-compare tautology. All four OpaqueId rejection paths hit real string-validation logic with real invalid strings. Zero `unwrap()` on `Result`, zero `panic!`, `todo!`, `unimplemented!`, `TODO`, `FIXME`, or placeholder markers in the new source. Two nits, both informational.

## Per-Item Findings

### WU-0B-02-SHORTCUT-F01 — JsonField round-trip is through a REAL sqlx Encode + Decode cycle, not serde::to_string/from_str (PASS)

The codec impls themselves (`src-tauri/src/graphstore/prelude.rs:74-106`) are wired through SQLx's existing `String` text codec:

```rust
impl<'q, T> Encode<'q, Sqlite> for JsonField<T> where T: Serialize {
    fn encode_by_ref(&self, buf: &mut <Sqlite as Database>::ArgumentBuffer<'q>)
        -> Result<IsNull, BoxDynError> {
        let encoded = serde_json::to_string(&self.0)?;
        <String as Encode<Sqlite>>::encode(encoded, buf)   // ← real sqlx String encoder
    }
}
impl<'r, T> Decode<'r, Sqlite> for JsonField<T> where T: DeserializeOwned {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let encoded = <String as Decode<Sqlite>>::decode(value)?;   // ← real sqlx String decoder
        let decoded = serde_json::from_str(&encoded)?;
        Ok(Self(decoded))
    }
}
```

The contract test `jsonfield_round_trips_documented_json_shapes_through_sqlite_text` (`src-tauri/tests/graphstore_prelude_contract.rs:162-208`) exercises this end-to-end against an actual SQLite pool:

1. `SqlitePoolOptions::new().connect("sqlite::memory:").await` — real in-memory pool.
2. `sqlx::query("CREATE TABLE jsonfield_contract (name TEXT PRIMARY KEY, body TEXT NOT NULL)").execute(&pool)` — real table.
3. For each of the six fixture cases: `sqlx::query("INSERT...").bind(&case.name).bind(JsonField(case.value.clone())).execute(&pool).await` — `.bind(JsonField(...))` forces SQLx to invoke the `JsonField::Encode` impl above. This is **not** equivalent to `bind(serde_json::to_string(&value))` — the `Encode` path is what's exercised.
4. For each case: `let decoded: JsonField<Value> = row.try_get("body")?` — `try_get` for the `JsonField<Value>` target type forces SQLx to invoke the `JsonField::Decode` impl above.
5. `assert_eq!(decoded.0, case.value)` — round-trip equality on the original `serde_json::Value`, so any lossy stringification (e.g. number→string, bool→"true") would fail the assertion.

Coverage of all six AC categories is fixture-named: `array`, `map`, `nested_ref`, `empty_array`, `null_optional`, `enum_payload`. Test asserts `fixture.cases.len() == 6` (a canary). The test additionally asserts `<JsonField<Value> as Type<Sqlite>>::compatible(&type_info)` (`graphstore_prelude_contract.rs:170-172`) so the `Type` impl is also exercised. PASS.

### WU-0B-02-SHORTCUT-F02 — Every `GraphStoreError::code()` is a stable snake_case `&'static str` from a total match (PASS)

`src-tauri/src/graphstore/prelude.rs:120-130`:

```rust
match self {
    Self::UnknownRef => "unknown_ref",
    Self::DuplicateId => "duplicate_id",
    Self::InvalidEnum => "invalid_enum",
    Self::InvalidTransition => "invalid_transition",
    Self::InvariantViolation => "invariant_violation",
    Self::OptimisticConflict => "optimistic_conflict",
    Self::SqlxFailure => "sqlx_failure",
}
```

Each arm returns a unique string-literal that is stable at the source level (no formatting, no concatenation, no `Display::to_string` round-trip). The match is total — adding a variant without a `code()` arm is a compile error, so the variant ↔ code mapping cannot drift silently. The contract test asserts each code via fixture comparison (`graphstore_prelude_contract.rs:131-159`). `Display` is delegated to `code()` (`prelude.rs:133-137`) so the user-visible string is the same as the machine-readable code. PASS.

### WU-0B-02-SHORTCUT-F03 — `validate_record_meta` does a REAL timestamp ordering check, not a tautology (PASS)

`prelude.rs:147-161`:

```rust
let created_at = parse_contract_timestamp(&meta.created_at).ok_or(GraphStoreError::InvariantViolation)?;
let updated_at = parse_contract_timestamp(&meta.updated_at).ok_or(GraphStoreError::InvariantViolation)?;
if updated_at < created_at
    || meta.actor.value.trim().is_empty()
    || meta.policy_version.trim().is_empty()
{
    return Err(GraphStoreError::InvariantViolation);
}
Ok(meta)
```

`parse_contract_timestamp` (`prelude.rs:209-247`) parses the exact `YYYY-MM-DDTHH:MM:SSZ` shape into a `TimestampParts { year, month, day, hour, minute, second }` and validates month/day/hour/minute/second ranges (including `days_in_month` and `is_leap_year`). `TimestampParts` derives `PartialOrd`/`Ord` so `updated_at < created_at` compares structured fields lexicographically year-first — this is **not** the same as `meta.updated_at < meta.created_at` on the raw strings (which would only happen to be correct for normalized `Z`-suffixed strings of equal length, but would break under any tz-suffix variation). The fixture `record-meta-invalid-updated-before-created.json` (`created_at: "2026-04-30T10:15:30Z"`, `updated_at: "2026-04-30T10:00:00Z"`) is rejected because the typed comparison evaluates `(year=2026, month=4, day=30, hour=10, minute=0, second=0) < (year=2026, month=4, day=30, hour=10, minute=15, second=30)`. PASS — not a tautology.

`actor.value.trim().is_empty()` and `policy_version.trim().is_empty()` are real checks: the `record-meta-invalid-empty-actor.json` fixture uses `"   "` (whitespace), exercising the `trim()` arm; `record-meta-invalid-empty-policy-version.json` uses `""`. Both are exercised in `record_meta_validation_preserves_canonical_and_rejects_invalid_inputs`.

### WU-0B-02-SHORTCUT-F04 — All four OpaqueId rejection paths fire on REAL invalid strings (PASS)

Each rejection branch is hit by a fixture whose string actually triggers the corresponding guard:

| Fixture string | Guard fired |
|---|---|
| `value: "node/contract"` | `has_forbidden_id_char` — `value.contains('/')` returns `true` (`prelude.rs:188`) |
| `value: "sha256:abcdef"` | `has_content_hash_prefix` — `value.starts_with("sha256:")` returns `true` (`prelude.rs:184`) |
| `namespace: "   "` | `value.trim().is_empty() || namespace.trim().is_empty()` — `"   ".trim().is_empty()` returns `true` (`prelude.rs:164`) |
| `value: "parent/child"` | `has_parent_path_encoding` — `value.contains('/')` returns `true` (`prelude.rs:195`) (and `has_forbidden_id_char` would also reject it, so the OR-chain is redundant for this string but each guard is independently reachable through the path-separator and parent-path fixtures) |

Each `OpaqueId::<ContractOpaqueMarker>::new(id.value, id.namespace)` call hits the real `validate_opaque_id_parts` function (`prelude.rs:163-178`); there is no stub. The validation is total: every error path returns `GraphStoreError::InvariantViolation` and the test asserts both the variant AND `code() == "invariant_violation"` via `assert_invariant_violation`. PASS.

### WU-0B-02-SHORTCUT-F05 — No `unwrap()` / `panic!` / `todo!` / `unimplemented!` in production paths (PASS)

`grep -nE 'unwrap\(|todo!|unimplemented!|panic!|TODO|FIXME'` against `src-tauri/src/graphstore/prelude.rs` returns zero matches. All fallible operations use `?` over `Result` or `Option::ok_or(GraphStoreError::InvariantViolation)`. The serde-encoding path (`encode_by_ref`) propagates `serde_json::Error` via `?` into `BoxDynError`. The serde-decoding path (`decode`) does the same. PASS.

### WU-0B-02-SHORTCUT-F06 — No TODO / FIXME / placeholder markers (PASS)

`grep -niE 'TODO|FIXME|placeholder|stub|XXX|HACK'` against `src-tauri/src/graphstore/prelude.rs`, `src-tauri/src/contracts/graphstore_prelude.rs`, and `src-tauri/tests/graphstore_prelude_contract.rs` returns zero matches.

### WU-0B-02-SHORTCUT-N01 — Nit: `From<sqlx::Error> for GraphStoreError` discards the cause (NIT)

`prelude.rs:141-145` collapses any `sqlx::Error` into the bare `SqlxFailure` discriminant, dropping the underlying cause string. The contract enumerates only seven unit variants and contains no `Sqlx { source: String }` shape, so widening the taxonomy is out of scope here. Same trade-off as WU-0B-01 (where `MigrationError::SqlxFailure` makes the same choice). Flag for future taxonomy evolution; no action required for WU-0B-02.

### WU-0B-02-SHORTCUT-N02 — Nit: deserializing `OpaqueId<T>` directly bypasses validation (NIT)

`OpaqueId<T>` derives `Deserialize`, so `serde_json::from_str::<OpaqueId<T>>(...)` constructs an `OpaqueId<T>` whose `value`/`namespace` were never run through `validate_opaque_id_parts`. The contract test acknowledges this: it deserializes the invalid fixtures into `OpaqueId<T>` only to extract the strings, then explicitly calls `OpaqueId::<T>::new(id.value, id.namespace)` to re-trigger validation (`graphstore_prelude_contract.rs:111-114`). Future code that deserializes `OpaqueId<T>` from disk/network will get unvalidated values. The proposal frames `OpaqueId<T>::new` as the validation entry point and the contract md only requires construction-time validation, so this is in-scope for the WU as written; later WUs that deserialize IDs from untrusted sources should re-validate, or the type should grow a custom `Deserialize` impl that calls `new`. NIT.

## Conclusion

Severity **LOW**. The JsonField codec is genuinely SQLx-wired and the round-trip test invokes it end-to-end through a real SQLite pool, not via serde-only string fakery. Every error variant has a stable snake_case code from a total match. Timestamp ordering is structural over typed parts, not a string tautology. All four OpaqueId rejection paths are exercised with real invalid strings against real validation logic. Two nits, neither of which crosses the MEDIUM threshold.
