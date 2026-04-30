# WU-0A-14a Risk Gate: Scope-Creep

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The implementation is faithful to the WU contract field and ticket scope. `TempHarnessHandle` carries exactly the four documented fields with `serde(deny_unknown_fields)`. `temp_harness_state` accepts the documented `(seed_name, settings?, storage_layout?, runtime_event_fixtures?)` arguments. All five `TempHarnessError` variants are implemented and reachable through documented fixtures. All ten acceptance criteria are exercised by `src-tauri/tests/temp_harness_contract.rs`. The harness composes WU-0A-02/03/04/06/08/10 contracts via imports without re-implementing them and applies no GraphStore DDL. No Phase 0B/0C/1+ behavior leaked in.

## Verification

### TempHarnessHandle DTO shape

`src-tauri/src/contracts/temp_harness.rs:3-10` declares `TempHarnessHandle { workspace_id, database_path, fixture_manifest_id, app_state_ready }` with `#[serde(deny_unknown_fields)]`. The round-trip test (`src-tauri/tests/temp_harness_contract.rs:119-142`) parses `handle-round-trip.json`, re-serializes it back to the original `Value`, and asserts that an extra `extra: true` key is rejected.

### Entrypoint signature

`src-tauri/src/test_harness/temp_harness.rs:41-46` declares:

```rust
pub async fn temp_harness_state(
    seed_name: &str,
    settings: Option<HarnessSettings>,
    storage_layout: Option<LocalStorageLayout>,
    runtime_event_fixtures: Vec<BackendSpanEvent>,
) -> Result<TempHarnessHandle, TempHarnessError>
```

This matches the WU contract `temp_harness_state(seed_name, settings?, storage_layout?, runtime_event_fixtures?) -> Result<TempHarnessHandle, TempHarnessError>`.

### Five error variants

`src-tauri/src/contracts/temp_harness.rs:12-19` defines exactly `UnknownSeed`, `DatabaseCreateFailed`, `AppStateInitFailed`, `RealAgentsInvocationAttempted`, `FixtureManifestMissing`. The fixture array `temp-harness-errors.json` lists the same five strings in the same order, and `temp_harness_error_variants_round_trip_and_unknown_is_rejected` (`src-tauri/tests/temp_harness_contract.rs:220-251`) asserts each variant round-trips and that `"OtherError"` is rejected.

### Acceptance-criteria mapping

| AC | Source | Verification |
| --- | --- | --- |
| AC1 (handle round-trip) | `temp_harness_handle_round_trips_stable_field_names` (`tests/temp_harness_contract.rs:119`) | full-fixture round-trip + unknown-field rejection |
| AC2 (handle has non-empty fields, `app_state_ready: true`) | `documented_seeds_create_ready_isolated_empty_sqlite_handles` (`tests/temp_harness_contract.rs:144-187`) | `!handle.workspace_id.is_empty()` + `!handle.database_path.is_empty()` + `Path::new(&handle.database_path).exists()` + `app_state_ready == true` |
| AC3 (init via WU-0A-04 settings/layout) | `temp_harness.rs:62-65` calls `crate::app_state::init_harness_app_state(settings, storage_layout)` | live `HarnessAppState` resolved through `harness_app_state(&handle)` |
| AC4 (no GraphStore migrations, SQLite empty) | `documented_seeds_create_ready_isolated_empty_sqlite_handles` asserts `sqlite_table_names == []`; `temp_harness_source_contains_no_graphstore_migration_hookpoints` (line 333) greps the source for `sqlx::migrate!` and `MIGRATIONS_DIR` |
| AC5 (every error round-trips + reachable) | round-trip test + `documented_error_inputs_reach_every_temp_harness_error_variant` (line 253) |
| AC6 (`UnknownSeed` for `"unknown"`) | `documented_error_inputs_reach_every_temp_harness_error_variant:258-266` |
| AC7 (`DatabaseCreateFailed`) | same test, lines 268-289 — passes a layout whose `database_path` is a directory |
| AC8 (`AppStateInitFailed`) | same test, lines 291-307 — passes a layout with empty `workspace_id` |
| AC9 (`RealAgentsInvocationAttempted`) | same test, lines 320-330 — calls `attempt_real_agents_spawn(&handle)` |
| AC10 (`FixtureManifestMissing`) | same test, lines 309-318 — `missing-manifest` seed targets absent manifest file |

