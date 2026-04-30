# WU-0A-06 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-06 boundary (the generic `IpcEvent<T>` envelope, `build_ipc_event(workspace_id, topic, payload, trace_context_id)` builder, the four-variant `IpcEventError` taxonomy, and bilingual fixtures) or does it bleed into the IPC subscription wiring, event bus, domain payload schemas, GraphStore, providers, optimizers, workers, questions, recovery, budget, audit, trace context creation, or any later-phase producer work the proposal anti-scope reserves (`proposals/06-wu-0a-06.md:23-28`)?

## Findings

### WU-0A-06-SCOPE-F01 — IpcEvent shape matches the contract exactly (in-scope)

`IpcEvent<T>` in `src-tauri/src/contracts/ipc_event.rs:5-15` declares the documented six fields and only those: `event_id`, `workspace_id`, `topic` (typed as `EventTopic` from WU-0A-05), generic `payload`, `created_at`, and optional `trace_context_id` with `#[serde(skip_serializing_if = "Option::is_none")]`. `#[serde(deny_unknown_fields)]` enforces the contract clause "Unknown JSON fields are rejected by Rust serde" (`product-strategy/contracts/wu-0a-06-ipc-event.md:18`). The TypeScript mirror in `src/contracts/ipc-event.ts:3-10` carries the same shape and the parser in `src/contracts/ipc-event.ts:21-73` enforces an exact-key match against the canonical sorted list, rejecting both extra and missing keys. Loop-bound to `canonical-event.json` and `minimal-event.json` in `src-tauri/tests/ipc_event_contract.rs:67-94` and `src/test/ipc-event.test.ts:46-64`.

### WU-0A-06-SCOPE-F02 — Builder signature matches the WU contract field

