# WU-0A-11 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside WU-0A-11's boundary (the TS `PaneId` union + `PANE_IDS`, the `PaneIdError` union, `parsePaneId(raw)`, and the Phase 0A live-vs-inert pane metadata) or does it leak into GraphStore, seeded domain data, provider/agent/worker/optimizer/recovery code, multi-tab navigation, route expansion, pane-specific feature UI, or Rust/Tauri changes that the proposal anti-scope reserves for later WUs (`proposals/11-wu-0a-11.md:21-25`)?

## Findings

### In-scope, confirmed

- `PANE_IDS` is the canonical 12-string `as const` tuple in the documented order, and `PaneId = (typeof PANE_IDS)[number]` makes the union the single source of truth (`src/contracts/pane-id.ts:1-16`). Strings match the contract list character-for-character (`product-strategy/contracts/wu-0a-11-pane-id-taxonomy.md:9-22`) and the `pane-ids.json` fixture (`product-strategy/contracts/fixtures/wu-0a-11/pane-ids.json:1-15`).
- `PANE_ID_ERRORS` exposes exactly `["EmptyPaneId", "UnknownPaneId"]` and `PaneIdError = (typeof PANE_ID_ERRORS)[number]` (`src/contracts/pane-id.ts:18-20`), matching the contract error taxonomy (`…taxonomy.md:46-49`).
- `PaneIdParseResult` is the documented sum type with `{ ok: true, value }` and `{ ok: false, error: { kind, raw } }` shapes (`src/contracts/pane-id.ts:22-35`), matching the contract result examples (`…taxonomy.md:34-43`).
- `parsePaneId(raw)` is exact-match: empty → `EmptyPaneId`, documented string → `{ ok: true }`, anything else → `UnknownPaneId`. No trim, case fold, alias, or prefix logic (`src/contracts/pane-id.ts:37-62`). The negative fixtures `"RuntimeStatus"` (case variation) and `" runtimeStatus "` (whitespace) round-trip to `UnknownPaneId` (`fixtures/wu-0a-11/parse-errors.json:22-41`), pinning the no-normalization rule from the contract (`…taxonomy.md:32`).
- Phase 0A pane mode metadata lives in `src/shell/pane-id.ts:1-44`: `PHASE_0A_LIVE_PANE_IDS` is `["runtimeStatus"]`, `PHASE_0A_INERT_PANE_IDS` is the other eleven, `PHASE_0A_PANE_MODES` records each pane's mode, and `getPhase0aPaneMode` / `isPhase0aLivePane` are the public predicates. Matches the contract live-vs-inert section (`…taxonomy.md:57-75`) and the `phase-0a-live-panes.json` / `inert-panes.json` fixtures.
- `DEFAULT_PHASE_0A_PANE_ID = "runtimeStatus"` (`src/shell/pane-id.ts:5`) is the single live Phase 0A pane and is used by `ShellRoot` to set `data-pane-id` / `data-pane-mode` attributes on the existing scaffold panel (`src/ShellRoot.tsx:1-23`). Proposal explicitly authorizes this read (`proposals/11-wu-0a-11.md:11`, "ShellRoot may read this metadata").

### Anti-scope honored

The proposal anti-scope (`proposals/11-wu-0a-11.md:23-25`) bars: GraphStore / seeded graph data, provider probes, question routing, worker launch, optimizer queue, evidence browser, cost ledger, recovery implementation, Rust contract / Tauri command changes, multi-tab navigation, route expansion, and pane-specific feature UI. Verified:

- Zero Rust / Tauri delta: `git diff HEAD -- src-tauri/ Cargo.toml Cargo.lock package.json bun.lock` is empty. `src-tauri/src/lib.rs` is byte-identical and `phase_0a_registers_no_value_slice_commands` (the WU-0A-01 invariant) still applies.
- No GraphStore / seeded domain code: no module under `src/` constructs initiative-map nodes, focus state, working-set entries, configuration values, provider records, queued questions, worker state, optimizer events, evidence rows, cost entries, or recovery state. `Grep PaneId` across `src/` finds only the four WU-0A-11 files (`pane-id.ts`, `shell/pane-id.ts`, `ShellRoot.tsx`, `pane-id.test.ts`).
- No router / multi-tab edits: `git diff HEAD -- src/main.tsx src/App.tsx src/router.tsx src/styles.css index.html vite.config.ts vitest.config.ts playwright.config.ts` is empty.
- No pane-specific feature UI: `ShellRoot` still renders the same `<main>` / `<section>` / heading / kicker / copy as before; the diff is +11 lines and adds only `data-pane-id` / `data-pane-mode` plus the imports / consts that compute them (`git diff src/ShellRoot.tsx`). No new components, no per-pane render branches.

### Adjacent-path additions (in-scope)

- `src/shell/` is a new directory with one file (`pane-id.ts`). No `index.ts` re-export barrel, no shell-namespace seed beyond the WU's own metadata module.
- `src/ShellRoot.tsx` gains a 1-line import and a 6-line metadata read + 2 attribute additions on the existing `<section>`. The existing `data-testid="app-root"` and `Agent Harness` heading remain unchanged so the WU-0A-01 scaffold smoke test (`src/test/scaffold.test.tsx:12-13`) still passes against this branch.

### Potential concerns — none rising to MEDIUM

- INFO: `Phase0aPaneMode` (`src/shell/pane-id.ts:3`) introduces two literal strings (`"inertPlaceholder"`, `"liveRuntimeScaffold"`). The contract describes the concept abstractly (`…taxonomy.md:57-75`) but does not pin these exact labels. They are a local implementation detail — also surfaced as `data-pane-mode` DOM attribute values — and not a contract surface; future renaming would not break any WU-0A-11 fixture. Acceptable.
- INFO: `isPaneId` (`src/contracts/pane-id.ts:64-66`) is exported alongside `parsePaneId`. Convenience type guard for callers that already hold a `string` — does not expand the contract surface.

## Verdict

**LOW.** Implementation lands exactly the union, error union, parser, parse-result type, and Phase 0A live/inert metadata that the contract and ticket call out. Anti-scope is honored end-to-end (no GraphStore, no seeded domain data, no Rust / Tauri edits, no router or multi-tab work, no pane-specific feature UI), and the only adjacent edit is the 11-line additive metadata read in `ShellRoot` that the proposal explicitly authorizes.
