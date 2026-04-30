# WU-0B-04 — Shortcut Gate

Reviewer: claude-opus
Branch: impl-wu-0b-04

## Verdict

LOW. Validation is real per-field (not a stub), the transaction wrapper is a true `pool.begin()` / `commit` / `rollback`, the recursive migration-discovery patch is a one-helper addition that preserves all WU-0B-01 invariants (a new test pins cross-subdirectory duplicate-version rejection), JSON columns flow through `JsonField<T>` rather than ad-hoc strings, and there are no `unwrap()` calls or TODOs in production code.

## Validation is real, not stubbed

- `validate_policy_set` (`policyset.rs:261-282`) re-runs `OpaqueId::<PolicySet>::new` (which calls into the WU-0B-02 `validate_opaque_id_parts`), `validate_record_meta`, and `validate_policy_version` for each of the 10 policy-version fields with the field-specific expected token (`SUMMARY_CONTRACT_POLICY` … `PROVIDER_POLICY`, `policyset.rs:10-19`). Every field-token mismatch returns `InvariantViolation`.
- `validate_policy_version` (`policyset.rs:284-307`) covers the four documented rejection modes:
  - `empty`: `value.policy.trim().is_empty() || value.version.trim().is_empty()`.
  - `unknown`: `value.policy != expected_policy`.
  - `malformed`: `parse_policy_semver(&value.version)?` rejects missing `v` prefix, missing parts, extra parts, non-digit content, and out-of-range integers (`policyset.rs:309-331`).
  - `non_monotonic`: `previous_version` is parsed and the validator returns `InvariantViolation` if `version <= previous`. Whitespace-only `previous_version` is also rejected (`policyset.rs:296-299`).
- The contract test enumerates all 10 fields × 4 modes (40 cases) and confirms every case returns `InvariantViolation` (`wu_0b_04_policyset_contract.rs:296-318`).
- Validation runs both on insert (`policyset.rs:80-89`, before any DB call) and on read (`PolicySet::try_from(PolicySetRow)`, `policyset.rs:229-259`). The read-path defense protects against forward-compatibility drift if a future migration relaxes a CHECK constraint.

## Transaction wrapper uses real pool.begin / commit / rollback

- `insert_policy_set` (`policyset.rs:77-153`):
  1. `let mut transaction = self.pool.begin().await?;` — opens a real SQLite transaction.
  2. Validation runs against the in-memory record. On failure, `transaction.rollback().await` runs before returning.
  3. The duplicate-id `SELECT COUNT(*)` runs `fetch_one(&mut *transaction)` so the read sees the same connection. On non-zero, rollback then return `DuplicateId`.
  4. The `INSERT` runs `execute(&mut *transaction)`. On `Ok`, `transaction.commit()`. On `Err`, `transaction.rollback()` then map to `DuplicateId` (UNIQUE constraint match) or `SqlxFailure`.
- The contract test asserts row count is unchanged after a validation failure (`wu_0b_04_policyset_contract.rs:285-294`), confirming no partial write reaches durable state.

## Recursive walk preserves WU-0B-01 invariants

- The patch is a single helper extraction: `load_migration_files` continues to sort and de-duplicate the assembled `Vec<MigrationFile>`; only the discovery walk has been replaced (`migrations.rs:78-116`). The post-collection logic — sort by `(version, name)`, then `BTreeSet::insert`-based duplicate detection returning `OutOfOrderVersion` — operates on the full recursive output, so cross-subdirectory duplicates fail discovery before any apply call.
- `recursive_migration_walk_rejects_duplicate_versions_across_subdirectories` (`graphstore_migrations_contract.rs:281-307`) is the new pin: it writes `0001_schema_versions.sql` at the root and `0b/0001_duplicate_schema_versions.sql` in a nested directory, asserts `run_migrations` returns `OutOfOrderVersion`, and asserts `schema_versions` is *not* created (no partial apply).
- All five pre-existing WU-0B-01 invariants still pass after the patch: idempotent rerun (`graphstore_migrations_contract.rs:196-217`), checksum mismatch (`:219-251`), duplicate version rejection (`:253-278`), out-of-order lower version (`:309-342`), version-already-applied-differently (`:344-378`). Verified locally: `cargo test --test graphstore_migrations_contract` reports 12/12 passing.

