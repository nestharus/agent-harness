# WU-0A-13 Contract: Workspace Route Shell

## Ownership

This WU owns the frontend route invariant for `/workspace/:workspaceId` and `/workspace/:workspaceId/node/:nodeId`, the pure route-to-shell-state derivation helper, the `WorkspaceShell` React component, and the route fixtures under `product-strategy/contracts/fixtures/wu-0a-13/`.

It does not own Rust commands, GraphStore, provider config, sqlx migrations, agent subprocesses, worker execution, optimizer state, evidence storage, cost tracking, or recovery behavior.

## Routes

The existing code-based TanStack Router registers:

```text
/workspace/$workspaceId
/workspace/$workspaceId/node/$nodeId
```

These are the TanStack equivalents of the user-facing paths:

```text
/workspace/:workspaceId
/workspace/:workspaceId/node/:nodeId
```

Both routes render exactly one `WorkspaceShell` root.

## Route Search

The only route-search field this WU consumes is `pane`.

```ts
interface WorkspaceRouteSearch {
  pane?: string;
}
```

The shell parses `pane` with `parsePaneId`. Missing, empty, non-string, or unknown values fall back to `runtimeStatus`.

## ShellRegionState Derivation

```text
deriveWorkspaceShellRegionState(input) -> ShellRegionState
```

Input:

```ts
interface WorkspaceShellRouteInput {
  workspaceId: string;
  nodeId?: string;
  pane?: unknown;
}
```

Output:

```ts
ShellRegionState
```

Rules:

- `workspaceId` is copied exactly from the route param.
- `nodeId`, when present, is copied exactly to `selectedNodeId`.
- `activePane` is the parsed `pane` query value, or `runtimeStatus` fallback.
- `actionNeededCount` is `0`.
- `passiveProgressCount` is `0`.
- `runtimeConnected` is `false`.

The derived object must pass WU-0A-12 `parseShellRegionState`.

## Runtime Status

The Phase 0A workspace shell does not invoke `ping_runtime`. The Rust scaffold currently registers only `subscribe_workspace_events`, and this WU must not add a command. Runtime status displays the Phase 0A placeholder state: `runtimeConnected === false`.

## Side Effects

Rendering either route must not:

- Spawn `/home/nes/.local/bin/agents`.
- Read provider config files.
- Invoke GraphStore queries or migrations.
- Apply sqlx migrations.
- Invoke unregistered runtime commands.

The frontend assertion point is no Tauri IPC call on route render and no provider config file read in the component test fixture.

## Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-13/`:

- `route-fixtures.json`: documented route and query cases, including every `PaneId` and unknown-pane fallback.
- `shell-region-state-round-trips.json`: expected `ShellRegionState` values derived from route fixtures.
- `side-effect-absence.json`: documented side-effect absence assertions for route render.

## Test Handoff

- Component test: `src/test/workspace-route-shell.test.tsx`.
- End-to-end test: `tests-e2e/workspace-route-shell.spec.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
