# WU-0B-15 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern PR.
**Severity:** LOW.

## Question

Is the change set strictly single-concern (only WU-0B-15 — `AuditEvent` row, table, repository, fixtures, and the unavoidable migration-list bumps in prior-WU contract tests / WU-0B-03 / WU-0B-04 fixtures)? Are prior-WU implementation files modified, are unrelated refactors bundled in, or is `Cargo.toml` / `package.json` / `tauri.conf.json` / `src-tauri/src/lib.rs` touched?

## Findings

### WU-0B-15-MULTI-F01 — Tracked diff against `main` is 23 insertions / 14 deletions across 14 files; every line is mechanical

`git diff main --stat`:

```
 product-strategy/contracts/fixtures/wu-0b-03/graphstore-fixture-shape.json    | 2 +-
 product-strategy/contracts/fixtures/wu-0b-03/graphstore-pool-shape.json       | 2 +-
 product-strategy/contracts/fixtures/wu-0b-03/no-side-effects-assertions.json  | 1 +
 product-strategy/contracts/fixtures/wu-0b-03/reset-preserves-migration-history.json | 2 +-
 product-strategy/contracts/fixtures/wu-0b-04/no-side-effects.json             | 2 +-
 src-tauri/src/contracts/mod.rs                                                | 1 +
 src-tauri/src/graphstore/mod.rs                                               | 1 +
 src-tauri/tests/graphstore_fixture_contract.rs                                | 9 +++++----
 src-tauri/tests/graphstore_migrations_contract.rs                             | 3 ++-
 src-tauri/tests/wu_0b_04_policyset_contract.rs                                | 2 +-
 src-tauri/tests/wu_0b_05_graphconfiguration_contract.rs                       | 3 ++-
 src-tauri/tests/wu_0b_06_graphworkspace_contract.rs                           | 3 ++-
 src-tauri/tests/wu_0b_07_graphnode_contract.rs                                | 3 ++-
 src-tauri/tests/wu_0b_09_evidenceartifact_contract.rs                         | 3 ++-
 14 files changed, 23 insertions(+), 14 deletions(-)
```

Categorized:
- **Module wiring (2 lines):** `src-tauri/src/contracts/mod.rs` adds `pub mod auditevent;` alphabetically; `src-tauri/src/graphstore/mod.rs` adds the same. Unavoidable.
- **Prior-WU contract test migration-list bumps (7 files):** every hunk is a literal-list bump from `[1,4,5,6,7,9]` to `[1,4,5,6,7,9,15]` and a runtime-table list that adds `"audit_events"` alphabetically (first position). No assertion shape changes.
- **Prior-WU fixture migration-list bumps (5 files):** `wu-0b-03/graphstore-fixture-shape.json`, `wu-0b-03/graphstore-pool-shape.json`, `wu-0b-03/no-side-effects-assertions.json`, `wu-0b-03/reset-preserves-migration-history.json`, `wu-0b-04/no-side-effects.json`. All are JSON-fixture mirrors of the same migration-list / runtime-table updates.

### WU-0B-15-MULTI-F02 — All untracked additions sit inside the WU-0B-15 boundary

`git ls-files --others --exclude-standard` reports the following paths, each mapping to a ticket boundary entry:

| Untracked file | Boundary |
| --- | --- |
| `product-strategy/contracts/wu-0b-15-auditevent.md` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-15/canonical-row.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-15/round-trip.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-15/decision-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-15/unknown-decision.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-15/output-with-input-invariants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-15/fk-rejections.json` | Test boundary. |
| `proposals/0b-15-wu-0b-15.md` | Pipeline artifact (proposer output). |
| `src-tauri/migrations/0b/15_auditevent.sql` | Code boundary. |
| `src-tauri/src/graphstore/auditevent.rs` | Code boundary. |
| `src-tauri/src/contracts/auditevent.rs` | Code boundary (re-export shim). |
| `src-tauri/tests/wu_0b_15_auditevent_contract.rs` | Code boundary. |

No stray files.

### WU-0B-15-MULTI-F03 — Prior-WU implementation files are not modified

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs src-tauri/src/graphstore/evidenceartifact.rs` is empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs src-tauri/src/contracts/evidenceartifact.rs` is empty. Predecessor types (`PolicySet`, `GraphConfiguration`, `GraphWorkspace`) are *imported* — `src-tauri/src/graphstore/auditevent.rs:9-13` — for use as `OpaqueId<T>` type markers and FK-target validators; nothing in those modules is edited.

### WU-0B-15-MULTI-F04 — `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `turbo.json`, `tauri.conf.json`, `lib.rs` all unchanged

