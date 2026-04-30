# WU-0B-02 Scope Gate

**Severity:** LOW

## Verdict

The implementer kept the work strictly inside the ticket-declared code/test boundary. `RecordMeta` has all four contract fields, `OpaqueId<T>` is phantom-typed and carries `value` + `namespace` only, all four OpaqueId rejection patterns (path separator, content-hash prefix, empty namespace, mutable parent-path encoding) are individually exercised through named fixtures, and all seven `GraphStoreError` variants are reachable through fixtures with stable snake_case `code()` strings. All six acceptance criteria are exercised by dedicated tests. No findings beyond one nit.

## Boundary Verification

Ticket boundary (verbatim from `tickets-phase-0b:plans/tickets/phase-0b/WU-0B-02.md`):

- Test boundary: `product-strategy/contracts/wu-0b-02-recordmeta-prelude.md`; `src-tauri/src/contracts/graphstore_prelude.rs`.
- Code boundary: `src-tauri/src/graphstore/prelude.rs`; `src-tauri/tests/graphstore_prelude_contract.rs`.

Files actually changed/added on `impl-wu-0b-02` vs `main`:

| Path | Status | Inside boundary? |
|---|---|---|
| `product-strategy/contracts/wu-0b-02-recordmeta-prelude.md` | new | yes (test boundary) |
| `product-strategy/contracts/fixtures/wu-0b-02/*.json` (18 files) | new | yes (named-fixture surface declared in contract; supports test boundary) |
| `src-tauri/src/graphstore/prelude.rs` | new | yes (code boundary) |
| `src-tauri/src/contracts/graphstore_prelude.rs` | new | yes (test boundary) |
| `src-tauri/tests/graphstore_prelude_contract.rs` | new | yes (code boundary) |
| `src-tauri/src/graphstore/mod.rs` | modified (added `pub mod prelude;`) | bounded coherent wiring |
| `src-tauri/src/contracts/mod.rs` | modified (added `pub mod graphstore_prelude;`) | bounded coherent wiring |
| `proposals/0b-02-wu-0b-02.md` | new | proposal artifact |

`git diff main --stat` shows exactly two modified files; everything else is new and lives under WU-0B-02-owned paths. `Cargo.toml` and `Cargo.lock` are byte-identical to `main`.

## Per-Item Findings

### WU-0B-02-SCOPE-F01 — `RecordMeta` 4-field coverage (PASS)

`src-tauri/src/graphstore/prelude.rs:22-27` declares all four contract fields with the documented types: `created_at: Timestamp`, `updated_at: Timestamp`, `actor: ActorRef`, `policy_version: String`. `Timestamp` is a `String` type alias (`:12`) and `ActorRef { value: String }` is the documented DTO (`:16-18`), matching the proposal's design and the contract's surface section. Round-trip through serde is exercised by `record-meta-canonical.json` populating all four fields and `record_meta_validation_preserves_canonical_and_rejects_invalid_inputs` (`graphstore_prelude_contract.rs:80-93`) asserting `validate_record_meta(canonical.clone()) == canonical`.

### WU-0B-02-SCOPE-F02 — `OpaqueId<T>` is phantom-typed and carries only `value` + `namespace` (PASS)

`src-tauri/src/graphstore/prelude.rs:29-36` declares `OpaqueId<T> { value: String, namespace: String, marker: PhantomData<fn() -> T> }`. The marker is `#[serde(skip)]` so the on-wire shape is exactly `{ "value": ..., "namespace": ... }`. The `bound = ""` clause on the serde derive prevents serde from requiring `T: Serialize`/`T: Deserialize`. `PartialEq`, `Eq`, and `Hash` (`:38-51`) are hand-implemented over `(value, namespace)` only, so two `OpaqueId<T>`s with different `T` cannot accidentally compare equal at the type level (the comparison only typechecks if both share the same `T`). The contract test `opaque_ids_preserve_valid_values_and_reject_documented_invalid_patterns` (`graphstore_prelude_contract.rs:96-116`) constructs `OpaqueId::<ContractOpaqueMarker>` with the canonical fixture and asserts the constructed value equals the deserialized fixture — proving the carrier-type discipline holds at the API surface.

### WU-0B-02-SCOPE-F03 — All four OpaqueId rejection patterns reachable through NAMED fixtures (PASS)

