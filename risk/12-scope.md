# WU-0A-12 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside WU-0A-12's boundary (TS-only `ShellRegionState` DTO, `parseShellRegionState` parser, the five `ShellRegionStateError` variants, and the canonical fixtures) without leaking into Rust contracts, Tauri commands, router state, shell stores, IPC subscriptions, GraphStore, providers, agents, workers, optimizer state, evidence, cost, or recovery code that the ticket and proposal anti-scope reserve for later WUs (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-12.md`, `proposals/12-wu-0a-12.md:24-29`)?

## Findings

### Code / test boundary matches the ticket

Ticket boundaries (`WU-0A-12.md` "Code boundary" / "Test boundary"):

- Code: `src/shell/shell-region-state.ts`, `src/contracts/shell-region-state.ts`, `src/test/shell-region-state.test.ts`.
- Test: `product-strategy/contracts/wu-0a-12-shell-region-state.md`, `src/contracts/shell-region-state.ts`, `product-strategy/contracts/fixtures/wu-0a-12/*.json`.

Files shipped on the branch (untracked working tree against `main` 943b420; no committed delta yet):

- `src/shell/shell-region-state.ts` ✓
- `src/contracts/shell-region-state.ts` ✓
- `src/test/shell-region-state.test.ts` ✓
- `product-strategy/contracts/wu-0a-12-shell-region-state.md` ✓
- `product-strategy/contracts/fixtures/wu-0a-12/canonical-states.json` ✓
- `product-strategy/contracts/fixtures/wu-0a-12/parse-errors.json` ✓
- `product-strategy/contracts/fixtures/wu-0a-12/shell-region-state-errors.json` ✓ (additional error-taxonomy fixture used to pin `SHELL_REGION_STATE_ERRORS` against the contract; declared in the contract file at `…wu-0a-12-shell-region-state.md:79`)
- `proposals/12-wu-0a-12.md` ✓ (proposal artifact, expected for the WU)

No other working-tree changes (`git ls-files -m -o --exclude-standard` is exactly the eight files above). No tracked-file diff against `main`.

### `ShellRegionState` DTO shape matches the contract

`src/shell/shell-region-state.ts:1-10`:

```ts
export interface ShellRegionState {
  workspaceId: string;
  activePane: PaneId;
  selectedNodeId?: string;
  actionNeededCount: number;
  passiveProgressCount: number;
  runtimeConnected: boolean;
}
```

Field names, types, and the optionality of `selectedNodeId` match the WU contract field block in `WU-0A-12.md` and the contract markdown at `product-strategy/contracts/wu-0a-12-shell-region-state.md:11-22`. `activePane` is typed as `PaneId` imported from `../contracts/pane-id` (the WU-0A-11 union of 12 strings — `src/contracts/pane-id.ts:1-16`), so the DTO inherits the documented pane taxonomy without redefining it.

### `parseShellRegionState` signature matches

`src/contracts/shell-region-state.ts:28`: `export function parseShellRegionState(input: unknown): ShellRegionStateParseResult`. Input is `unknown`; result is the `{ ok: true, value: ShellRegionState } | { ok: false, error: { kind: ShellRegionStateError } }` sum type (`src/contracts/shell-region-state.ts:18-26`). This matches the contract signature `parseShellRegionState(input: unknown) -> Result<ShellRegionState, ShellRegionStateError>` (`WU-0A-12.md` schema block; `…wu-0a-12-shell-region-state.md:26-50`) and the WU-0A-11 result-shape convention (`src/contracts/pane-id.ts:27-35`).

### All five `ShellRegionStateError` variants implemented and tested

- `SHELL_REGION_STATE_ERRORS` is the `as const` tuple of the five documented kinds in declaration order (`src/contracts/shell-region-state.ts:4-10`), and the `shell-region-state-errors.json` fixture matches it character-for-character (`fixtures/wu-0a-12/shell-region-state-errors.json:1-7`). The "ShellRegionStateError variants aligned with fixtures" test asserts both equality (`src/test/shell-region-state.test.ts:74-92`).
- Each variant has a dedicated parser branch:
  - `EmptyWorkspaceId` — `src/contracts/shell-region-state.ts:29-37` (also covers null/array/non-object input as workspaceId-shaped failure).
  - `UnknownPaneId` — lines 39-46 (non-string activePane and unknown PaneId both route through `parsePaneId`).
  - `NegativeActionNeededCount` — lines 48-54.
  - `NegativePassiveProgressCount` — lines 56-62.
  - `InvalidRuntimeConnected` — lines 64-66.
