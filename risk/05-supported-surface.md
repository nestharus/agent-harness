# WU-0A-05 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does deployment-mode + customer-cohort + adjacent-public-paths analysis match the WU-0A-05 supported-surface declaration? What is the blast radius for adjacent paths (the WU-0A-01 inert scaffold and the WU-0A-02 settings loader)? Migration / rollback / observability story?

## Supported-surface contract (proposal `proposals/05-wu-0a-05.md:28-36`)

- Deployment mode: local Phase 0A desktop scaffold.
- Customer cohort: developer/test harness consumers only.
- Public paths: Rust library consumers and TypeScript contract importers.
- Adjacent paths: existing Tauri bootstrap, React shell, and settings loader remain unchanged.
- Migration path: additive contract, event-topic module, fixtures, and tests.
- Rollback path: remove the added WU-0A-05 files and module declarations.
- Observability: fixture-backed tests verify exact taxonomy, parser behavior, inert placeholders, and live-topic gating.

## Findings

### Public surface matches the declaration

- Rust: `src-tauri/src/lib.rs:2` adds `pub mod events;`; `src-tauri/src/contracts/mod.rs:1` adds `pub mod event_topic;`. Public symbols introduced are limited to:
  - `agent_harness_lib::contracts::event_topic::{EventTopic, EventTopicError}` (`src-tauri/src/contracts/event_topic.rs:5-22`).
  - `agent_harness_lib::events::topic::{EVENT_TOPICS, parse_event_topic, phase_0a_live_topics, is_phase_0a_live_topic}` (`src-tauri/src/events/topic.rs:3, 18, 35, 39`).
  - `PHASE_0A_LIVE_TOPICS` is a private `const` (`topic.rs:16`); access goes through the two helpers, so the slice cannot be mutated by consumers.
- TypeScript: `src/contracts/event-topic.ts` exports `EVENT_TOPICS`, `EventTopic` type, `EVENT_TOPIC_ERRORS`, `EventTopicError` type, `PHASE_0A_LIVE_TOPICS`, `EventTopicParseError`, `parseEventTopic`, `isPhase0aLiveTopic`. No default export, no module-side-effect code.
- The exposed surface is exactly what the contract calls "Rust library consumers and TypeScript contract importers."

### Adjacent paths remain unchanged

- WU-0A-01 inert scaffold:
  - `src-tauri/src/lib.rs:6-11` is unchanged in shape: `tauri::Builder::default().setup(|_app| Ok(())).run(...)`. Zero `generate_handler!` invocations; the WU-0A-01 contract assertion `phase_0a_registers_no_value_slice_commands` (`lib.rs:18-24`) still passes (`registered_command_count()` still returns `0` at `lib.rs:13-15`).
  - No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The visible application root and capability set are unchanged.
  - No new bundle targets, no new icons, no new Tauri permissions.
- WU-0A-02 settings loader:
  - No edits to `src-tauri/src/settings.rs` or `src-tauri/src/contracts/harness_settings.rs`. The settings DTO, log-level union, error taxonomy, and loader are entirely untouched.
  - `src/contracts/harness-settings.ts` is unchanged.
  - `Cargo.toml` and `Cargo.lock` are unchanged (`git diff HEAD -- src-tauri/Cargo.toml src-tauri/Cargo.lock` is empty), so the `serde` / `serde_json` runtime dep promotion from WU-0A-02 carries over without further churn.

### Blast radius

- Greenfield Phase 0A: no prior runtime users of `agent_harness_lib::events` or `src/contracts/event-topic`. The new public API is purely additive; nothing pre-existing depends on it.
- The `events` module is a new crate-internal namespace; reordering or renaming inside it cannot affect the WU-0A-01 / WU-0A-02 namespaces.
- The fixtures live under `product-strategy/contracts/fixtures/wu-0a-05/`, namespaced by WU. They cannot collide with WU-0A-01 / WU-0A-02 fixture sets.
- No transitive dep changes; no feature flags toggled.

### Migration / rollback path

- Migration: none. No persistent state, no schema, no on-disk artifacts produced by parsing or by the gating helpers. Both functions are pure over their inputs.
- Rollback: delete the added files (`src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src-tauri/tests/event_topic_contract.rs`, `src/contracts/event-topic.ts`, `src/test/event-topic.test.ts`, `proposals/05-wu-0a-05.md`, `product-strategy/contracts/wu-0a-05-event-topic-taxonomy.md`, `product-strategy/contracts/fixtures/wu-0a-05/`) and revert the two one-line additions to `src-tauri/src/lib.rs` and `src-tauri/src/contracts/mod.rs`. No stateful reconciliation needed.

### Observability

- Taxonomy and serde round-trip: `src-tauri/tests/event_topic_contract.rs:55-78` asserts `EVENT_TOPICS` matches `event-topics.json` exactly and that every topic round-trips through serde with the documented lowercase string; also asserts that `"unknown"` is rejected by serde (not just the parser).
- Parser success: `event_topic_contract.rs:81-96` loops `parse-success.json` (10 cases, all variants).
- Parser errors: `event_topic_contract.rs:99-114` loops `parse-errors.json` (4 cases covering both error variants plus the no-normalization invariant).
- Phase 0A live-topic gating: `event_topic_contract.rs:117-142` asserts `phase_0a_live_topics() == ["runtime"]`, every inert topic from `inert-topics.json` is in the taxonomy and not live, and `Runtime` is live.
- TS parity: `src/test/event-topic.test.ts:18-83` covers union equality (with a `// @ts-expect-error` compile-time check at line 36-37), parser success, parser errors with `kind` discrimination, and Phase 0A inert/live gating against the same fixtures.

### Boundary observations (informational)

- INFO: `is_phase_0a_live_topic` and `isPhase0aLiveTopic` are advisory predicates — they do not *prevent* a future caller from emitting on an inert topic; they merely make the policy observable. That is the right shape for Phase 0A: enforcement belongs to the as-yet-unbuilt IPC subscription command (anti-scope `proposals/05-wu-0a-05.md:23`). When that command lands, it should consult these predicates rather than re-encoding the gate.
- INFO: `EventTopic` derives `Copy` (`event_topic.rs:3`), making the slice helpers cheap; safe because the enum is unit-only.

## Verdict

**LOW.** Public surface is the declared library types + parser function + gating predicates, nothing else. The WU-0A-01 inert scaffold is untouched in shape and behavior — still command-free, capability-minimal — and the WU-0A-02 settings loader is byte-identical. Migration is additive only, rollback is a clean revert, and every acceptance criterion is observable via at least one test that pins it to a fixture rather than to internal state.
