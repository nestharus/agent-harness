# WU-0B-04 — Scope Gate

Reviewer: claude-opus
Branch: impl-wu-0b-04
Inputs: ticket `WU-0B-04.md`, `proposals/0b-04-wu-0b-04.md`, `product-strategy/contracts/wu-0b-04-policyset.md`, `src-tauri/migrations/0b/04_policyset.sql`, `src-tauri/src/graphstore/policyset.rs`, `src-tauri/src/contracts/policyset.rs`, `src-tauri/tests/wu_0b_04_policyset_contract.rs`, plus the bounded patches to `src-tauri/src/graphstore/migrations.rs`, `src-tauri/tests/graphstore_migrations_contract.rs`, `src-tauri/tests/graphstore_fixture_contract.rs`, `src-tauri/tests/scaffold_contract.rs`, and `product-strategy/contracts/fixtures/wu-0b-03/*.json`.

## Verdict

LOW. All seven acceptance criteria are exercised by the contract test, the schema and Rust surface match the contract verbatim, and the bounded migration-runner patch preserves WU-0B-01 invariants (idempotency, checksum mismatch, out-of-order, duplicate version) including across the new recursive subtree.

## Acceptance criteria coverage

- AC1 — Fresh WU-0A temp SQLite database plus Phase 0B migrations creates `policy_sets` with declared columns, FK, unique, JSON, indexes, and `RecordMeta` fields.
  - Exercised by `policy_sets_schema_contains_declared_columns_constraints_and_indexes` (`wu_0b_04_policyset_contract.rs:170-245`). Reads `sqlite_master` SQL, asserts every declared column substring (17 columns including the 10 JSON policy-version columns, RecordMeta columns, and `schema_version`), the `UNIQUE(policy_set_id_value, policy_set_id_namespace)` clause, the `json_valid(summary_contract_version)` CHECK, and the `FOREIGN KEY(schema_version) REFERENCES schema_versions(version)` clause. PRAGMA `foreign_key_list(policy_sets)` and `index_list(policy_sets)` confirm the FK target and the presence of `idx_policy_sets_policy_set_id`, `idx_policy_sets_namespace`, plus a unique autoindex.
  - Migration source `src-tauri/migrations/0b/04_policyset.sql:1-29` matches.
- AC2 — `insert_policy_set` followed by `get_policy_set` round-trips every declared scalar, enum, JSON, timestamp, and opaque ID field byte-equivalent.
  - `insert_then_get_round_trips_every_policyset_field_byte_equivalent` (`wu_0b_04_policyset_contract.rs:247-274`). Compares `serde_json::to_string(...)` of the inserted struct, the fetched struct, and the source fixture row. The fixture (`round-trip.json`) exercises all 10 policy-version JSON envelopes (each with `policy`, `version`, `previous_version`, `payload`), the `policy_set_id` opaque ID, the timestamps, and the `RecordMeta`. `JsonField<T>` is bound via the WU-0B-02 codec (`prelude.rs:74-106`), so JSON columns survive a real SQLite write/read cycle.
- AC3 — Repository methods run inside an explicit transaction and roll back all writes when validation fails.
  - `insert_policy_set` (`policyset.rs:77-153`) opens `pool.begin()`, validates first; on validation failure (or duplicate, or sqlx execute failure) calls `transaction.rollback()` before returning. `invalid_insert_rolls_back_and_leaves_row_count_unchanged` (`wu_0b_04_policyset_contract.rs:276-294`) asserts `policy_sets` row count before == after for an invalid insert (`summary_contract_version.version = "not-semver"`), and asserts the returned variant is `InvariantViolation`.
- AC4 — No operator-visible feature behavior, UI pane, agent launch, optimizer execution, provider probe, or recovery execution is enabled.
  - `policyset_wu_has_no_operator_visible_behavior` (`wu_0b_04_policyset_contract.rs:357-385`) asserts `phase_0a_scaffold_commands()` still equals `["subscribe_workspace_events"]`, asserts the only durable runtime tables are `["policy_sets", "schema_versions"]`, and asserts `policyset.rs` source contains none of `Command::new`, `std::process::Command`, `tokio::process`, `provider_probe`, `optimizer`, `recovery_execution`, `invoke_handler`.
