# WU-0A-13 Risk Gate: Shortcut / Placeholder

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The implementation does real parsing — it routes both `pane` and the assembled `ShellRegionState` through their canonical WU-0A-11 / WU-0A-12 parsers rather than blanket-casting strings. The unknown-pane fallback uses `parsePaneId(...).ok` rather than a string-equality shortcut. The "runtime status" panel does not call any Tauri command (consistent with `phase_0a_scaffold_commands()` containing only `subscribe_workspace_events`); it displays the `runtimeConnected: false` placeholder honestly and the contract documents the placeholder explicitly. Errors from `parseShellRegionState` throw rather than being swallowed. There are no TODO / FIXME / unimplemented stubs in shipped code, and the test suite asserts each documented invariant rather than merely rendering and probing for presence.

## Verification

### Real parsing in route-state derivation
`src/contracts/workspace-route-shell.ts:41-48`:
```ts
function deriveActivePane(rawPane: unknown): PaneId {
  if (typeof rawPane !== "string") {
    return DEFAULT_PHASE_0A_PANE_ID;
  }
  const parsed = parsePaneId(rawPane);
  return parsed.ok ? parsed.value : DEFAULT_PHASE_0A_PANE_ID;
}
```

This delegates to WU-0A-11's `parsePaneId` (`src/contracts/pane-id.ts:37-62`). It is not a `string` cast and not a substring match. Empty, non-string, and unknown values all fall back to `DEFAULT_PHASE_0A_PANE_ID` (`"runtimeStatus"`, `src/shell/pane-id.ts:5`).

### Real validation of the assembled state
`src/contracts/workspace-route-shell.ts:33-38`:
```ts
const parsed = parseShellRegionState(state);
if (!parsed.ok) {
  throw new Error(`Invalid workspace shell route state: ${parsed.error.kind}`);
}
return parsed.value;
```

This sends the assembled state through the WU-0A-12 parser before returning it. Errors are not silently swallowed — they throw with the kind tag, which is the strongest signal possible at this layer (a `parseShellRegionState` failure in this code path would indicate a regression in the WU-0A-12 contract or a code-level bug, not user input).

### Test cross-check on the parser path
`src/test/workspace-route-shell.test.tsx:36-51` (`readShellState`) re-parses the rendered `<script>` JSON through `parseShellRegionState` and asserts `{ ok: true, value: rawState }`. So every fixture-equality assertion implicitly proves the rendered state is valid against WU-0A-12. The `it("round-trips ...")` block (line 129) also runs each `roundTripStates` entry through `parseShellRegionState`.

### Honest runtime status
The `WorkspaceShell` "Runtime status" section (`src/shell/workspace-shell.tsx:76-86`) reads only `shellState.runtimeConnected` (always `false`), `actionNeededCount` (`0`), and `passiveProgressCount` (`0`). It calls **no** IPC. The contract is explicit: `product-strategy/contracts/wu-0a-13-workspace-route-shell.md:72-74` says "The Phase 0A workspace shell does not invoke `ping_runtime`. The Rust scaffold currently registers only `subscribe_workspace_events`, and this WU must not add a command. Runtime status displays the Phase 0A placeholder state: `runtimeConnected === false`." The proposal repeats this in `proposals/13-wu-0a-13.md:20`. No fixture-stub or mocked invoke is required because the production code path does not call `invokeCommand`. The side-effect-absence test (`src/test/workspace-route-shell.test.tsx:144`) hoists `vi.fn()` mocks for `@tauri-apps/api/core` and `node:fs/promises` and asserts neither was ever called during render — this is the strongest possible "we made no real call" evidence.

