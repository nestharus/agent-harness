# WU-0A-08 — Multi-Concern PR Risk Review

**Gate:** Phase 4 multi-concern.
**Severity:** LOW.

## Question

Does this PR cleanly map to a single concern (WU-0A-08: SubscribeWorkspaceEvents IPC command + the allowlist-update of two pre-existing scaffold assertions), or does it bundle multiple WUs / unbounded refactors / Cargo / `tauri.conf.json` churn?

## Findings

### WU-0A-08-MULTI-F01 — `git diff main` matches the documented WU-0A-08 surface

`git diff main --name-only` lists exactly the files this WU is allowed to touch:

```
src-tauri/src/app_state.rs                       (M — adds subscription registry to existing EventBusHandle)
src-tauri/src/commands/mod.rs                    (M — single-line `pub mod subscribe_workspace_events;`)
src-tauri/src/contracts/mod.rs                   (M — single-line `pub mod subscribe_workspace_events;`)
src-tauri/src/events/mod.rs                      (M — single-line `pub mod subscription;`)
src-tauri/src/lib.rs                             (M — invoke_handler + managed state + allowlist)
src-tauri/tests/harness_app_state_contract.rs    (M — count/list assertion update only)
src-tauri/tests/scaffold_contract.rs             (M — `tauri_bootstrap_is_inert_and_command_free` allowlist update)
```

Untracked WU-0A-08 files:

```
product-strategy/contracts/fixtures/wu-0a-08/        (11 fixture JSON files)
product-strategy/contracts/wu-0a-08-subscribe-workspace-events.md
proposals/08-wu-0a-08.md
src-tauri/src/commands/subscribe_workspace_events.rs
src-tauri/src/contracts/subscribe_workspace_events.rs
src-tauri/src/events/subscription.rs
src-tauri/tests/subscribe_workspace_events_contract.rs
src/contracts/subscribe-workspace-events.ts
src/test/subscribe-workspace-events.test.ts
```

These are exactly the files the ticket lists in the code and test boundaries (`plans/tickets/phase-0a/WU-0A-08.md`). **Single concern.**

### WU-0A-08-MULTI-F02 — No incidental edits to WU-0A-02/03/05/06/07/09/11/12 implementation files

Verified by inspecting `git diff main`:

- `src-tauri/src/app_state.rs` (WU-0A-04) — only additive: imports for `HashMap`, `Arc`, `Mutex`, `EventTopic`, `Subscription`, `SubscriptionMetadata`, `WorkspaceEventChannel`; new `subscriptions` field on `EventBusHandle`; new `register_subscription`, `subscription_metadata`, `subscription_count` methods; new manual `Debug` impl (replaces the previously-derived one because `Subscription` is not `Debug`-deriveable). No existing WU-0A-04 method signatures are changed.
- `src-tauri/src/contracts/mod.rs` (WU-0A-02..07) — single-line module declaration insert; no edits to existing lines.
- `src-tauri/src/commands/mod.rs` (WU-0A-07-adjacent) — replaces a placeholder comment with one `pub mod` declaration. No symbols touched.
- `src-tauri/src/events/mod.rs` (WU-0A-05/06) — single-line module declaration insert.
- `src-tauri/src/lib.rs` — adds the invoke handler / managed state / allowlist; the existing `pub mod` declarations and the `tauri::Builder::default()...setup(|_app| Ok(()))...run(...)` skeleton remain.
- WU-0A-09 (`trace_context*`), WU-0A-11 (`pane_id`), WU-0A-12 (`shell_region_state`) sources are untouched (`git diff main --stat` shows no entries for those paths).
- Test files for prior WUs that are *not* WU-0A-04 / scaffold are untouched.

### WU-0A-08-MULTI-F03 — Two scaffold-contract assertion updates are minimal and coherent

The ticket explicitly authorizes this slice: WU-0A-08 is the first WU to register a real Tauri command, so the prior "Phase 0A registers zero commands" assertions cannot survive. The two updated assertion blocks are:

- `src-tauri/tests/scaffold_contract.rs:136-184` (`tauri_bootstrap_is_inert_and_command_free`) — comment retitled, allowlist asserted, value-slice forbidden list added, empty-handler check tightened to `generate_handler![]`. No unrelated tests in this file are touched (`workspace_manifests_match_phase_0a_contract_fixtures` and `cargo_manifest_declares_phase_0a_runtime_dependencies_without_migrations` are unchanged per the diff).
- `src-tauri/tests/harness_app_state_contract.rs:139-146` — the single `assert_eq!(registered_command_count(), 0)` is replaced with two `assert_eq!`s anchoring to the allowlist. No other test in this file is touched (the four `tokio::test` cases for bootstrap errors and the `bootstrap_error_variants_round_trip…` test all retain their original bodies per the diff).

Both are required and bounded by the WU's own scope. **No drive-by edits.**

### WU-0A-08-MULTI-F04 — No Cargo / `tauri.conf.json` / dependency churn

```
$ git diff main --stat -- src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json package.json bun.lock
(empty)
```

No new crate dependencies, no Tauri capability-allowlist edit, no npm/bun dependency change. Tauri v2 command registration lives in the `tauri::Builder` `invoke_handler` macro inside `lib.rs` (`src-tauri/src/lib.rs:21-23`); it does not require a `tauri.conf.json` allowlist entry, and none was added. **No incidental config drift.**

### WU-0A-08-MULTI-F05 — Diff size is small and explicable

`git diff main --stat`:

```
 src-tauri/src/app_state.rs                    | 55 +++++++++++++++++++++++-
 src-tauri/src/commands/mod.rs                 |  2 +-
 src-tauri/src/contracts/mod.rs                |  1 +
 src-tauri/src/events/mod.rs                   |  1 +
 src-tauri/src/lib.rs                          | 61 +++++++++++++++++++++++++--
 src-tauri/tests/harness_app_state_contract.rs |  9 +++-
 src-tauri/tests/scaffold_contract.rs          | 25 +++++++++--
 7 files changed, 144 insertions(+), 10 deletions(-)
```

Plus the untracked WU-0A-08 files (proposal, contract, fixtures, Rust + TS contract & tests, command, subscription registry, contract module). Every line is justified by either (a) wiring the new modules into existing index files, (b) registering the command and allowlist, (c) extending `EventBusHandle` with the subscription registry the WU contract requires, or (d) anchoring the two pre-existing scaffold assertions to the allowlist. Nothing in the diff is opportunistic refactoring or unrelated cleanup.

## Verdict

**LOW.** Single-concern PR. Touched files match the WU-0A-08 ticket's code/test boundary plus the two pre-existing scaffold-contract assertions whose update the proposal explicitly mandates. No edits to WU-0A-02/03/05/06/07/09/11/12 implementation files, no Cargo / `tauri.conf.json` / lockfile drift, no incidental refactors. Diff size matches the documented surface.