The contract enumerates four rejection patterns. Each has a dedicated fixture and is exercised by a dedicated iteration in `opaque_ids_preserve_valid_values_and_reject_documented_invalid_patterns` (`graphstore_prelude_contract.rs:105-115`):

| Pattern | Fixture | Implementation guard |
|---|---|---|
| Path separator (`/`, `\`, `:` in `value` or `namespace`) | `opaque-id-invalid-path-separator.json` (`value: "node/contract"`) | `has_forbidden_id_char` (`prelude.rs:187-189`) |
| Content-hash prefix (`sha256:`, `sha512:`, `blake3:`, `md5:`) | `opaque-id-invalid-content-hash-prefix.json` (`value: "sha256:abcdef"`) | `has_content_hash_prefix` (`prelude.rs:180-185`) |
| Empty namespace (empty or whitespace-only) | `opaque-id-invalid-empty-namespace.json` (`namespace: "   "`) | `value.trim().is_empty()` / `namespace.trim().is_empty()` (`prelude.rs:163-165`) |
| Mutable parent-path encoding (`.`, `..`, `../`, `..\`, `/`, `\`) | `opaque-id-invalid-parent-path.json` (`value: "parent/child"`) | `has_parent_path_encoding` (`prelude.rs:191-197`) |

Each fixture iteration calls `OpaqueId::<ContractOpaqueMarker>::new(id.value, id.namespace).expect_err(...)` and `assert_invariant_violation(error)` — the assertion is structural via `assert_eq!(error, GraphStoreError::InvariantViolation)` AND `assert_eq!(error.code(), "invariant_violation")`, so the test fails if either the variant or the code drifts. PASS.

### WU-0B-02-SCOPE-F04 — All 7 `GraphStoreError` variants reachable with stable `code()` (PASS)

`src-tauri/src/graphstore/prelude.rs:108-117` enumerates all seven contract variants. `code()` (`:120-130`) is a total `match` returning a unique stable snake_case `&'static str` for each. The contract test `every_graphstore_error_variant_has_a_stable_machine_code_fixture` (`graphstore_prelude_contract.rs:118-160`) is a layered coverage assertion:

1. It loads `graphstore-errors.json` (the index) and asserts `index.errors.len() == 7` — a canary that fails if a variant is added/removed without updating the index.
2. It iterates the index, deserializes each pointed-to fixture, and asserts `fixture.variant == entry.variant && fixture.code == entry.code && error.code() == fixture.code`.
3. It then explicitly re-asserts every one of the seven variants with its named fixture (`error-unknown-ref.json` … `error-sqlx-failure.json`) — so the test fails if any single fixture is renamed or its content drifts, even if the index were silently kept consistent.

| Variant | Fixture | `code()` |
|---|---|---|
| `UnknownRef` | `error-unknown-ref.json` | `unknown_ref` |
| `DuplicateId` | `error-duplicate-id.json` | `duplicate_id` |
| `InvalidEnum` | `error-invalid-enum.json` | `invalid_enum` |
| `InvalidTransition` | `error-invalid-transition.json` | `invalid_transition` |
| `InvariantViolation` | `error-invariant-violation.json` | `invariant_violation` |
| `OptimisticConflict` | `error-optimistic-conflict.json` | `optimistic_conflict` |
| `SqlxFailure` | `error-sqlx-failure.json` | `sqlx_failure` |

PASS.

### WU-0B-02-SCOPE-F05 — All 6 acceptance criteria exercised (PASS)

| AC | Exercised by |
|---|---|
| `validate_record_meta(meta)` preserves valid canonical input; documented invalid inputs return `InvariantViolation` without partial writes | `record_meta_validation_preserves_canonical_and_rejects_invalid_inputs` (`graphstore_prelude_contract.rs:76-93`); the proposal asserts `validate_record_meta` performs no writes and the implementation at `prelude.rs:147-161` returns the input unchanged on success and an error on failure with no side effects |
| Opaque IDs with path separators, content-hash prefixes, empty namespaces, or mutable parent-path encodings rejected with `InvariantViolation` | `opaque_ids_preserve_valid_values_and_reject_documented_invalid_patterns` (`graphstore_prelude_contract.rs:96-116`) |
| Every `GraphStoreError` variant reachable through documented fixture + stable machine-readable code | `every_graphstore_error_variant_has_a_stable_machine_code_fixture` (`graphstore_prelude_contract.rs:118-160`) |
| `JsonField<T>` round-trips arrays, maps, nested refs, empty arrays, null optional refs, enum payloads | `jsonfield_round_trips_documented_json_shapes_through_sqlite_text` (`graphstore_prelude_contract.rs:162-208`) — six fixture cases match the AC's six categories one-for-one |
| `updated_at` cannot precede `created_at`; rejected before insert | `invalid_timestamp_order_is_rejected_before_insert` (`graphstore_prelude_contract.rs:210-240`) creates a real SQLite table, calls `validate_record_meta` on the invalid fixture, asserts `expect_err`, then `SELECT COUNT(*)` against the table and asserts `count.0 == 0` |
| Prelude exports no durable domain table and no value-slice service operation | `prelude_exports_no_durable_domain_table_or_value_slice_command` (`graphstore_prelude_contract.rs:242-270`) introspects `sqlite_master` after exercising the codec and asserts no tables remain; also asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]` |

PASS.

### WU-0B-02-SCOPE-F06 — No durable domain table and no migration file added (PASS)

`ls src-tauri/migrations/` returns exactly `0001_schema_versions.sql` (the WU-0B-01 bootstrap). `git diff main -- src-tauri/migrations/` is empty. The implementation file `src-tauri/src/graphstore/prelude.rs` contains zero `CREATE TABLE` strings; the only SQL strings in the WU-0B-02 surface are the `CREATE TABLE` and `CREATE TEMP TABLE` statements inside the contract test (which run only against `sqlite::memory:` and are wiped at pool drop). The `prelude_exports_no_durable_domain_table_or_value_slice_command` test (`graphstore_prelude_contract.rs:264-268`) is a runtime guard: after exercising the codec it queries `sqlite_master WHERE type = 'table'` and asserts the result is empty.

### WU-0B-02-SCOPE-F07 — `phase_0a_scaffold_commands()` length unchanged (PASS)

`src-tauri/src/lib.rs:31-37` is byte-identical to `main`. The internal `tests` module at `:78-85` still asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`. The Phase 0A `tauri_bootstrap_is_inert_and_command_free` contract test is unchanged and still passes. The new WU-0B-02 contract test additionally re-asserts the same allowlist (`graphstore_prelude_contract.rs:269`) — the runtime length is **1**.

