# WU-0A-14b Risk Gate: Supported Surface

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

WU-0A-14b is a frontend-only Vitest/jsdom utility. The implementation introduces no new Tauri commands, no Rust file changes, no real subprocess spawn, and no GraphStore migration apply. `phase_0a_scaffold_commands()` is unchanged at length 1 (`["subscribe_workspace_events"]`). The `src/ipc/invoke-command.ts` modification is inactive by default — consumers that do not call `seedInvokeCommandFixtures` (i.e., everything outside `renderWithHarness`) see the original WU-0A-07 path through `tauriInvoke`. The proposal's Supported Surface section matches the implementation. No provider-credential or config-file reads were added.

## Verification

### `phase_0a_scaffold_commands()` unchanged

`git diff main -- src-tauri/` is empty. `src-tauri/src/lib.rs:30-32` continues to expose `phase_0a_scaffold_commands()` returning `&PHASE_0A_SCAFFOLD_COMMANDS` with length 1. `cargo test --manifest-path src-tauri/Cargo.toml` passes all suites including:

- `subscribe_workspace_events_contract.rs:146` — asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`.
- `harness_app_state_contract.rs:141-144` — asserts the slice length and contents.
- `temp_harness_contract.rs:183` — asserts the same after WU-0A-14a's contract.
- `backend_span_event_contract.rs:318` — asserts `["subscribe_workspace_events"]`.
- `scaffold_contract.rs:163` — uses the same accessor.

The component-level structural-absence fixture (`product-strategy/contracts/fixtures/wu-0a-14b/structural-absence.json:8-10`) declares `phase0a_registered_commands_remain: ["subscribe_workspace_events"]` and the test (`src/test/temp-harness.test.tsx:186-188`) asserts equality.

### Zero Rust file changes

`git diff main -- src-tauri/` returns nothing. The implementation neither adds nor modifies any `.rs` file.

### No real `/home/nes/.local/bin/agents` spawn

The renderer never invokes the singleton `invokeCommand` for `spawn_agents` (it is not in the `HarnessCommand` taxonomy at all). The seed-IPC fixtures (`product-strategy/contracts/fixtures/wu-0a-14b/ipc-seeding.json`) only seed `get_harness_settings`, `ping_runtime`, and `subscribe_workspace_events`. The structural-absence fixture lists `forbidden_agents_binary: "/home/nes/.local/bin/agents"`, and the test (`temp-harness.test.tsx:199-202`) asserts `sideEffectSpies.invoke` was never called with any string containing that path. The seeded `agent_runner_bin` value is a fixture-only path (`product-strategy/contracts/fixtures/wu-0a-02/bin/fake-agents`) — not a real binary.

### No real GraphStore migration apply

The renderer never invokes `apply_graphstore_migrations` or `sqlx_migrate` (neither exists in the `HarnessCommand` taxonomy). `structuralAbsence.forbidden_tauri_commands` lists both, and the test sweep at `temp-harness.test.tsx:193-198` asserts neither is invoked through the mocked Tauri path. `structuralAbsence.renderer_applies_migrations === false` is asserted at line 190.

### `src/ipc/invoke-command.ts` is inactive by default

The 32-line addition is gated on `activeFixtureResponses !== undefined`:

```ts
if (activeFixtureResponses) {
  // seeded path
}
return tauriInvoke<TResponse>(command, args as Record<string, unknown>);
```

`activeFixtureResponses` is `undefined` until `seedInvokeCommandFixtures` is called, and it is `clearInvokeCommandFixtures` after every test (via the module-level `afterEach` and the returned `unmount`). Consumers that do not go through `renderWithHarness` — production code, the WU-0A-07 contract test, the WU-0A-13 workspace-route-shell test, etc. — see the original WU-0A-07 path. The WU-0A-07 contract test (`src/test/harness-command.test.ts`) constructs its own `createInvokeCommand(shim)` instances and never touches the singleton, so the conditional cannot influence it.

Confirmed by `bun run test`:

- `src/test/harness-command.test.ts` — 10/10 pass (WU-0A-07).
- `src/test/scaffold.test.tsx` — 1/1 pass.
- `src/test/workspace-route-shell.test.tsx` — 17/17 pass (WU-0A-13).
- `src/test/subscribe-workspace-events.test.ts` — 2/2 pass (WU-0A-08).
- `src/test/backend-span-event.test.ts` — 5/5 pass (WU-0A-10).
- All other contract tests pass.

Total: 86/86 across 13 files.

`bun run typecheck` and `bun run lint` both pass (cached or fresh, all clean).

### No provider config or credential reads

`src/test/render-with-harness.tsx` and `src/contracts/temp-harness.ts` import only:

- `@tanstack/react-query` (provider construction).
- `@tanstack/react-router` (provider construction).
- `@testing-library/react` (render utility).
- `react`, `vitest`.
- WU-0A-14a fixture JSON (canonical seed renders, IPC seeding, seed failure).
- `../contracts/temp-harness` (the new DTO).
- `../ipc/invoke-command` (the seeded shim hook).

No filesystem provider config, no `.env` reads, no credential vault access, no telemetry or runtime producer wiring.

### Proposal supported-surface track matches

`proposals/14b-wu-0a-14b.md:37-49` declares:

- Deployment mode: frontend Vitest/jsdom tests only — matches.
- Customer cohort: contributors writing RTL tests for Phase 0A frontend UI — matches.
- Adjacent supported paths: `AppShell` retains production QueryClient/router wiring, WU-0A-07 retains command parsing, WU-0A-14a retains Rust temp harness state — matches.
- Migration path: import `renderWithHarness` from `src/test/render-with-harness` — matches.
- Rollback path: remove utility, contract DTO, fixtures; no persisted state — matches (the only modified file outside that scope is `src/ipc/invoke-command.ts`, where rollback would also remove the 32-line additive seam without breaking anything).
- Observability: Vitest assertions observe provider reachability, seeded IPC, no Tauri fallback, no mounted result on failed seeds — matches the test file structure.

The proposal's Anti-Scope section ("No Rust file changes; No new Tauri commands; No graph/provider/migration/credential/process-spawn code; No changes to phase 0A command registration count") is fully respected.

## Findings

None above LOW.

### WU-0A-14b-SUPPORTED-F01 — Frontend-only seam reuses `CommandError.InvokeRejected` for unseeded commands (LOW)

**Where:** `src/ipc/invoke-command.ts:47-50`

**Detail:** When a component-under-test calls `invokeCommand` with a command name that is not in the seeded fixture map, the seam throws `CommandErrorFailure` with `CommandError.InvokeRejected`. This reuses an existing taxonomy variant rather than introducing a new one (e.g., `FixtureNotSeeded`), which keeps the WU-0A-07 contract unchanged. The error message in the underlying `cause` includes the unseeded command name (`No invoke fixture seeded for ${parsedCommand}`), so debug visibility is preserved. The semantics are reasonable: from the singleton caller's perspective, the underlying invoke path did refuse the call.

**Impact:** None at the contract level. Tests that need to assert "no fixture for X" can match on the `cause` message.

**Recommendation:** Optional — accept as-is. Reusing `InvokeRejected` is the right call to avoid expanding the WU-0A-07 `CommandError` enum from a WU-0A-14b ticket.

## Verdict

**LOW.** No new Tauri commands; zero Rust file changes; `phase_0a_scaffold_commands()` length unchanged at 1; no real spawn or migration apply; the `src/ipc/invoke-command.ts` modification is gated on `activeFixtureResponses !== undefined` and inactive in all non-`renderWithHarness` flows; no provider config/credential reads; the proposal's supported-surface track matches the implementation. Cargo and bun toolchains are clean.
