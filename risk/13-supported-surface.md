# WU-0A-13 Risk Gate: Supported Surface

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The implementation stays inside the Phase 0A frontend supported surface. There are zero Rust file changes, `phase_0a_scaffold_commands()` is unchanged at length 1 with the single entry `"subscribe_workspace_events"`, no new Tauri command is registered, no GraphStore query / migration / sqlx call appears anywhere in the diff, no `/home/nes/.local/bin/agents` spawn is reachable from the route render, and no provider config file read is performed. The proposal's "Anti-Scope" and "Supported Surface" sections accurately describe the implementation, and the side-effect-absence fixture matches the test's assertions.

## Verification

### Zero Rust changes
```
git diff main -- src-tauri/   →   (empty)
```
The Rust scaffold is untouched. `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features -- -D warnings` all pass against the existing code.

### `phase_0a_scaffold_commands()` is pinned

`src-tauri/src/lib.rs:15-30`:
```rust
const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"];
...
pub fn phase_0a_scaffold_commands() -> &'static [&'static str] {
    &PHASE_0A_SCAFFOLD_COMMANDS
}
```

The Rust unit test at line 78 asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`, plus the integration tests in `src-tauri/tests/scaffold_contract.rs:163`, `backend_span_event_contract.rs:318`, `harness_app_state_contract.rs:144`, and `subscribe_workspace_events_contract.rs:146` all re-assert the list. All pass. The frontend side-effect fixture (`product-strategy/contracts/fixtures/wu-0a-13/side-effect-absence.json`) records `"phase0aRegisteredCommandsRemain": ["subscribe_workspace_events"]`, and the Vitest test at `src/test/workspace-route-shell.test.tsx:151` asserts the fixture matches that exact list — providing the cross-source pinning.

### No GraphStore query, migration, sqlx, agents spawn, or provider config read in the frontend

```
grep -rn "agents|/home/nes/\.local/bin" src/   →
  src/test/scaffold.test.tsx:9        # comment in a different test
  src/test/workspace-route-shell.test.tsx:144   # the assertion test name
```
The only frontend mentions of `agents` and `/home/nes/.local/bin` are in test code asserting their absence, which matches the contract.

```
grep "invoke|GraphStore|migration|sqlx|fs/promises|readFile|import.meta.glob|Command\.create|Command\.spawn|invokeCommand"
   in src/contracts/workspace-route-shell.ts, src/shell/workspace-shell.tsx, src/routes/   →   (no matches)
```

The shipped code never imports `@tauri-apps/api/core`, `node:fs/promises`, or `@tauri-apps/plugin-shell`. The Vitest mock setup (`src/test/workspace-route-shell.test.tsx:12-23`) registers `vi.fn()` mocks for both seams and asserts neither was called during route render. The hoisted spy approach catches a regression even if a future contributor adds an import without exercising it.

### No new Tauri command registered

`src-tauri/src/lib.rs` is unchanged: the `tauri::generate_handler!` invocation at line 22 still registers only `commands::subscribe_workspace_events::subscribe_workspace_events`. No new command file in `src-tauri/src/commands/` (there is no diff). No new entry in `PHASE_0A_SCAFFOLD_COMMANDS`. The `WU-0A-07` invoke helper is not even imported by the WU-0A-13 source files (verified via grep above).

### Proposal supported-surface track matches implementation

`proposals/13-wu-0a-13.md` "Supported Surface" (lines 50-58) describes:
- Local Phase 0A Tauri/Vite scaffold ✓ (no deployment changes)
- User-reachable paths `/`, `/workspace/:workspaceId`, `/workspace/:workspaceId/node/:nodeId` ✓ (verified in `src/router.tsx`)
- Migration path: additive frontend route/component/contract fixtures ✓ (verified via diff stat)
- Rollback path: remove WU-0A-13 routes, shell, fixtures, tests ✓ (clean separation; only `src/router.tsx` would need a tiny revert)
- Observability: visible `data-testid` indicators ✓ (`workspace-shell`, `workspace-id`, `selected-node-id`, `active-pane`, `runtime-connected`, `shell-region-state-json`)

The "Anti-Scope" list (proposal lines 41-48) — no Rust, no new Tauri commands, scaffold list still length 1, no GraphStore / agents / provider / `ping_runtime` — is satisfied per the checks above.

### Build artifact pinning
`bun run lint`, `bun run typecheck`, `bun run test`, `bun run test:e2e`, `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` all pass. The Rust unit + integration tests independently re-assert the scaffold command list at every Rust seam.

## Findings

None.

## Verdict

**LOW.** Zero Rust diff. Scaffold command list pinned at 1 entry from both the Rust source and the frontend fixture cross-check. No GraphStore / agents / provider / migration / new-Tauri-command surface added. Proposal's supported-surface and anti-scope sections match the diff one-for-one.
