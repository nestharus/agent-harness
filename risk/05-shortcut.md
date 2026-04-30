# WU-0A-05 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-05's purpose (a Rust+TS topic taxonomy + parser whose every documented variant and error is fixture-pinned, with Phase 0A live-topic gating)? Are TODOs, stubbed branches, weak assertions, or deferred behaviors hiding incomplete work?

## Shortcuts examined

### S1 — Parser is a hand-rolled `match` rather than a derive / from_str

`src-tauri/src/events/topic.rs:18-33` matches each documented lowercase string explicitly. Justified: the contract requires *exact* lowercase matching with no normalization (`product-strategy/contracts/wu-0a-05-event-topic-taxonomy.md:30`), so a strum / `EnumString` derive that allowed case-insensitive or alias variants would break the contract. The hand-rolled match is the simplest implementation that matches contract semantics exactly. Loop-bound to `parse-success.json` and `parse-errors.json` in `src-tauri/tests/event_topic_contract.rs:81-114`. **Justified.**

### S2 — Empty-string check is exact equality, not `trim().is_empty()`

`topic.rs:20`: only `""` returns `EmptyTopic`. A single space, tab, or `" runtime "` returns `UnknownTopic` instead. Justified: the contract says parsing is exact-match with "no trimming" (`…taxonomy.md:30`); whitespace must therefore be untrusted input that fails as `UnknownTopic`, not silently coerced to `EmptyTopic`. The `parse-errors.json` fixture pins `" runtime "` → `UnknownTopic` (`fixtures/wu-0a-05/parse-errors.json:14-17`), so this distinction is observed by tests, not just stated. **Justified.**

### S3 — `EventTopicError` PascalCase strings via default serde, no explicit `rename` per variant

`src-tauri/src/contracts/event_topic.rs:18-22` derives `Serialize, Deserialize` with no `rename_all`, so unit variants serialize to their identifier names (`"UnknownTopic"`, `"EmptyTopic"`). Matches the contract's exact-string table (`…taxonomy.md:34-37`) and the `parse-errors.json` fixture's `"error"` field, which is deserialized into `EventTopicError` directly (`event_topic_contract.rs:19-23, 102-114`). The Rust test would fail loudly if the wire form ever drifted. **Justified.**

### S4 — `Display` for `EventTopicError` reuses serde

`event_topic.rs:24-31` formats by calling `serde_json::to_string(self)` and trimming the surrounding quotes. Couples the human-readable form to the wire form so they cannot drift. Not a shortcut around correctness. **Justified.**

### S5 — Phase 0A live topics is a static `[EventTopic; 1]` literal, not derived from the enum

`topic.rs:16`: `const PHASE_0A_LIVE_TOPICS: [EventTopic; 1] = [EventTopic::Runtime];`. Hand-maintained against the inert-topics / phase-0a-live-topics fixtures. Justified by ticket criterion "the `runtime` topic is the only Phase 0A-origin topic that may emit real payloads" — the constraint is *policy* against the taxonomy, not a derivation from it; hand-coding plus fixture-pinning is the right shape. The contract test exercises every inert variant and asserts `Runtime` is the lone live one (`event_topic_contract.rs:117-142`). **Justified.**

### S6 — TS parser uses an inline `EVENT_TOPICS.includes(value)` rather than zod / a generated codec

`src/contracts/event-topic.ts:32-50`. Mirrors S1: exact-match, no normalization, simplest possible implementation. The const tuple `as const` plus `EventTopic = (typeof EVENT_TOPICS)[number]` makes the union the single source of truth on the TS side, asserted against the Rust-side `event-topics.json` fixture in `src/test/event-topic.test.ts:23-43`. **Justified.**

### S7 — TS uses an `EventTopicParseError` exception, Rust returns `Result`

`event-topic.ts:22-42` throws `EventTopicParseError` carrying `kind: EventTopicError`. The contract spec is wire-form-only and does not mandate an in-language signature for TS; the test loops `parse-errors.json` and asserts both the throw and the `kind` value (`event-topic.test.ts:54-68`). Idiomatic for TS, contract-aligned, observable. **Justified.**

### S8 — `EVENT_TOPICS` constant exported from Rust crate

`topic.rs:3-14` exports `pub const EVENT_TOPICS: [EventTopic; 10]`. Provides a stable iteration order for the contract test (`event_topic_contract.rs:55-78`) which compares it against `event-topics.json`. Without this, the order pinning would have to be re-asserted variant-by-variant. Single-purpose, on-concern. **Justified.**

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented!()`, or `todo!()` markers in `src-tauri/src/events/topic.rs`, `src-tauri/src/contracts/event_topic.rs`, `src/contracts/event-topic.ts`, `src-tauri/tests/event_topic_contract.rs`, or `src/test/event-topic.test.ts`.
- No commented-out arms in the parser `match`; the `_` arm is the documented `UnknownTopic` fallthrough, not a stub.
- No `#[allow(dead_code)]`, no `#[ignore]` tests, no `it.skip(...)` / `it.todo(...)` in the vitest file.
- Both error variants are reachable from fixtures: `"" → EmptyTopic` and `"unknown"|"Runtime"|" runtime " → UnknownTopic` (`parse-errors.json`).
- Every taxonomy variant is exercised by the success fixture (`parse-success.json` covers all 10), and the Rust test asserts a serde round-trip per variant (`event_topic_contract.rs:64-72`).
- The TS file's `// @ts-expect-error` line on `const rejected: EventTopic = "unknown";` (`event-topic.test.ts:36-37`) is a compile-time assertion that the union actually rejects strings outside the canonical set — not a silenced lint.
- Both contract tests use shared loop-driven assertion patterns, so adding a future topic only requires updating the fixture; no new variant could be silently absent.

## Verdict

**LOW.** Every shortcut is either documented in the contract (exact-match parsing, no normalization), justified by the proposal anti-scope (no IPC / payloads), or compensated by a negative fixture (`"Runtime"`, `" runtime "`, `"unknown"`, `""`). No stubs, no skipped tests, no TODOs; both error variants and all 10 topic variants are reachable, round-tripped, and pinned to fixtures rather than to internal state.
