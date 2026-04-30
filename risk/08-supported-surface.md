# WU-0A-08 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does this slice keep the supported surface inside the documented Phase 0A scaffold-command allowlist (deployment mode: local Tauri bootstrap; one frontend-reachable command `subscribe_workspace_events`; no Phase 0B+ value-slice commands, GraphStore tables, migrations, providers, agent invocation, or domain producers, per `proposals/08-wu-0a-08.md:43-51`)?

## Findings

### WU-0A-08-SURFACE-F01 — Exactly one Tauri command is registered

`src-tauri/src/lib.rs:21-23`:

```rust
.invoke_handler(tauri::generate_handler![
    commands::subscribe_workspace_events::subscribe_workspace_events
])
```

The `generate_handler![...]` macro contains exactly one comma-free entry. No other `commands::*` symbol appears in `lib.rs`. `commands/mod.rs` (`src-tauri/src/commands/mod.rs:1`) declares only `pub mod subscribe_workspace_events;` — no other handler module. `grep -n "#\[tauri::command\]" src-tauri/src` returns one hit (`commands/subscribe_workspace_events.rs:12`). **Single command.**

### WU-0A-08-SURFACE-F02 — Allowlist returns exactly `["subscribe_workspace_events"]`

`PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:15`). `phase_0a_scaffold_commands()` returns `&PHASE_0A_SCAFFOLD_COMMANDS` (`:29-31`). `registered_command_count()` returns `phase_0a_scaffold_commands().len()` (`:33-35`) — i.e. `1`. The compile-time array length pins the count; nothing computes the list dynamically from a registration side effect. The lib unit test (`:73-83`), the `tauri_bootstrap_is_inert_and_command_free` integration test (`src-tauri/tests/scaffold_contract.rs:163-164`), the `command_registration_matches_phase_0a_scaffold_allowlist` test (`src-tauri/tests/subscribe_workspace_events_contract.rs:140-150`), and the WU-0A-04 happy-path assertion (`src-tauri/tests/harness_app_state_contract.rs:139-146`) all anchor on the same allowlist. **One source of truth.**

### WU-0A-08-SURFACE-F03 — Updated `tauri_bootstrap_is_inert_and_command_free` rejects value-slice commands

`src-tauri/tests/scaffold_contract.rs:174-179` explicitly forbids the two value-slice command names called out in the dispatch (`get_harness_settings`, `ping_runtime`):

```rust
for forbidden_command in ["get_harness_settings", "ping_runtime"] {
    assert!(
        !lib_content.contains(&format!("commands::{forbidden_command}")),
        "Phase 0A scaffold must not register value-slice command {forbidden_command}"
    );
}
```

The test also asserts the registered list equals `["subscribe_workspace_events"]` (`:163-164`) and that the `generate_handler![]` registration is non-empty (`:180-183`). Together these prevent both directions of regression: a future commit cannot re-register a value-slice command without flipping these assertions, and the assertion can no longer be satisfied by an empty handler. The previous "no `generate_handler!` at all" rule is replaced with a strictly tighter rule that still has a test; it is not a coverage downgrade. **Strong barrier.**

### WU-0A-08-SURFACE-F04 — `harness_app_state_contract.rs` count assertion now binds to the allowlist

`src-tauri/tests/harness_app_state_contract.rs:139-146` is updated from `registered_command_count() == 0` to:

```rust
assert_eq!(
    agent_harness_lib::registered_command_count(),
    agent_harness_lib::phase_0a_scaffold_commands().len()
);
assert_eq!(
    agent_harness_lib::phase_0a_scaffold_commands(),
    ["subscribe_workspace_events"]
);
```

The first equality re-checks the count/list invariant from the WU-0A-04 perspective; the second pins the list contents. With `PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1]` (`lib.rs:15`), `registered_command_count()` returns 1 — and the allowlist is asserted explicitly. **Coherent with the new allowlist.**

### WU-0A-08-SURFACE-F05 — No GraphStore / migrations / providers / agents / domain producers

Verified:

- No `CREATE TABLE`, `ALTER TABLE`, `INSERT`, or migration file added (`src-tauri/migrations/` remains empty per `tests/scaffold_contract.rs:124-133`).
- `src-tauri/src/app_state.rs` retains the WU-0A-04 `db: SqlitePool`; the diff only adds the in-memory subscription registry — no SQL paths.
- No provider crates, no API keys, no `.env` references.
- No `agents` invocation: `rg -n "\\bagents\\b" src-tauri/src` returns zero matches in scope.
- No event producer: `subscribe_workspace_events` only validates inputs and inserts into `subscriptions`; `state.event_bus.sender.send(...)` is never invoked. `Subscription` (`events/subscription.rs:22-28`) holds `_receiver` but never spawns a task to drain it. **No producer wired.**
- No `tauri.conf.json` capability-allowlist edit (`git diff main -- src-tauri/tauri.conf.json` is empty). The Tauri v2 invoke handler registration in `lib.rs` is the sole binding.

### WU-0A-08-SURFACE-F06 — Proposal Supported-Surface track matches the diff

`proposals/08-wu-0a-08.md:43-51` declares: deployment mode = local Tauri bootstrap; user-reachable path = frontend IPC command name `subscribe_workspace_events`; adjacent paths = WU-0A-04..07 plus scaffold contract tests; migration path = replace zero-command assertion with allowlist; rollback path = remove handler + restore empty list; observability = test-visible only, no production telemetry. The diff matches each:

- Tauri builder bootstrap is unchanged in shape (`lib.rs:18-27`); only the handler registration and managed state are added.
- `subscribe_workspace_events` is the sole frontend-reachable name (`commands/subscribe_workspace_events.rs:12-20`).
- Touched modules are exactly the WU-0A-04..07 surface (`app_state.rs`, `events/subscription.rs`, `events/topic.rs` consumed via `parse_event_topic`, `contracts/event_topic.rs` consumed via `EventTopic`, `contracts/ipc_event.rs` consumed via `IpcEvent`, `contracts/harness_command.rs` consumed only by name parity; no edits beyond imports).
- The migration step is exactly the assertion swap documented in `tests/scaffold_contract.rs:182-183` and `tests/harness_app_state_contract.rs:139-146`.
- Rollback path is straightforward: removing `commands::subscribe_workspace_events::subscribe_workspace_events` from `generate_handler!`, restoring `PHASE_0A_SCAFFOLD_COMMANDS: [&str; 0] = []`, and reverting the two test assertions returns the surface to the WU-0A-04 zero-command scaffold.
- No production telemetry: no `tracing::info!`, `println!`, or `app_handle.emit` in the diff outside `Debug` impls.

## Verdict

**LOW.** The Tauri invoke handler now contains exactly one entry (`subscribe_workspace_events`). The `phase_0a_scaffold_commands()` allowlist is a compile-time array of length 1, anchoring all four lock-step assertion sites. The updated scaffold-contract test explicitly rejects `commands::get_harness_settings` and `commands::ping_runtime`, and forbids an empty `generate_handler![]` — preventing regressions in both directions. No GraphStore tables, no migrations, no providers, no `agents` invocation, no domain producers, no `tauri.conf.json` allowlist edit. Proposal Supported-Surface track is met end-to-end.
