# WU-0A-09 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does deployment-mode + customer-cohort + adjacent-public-paths analysis match the WU-0A-09 supported-surface declaration? What is the blast radius for adjacent paths (the WU-0A-01 inert scaffold, WU-0A-02 settings loader, WU-0A-05 event-topic taxonomy)? Migration / rollback / observability story?

## Supported-surface contract (proposal `proposals/09-wu-0a-09.md:31-39`)

- Deployment mode: local Phase 0A Tauri scaffold and library contract tests.
- Customer cohort: internal contract consumers and future Phase 0A slices importing the Rust/TS DTOs.
- Public paths: Rust library consumers and TypeScript contract importers.
- Adjacent paths: existing contract modules, `lib.rs` module declarations, `bun run *` / Cargo checks.
- Migration path: additive contract, tracing module, fixtures, tests.
- Rollback path: remove the added TraceContext modules, fixtures, contract doc, proposal, and module declarations.
- Observability: Rust contract tests and Vitest tests expose DTO drift, actor drift, create success behavior, and all documented error variants.

## Findings

### Public surface matches the declaration

- Rust: `src-tauri/src/lib.rs:4` adds `pub mod tracing;`; `src-tauri/src/contracts/mod.rs:3` adds `pub mod trace_context;`. Public symbols introduced are limited to:
  - `agent_harness_lib::contracts::trace_context::{TraceContext, TraceActor, TraceContextError}` (`src-tauri/src/contracts/trace_context.rs:5, 21, 32`).
  - `agent_harness_lib::tracing::trace_context::{parse_trace_actor, create_trace_context}` (`src-tauri/src/tracing/trace_context.rs:8, 21`).
  - `NEXT_CORRELATION_COUNTER` is module-private (`tracing/trace_context.rs:6`); `generate_correlation_id` is module-private (`tracing/trace_context.rs:50`). Consumers cannot tamper with the counter or override ID format.
- TypeScript: `src/contracts/trace-context.ts` exports `TRACE_ACTORS`, `TraceActor` type, `TRACE_CONTEXT_ERRORS`, `TraceContextError` type, `TraceContext` interface, `parseTraceContext`, `parseTraceActor`, `parseTraceContextError`. No default export, no module-side-effect code. The `isTraceContext` / `isTraceActor` / `isTraceContextError` predicates are file-private (`trace-context.ts:55, 94, 98`).
- The exposed surface is exactly what the contract calls "Rust library consumers and TypeScript contract importers."

### Adjacent paths remain unchanged

- WU-0A-01 inert scaffold:
  - `src-tauri/src/lib.rs:7-12` is unchanged in shape: `tauri::Builder::default().setup(|_app| Ok(())).run(...)`. Zero `generate_handler!` invocations; the WU-0A-01 contract assertion `phase_0a_registers_no_value_slice_commands` (`lib.rs:20-25`) still passes (`registered_command_count()` still returns `0` at `lib.rs:14-16`).
  - No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`.
  - No new bundle targets, icons, or Tauri permissions.
- WU-0A-02 settings loader:
  - No edits to `src-tauri/src/settings.rs` or `src-tauri/src/contracts/harness_settings.rs`. The settings DTO, log-level union, error taxonomy, and loader are entirely untouched.
  - `src/contracts/harness-settings.ts` is unchanged.
- WU-0A-05 event-topic taxonomy:
  - No edits to `src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src/contracts/event-topic.ts`, or any WU-0A-05 fixture. Topic taxonomy and Phase 0A live-topic gating are byte-identical.
- `Cargo.toml` and `Cargo.lock` are unchanged (`git diff HEAD -- src-tauri/Cargo.toml src-tauri/Cargo.lock` is empty), so the `serde` / `serde_json` runtime deps from WU-0A-02 carry over without further churn.

### Blast radius

- Greenfield Phase 0A: no prior runtime users of `agent_harness_lib::tracing` or `src/contracts/trace-context`. The new public API is purely additive; nothing pre-existing depends on it.
- The `tracing` module is a new crate-internal namespace; reordering or renaming inside it cannot affect the WU-0A-01 / WU-0A-02 / WU-0A-05 namespaces.
- Note: Rust has a `tracing` crate in the wider ecosystem; this module name does not collide with it because no `tracing = "..."` dependency exists in `Cargo.toml`. If a future WU pulls in the `tracing` crate, the local `crate::tracing` module would still resolve unambiguously inside the lib crate.
- The fixtures live under `product-strategy/contracts/fixtures/wu-0a-09/`, namespaced by WU. They cannot collide with WU-0A-01 / WU-0A-02 / WU-0A-05 fixture sets.
- No transitive dep changes; no feature flags toggled.

### Migration / rollback path

- Migration: none. No persistent state, no schema, no on-disk artifacts produced by `create_trace_context`. The atomic counter is process-local and resets per process — acceptable because the contract never mandates ID stability across processes.
- Rollback: delete the added files (`src-tauri/src/tracing/`, `src-tauri/src/contracts/trace_context.rs`, `src-tauri/tests/trace_context_contract.rs`, `src/contracts/trace-context.ts`, `src/test/trace-context.test.ts`, `proposals/09-wu-0a-09.md`, `product-strategy/contracts/wu-0a-09-trace-context-correlation-schema.md`, `product-strategy/contracts/fixtures/wu-0a-09/`) and revert the two one-line additions to `src-tauri/src/lib.rs` and `src-tauri/src/contracts/mod.rs`. No stateful reconciliation needed.

### Observability

- DTO shape and serde round-trip: `src-tauri/tests/trace_context_contract.rs:64-89` round-trips both `canonical-context.json` (all optional fields) and `minimal-context.json` (required-only) and asserts that an unknown field (`extra: true`) is rejected by serde.
- Actor taxonomy: `trace_context_contract.rs:91-117` loops every variant in `trace-actors.json` through both `parse_trace_actor` and serde round-trip, then asserts `"unknown"` is rejected by both paths.
- Create success: `trace_context_contract.rs:119-174` exercises minimal and full helper inputs from `create-success-{minimal,full}.json` and asserts both correlation-ID non-emptiness and exact preservation of every supplied field.
- Create errors: `trace_context_contract.rs:176-192` loops every case in `create-errors.json` and asserts the documented error variant is reached.
- Error taxonomy round-trip: `trace_context_contract.rs:194-223` asserts `trace-context-errors.json` matches the canonical-order list, every variant round-trips through serde, and unknown variants are rejected.
- TS parity: `src/test/trace-context.test.ts:19-91` covers actor union equality (with `// @ts-expect-error` compile-time check at line 34-35), DTO parse/round-trip, invalid-context rejection by `invalid-contexts.json`, and error union equality (with `// @ts-expect-error` at line 79-80) plus parser round-trip against `create-errors.json`.

### Boundary observations (informational)

- INFO: Correlation IDs prefix `trace-` is convenient for grep / log scanning but is not a contract surface — tests assert only non-emptiness (`trace_context_contract.rs:130, 156`). A future WU could change the format without breaking contract observability.
- INFO: `TraceActor` derives `Copy` (`contracts/trace_context.rs:19`), making it cheap to pass to `create_trace_context` from a typed call site once parsing is no longer needed; safe because the enum is unit-only.

## Verdict

**LOW.** Public surface is the declared library types + parser + creation helper, nothing else. The WU-0A-01 inert scaffold is untouched in shape and behavior — still command-free, capability-minimal — and the WU-0A-02 settings loader plus WU-0A-05 event-topic taxonomy are byte-identical. Migration is additive only, rollback is a clean revert, and every acceptance criterion is observable via at least one test that pins it to a fixture rather than to internal state.
