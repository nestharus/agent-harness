# Risk Gate — Supported Surface (WU-0A-10)

**Severity:** LOW

## Summary

No new Tauri commands, no producer wiring, no GraphStore tables, no migrations, no provider credentials, no `agents` invocation. The runtime topic is the only `EventTopic` variant referenced inside this WU's source files. Proposal's supported-surface track matches.

## Verification

### No new Tauri commands
- `src-tauri/src/lib.rs:15` — `PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"];` (length 1, unchanged).
- `phase_0a_scaffold_commands()` test at `src-tauri/src/lib.rs:78` — `assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);` still passes.
- `tauri::generate_handler![commands::subscribe_workspace_events::subscribe_workspace_events]` (`src-tauri/src/lib.rs:21-23`) — single command, unchanged.
- `git diff main -- src-tauri/src/lib.rs` returns empty: lib.rs untouched.
- WU-0A-10's contract test re-asserts the allowlist on line 318: `assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);`. ✓

### No real event-bus emission / producer wiring
- `src-tauri/src/events/runtime_events.rs` exposes only `build_backend_span_runtime_event(...)` — a builder that returns an `IpcEvent<BackendSpanEvent>`. It does not publish, does not invoke `app.emit`, does not push onto a queue, and does not register any subscription. ✓
- `emit_backend_span` is a pure builder over caller-supplied data; no I/O, no event bus interaction. ✓
- Proposal explicitly states "It does not emit to the event bus and does not wire any producer. Producer wiring belongs to a later WU." (`proposals/10-wu-0a-10.md:31`). ✓

### Runtime topic is the only EventTopic referenced in this WU's source
- Grep over the three WU source files (`src-tauri/src/tracing/backend_span_event.rs`, `src-tauri/src/events/runtime_events.rs`, `src-tauri/src/contracts/backend_span_event.rs`):
  - Only match: `events/runtime_events.rs:11` — `build_ipc_event(workspace_id, EventTopic::Runtime, payload, trace_context_id)`.
- `wu_0a_10_source_references_no_later_domain_event_topic_variants` (`backend_span_event_contract.rs:327-365`) statically asserts presence of `EventTopic::Runtime` and absence of `Graph`, `Render`, `Provider`, `Optimizer`, `Worker`, `Question`, `Recovery`, `Budget`, `Audit`. Test passes. ✓

### No GraphStore tables / migrations
- No SQL files added. No migration scripts. No `storage/migrations/...` files added.
- `git diff main --stat` only touches three `mod.rs` files. ✓

### No provider credentials
- No keys, no secrets, no env-loading code added. ✓

### No `agents` invocation
- No subprocess spawn, no `Command::new`, no agent-runner reference in WU source. (The pre-existing fake agent runner stub in `lib.rs` is untouched.) ✓

### Proposal supported-surface track matches
`proposals/10-wu-0a-10.md:51-59` enumerates:
- Deployment mode: local Phase 0A Tauri scaffold and library contract tests
- Customer cohort: internal contract consumers + future Phase 0A runtime producers
- Public paths: Rust library consumers, TypeScript contract importers, later producer wiring
- Adjacent paths in blast radius: existing event topic, IPC envelope, trace context, module declarations, build/test commands
- Migration path: additive contract, tracing module, runtime envelope helper, fixtures, tests
- Rollback path: remove the added BackendSpanEvent modules, fixtures, contract doc, proposal, and module declarations
- Observability: Rust contract tests + Vitest tests cover DTO/status/builder/topic-binding drift

All match the actual implementation. ✓

## Findings

None at MEDIUM or HIGH. The supported surface stays within Phase 0A contract-only scope.

## Verification commands run
- `grep "EventTopic::" src-tauri/src/tracing/backend_span_event.rs src-tauri/src/events/runtime_events.rs src-tauri/src/contracts/backend_span_event.rs` — only `EventTopic::Runtime` appears.
- `cargo test --manifest-path src-tauri/Cargo.toml --test backend_span_event_contract` — 10/10 passing, including the static topic-only test and the command allowlist assertion.
- `git diff main -- src-tauri/src/lib.rs` — empty.
- `git diff main -- src-tauri/Cargo.toml src-tauri/tauri.conf.json` — empty.
