# WU-0A-04 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-04 `HarnessAppState` container + `init_harness_app_state` / `init_harness_app_state_with_factories` entrypoints + `BootstrapError` taxonomy + Rust-only fixtures and contract test — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-04.md` "Handoff notes") explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Branch composition

`git diff main --stat` reports three modified tracked files plus six new (untracked) paths:

```
src-tauri/Cargo.toml           | 2 +-
src-tauri/src/contracts/mod.rs | 1 +
src-tauri/src/lib.rs           | 1 +
3 files changed, 3 insertions(+), 1 deletion(-)
```

Untracked additions (`git status`):

- `proposals/04-wu-0a-04.md`
- `product-strategy/contracts/wu-0a-04-harness-app-state.md`
- `product-strategy/contracts/fixtures/wu-0a-04/` (8 JSON fixtures)
- `src-tauri/src/app_state.rs`
- `src-tauri/src/contracts/harness_app_state.rs`
- `src-tauri/tests/harness_app_state_contract.rs`

`Cargo.lock` is untouched (`git diff main -- src-tauri/Cargo.lock` is empty).

## Concern enumeration

Files added/modified on the branch (relative to the `main` parent at WU-0A-12) group cleanly under one logical concern: "ship the Phase 0A `HarnessAppState` backend container + `init_harness_app_state(...)` entrypoint pair + `BootstrapError` taxonomy with Rust-only fixture-backed contract tests."

| Group | Files | Why required for this single concern |
|---|---|---|
| Rust contract type | `src-tauri/src/contracts/harness_app_state.rs`, `src-tauri/src/contracts/mod.rs` (one-line addition at line 2) | The `BootstrapError` enum that the WU contract owns (`product-strategy/contracts/wu-0a-04-harness-app-state.md:51-57`). |
| Rust app-state module | `src-tauri/src/app_state.rs`, `src-tauri/src/lib.rs` (one-line addition at line 1) | `HarnessAppState`, `EventBusHandle`, `TraceContextFactory`, the two factory traits + defaults, `init_harness_app_state(...)`, and `init_harness_app_state_with_factories(...)` per WU contract container + initialization sections (`…app-state.md:9-47`). |
| Tokio feature flip | `src-tauri/Cargo.toml` (one-token addition: `"sync"`) | `tokio::sync::broadcast::Sender` is the documented backing type for `EventBusHandle` (`proposals/04-wu-0a-04.md:17-21`); the `sync` feature gate is required to pull it in. |
| Rust contract test | `src-tauri/tests/harness_app_state_contract.rs` | Fixture-backed coverage of all 8 ticket acceptance criteria: happy path with handle preservation, every `BootstrapError` variant (settings/layout/db/event/trace), and serde round-trip + unknown rejection. |
| Fixtures | `product-strategy/contracts/fixtures/wu-0a-04/{happy-path,error-settings-invalid,error-storage-layout-invalid,error-database-open-failed,error-event-bus-init-failed,error-trace-init-failed,bootstrap-errors,invalid-bootstrap-error}.json` | Canonical inputs/outputs for the success path and one fixture per documented error variant, plus the canonical taxonomy and unknown-variant rejection fixture (proposal test-intent at `proposals/04-wu-0a-04.md:97-105`). |
| Contract spec + proposal | `product-strategy/contracts/wu-0a-04-harness-app-state.md`, `proposals/04-wu-0a-04.md` | Phase-3 proposal and WU-owned contract document — required workflow artifacts. |

## Severability check — can any group ship independently?

- **Rust contract type alone** (without `app_state.rs`): does not satisfy ticket criteria 1-7 (no entrypoint to emit any `BootstrapError`). The error enum has no observable use without the initializer.
- **Rust app-state module alone** (without the contract type): does not compile — `init_harness_app_state` returns `Result<HarnessAppState, BootstrapError>`.
- **Tokio `sync` feature flip alone**: would compile in isolation but be dead weight. The feature is consumed only by `app_state.rs:6`'s `use tokio::sync::broadcast;` import.
- **Fixtures alone**: cannot ship — they have no consumer; the contract test loader (`harness_app_state_contract.rs:36-44`) references them by relative path.
- **Tests alone**: cannot ship — they reference symbols (`agent_harness_lib::app_state::*`, `agent_harness_lib::contracts::harness_app_state::BootstrapError`) that only exist with the rest of the change.
- **Contract spec / proposal alone**: required workflow artifacts; not separable from the implementation they describe.
- **`lib.rs` / `contracts/mod.rs` wirings**: cannot ship without the modules they declare; would break the build (orphan `pub mod` of a missing file).

No group is independently shippable.

## Cross-WU contamination check