`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` is empty. No new dependencies, no version bumps, no script changes, no allowlist edits, no command registration.

### WU-0B-15-MULTI-F05 — No cross-WU refactor bundled in

The diff introduces no rename, no signature change, no reformat, no trait extraction, and no helper-function extraction in any prior-WU file. The new `AuditEventRepo` implements the existing `GraphStoreRepo<AuditEvent>` trait from WU-0B-03 (`src-tauri/src/graphstore/auditevent.rs:224-250`) — the trait is *consumed*, not modified. Reuse of `validate_record_meta`, `JsonField`, `OpaqueId`, `RecordMeta`, `ActorRef`, and `GraphStoreError` is import-only (`:12-14`). The `impl_sqlite_text_enum!` macro is locally defined per-module (consistent with `graphnode.rs`, `graphconfiguration.rs`, `evidenceartifact.rs`) — no extraction into a shared module was attempted, which keeps the change set strictly additive.

### WU-0B-15-MULTI-F06 — `wu-0b-03` and `wu-0b-04` fixture mirror updates are JSON-only data follow-on, not behavioral edits

The five JSON-fixture diffs are direct mirrors of the migration-list / runtime-table updates that the contract tests assert against. Each one is a `[1,4,5,6,7,9]` → `[1,4,5,6,7,9,15]` literal bump or an alphabetically-inserted `"audit_events"` runtime-table entry. The graphstore_fixture_contract test at `tests/graphstore_fixture_contract.rs:163` asserts `rerun.skipped_versions == vec![1, 4, 5, 6, 7, 9, 15]` and passes — meaning the new fixture values match actual harness behavior. A clean separation: the implementer paid down the JSON-fixture data debt that the migration-list bump would otherwise have exposed as test failure. **Acceptable inside this WU because (a) it's fixture data only, no production code changed, and (b) leaving it inconsistent would have failed the test that passes today.**

### WU-0B-15-MULTI-F07 — Branch state is uncommitted but cleanly bounded

`git log main..HEAD --oneline` is empty — there is no commit on the branch yet. The reviewer instructions explicitly forbid committing or pushing, so this is the expected state at gate-review time. When the implementer commits, the staged set should be exactly the 26 paths enumerated by `git status` (14 modified + 12 untracked) — no other files appear in the working tree.

### Verification commands

| Command | Result |
| --- | --- |
| `git diff main --stat` | 14 files, 23 insertions, 14 deletions — all mechanical migration/runtime-list bumps + 2 `pub mod` lines. |
| `git diff main -- src-tauri/src/graphstore/{policyset,graphconfiguration,graphworkspace,graphnode,evidenceartifact}.rs` | empty. |
| `git diff main -- src-tauri/src/contracts/{policyset,graphconfiguration,graphworkspace,graphnode,evidenceartifact}.rs` | empty. |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty. |
| `git ls-files --others --exclude-standard` | 12 files, all inside the WU-0B-15 boundary or pipeline-artifact set. |
| `git log main..HEAD --oneline` | empty (uncommitted). |

## Verdict

**LOW.** The PR is single-concern. Tracked edits decompose to: 2 unavoidable `pub mod auditevent;` wiring lines, 7 prior-WU contract test files with literal migration-list / runtime-table bumps (no behavioral or assertion-shape changes), and 5 prior-WU fixture files with the same JSON-only migration-list / runtime-table mirror updates. All untracked additions map to ticket or pipeline boundaries. No prior-WU *implementation* file (`graphstore/*.rs` or `contracts/*.rs`) is modified — predecessor types are imported and reused, never edited. No `Cargo.toml` / `Cargo.lock` / `package.json` / `bun.lock` / `turbo.json` / `tauri.conf.json` / `lib.rs` churn. The fixture mirror updates are pure data debt the migration-list bump would otherwise have exposed as test failure; they are the right thing to fix here, contained inside this PR's strict scope.
