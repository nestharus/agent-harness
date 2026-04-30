# WU-0A-14b Contract: Render With Harness React Testing Utility

## Ownership

This WU owns the frontend-only React Testing Library utility `renderWithHarness(ui, seedName)`, the TypeScript `TempHarnessHandle` DTO mirror, and the fixtures that bind WU-0A-14a temp harness seeds to frontend render behavior. It does not own Rust harness state, Tauri command registration, GraphStore migrations, provider credentials, or subprocess execution.

## DTO

`TempHarnessHandle` serializes as a JSON object with exactly these fields:

- `workspace_id`: required string.
- `database_path`: required string.
- `fixture_manifest_id`: required string.
- `app_state_ready`: required boolean.

`TempHarnessError` serializes as one of these exact strings:

- `UnknownSeed`
- `DatabaseCreateFailed`
- `AppStateInitFailed`
- `RealAgentsInvocationAttempted`
- `FixtureManifestMissing`

## Utility

```text
renderWithHarness(ui: ReactNode, seedName: string) -> RenderResult
```

Behavior:

- Resolves `seedName` from fixture-owned WU-0A-14a seed metadata.
- Parses the resolved handle through `parseTempHarnessHandle`.
- Seeds the WU-0A-07 `invokeCommand` fixture shim with the command responses documented for the selected seed.
- Renders `ui` under a fresh `QueryClientProvider` and TanStack Router context provider backed by a fresh memory router.
- Returns the unextended React Testing Library `RenderResult` surface: `container`, `baseElement`, `debug`, `rerender`, `unmount`, `asFragment`, and standard Testing Library query methods.

Seed failures throw `TempHarnessErrorFailure` before mounting. Unknown seed names throw `UnknownSeed`; the documented `missing-manifest` seed throws `FixtureManifestMissing`.

## Side-Effect Boundary

The renderer does not invoke Tauri, execute `/home/nes/.local/bin/agents`, apply migrations, read provider credentials, or start runtime producers. IPC behavior is fixture-only through the WU-0A-07 invoke shim.

## Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-14b/`:

- `canonical-seed-renders.json`: happy WU-0A-14a seeds and renderable handles.
- `render-result-shape.json`: documented `RenderResult` keys and query examples.
- `provider-mounting.json`: marker IDs used to prove QueryClient and Router providers wrap UI.
- `ipc-seeding.json`: per-seed invoke responses loaded into the fixture shim.
- `seed-failure.json`: documented seed failures that must throw before mount.
- `structural-absence.json`: no real agents, migration, provider, or subprocess behavior.

## Test Handoff

Vitest contract test: `src/test/temp-harness.test.tsx`.

Every test group carries a risk annotation mapped to the WU-0A-14b proposal test-intent track.
