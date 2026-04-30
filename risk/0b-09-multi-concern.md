# WU-0B-09 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern PR.
**Severity:** LOW.

## Question

Is the change set strictly single-concern (only WU-0B-09 — `EvidenceArtifact` row, table, repository, fixtures, and the unavoidable migration-list bumps in prior-WU contract tests)? Are prior-WU implementation files modified, are unrelated refactors bundled in, or is `Cargo.toml` / `package.json` / `tauri.conf.json` / `src-tauri/src/lib.rs` touched?

## Findings

### WU-0B-09-MULTI-F01 — Tracked diff against `main` is 21 insertions / 13 deletions across 13 files; every line is mechanical

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
 13 files changed, 21 insertions(+), 13 deletions(-)
```

Categorized:
- **Module wiring (2 lines):** `src-tauri/src/contracts/mod.rs` adds `pub mod evidenceartifact;` alphabetically; `src-tauri/src/graphstore/mod.rs` adds the same. Unavoidable.
- **Prior-WU contract test migration-list bumps (6 files):** every hunk is a literal-list bump from `[1,4,5,6,7]` to `[1,4,5,6,7,9]` and a runtime-table list that adds `"evidence_artifacts"` alphabetically. No assertion shape changes. See SUPPORTED-SURFACE-F04 for per-file detail.
- **Prior-WU fixture migration-list bumps (5 files):** `wu-0b-03/graphstore-fixture-shape.json`, `wu-0b-03/graphstore-pool-shape.json`, `wu-0b-03/no-side-effects-assertions.json`, `wu-0b-03/reset-preserves-migration-history.json`, `wu-0b-04/no-side-effects.json`. All are JSON-fixture mirrors of the same migration-list / runtime-table updates.

### WU-0B-09-MULTI-F02 — All untracked additions sit inside the WU-0B-09 boundary

`git ls-files --others --exclude-standard` reports 15 paths, every one of which maps to a ticket boundary entry:

| Untracked file | Boundary |
| --- | --- |
| `product-strategy/contracts/wu-0b-09-evidenceartifact.md` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/canonical-row.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/round-trip.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/source-type-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/tool-protocol-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/capture-state-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/privilege-origin-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/unknown-variants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/content-hash-invariants.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/path-escape-attempts.json` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0b-09/transaction-rollback-fk.json` | Test boundary. |
| `src-tauri/migrations/0b/09_evidenceartifact.sql` | Code boundary. |
| `src-tauri/src/graphstore/evidenceartifact.rs` | Code boundary. |
| `src-tauri/src/contracts/evidenceartifact.rs` | Code boundary (re-export shim). |
| `src-tauri/tests/wu_0b_09_evidenceartifact_contract.rs` | Code boundary. |

No stray files.

### WU-0B-09-MULTI-F03 — Prior-WU implementation files are not modified

`git diff main -- src-tauri/src/graphstore/policyset.rs src-tauri/src/graphstore/graphconfiguration.rs src-tauri/src/graphstore/graphworkspace.rs src-tauri/src/graphstore/graphnode.rs` is empty. `git diff main -- src-tauri/src/contracts/policyset.rs src-tauri/src/contracts/graphconfiguration.rs src-tauri/src/contracts/graphworkspace.rs src-tauri/src/contracts/graphnode.rs` is empty. The `PrivilegeOrigin` reuse is achieved purely by *importing* the existing definition from `crate::graphstore::graphnode` (`src-tauri/src/graphstore/evidenceartifact.rs:12`) and re-exporting it through the WU-0B-09 contract module (`src-tauri/src/contracts/evidenceartifact.rs:4`); no edit to `graphnode.rs` is needed.

### WU-0B-09-MULTI-F04 — `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `turbo.json`, `tauri.conf.json`, `lib.rs` all unchanged

`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` is empty. No new dependencies, no version bumps, no script changes, no allowlist edits, no command registration.

### WU-0B-09-MULTI-F05 — No cross-WU refactor bundled in

