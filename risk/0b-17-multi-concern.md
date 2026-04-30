# WU-0B-17 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern PR.
**Severity:** LOW.

## Question

Is the change set strictly single-concern (only WU-0B-17 — `ProviderState` row, table, repository, fixtures, and the unavoidable migration-list / FK-target / runtime-table bumps in prior-WU contract tests + WU-0B-03 / WU-0B-04 fixtures + WU-0B-15 / WU-0B-16 fixtures whose previously-soft-ref `provider_state_id` must now reflect the real FK)? Are prior-WU implementation files modified, are unrelated refactors bundled in, or is `Cargo.toml` / `package.json` / `tauri.conf.json` / `src-tauri/src/lib.rs` touched? Are the WU-0B-15 / WU-0B-16 fixture changes bounded to FK-target reflection rather than semantic schema changes to `AuditEvent` / `BudgetLedger`?

## Findings

### WU-0B-17-MULTI-F01 — Tracked diff against `main` is 33 insertions / 18 deletions across 20 files; every line is mechanical

`git diff main --stat`:

```
 .../fixtures/wu-0b-03/graphstore-fixture-shape.json          |  2 +-
 .../fixtures/wu-0b-03/graphstore-pool-shape.json             |  2 +-
 .../fixtures/wu-0b-03/no-side-effects-assertions.json        |  1 +
 .../fixtures/wu-0b-03/reset-preserves-migration-history.json |  2 +-
 .../contracts/fixtures/wu-0b-04/no-side-effects.json         |  2 +-
 .../contracts/fixtures/wu-0b-15/canonical-row.json           |  5 +----
 product-strategy/contracts/fixtures/wu-0b-15/round-trip.json |  5 +----
 .../contracts/fixtures/wu-0b-16/canonical-row.json           |  5 +----
 product-strategy/contracts/fixtures/wu-0b-16/round-trip.json |  5 +----
 src-tauri/src/contracts/mod.rs                               |  1 +
 src-tauri/src/graphstore/mod.rs                              |  1 +
 src-tauri/tests/graphstore_fixture_contract.rs               |  9 +++++----
 src-tauri/tests/graphstore_migrations_contract.rs            |  3 ++-
 src-tauri/tests/wu_0b_04_policyset_contract.rs               |  2 +-
 src-tauri/tests/wu_0b_05_graphconfiguration_contract.rs      |  3 ++-
 src-tauri/tests/wu_0b_06_graphworkspace_contract.rs          |  3 ++-
 src-tauri/tests/wu_0b_07_graphnode_contract.rs               |  3 ++-
 src-tauri/tests/wu_0b_09_evidenceartifact_contract.rs        |  3 ++-
 src-tauri/tests/wu_0b_15_auditevent_contract.rs              |  4 +++-
 src-tauri/tests/wu_0b_16_budgetledger_contract.rs            | 10 +++-------
 20 files changed, 33 insertions(+), 38 deletions(-)
```

Categorized:
- **Module wiring (2 lines):** `src-tauri/src/contracts/mod.rs` adds `pub mod providerstate;` alphabetically; `src-tauri/src/graphstore/mod.rs` adds the same. Unavoidable.
- **Prior-WU contract test migration-list / runtime-table / FK-target bumps (8 files):** every hunk is one of the three structural shapes catalogued in `0b-17-supported-surface.md` SURFACE-F05. No new tests, no deleted tests, no renamed tests, no behavioral assertion shape changes (other than the WU-0B-16 soft-ref negative-assertion inversion described in MULTI-F03 below).
- **Prior-WU fixture migration-list / runtime-table mirror updates (5 wu-0b-03/04 files):** JSON-fixture mirrors of the same migration-list / runtime-table updates that the contract tests assert against.
- **Prior-WU fixture FK-target reflection (4 wu-0b-15/16 fixture files):** `provider_state_id` field flipped from a hard-coded soft-ref id (which would now violate the new real FK) to `null` — see MULTI-F04 for justification.

### WU-0B-17-MULTI-F02 — All untracked additions sit inside the WU-0B-17 boundary

`git ls-files --others --exclude-standard | grep -v "^risk/"` reports the following 19 paths, each mapping to a ticket boundary entry:

| Untracked file | Boundary |
| --- | --- |
| `product-strategy/contracts/wu-0b-17-providerstate.md` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/canonical-row.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/round-trip.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/provider-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/cli-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/auth-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/billing-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/quota-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/network-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/runtime-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/freshness-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/confidence-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/unknown-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/secret-rejections.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-17/stale-marking.json` | Test boundary. |
| `src-tauri/migrations/0b/17_providerstate.sql` | Code boundary. |
| `src-tauri/src/graphstore/providerstate.rs` | Code boundary. |
| `src-tauri/src/contracts/providerstate.rs` | Code boundary (re-export shim). |
| `src-tauri/tests/wu_0b_17_providerstate_contract.rs` | Code boundary. |

No stray files, no scratch artifacts.

### WU-0B-17-MULTI-F03 — Prior-WU implementation files (Rust) are not modified

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs src-tauri/src/graphstore/auditevent.rs src-tauri/src/graphstore/budgetledger.rs` is empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs src-tauri/src/contracts/evidenceartifact.rs src-tauri/src/contracts/auditevent.rs src-tauri/src/contracts/budgetledger.rs` is empty.

`git diff main -- src-tauri/migrations/0b/15_auditevent.sql src-tauri/migrations/0b/16_budgetledger.sql` is **empty** — the WU-0B-01 migration-checksum invariant is preserved (see `0b-17-supported-surface.md` SURFACE-F03 for the full mechanism analysis).

Predecessor types are *imported* — `src-tauri/src/graphstore/providerstate.rs:9-14` — for use as `OpaqueId<T>` type markers (`ProviderStateRef`, `GraphWorkspace`, `GraphWorkspaceRef`) and for the `GraphStoreRepo` trait. Notably `ProviderStateRef` is reused from WU-0B-15 (`auditevent.rs:18`) rather than redefined locally — same pattern as WU-0B-16 — which avoids a stealth duplicate marker drift.

### WU-0B-17-MULTI-F04 — `wu-0b-15` / `wu-0b-16` fixture changes are bounded to FK-target reflection, not semantic schema changes

The four affected fixtures (`canonical-row.json` and `round-trip.json` for both WU-0B-15 and WU-0B-16) flip the `provider_state_id` field from a hard-coded soft-ref id (e.g. `"provider-state-soft-ref-v1"`) to `null`. **This is required**, not optional: once migration 17 introduces the real FK, the previously-stored soft-ref ids point to non-existent `provider_states` rows and would fail the FK constraint. Three resolutions were possible:

| Path | Trade-off | Status |
| --- | --- | --- |
| (a) Set `provider_state_id` to `null` | Smallest diff; loses round-trip coverage of the *value* path in WU-0B-15 / WU-0B-16 | **Taken.** |
| (b) Have the WU-0B-15 / WU-0B-16 contract test seed a real `provider_states` row before inserting | Adds a cross-WU dependency to the prior contract tests | Not taken. |
| (c) Move the seeding to the fixture file by referencing a `provider-state` `OpaqueId` and have the contract test seed a matching row | Larger diff; arguably cleanest | Not taken. |

Path (a) is acceptable because the value path of `provider_state_id` is *now exercised* in this WU's `wu_0b_17_providerstate_contract.rs` via:
- `provider_states_schema_contains_declared_columns_constraints_indexes_and_fks` (`:253-263`) asserts the FK both exists and points to `provider_states` from both `audit_events` and `budget_ledgers`.
- `insert_then_get_round_trips_every_provider_state_field_byte_equivalent` (`:288-314`) round-trips the `provider_state_id` opaque ID byte-equivalent (it is the row's primary key, with `value` and `namespace` both populated).

The previously-tested round-trip-of-a-soft-ref-value behavior in WU-0B-15 / WU-0B-16 is not lost in spirit — it has been *promoted to a real FK* and is exercised end-to-end in WU-0B-17. The downstream insert/get/list assertions in WU-0B-15 / WU-0B-16 still cover every other field byte-equivalent (`assert_eq!(serde_json::to_string(&fixture.row), serde_json::to_string(&inserted))`) — those tests still pass with `provider_state_id: null` because the `Option<OpaqueId<ProviderStateRef>>` field on `AuditEvent` / `BudgetLedger` correctly serializes `None` as `null`. **Bounded to FK-target reflection.**

### WU-0B-17-MULTI-F05 — WU-0B-16 contract test inversion is the natural consequence of the FK being introduced

`tests/wu_0b_16_budgetledger_contract.rs` removes (10 lines) the negative assertion that previously read:
```
assert!(
    !fks.iter().any(|row| row.get::<String, _>("table").contains("provider")),
    "provider_state_id must remain a soft ref in WU-0B-16"
);
```
and adds `"provider_states"` to the positive expected-FK-target list. The removed negative was bounded to WU-0B-16's slice (it asserted "remain a soft ref *in WU-0B-16*", consistent with the contract document `wu-0b-16-budgetledger.md:27` that explicitly authorizes this as a soft reference *until WU-0B-17*). Removing it in WU-0B-17 — the WU that introduces `provider_states` as the real FK target — is the contractually correct inversion. Leaving the negative in place would have made `wu_0b_16_budgetledger_contract` fail the moment migration 17 is applied to its fixture pool, which is exactly the case after this WU.

### WU-0B-17-MULTI-F06 — `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `turbo.json`, `tauri.conf.json`, `lib.rs` all unchanged

`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` is empty. No new dependencies, no version bumps, no script changes, no allowlist edits, no command registration.

### WU-0B-17-MULTI-F07 — No cross-WU refactor bundled in

