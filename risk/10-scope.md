# Risk Gate — Scope (WU-0A-10)

**Severity:** LOW

## Summary

Scope of the WU-0A-10 implementation matches the ticket. Code/test boundary, struct shape, status taxonomy, builder signature, error taxonomy, all 10 acceptance criteria, and the runtime-only topic invariant are exercised. No Phase 0B/0C/1+ behavior was added.

## Verification

### Code/test boundary
- Code boundary expected and present:
  - `src-tauri/src/tracing/backend_span_event.rs` ✓
  - `src-tauri/src/events/runtime_events.rs` ✓
  - `src-tauri/src/contracts/backend_span_event.rs` ✓
  - `src/contracts/backend-span-event.ts` ✓
  - `src-tauri/tests/backend_span_event_contract.rs` ✓
  - `src/test/backend-span-event.test.ts` ✓
- Test boundary present: `product-strategy/contracts/wu-0a-10-backend-span-event.md` and `product-strategy/contracts/fixtures/wu-0a-10/*.json` ✓
- `git diff main --stat` outside the boundary: only 3 single-line `pub mod ...` additions in `src-tauri/src/{contracts,events,tracing}/mod.rs`. These are minimal module wiring and necessary for the WU's modules to be reachable.

### BackendSpanEvent fields (7 documented)
`src-tauri/src/contracts/backend_span_event.rs:7-17` defines `BackendSpanEvent` with exactly: `span_event_id`, `trace_context`, `span_name`, `started_at`, `completed_at`, `status`, `error_ref` ✓ (`#[serde(deny_unknown_fields)]` enforces no surplus fields).

### BackendSpanStatus (3 variants)
`src-tauri/src/contracts/backend_span_event.rs:19-25` declares `Started`, `Completed`, `Failed` with `#[serde(rename_all = "lowercase")]` ✓.

### emit_backend_span signature
`src-tauri/src/tracing/backend_span_event.rs:9-16` signature:
```rust
emit_backend_span(
    trace_context: TraceContext,
    span_name: &str,
    started_at: &str,
    completed_at: Option<&str>,
    status: BackendSpanStatus,
    error_ref: Option<&str>,
) -> Result<BackendSpanEvent, BackendSpanError>
```
Matches contract. The `&str` / `Option<&str>` shape is idiomatic Rust; types preserve the caller-supplied values. ✓

### BackendSpanError variants (4)
`src-tauri/src/contracts/backend_span_event.rs:27-33` declares exactly: `EmptySpanName`, `CompletedBeforeStarted`, `FailedWithoutErrorRef`, `TraceContextInvalid` ✓.

### Acceptance criteria — all 10 exercised
| AC | Test |
|----|------|
| Round-trip preserves all 7 fields | `backend_span_event_round_trips_all_happy_path_fixtures` (Rust), `round-trips started, completed, and failed BackendSpanEvent fixture shapes` (TS) |
| Every `BackendSpanStatus` variant round-trips and is reachable | `backend_span_status_variants_round_trip_and_are_reachable_from_fixtures` |
| Started returns `status: "started"` and no completed_at | `emit_backend_span_started_returns_started_without_completion_or_error` |
| Completed preserves both timestamps when ordered | `emit_backend_span_completed_preserves_ordered_timestamps` |
| Failed preserves `error_ref` | `emit_backend_span_failed_preserves_error_ref` |
| Empty `span_name` → `EmptySpanName` | `backend_span_error_variants_are_reachable_through_documented_inputs` (fixture `error-empty-span-name.json`) |
| `completed_at < started_at` → `CompletedBeforeStarted` | same test (fixture `error-completed-before-started.json`) |
| Failed without `error_ref` → `FailedWithoutErrorRef` | same test (fixture `error-failed-without-error-ref.json`) |
| Invalid `TraceContext` → `TraceContextInvalid` | same test (fixture `error-trace-context-invalid.json`) + delegated-error assertion |
| Runtime topic only | `backend_span_runtime_event_uses_runtime_topic_only_and_adds_no_command` + `wu_0a_10_source_references_no_later_domain_event_topic_variants` |

### Runtime-only topic invariant
- Source-file scan in `wu_0a_10_source_references_no_later_domain_event_topic_variants` reads `tracing/backend_span_event.rs`, `events/runtime_events.rs`, `contracts/backend_span_event.rs` and asserts presence of `EventTopic::Runtime` and absence of every other documented variant (`Graph`, `Render`, `Provider`, `Optimizer`, `Worker`, `Question`, `Recovery`, `Budget`, `Audit`).
- Manual grep confirms `EventTopic::Runtime` is the only `EventTopic::*` reference within the WU's source files (`src-tauri/src/events/runtime_events.rs:11`); other `EventTopic` references in the repo live in pre-existing files (`events/topic.rs`, `events/ipc_event.rs`).

### Phase 0B/0C/1+ creep
None. No new GraphStore tables, migrations, providers, agents invocation, queue handlers, or budget/recovery scaffolds. No new IPC commands. The runtime envelope helper does not publish to a bus.

## Findings

None at MEDIUM or HIGH. The implementation stays within the documented scope.

## Verification commands run
- `cargo test --manifest-path src-tauri/Cargo.toml` — 0 failures across all suites.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` — clean.
- `bun run lint` / `bun run typecheck` / `bun run test` — clean.
- `git diff main --stat` — only 3 module-wiring lines outside the WU boundary.