The diff introduces no rename, no signature change, no reformat, no trait extraction, and no helper-function extraction in any prior-WU file. The new `EvidenceArtifactRepo` implements the existing `GraphStoreRepo<EvidenceArtifact>` trait from WU-0B-03 (`src-tauri/src/graphstore/evidenceartifact.rs:237-267`) — the trait is *consumed*, not modified. Reuse of `validate_record_meta`, `JsonField`, `OpaqueId`, `RecordMeta`, `ActorRef`, and `GraphStoreError` is import-only (`:14-16`).

### WU-0B-09-MULTI-F06 — `wu-0b-03/reset-preserves-migration-history.json` correction is a fixture-only data-debt update, not a behavioral edit

The reset-preserves-migration-history fixture was originally written at WU-0B-03 with `skipped_versions: [1]` (verified via `git log` on that file: the only commit touching it on `main` is `d15c13e WU-0B-03`). When WU-0B-04/05/06/07 added migrations 4/5/6/7, those WUs updated the *other* WU-0B-03 fixtures (`graphstore-fixture-shape.json`, `graphstore-pool-shape.json`) but did not update this one — the file fell out of sync.

WU-0B-09's edit (`[1]` → `[1, 4, 5, 6, 7, 9]`) brings the fixture to truth: after `reset_graphstore_fixture` and a re-run of `run_migrations`, the harness will see migrations 1, 4, 5, 6, 7, 9 as already-applied and re-emit them as `skipped_versions`. The diff is JSON-only; no code change accompanies it. The graphstore_fixture_contract test at `tests/graphstore_fixture_contract.rs:163` asserts `rerun.skipped_versions == vec![1, 4, 5, 6, 7, 9]` and passes — meaning the new fixture value matches actual harness behavior. A clean separation: the implementer paid down a documentation-only debt that the fixture update was already exposing through test failure on bumping the migration list. **Acceptable inside this WU because (a) it's fixture data only, no production code changed, and (b) leaving it inconsistent would have failed the test that passes today.**

### WU-0B-09-MULTI-F07 — Branch state is uncommitted but cleanly bounded

`git log main..HEAD --oneline` is empty — there is no commit on the branch yet. The reviewer instructions explicitly forbid committing or pushing, so this is the expected state at gate-review time. When the implementer commits, the staged set should be exactly the 28 paths enumerated by `git status` (13 modified + 15 untracked) — no other files appear in the working tree.

### Verification commands

| Command | Result |
| --- | --- |
| `git diff main --stat` | 13 files, 21 insertions, 13 deletions — all mechanical migration/runtime-list bumps + 2 `pub mod` lines. |
| `git diff main -- src-tauri/src/graphstore/{policyset,graphconfiguration,graphworkspace,graphnode}.rs` | empty. |
| `git diff main -- src-tauri/src/contracts/{policyset,graphconfiguration,graphworkspace,graphnode}.rs` | empty. |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock turbo.json src-tauri/tauri.conf.json src-tauri/src/lib.rs` | empty. |
| `git ls-files --others --exclude-standard` | 15 files, all inside the WU-0B-09 boundary. |
| `git log main..HEAD --oneline` | empty (uncommitted). |

## Verdict

**LOW.** The PR is single-concern. Tracked edits decompose to: 2 unavoidable `pub mod evidenceartifact;` wiring lines, 6 prior-WU contract test files with literal migration-list / runtime-table bumps (no behavioral or assertion-shape changes), and 5 prior-WU fixture files with the same JSON-only migration-list / runtime-table mirror updates. All 15 untracked additions map to ticket boundaries. No prior-WU *implementation* file (`graphstore/*.rs` or `contracts/*.rs`) is modified — `PrivilegeOrigin` reuse is achieved purely by import + re-export. No `Cargo.toml` / `Cargo.lock` / `package.json` / `bun.lock` / `turbo.json` / `tauri.conf.json` / `lib.rs` churn. The `wu-0b-03/reset-preserves-migration-history.json` correction (`[1]` → `[1,4,5,6,7,9]`) is a fixture-only data-debt repair that the migration-list bump would otherwise have exposed as test failure; it's the right thing to fix here, contained inside this PR's strict scope.
