# WU-0A-14a Contract: Temp SQLite Harness Entrypoint

## Ownership

This WU owns the Rust temp SQLite harness entrypoint for tests and dev scripts. It composes existing Phase 0A Rust contracts and runtime handles. It does not own frontend rendering, Tauri command registration, GraphStore migrations, provider credentials, or real agents process execution.

## DTO

`TempHarnessHandle` serializes as a JSON object with exactly these fields:

- `workspace_id`: required string.
- `database_path`: required string.
- `fixture_manifest_id`: required string.
- `app_state_ready`: required boolean.

Unknown fields are rejected by Rust serde.

## Entrypoint

```text
temp_harness_state(
  seed_name: string,
  settings?: HarnessSettings,
  storage_layout?: LocalStorageLayout,
  runtime_event_fixtures?: BackendSpanEvent[]
) -> Result<TempHarnessHandle, TempHarnessError>
```

`seed_name` must be present in the internal seed registry. When `settings` or `storage_layout` are omitted, the selected seed derives values under a unique temp directory. When supplied, those values are used verbatim.

The documented happy-path seeds are:

- `empty`
- `minimal-runtime`

The documented error seed is:

- `missing-manifest`

## Live App State Lookup

`TempHarnessHandle` stays serializable and does not embed runtime handles. Rust tests/dev scripts can resolve a live `HarnessAppState` from a successful handle through the harness lookup seam. The resolved state must have been initialized by `init_harness_app_state`.

## Runtime Event Fixture Replay

`runtime_event_fixtures` accepts `BackendSpanEvent[]`. The harness records these as runtime-topic fixture events associated with the returned handle. Phase 0A does not run a domain producer. Tests/dev scripts can replay the recorded fixture events through the live event bus after creating a subscriber.

## Errors

`TempHarnessError` serializes as one of these exact strings:

- `UnknownSeed`
- `DatabaseCreateFailed`
- `AppStateInitFailed`
- `RealAgentsInvocationAttempted`
- `FixtureManifestMissing`

Behavior:

- Unknown seed names return `UnknownSeed`.
- SQLite create/open probe failure returns `DatabaseCreateFailed`.
- `init_harness_app_state` failure returns `AppStateInitFailed`.
- Calling `attempt_real_agents_spawn` returns `RealAgentsInvocationAttempted` without executing `/home/nes/.local/bin/agents`.
- A registered seed whose manifest metadata file is absent returns `FixtureManifestMissing`.

## No GraphStore Migrations

The harness performs no GraphStore migrations and no schema DDL. It must not call `sqlx::migrate!`, define a `MIGRATIONS_DIR` constant, or create tables. SQLite is opened with `create_if_missing` only.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-14a/`:

- `seed-registry.json`: documented seed names and manifest files.
- `handle-round-trip.json`: canonical `TempHarnessHandle`.
- `happy-empty.json`: expected happy-path assertions for the `empty` seed.
- `happy-minimal-runtime.json`: expected happy-path assertions for the `minimal-runtime` seed.
- `temp-harness-errors.json`: all documented error variants.
- `error-unknown-seed.json`: unknown seed trigger.
- `error-database-create-failed.json`: SQLite create/open failure trigger.
- `error-app-state-init-failed.json`: app-state init failure trigger.
- `error-real-agents-invocation-attempted.json`: real agents spawn-attempt refusal trigger.
- `error-fixture-manifest-missing.json`: missing manifest trigger.
- `runtime-event-replay.json`: runtime fixture payloads and replay assertion.
- `no-agents-binary-executed.json`: structural assertion for `/home/nes/.local/bin/agents`.
- `manifests/empty.json` and `manifests/minimal-runtime.json`: present manifest metadata for happy seeds.

## Test Handoff

Rust contract test: `src-tauri/tests/temp_harness_contract.rs`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
