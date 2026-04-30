# WU-0A-13 Risk Gate: Scope-Creep

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The implementation is faithful to the WU contract field and ticket scope. Both routes are registered, both render exactly one shell instance (asserted by `findAllByTestId(...).toHaveLength(1)` in four places), every documented `PaneId` is covered by `it.each`, the unknown-pane fallback is tested with full `ShellRegionState` equality, and there are no Phase 0B/0C/1+ behaviors. The only off-boundary edits (`src/router.tsx`, `src/styles.css`, `src/test/setup.ts`) are the minimum wiring required to register the routes, style the new component, and polyfill `window.scrollTo` for jsdom (TanStack Router calls it during navigation). No nit-level findings rise above LOW.

## Verification

### Route registration
`src/router.tsx:1-37` registers `createWorkspaceRoute` (path `"/workspace/$workspaceId"`) and `createWorkspaceNodeRoute` (path `"/workspace/$workspaceId/node/$nodeId"`) under the root route. The root component is now `RootOutlet` (an `<Outlet />` wrapper); the `/` index route still renders `ShellRoot`. Both workspace routes resolve to the single component `WorkspaceShell` (`src/routes/workspace.$workspaceId.tsx:11`, `src/routes/workspace.$workspaceId.node.$nodeId.tsx:11`).

### Exactly-one-shell invariant
`src/test/workspace-route-shell.test.tsx` asserts `findAllByTestId("workspace-shell").toHaveLength(1)` at lines 73, 90, 104, and 150. The Playwright spec mirrors this with `getByTestId("workspace-shell").toHaveCount(1)` at `tests-e2e/workspace-route-shell.spec.ts:12,23,33`.

### Param copy without GraphStore touch
`deriveWorkspaceShellRegionState` (`src/contracts/workspace-route-shell.ts:20-39`) copies `workspaceId` and optional `nodeId` straight into `ShellRegionState`, with no fetch / IPC / store call. The test asserts the full `ShellRegionState` equals the fixture (`expect(await readShellState()).toEqual(fixture!.expectedState)`) at lines 77, 94, and 124.

### Every PaneId covered
The `it.each(paneIds)` block (line 97) iterates the WU-0A-11 fixture `pane-ids.json` (12 entries). Each iteration renders `?pane=<id>` and asserts `data-active-pane`, `findByTestId("active-pane").toHaveTextContent(paneId)`, and `toMatchObject({ workspaceId, activePane: paneId })`. The `it.each` runs as 12 separate Vitest cases and 12 separate Playwright cases (confirmed in the e2e run output).

### Unknown-pane fallback with no workspace mutation
`src/test/workspace-route-shell.test.tsx:113-127` renders `?pane=unknown_value`, asserts the full `ShellRegionState` equals the fixture (which keeps `workspaceId` unchanged and `activePane === "runtimeStatus"`), and re-asserts the rendered `workspace-id` and `active-pane` text content. The "no mutation" claim is enforced by the full-state equality, not a partial match. The independent side-effect-absence test (line 144) extends this by asserting `vi.fn()` mocks for `@tauri-apps/api/core` and `node:fs/promises` were never called.

### Acceptance criteria mapping

| AC | Source line(s) | Verification |
| --- | --- | --- |
| AC1 (workspace route → 1 shell) | `src/test/workspace-route-shell.test.tsx:63-78` | shell count + workspaceId text + state equality |
| AC2 (node route → 1 shell + selectedNodeId) | `src/test/workspace-route-shell.test.tsx:80-95` | shell count + selected-node-id text + state equality |
| AC3 (workspaceId copy, no GraphStore) | `src/test/workspace-route-shell.test.tsx:63-78,144-156` | param copy + invoke spy not called |
| AC4 (nodeId copy, no GraphStore) | `src/test/workspace-route-shell.test.tsx:80-95,144-156` | selectedNodeId copy + invoke spy not called |
| AC5 (each PaneId → activePane) | `src/test/workspace-route-shell.test.tsx:97-111` + `tests-e2e/workspace-route-shell.spec.ts:17-26` | `it.each(paneIds)` + Playwright per-pane cases |
| AC6 (unknown pane → runtimeStatus, no mutation) | `src/test/workspace-route-shell.test.tsx:113-127` + `tests-e2e/workspace-route-shell.spec.ts:28-35` | full-state equality + e2e text check |
| AC7 (no agents/provider/GraphStore migrations) | `src/test/workspace-route-shell.test.tsx:144-156` + `src-tauri/src/lib.rs:15-30` (unchanged) | invoke + readFile spies + scaffold list pinned |
| AC8 (inert placeholders + live runtime only) | `src/shell/workspace-shell.tsx:88-106` (renders `data-pane-mode` per `getPhase0aPaneMode`) | rendered, not asserted in vitest/Playwright |

### Phase boundary
No Phase 0B/0C/1+ behavior. `WorkspaceShell` renders only the static placeholder chrome and the WU-0A-12 view-state values; there is no graph fetch, no provider call, no question/worker/optimizer/evidence wiring, no migration trigger, no agent spawn, no live IPC. `runtimeConnected` is hard-coded to `false` in the derivation helper (`src/contracts/workspace-route-shell.ts:30`), which the contract documents as the Phase 0A placeholder.

### Off-boundary edits (justification)
The ticket's code boundary is `src/routes/**/*`, `src/shell/workspace-shell.tsx`, `src/contracts/workspace-route-shell.ts`, `src/test/workspace-route-shell.test.tsx`, `e2e/workspace-route-shell.spec.ts`. The implementer also touched:

- `src/router.tsx` — adds the two workspace routes to the route tree and swaps the root component to an `<Outlet />` wrapper so `/` still renders `ShellRoot` while the workspace child routes mount their own component. This is the minimum mechanical wiring for the routes to exist.
- `src/styles.css` — additive CSS for the new `.workspace-shell*` selectors. No edits to existing rules.
- `src/test/setup.ts` — polyfills `window.scrollTo` for jsdom; TanStack Router invokes it on navigation. Without this, every workspace-route test would throw.

These are all necessary, additive, and minimal.

## Findings

### WU-0A-13-SCOPE-F01 — AC8 inert/live pane mode is rendered but not asserted (LOW)

**Where:** `src/shell/workspace-shell.tsx:88-106`, `src/test/workspace-route-shell.test.tsx`, `tests-e2e/workspace-route-shell.spec.ts`

**Detail:** The shell emits `data-pane-mode={"inertPlaceholder" | "liveRuntimeScaffold"}` per pane and the body text reads `"Phase 0A runtime state"` vs `"Inert placeholder"`. AC8 ("the single-tab shell displays inert placeholders for later domain panes and live runtime status only for Phase 0A runtime state") is implicitly satisfied by the render path, but neither the Vitest nor the Playwright suite explicitly queries `data-pane-mode` or the placeholder copy. A future regression that flips a domain pane to "liveRuntimeScaffold" would not be caught by the existing tests.

**Impact:** Test gap, not an implementation defect. The data attribute is present and correctly derived from the WU-0A-12 / WU-0A-11 helpers, so this is a low-risk gap.

**Recommendation:** Optional — add one assertion that `getAllByText("Inert placeholder")` equals 11 and `getAllByText("Phase 0A runtime state")` equals 1 for the default route fixture. Not required to land WU-0A-13.

## Verdict

**LOW.** All 8 acceptance criteria are satisfied; AC1–AC7 are exercised with strong assertions. AC8 is implemented and visible in the DOM but not directly asserted — a minor test-coverage nit. Off-boundary edits are minimum mechanical wiring. No scope creep.