- AC5 — Contract tests are written only from this WU contract.
  - `policyset_contract_tests_stay_inside_declared_wu_boundary` (`wu_0b_04_policyset_contract.rs:387-409`) reads its own source and asserts the harness imports are the closed set `graphstore_fixture`, `graphstore_prelude`, `policyset`, `graphstore::fixture::{GraphStoreRepo, GraphWorkspaceRef}`, `graphstore::policyset::PolicySetRepo`, `phase_0a_scaffold_commands`. No optimizer, agent, provider, or recovery imports.
- AC6 — Every policy-version field rejects empty, unknown, non-monotonic, or malformed versions with `GraphStoreError::InvariantViolation`.
  - `every_policy_version_field_rejects_empty_unknown_non_monotonic_and_malformed` (`wu_0b_04_policyset_contract.rs:296-318`) iterates all 10 fields × 4 modes (40 cases) and asserts each insert returns `InvariantViolation`. The validator (`policyset.rs:284-307`) checks `policy.trim().is_empty() || version.trim().is_empty()` (empty), `policy != expected_policy` (unknown), `parse_policy_semver` for `vMAJOR.MINOR.PATCH` (malformed), and `version <= previous_version` (non-monotonic). `parse_policy_semver` (`policyset.rs:309-331`) rejects missing `v` prefix, missing parts, extra parts, non-digit characters, and negative integers. The `previous_version: Some("")` whitespace case is also rejected (`policyset.rs:296-299`).
- AC7 — Updating governance creates a new PolicySet row; existing rows are immutable and remain addressable.
  - `governance_update_creates_new_row_and_existing_rows_remain_addressable` (`wu_0b_04_policyset_contract.rs:320-355`) inserts two rows from `append-only-governance.json` (different `policy_set_id` values, same namespace, monotonically increasing policy versions). Asserts `COUNT(*) == 2`, asserts both rows fetch back byte-equivalent, and asserts `list_by_workspace` returns 2 rows.
  - There is no UPDATE statement and no `update_policy_set` method on `PolicySetRepo`. `grep -n "UPDATE policy_sets\|update_policy_set\|upsert" src-tauri/src/graphstore/policyset.rs` returns no matches.

## Schema fields and constraints

- 10 JSON policy-version columns each with `TEXT NOT NULL CHECK (json_valid(...))`: `summary_contract_version`, `render_policy_version`, `identity_policy_version`, `privilege_policy_version`, `tool_protocol_policy_version`, `budget_policy_version`, `review_sampling_policy_version`, `recovery_policy_version`, `configuration_policy_version`, `provider_policy_version` (`04_policyset.sql:5-14`).
- Opaque ID columns: `policy_set_id_value`, `policy_set_id_namespace`. RecordMeta columns: `created_at`, `updated_at`, `actor` (with `json_valid` check), `record_policy_version`. `schema_version INTEGER NOT NULL DEFAULT 4 REFERENCES schema_versions(version)` (`04_policyset.sql:15-21`).
- `UNIQUE(policy_set_id_value, policy_set_id_namespace)` plus explicit `idx_policy_sets_policy_set_id` and namespace-list `idx_policy_sets_namespace` (`04_policyset.sql:20, 24-28`).

## Rust surface match

- `PolicySet { policy_set_id, summary_contract_version, render_policy_version, identity_policy_version, privilege_policy_version, tool_protocol_policy_version, budget_policy_version, review_sampling_policy_version, recovery_policy_version, configuration_policy_version, provider_policy_version, meta }` (`policyset.rs:30-45`) matches the ticket Contract field verbatim and the `wu-0b-04-policyset.md:15-28` declaration.
- `PolicyVersionRef { policy, version, previous_version, payload }` (`policyset.rs:21-28`) is JSON-storable and matches the contract.
- `PolicySetRepo::{new, insert_policy_set, get_policy_set, list_policy_sets_by_workspace}` plus the `GraphStoreRepo<PolicySet>` trait alias (`policyset.rs:47-227`) match the contract.