The diff introduces no rename, no signature change, no reformat, no trait extraction, and no helper-function extraction in any prior-WU file. The new `ProviderStateRepo` implements the existing `GraphStoreRepo<ProviderState>` trait from WU-0B-03 (`src-tauri/src/graphstore/providerstate.rs:348-374`) — the trait is *consumed*, not modified. Reuse of `validate_record_meta`, `JsonField`, `OpaqueId`, `RecordMeta`, `ActorRef`, `GraphStoreError` is import-only (`:12-14`). The `impl_sqlite_text_enum!` macro is locally defined per-module (`:884-913`) — consistent with `auditevent.rs`, `budgetledger.rs`, `graphnode.rs`, `graphconfiguration.rs`, `evidenceartifact.rs`. No extraction into a shared module was attempted, which keeps the change set strictly additive and consistent with prior-WU convention.

### WU-0B-17-MULTI-F08 — `wu-0b-03` and `wu-0b-04` fixture mirror updates are JSON-only data follow-on, not behavioral edits

The five JSON-fixture diffs at the wu-0b-03/04 level are direct mirrors of the migration-list / runtime-table updates that the contract tests assert against:
- `wu-0b-03/no-side-effects-assertions.json`: adds `"provider_states"` to `allowed_runtime_tables`.
- `wu-0b-03/reset-preserves-migration-history.json`: bumps `skipped_versions` to `[1, 4, 5, 6, 7, 9, 15, 16, 17]`.
- `wu-0b-04/no-side-effects.json`: adds `"provider_states"` to `allowed_runtime_tables`.
- `wu-0b-03/graphstore-fixture-shape.json` / `graphstore-pool-shape.json`: bumps `migrations_applied` from `[1, 4, 5, 6, 7, 9, 15]` (a pre-existing inconsistency on `main`) to `[1, 4, 5, 6, 7, 9, 15, 16, 17]`. Catches up by two, fixing both the missing 16 and adding 17.

The `graphstore_fixture_contract` test asserts `rerun.skipped_versions == vec![1, 4, 5, 6, 7, 9, 15, 16, 17]` and passes — meaning the fixture values match actual harness behavior. **Acceptable inside this WU because (a) it's fixture data only, no production code changed, and (b) leaving it inconsistent would have left the wu-0b-03 illustrative fixture two migrations behind reality.**

### WU-0B-17-MULTI-F09 — Branch state is uncommitted but cleanly bounded

`git log main..HEAD --oneline` is empty — there is no commit on the branch yet. The reviewer instructions explicitly forbid committing or pushing, so this is the expected state at gate-review time. When the implementer commits, the staged set should be exactly the 39 paths enumerated by `git status` (20 modified + 19 untracked, excluding `risk/`) — no other files appear in the working tree.

### Verification commands

| Command | Result |
| --- | --- |
| `git diff main --stat` | 20 files, 33 insertions, 18 deletions — categorized in MULTI-F01. |
| `git diff main -- src-tauri/src/graphstore/{policyset,graphconfiguration,graphworkspace,graphnode,evidenceartifact,auditevent,budgetledger}.rs` | empty. |
| `git diff main -- src-tauri/src/contracts/{policyset,graphconfiguration,graphworkspace,graphnode,evidenceartifact,auditevent,budgetledger}.rs` | empty. |
| `git diff main -- src-tauri/migrations/0b/15_auditevent.sql src-tauri/migrations/0b/16_budgetledger.sql` | **empty** (checksum invariant intact). |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty. |
| `git ls-files --others --exclude-standard \| grep -v "^risk/"` | 19 files, all inside the WU-0B-17 boundary. |
| `git log main..HEAD --oneline` | empty (uncommitted). |

## Verdict

**LOW.** The PR is single-concern. Tracked edits decompose to:
- 2 unavoidable `pub mod providerstate;` wiring lines.
- 8 prior-WU contract test files with literal migration-list / runtime-table / FK-target bumps (no behavioral or assertion-shape changes other than the contractually-required WU-0B-16 soft-ref negative-assertion inversion).
- 5 prior-WU fixture files at the wu-0b-03/04 level with JSON-only migration-list / runtime-table mirror updates.
- 4 prior-WU fixture files at the wu-0b-15/16 level with `provider_state_id` flipped from a hard-coded soft-ref id to `null` — required by the introduction of the real FK, bounded to FK-target reflection rather than semantic schema changes (`AuditEvent` / `BudgetLedger` row shapes are unchanged; the tests still serialize-round-trip every other field byte-equivalent; the value path is now exercised in this WU's contract test as a true FK).

All untracked additions map to ticket boundaries (the contract md, 14 fixtures under `wu-0b-17/`, the migration SQL, the two Rust source files, the contract test). No prior-WU *implementation* file (`graphstore/*.rs`, `contracts/*.rs`, or shipped migration SQL) is modified — predecessor types are imported and reused, never edited. No `Cargo.toml` / `Cargo.lock` / `package.json` / `bun.lock` / `turbo.json` / `tauri.conf.json` / `lib.rs` churn.
