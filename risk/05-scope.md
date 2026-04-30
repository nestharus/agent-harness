# WU-0A-05 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-05 boundary (the 10-variant `EventTopic` taxonomy + 2-variant `EventTopicError` + `parse_event_topic` parser + Phase 0A live-topic gating + bilingual fixtures), or does it bleed into IPC subscription wiring, event payload schemas, GraphStore, providers, optimizers, workers, questions, recovery, budget, audit, or any later-domain producer work the proposal anti-scope reserves for later WUs (`proposals/05-wu-0a-05.md:21-26`)?

## Findings

### In-scope, confirmed

- Rust `EventTopic` enum encodes exactly the 10 documented variants in canonical order with `#[serde(rename_all = "lowercase")]`, matching the contract (`src-tauri/src/contracts/event_topic.rs:3-16`, contract `product-strategy/contracts/wu-0a-05-event-topic-taxonomy.md:9-20`).
- Rust `EventTopicError` encodes exactly `UnknownTopic` and `EmptyTopic` and serializes via default serde to those PascalCase strings (`event_topic.rs:18-22`), matching the contract (`…taxonomy.md:34-37`) and the `parse-errors.json` fixture's `"error"` field.
- `parse_event_topic(raw: &str)` performs the documented exact-match: empty → `EmptyTopic`, the 10 lowercase strings → their variants, anything else → `UnknownTopic` (`src-tauri/src/events/topic.rs:18-33`). No trimming, no case folding, no aliasing — confirmed by the negative fixtures `"Runtime"` and `" runtime "` (`product-strategy/contracts/fixtures/wu-0a-05/parse-errors.json:11-17`).
- Phase 0A live-topic gating exposes only `Runtime`: `PHASE_0A_LIVE_TOPICS = [Runtime]` (`topic.rs:16`), `phase_0a_live_topics()` returns that slice (`topic.rs:35-37`), `is_phase_0a_live_topic(topic)` discriminates (`topic.rs:39-41`). Matches `phase-0a-live-topics.json` and `inert-topics.json`.
- TypeScript mirror: `EVENT_TOPICS` const tuple in canonical order (`src/contracts/event-topic.ts:1-12`), `EVENT_TOPIC_ERRORS` covering both variants (`event-topic.ts:16`), `PHASE_0A_LIVE_TOPICS` constrained `satisfies readonly EventTopic[]` (`event-topic.ts:20`), `parseEventTopic` performs the same exact-match (`event-topic.ts:32-42`).

### Anti-scope honored

The proposal anti-scope (`proposals/05-wu-0a-05.md:21-26`) bars: IPC subscription commands, event payload schemas, and producer work for graph/render/provider/optimizer/worker/question/recovery/budget/audit. Verified:

- No `tauri::generate_handler!` invocation; `src-tauri/src/lib.rs:7-10` still uses `tauri::Builder::default().setup(|_app| Ok(()))`. `registered_command_count()` still returns `0` and the WU-0A-01 assertion `phase_0a_registers_no_value_slice_commands` still passes (`lib.rs:13-24`).
- No event payload structs anywhere in the new files — only the topic enum and the error enum. The contract is taxonomy-only.
- No GraphStore/provider/optimizer/worker/question/recovery/budget/audit producers: the names appear *only* as enum variants and as fixture strings. The `is_phase_0a_live_topic` predicate intentionally guards against any later-domain producer accidentally using these in Phase 0A.
- No subprocess code (no `std::process` / `tokio::process` imports), no `app.manage(...)`, no `tauri::State`, no global statics, no SQL/migration files.

### Adjacent-path additions (in-scope)

- `src-tauri/src/lib.rs:1-3`: adds `pub mod events;` next to the existing `contracts;` and `settings;`. One-line module declaration. Required to expose the new parser to the integration test crate.
- `src-tauri/src/contracts/mod.rs:1`: adds `pub mod event_topic;` next to `harness_settings;`. One-line module declaration.
- `src-tauri/src/events/mod.rs`: single line `pub mod topic;` — minimal events-namespace seed.
- No `Cargo.toml` / `Cargo.lock` changes (`git diff HEAD -- src-tauri/Cargo.toml src-tauri/Cargo.lock` is empty). The serde / serde_json runtime deps from WU-0A-02 cover the new contract type derives.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The shell, capabilities, and bundle config are untouched.

### Potential concerns — none rising to MEDIUM

- INFO: `phase_0a_live_topics()` returns `&'static [EventTopic]` and `is_phase_0a_live_topic(topic)` is a public helper. They are arguably gating policy rather than parser/taxonomy. They land here because the contract section "Phase 0A Live Topics" (`…taxonomy.md:45-61`) and the ticket acceptance criterion "the `runtime` topic is the only Phase 0A-origin topic that may emit real payloads" are scoped to this WU. Both helpers are loop-bound to the inert/live fixtures (`src-tauri/tests/event_topic_contract.rs:117-142`); without them the gating criterion is unobservable. On-concern.
- INFO: `EventTopicError` derives `std::error::Error` and a `Display` impl that synthesizes the wire string via `serde_json::to_string` (`event_topic.rs:24-31`). Convenience for `?` interop, ties human form to wire form. Not a scope expansion.

## Verdict

**LOW.** Implementation lands exactly the taxonomy, parser, error taxonomy, and Phase 0A live-topic gating that the WU contract and ticket call out. Anti-scope is honored end-to-end (no IPC, no payloads, no producers, no GraphStore/providers/optimizers/workers/questions/recovery/budget/audit), and adjacent edits are only the unavoidable module wiring for the new files.
