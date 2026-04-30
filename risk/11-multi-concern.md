# WU-0A-11 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-11 `PaneId` taxonomy + `PaneIdError` taxonomy + `parsePaneId` + Phase 0A live-vs-inert metadata + minimal `ShellRoot` metadata wiring, with one bilingual-style TS contract test — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-11.md`) explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Concern enumeration

Files added/modified on the branch group cleanly under one logical concern: "ship the Phase 0A `PaneId` taxonomy, parser, error taxonomy, and live-vs-inert pane metadata, with a fixture-pinned TS contract test and the minimal `ShellRoot` read that observes the live-pane gating."

| Group | Files | Why required for this single concern |
|---|---|---|
| TS contract types + parser | `src/contracts/pane-id.ts` | `PaneId` union, `PANE_IDS`, `PaneIdError` union, `PaneIdParseResult`, `parsePaneId`, `isPaneId` per ticket criteria 1-4 (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-11.md`). |
| TS shell metadata | `src/shell/pane-id.ts`, `src/shell/` (new directory) | Phase 0A live/inert pane metadata + predicates required by ticket criteria 5-6 ("non-runtimeStatus pane id renders an inert placeholder", "runtimeStatus is the only Phase 0A pane allowed to display live runtime scaffold state"). |
| TS contract test | `src/test/pane-id.test.ts` | Fixture-backed coverage of taxonomy parity, parser success, parser errors, and live/inert gating — risk-annotated per proposal test-intent tracks (`proposals/11-wu-0a-11.md:43-48`). |
| Fixtures | `product-strategy/contracts/fixtures/wu-0a-11/{pane-ids,parse-success,parse-errors,phase-0a-live-panes,inert-panes}.json` | Canonical inputs/outputs per contract canonical-fixtures section (`product-strategy/contracts/wu-0a-11-pane-id-taxonomy.md:77-85`). |
| ShellRoot metadata wiring | `src/ShellRoot.tsx` (+11 lines) | Acceptance criterion 6 ("runtimeStatus is the only Phase 0A pane allowed to display live runtime scaffold state") needs an observable shell-side read. The diff is the minimum needed: import `DEFAULT_PHASE_0A_PANE_ID` + `getPhase0aPaneMode`, set `data-pane-id` / `data-pane-mode` on the existing `<section>`. Existing `data-testid="app-root"` and heading text preserved. |
| Contract spec + proposal | `product-strategy/contracts/wu-0a-11-pane-id-taxonomy.md`, `proposals/11-wu-0a-11.md` | Phase-3 proposal and WU-owned contract document. |

## Severability check — can any group ship independently?

- **TS contract types alone** (without the shell metadata module): would not satisfy ticket criteria 5-6 (live/inert gating). The `PaneId` union has no Phase-0A-mode observable on its own.
- **TS shell metadata alone** (without `src/contracts/pane-id.ts`): does not compile — `Phase0aPaneMode`, `DEFAULT_PHASE_0A_PANE_ID`, `PHASE_0A_LIVE_PANE_IDS`, `PHASE_0A_INERT_PANE_IDS`, and `PHASE_0A_PANE_MODES` all reference `PaneId` from `../contracts/pane-id`.
- **TS contract test alone** (without contracts + shell): does not compile — imports `PANE_IDS`, `parsePaneId`, `PaneId`, `PaneIdParseResult`, `PHASE_0A_LIVE_PANE_IDS`, `getPhase0aPaneMode`, `isPhase0aLivePane`.
- **Fixtures alone**: cannot ship — they have no consumer.
- **ShellRoot edit alone** (without the shell metadata module): does not compile — `DEFAULT_PHASE_0A_PANE_ID` and `getPhase0aPaneMode` only exist after `src/shell/pane-id.ts` lands.
- **Contract spec / proposal alone**: required workflow artifacts for this WU; not a separable concern.

No group is independently shippable.

## Cross-WU contamination check

- **No Rust / Tauri changes**: `git diff HEAD -- src-tauri/ Cargo.toml Cargo.lock` is empty. `phase_0a_registers_no_value_slice_commands` (WU-0A-01) and the no-default-Tauri-capabilities invariants are byte-identical.
- **No WU-0A-02 (settings) edits**: `src-tauri/src/settings.rs`, `src-tauri/src/contracts/harness_settings.rs`, `src/contracts/harness-settings.ts`, and fixtures under `product-strategy/contracts/fixtures/wu-0a-02/` are byte-identical.
- **No WU-0A-05 (event-topic) edits**: `src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src/contracts/event-topic.ts`, and fixtures under `product-strategy/contracts/fixtures/wu-0a-05/` are byte-identical.
- **No WU-0A-09 (trace-context) edits**: `src-tauri/src/contracts/trace_context.rs`, `src-tauri/src/tracing/`, `src/contracts/trace-context.ts`, and fixtures under `product-strategy/contracts/fixtures/wu-0a-09/` are byte-identical.
- **No later-WU code**: no GraphStore migrations (`src-tauri/migrations/` still absent), no audit-log code, no provider crates added (`Cargo.toml` unchanged), no subprocess code, no `tauri::generate_handler!`, no `app.manage(...)`, no IPC commands. No file references WU-0A-03 (`LocalStorageLayout`), WU-0A-04, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-10, WU-0A-12+, WU-0A-14, or any later phase ticket.
- **No router / multi-tab work**: `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts` are byte-identical.
- **No pane-specific feature UI**: `ShellRoot.tsx` diff is purely metadata wiring (import + 2 consts + 2 attrs); no per-pane render branches, no new components, no seeded domain text.
- `proposals/` adds only `11-wu-0a-11.md`; no other proposal snuck in. `product-strategy/contracts/fixtures/` adds only `wu-0a-11/`; no cross-WU fixture set was touched.

## Observations

- The taxonomy / parser pair lives in `src/contracts/pane-id.ts` (the contract surface) and the live-vs-inert metadata lives in `src/shell/pane-id.ts` (the shell-side concern). This split mirrors WU-0A-05 (taxonomy in `src/contracts/event-topic.ts` + Phase 0A live-topic gating in the same module) and WU-0A-09 (DTO types in `src-tauri/src/contracts/` + parser/creator in `src-tauri/src/tracing/`) — same pattern, same single-concern footprint, no shared "shell" or "panes" crate invented.
- The new `src/shell/` directory is created with one file (no `index.ts` barrel), keeping the change scoped to one concern instead of seeding shell-namespace plumbing for later WUs.
- Both inert-pane and live-pane fixtures are required to fully observe the contract: `phase-0a-live-panes.json` pins the positive list, `inert-panes.json` pins the negative list, and `PHASE_0A_PANE_MODES` is `as const satisfies Record<PaneId, Phase0aPaneMode>` so the typecheck plus the two fixtures together prove the partition is total. Neither fixture is redundant, neither is scope-bleed.
- The `ShellRoot` edit is the minimum that makes the live-pane gating observable in the actual shell render — it does not introduce a router, a per-pane component table, or seeded domain data, and the existing scaffold smoke test (`src/test/scaffold.test.tsx:12-13`) keeps passing without modification.

## Verdict

**LOW.** This is a single-concern PR for WU-0A-11. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, no group serves another WU, and there is no contamination from WU-0A-01 / WU-0A-02 / WU-0A-05 / WU-0A-09 (all byte-identical) or any later WU (no GraphStore, audit log, providers, optimizers, workers, questions, recovery, budget, IPC, app-state, subprocesses, payload schemas, storage-layout work, router work, or per-pane feature UI). The single-concern handoff note is honored.
