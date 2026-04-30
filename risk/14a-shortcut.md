# WU-0A-14a Risk Gate: Shortcut / Placeholder

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

Each documented error variant is reachable through real failure modes — directory-as-DB-file for `DatabaseCreateFailed`, empty `workspace_id` driving the WU-0A-04 `validate_settings` failure for `AppStateInitFailed`, missing manifest file on disk for `FixtureManifestMissing`, and unknown seed for `UnknownSeed`. Each `temp_harness_state` call produces a unique temp directory derived from `process::id()` + a nanos-seeded counter. The harness uses `init_harness_app_state` directly rather than re-implementing app-state setup. SQLite is opened with `create_if_missing` only and contains zero user tables (asserted by `sqlite_master` query). No TODOs or `unimplemented!()` stubs are present in the shipped code.

## Verification

### Per-call SQLite isolation

`temp_harness.rs:227-233` defines `unique_temp_dir`:

```rust
fn unique_temp_dir(seed_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "agent-harness-wu-0a-14a-{seed_name}-{}-{}",
        std::process::id(),
        unique_suffix()
    ))
}
```

`unique_suffix` (`temp_harness.rs:239-247`) combines a process-wide `AtomicU64` counter (`NEXT_TEMP_HARNESS_COUNTER`) with `SystemTime::now().duration_since(UNIX_EPOCH).as_nanos()`, so two calls in the same nanosecond still get distinct paths. The materialized `HarnessSettings` then derives `database_path = storage_root.join("harness.sqlite")` from this unique root, so two consecutive `temp_harness_state("empty", None, None, ...)` calls will write to disjoint files. Confirmed by reading the test which calls `temp_harness_state` for both `empty` and `minimal-runtime` seeds without any cleanup, and by the registry keying inserts on `fixture_manifest_id` (also nanos-seeded).

### Real `fixture_manifest_id`

`temp_harness.rs:235-237`:

```rust
fn fixture_manifest_id(seed_name: &str) -> String {
    format!("manifest-{seed_name}-{}", unique_suffix())
}
```

The id is `manifest-<seed>-<nanos>-<counter>`. Not a hardcoded constant. The happy-empty fixture asserts `expected_manifest_prefix: "manifest-empty"` and the test uses `starts_with(&fixture.expected_manifest_prefix)`, so the runtime-generated suffix is verified to vary while the prefix is stable.

### Reuses `init_harness_app_state`

`temp_harness.rs:62-65`:

```rust
let app_state =
    crate::app_state::init_harness_app_state(settings.clone(), storage_layout.clone())
        .await
        .map_err(|_| TempHarnessError::AppStateInitFailed)?;
```

No re-implementation of `HarnessAppState` construction; the WU-0A-04 entry point is reused directly.

### SQLite genuinely empty

`probe_sqlite_create_if_missing` (`temp_harness.rs:180-193`) opens with `SqliteConnectOptions::new().filename(...).create_if_missing(true)` and immediately closes the pool. `init_harness_app_state` then opens a second pool, again `create_if_missing(true)`, with no `sqlx::migrate!` call. The contract test queries `sqlite_master`:

```rust
sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
```

(`tests/temp_harness_contract.rs:111`) and asserts `expected_sqlite_tables: []` for every documented happy seed. The standalone `temp_harness_source_contains_no_graphstore_migration_hookpoints` test (line 333) greps the source for `sqlx::migrate!` and `MIGRATIONS_DIR` — both absent.

### `DatabaseCreateFailed` is a real sqlx failure

The fixture `error-database-create-failed.json` documents the trigger as `"database_path_is_directory"`. The test (`tests/temp_harness_contract.rs:268-289`) calls `fs::create_dir_all(&database_path)` so the SQLite file path is itself a directory, then passes the matching layout. `prepare_database_parent` succeeds (the parent `storage` exists). `probe_sqlite_create_if_missing` then calls `SqlitePoolOptions::connect_with(...)` which fails because SQLite cannot open a directory as a database file. The error path is real, not a stub.

### `AppStateInitFailed` reuses WU-0A-04's failing path

The fixture `error-app-state-init-failed.json` documents the trigger as `"empty_workspace_id_after_successful_sqlite_probe"`. The test (lines 291-307) calls `settings_and_layout(&temp_root, "", database_path)` so `HarnessSettings.workspace_id` is empty. `prepare_database_parent` and `probe_sqlite_create_if_missing` both succeed (the path is a fresh file). `init_harness_app_state` then runs `validate_settings` (`app_state.rs:212-225`), which trims each required field and returns `BootstrapError::SettingsInvalid` for any empty value. The `temp_harness_state` mapping `Err(_) => TempHarnessError::AppStateInitFailed` then returns the documented variant. This exercises the real WU-0A-04 validation path.

### `RealAgentsInvocationAttempted` is structural

`temp_harness.rs:116-118`:

```rust
pub fn attempt_real_agents_spawn(_handle: &TempHarnessHandle) -> Result<(), TempHarnessError> {
    Err(TempHarnessError::RealAgentsInvocationAttempted)
}
```

