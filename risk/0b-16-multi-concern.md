# WU-0B-16 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern PR.
**Severity:** LOW.

## Question

Is the change set strictly single-concern (only WU-0B-16 — `BudgetLedger` row, table, repository, fixtures, and the unavoidable migration-list bumps in prior-WU contract tests / WU-0B-03 / WU-0B-04 fixtures)? Are prior-WU implementation files modified, are unrelated refactors bundled in, or is `Cargo.toml` / `package.json` / `tauri.conf.json` / `src-tauri/src/lib.rs` touched?

## Findings

### WU-0B-16-MULTI-F01 — Tracked diff against `main` is 44 insertions / 13 deletions across 13 files; every line is mechanical

`git diff main --stat`:

```
 .../fixtures/wu-0b-03/no-side-effects-assertions.json        |  1 +
 .../fixtures/wu-0b-03/reset-preserves-migration-history.json |  2 +-
 .../contracts/fixtures/wu-0b-04/no-side-effects.json         |  2 +-
 src-tauri/src/contracts/mod.rs                               |  1 +
 src-tauri/src/graphstore/mod.rs                              |  1 +
 src-tauri/tests/graphstore_fixture_contract.rs               | 12 ++++++++----
 src-tauri/tests/graphstore_migrations_contract.rs            |  3 ++-
 src-tauri/tests/wu_0b_04_policyset_contract.rs               |  5 ++++-
 src-tauri/tests/wu_0b_05_graphconfiguration_contract.rs      |  6 +++++-
 src-tauri/tests/wu_0b_06_graphworkspace_contract.rs          |  6 +++++-
 src-tauri/tests/wu_0b_07_graphnode_contract.rs               |  6 +++++-
 src-tauri/tests/wu_0b_09_evidenceartifact_contract.rs        |  6 +++++-
 src-tauri/tests/wu_0b_15_auditevent_contract.rs              |  6 +++++-
 13 files changed, 44 insertions(+), 13 deletions(-)
```

Categorized:
- **Module wiring (2 lines):** `src-tauri/src/contracts/mod.rs` adds `pub mod budgetledger;` alphabetically; `src-tauri/src/graphstore/mod.rs` adds the same. Unavoidable.
- **Prior-WU contract test migration-list bumps (8 files):** every hunk is a literal-list bump from `[1,4,5,6,7,9,15]` to `[1,4,5,6,7,9,15,16]` and a runtime-table list that adds `"budget_ledgers"` alphabetically (position 2 after `audit_events`). No assertion shape changes.
- **Prior-WU fixture migration-list bumps (3 files):** `wu-0b-03/no-side-effects-assertions.json`, `wu-0b-03/reset-preserves-migration-history.json`, `wu-0b-04/no-side-effects.json`. All are JSON-fixture mirrors of the same migration-list / runtime-table updates.

### WU-0B-16-MULTI-F02 — All untracked additions sit inside the WU-0B-16 boundary

`git ls-files --others --exclude-standard` reports the following paths, each mapping to a ticket boundary entry:

| Untracked file | Boundary |
| --- | --- |
| `product-strategy/contracts/wu-0b-16-budgetledger.md` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/canonical-row.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/round-trip.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/scope-type-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/budget-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/policy-action-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/unknown-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/counter-rejections.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/fk-rejections.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-16/policy-action-invariants.json` | Test boundary. |
| `proposals/0b-16-wu-0b-16.md` | Pipeline artifact (proposer output). |
| `src-tauri/migrations/0b/16_budgetledger.sql` | Code boundary. |
| `src-tauri/src/graphstore/budgetledger.rs` | Code boundary. |
| `src-tauri/src/contracts/budgetledger.rs` | Code boundary (re-export shim). |
| `src-tauri/tests/wu_0b_16_budgetledger_contract.rs` | Code boundary. |

No stray files.

### WU-0B-16-MULTI-F03 — Prior-WU implementation files are not modified

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs src-tauri/src/graphstore/auditevent.rs` is empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs src-tauri/src/contracts/evidenceartifact.rs src-tauri/src/contracts/auditevent.rs` is empty. Predecessor types (`PolicySet`, `GraphConfiguration`, `GraphWorkspace`, `ProviderStateRef`) are *imported* — `src-tauri/src/graphstore/budgetledger.rs:8-13` — for use as `OpaqueId<T>` type markers and FK-target validators; nothing in those modules is edited. Notably `ProviderStateRef` is reused from WU-0B-15 (`auditevent.rs:16-18`) rather than redefined locally, which avoids a stealth duplicate marker drift.

### WU-0B-16-MULTI-F04 — `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `turbo.json`, `tauri.conf.json`, `lib.rs` all unchanged

