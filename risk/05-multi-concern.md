# WU-0A-05 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-05 `EventTopic` taxonomy + parser + Phase 0A live-topic gating — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-05.md`) explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Concern enumeration

Files added/modified on the branch group cleanly under one logical concern: "ship the Phase 0A `EventTopic` taxonomy + `EventTopicError` taxonomy + `parse_event_topic` parser + Phase 0A live-topic gating, with bilingual contract tests."

| Group | Files | Why required for this single concern |
|---|---|---|
| Rust contract types | `src-tauri/src/contracts/event_topic.rs`, `src-tauri/src/contracts/mod.rs` (one-line addition) | The Rust enum + error enum the WU contract owns (`product-strategy/contracts/wu-0a-05-event-topic-taxonomy.md:9-37`). |
| Rust event-topic module | `src-tauri/src/events/topic.rs`, `src-tauri/src/events/mod.rs`, `src-tauri/src/lib.rs` (one-line addition) | `parse_event_topic` plus `EVENT_TOPICS`, `phase_0a_live_topics`, `is_phase_0a_live_topic` per contract parser + Phase 0A live-topic clauses (`…taxonomy.md:24-61`). |
| TypeScript contract types + parser | `src/contracts/event-topic.ts` | TS union, error union, `parseEventTopic`, `EventTopicParseError`, `isPhase0aLiveTopic` required by ticket criteria 1-2 and 5-6. |
| Rust contract test | `src-tauri/tests/event_topic_contract.rs` | Fixture-backed coverage of taxonomy round-trip, parser success / error, and Phase 0A inert / live gating. |
| TS contract test | `src/test/event-topic.test.ts` | Same coverage on the TS side, including a `// @ts-expect-error` compile-time check that the union rejects unknown strings. |
| Fixtures | `product-strategy/contracts/fixtures/wu-0a-05/{event-topics,parse-success,parse-errors,inert-topics,phase-0a-live-topics}.json` | Canonical inputs/outputs per contract `…taxonomy.md:63-71`. |
| Contract spec + proposal | `product-strategy/contracts/wu-0a-05-event-topic-taxonomy.md`, `proposals/05-wu-0a-05.md` | Phase-3 proposal and WU-owned contract document. |

## Severability check — can any group ship independently?

- **Rust contract types alone** (without the parser module): would not satisfy ticket criteria 2-5 (parser behavior). The error enum has no observable use without the parser.
- **Rust parser module alone** (without the enum types): does not compile — the parser returns `Result<EventTopic, EventTopicError>`.
- **TypeScript types + parser alone** (without Rust + fixtures): cannot satisfy criterion 1 (Rust → JSON → TS taxonomy parity) because the canonical fixture is the cross-language pin.
- **Phase 0A live-topic gating helpers alone** (without the taxonomy): would have nothing to gate; the `is_phase_0a_live_topic` predicate consumes `EventTopic`.
- **Fixtures alone**: cannot ship — they have no consumer.
- **Tests alone**: cannot ship — they reference symbols that only exist with the rest of the change.
- **Contract spec / proposal alone**: required workflow artifacts for this WU; not a separable concern.
- **`lib.rs` / `contracts/mod.rs` wiring**: cannot ship without the modules they declare; would break the build.

No group is independently shippable.

## Cross-WU contamination check

- No file references WU-0A-03 (`LocalStorageLayout`), WU-0A-04, WU-0A-06+, WU-0A-14, or any later phase ticket.
- No GraphStore migration files (`src-tauri/migrations/` is still absent), no provider crates added (`Cargo.toml` is unchanged — `git diff HEAD -- src-tauri/Cargo.toml src-tauri/Cargo.lock` is empty), no subprocess code (no `std::process` / `tokio::process` imports), no `tauri::generate_handler!`, no `app.manage(...)`, no IPC commands, no event payload structs. The WU-0A-01 scaffold contract assertion `phase_0a_registers_no_value_slice_commands` still passes (`src-tauri/src/lib.rs:18-24`).
- No edits to `src-tauri/src/settings.rs`, `src-tauri/src/contracts/harness_settings.rs`, `src/contracts/harness-settings.ts`, or any WU-0A-02 fixture under `product-strategy/contracts/fixtures/wu-0a-02/`. The settings loader is byte-identical.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The user-facing shell and capability set are untouched.
- `proposals/` now contains `01-wu-0a-01.md`, `02-wu-0a-02.md`, `05-wu-0a-05.md`; no other proposal snuck in. `product-strategy/contracts/fixtures/` adds only `wu-0a-05/`; no cross-WU fixtures touched.

## Observations

- The events module is created as a new namespace (`src-tauri/src/events/`) rather than living under `contracts/`. The parser owns control flow over the enum and is correctly placed in `events/topic.rs`; the enum itself is correctly placed in `contracts/event_topic.rs` next to `harness_settings.rs`. Two-file split mirrors the WU-0A-02 split (contracts type vs. settings loader) and keeps the change scoped to one concern instead of inventing a shared "topics" crate.
- Both contract tests use shared loop-driven assertion patterns, so adding a future topic / error variant only requires updating the fixture; no new variant could be silently absent in a future WU.

## Verdict

**LOW.** This is a single-concern PR for WU-0A-05. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, no group serves another WU, and there is no contamination from WU-0A-01 / WU-0A-02 (both untouched) or any later WU (no GraphStore, providers, optimizers, workers, questions, recovery, budget, audit, IPC, app-state, subprocesses, payload schemas, or storage-layout work). The single-concern handoff note is honored.
