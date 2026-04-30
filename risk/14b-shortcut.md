# WU-0A-14b Risk Gate: Shortcut / Placeholder

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The TypeScript `TempHarnessHandle` DTO mirrors the Rust shape exactly: four required fields with strict primitive checks and an exact-key sort comparison that rejects extra or missing keys. The DTO parser performs real shape validation, not a blanket cast. Documented seed failures throw `TempHarnessErrorFailure` before any React tree is constructed and before any `render()` call, so no partial mount survives. IPC fixture seeding is real — `invokeCommand("ping_runtime", {})` from a child component resolves to the documented seeded response and the `vi.mock` of `@tauri-apps/api/core` is never invoked. No TODOs or unimplemented stubs are present in the shipped code.

## Verification

### TS DTO mirrors Rust shape

`src/contracts/temp-harness.ts:1-6` declares:

```ts
export interface TempHarnessHandle {
  workspace_id: string;
  database_path: string;
  fixture_manifest_id: string;
  app_state_ready: boolean;
}
```

This matches the Rust `TempHarnessHandle` (verified earlier in WU-0A-14a via `src-tauri/src/contracts/temp_harness.rs`) with the same four field names in `snake_case`, in the same order, with the same primitive types (`string`, `string`, `string`, `bool`). The error union `TEMP_HARNESS_ERRORS` (`temp-harness.ts:8-14`) lists exactly the same five variants that the Rust enum exposes (`UnknownSeed`, `DatabaseCreateFailed`, `AppStateInitFailed`, `RealAgentsInvocationAttempted`, `FixtureManifestMissing`). The fixture parity test (`src/test/temp-harness.test.tsx:99-128`) cross-checks both the handle round-trip via `handleRoundTrip` (the WU-0A-14a fixture) and the error variants via `tempHarnessErrors` (also WU-0A-14a fixture).

### DTO parser performs real shape validation

`temp-harness.ts:46-70` (`isTempHarnessHandle`) does the following:

1. Rejects non-object, null, or array inputs.
2. Sorts the actual keys and compares against the sorted allowed-keys list (`["app_state_ready", "database_path", "fixture_manifest_id", "workspace_id"]`). This rejects both missing keys (length mismatch) and extra keys (key inequality).
3. Type-checks each field individually (`typeof === "string"` for the three string fields; `typeof === "boolean"` for `app_state_ready`).

This is not a blanket `any` cast. The test `parseTempHarnessHandle({ ...handleRoundTrip, extra: true })` is asserted to throw (`temp-harness.test.tsx:125-127`), and the round-trip test confirms the parser returns a value `toEqual` to the input fixture rather than just casting it through.

`parseTempHarnessError` (lines 38-44) requires `typeof === "string"` and membership in `TEMP_HARNESS_ERRORS` before returning. `temp-harness.test.tsx:124` asserts `"OtherError"` is rejected.

### Seed-failure is a real throw, not a partial mount

`render-with-harness.tsx:46-101` flow:

1. Line 47: `resolveTempHarnessSeed(seedName)` is called first.
2. Lines 103-123: if `seedName` matches a `seedFailures` entry, throw `TempHarnessErrorFailure(parseTempHarnessError(failure.expected_error), { seedName })`. If no canonical seed match is found, throw `TempHarnessErrorFailure("UnknownSeed", ...)`.
3. Line 48: only on success of step 1, `seedHarnessIpcFixtures` runs (which can also throw `FixtureManifestMissing`).
4. Lines 50-77: only on success of steps 1–2, `QueryClient`, `router`, and the JSX tree are constructed.
5. Line 80: only after construction, `render(...)` is called.

So the throw happens before any DOM mount. The test (`temp-harness.test.tsx:205-227`) confirms this end-to-end: it iterates both `seedFailures` (`unknown` and `missing-manifest`), asserts the throw, asserts the error fields (`{ kind, seedName }`), and `await waitFor` asserts that no `data-testid="failed-<seed>"` element exists in the document — proving no partial mount survived the throw. This is not a `console.warn` followed by a partial mount; it is a hard throw before mount.

The render path also has a `try/catch` around `render(createTree())` (lines 79-100) that calls `clearInvokeCommandFixtures()` and `queryClient.clear()` before re-throwing, so any subsequent runtime error during the actual render also clears the registry — preventing seed leakage into the next test.

