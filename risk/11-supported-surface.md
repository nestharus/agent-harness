# WU-0A-11 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does deployment-mode + customer-cohort + adjacent-public-paths analysis match the WU-0A-11 supported-surface declaration? What is the blast radius for adjacent paths (the WU-0A-01 scaffold, WU-0A-02 settings, WU-0A-05 event-topic, WU-0A-09 trace-context)? Migration / rollback / observability story?

## Supported-surface contract (proposal `proposals/11-wu-0a-11.md:27-35`)

- Deployment mode: local Phase 0A desktop scaffold.
- Customer cohort: developer / test harness consumers only.
- Public paths: TypeScript contract importers and the React shell root.
- Adjacent paths: existing router, app shell, settings, event-topic, and trace-context contracts remain unchanged.
- Migration path: additive contract, shell metadata, fixtures, tests, and shell-root metadata wiring.
- Rollback path: remove the WU-0A-11 files and the additive `ShellRoot` metadata read.
- Observability: Vitest verifies fixture parity, parser success/error behavior, and live-vs-inert Phase 0A pane gating.

## Findings

### Public surface matches the declaration

- TypeScript contract symbols: `PANE_IDS`, `PaneId` type, `PANE_ID_ERRORS`, `PaneIdError` type, `PaneIdParseFailure`, `PaneIdParseResult`, `parsePaneId`, `isPaneId` (`src/contracts/pane-id.ts:1-66`). No default export, no module-side-effect code.
- TypeScript shell metadata symbols: `Phase0aPaneMode`, `DEFAULT_PHASE_0A_PANE_ID`, `PHASE_0A_LIVE_PANE_IDS`, `PHASE_0A_INERT_PANE_IDS`, `PHASE_0A_PANE_MODES`, `getPhase0aPaneMode`, `isPhase0aLivePane` (`src/shell/pane-id.ts:1-44`). No default export, no module-side-effect code.
- React surface: `ShellRoot` continues to export the same component name and renders the same `<main data-testid="app-root">` / `<section>` / `<h1 id="shell-title">Agent Harness</h1>` markup, with two added `data-pane-id` / `data-pane-mode` attributes (`src/ShellRoot.tsx:1-23`). No new components, no new exports.
- The exposed surface is exactly what the proposal calls "TypeScript contract importers and the React shell root."

### Adjacent paths remain unchanged

