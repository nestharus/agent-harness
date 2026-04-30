# WU-0B-01 Scope Gate

**Severity:** LOW

## Verdict

The implementer kept the work strictly inside the ticket-declared code/test boundary. The seven `MigrationRecord` fields, four `MigrationRollbackState` variants, five `MigrationReport` fields, and five `MigrationError` variants are all materialised in `src-tauri/src/contracts/graphstore_migrations.rs` and exercised through named fixtures under `product-strategy/contracts/fixtures/wu-0b-01/`. All eight acceptance criteria are wired to dedicated `#[tokio::test]` cases or to the round-trip `#[test]`. The only domain-level table created is `schema_versions`. The Phase 0A scaffold-test rename is documented in the proposal and is bounded to the migration-related assertion. No findings beyond a single nit.

## Boundary Verification

Ticket boundary (verbatim from `tickets-phase-0b:plans/tickets/phase-0b/WU-0B-01.md`):

- Test boundary: `product-strategy/contracts/wu-0b-01-migration-framework.md`; `src-tauri/migrations/0001_schema_versions.sql`; `src-tauri/src/contracts/graphstore_migrations.rs`.
- Code boundary: `src-tauri/migrations/0001_schema_versions.sql`; `src-tauri/src/graphstore/migrations.rs`; `src-tauri/tests/graphstore_migrations_contract.rs`.

Files actually changed/added on `impl-wu-0b-01` vs `main`:

