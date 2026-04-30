# WU-0A-09 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-09's purpose (a Rust+TS `TraceContext` DTO whose actor taxonomy, optional fields, and creation-error reachability are fixture-pinned)? Are TODOs, stubbed branches, weak assertions, or deferred behaviors hiding incomplete work?

## Shortcuts examined

### S1 — `parse_trace_actor` is a hand-rolled `match` rather than a derive / from_str

`src-tauri/src/tracing/trace_context.rs:8-19` matches each documented lowercase string explicitly. Justified: the contract requires *exact* lowercase matching with no normalization (`product-strategy/contracts/wu-0a-09-trace-context-correlation-schema.md:33`); a strum-style derive that allowed case-insensitive or alias variants would break the contract. The hand-rolled match is the simplest implementation that matches contract semantics. Loop-bound to `trace-actors.json` and `create-errors.json` in `src-tauri/tests/trace_context_contract.rs:91-117, 176-192`. **Justified.**

### S2 — `create_trace_context` accepts `actor: &str` rather than `actor: TraceActor`

`tracing/trace_context.rs:21-33` takes the raw string and parses inside the helper, even though the ticket's pseudo-signature shows `actor: TraceActor` (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-09.md` Scope block). Justified explicitly by Assumption A3 in the proposal (`proposals/09-wu-0a-09.md:45`): the acceptance criterion `create_trace_context(workspace_id, "unknown", ...)` must reach `UnknownActor`, which requires a string-typed boundary. The successful result still stores `TraceActor` as the typed enum (`tracing/trace_context.rs:33, 44`). **Justified.**

### S3 — Correlation ID is `format!("trace-{timestamp}-{counter}")`, not UUID

`tracing/trace_context.rs:50-58` builds the ID from `SystemTime::now()` nanoseconds plus an `AtomicU64` counter. Assumption A1 explicitly limits Phase 0A to "opaque non-empty correlation IDs" with no UUID / monotonic / persistence requirement (`proposals/09-wu-0a-09.md:43`); the contract only says "non-empty opaque" (`…schema.md:50`). Tests pin only non-emptiness (`trace_context_contract.rs:130-133, 156-159`), so the format is intentionally not a contract surface. The atomic counter prevents the (unlikely) two-IDs-in-one-nanosecond collision without introducing a uuid dep. **Justified.**

### S4 — `TraceContextError` PascalCase strings via default serde, no explicit `rename` per variant

`src-tauri/src/contracts/trace_context.rs:31-36` derives `Serialize, Deserialize` with no `rename_all`, so unit variants serialize to identifier names (`"EmptyWorkspaceId"`, `"UnknownActor"`, `"ParentWithoutInvocation"`). Matches the contract's exact-string table (`…schema.md:60-62`) and `trace-context-errors.json`, which is deserialized into `TraceContextError` directly (`trace_context_contract.rs:31-32, 195-223`). The Rust test would fail if the wire form drifted. **Justified.**

### S5 — `Display` for `TraceContextError` reuses serde

`contracts/trace_context.rs:38-45` formats by calling `serde_json::to_string(self)` and trimming the surrounding quotes — same pattern as WU-0A-05 `EventTopicError`. Couples the human-readable form to the wire form so they cannot drift. Not a shortcut around correctness. **Justified.**

### S6 — `parent_invocation_id` validation is `is_some() && invocation_id.is_none()`, not a richer state machine

`tracing/trace_context.rs:35-37` enforces only the parent-without-invocation rule; empty / whitespace / format of either ID is intentionally not validated. The contract explicitly says "Empty optional strings are preserved when supplied; this WU only validates the parent/invocation relationship" (`…schema.md:69`). The `create-errors.json` fixture pins `parent_invocation_id: "invocation-parent"` with no `invocation_id` → `ParentWithoutInvocation` (`fixtures/wu-0a-09/create-errors.json:18-26`). **Justified.**

### S7 — TS `parseTraceContext` uses an inline structural check rather than zod / a generated codec

`src/contracts/trace-context.ts:31-92`. Mirrors S1: exact-match, no normalization. The `as const` tuple `TRACE_ACTORS` plus `TraceActor = (typeof TRACE_ACTORS)[number]` makes the union the single source of truth on the TS side; `isTraceContext` enforces both presence of required fields and absence of unknown fields by sorted-key comparison (`trace-context.ts:60-78`), so the TS validator rejects the same `extra: true` shape the Rust serde rejects. Asserted against the fixture set in `src/test/trace-context.test.ts:19-91`. **Justified.**

### S8 — TS surfaces parse failures as plain `Error("Invalid TraceContext")` rather than a typed exception

`trace-context.ts:31-53`. The contract spec is wire-form-only and does not mandate an in-language signature for TS; the test loops `invalid-contexts.json` and asserts that every documented invalid shape throws (`trace-context.test.ts:59-67`). Idiomatic for TS, contract-aligned, observable. **Justified.**

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented!()`, or `todo!()` markers in `src-tauri/src/contracts/trace_context.rs`, `src-tauri/src/tracing/trace_context.rs`, `src-tauri/src/tracing/mod.rs`, `src/contracts/trace-context.ts`, `src-tauri/tests/trace_context_contract.rs`, or `src/test/trace-context.test.ts`.
- No commented-out arms in `parse_trace_actor`; the `_` arm is the documented `UnknownActor` fallthrough, not a stub.
- No `#[allow(dead_code)]`, no `#[ignore]` tests, no `it.skip(...)` / `it.todo(...)` in the vitest file.
- All three error variants are reachable from fixtures: `"" → EmptyWorkspaceId`, `"unknown" → UnknownActor`, parent-only → `ParentWithoutInvocation` (`create-errors.json:1-27`), exercised in a loop (`trace_context_contract.rs:176-192`).
- All seven actor variants are exercised by `trace-actors.json` and round-tripped in a loop (`trace_context_contract.rs:91-107`).
- The TS test's `// @ts-expect-error` lines on `const rejected: TraceActor = "unknown";` and `const rejected: TraceContextError = "OtherError";` (`trace-context.test.ts:34-35, 79-80`) are compile-time assertions that the unions actually reject strings outside the canonical set — not silenced lints.
- The TS `invalid-contexts.json` set covers missing-required, unknown-actor, extra-field, and non-string optional cases — same shape that Rust serde rejects (`trace_context_contract.rs:79-88`), giving symmetric drift detection.

## Verdict

**LOW.** Every shortcut is either documented in the contract (exact-match parsing, no normalization, opaque correlation IDs, parent/invocation-only validation), justified by the proposal anti-scope (no GraphStore / audit / persistence), or compensated by a negative fixture (`""`, `"unknown"`, parent-without-invocation, extra-field, non-string graph_ref). No stubs, no skipped tests, no TODOs; all three error variants and all seven actor variants are reachable, round-tripped, and pinned to fixtures rather than internal state.
