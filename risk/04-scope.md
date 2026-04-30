# WU-0A-04 — Scope Risk Review

**Gate:** Phase 8 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-04 boundary (the `HarnessAppState` 5-field container, `init_harness_app_state(settings, storage_layout)` entrypoint, the documented `BootstrapError` taxonomy with 5 variants, the test-seam variant `init_harness_app_state_with_factories(...)`, and bilingual / Rust-only fixtures), or does it bleed into GraphStore migrations, IPC command registration, app-managed Tauri state, provider configuration, agent subprocess invocation, or any Phase 0B/0C behavior the proposal anti-scope reserves for later WUs (`proposals/04-wu-0a-04.md:73-79`)?

## Findings

### In-scope, confirmed

- `HarnessAppState` declares exactly the five documented fields with the documented types: `settings: HarnessSettings`, `storage_layout: LocalStorageLayout`, `event_bus: EventBusHandle`, `db: SqlitePool`, `trace_context_factory: TraceContextFactory` (`src-tauri/src/app_state.rs:18-25`). Matches contract container clause (`product-strategy/contracts/wu-0a-04-harness-app-state.md:9-17`) and ticket schema (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-04.md` lines 11-17 of the contract block).
- `init_harness_app_state(settings, storage_layout)` signature matches the contract verbatim (`app_state.rs:118-129`): takes owned `HarnessSettings` + owned `LocalStorageLayout`, returns `Result<HarnessAppState, BootstrapError>`. It delegates to `init_harness_app_state_with_factories` with `DefaultEventBusFactory` and `DefaultTraceContextFactoryInitializer` so production callers never instantiate the seam directly.
- `init_harness_app_state_with_factories(settings, storage_layout, &impl EventBusFactory, &impl TraceContextFactoryInitializer)` matches the contract test-seam clause (`…app-state.md:43-45`). Documented in the proposal's "Test Seam" section (`proposals/04-wu-0a-04.md:68-70`).
- `BootstrapError` declares exactly the five documented variants in the documented order: `SettingsInvalid`, `StorageLayoutInvalid`, `DatabaseOpenFailed`, `EventBusInitFailed`, `TraceInitFailed` (`src-tauri/src/contracts/harness_app_state.rs:3-10`). Default serde produces the PascalCase wire strings the contract specifies (`…app-state.md:51-57`). The `bootstrap-errors.json` fixture pins the canonical ordering (`product-strategy/contracts/fixtures/wu-0a-04/bootstrap-errors.json:1-7`).
- Initialization order in `init_harness_app_state_with_factories` matches the contract sequence (`…app-state.md:33-39`): validate settings → validate storage layout → open SQLite → initialize event bus → initialize trace factory (`app_state.rs:137-146`). Cheap validation runs before any subsystem opening, which keeps SQLite/event-bus/trace allocations off the failure path for `SettingsInvalid` and `StorageLayoutInvalid`.

### Test fixture exercises every variant — not just the happy path

The Rust contract test (`src-tauri/tests/harness_app_state_contract.rs`) covers every WU acceptance criterion:

| Acceptance criterion (ticket) | Test |
|---|---|
| Happy-path returns `Ok(HarnessAppState)` with all five handles, preserves `workspace_id`/`storage_root`/`database_path` | `init_harness_app_state_creates_all_handles_and_preserves_paths` (`harness_app_state_contract.rs:103-142`) |
| `SettingsInvalid` for empty required field | `init_harness_app_state_rejects_invalid_settings` (`…contract.rs:144-157`) |
| `StorageLayoutInvalid` for layout outside storage root | `init_harness_app_state_rejects_invalid_storage_layout` (`…contract.rs:159-172`) |
| `DatabaseOpenFailed` when SQLite path unopenable | `init_harness_app_state_reports_database_open_failure` (`…contract.rs:174-187`) |
| `EventBusInitFailed` from fixture-controlled factory | `injectable_event_bus_factory_failure_is_reported` (`…contract.rs:189-208`) |
| `TraceInitFailed` from fixture-controlled initializer | `injectable_trace_factory_failure_is_reported` (`…contract.rs:210-229`) |
| Every variant round-trips serde, unknown rejected | `bootstrap_error_variants_round_trip_and_unknown_variant_is_rejected` (`…contract.rs:231-264`) |

The happy-path test is not a stub: it actually subscribes to the event bus (`state.event_bus.subscribe()` at `…contract.rs:126`), creates a real `TraceContext` through the factory and asserts the workspace ID round-trips (`…contract.rs:127-131`), and executes a real `SELECT 1` query against the SQLite pool to prove the connection is live (`…contract.rs:133-137`). It also re-asserts the WU-0A-01 invariant `agent_harness_lib::registered_command_count() == 0` (`…contract.rs:139`).

### Serde round-trip coverage

`bootstrap_error_variants_round_trip_and_unknown_variant_is_rejected` (`…contract.rs:231-264`):

1. Loads `bootstrap-errors.json` and asserts the fixture ordering matches `expected_names` (`…contract.rs:235-246`), pinning the canonical taxonomy.
2. For each name, deserializes into `BootstrapError` and re-serializes, asserting equality with the original string (`…contract.rs:248-255`).
3. Loads `invalid-bootstrap-error.json` (`"Unknown"`) and asserts `serde_json::from_value::<BootstrapError>` returns `Err` (`…contract.rs:257-263`).

All five variants are covered; the unknown-variant rejection is exercised against a real fixture.

### Phase boundary respected (no Phase 0B/0C/1+ behavior)

The proposal's anti-scope (`proposals/04-wu-0a-04.md:73-79`) bars: registered IPC commands, GraphStore migrations / tables / schemas / repositories, provider credentials, `agents` subprocess invocation, and Tauri command registration / invoke-handler edits / `tauri.conf.json` command additions. Verified:

- No `tauri::generate_handler!` invocation; `src-tauri/src/lib.rs:9-14` still uses `tauri::Builder::default().setup(|_app| Ok(()))` and never calls `app.manage(...)` or `invoke_handler(...)`. The `HarnessAppState` is constructed only inside the contract test, never inserted into a Tauri builder. `registered_command_count()` still returns `0` and the WU-0A-01 assertion `phase_0a_registers_no_value_slice_commands` still passes (`lib.rs:16-28`).
- `src-tauri/migrations/` does not exist (no GraphStore migration files added). `src-tauri/src/app_state.rs:187-197` opens SQLite with `create_if_missing(true)` and no migrator; no `sqlx::migrate!()` macro, no schema DDL strings.
- No provider crates added (`Cargo.toml` change is bounded to the Tokio `sync` feature flip — `git diff main -- src-tauri/Cargo.toml` shows the single feature addition; `Cargo.lock` is unchanged). No `tokio::process` / `std::process::Command` imports in `app_state.rs`.
- No edits to `src-tauri/tauri.conf.json` or `src-tauri/capabilities/default.json` (`git diff main` shows them unchanged). Capability set is byte-identical to WU-0A-01.

### Adjacent-path additions (in-scope)

- `src-tauri/src/lib.rs:1`: adds `pub mod app_state;` next to existing `pub mod contracts; pub mod events; pub mod settings; pub mod storage; pub mod tracing;`. One-line module declaration. Required to expose the entrypoint to the integration test crate.
- `src-tauri/src/contracts/mod.rs:2`: adds `pub mod harness_app_state;` between `event_topic` and `harness_settings`. One-line module declaration.
- `src-tauri/Cargo.toml:20`: adds `"sync"` to the existing Tokio features list `["rt-multi-thread", "macros"]`. Required because the proposal's chosen `EventBusHandle` design wraps `tokio::sync::broadcast::Sender` (`proposals/04-wu-0a-04.md:17-21`); `tokio::sync::broadcast` is gated behind the `sync` feature in tokio.

### Boundary observations (informational, not scope creep)

- INFO: `validate_storage_layout_pair` (`app_state.rs:172-185`) enforces `settings.storage_root == storage_layout.storage_root` and `settings.database_path == storage_layout.database_path` on top of `validate_local_storage_layout` (which is owned by WU-0A-03). The contract clause (`…app-state.md:35-36`) only says "reject invalid storage-layout containment", which would be satisfied by calling WU-0A-03's validator alone. The settings/layout consistency check is a sensible additional invariant (the bootstrap container should not store inconsistent paths), and it is reachable via the documented `StorageLayoutInvalid` taxonomy. Not scope creep — it is internal hardening of the same documented variant.
- INFO: `EVENT_BUS_CAPACITY = 256` (`app_state.rs:16`) is a private fixed broadcast capacity. The proposal documents the choice as "fixed positive broadcast capacity" (`proposals/04-wu-0a-04.md:25`). Not scope creep — it is the minimum primitive a broadcast sender requires.
- INFO: `EventBusHandle::subscribe(...)` and `receiver_count(...)` (`app_state.rs:37-44`) expose the broadcast sender's documented surface so downstream WUs (WU-0A-08 and later) can subscribe without re-creating the sender. The proposal scopes the handle to a "minimal broadcast sender for JSON-valued IPC events" (`proposals/04-wu-0a-04.md:21`); these accessor methods are part of that minimum. The happy-path test exercises `subscribe()` (`…contract.rs:126`).
- INFO: `TraceContextFactory::create(...)` (`app_state.rs:62-79`) forwards verbatim to WU-0A-09's `create_trace_context(...)` (`src-tauri/src/tracing/trace_context.rs:21-48`). It does not re-implement trace logic; the wrapper exists only to bind `workspace_id` once at bootstrap so callers do not need to pass it on every call. The factory's `workspace_id()` accessor (`app_state.rs:58-60`) is read-only.
- INFO: The fixture markers `event_bus_failure: true` (`error-event-bus-init-failed.json:19`) and `trace_failure: true` (`error-trace-init-failed.json:19`) are not parsed by `AppStateCase` (`…contract.rs:20-26`); the test selects the failure factory directly in the test body. The markers are documentary only. Removing them or wiring them into the case parser is a stylistic question — not a scope issue.

## Verdict

**LOW.** Implementation lands exactly the container, entrypoint pair, error taxonomy, and bilingual / Rust-only fixtures the WU contract and ticket call out. The 5 `BootstrapError` variants are each reachable from a fixture-backed test (not just stub assertions); serde round-trip + unknown-variant rejection are pinned to two dedicated fixtures. Anti-scope is honored end-to-end (no IPC command registration, no `app.manage(...)`, no GraphStore migrations or schemas, no provider crates, no agents invocation, no `tauri.conf.json` edits, no capability changes). The only adjacent-path edits are the two unavoidable one-line module wirings plus the bounded Tokio `sync` feature addition required by the documented broadcast-based `EventBusHandle`.