- No file references any later WU (WU-0A-07, WU-0A-08, WU-0A-10, WU-0A-13, WU-0A-14a/b, WU-0A-15) or any later phase (0B, 0C, 1+).
- No GraphStore migration files (`src-tauri/migrations/` is still absent), no provider crates added, no subprocess code (no `std::process` / `tokio::process` imports anywhere on the branch).
- No `tauri::generate_handler!`, no `app.manage(...)`, no IPC commands. The WU-0A-01 scaffold contract assertion `phase_0a_registers_no_value_slice_commands` still passes (`src-tauri/src/lib.rs:20-28`); `registered_command_count() == 0` still holds (`lib.rs:16-18`); the WU-0A-04 happy-path test re-asserts the same invariant after constructing `HarnessAppState` (`harness_app_state_contract.rs:139`).
- WU-0A-02 settings DTO/loader untouched: no edits to `src-tauri/src/settings.rs` or `src-tauri/src/contracts/harness_settings.rs`. `app_state.rs:9` only *imports* `HarnessSettings`. No edits to `src/contracts/harness-settings.ts` or any WU-0A-02 fixture under `product-strategy/contracts/fixtures/wu-0a-02/`.
- WU-0A-03 storage layout DTO / derivation untouched: no edits to `src-tauri/src/storage.rs` or `src-tauri/src/contracts/local_storage_layout.rs`. `app_state.rs:11, 13` only import `LocalStorageLayout` and `validate_local_storage_layout`; the validator is consumed, not modified.
- WU-0A-05 (event topic), WU-0A-06 (IpcEvent), WU-0A-09 (trace context), WU-0A-11 (pane id), WU-0A-12 (shell-region-state) all untouched: no edits to `src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src-tauri/src/contracts/ipc_event.rs`, `src-tauri/src/contracts/trace_context.rs`, `src-tauri/src/tracing/`, the pane-id module, or the shell-region-state module. `app_state.rs:10, 12, 14` only import `IpcEvent`, `TraceContext`, `TraceContextError`, `create_trace_context` from those modules — pure consumers, no edits.
- No edits to `src-tauri/tauri.conf.json` or `src-tauri/capabilities/default.json` — Tauri command allowlist and capability set are unchanged.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `package.json`, `turbo.json`, or `bun.lockb`. The user-facing shell, build pipeline, and JS workspace are untouched.
- `proposals/` adds only `04-wu-0a-04.md`; no other proposal snuck in. `product-strategy/contracts/fixtures/` adds only `wu-0a-04/`; no cross-WU fixture sets touched.

## Cargo.toml change is bounded

The `Cargo.toml` diff is the smallest possible expression of the dependency requirement:

```diff
-tokio = { version = "1.52.1", features = ["rt-multi-thread", "macros"] }
+tokio = { version = "1.52.1", features = ["rt-multi-thread", "macros", "sync"] }
```

- No new top-level dependencies (sqlx, serde, serde_json, tauri, tauri-build versions unchanged).
- No new dev-dependencies.
- No version bumps.
- The `sync` feature is genuinely required: `app_state.rs:6` is `use tokio::sync::broadcast;` and `tokio::sync::broadcast` is gated behind the `sync` feature. The proposal records the choice (`proposals/04-wu-0a-04.md:17-21`) — the broadcast sender is the documented backing type for `EventBusHandle`.
- `Cargo.lock` is unchanged on the branch — no transitive crate added or upgraded. The `sync` feature is a feature-flag flip on an already-resolved dependency.

## Observations

- The split between `contracts/harness_app_state.rs` (error type) and `app_state.rs` (container + entrypoints + factory traits) mirrors the WU-0A-02 split (`contracts/harness_settings.rs` + `settings.rs`) and the WU-0A-03 split (`contracts/local_storage_layout.rs` + `storage.rs`). Two files keep the change scoped to one concern instead of inventing a shared module.
- The contract test uses a single shared fixture-loader (`AppStateCase`, `read_case`, `prepare_filesystem` at `harness_app_state_contract.rs:20-101`) and one test per acceptance criterion. Adding a future error variant only requires updating the fixture set + the `expected_names` array (`…contract.rs:235-241`) — no new variant could be silently absent because `bootstrap-errors.json` pins the canonical taxonomy and is asserted equal to the in-test list.
- The two-arity initializer pattern (`init_harness_app_state` → `init_harness_app_state_with_factories`) keeps production callers free of test-only injection without forcing a `#[cfg(test)]` boundary; both arities run the same validation and SQLite path, so the test seam cannot drift from production behavior.

## Verdict

**LOW.** This is a single-concern PR for WU-0A-04. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, no group serves another WU, and there is no contamination from WU-0A-01 / WU-0A-02 / WU-0A-03 / WU-0A-05 / WU-0A-06 / WU-0A-09 / WU-0A-11 / WU-0A-12 (all untouched in shape and behavior — only consumed via imports) or any later WU (no GraphStore, providers, optimizers, workers, questions, recovery, budget, audit, IPC commands, registered Tauri handlers, capability changes, subprocess code, or render/policy/transcript work). The Cargo.toml change is bounded to a single feature flag flip on an already-present `tokio` dependency, required by the documented `EventBusHandle` design; `Cargo.lock` is unchanged. The single-concern handoff note is honored.
