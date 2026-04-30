# WU-0A-11 Contract: PaneId Taxonomy

## Ownership

This WU owns the TypeScript Phase 0A shell pane taxonomy, pane parser, parser error taxonomy, and live-vs-inert pane metadata. It does not own GraphStore, seeded domain data, pane-specific feature implementation, provider state, agents, workers, optimizer state, evidence, cost, recovery, Tauri IPC, or Rust contracts.

## PaneId

`PaneId` serializes as one of these exact strings:

- `initiativeMap`
- `currentFocus`
- `workingSetInspector`
- `configurationInspector`
- `providerPanel`
- `questionQueue`
- `workerBoard`
- `optimizerLog`
- `evidenceDrilldown`
- `costSurface`
- `recoverySurface`
- `runtimeStatus`

TypeScript must preserve this set exactly. Unknown strings are rejected by the parser.

## Parser

```text
parsePaneId(raw: string) -> PaneIdParseResult
```

Parsing is exact-match against the documented strings. No trimming, case folding, aliasing, or prefix matching is allowed.

`PaneIdParseResult` serializes as one of these shapes:

```json
{ "ok": true, "value": "runtimeStatus" }
```

```json
{ "ok": false, "error": { "kind": "UnknownPaneId", "raw": "unknown" } }
```

## Errors

`PaneIdError` serializes as one of these exact strings:

- `EmptyPaneId`
- `UnknownPaneId`

Parser behavior:

- `parsePaneId("")` returns `{ "ok": false, "error": { "kind": "EmptyPaneId", "raw": "" } }`.
- Any other undocumented string returns `{ "ok": false, "error": { "kind": "UnknownPaneId", "raw": raw } }`.
- A documented pane string returns `{ "ok": true, "value": paneId }`.

## Phase 0A Live Panes

Only `runtimeStatus` may display live Phase 0A runtime scaffold state.

The other documented panes are intentionally present as inert later-domain placeholders:

- `initiativeMap`
- `currentFocus`
- `workingSetInspector`
- `configurationInspector`
- `providerPanel`
- `questionQueue`
- `workerBoard`
- `optimizerLog`
- `evidenceDrilldown`
- `costSurface`
- `recoverySurface`

They must not be removed from the taxonomy, but Phase 0A must not treat them as live panes or claim seeded domain data for them.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-11/`:

- `pane-ids.json`: all documented pane strings in canonical order.
- `parse-success.json`: documented successful parser cases.
- `parse-errors.json`: documented parser error cases and error result shape.
- `phase-0a-live-panes.json`: pane IDs allowed to display real Phase 0A runtime scaffold state.
- `inert-panes.json`: later-domain placeholders retained for taxonomy stability.

## Test Handoff

- TypeScript contract test: `src/test/pane-id.test.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