### WU-0B-02-SCOPE-N01 — Nit: contract md lists "malformed timestamp strings" as a rejection but no dedicated fixture exists (NIT)

`product-strategy/contracts/wu-0b-02-recordmeta-prelude.md:25` says `validate_record_meta` rejects "malformed timestamp strings". The implementation honours this: `parse_contract_timestamp` (`prelude.rs:209-247`) requires the exact `YYYY-MM-DDTHH:MM:SSZ` shape, parses each digit, validates month/day/hour/minute/second ranges, and returns `None` (which becomes `InvariantViolation` at the call site) for any malformed input. But the fixtures cover only the three semantic-invalid cases (updated-before-created, empty actor, empty policy version) — there is no `record-meta-invalid-malformed-timestamp.json`. The `invalid-updated-before-created` fixture happens to exercise the parser-success path (both timestamps parse, then the order check fires), so the parser's rejection branch is not test-covered. Adding one fixture (e.g. `created_at: "not-a-timestamp"`) would close the gap. NIT — the implementation is correct; only the fixture surface is short of the documented rejection set.

## Acceptance Criteria Source

- `git show tickets-phase-0b:plans/tickets/phase-0b/WU-0B-02.md` §"Acceptance criteria"
- `product-strategy/contracts/wu-0b-02-recordmeta-prelude.md` §"RecordMeta", §"OpaqueId", §"JsonField", §"GraphStoreError", §"Test Intent Handoff"
- `proposals/0b-02-wu-0b-02.md` §"Test Intent"

## Conclusion

Severity **LOW**. All ticket-named DTOs, error variants, and acceptance criteria are exercised; named fixtures back every error variant and every OpaqueId rejection pattern; the prelude exports no durable domain table; the Phase 0A command count is preserved. One NIT for an undocumented-but-implemented rejection branch (malformed timestamp) lacking a fixture. No scope creep.
