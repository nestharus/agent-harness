# WU-0A-09 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-09 boundary (the `TraceContext` Rust+TS DTO, the 7-variant `TraceActor` taxonomy, the 3-variant `TraceContextError`, `parse_trace_actor`, and `create_trace_context` with documented validation), or does it bleed into GraphStore writes, audit-log persistence, durable correlation-ID storage, IPC commands, or any provider/agent/orchestration work the proposal anti-scope reserves for later WUs (`proposals/09-wu-0a-09.md:23-29`)?

## Findings

### In-scope, confirmed

- Rust `TraceContext` declares exactly the seven documented fields with correct optionality and `#[serde(deny_unknown_fields)]` (`src-tauri/src/contracts/trace_context.rs:3-17`), matching the contract (`product-strategy/contracts/wu-0a-09-trace-context-correlation-schema.md:9-19`). Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]` so they are omitted when absent, per the contract's "Optional fields are omitted when absent" rule (`…schema.md:19`).
- Rust `TraceActor` encodes exactly the seven documented variants in canonical order with `#[serde(rename_all = "lowercase")]` (`trace_context.rs:19-29`), matching the canonical order in `fixtures/wu-0a-09/trace-actors.json:1-9` and the contract (`…schema.md:23-31`).
- Rust `TraceContextError` encodes exactly `EmptyWorkspaceId`, `UnknownActor`, `ParentWithoutInvocation` and serializes via default serde to those PascalCase strings (`trace_context.rs:31-36`), matching the contract (`…schema.md:60-62`) and the `trace-context-errors.json` fixture.
- `parse_trace_actor(raw: &str)` performs exact-match: each documented lowercase string → its variant, anything else → `UnknownActor` (`src-tauri/src/tracing/trace_context.rs:8-19`). No trimming, no case folding, no aliasing — confirmed by the negative branch and by serde's `rename_all = "lowercase"` (which also rejects `"unknown"`, asserted at `src-tauri/tests/trace_context_contract.rs:113-116`).
- `create_trace_context(...)` runs the documented validation order: empty workspace → `EmptyWorkspaceId`, then actor parse → `UnknownActor`, then parent-without-invocation → `ParentWithoutInvocation`, then constructs the DTO with a generated correlation ID and preserves all supplied optional fields (`tracing/trace_context.rs:21-48`). Matches the contract creation behavior (`…schema.md:48-69`) and is loop-pinned to `create-errors.json` and `create-success-{minimal,full}.json` fixtures.
- Correlation IDs are generated as opaque non-empty strings via a process-local timestamp + atomic counter (`tracing/trace_context.rs:6, 50-58`). Asserted non-empty in `trace_context_contract.rs:130-133, 156-159`. Matches the proposal Assumption A1 ("opaque non-empty correlation IDs", `proposals/09-wu-0a-09.md:43`) and contract (`…schema.md:50`).

### Anti-scope honored

The proposal anti-scope (`proposals/09-wu-0a-09.md:23-29`) bars: GraphStore schema/writes/ref validation, audit-log work, provider/agent/worker/optimizer/reviewer/orchestration behavior, Tauri command registration / IPC, and durable correlation-ID persistence. Verified:

- No `tauri::generate_handler!` invocation; `src-tauri/src/lib.rs:7-12` still uses `tauri::Builder::default().setup(|_app| Ok(())).run(...)`. `registered_command_count()` still returns `0` and the WU-0A-01 assertion `phase_0a_registers_no_value_slice_commands` still passes (`lib.rs:14-25`).
- No GraphStore / audit-log code: `graph_ref` and `audit_event_ref` are stored as `Option<String>` and never validated against any store, exactly as Assumption A2 declares (`proposals/09-wu-0a-09.md:44`). The `audit:event:99` and `graph:node:42` strings appear only in fixtures, not in any module that touches storage.
- No persistence surface: `NEXT_CORRELATION_COUNTER` is a process-local `AtomicU64` (`tracing/trace_context.rs:6`); no SQL, no file I/O, no `std::fs`, no `serde_json::to_writer`, no migration files added.
- No provider / agent / orchestration code: variant names like `Worker`, `Optimizer`, `Reviewer`, `Orchestrator` appear only as enum variants and as fixture strings — no module imports a provider crate, no subprocess code (no `std::process` / `tokio::process` imports).
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`.

### Adjacent-path additions (in-scope)

- `src-tauri/src/lib.rs:4`: adds `pub mod tracing;` next to existing `contracts;`, `events;`, `settings;`. One-line module declaration.
- `src-tauri/src/contracts/mod.rs:3`: adds `pub mod trace_context;` next to `event_topic;` and `harness_settings;`. One-line module declaration.
- `src-tauri/src/tracing/mod.rs:1`: single line `pub mod trace_context;` — minimal tracing-namespace seed.
- No `Cargo.toml` / `Cargo.lock` changes (`git diff HEAD -- src-tauri/Cargo.toml src-tauri/Cargo.lock` is empty). The serde / serde_json runtime deps already promoted by WU-0A-02 cover all derives.

### Potential concerns — none rising to MEDIUM

- INFO: `TraceActor` derives `Hash` (`trace_context.rs:19`). Not strictly required by the WU but allows future map-keying; unit-only enum, no semantic surface change.
- INFO: `TraceContextError` derives `std::error::Error` and a `Display` impl that synthesizes the wire string via `serde_json::to_string` (`trace_context.rs:38-45`). Same pattern as WU-0A-05 `EventTopicError` — convenience for `?` interop, ties human form to wire form. Not a scope expansion.

## Verdict

**LOW.** Implementation lands exactly the DTO, actor taxonomy, error taxonomy, parser, and creation helper that the WU contract and ticket call out. Anti-scope is honored end-to-end (no GraphStore, no audit log, no providers / agents / orchestration, no IPC, no persistence, no subprocesses), and adjacent edits are only the unavoidable module wiring for the new files.
