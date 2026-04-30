# WU-0A-04 — Supported-Surface Risk Review

**Gate:** Phase 8 supported-surface.
**Severity:** LOW.

## Question

Does deployment-mode + customer-cohort + adjacent-public-paths analysis match the WU-0A-04 supported-surface declaration (`proposals/04-wu-0a-04.md:80-88`)? What is the blast radius for adjacent paths (the WU-0A-01 inert scaffold, WU-0A-02 settings DTO/loader, WU-0A-03 storage layout DTO/derivation, WU-0A-05 event-topic taxonomy, WU-0A-06 IpcEvent envelope, WU-0A-09 trace-context schema, WU-0A-11 pane-id taxonomy, WU-0A-12 shell-region-state)? Migration / rollback / observability story? Does the implementation register any Tauri command or modify the capability set?

## Supported-surface contract (proposal `proposals/04-wu-0a-04.md:80-88`)

- Deployment mode: local Phase 0A Tauri backend library and desktop scaffold.
- Customer cohort: internal contract consumers and future Phase 0A slices importing app state.
- Public paths: Rust library consumers of `app_state` and `contracts::harness_app_state`.
- Adjacent paths: settings validation, storage-layout validation, event/trace DTO modules, sqlx dependency use, command-free Tauri scaffold invariant.
- Migration path: additive Rust module, additive contract module, fixtures, tests.
- Rollback path: remove the added app-state module, contract module, fixtures, tests, proposal, and module declarations.
- Observability: Rust contract tests expose happy-path handle creation, each bootstrap failure variant, serde round-trip, and the no-command invariant.

## Findings

### Public surface matches the declaration

- Rust: `src-tauri/src/lib.rs:1` adds `pub mod app_state;`; `src-tauri/src/contracts/mod.rs:2` adds `pub mod harness_app_state;`. Public symbols introduced are limited to:
  - `agent_harness_lib::contracts::harness_app_state::BootstrapError` (`src-tauri/src/contracts/harness_app_state.rs:3-10`).
  - `agent_harness_lib::app_state::{HarnessAppState, EventBusHandle, TraceContextFactory, EventBusFactory, DefaultEventBusFactory, TraceContextFactoryInitializer, DefaultTraceContextFactoryInitializer, init_harness_app_state, init_harness_app_state_with_factories}` (`src-tauri/src/app_state.rs:18-129`).
  - The `EVENT_BUS_CAPACITY` constant (`app_state.rs:16`) is private; `validate_settings`, `validate_storage_layout_pair`, and `open_sqlite_pool` (`app_state.rs:157-197`) are private. Consumers can only access the container through the public types and the two initializer functions.
- TypeScript: no new TS module added — `BootstrapError` is Rust-only because the WU contract test boundary is Rust-only (`product-strategy/contracts/wu-0a-04-harness-app-state.md:74-76`). The contract scopes the public path to "Rust library consumers" only (`proposals/04-wu-0a-04.md:84`); no TS contract was promised, so none is missing.
- The exposed surface matches "Rust library consumers of `app_state` and `contracts::harness_app_state`" exactly.

### Tauri command-free invariant preserved

- `src-tauri/src/lib.rs:8-14` is unchanged in shape: `tauri::Builder::default().setup(|_app| Ok(())).run(...)`. Zero `tauri::generate_handler!` invocations, zero `app.manage(...)` invocations, zero `invoke_handler(...)` calls. The `HarnessAppState` is constructed only inside the contract test, never inserted into the Tauri builder.
- `agent_harness_lib::registered_command_count()` still returns `0` (`lib.rs:16-18`); the WU-0A-01 assertion `phase_0a_registers_no_value_slice_commands` still passes (`lib.rs:20-28`); the happy-path WU-0A-04 contract test re-asserts the same invariant after constructing `HarnessAppState` (`src-tauri/tests/harness_app_state_contract.rs:139`).
- `src-tauri/tauri.conf.json` is unchanged (`git diff main` shows no diff). The bundle config, window config, and CSP are byte-identical to WU-0A-01.
- `src-tauri/capabilities/default.json` is unchanged. Permissions still consist of only `core:default` (`capabilities/default.json:8-10`); no `sql:default`, no `event:allow-emit`, no other capability strings added.
- No entries added to a Tauri command allowlist (the file does not exist in this scaffold; no allowlist file added).

### Adjacent paths remain unchanged

