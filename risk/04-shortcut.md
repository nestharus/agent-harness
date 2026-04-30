# WU-0A-04 — Shortcut Risk Review

**Gate:** Phase 8 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-04's purpose (a contract-pinned `HarnessAppState` that owns real subsystem handles — settings, storage layout, tokio broadcast event bus, sqlx SQLite pool, WU-0A-09-backed trace factory — with every documented `BootstrapError` variant reachable from a fixture-backed test, no faked subsystem, and no test-seam that hides a real failure path)? Are TODOs, stubbed branches, weak assertions, or runtime feature flags hiding incomplete work?

## Subsystem reality check (the high-stakes question)

This WU wires real subsystems for the first time in Phase 0A. Each subsystem must be *real*, not a stub.

### SQLite pool — real

`open_sqlite_pool` (`src-tauri/src/app_state.rs:187-197`) constructs `sqlx::sqlite::SqliteConnectOptions::new().filename(Path::new(database_path)).create_if_missing(true)` and connects via `sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect_with(options).await`. The pool type stored in `HarnessAppState.db` is `sqlx::SqlitePool` (`app_state.rs:5, 23`). The happy-path test executes a real round-trip query against the pool: `sqlx::query_as("SELECT 1").fetch_one(&state.db).await` returns `(1,)` (`src-tauri/tests/harness_app_state_contract.rs:133-137`). No mock, no in-memory placeholder, no `Box<dyn …>` stub.

### EventBusHandle — real

`EventBusHandle` (`app_state.rs:27-44`) wraps `tokio::sync::broadcast::Sender<IpcEvent<serde_json::Value>>` and exposes `subscribe()` and `receiver_count()`. `DefaultEventBusFactory::create_event_bus` (`app_state.rs:88-93`) calls `tokio::sync::broadcast::channel(EVENT_BUS_CAPACITY)` with `EVENT_BUS_CAPACITY = 256`. The happy-path test calls `state.event_bus.subscribe()` (`…contract.rs:126`) — if the sender were a no-op stub, `subscribe()` would not return a usable receiver. The Cargo.toml change adds the Tokio `sync` feature precisely because `tokio::sync::broadcast` requires it (`src-tauri/Cargo.toml:20`); a no-op stub would not have needed that dependency change.

### TraceContextFactory — real, wraps WU-0A-09

`TraceContextFactory::create` (`app_state.rs:62-79`) calls `crate::tracing::trace_context::create_trace_context(...)` directly — the same function WU-0A-09 ships at `src-tauri/src/tracing/trace_context.rs:21-48`. The happy-path test calls the factory with `actor="backend"` and asserts `trace_context.workspace_id == state.settings.workspace_id` (`…contract.rs:127-131`). If the factory were a separate (faked) implementation that ignored the workspace_id, the assertion would fail when the factory's static workspace_id drifted from the bootstrapped one.

### Test seam — documented and used cleanly, not a runtime feature flag

`init_harness_app_state_with_factories(...)` is a sibling public function, not a `#[cfg(test)]` gate, not a runtime env-var switch, not a feature flag. Production callers use `init_harness_app_state(...)` (`app_state.rs:118-129`), which always supplies `&DefaultEventBusFactory` and `&DefaultTraceContextFactoryInitializer`. There is no path by which production code can accidentally pick the failing factory:

- `FailingEventBusFactory` and `FailingTraceContextFactoryInitializer` (`…contract.rs:266-288`) live inside the `tests/` integration crate and are never exported from `agent_harness_lib`.
- `DefaultEventBusFactory` and `DefaultTraceContextFactoryInitializer` are zero-sized unit structs (`app_state.rs:85, 102`); they cannot smuggle hidden state.
- The seam variant is documented in both the contract (`product-strategy/contracts/wu-0a-04-harness-app-state.md:41-47`) and the proposal (`proposals/04-wu-0a-04.md:68-70`); the contract explicitly says the seam variant has the *same validation and SQLite behavior* as the production initializer and exists only to make `EventBusInitFailed` and `TraceInitFailed` reachable.

### `EventBusInitFailed` and `TraceInitFailed` paths actually fail

Both fixture failure paths force the factory/initializer to return `Err(...)` *before* `init_harness_app_state_with_factories` constructs the `HarnessAppState`:

