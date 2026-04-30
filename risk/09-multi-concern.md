# WU-0A-09 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-09 `TraceContext` DTO + `TraceActor` taxonomy + `TraceContextError` taxonomy + `parse_trace_actor` + `create_trace_context` with bilingual contract tests — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-09.md`) explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Concern enumeration

Files added/modified on the branch group cleanly under one logical concern: "ship the Phase 0A `TraceContext` correlation DTO + `TraceActor` taxonomy + `TraceContextError` taxonomy + `create_trace_context` helper, with bilingual contract tests."

| Group | Files | Why required for this single concern |
|---|---|---|
| Rust contract types | `src-tauri/src/contracts/trace_context.rs`, `src-tauri/src/contracts/mod.rs` (one-line addition) | The Rust struct + actor enum + error enum the WU contract owns (`product-strategy/contracts/wu-0a-09-trace-context-correlation-schema.md:9-62`). |
| Rust tracing module | `src-tauri/src/tracing/trace_context.rs`, `src-tauri/src/tracing/mod.rs`, `src-tauri/src/lib.rs` (one-line addition) | `parse_trace_actor` + `create_trace_context` + correlation-ID generator per contract creation-helper section (`…schema.md:36-69`). |
| TypeScript contract types + parser | `src/contracts/trace-context.ts` | TS interface, actor union, error union, `parseTraceContext`, `parseTraceActor`, `parseTraceContextError` required by ticket criteria 1-2. |
| Rust contract test | `src-tauri/tests/trace_context_contract.rs` | Fixture-backed coverage of DTO round-trip, actor taxonomy, create success / error reachability, error round-trip. |
| TS contract test | `src/test/trace-context.test.ts` | Same coverage on the TS side, including `// @ts-expect-error` compile-time checks that the unions reject unknown strings. |
| Fixtures | `product-strategy/contracts/fixtures/wu-0a-09/{trace-actors,trace-context-errors,canonical-context,minimal-context,create-success-minimal,create-success-full,create-errors,invalid-contexts}.json` | Canonical inputs/outputs per contract `…schema.md:73-82`. |
| Contract spec + proposal | `product-strategy/contracts/wu-0a-09-trace-context-correlation-schema.md`, `proposals/09-wu-0a-09.md` | Phase-3 proposal and WU-owned contract document. |

## Severability check — can any group ship independently?

- **Rust contract types alone** (without the tracing module): would not satisfy ticket criteria 3-7 (creation helper behavior). The actor enum and error enum have no observable use without the parser and creation helper.
- **Rust tracing module alone** (without the contract types): does not compile — `parse_trace_actor` returns `Result<TraceActor, TraceContextError>` and `create_trace_context` returns `Result<TraceContext, TraceContextError>`.
- **TypeScript types + parser alone** (without Rust + fixtures): cannot satisfy criterion 1 (Rust → JSON → TS DTO parity) because the canonical/minimal fixtures are the cross-language pin.
- **Fixtures alone**: cannot ship — they have no consumer.
- **Tests alone**: cannot ship — they reference symbols that only exist with the rest of the change.
- **Contract spec / proposal alone**: required workflow artifacts for this WU; not a separable concern.
- **`lib.rs` / `contracts/mod.rs` wiring**: cannot ship without the modules they declare; would break the build.

No group is independently shippable.

## Cross-WU contamination check

- No file references WU-0A-03 (`LocalStorageLayout`), WU-0A-04, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-10+, WU-0A-14, or any later phase ticket.
- No GraphStore migration files (`src-tauri/migrations/` is still absent), no audit-log code, no provider crates added (`Cargo.toml` is unchanged — `git diff HEAD -- src-tauri/Cargo.toml src-tauri/Cargo.lock` is empty), no subprocess code (no `std::process` / `tokio::process` imports), no `tauri::generate_handler!`, no `app.manage(...)`, no IPC commands, no event payload structs that reference TraceContext. The WU-0A-01 scaffold contract assertion `phase_0a_registers_no_value_slice_commands` still passes (`src-tauri/src/lib.rs:20-25`).
- No edits to `src-tauri/src/settings.rs`, `src-tauri/src/contracts/harness_settings.rs`, `src/contracts/harness-settings.ts`, or any WU-0A-02 fixture under `product-strategy/contracts/fixtures/wu-0a-02/`. The settings loader is byte-identical.
- No edits to `src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src/contracts/event-topic.ts`, or any WU-0A-05 fixture under `product-strategy/contracts/fixtures/wu-0a-05/`. The event-topic taxonomy is byte-identical.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The user-facing shell and capability set are untouched.
- `proposals/` adds only `09-wu-0a-09.md`; no other proposal snuck in. `product-strategy/contracts/fixtures/` adds only `wu-0a-09/`; no cross-WU fixtures touched.

## Observations

- The tracing module is created as a new namespace (`src-tauri/src/tracing/`) rather than living under `contracts/`. The parser and creation helper own control flow and ID generation, so they belong in `tracing/trace_context.rs`; the DTO + actor + error types are correctly placed in `contracts/trace_context.rs` next to `event_topic.rs` and `harness_settings.rs`. Two-file split mirrors WU-0A-02 (DTO vs. loader) and WU-0A-05 (taxonomy vs. parser), and keeps the change scoped to one concern instead of inventing a shared "correlation" crate.
- Both contract tests use shared fixture-loading helpers (`fixture_path`, `read_json`, `create_from_input`) defined inline in the Rust test file rather than a new shared crate — keeps the change scoped to one concern.
- `invalid-contexts.json` covers a different shape from `create-errors.json` (validator-reject cases vs. creation-error cases). Both are required to fully observe the contract — neither is redundant nor scope-bleed.

## Verdict

**LOW.** This is a single-concern PR for WU-0A-09. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, no group serves another WU, and there is no contamination from WU-0A-01 / WU-0A-02 / WU-0A-05 (all untouched) or any later WU (no GraphStore, audit log, providers, optimizers, workers, questions, recovery, budget, IPC, app-state, subprocesses, payload schemas, or storage-layout work). The single-concern handoff note is honored.
