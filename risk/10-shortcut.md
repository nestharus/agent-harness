# Risk Gate — Shortcut / Placeholder (WU-0A-10)

**Severity:** LOW

## Summary

No stubs, TODOs, or tautological checks. ID generation, timestamp comparison, error variants, and status round-trip are real. One observation worth recording: the proposal states `TraceContextInvalid` "delegates to WU-0A-09's contract validation," but the implementation re-implements the equivalent invariants inline because WU-0A-09 only exposes `create_trace_context` (a constructor) and not a public validate-existing function. The behavior is functionally equivalent and the delegated-error fixture asserts the WU-0A-09 error name.

## Verification

### Span event ID generation
`src-tauri/src/tracing/backend_span_event.rs:68-76`:
```rust
fn generate_span_event_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let counter = NEXT_SPAN_EVENT_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("span-{timestamp}-{counter}")
}
```
Real unix-nanos timestamp + atomic counter. Not a stub or hardcoded id. Format mirrors WU-0A-09's `trace-{timestamp}-{counter}` pattern. ✓

### Timestamp values
`emit_backend_span` preserves caller-supplied `started_at` and `completed_at` strings via `to_string()` (lines 31, 43). Nothing is hardcoded; the builder takes the values from arguments. ✓

### `completed_at < started_at` comparison
`src-tauri/src/tracing/backend_span_event.rs:61-66`:
```rust
fn timestamp_before(candidate: &str, baseline: &str) -> bool {
    match (candidate.parse::<u128>(), baseline.parse::<u128>()) {
        (Ok(candidate), Ok(baseline)) => candidate < baseline,
        _ => candidate < baseline,
    }
}
```
Real comparison. When both strings are decimal-digit unix-nanos they compare numerically; otherwise the fallback uses lexicographic string comparison (still a real `<` on `&str`, not a tautology). The proposal A2 documents this surface. ✓

### TraceContextInvalid path
`src-tauri/src/tracing/backend_span_event.rs:50-59`:
```rust
fn validate_trace_context(trace_context: &TraceContext) -> Result<(), BackendSpanError> {
    if trace_context.correlation_id.is_empty()
        || trace_context.workspace_id.is_empty()
        || (trace_context.parent_invocation_id.is_some() && trace_context.invocation_id.is_none())
    {
        return Err(BackendSpanError::TraceContextInvalid);
    }
    Ok(())
}
```
- Reimplements WU-0A-09's invariants on a constructed `TraceContext` rather than calling a WU-0A-09 helper. WU-0A-09 today does not export a `validate_trace_context(&TraceContext)`; only `create_trace_context(...)` is public. The actor invariant is enforced by the `TraceActor` enum at deserialization time, so it cannot be violated here.
- The fixture `error-trace-context-invalid.json` records `delegated_trace_context_error: "EmptyWorkspaceId"`, and the contract test (`backend_span_event_contract.rs:251-266`) asserts that documented WU-0A-09 error name is preserved alongside `TraceContextInvalid`. So the cross-WU traceability is verified even though the call is not literal delegation.
- **Finding LOW:** Not literal delegation. Easy follow-up if WU-0A-09 later exposes a public validator; for now, behavior matches the contract.

### Error variants exhaustive — no `_` fallthrough
- `match status` in `emit_backend_span` (lines 23-37) covers all three `BackendSpanStatus` variants explicitly with no `_` arm. ✓
- `BackendSpanError` is `#[derive(Serialize, Deserialize)]` with no `#[serde(other)]` and no fallback variant. ✓

### Status string round-trip
`backend_span_status_variants_round_trip_and_are_reachable_from_fixtures` (`backend_span_event_contract.rs:146-184`) asserts the canonical fixture array `["started", "completed", "failed"]` round-trips through `serde_json::from_value` / `to_value` for every entry, and rejects unknown strings. The TS test `keeps the TypeScript status union aligned with the canonical Rust fixture` mirrors this. ✓

### TODOs / unimplemented stubs
Grep over the WU's source files (`tracing/backend_span_event.rs`, `events/runtime_events.rs`, `contracts/backend_span_event.rs`, `src/contracts/backend-span-event.ts`) finds no `todo!`, `unimplemented!`, `TODO`, or `FIXME`. ✓

## Findings

- **WU-0A-10-SHORTCUT-F01 (LOW):** `validate_trace_context` re-implements WU-0A-09's invariants inline rather than calling a shared validator. WU-0A-09 currently does not expose a public validate-existing function, so there is no shared API to call today. The fixture and contract test still record the delegated WU-0A-09 error name, preserving cross-WU traceability. No change required for this WU; consider unifying when WU-0A-09 grows a public validator.

No MEDIUM or HIGH findings.