- WU-0A-01 inert scaffold:
  - `lib.rs:8-14` shape unchanged (Tauri builder with empty setup), `registered_command_count()` unchanged, `phase_0a_registers_no_value_slice_commands` unchanged. The only edit to `lib.rs` is the `pub mod app_state;` addition at line 1.
  - No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`. The visible application root is unchanged.
  - `src-tauri/tests/scaffold_contract.rs` still passes (verified locally — `tauri_bootstrap_is_inert_and_command_free`, `cargo_manifest_declares_phase_0a_runtime_dependencies_without_migrations`, `workspace_manifests_match_phase_0a_contract_fixtures` all pass).
- WU-0A-02 settings DTO/loader:
  - No edits to `src-tauri/src/settings.rs` or `src-tauri/src/contracts/harness_settings.rs`. `app_state.rs:9` only *imports* `HarnessSettings` to satisfy the `init_harness_app_state` input type. Loader behavior is untouched.
  - No edits to `src/contracts/harness-settings.ts` or any WU-0A-02 fixture.
- WU-0A-03 storage layout DTO / derivation:
  - No edits to `src-tauri/src/storage.rs` or `src-tauri/src/contracts/local_storage_layout.rs`. `app_state.rs:11, 13` import `LocalStorageLayout` and `validate_local_storage_layout`; the validator is *consumed*, not modified, so the lexical-containment behavior remains exactly what WU-0A-03 ships.
  - No edits to `src/contracts/local-storage-layout.ts`.
- WU-0A-05 event-topic taxonomy: `src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src/contracts/event-topic.ts` untouched.
- WU-0A-06 IpcEvent envelope: `src-tauri/src/contracts/ipc_event.rs` and the TS contract untouched. `app_state.rs:10` only imports `IpcEvent` so the broadcast channel can carry `IpcEvent<serde_json::Value>` payloads.
- WU-0A-09 trace context: `src-tauri/src/contracts/trace_context.rs` and `src-tauri/src/tracing/trace_context.rs` untouched. `app_state.rs:12, 14` import `TraceContext`, `TraceContextError`, and `create_trace_context`; the factory wrapper forwards verbatim without re-implementing any trace logic.
- WU-0A-11 pane-id and WU-0A-12 shell-region-state: completely untouched (no imports, no edits).

### Dependency surface — bounded change

- `Cargo.toml` change is bounded to flipping the Tokio `sync` feature flag on (`src-tauri/Cargo.toml:20`):

  ```diff
  -tokio = { version = "1.52.1", features = ["rt-multi-thread", "macros"] }
  +tokio = { version = "1.52.1", features = ["rt-multi-thread", "macros", "sync"] }
  ```

  No new top-level dependencies; sqlx, serde, serde_json, tauri, and tauri-build versions are unchanged. The `sync` feature is required because `tokio::sync::broadcast` (the type underlying `EventBusHandle`) is gated behind that feature in tokio. `Cargo.lock` is unchanged on the branch (`git diff main -- src-tauri/Cargo.lock` is empty), confirming no transitive crate added or upgraded.
- No new dev-dependencies in `Cargo.toml`.
- No edits to root `package.json`, `turbo.json`, `bun.lockb`, or any TS workspace package — the test boundary for WU-0A-04 is Rust-only.

### Blast radius

- Greenfield Phase 0A: no prior runtime users of `agent_harness_lib::app_state` or `agent_harness_lib::contracts::harness_app_state`. The new public API is purely additive; nothing pre-existing depends on it.
- The `app_state` module is a new top-level crate-internal namespace; reordering or renaming inside it cannot affect the WU-0A-01 / WU-0A-02 / WU-0A-03 / WU-0A-05 / WU-0A-06 / WU-0A-09 / WU-0A-11 / WU-0A-12 namespaces.
- The fixtures live under `product-strategy/contracts/fixtures/wu-0a-04/`, namespaced by WU. They cannot collide with other WU fixture sets.
- The Tokio `sync` feature flip is additive: features are unioned across consumers, so other workspace crates already requiring `sync` (none today) would not be affected, and crates not using broadcast/mutex/oneshot would only pay the compile cost (no runtime cost).
- No transitive dep changes (`Cargo.lock` unchanged), no feature flags toggled in `tauri` or `sqlx`, no new bundle targets, no Tauri permissions added, no capability strings added.

### Phase boundary preserved

- No GraphStore migration files added (`src-tauri/migrations/` still does not exist). `open_sqlite_pool` opens with `create_if_missing(true)` and never invokes `sqlx::migrate!()` or any DDL.
- No provider crates added, no provider env-var reads, no provider config schema.
- No `agents` invocation paths: no `tokio::process::Command`, no `std::process::Command`, no shell-out helpers in `app_state.rs`.
- No new Tauri capability strings, no plugin registrations.
- The proposal's supported-surface track ("local Phase 0A Tauri backend library and desktop scaffold", "internal contract consumers and future Phase 0A slices importing app state") matches the implementation precisely.

### Migration / rollback path

- Migration: none. No persistent on-disk state created by `init_harness_app_state` itself other than the SQLite file the consumer points it at (and which is created by sqlx with `create_if_missing(true)` only when an actual call is made — the contract test cleans up its temp directory after every run).
- Rollback: delete `src-tauri/src/app_state.rs`, `src-tauri/src/contracts/harness_app_state.rs`, `src-tauri/tests/harness_app_state_contract.rs`, `proposals/04-wu-0a-04.md`, `product-strategy/contracts/wu-0a-04-harness-app-state.md`, and `product-strategy/contracts/fixtures/wu-0a-04/`; revert the `pub mod app_state;` line in `src-tauri/src/lib.rs:1`, the `pub mod harness_app_state;` line in `src-tauri/src/contracts/mod.rs:2`, and the Tokio `sync` feature in `src-tauri/Cargo.toml:20`. No stateful reconciliation needed — nothing depends on the new API yet.

### Observability

- Happy-path handle creation: `init_harness_app_state_creates_all_handles_and_preserves_paths` (`harness_app_state_contract.rs:103-142`) asserts all five fields are present and the documented identity values (`workspace_id`, `storage_root`, `database_path`) are preserved; it also exercises the event bus subscribe path, executes a real SQL query, and creates a real trace context.
- Each `BootstrapError` variant: one dedicated test per variant (`…contract.rs:144-229`).
- Serde round-trip and unknown-variant rejection: `…contract.rs:231-264`.
- No-command invariant: re-asserted inside the WU-0A-04 happy-path test (`…contract.rs:139`); also independently asserted by WU-0A-01's `phase_0a_registers_no_value_slice_commands` (`lib.rs:20-28`) and WU-0A-01's `tauri_bootstrap_is_inert_and_command_free` (`scaffold_contract.rs`).

### Boundary observations (informational)

- INFO: `EventBusHandle::subscribe()` and `receiver_count()` (`app_state.rs:37-44`) are public methods on the new handle. Future Phase 0A WUs (WU-0A-08 and later) will consume them; documenting this as part of the supported surface in any later WU's proposal would be worthwhile.
- INFO: `init_harness_app_state_with_factories` is a public symbol intended for tests, not production. The contract documents the seam (`product-strategy/contracts/wu-0a-04-harness-app-state.md:41-47`); the proposal documents it (`proposals/04-wu-0a-04.md:68-70`). Future WUs that add a "production-only" lint should consider routing this seam through a `#[doc(hidden)]` attribute or moving it behind a `cfg(any(test, feature = "test-support"))` gate. For Phase 0A this is appropriate as-is — the seam is Rust-only, the failing factory implementations live in the test crate, and there is no IPC-level surface that could be reached from a Tauri command.

## Verdict

**LOW.** Public surface is exactly the declared library symbols (`HarnessAppState`, `EventBusHandle`, `TraceContextFactory`, the two factory traits + their default implementations, `init_harness_app_state`, `init_harness_app_state_with_factories`, `BootstrapError`). The WU-0A-01 inert Tauri scaffold and the WU-0A-02 / 03 / 05 / 06 / 09 / 11 / 12 modules are all untouched in shape and behavior. `tauri.conf.json` and `capabilities/default.json` are byte-identical; no Tauri command allowlist entries; `registered_command_count() == 0` invariant preserved and re-asserted by the new contract test. Cargo dependency surface change is bounded to flipping the Tokio `sync` feature on (required for `tokio::sync::broadcast`); `Cargo.lock` is unchanged. No GraphStore migration files, no provider credentials, no `agents` invocation paths. Migration is additive only, rollback is a clean revert, and every acceptance criterion is observable via at least one fixture-pinned test.
