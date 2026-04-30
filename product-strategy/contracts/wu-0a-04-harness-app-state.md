# WU-0A-04 Contract: HarnessAppState Backend Container

## Ownership

This WU owns the Rust backend app-state container, its initialization entrypoints, the event-bus handle, the trace-context factory wrapper, and the bootstrap error taxonomy. It does not own settings loading, storage-layout derivation, event topic taxonomy, IPC command registration, GraphStore migrations, provider credentials, or agent subprocess execution.

## Container

```text
HarnessAppState {
  settings: HarnessSettings,
  storage_layout: LocalStorageLayout,
  event_bus: EventBusHandle,
  db: SqlitePool,
  trace_context_factory: TraceContextFactory
}
```

Rules:

1. `settings` is the owned `HarnessSettings` supplied to initialization.
2. `storage_layout` is the owned `LocalStorageLayout` supplied to initialization.
3. `event_bus` is a cloneable backend handle for `IpcEvent<serde_json::Value>` messages.
4. `db` is a real `sqlx::SqlitePool` opened against `storage_layout.database_path`.
5. `trace_context_factory` is a cloneable wrapper that calls WU-0A-09 `create_trace_context(...)` using the initialized workspace ID.

## Initialization

```text
init_harness_app_state(settings: HarnessSettings, storage_layout: LocalStorageLayout) -> Result<HarnessAppState, BootstrapError>
```

Initialization order:

1. Reject empty required settings strings with `BootstrapError::SettingsInvalid`.
2. Reject invalid storage-layout containment with `BootstrapError::StorageLayoutInvalid`.
3. Open SQLite using `storage_layout.database_path`, `create_if_missing(true)`, and no migrations.
4. Initialize the event-bus handle.
5. Initialize the trace-context factory.

The production initializer uses default factories. Tests may use:

```text
init_harness_app_state_with_factories(settings, storage_layout, event_bus_factory, trace_context_factory_initializer)
```

The factory variant has the same validation and SQLite behavior as the production initializer. It exists only to make `EventBusInitFailed` and `TraceInitFailed` reachable without adding runtime-only magic tokens to settings.

## Errors

`BootstrapError` serializes as one of these exact strings:

- `SettingsInvalid`
- `StorageLayoutInvalid`
- `DatabaseOpenFailed`
- `EventBusInitFailed`
- `TraceInitFailed`

Unknown error strings are rejected by Rust serde.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-04/`:

- `happy-path.json`: canonical settings and storage layout pair. `__TEMP_ROOT__` is replaced by the Rust contract test temp directory.
- `error-settings-invalid.json`: settings input with an empty required field.
- `error-storage-layout-invalid.json`: layout input with a database path outside the storage root.
- `error-database-open-failed.json`: valid pair whose database path is materialized as an unopenable directory.
- `error-event-bus-init-failed.json`: valid pair plus `event_bus_failure: true` for the injectable factory seam.
- `error-trace-init-failed.json`: valid pair plus `trace_failure: true` for the injectable factory seam.
- `bootstrap-errors.json`: all documented error variants.
- `invalid-bootstrap-error.json`: unknown error string rejection fixture.

## Test Handoff

Rust contract test: `src-tauri/tests/harness_app_state_contract.rs`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