`build_ipc_event<T>(workspace_id: &str, topic: EventTopic, payload: T, trace_context_id: Option<&str>) -> Result<IpcEvent<T>, IpcEventError>` (`src-tauri/src/events/ipc_event.rs:12-29`) is the literal shape printed in the ticket scope (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-06.md`) and the contract Builder block (`product-strategy/contracts/wu-0a-06-ipc-event.md:22-29`), with `T: Serialize` so payload serialization failures map to `PayloadSerializationFailed` per `product-strategy/contracts/wu-0a-06-ipc-event.md:41`. The success path generates a non-empty `event_id`, a non-empty `created_at`, preserves `workspace_id`/`topic`/`payload`/`trace_context_id`, and is asserted by the `build-success.json`-driven test in `src-tauri/tests/ipc_event_contract.rs:96-137` against the four documented preserved keys.

### WU-0A-06-SCOPE-F03 — Raw-topic helper is contract-documented, not scope creep

`build_ipc_event_from_raw_topic` (`src-tauri/src/events/ipc_event.rs:31-42`) parses an untrusted string via `parse_event_topic` from WU-0A-05 and maps any `EventTopicError` to `IpcEventError::UnknownTopic`. The acceptance criterion "Attempting to build an event from an invalid raw topic returns `IpcEventError::UnknownTopic`" (ticket scope) is not reachable from the typed `build_ipc_event(..., topic: EventTopic, ...)` because `EventTopic` is a closed enum; the proposal records this as A4 (`proposals/06-wu-0a-06.md:45`) and the contract documents the helper at `product-strategy/contracts/wu-0a-06-ipc-event.md:43-54`. Loop-bound by the `unknown raw topic` row in `build-errors.json` (`src-tauri/tests/ipc_event_contract.rs:161-173`). On-concern.

### WU-0A-06-SCOPE-F04 — All four IpcEventError variants implemented, serde-pinned, and tested

`IpcEventError` (`src-tauri/src/contracts/ipc_event.rs:17-23`) declares exactly the four documented variants in the documented order (`EmptyEventId`, `EmptyWorkspaceId`, `UnknownTopic`, `PayloadSerializationFailed`). The TypeScript mirror `IPC_EVENT_ERRORS` (`src/contracts/ipc-event.ts:12-19`) is a `const` tuple deriving the union type, asserted equal to `ipc-event-errors.json` in `src/test/ipc-event.test.ts:101-124`. Reachability is loop-driven from `build-errors.json` (`src-tauri/tests/ipc_event_contract.rs:139-204`): empty workspace → `EmptyWorkspaceId`, unknown raw topic → `UnknownTopic`, the `non_serializable` payload fixture → `PayloadSerializationFailed`. `EmptyEventId` is reached via the private `build_ipc_event_with_metadata` helper in `src-tauri/src/events/ipc_event.rs:99-111` (the public `generate_event_id` path always emits a non-empty string by construction); all four variants are round-tripped through serde and the unknown-variant rejection is asserted in `src-tauri/tests/ipc_event_contract.rs:207-235`.

### WU-0A-06-SCOPE-F05 — Anti-scope honored

Anti-scope (`proposals/06-wu-0a-06.md:23-28`) bars IPC subscription commands, event bus / channel / producer wiring, Tauri command registration, domain payload schemas, GraphStore, providers, optimizers, workers, questions, recovery, budget, audit, trace context creation, and durable event-id / timestamp persistence. Verified:

- No `tauri::generate_handler!` or `app.manage(...)` invocation; `src-tauri/src/lib.rs` is unchanged from `main` (`git diff main -- src-tauri/src/lib.rs` returns no output) and `registered_command_count()` still returns `0`. The `phase_0a_registers_no_value_slice_commands` assertion still passes (`cargo test ... lib tests::phase_0a_registers_no_value_slice_commands ... ok`).
- No domain payload structs anywhere — payloads are received as `T: Serialize` in Rust and validated by a caller-supplied parser in TypeScript (`src/contracts/ipc-event.ts:21-73`).
- No GraphStore tables, SQL/migrations, provider credentials, agents subprocess invocation, optimizer/worker/budget/audit/recovery/question modules.
- No trace-context creation: the optional `trace_context_id` is treated as an opaque string and only round-tripped (`src-tauri/src/events/ipc_event.rs:71`, `src/contracts/ipc-event.ts:50-52, 69-72`). No import of `crate::contracts::trace_context`.
- No durable event-id registry or persisted timestamps: `generate_event_id` is an in-process atomic counter prefixed by `SystemTime::now()` nanos (`src-tauri/src/events/ipc_event.rs:75-80`), and `generate_created_at` is a derived string (`ipc_event.rs:82-91`); nothing is written to disk or pushed to a bus.

### Adjacent-path additions (in-scope)

- `src-tauri/src/contracts/mod.rs:3` — single-line `pub mod ipc_event;` next to the existing modules. One-line module declaration.
- `src-tauri/src/events/mod.rs:1` — single-line `pub mod ipc_event;` next to the existing `topic`. One-line module declaration.
- No `Cargo.toml` / `Cargo.lock` / `package.json` / `bun.lock` changes (`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock` is empty); the existing serde / serde_json deps cover the new derives.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or any prior-WU contract / event / fixture file.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ (eslint + cargo clippy via turbo) |
| `bun run typecheck` | ✓ (tsc + cargo check via turbo) |
| `bun run test` | ✓ 24/24 vitest tests pass (4 in `src/test/ipc-event.test.ts`) |
| `cargo test --manifest-path src-tauri/Cargo.toml` | ✓ all suites pass; `tests/ipc_event_contract.rs` 4/4, `events::ipc_event::tests::empty_generated_event_id_is_reported` 1/1 |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | ✓ no diff |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** The implementation lands the documented `IpcEvent<T>` shape, the documented `build_ipc_event` signature with all metadata generation, the contract-documented `build_ipc_event_from_raw_topic` boundary, and all four `IpcEventError` variants — every one of them reached by a fixture or a serde round-trip. Anti-scope is honored end-to-end (no IPC command registration, no event bus, no domain payloads, no GraphStore/providers/optimizers/workers/etc., no trace context creation, no persistence), and the only adjacent edits are the unavoidable single-line module declarations.
