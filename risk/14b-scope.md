# WU-0A-14b Risk Gate: Scope-Creep

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The implementation faithfully realizes the WU-0A-14b contract field. `renderWithHarness(ui, seedName)` returns the standard React Testing Library `RenderResult` (all six explicit keys plus the bound query methods), wraps the supplied UI in fresh `QueryClientProvider` and TanStack `RouterContextProvider` (memory history) instances per call, seeds `invokeCommand` responses through a small fixture registry on the WU-0A-07 default shim, and throws `TempHarnessErrorFailure` before any render for the documented `unknown` and `missing-manifest` failure seeds. All five acceptance criteria are exercised by `src/test/temp-harness.test.tsx`. The `src/ipc/invoke-command.ts` modification crosses the ticket's stated code boundary but is bounded (32 line additions), inactive unless explicitly seeded, and preserves every WU-0A-07 acceptance criterion (the WU-0A-07 contract test continues to pass — 10/10 in `bun run test`). No Phase 0B/0C/1+ behavior leaked in.

## Verification

### `RenderResult` shape

`src/test/render-with-harness.tsx:46-101` calls `render(createTree())` and returns `{ ...result, rerender, unmount }`, preserving every standard RTL property. The component test (`src/test/temp-harness.test.tsx:130-148`) iterates `renderResultShape.required_keys` (`container`, `baseElement`, `debug`, `rerender`, `unmount`, `asFragment`) and asserts each property is present, then explicitly checks `getByTestId("rendered-child")` and `queryByText("mounted")` to prove the bound queries are exposed. The WU contract field's seventh "key" — `getBy*/queryBy*/findBy*` — is structurally a method family, not a single key; it is verified through the documented `query_examples` fixture (`getByTestId`, `queryByText`, `findByRole`).

### Provider mounting

`render-with-harness.tsx:71-77` mounts UI inside `<QueryClientProvider><RouterContextProvider router={router}>{ui}</RouterContextProvider></QueryClientProvider>`. The `ProviderProbe` test component (`temp-harness.test.tsx:34-48`) reads `useQueryClient()` and `useRouter()` from inside the supplied UI; the assertion at `temp-harness.test.tsx:150-161` confirms both probes resolve to `combined_ready_text === "providers-ready"`, which only fires when the providers are real (not stubs).

### Seed → IPC fixture pre-loading

`render-with-harness.tsx:125-138` calls `seedInvokeCommandFixtures(ipcSeed.responses)` before `render(...)`. The `PingRuntimeProbe` test component (`temp-harness.test.tsx:50-68`) calls the singleton `invokeCommand("ping_runtime", {})` from `useEffect`; the assertion at `temp-harness.test.tsx:163-175` confirms the rendered text is the documented seed response (`ping_runtime:0A:agent-harness`) **and** `sideEffectSpies.invoke` (the `vi.mock` of `@tauri-apps/api/core`) was never called. The test for `get_harness_settings` (`temp-harness.test.tsx:177-203`) provides equivalent coverage for the second seeded command.

### Seed-failure path

`render-with-harness.tsx:103-123` throws `TempHarnessErrorFailure` before constructing the React tree when `seedName` is in `seedFailures` (`unknown` → `UnknownSeed`, `missing-manifest` → `FixtureManifestMissing`). The test at `temp-harness.test.tsx:205-227` iterates both documented failure seeds, asserts the throw, asserts the error carries `{ kind, seedName }` exactly, and verifies via `waitFor` that no `data-testid="failed-<seed>"` element exists in the document — proving no partial mount survived.

### Acceptance-criteria mapping

| AC | Source | Verification |
| --- | --- | --- |
| AC1 (`RenderResult` keys + queries) | `temp-harness.test.tsx:130-148` | iterates `required_keys`; asserts `getByTestId`/`queryByText` work |
| AC2 (TanStack Query + router providers wrap UI) | `temp-harness.test.tsx:150-161` | `ProviderProbe` reads `useQueryClient` + `useRouter` |
| AC3 (IPC fixtures seeded via WU-0A-14a handle) | `temp-harness.test.tsx:163-175,177-203` | `PingRuntimeProbe` and `HarnessSettingsProbe` resolve seeded responses; mocked Tauri invoke never called |
| AC4 (no agents binary spawn, no GraphStore migrations) | `temp-harness.test.tsx:177-203` | `structuralAbsence.forbidden_tauri_commands` (`spawn_agents`, `apply_graphstore_migrations`, `sqlx_migrate`) and `forbidden_agents_binary` not invoked; `phase0a_registered_commands_remain === ["subscribe_workspace_events"]` |
| AC5 (documented seed failure throws before mounting) | `temp-harness.test.tsx:205-227` | iterates both `seedFailures`; asserts throw + no `data-testid` in document |

### `src/ipc/invoke-command.ts` change

The diff against `main` is `+32/-1` lines, all additive:

