# WU-0A-05 Contract: EventTopic Taxonomy

## Ownership

This WU owns the shared Phase 0A event topic taxonomy, topic parser, and parse error taxonomy. It does not own IPC subscription commands, event payload schemas, event producer implementation, GraphStore, providers, optimizers, workers, questions, recovery flows, budget accounting, or audit streams.

## EventTopic

`EventTopic` serializes as one of these exact lowercase strings:

- `graph`
- `render`
- `provider`
- `optimizer`
- `worker`
- `question`
- `recovery`
- `budget`
- `audit`
- `runtime`

Rust and TypeScript must preserve this set exactly. Unknown strings are rejected.

## Parser

```text
parse_event_topic(raw: string) -> Result<EventTopic, EventTopicError>
```

Parsing is exact-match against the documented lowercase strings. No trimming, case folding, aliasing, or prefix matching is allowed.

## Errors

`EventTopicError` serializes as one of these exact strings:

- `UnknownTopic`
- `EmptyTopic`

Parser behavior:

- `parse_event_topic("")` returns `EmptyTopic`.
- Any other undocumented string returns `UnknownTopic`.
- A documented lowercase topic string returns the corresponding `EventTopic`.

## Phase 0A Live Topics

Only `runtime` may be used as a real Phase 0A event payload emission target.

The other documented topics are intentionally present as inert later-domain placeholders:

- `graph`
- `render`
- `provider`
- `optimizer`
- `worker`
- `question`
- `recovery`
- `budget`
- `audit`

They must not be removed from the taxonomy, but Phase 0A must not treat them as live emission targets.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-05/`:

- `event-topics.json`: all documented topic strings in canonical order.
- `parse-success.json`: documented successful parser cases.
- `parse-errors.json`: documented parser error cases.
- `phase-0a-live-topics.json`: topics allowed to emit real Phase 0A payloads.
- `inert-topics.json`: later-domain placeholders retained for taxonomy stability.

## Test Handoff

- Rust contract test: `src-tauri/tests/event_topic_contract.rs`.
- TypeScript contract test: `src/test/event-topic.test.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