### IPC fixture seeding is real

`render-with-harness.tsx:125-138` calls `seedInvokeCommandFixtures(ipcSeed.responses)`, which writes to the module-level `activeFixtureResponses` map (`src/ipc/invoke-command.ts:24-30`). When the seeded `PingRuntimeProbe` then calls the singleton `invokeCommand("ping_runtime", {})`, the path is:

1. `createInvokeCommand`'s closure parses the command name (`parseHarnessCommand("ping_runtime")` → `"ping_runtime"`).
2. Calls `defaultInvokeShim("ping_runtime", {})`.
3. `defaultInvokeShim` sees `activeFixtureResponses !== undefined`, parses the command, finds it in the map, and returns `Promise.resolve(map.get("ping_runtime"))` — i.e., the documented seeded response object.
4. The returned value flows back through `parseHarnessCommandResponse("ping_runtime", value)`, which validates the response shape (so a malformed seeded response would still be rejected through `ResponseDeserializationFailed`).

The probe then renders the seeded response text. The test (`temp-harness.test.tsx:163-175`) confirms `await screen.findByTestId("ping-runtime-response")` produces `ping_runtime:0A:agent-harness` — the exact composition of `command:phase:runtime` from the `minimal-runtime` ipc-seeding fixture. Crucially, `expect(sideEffectSpies.invoke).not.toHaveBeenCalled()` confirms the `vi.mock("@tauri-apps/api/core")` was never reached — proof that the seeded response is real, not a stubbed bypass.

### No real Tauri invoke

`temp-harness.test.tsx:26-32` declares:

```ts
const sideEffectSpies = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: sideEffectSpies.invoke }));
```

This replaces the entire `@tauri-apps/api/core` module with a mocked `invoke` function that records calls. Even if the fixture-registry path were bypassed, the underlying `tauriInvoke` would still go to the spy, not to a real Tauri runtime. Across every render test (`temp-harness.test.tsx:163-203`), `sideEffectSpies.invoke` is asserted to have zero calls. The forbidden-command sweep at lines 193-202 also asserts no call ever matches `spawn_agents`, `apply_graphstore_migrations`, `sqlx_migrate`, or `/home/nes/.local/bin/agents` paths.

### No TODOs / unimplemented stubs

Grep across `src/test/render-with-harness.tsx`, `src/contracts/temp-harness.ts`, and `src/ipc/invoke-command.ts` for `TODO`, `FIXME`, `XXX`, or `unimplemented` returns no matches. Every code path either returns a real value, throws a typed error, or reuses a documented WU-0A-07 mechanism (`createInvokeCommand`, `parseHarnessCommand`, `parseHarnessCommandResponse`).

## Findings

None above LOW.

### WU-0A-14b-SHORTCUT-F01 — Module-level `afterEach` registration in `render-with-harness.tsx` (LOW)

**Where:** `src/test/render-with-harness.tsx:27-29`

**Detail:** `afterEach(() => { clearInvokeCommandFixtures(); })` is called at module scope (outside any `describe` block) when the file is imported. Vitest tolerates this and registers the hook against the active test context, but it relies on importing the file from a vitest test runner. Outside vitest the import would fail. Since the file lives in `src/test/` and is only ever imported by other test files, this is contained — but it is a slightly unusual pattern.

**Impact:** None at the contract level. The `unmount` returned by `renderWithHarness` also calls `clearInvokeCommandFixtures()` (line 92), so per-call cleanup is layered: the explicit `unmount` path always clears, and the test-runner hook is a defense-in-depth backstop for tests that throw before unmount.

**Recommendation:** Optional — accept as-is. If the project later moves to a non-vitest test runner, the module-level `afterEach` would need to migrate to the test files that consume `renderWithHarness`.

## Verdict

**LOW.** The TypeScript DTO mirrors the Rust shape exactly with strict shape validation; documented seed failures hard-throw before mount; IPC fixture seeding is structurally real (verified by no spy invocation on the mocked Tauri module); no real Tauri invoke is reachable; no TODOs or unimplemented stubs ship. One LOW finding flags the module-level `afterEach` pattern but it is contained.
