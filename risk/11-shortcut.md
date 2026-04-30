# WU-0A-11 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-11's purpose (a fixture-pinned `PaneId` taxonomy, exact-match parser, and live-vs-inert Phase 0A metadata)? Are TODOs, stubbed branches, weak assertions, or deferred behaviors hiding incomplete work?

## Shortcuts examined

### S1 — `parsePaneId` is a hand-rolled `if`/`includes` rather than a Set lookup or codegen

`src/contracts/pane-id.ts:37-66`. Empty → `EmptyPaneId`; otherwise `isPaneId` (an `Array.includes` over `PANE_IDS as readonly string[]`) decides; otherwise `UnknownPaneId`. Justified: the contract requires *exact* matching with no normalization (`product-strategy/contracts/wu-0a-11-pane-id-taxonomy.md:32`); a Set or generated-from-spec parser would behave identically here, and `Array.includes` over 12 short strings is the simplest implementation that satisfies the spec without introducing a build-time codegen step. The empty-string check is positioned before `isPaneId` so an empty raw cannot accidentally match `""` if a future fixture mistake leaks one in. Loop-pinned to `parse-success.json` and `parse-errors.json` (`src/test/pane-id.test.ts:48-62`). **Justified.**

### S2 — `isPaneId` casts via `(PANE_IDS as readonly string[]).includes`

`src/contracts/pane-id.ts:64-66`. `PANE_IDS` is a `readonly` tuple of literal types, so `Array.prototype.includes` rejects a plain `string` argument under strict TS — the cast widens the array element type to `string` purely so the runtime check can run. The return type narrows back to `value is PaneId`, so callers get exact typing. Pattern matches `src/contracts/event-topic.ts` from WU-0A-05. **Justified.**

### S3 — Phase 0A live/inert metadata is a flat `Record<PaneId, Phase0aPaneMode>` rather than a tagged enum or per-pane component table

`src/shell/pane-id.ts:23-44`. `PHASE_0A_PANE_MODES` is the single source of truth; `PHASE_0A_LIVE_PANE_IDS` and `PHASE_0A_INERT_PANE_IDS` are the two derived lists; `getPhase0aPaneMode` / `isPhase0aLivePane` are the public predicates. `as const satisfies Record<PaneId, Phase0aPaneMode>` enforces exhaustiveness — adding a `PaneId` variant without updating the record fails typecheck. Tests pin all twelve panes' modes against the fixture (`src/test/pane-id.test.ts:64-78`). A richer "pane → component" table would land per-pane render branches, which is exactly what the proposal anti-scope (`proposals/11-wu-0a-11.md:25`) bars. **Justified.**

### S4 — `ShellRoot` hard-codes `DEFAULT_PHASE_0A_PANE_ID` instead of reading the active pane from a router or store

`src/ShellRoot.tsx:1-23`. Phase 0A is single-tab and has no router state for active panes — `src/main.tsx`, `src/router.tsx`, and `src/App.tsx` are unchanged. Picking the constant default rather than wiring a route param keeps the ShellRoot edit at +11 lines and avoids inventing navigation infra the WU explicitly anti-scopes (`proposals/11-wu-0a-11.md:25`). The DOM-level `data-pane-id` / `data-pane-mode` attributes are observable for any future Playwright check that wants to assert "Phase 0A only renders the live runtime-status pane." **Justified.**

### S5 — Acceptance criterion "non-runtimeStatus panes render an inert placeholder" is satisfied by metadata + non-render rather than per-pane component stubs

`src/ShellRoot.tsx:1-23` renders only the runtime-status panel; the other eleven panes are not mounted at all. Justified by the contract section (`…taxonomy.md:60-75`: "intentionally present as inert later-domain placeholders … must not be removed from the taxonomy, but Phase 0A must not treat them as live panes or claim seeded domain data") plus the proposal's interpretation (`proposals/11-wu-0a-11.md:11`: "Phase 0A must not seed or claim real domain data for initiative maps, focus state, working sets, …"). Mounting empty per-pane components would either render literally nothing (worse observability than the metadata table) or risk slipping seeded text/icons that pretend to be domain data. The metadata table — typecheck-exhaustive on `PaneId` and fixture-pinned at `inert-panes.json` — is the strictly stronger guarantee. **Justified.**

### S6 — `Phase0aPaneMode` literals doubled as `data-pane-mode` DOM values

`src/shell/pane-id.ts:3, 23-44` and `src/ShellRoot.tsx:13`. The `"inertPlaceholder"` / `"liveRuntimeScaffold"` strings are not named in the contract, so they are not a wire-form surface. Doubling them as DOM attribute values keeps the data path one-step (no string mapping at the render site) and gives Playwright a deterministic selector — but does not commit the team to those exact labels for any cross-WU consumer. **Justified.**

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented`, `todo`, `.skip(`, `.todo(`, or `@ts-ignore` markers in `src/contracts/pane-id.ts`, `src/shell/pane-id.ts`, `src/test/pane-id.test.ts`, or `src/ShellRoot.tsx` (verified by Grep across `src/`).
- No commented-out branches in `parsePaneId`; the structure is empty → match → fallback. The fallback is the documented `UnknownPaneId` arm, not a stub.
- All twelve `PaneId` variants are exercised by `parse-success.json` and looped through `parsePaneId` (`src/test/pane-id.test.ts:48-54` × `fixtures/wu-0a-11/parse-success.json:1-86`).
- Both error variants are reachable from fixtures: `""` → `EmptyPaneId`; `"unknown"`, `"RuntimeStatus"` (case), `" runtimeStatus "` (whitespace) → `UnknownPaneId` (`fixtures/wu-0a-11/parse-errors.json:1-42`), looped at `pane-id.test.ts:56-62`. The case- and whitespace-variant fixtures specifically pin the no-normalization rule.
- The `// @ts-expect-error "unknown" is outside the documented PaneId union.` line (`src/test/pane-id.test.ts:39-40`) is a compile-time assertion that the union actually rejects strings outside the canonical set — not a silenced lint.
- `PHASE_0A_PANE_MODES` is `as const satisfies Record<PaneId, Phase0aPaneMode>` (`src/shell/pane-id.ts:36`), so omitting a pane fails typecheck rather than silently defaulting to `inertPlaceholder`. The test loops `inert-panes.json` and asserts each mode is `"inertPlaceholder"` plus `runtimeStatus → "liveRuntimeScaffold"` (`pane-id.test.ts:64-78`), pinning the fixture to the metadata.
- The scaffold smoke test (`src/test/scaffold.test.tsx:12-13`) still asserts `data-testid="app-root"` and `Agent Harness` heading visibility against the modified `ShellRoot`, so the WU-0A-01 invariant is observed unchanged.

## Verdict

**LOW.** Every shortcut is either documented in the contract (exact-match parsing, no normalization, single live Phase 0A pane), justified by the proposal anti-scope (no router / per-pane components / seeded domain data), or compensated by a typecheck-exhaustive table plus a negative fixture (`""`, `"unknown"`, `"RuntimeStatus"`, `" runtimeStatus "`). No stubs, no skipped tests, no TODOs; both error variants and all twelve pane variants are reachable, looped, and pinned to fixtures rather than internal state.