| Path | Status | Inside boundary? |
|---|---|---|
| `product-strategy/contracts/wu-0b-01-migration-framework.md` | new | yes (test boundary) |
| `product-strategy/contracts/fixtures/wu-0b-01/*.json` (11 files) | new | yes (named-fixture surface declared in contract; supports test boundary) |
| `src-tauri/migrations/0001_schema_versions.sql` | new | yes (test + code boundary) |
| `src-tauri/src/contracts/graphstore_migrations.rs` | new | yes (test boundary) |
| `src-tauri/src/graphstore/{mod,migrations}.rs` | new | yes (code boundary) |
| `src-tauri/tests/graphstore_migrations_contract.rs` | new | yes (code boundary) |
| `src-tauri/src/lib.rs` | modified (added `pub mod graphstore;`) | bounded coherent wiring |
| `src-tauri/src/contracts/mod.rs` | modified (added `pub mod graphstore_migrations;`) | bounded coherent wiring |
| `src-tauri/Cargo.toml` | modified (added `sha2 = "0.10.9"`) | bounded coherent dep add |
| `src-tauri/Cargo.lock` | modified (added the `sha2` line under the package's deps; no transitive crates introduced) | bounded |
| `src-tauri/tests/scaffold_contract.rs` | modified (rename + assertion update) | called out in proposal §"Phase 0A Scaffold Contract Update" |
| `proposals/0b-01-wu-0b-01.md` | new | proposal artifact |

## Per-Item Findings

### WU-0B-01-SCOPE-F01 — `MigrationRecord` field coverage (PASS)

`src-tauri/src/contracts/graphstore_migrations.rs:17-25` declares all seven contract fields with the documented types: `version: i64`, `migration_name: String`, `applied_at: String`, `checksum: String`, `execution_ms: i64`, `applied_by: String`, `rollback_state: MigrationRollbackState`. Round-trip is verified by `migration_record_report_and_error_fixtures_round_trip_stable_shapes` (`src-tauri/tests/graphstore_migrations_contract.rs:464-481`) against `migration-record-round-trip.json` and `schema-versions-canonical-row.json`. Both fixtures populate all seven fields.

### WU-0B-01-SCOPE-F02 — `MigrationRollbackState` 4-variant coverage (PASS)

`src-tauri/src/contracts/graphstore_migrations.rs:8-13` enumerates `Applied`, `RollbackUnsupported`, `RollbackPlanned`, `RollbackFailed`. All four are reachable from `rollback_states_round_trip_through_serde_and_sqlx_and_unknown_is_rejected` (`src-tauri/tests/graphstore_migrations_contract.rs:381-439`), which iterates the `valid` array of `rollback-states.json` (which lists all four) and asserts both serde round-trip *and* a real sqlx INSERT followed by sqlx FromRow decode. The same test asserts that decoding the `invalid` token (`rollback_unknown`) fails through both serde and sqlx. The `migration-report-variants.json` fixture additionally exercises all four variants in the `rollback_state` slot of `MigrationReport`.

### WU-0B-01-SCOPE-F03 — `MigrationReport` 5-field coverage (PASS)

`src-tauri/src/contracts/graphstore_migrations.rs:29-35` declares `applied_versions`, `skipped_versions`, `failed_version: Option<i64>`, `checksum_mismatches: Vec<i64>`, `rollback_state`. Both `migration-report-canonical.json` and the four-element `migration-report-variants.json` populate every field, and both are round-tripped in `migration_record_report_and_error_fixtures_round_trip_stable_shapes`.

### WU-0B-01-SCOPE-F04 — `MigrationError` 5-variant coverage with NAMED fixtures and exact equality (PASS)

`src-tauri/src/contracts/graphstore_migrations.rs:50-56` declares the five contract variants. Each variant has a dedicated fixture and a dedicated test that asserts via `assert_eq!(error, fixture.expected_error)` AND `assert_eq!(error, MigrationError::<Variant>)` — so the assertion is structural, not string containment:

| Variant | Fixture | Test |
|---|---|---|
| `OutOfOrderVersion` | `error-out-of-order-version.json` | `duplicate_versions_are_rejected_before_later_migrations_apply`; also `lower_missing_version_after_higher_applied_version_is_out_of_order` covers the runtime-state path |
| `VersionChecksumMismatch` | `error-version-checksum-mismatch.json` | `changed_applied_migration_returns_checksum_mismatch_without_rewriting_rows` |
| `VersionAlreadyAppliedDifferently` | `error-version-already-applied-differently.json` | `already_applied_version_with_different_name_is_rejected_exactly` |
| `RollbackUnsupported` | `error-rollback-unsupported.json` | `unsupported_rollback_returns_exact_error_and_preserves_history` |
| `SqlxFailure` | `error-sqlx-failure.json` | `sqlx_failure_variant_is_produced_by_named_fixture` |

In addition, `migration_record_report_and_error_fixtures_round_trip_stable_shapes` reads each fixture's `expected_error` and asserts the full ordered vector matches the five variants — a coverage canary that fails if a fixture is renamed or a variant is added without a fixture.

### WU-0B-01-SCOPE-F05 — All 8 acceptance criteria exercised (PASS)

| AC | Exercised by |
|---|---|
| Fresh DB applies N ordered, records exactly N rows | `fresh_temp_sqlite_applies_ordered_migrations_and_records_schema_versions` |
| Idempotent rerun: empty applied + no row mutation | `rerunning_identical_migrations_is_idempotent_and_does_not_rewrite_rows` (snapshots `before` and `after`, asserts equality) |
| Changed applied migration → `VersionChecksumMismatch`, rows unchanged | `changed_applied_migration_returns_checksum_mismatch_without_rewriting_rows` |
| Duplicate / out-of-order → exact errors before later migrations apply | `duplicate_versions_are_rejected_before_later_migrations_apply` (asserts `!schema_versions_exists` after rejection — proving validation fires before bootstrap is applied); `lower_missing_version_after_higher_applied_version_is_out_of_order`; `already_applied_version_with_different_name_is_rejected_exactly` |
| Unsupported rollback → exact error, history unchanged | `unsupported_rollback_returns_exact_error_and_preserves_history` |
| All 4 rollback states round-trip + unknown rejected | `rollback_states_round_trip_through_serde_and_sqlx_and_unknown_is_rejected` |
| Each `MigrationError` variant from a named fixture, exact assertion | All 5 error tests above + the round-trip canary |
| Only `schema_versions` durable table after WU-0B-01 directory runs | `shipped_wu_0b_01_migration_emits_only_schema_versions` (filters `sqlite_*` and asserts `vec!["schema_versions"]`) |

### WU-0B-01-SCOPE-F06 — Only `schema_versions` is created (PASS)

`grep -rn 'CREATE TABLE' src-tauri/migrations/` returns exactly one hit: `src-tauri/migrations/0001_schema_versions.sql:1:CREATE TABLE IF NOT EXISTS schema_versions (`. No `graph_workspace`, `graph_node`, or other Phase 0B domain tables exist on this branch. The contract test `shipped_wu_0b_01_migration_emits_only_schema_versions` is a runtime guard against future leakage.

### WU-0B-01-SCOPE-F07 — Phase 0A scaffold-test rename is bounded and documented (PASS)

`git diff main -- src-tauri/tests/scaffold_contract.rs` shows two related edits inside a single test:

1. The test was renamed from `cargo_manifest_declares_phase_0a_runtime_dependencies_without_migrations` to `cargo_manifest_declares_phase_0a_runtime_dependencies_without_sqlx_migrate_feature`. The rename clarifies the surviving invariant: the assertion is now exclusively about `sqlx`'s `migrate` feature flag rather than about the directory being empty.
2. The migrations-directory assertion changes from "no entries" to "exactly `0001_schema_versions.sql`".

Both edits are inside the same test and both are explicitly called out in the proposal §"Phase 0A Scaffold Contract Update" as a bounded scaffold-contract update. The other two tests in `scaffold_contract.rs` (`workspace_manifests_match_phase_0a_contract_fixtures`, `tauri_bootstrap_is_inert_and_command_free`) are byte-identical to `main`. Most importantly, the renamed test still asserts:

```rust
assert!(!features.iter().any(|feature| feature.as_str() == Some("migrate")), ...);
```

so the sqlx `migrate` feature is still rejected at the assertion level (`src-tauri/tests/scaffold_contract.rs:117-122`).

### WU-0B-01-SCOPE-F08 — `phase_0a_scaffold_commands()` length unchanged (PASS)

`tauri_bootstrap_is_inert_and_command_free` (`src-tauri/tests/scaffold_contract.rs:139-187`) is unchanged and still asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`. The library-internal `tests` module in `src-tauri/src/lib.rs:78-85` independently asserts the same. Both tests pass under `cargo test`. The runtime length is **1**.

### WU-0B-01-SCOPE-N01 — Nit: `migration-record-round-trip.json` and `schema-versions-canonical-row.json` overlap (NIT)

Both fixtures are pure DTO shapes for `MigrationRecord` and serve essentially the same round-trip role with slightly different rollback-state variants (`rollback_planned` vs `applied`). They are not redundant — together they exercise non-default and default variants — but the two-file split is borderline against the "minimum named fixtures" principle. No action required; this is purely an observation for future curation.

## Acceptance Criteria Source

- `git show tickets-phase-0b:plans/tickets/phase-0b/WU-0B-01.md` §"Acceptance criteria"
- `product-strategy/contracts/wu-0b-01-migration-framework.md` §"Behavioral Requirements"
- `proposals/0b-01-wu-0b-01.md` §"Test Intent"

## Conclusion

Severity **LOW**. All ticket-named DTOs, error variants, and acceptance criteria are exercised; named fixtures back every error variant; the only durable table is `schema_versions`; the Phase 0A scaffold change is bounded, documented, and preserves the `migrate`-feature rejection. No scope creep.