### Runtime-event-fixture replay path

`temp_harness.rs:66` builds `runtime_events: Vec<IpcEvent<Value>>` from each `BackendSpanEvent` via `runtime_fixture_envelopes`, which goes through `build_ipc_event` from WU-0A-06. The events are stored in `StoredHarness.runtime_events` keyed by `fixture_manifest_id`. `replay_recorded_runtime_events` (line 98) iterates and calls `event_bus.publish(event)`. The test `temp_harness_uses_live_app_state_and_replays_runtime_event_fixtures` (line 189) subscribes via `state.event_bus.subscribe()`, triggers replay, and asserts a runtime-topic envelope arrives with the same `span_event_id` and `workspace_id`. (Note: the contract phrases this as "associated with the returned handle", and the implementation associates via the registry HashMap and replays through the same `EventBusHandle` that subscribers consume — functionally equivalent to "recorded into the bus subscription registry for replay".)

### Phase boundary

No Phase 0B+ behavior. The harness:

- Does not register a Tauri command (`phase_0a_scaffold_commands()` length is still 1, asserted on `tests/temp_harness_contract.rs:183-186`).
- Does not run `sqlx::migrate!` or define `MIGRATIONS_DIR`.
- Does not create user tables (the SQLite-empty assertion enforces this).
- Does not invoke `/home/nes/.local/bin/agents` (no `Command::new` anywhere in the source).
- Does not introduce provider config, credentials, or graph DDL.

### Code/test boundary alignment

Ticket code boundary: `src-tauri/src/test_harness/temp_harness.rs`, `src-tauri/src/contracts/temp_harness.rs`, `src-tauri/tests/temp_harness_contract.rs`. All three files exist; nothing else inside `src-tauri/src/test_harness/` was added beyond the boundary file plus a one-line `mod.rs`. Test boundary fixtures live under `product-strategy/contracts/fixtures/wu-0a-14a/` and the contract doc is `product-strategy/contracts/wu-0a-14a-temp-sqlite-harness-entrypoint.md`, both as required.

## Findings

None above LOW.

### WU-0A-14a-SCOPE-F01 — `attempt_real_agents_spawn` does not inspect or reference the refused binary path (LOW)

**Where:** `src-tauri/src/test_harness/temp_harness.rs:116-118`

**Detail:** The seam is implemented as an unconditional `Err(TempHarnessError::RealAgentsInvocationAttempted)`. It does not reference `/home/nes/.local/bin/agents` (or any path) and has no spawn logic to refuse. The behavior is structurally safe — there is literally no code path that could spawn the binary — and it satisfies the contract bullet "Calling `attempt_real_agents_spawn` returns `RealAgentsInvocationAttempted` without executing `/home/nes/.local/bin/agents`." The fixture `error-real-agents-invocation-attempted.json` documents `refused_binary: "/home/nes/.local/bin/agents"`, but the assertion is purely contractual rather than guarding a real spawn attempt.

**Impact:** None at the contract level; the seam is a marker, and the absence of a real spawn path *is* the structural guarantee. Future regressions that introduce a real spawn site would need to be caught by the broader source-grep `Command::new` check, not by this seam.

**Recommendation:** Optional — accept as-is. No blocker.

## Verdict

**LOW.** All ten acceptance criteria are exercised; the DTO, signature, and error taxonomy match the WU contract exactly; the runtime-event replay path is wired through the WU-0A-04 event bus; no Phase 0B+ behavior leaked in.