### Inert placeholders for non-runtime panes
`src/shell/workspace-shell.tsx:88-106` renders an `<article>` for every `PaneId` with:
- `data-pane-mode={getPhase0aPaneMode(paneId)}` — pulls from the WU-0A-11 / shell `PHASE_0A_PANE_MODES` table.
- Body text `"Phase 0A runtime state"` for the live `runtimeStatus` pane and `"Inert placeholder"` for the other 11 panes.
- The pane label from a complete `paneLabels: Record<PaneId, string>` map (line 15-28), which is `as const satisfies Record<PaneId, string>`-equivalent in spirit (TypeScript will reject any missing key).

These are concrete, labeled, semantic articles — not blank `<div />` stubs — and they make the Phase 0A vs later-Phase distinction visible in the DOM (`data-pane-mode`).

### No silent error swallowing
The only error path is the `parseShellRegionState` throw at `src/contracts/workspace-route-shell.ts:35`. The `WorkspaceShell` component (`src/shell/workspace-shell.tsx:34-36`) also throws if `params.workspaceId` is missing — this guards against route-shape drift rather than swallowing it. No `try/catch (_e)` blocks. No silent defaults that would hide a malformed `ShellRegionState`.

### No TODOs / FIXMEs / unimplemented stubs

```
Grep "TODO|FIXME|XXX|unimplemented|placeholder" against src/contracts/workspace-route-shell.ts,
src/shell/workspace-shell.tsx, src/routes/*.tsx, src/test/workspace-route-shell.test.tsx,
tests-e2e/workspace-route-shell.spec.ts → no shipped-code matches.
```

(The string `"placeholder"` does appear, but only as user-facing copy `"Inert placeholder"` and as a documented Phase 0A semantic in the contract — that is the intended behavior, not a stub.)

### Tests assert invariants, not just presence
Each test does more than render-and-find:

- The single-shell tests assert exact count via `findAllByTestId(...).toHaveLength(1)` (not `findByTestId`, which would silently pass with two matches in some libraries) — lines 73, 90, 104, 150.
- The single-shell tests then assert text content of the workspace-id / selected-node-id / active-pane test IDs, **plus** full state equality via `readShellState()` (which itself re-validates through WU-0A-12).
- The `it.each(paneIds)` block (line 97) iterates every WU-0A-11 fixture entry and asserts both DOM text content **and** `toMatchObject({ workspaceId, activePane })`.
- The unknown-pane test asserts full-state equality against the fixture, which fixes `workspaceId === "workspace-alpha"`, `activePane === "runtimeStatus"`, `actionNeededCount === 0`, `passiveProgressCount === 0`, `runtimeConnected === false`. This is the operational definition of "no workspace mutation" at this boundary.
- The side-effect-absence test mocks `@tauri-apps/api/core` and `node:fs/promises` via `vi.hoisted` and asserts both spies were never called — not just `toHaveBeenCalledTimes(0)`-with-no-mock, which would pass vacuously.

## Findings

None at MEDIUM or HIGH. One low-confidence nit:

### WU-0A-13-SHORTCUT-F01 — `WorkspaceRouteParams` cast (LOW / nit)

**Where:** `src/shell/workspace-shell.tsx:31-32`

**Detail:**
```ts
const params = useParams({ strict: false }) as WorkspaceRouteParams;
const search = useSearch({ strict: false }) as WorkspaceRouteSearch;
```
Both lines use a `as` cast. The runtime guard at line 34 (`if (typeof params.workspaceId !== "string" || params.workspaceId.length === 0) throw ...`) plus `validateSearch: parseWorkspaceRouteSearch` registered on each route ensure the cast is safe (search is parsed at the route layer, params come from the path matcher). This is not a parsing shortcut in the same class as the unknown-pane risk — TanStack Router's typed helpers do require some narrowing here when using `strict: false`. Documented for completeness only.

**Recommendation:** none.

## Verdict

**LOW.** No shortcuts, no swallowed errors, no fake IPC, no TODOs. Tests carry real assertions on the documented invariants and re-validate through the canonical parser at every step. The runtime-status display is honest and the contract documents it explicitly.