- WU-0A-01 inert scaffold:
  - `src-tauri/` is byte-identical (`git diff HEAD -- src-tauri/` is empty); `phase_0a_registers_no_value_slice_commands` and the no-default-Tauri-capabilities invariants are unaffected.
  - `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `package.json`, `bun.lock` are byte-identical (verified by `git diff HEAD -- …`).
  - `src/ShellRoot.tsx` keeps `data-testid="app-root"` and the heading text intact, so the WU-0A-01 scaffold smoke test (`src/test/scaffold.test.tsx:12-13`) still passes against this branch.
- WU-0A-02 settings loader: no edits to `src-tauri/src/settings.rs`, `src-tauri/src/contracts/harness_settings.rs`, or `src/contracts/harness-settings.ts`. The settings DTO, log-level union, error taxonomy, and loader are entirely untouched.
- WU-0A-05 event-topic taxonomy: no edits to `src-tauri/src/events/`, `src-tauri/src/contracts/event_topic.rs`, `src/contracts/event-topic.ts`, or any WU-0A-05 fixture under `product-strategy/contracts/fixtures/wu-0a-05/`.
- WU-0A-09 trace-context schema: no edits to `src-tauri/src/contracts/trace_context.rs`, `src-tauri/src/tracing/`, `src/contracts/trace-context.ts`, or any WU-0A-09 fixture under `product-strategy/contracts/fixtures/wu-0a-09/`.

### Blast radius

- Greenfield Phase 0A on the TS side: no prior runtime users of `src/contracts/pane-id` or `src/shell/pane-id`. The new public API is purely additive; nothing pre-existing depends on it.
- The new `src/shell/` directory is a fresh namespace; reordering / renaming inside it cannot affect WU-0A-01 / WU-0A-02 / WU-0A-05 / WU-0A-09 modules.
- Fixtures live under `product-strategy/contracts/fixtures/wu-0a-11/`, namespaced by WU. Cannot collide with prior WU fixture sets.
- No transitive dep changes: `package.json` and `bun.lock` are unchanged.
- No Rust impact: `src-tauri/Cargo.toml` and `src-tauri/Cargo.lock` are unchanged.
- DOM-attribute additions on `<section>` (`data-pane-id`, `data-pane-mode`) are non-semantic and do not collide with React or ARIA reserved attributes; the existing `aria-labelledby="shell-title"` is preserved.

### Migration / rollback path

- Migration: additive only. No persistent state, no schema, no on-disk artifacts produced. Any future caller can opt in by importing from `src/contracts/pane-id` or `src/shell/pane-id`; no existing caller is forced to change.
- Rollback: delete the added files (`src/contracts/pane-id.ts`, `src/shell/pane-id.ts`, `src/shell/` directory, `src/test/pane-id.test.ts`, `proposals/11-wu-0a-11.md`, `product-strategy/contracts/wu-0a-11-pane-id-taxonomy.md`, `product-strategy/contracts/fixtures/wu-0a-11/`) and revert the 11-line addition to `src/ShellRoot.tsx`. No stateful reconciliation needed — `ShellRoot` returns to its pre-WU markup, which still satisfies the WU-0A-01 scaffold smoke test.

### Observability

- TS taxonomy parity: `src/test/pane-id.test.ts:21-46` asserts `PANE_IDS` equals both the inline expected list (typed via `satisfies PaneId[]`) and the `pane-ids.json` fixture, with `// @ts-expect-error` at line 39-40 pinning that `"unknown"` is rejected by the union at compile time.
- Parser success: `pane-id.test.ts:48-54` loops `parse-success.json` (all twelve documented pane strings) through `parsePaneId`.
- Parser errors: `pane-id.test.ts:56-62` loops `parse-errors.json`, which covers `""` → `EmptyPaneId` and three `UnknownPaneId` cases — `"unknown"` (truly unknown), `"RuntimeStatus"` (case variation), `" runtimeStatus "` (whitespace) — pinning the no-normalization rule from the contract (`…taxonomy.md:32`).
- Phase 0A live/inert gating: `pane-id.test.ts:64-78` asserts `PHASE_0A_LIVE_PANE_IDS === phase-0a-live-panes.json === ["runtimeStatus"]`, then loops `inert-panes.json` and asserts each pane is in `PANE_IDS`, mode `"inertPlaceholder"`, and `isPhase0aLivePane === false`. Closes with `runtimeStatus → "liveRuntimeScaffold"` and `isPhase0aLivePane("runtimeStatus") === true`.

### Boundary observations (informational)

- INFO: `data-pane-id` / `data-pane-mode` DOM attributes (`src/ShellRoot.tsx:12-13`) give Playwright / future component tests a deterministic selector for "which Phase 0A pane is live" without exposing internal store state. Future WUs can assert against these attributes without taking a hard dep on `getPhase0aPaneMode`.
- INFO: `Phase0aPaneMode` is a TS-side label, not a contract surface; the contract speaks only about live-vs-inert behavior (`…taxonomy.md:57-75`). A future WU could rename the literals without breaking any WU-0A-11 fixture.

## Verdict

**LOW.** Public surface is the declared TS contract module, the new shell metadata module, and the shell-root metadata wiring — nothing else. WU-0A-01 / WU-0A-02 / WU-0A-05 / WU-0A-09 adjacent paths are byte-identical (`src-tauri/`, settings, event-topic, trace-context, router, App, main, configs, lockfiles all unchanged). Migration is additive only, rollback is a clean revert plus an 11-line undo on `ShellRoot.tsx`, and every acceptance criterion is observable via at least one Vitest case that pins it to a fixture rather than to internal state.