- Each error variant has a corresponding case in `parse-errors.json` (`fixtures/wu-0a-12/parse-errors.json:1-82`), driven by the `it.each(parseErrors)` table at `src/test/shell-region-state.test.ts:94-100`. The five acceptance-criterion error rows in `WU-0A-12.md` are all exercised.

### Pane variant coverage

`canonical-states.json` enumerates one valid `ShellRegionState` per documented `PaneId` in WU-0A-11's order (`fixtures/wu-0a-12/canonical-states.json:1-93`). `shell-region-state.test.ts:48-52` asserts `canonicalStates.map(s => s.activePane)` equals the WU-0A-11 `pane-ids.json` fixture, pinning order and completeness against the upstream taxonomy. A second `it.each(paneIds)` table at lines 54-72 explicitly parses each pane string as `activePane` and asserts an `{ ok: true, value }` round-trip — satisfying the "parser accepts every documented PaneId variant" acceptance criterion with one assertion per variant. Vitest reports 21 tests passed for this file (`bun run test`).

### Anti-scope honored

Proposal anti-scope (`proposals/12-wu-0a-12.md:24-29`) bars: Rust contract / Tauri command / IPC changes; router state, store, workspace state, IPC subscription wiring; GraphStore, provider, agent, worker, optimizer, evidence, cost, recovery work; pane UI implementation beyond consuming the existing `PaneId`. Verified:

- `git diff main -- src-tauri` empty; no untracked Rust files (`git ls-files -o --exclude-standard | grep -E '\.rs$|src-tauri'` empty). `cargo test --manifest-path src-tauri/Cargo.toml` shows the same prior-WU test inventory passing (no new tests for WU-0A-12).
- No router/store/IPC imports inside `parseShellRegionState`. The only imports in `src/contracts/shell-region-state.ts` are `parsePaneId` from the sibling contract and the `ShellRegionState` type from `../shell` — both are pure modules with no side-effecting dependencies. No `useRouter`, no Zustand store, no Tauri `invoke` / `listen` calls.
- No GraphStore / domain seed code. Grep for `ShellRegionState` across `src/` returns only the three WU-0A-12 files; nothing imports it from feature modules.
- No pane UI work: `src/ShellRoot.tsx` is unchanged.

### Potential concerns — none rising to MEDIUM

- INFO: The proposal lists "fixture round-trip, pane coverage, type-level DTO enforcement, and error behavior" as test-intent tracks. All four are exercised by `src/test/shell-region-state.test.ts` (round-trip at `:16-46`, pane coverage at `:48-72`, error-table coverage at `:74-100`, and `// @ts-expect-error` assertions at `:27-33` / `:84-87`). The "parser purity assumption" track is documented as an import-boundary review at `:102-114`. Ticket carries no acceptance criterion that fails as a result.
- INFO: `shell-region-state-errors.json` is an extra fixture beyond the two required by the ticket. The proposal lists it explicitly (`proposals/12-wu-0a-12.md:21`) and the contract file references it (`…wu-0a-12-shell-region-state.md:79`). It is a minimal restatement of the error taxonomy — additive, fixture-only, and covered by the alignment test. Not scope creep.

## Verdict

**LOW.** Implementation lands exactly the DTO, parser, error union, parse-result type, and canonical fixtures called out by the ticket and contract. Every acceptance criterion is exercised by a fixture-driven test, including one assertion per documented `PaneId` variant. Anti-scope is honored end-to-end (zero Rust / Tauri delta, no router / store / IPC wiring, no GraphStore or feature-domain code, no `ShellRoot` edits) and the only adjacent additions are the proposal artifact and the contract-aligned error-taxonomy fixture.