## JSON columns use JsonField<T>, not blob string

- `PolicySet` declares each policy-version field as `JsonField<PolicyVersionRef>` (`policyset.rs:32-44`). The struct is `serde(deny_unknown_fields)` and `JsonField` is `serde(transparent)` (`prelude.rs:70-72`).
- The insert binds `record.summary_contract_version` etc. directly (`policyset.rs:119-128`); the WU-0B-02 `Encode<Sqlite> for JsonField<T>` impl (`prelude.rs:84-95`) serializes through `serde_json::to_string`. Decoding through `Decode<Sqlite> for JsonField<T>` (`prelude.rs:97-106`) calls `serde_json::from_str`.
- `PolicySetRow` (`policyset.rs:52-70`) types every JSON column as `JsonField<PolicyVersionRef>` and the actor as `JsonField<ActorRef>`, so `sqlx::FromRow` flows through the codec rather than raw `String`.

## No `unwrap()` in production code, no TODOs

- `grep -nE "TODO|FIXME|todo!\(\)|unimplemented!\(\)|panic!\(|unwrap\(\)"` against `src-tauri/src/graphstore/policyset.rs` and `src-tauri/src/graphstore/migrations.rs` returns no matches. The contract test uses `unwrap_or_else` and `expect` for fixture-loading panics, which is appropriate for a test rig.
- `is_i64::try_from(...).unwrap_or(i64::MAX)` (`migrations.rs:197`) is `unwrap_or`, not `unwrap`, and is the same defensive pattern WU-0B-01 already shipped — not new.

## list_by_workspace surface

- `list_policy_sets_by_workspace` (`policyset.rs:179-204`) is documented as a global/workspace no-op until `GraphWorkspace` exists; it filters on `policy_set_id_namespace = workspace_id.value`, validates the supplied `OpaqueId<GraphWorkspaceRef>` through `OpaqueId::new`, and returns the (possibly empty) row vector. The append-only contract test exercises the populated case (`wu_0b_04_policyset_contract.rs:347-355`), and the proposal explicitly names this as the workspace-substitute behavior (`proposals/0b-04-wu-0b-04.md:79-81`).

## Findings

- `WU-0B-04-SHORTCUT-F01` (NIT). The duplicate-id pre-check (`policyset.rs:91-105`) issues a `SELECT COUNT(*)` before the `INSERT`. The terminal `UNIQUE` constraint plus `is_sqlite_unique_violation` recovery (`policyset.rs:107-152, 333-340`) already guarantees `DuplicateId`; the pre-check is redundant defense-in-depth that costs an extra round trip per insert. Acceptable for inert governance writes; flagging as a future simplification.
- `WU-0B-04-SHORTCUT-F02` (NIT). `is_sqlite_unique_violation` matches the SQLite error message substring `"UNIQUE constraint failed: policy_sets.policy_set_id_value"`. Message-string matching is brittle across SQLite/sqlx version bumps. The `dependencies on earlier slices` constrain this to SQLite, so the risk is bounded; consider checking the constraint code (`error.code()`) instead in a follow-up. Not actionable now.
- `WU-0B-04-SHORTCUT-F03` (NIT). `MigrationError` from `collect_migration_files` collapses every `fs::read_dir` and `entry` failure into `SqlxFailure` (`migrations.rs:102-103`). This is the same mapping WU-0B-01 used; it remains accurate-but-coarse for filesystem errors that have nothing to do with sqlx. Inherited from WU-0B-01, not introduced here.