- `FailingEventBusFactory::create_event_bus` returns `Err(BootstrapError::EventBusInitFailed)` unconditionally (`…contract.rs:268-272`). The injected factory is invoked at `app_state.rs:141-143`; the `?` plus `.map_err(|_| BootstrapError::EventBusInitFailed)` propagates the failure. The test asserts `result.map(|_| ()) == Err(BootstrapError::EventBusInitFailed)` (`…contract.rs:206`).
- `FailingTraceContextFactoryInitializer::create_trace_context_factory` returns `Err(BootstrapError::TraceInitFailed)` unconditionally (`…contract.rs:281-287`). The injected initializer is invoked at `app_state.rs:144-146`; the `?` plus `.map_err(|_| BootstrapError::TraceInitFailed)` propagates the failure. The test asserts `result.map(|_| ()) == Err(BootstrapError::TraceInitFailed)` (`…contract.rs:227`).

These are not happy-path stubs that return Err — they are explicit failing implementations whose only purpose is to reach the documented variants.

### `DatabaseOpenFailed` triggers a real sqlx connection failure

`error-database-open-failed.json` sets `database_path` to `__TEMP_ROOT__/storage/unopenable.sqlite` and carries `database_path_is_directory: true` (`product-strategy/contracts/fixtures/wu-0a-04/error-database-open-failed.json:5, 19`). The test's `prepare_filesystem` honors that flag and creates a *directory* at the database path (`…contract.rs:97-101`), so when `sqlx::SqlitePool::connect_with(...)` opens the path it fails because the path exists as a directory rather than a file (or as a writable file). The `validate_storage_layout_pair` containment check passes because the path is still under `storage_root`; the failure is downstream at the actual sqlx open. This is a genuine sqlx error mapped to `BootstrapError::DatabaseOpenFailed` via `.map_err(|_| BootstrapError::DatabaseOpenFailed)` (`app_state.rs:196`). No fake taxonomy.

### Validation order — cheap checks first

`SettingsInvalid` and `StorageLayoutInvalid` are reached before any subsystem opens (`app_state.rs:137-146`):

1. `validate_settings(&settings)?` — pure string-emptiness check.
2. `validate_storage_layout_pair(&settings, &storage_layout)?` — pure equality + lexical containment.
3. `let db = open_sqlite_pool(...).await?` — only now does sqlx touch the filesystem.
4. `let event_bus = event_bus_factory.create_event_bus()?` — broadcast channel creation.
5. `let trace_context_factory = trace_context_factory_initializer.create_trace_context_factory(...)?`.

This matches the contract's sequenced initialization (`…app-state.md:33-39`). Cheap validation never rides on top of an expensive subsystem opening, so an empty `workspace_id` cannot accidentally create a temp SQLite file.

## Shortcuts examined

### S1 — `EventBusHandle` is a thin wrapper over `tokio::sync::broadcast::Sender`

`app_state.rs:27-44`. The handle exposes only `subscribe()` and `receiver_count()`. Justified: the proposal explicitly scopes `EventBusHandle` to "a cloneable backend handle for `IpcEvent<serde_json::Value>` messages" (`product-strategy/contracts/wu-0a-04-harness-app-state.md:23`) and the proposal records that "Phase 0A does not register commands or emit domain events from this WU" (`proposals/04-wu-0a-04.md:23`). Future WUs (WU-0A-08+) will layer publish/subscribe wrappers on top. **Justified.**

### S2 — `DefaultEventBusFactory::create_event_bus` is infallible in production

`app_state.rs:88-93`. `tokio::sync::broadcast::channel(256)` cannot fail at runtime (panics only on `capacity == 0`, which is impossible because the constant is fixed). The proposal explicitly notes this and routes `EventBusInitFailed` through the test seam (`proposals/04-wu-0a-04.md:25`). The contract test reaches the variant via the seam (`…contract.rs:189-208`). **Justified.**

### S3 — `DefaultTraceContextFactoryInitializer` carries a defensive empty-workspace-id guard

`app_state.rs:105-116`: returns `Err(BootstrapError::TraceInitFailed)` if `workspace_id.trim().is_empty()`. In practice this branch is unreachable in production because `validate_settings` already rejects empty workspace_id (`app_state.rs:158-167`), so a seam-free production path would never reach the empty-id check. The branch is defensive in case a future caller bypasses settings validation. **Justified, but redundant** — not a hidden incompleteness because the documented `TraceInitFailed` variant is reached via the test seam, not via this branch (`…contract.rs:210-229`).

### S4 — `validate_storage_layout_pair` adds a settings/layout consistency check on top of WU-0A-03 validation

`app_state.rs:172-185`. Enforces `settings.storage_root == layout.storage_root` and `settings.database_path == layout.database_path` before delegating to `validate_local_storage_layout`. Not strictly required by the contract but a sensible internal invariant for the bootstrap container. Both branches map to the documented `StorageLayoutInvalid` taxonomy. **Justified.**

### S5 — `Display` for `BootstrapError` synthesizes the wire string via `serde_json`

