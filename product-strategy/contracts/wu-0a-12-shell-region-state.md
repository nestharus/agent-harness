# WU-0A-12 Contract: ShellRegionState View Model

## Ownership

This WU owns the TypeScript `ShellRegionState` DTO, parser result shape, parser error taxonomy, and canonical fixtures. It does not own Rust contracts, Tauri commands, router state, shell stores, IPC clients, GraphStore, providers, agents, workers, optimizer state, evidence, cost, or recovery behavior.

## ShellRegionState

`ShellRegionState` serializes as a TypeScript camelCase object:

```ts
interface ShellRegionState {
  workspaceId: string;
  activePane: PaneId;
  selectedNodeId?: string;
  actionNeededCount: number;
  passiveProgressCount: number;
  runtimeConnected: boolean;
}
```

`activePane` must be one of the documented WU-0A-11 `PaneId` strings. `selectedNodeId` is optional and omitted when no shell node is selected. Both counters are non-negative numbers.

## Parser

```text
parseShellRegionState(input: unknown) -> ShellRegionStateParseResult
```

Parsing is exact-match for `activePane` through `parsePaneId`. No trimming, case folding, aliasing, or prefix matching is allowed for `workspaceId`, `activePane`, or `selectedNodeId`.

`ShellRegionStateParseResult` serializes as one of these shapes:

```json
{
  "ok": true,
  "value": {
    "workspaceId": "workspace-alpha",
    "activePane": "runtimeStatus",
    "selectedNodeId": "node-runtime",
    "actionNeededCount": 1,
    "passiveProgressCount": 2,
    "runtimeConnected": true
  }
}
```

```json
{ "ok": false, "error": { "kind": "UnknownPaneId" } }
```

## Errors

`ShellRegionStateError` serializes as one of these exact strings:

- `EmptyWorkspaceId`
- `UnknownPaneId`
- `NegativeActionNeededCount`
- `NegativePassiveProgressCount`
- `InvalidRuntimeConnected`

Parser behavior:

- Empty or non-string `workspaceId` returns `{ "ok": false, "error": { "kind": "EmptyWorkspaceId" } }`.
- Unknown or non-string `activePane` returns `{ "ok": false, "error": { "kind": "UnknownPaneId" } }`.
- Negative or non-number `actionNeededCount` returns `{ "ok": false, "error": { "kind": "NegativeActionNeededCount" } }`.
- Negative or non-number `passiveProgressCount` returns `{ "ok": false, "error": { "kind": "NegativePassiveProgressCount" } }`.
- Non-boolean `runtimeConnected` returns `{ "ok": false, "error": { "kind": "InvalidRuntimeConnected" } }`.
- A valid object returns `{ "ok": true, "value": ShellRegionState }`.

The parser is pure: it must not import or mutate route state, workspace state, stores, or IPC subscriptions.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-12/`:

- `canonical-states.json`: valid shell region states spanning every documented `PaneId` variant.
- `parse-errors.json`: one parser error case for every documented `ShellRegionStateError` variant.
- `shell-region-state-errors.json`: canonical error taxonomy.

## Test Handoff

- TypeScript contract test: `src/test/shell-region-state.test.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