## Recursive migration walk preserves WU-0B-01 invariants

- `collect_migration_files` (`migrations.rs:98-116`) recurses into subdirectories before adding `*.sql` files. `load_migration_files` (`migrations.rs:78-96`) sorts by `(version, migration_name)` then scans `BTreeSet<i64>` and returns `OutOfOrderVersion` on the first duplicate. The duplicate check operates on the full collected list, so version collisions across the recursive tree are caught before any apply call.
- `recursive_migration_walk_rejects_duplicate_versions_across_subdirectories` (`graphstore_migrations_contract.rs:281-307`) writes `0001_schema_versions.sql` at the root and a duplicate-version `0001_duplicate_schema_versions.sql` under `0b/`, runs `run_migrations`, asserts `OutOfOrderVersion`, and asserts `schema_versions` does not exist (i.e., nothing partially applied).
- All five prior WU-0B-01 invariant tests still pass: `fresh_temp_sqlite_applies_ordered_migrations_and_records_schema_versions`, `rerunning_identical_migrations_is_idempotent_and_does_not_rewrite_rows`, `changed_applied_migration_returns_checksum_mismatch_without_rewriting_rows`, `lower_missing_version_after_higher_applied_version_is_out_of_order`, `already_applied_version_with_different_name_is_rejected_exactly`.
- `shipped_phase_0b_migrations_emit_schema_versions_and_policy_sets` (`graphstore_migrations_contract.rs:560-581`) asserts the shipped `src-tauri/migrations` tree applies `[1, 4]` and produces exactly `["policy_sets", "schema_versions"]` durable tables.

## Append-only governance enforcement

- No `UPDATE` statement, no `update_policy_set` method, no upsert path (`grep` clean, `policyset.rs:1-341`).
- `insert_policy_set` performs an explicit `SELECT COUNT(*)` pre-check and returns `DuplicateId` on the existing-id path (`policyset.rs:91-105`). This is also defended by the `UNIQUE(policy_set_id_value, policy_set_id_namespace)` constraint and a `is_sqlite_unique_violation` recovery path that maps the SQLite UNIQUE violation back to `DuplicateId` (`policyset.rs:333-340`).
- The append-only test inserts two rows with distinct IDs and verifies both remain addressable; the duplicate path is implicitly covered by the `DuplicateId` mapping but not exercised by an explicit "same ID retry" test (see SCOPE-F01 below).

## Findings

- `WU-0B-04-SCOPE-F01` (NIT). The append-only contract test (`wu_0b_04_policyset_contract.rs:320-355`) verifies the *positive* shape — two rows with distinct IDs persist and remain addressable — but does not directly assert that `insert_policy_set` rejects a second insert with the *same* `policy_set_id` (i.e., that updating an existing ID via the repository surface returns `DuplicateId` rather than overwriting). The behavior is implemented (`policyset.rs:91-105` plus the UNIQUE-constraint recovery path), and the rejection-matrix test exercises `DuplicateId` indirectly because each invalid case uses a unique synthesized ID. Adding a single same-ID retry assertion would close this without expanding scope.
- `WU-0B-04-SCOPE-F02` (NIT). `product-strategy/contracts/fixtures/wu-0b-04/canonical-row.json` is declared in the contract (`wu-0b-04-policyset.md:92`) and proposal (`proposals/0b-04-wu-0b-04.md:113`) but no contract test reads it; only `round-trip.json`, `rejection-matrix.json`, `append-only-governance.json`, and `no-side-effects.json` are loaded. The intent is documentary, but a fixture without an asserting test cannot drift-check on its own. Either bind it to a test or drop it from the contract's fixture list.
