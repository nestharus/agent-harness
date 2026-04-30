# WU-0A-06 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does the implementation only add the surface declared in the proposal's supported-surface track (`proposals/06-wu-0a-06.md:30-38`) — local Phase 0A Tauri scaffold contract types, internal Rust+TS contract consumers, additive contract / event module / fixtures / tests — or does it expand operator-visible behavior, register new IPC commands, introduce GraphStore tables, provider credentials, or invoke `agents`?

## Findings

### WU-0A-06-SUPPORTED-SURFACE-F01 — Deployment mode unchanged: local Phase 0A Tauri scaffold

`src-tauri/src/lib.rs` is byte-identical with `main` (`git diff main -- src-tauri/src/lib.rs` returns no output). The Tauri builder is still `Builder::default().setup(|_app| Ok(())).run(...)`, no handlers added. The proposal's deployment-mode declaration ("local Phase 0A Tauri scaffold and library contract tests", `proposals/06-wu-0a-06.md:32`) is preserved. **On-track.**

### WU-0A-06-SUPPORTED-SURFACE-F02 — `registered_command_count() == 0` invariant preserved

`cargo test --manifest-path src-tauri/Cargo.toml` runs `tests::phase_0a_registers_no_value_slice_commands` (the WU-0A-01 invariant from `src-tauri/src/lib.rs`) and it passes. No `tauri::generate_handler!` invocation, no `#[tauri::command]` attribute, no `app.manage(...)`, no `tauri::State` import anywhere in the new files. This is the central DTO-only-WU guarantee and it holds.

### WU-0A-06-SUPPORTED-SURFACE-F03 — Customer cohort matches: internal contract consumers only

The proposal declares the cohort as "internal contract consumers and future Phase 0A/0C IPC slices importing the Rust/TS DTOs" (`proposals/06-wu-0a-06.md:33`). Verified:

- Rust public surface: `agent_harness_lib::contracts::ipc_event::{IpcEvent, IpcEventError}` and `agent_harness_lib::events::ipc_event::{build_ipc_event, build_ipc_event_from_raw_topic}`. Both are library exports, consumable by future Rust producers; neither is a Tauri command.
- TypeScript public surface: `IpcEvent`, `IpcEventError`, `IPC_EVENT_ERRORS`, `parseIpcEvent`, `parseIpcEventError` from `src/contracts/ipc-event.ts:1-89`. No React component, no router entry, no IPC subscriber.
- No new operator-visible UI surface: no edit to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, no new `*.tsx` component.

### WU-0A-06-SUPPORTED-SURFACE-F04 — Anti-scope honored on disallowed surfaces

Proposal anti-scope (`proposals/06-wu-0a-06.md:23-28`) bars IPC subscriptions, event bus / channel / producer wiring, Tauri command registration, domain payload schemas, GraphStore, providers, optimizer, worker, question, recovery, budget, audit, trace context creation, durable event-id registries, and persisted timestamps. Verified by inspection / grep across the new files:

- No GraphStore tables / SQL / migration files: nothing under `src-tauri/src/storage/` is touched, no `*.sql` file exists in the diff, no `rusqlite` / `sqlx` / `sea_orm` import.
- No provider credentials: no env-var read for API keys, no `keyring`, no secret-store crate.
- No `agents` invocation: no `std::process::Command` / `tokio::process::Command` reference in the new files; the harness `bin/fake-agents` fixture from WU-0A-02 is untouched.
- No event bus / channel: no `tokio::sync::broadcast`, no `crossbeam_channel`, no `tauri::AppHandle::emit`, no Tauri capability addition. `src-tauri/capabilities/default.json` is unchanged.
- No trace context creation: the optional `trace_context_id` is treated as an opaque string and only round-tripped through serialization (`src-tauri/src/events/ipc_event.rs:71`, `src/contracts/ipc-event.ts:50-52, 69-72`); no import of `crate::contracts::trace_context`.
- No durable event-id registry / persisted timestamp: `generate_event_id` and `generate_created_at` (`src-tauri/src/events/ipc_event.rs:75-91`) operate on a process-local `AtomicU64` and `SystemTime::now()`; nothing is written to disk.

### WU-0A-06-SUPPORTED-SURFACE-F05 — Adjacent paths in blast radius are exactly the declared ones

Proposal (`proposals/06-wu-0a-06.md:35`) declares the blast radius as "WU-0A-05 `EventTopic`, contract module declarations, event module declarations, and `bun run *` / Cargo checks." Observed:

- WU-0A-05 surface: `parse_event_topic` is *consumed* by `build_ipc_event_from_raw_topic` (`src-tauri/src/events/ipc_event.rs:8, 40`); no edit to `src-tauri/src/contracts/event_topic.rs`, `src-tauri/src/events/topic.rs`, `src/contracts/event-topic.ts`, or any WU-0A-05 fixture / test (verified by `git diff main` showing only `mod.rs` edits).
- Contract module declarations: `src-tauri/src/contracts/mod.rs:3` adds `pub mod ipc_event;` (one line).
- Event module declarations: `src-tauri/src/events/mod.rs:1` adds `pub mod ipc_event;` (one line).
- Build-system checks: `bun run lint`, `bun run typecheck`, `bun run test`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings` all green.

### WU-0A-06-SUPPORTED-SURFACE-F06 — Migration / rollback paths are clean

Proposal migration: "additive contract, event module, fixtures, and tests" (`proposals/06-wu-0a-06.md:36`). Observed addition is exactly that — six fixture files, one contract md, two Rust source files, one Rust test, one TS source file, one TS test, plus two single-line `mod.rs` lines. No edits to any pre-existing fixture, test, or source file outside the two `mod.rs` declarations. Rollback path ("remove the added WU-0A-06 files and module declarations") is mechanically reversible.

### WU-0A-06-SUPPORTED-SURFACE-F07 — Observability matches the declared surface

Proposal observability: "Rust contract tests and Vitest tests expose envelope drift, builder behavior, raw-topic rejection, serialization failure mapping, and error taxonomy drift" (`proposals/06-wu-0a-06.md:38`). Observed:

- Envelope drift: `ipc_event_shape_round_trips_canonical_and_minimal_fixtures` (`src-tauri/tests/ipc_event_contract.rs:67-94`) and the canonical/minimal vitest case (`src/test/ipc-event.test.ts:46-64`).
- Builder behavior: `build_ipc_event_generates_metadata_and_preserves_fixture_fields` (`src-tauri/tests/ipc_event_contract.rs:96-137`).
- Raw-topic rejection: `build-errors.json` row `unknown raw topic` (`src-tauri/tests/ipc_event_contract.rs:161-173`).
- Serialization failure mapping: `build-errors.json` row `payload serialization failed` plus the `NonSerializablePayload` fixture (`src-tauri/tests/ipc_event_contract.rs:35-46, 174-190`).
- Error taxonomy drift: `ipc_event_error_variants_round_trip_and_unknown_variant_is_rejected` (Rust, `src-tauri/tests/ipc_event_contract.rs:207-235`) and the TS parity case (`src/test/ipc-event.test.ts:101-124`).

No new metrics, no new log emitters, no new operator-visible event — observability stays inside the test boundary, which is what the proposal declared.

## Verdict

**LOW.** The implementation only adds the declared surfaces: a generic Rust+TS DTO, a builder, a raw-topic helper, error taxonomy, fixtures, and contract tests. `registered_command_count() == 0` still holds; no GraphStore, no providers, no `agents` invocation, no event bus, no trace context creation, no operator-visible UI surface. Adjacent edits are exactly the proposal-declared module declarations.