No `Command::new`, no `std::process::Command`, no path reference. `grep -rn 'Command::new\|/home/nes/.local/bin/agents\|sqlx::migrate' src-tauri/src` returns zero hits in source code (only the test source-grep assertion in `tests/temp_harness_contract.rs`). The seam unconditionally refuses, which is functionally equivalent to a structural guard — there is no spawn site to gate. See also `WU-0A-14a-SCOPE-F01` for a discussion of this design choice; it is acceptable as a contractual marker.

### `FixtureManifestMissing` is a real file-system check

`temp_harness.rs:51-54`:

```rust
let materialized = materialize_seed(seed_name, &seed)?;
if !materialized.manifest_path.exists() {
    return Err(TempHarnessError::FixtureManifestMissing);
}
```

The `missing-manifest` seed is registered with `manifest_relative_path: "manifests/missing-manifest.json"`, which is intentionally absent from `product-strategy/contracts/fixtures/wu-0a-14a/manifests/` (only `empty.json` and `minimal-runtime.json` exist). The two happy seeds resolve to real files (verified by `ls product-strategy/contracts/fixtures/wu-0a-14a/manifests/`).

### Runtime fixtures flow into the bus

`temp_harness.rs:98-114`:

```rust
pub fn replay_recorded_runtime_events(
    handle: &TempHarnessHandle,
) -> Result<usize, TempHarnessError> {
    let stored = registry().lock()...get(&handle.fixture_manifest_id).cloned()
        .ok_or(TempHarnessError::AppStateInitFailed)?;
    let count = stored.runtime_events.len();
    for event in stored.runtime_events {
        let _ = stored.app_state.event_bus.publish(event);
    }
    Ok(count)
}
```

The test `temp_harness_uses_live_app_state_and_replays_runtime_event_fixtures` (`tests/temp_harness_contract.rs:189-218`) subscribes via `state.event_bus.subscribe()` *before* invoking replay, then asserts the receiver gets a runtime-topic envelope with the matching `workspace_id` and `span_event_id` within 1 second. This proves the events traverse the live `EventBusHandle` rather than being stashed-and-discarded.

### No TODOs / unimplemented stubs

`grep -n 'TODO\|todo!\|unimplemented!\|FIXME\|XXX'` over `src-tauri/src/test_harness/` and `src-tauri/src/contracts/temp_harness.rs` returns zero matches. All branches return real values.

### Verification command outputs

| Command | Result |
| --- | --- |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | clean |
| `cargo clippy --all-targets --all-features -- -D warnings` | clean |
| `cargo test --manifest-path src-tauri/Cargo.toml` | 6 contract tests pass + 28 prior-WU tests pass |
| `bun run lint` | clean |
| `bun run typecheck` | clean |
| `bun run test` | 79 frontend tests pass |

## Findings

### WU-0A-14a-SHORTCUT-F01 — `runtime_fixture_envelopes` maps `serde_json::to_value` failures to `AppStateInitFailed` (LOW)

**Where:** `src-tauri/src/test_harness/temp_harness.rs:195-214`

**Detail:** Two `serde_json` errors inside `runtime_fixture_envelopes` (the `to_value(payload)` step and the `build_ipc_event` failure case) are mapped to `TempHarnessError::AppStateInitFailed`. `BackendSpanEvent` and `build_ipc_event` are deterministic over the input fields, so neither path is reachable through normal use, and no fixture exercises this branch. This is a trivial belt-and-suspenders error mapping rather than a stub return.

**Impact:** None at the contract level; the unreachable branch never fires for any documented input. If a future schema change makes `BackendSpanEvent` non-serializable in some shape, the variant attribution would be slightly misleading (`AppStateInitFailed` vs. a more specific input-validation error), but that is hypothetical.

**Recommendation:** Optional — leave as-is; this is acceptable defensive code.

### WU-0A-14a-SHORTCUT-F02 — `replay_recorded_runtime_events` returns `AppStateInitFailed` if the registry lookup misses (LOW)

**Where:** `src-tauri/src/test_harness/temp_harness.rs:101-106`

**Detail:** `replay_recorded_runtime_events` returns `TempHarnessError::AppStateInitFailed` when the registry entry for the given `handle.fixture_manifest_id` is absent. A registry miss only happens if `temp_harness_state` returned `Err` (so no entry was inserted) or if a caller hand-constructs a `TempHarnessHandle`. The variant attribution is loose — it isn't really an app-state init failure — but no fixture exercises this branch, and the public contract for replay outcomes is not codified.

**Impact:** None for the documented test path; only matters for hand-rolled handles, which are not part of the supported surface.

**Recommendation:** Optional — acceptable as defensive code. Could be tightened to a more accurate variant in a future refinement.

## Verdict

**LOW.** Every error variant is reachable through a real failure mode (directory-as-file SQLite open, WU-0A-04 validation rejection, missing-on-disk manifest, unknown registry key). Each call writes to a unique temp path, the `fixture_manifest_id` is generated at runtime from a counter+nanos, the database is genuinely empty (`sqlite_master` query returns `[]`), and the WU-0A-04 `init_harness_app_state` is reused rather than re-implemented. No TODOs / placeholders.