`harness_app_state.rs:12-17`. Same pattern as `SettingsError`/`StorageLayoutError`/`TraceContextError` (consistency check across `harness_settings.rs:35-40`, `local_storage_layout.rs:24-29`, `trace_context.rs:38-43`). Couples human-readable form to wire form so they cannot drift. **Justified.**

### S6 — `.map_err(|_| BootstrapError::…)` discards the underlying error cause

`app_state.rs:140`, `:142-143`, `:145-146`, `:184`, `:196`. The original sqlx / closure error is dropped at every variant boundary. Justified: `BootstrapError` is a closed taxonomy serializable to a single PascalCase string (`harness_app_state.rs:3-10`); preserving the underlying error would either require a free-form `String` field (which would not round-trip cleanly) or a wider error enum (which the WU contract forbids). Loss of underlying cause is acceptable for Phase 0A; future logging WUs (WU-0A-10) will own structured error capture upstream. **Justified.**

### S7 — `TraceContextFactory` stores only `workspace_id`, not the entire `HarnessSettings`

`app_state.rs:46-79`. The factory captures only the field it needs. Justified: WU-0A-09 `create_trace_context(...)` takes `workspace_id: &str` (`tracing/trace_context.rs:21-28`); pulling in the full settings would be unnecessary coupling. The factory is `Clone` so it can be cheaply moved into request handlers. **Justified.**

### S8 — `IpcEvent<serde_json::Value>` payload is a JSON-typed broadcast

`app_state.rs:29`. The bus carries `IpcEvent<Value>` rather than a typed payload. Justified: the proposal documents this as the broadcast envelope choice ("a minimal broadcast sender for JSON-valued IPC events", `proposals/04-wu-0a-04.md:21`). Phase 0A topics other than `runtime` are inert (per the WU-0A-05 acceptance criteria), so a strongly-typed payload would be premature. **Justified.**

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented!()`, `todo!()`, `panic!()`, `unwrap()`, or `expect()` in `src-tauri/src/app_state.rs` or `src-tauri/src/contracts/harness_app_state.rs` (verified via Grep — zero matches in the two production files).
- No `#[allow(dead_code)]`, no commented-out match arms, no `#[ignore]` Rust tests in `src-tauri/tests/harness_app_state_contract.rs`.
- All five `BootstrapError` variants are reachable from at least one fixture-backed test:
  - `SettingsInvalid` → `error-settings-invalid.json` via empty `workspace_id`.
  - `StorageLayoutInvalid` → `error-storage-layout-invalid.json` via `database_path` outside `storage_root`.
  - `DatabaseOpenFailed` → `error-database-open-failed.json` via directory-as-file at `database_path`.
  - `EventBusInitFailed` → `error-event-bus-init-failed.json` via `FailingEventBusFactory`.
  - `TraceInitFailed` → `error-trace-init-failed.json` via `FailingTraceContextFactoryInitializer`.
- Round-trip and unknown-variant rejection: `…contract.rs:231-264` covers serde of all five variants plus negative `invalid-bootstrap-error.json` (`"Unknown"`).
- Production paths use `Result` and the `?` operator throughout; the test crate uses `expect()` / `panic!()` only in fixture-loader helpers (`…contract.rs:31, 41-43, 53, 55, 82, 87, 91-95, 99, 114, 130, 136, 141, …`) — those are integration-test ergonomics and are appropriate (a fixture-load failure should fail the test loudly, not silently). They are not in production paths.
- `cargo clippy --all-targets --all-features -- -D warnings` passes with no warnings; `cargo fmt --check` is clean (verified locally).
- All 7 contract tests pass: `cargo test --manifest-path src-tauri/Cargo.toml` reports `test result: ok. 7 passed; 0 failed; 0 ignored` for `harness_app_state_contract`.

## Verdict

**LOW.** Every subsystem is real: SQLite is a live `sqlx::SqlitePool` proven by an end-to-end `SELECT 1` query, `EventBusHandle` is a real `tokio::sync::broadcast::Sender` proven by a working `subscribe()` call, and `TraceContextFactory` forwards verbatim to WU-0A-09's `create_trace_context`. The test seam is a documented sibling public function (not a `#[cfg(test)]` gate, not a runtime feature flag, not an env-var switch); the failing factory/initializer implementations live in the integration test crate and never leak into production. `EventBusInitFailed` and `TraceInitFailed` are forced via explicit failing implementations, not by a happy-path stub returning `Err`. `DatabaseOpenFailed` triggers a real sqlx error by materializing a directory at the database path. Validation order matches the contract: cheap settings/layout checks fail before any sqlx/broadcast/trace allocation. No production `unwrap`/`expect`/`panic`, no TODOs, no stubbed enum arms, no `#[ignore]` tests, and every documented variant is pinned to its own fixture.