- New types `InvokeCommandFixtureResponses` (`src/ipc/invoke-command.ts:20-22`).
- Module-level `activeFixtureResponses: ReadonlyMap<HarnessCommand, unknown> | undefined` (line 24).
- `seedInvokeCommandFixtures` / `clearInvokeCommandFixtures` setters (lines 26-34).
- A 16-line conditional inside `defaultInvokeShim` (lines 36-54): when `activeFixtureResponses` is set, parse the command name through the existing `parseHarnessCommand`, return the seeded response, or throw `CommandErrorFailure` with `CommandError.InvokeRejected` for an unseeded command. When `activeFixtureResponses` is `undefined`, fall through to the original `tauriInvoke` call.

The semantics are strictly inactive-unless-seeded:

1. The default state is `activeFixtureResponses === undefined`. Existing callers that import `invokeCommand` without going through `renderWithHarness` see exactly the original WU-0A-07 behavior.
2. WU-0A-07's contract test (`src/test/harness-command.test.ts`) constructs its own `createInvokeCommand(shim)` with a custom shim, never touching the singleton, so the new conditional is never executed by those tests. Confirmed by `bun run test`: 10/10 WU-0A-07 tests pass.
3. The CommandError taxonomy (`UnknownCommand`, `ArgumentSerializationFailed`, `InvokeRejected`, `ResponseDeserializationFailed`) is unchanged. The seam reuses `CommandError.InvokeRejected` for unseeded commands, which is consistent with the existing semantics ("the underlying invoke path refused the call").
4. `parseHarnessCommand` is reused, so an unknown command still routes through the existing `UnknownCommand` path before the seed lookup matters.
5. `clearInvokeCommandFixtures` is wired into both `afterEach` (module-level in `render-with-harness.tsx:27-29`) and the returned `unmount` (`render-with-harness.tsx:90-94`), so cross-test leakage is contained.

The change is a small, additive registry hook on the WU-0A-07 default shim. It is necessary to satisfy the WU-0A-14b contract clause "Seeds the WU-0A-07 `invokeCommand` fixture shim with the command responses documented for the selected seed", because the components-under-test import the singleton `invokeCommand` rather than constructing their own through `createInvokeCommand`.

### Phase boundary

No Phase 0B+ behavior. The renderer:

- Does not register a Tauri command (`phase_0a_scaffold_commands()` length is 1, asserted by `structuralAbsence.phase0a_registered_commands_remain` in the test).
- Does not run migrations (`structuralAbsence.renderer_applies_migrations === false`).
- Does not invoke real Tauri (`structuralAbsence.renderer_invokes_tauri === false`; `vi.mock("@tauri-apps/api/core")` ensures the spy is never called).
- Does not spawn `/home/nes/.local/bin/agents` (asserted by `structuralAbsence.forbidden_agents_binary` not present in any spy call).
- Does not introduce providers, credentials, graph DDL, or workspace persistence.

### Code/test boundary alignment

Ticket code boundary: `src/test/render-with-harness.tsx`. The implementer also modified `src/ipc/invoke-command.ts`, which sits outside that boundary (it is a WU-0A-07 owned file). Test boundary files (`src/contracts/temp-harness.ts`, `src/test/temp-harness.test.tsx`, `product-strategy/contracts/wu-0a-14b-render-with-harness.md`, fixture JSON under `product-strategy/contracts/fixtures/wu-0a-14b/`) are exactly what the ticket lists. See `WU-0A-14b-SCOPE-F01` below.

## Findings

### WU-0A-14b-SCOPE-F01 — `src/ipc/invoke-command.ts` modification crosses the ticket's stated code boundary (LOW)

**Where:** `src/ipc/invoke-command.ts:20-54` (32 added lines)

**Detail:** The ticket's `Code boundary` is `src/test/render-with-harness.tsx`. The implementer added a fixture-seam (`InvokeCommandFixtureResponses`, `seedInvokeCommandFixtures`, `clearInvokeCommandFixtures`, plus a 16-line conditional inside `defaultInvokeShim`) to a WU-0A-07-owned file. The contract field for WU-0A-14b explicitly requires "Seeds the WU-0A-07 `invokeCommand` fixture shim", and the semantics are strictly inactive-unless-seeded: the singleton's behavior is identical to the WU-0A-07 implementation when `activeFixtureResponses === undefined`. The WU-0A-07 contract test (`src/test/harness-command.test.ts`) continues to pass without any modification to its assertions or fixtures.

**Impact:** Within the explicit "MEDIUM minimum" rule (which fires only if the modification expands beyond inactive-unless-seeded), this stays LOW. Nonetheless, the ticket's code boundary was crossed; a future re-read of the ticket alone would not reveal the WU-0A-07 file edit.

**Recommendation:** Optional — accept as-is. If the reviewer wants stricter boundary discipline for future Layer-3 tickets, the alternative would have been to expose the registry as a separate `src/test/invoke-command-fixture-registry.ts` module imported by `defaultInvokeShim` (still a WU-0A-07 file edit, just smaller).

## Verdict

**LOW.** All five acceptance criteria are exercised; `RenderResult` keys, provider wrapping, IPC seeding, no-real-runtime invariant, and seed-failure throw are all verified by component tests; the `src/ipc/invoke-command.ts` modification is bounded and inactive unless seeded, and preserves the WU-0A-07 contract (10/10 tests still pass). One LOW finding records that the ticket's stated code boundary was crossed, accepted because the contract field explicitly required seeding the WU-0A-07 shim and the change is within the rubric's "inactive unless seeded" constraint.