`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` is empty. No new dependencies, no version bumps, no script changes, no allowlist edits, no command registration.

### WU-0B-16-MULTI-F05 — No cross-WU refactor bundled in

The diff introduces no rename, no signature change, no reformat, no trait extraction, and no helper-function extraction in any prior-WU file. The new `BudgetLedgerRepo` implements the existing `GraphStoreRepo<BudgetLedger>` trait from WU-0B-03 (`src-tauri/src/graphstore/budgetledger.rs:242-268`) — the trait is *consumed*, not modified. Reuse of `validate_record_meta`, `JsonField`, `OpaqueId`, `RecordMeta`, `ActorRef`, and `GraphStoreError` is import-only (`:11-13`). The `impl_sqlite_text_enum!` macro is locally defined per-module (consistent with `auditevent.rs`, `graphnode.rs`, `graphconfiguration.rs`, `evidenceartifact.rs`) — no extraction into a shared module was attempted, which keeps the change set strictly additive.

### WU-0B-16-MULTI-F06 — `wu-0b-03` and `wu-0b-04` fixture mirror updates are JSON-only data follow-on, not behavioral edits

The three JSON-fixture diffs are direct mirrors of the migration-list / runtime-table updates that the contract tests assert against:
- `wu-0b-03/no-side-effects-assertions.json`: adds `"budget_ledgers"` to `allowed_runtime_tables`.
- `wu-0b-03/reset-preserves-migration-history.json`: bumps `skipped_versions` to `[1, 4, 5, 6, 7, 9, 15, 16]`.
- `wu-0b-04/no-side-effects.json`: adds `"budget_ledgers"` to `allowed_runtime_tables`.

The `graphstore_fixture_contract` test at `tests/graphstore_fixture_contract.rs:169` asserts `rerun.skipped_versions == vec![1, 4, 5, 6, 7, 9, 15, 16]` and passes — meaning the new fixture values match actual harness behavior. **Acceptable inside this WU because (a) it's fixture data only, no production code changed, and (b) leaving it inconsistent would have failed the test that passes today.**

### WU-0B-16-MULTI-F07 — Branch state is uncommitted but cleanly bounded

`git log main..HEAD --oneline` is empty — there is no commit on the branch yet. The reviewer instructions explicitly forbid committing or pushing, so this is the expected state at gate-review time. When the implementer commits, the staged set should be exactly the 28 paths enumerated by `git status` (13 modified + 15 untracked) — no other files appear in the working tree.

### Verification commands

| Command | Result |
| --- | --- |
| `git diff main --stat` | 13 files, 44 insertions, 13 deletions — all mechanical migration/runtime-list bumps + 2 `pub mod` lines. |
| `git diff main -- src-tauri/src/graphstore/{policyset,graphconfiguration,graphworkspace,graphnode,evidenceartifact,auditevent}.rs` | empty. |
| `git diff main -- src-tauri/src/contracts/{policyset,graphconfiguration,graphworkspace,graphnode,evidenceartifact,auditevent}.rs` | empty. |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty. |
| `git ls-files --others --exclude-standard` | 15 files, all inside the WU-0B-16 boundary or pipeline-artifact set. |
| `git log main..HEAD --oneline` | empty (uncommitted). |

## Verdict

**LOW.** The PR is single-concern. Tracked edits decompose to: 2 unavoidable `pub mod budgetledger;` wiring lines, 8 prior-WU contract test files with literal migration-list / runtime-table bumps (no behavioral or assertion-shape changes), and 3 prior-WU fixture files with the same JSON-only migration-list / runtime-table mirror updates. All untracked additions map to ticket or pipeline boundaries. No prior-WU *implementation* file (`graphstore/*.rs` or `contracts/*.rs`) is modified — predecessor types are imported and reused (notably `ProviderStateRef` from WU-0B-15 is shared rather than redefined), never edited. No `Cargo.toml` / `Cargo.lock` / `package.json` / `bun.lock` / `turbo.json` / `tauri.conf.json` / `lib.rs` churn. The fixture mirror updates are pure data debt the migration-list bump would otherwise have exposed as test failure; they are the right thing to fix here, contained inside this PR's strict scope.
