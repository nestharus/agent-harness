# WU-0A-14b Risk Gate: Multi-Concern PR

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The PR is single-concern. The diff against `main` contains only WU-0A-14b artifacts plus the bounded `src/ipc/invoke-command.ts` seed-registry hook (`+32/-1` lines). No incidental edits leak into WU-0A-01/02/03/04/05/06/08/09/10/11/12/13/14a implementation files, no `package.json`/`vite.config.ts`/`playwright.config.ts` modifications, no `bun.lock` change, no Rust file edits. The WU-0A-07 contract test (`src/test/harness-command.test.ts`) continues to pass against the modified `invoke-command.ts`.

## Verification

### `git status -s`

```
 M src/ipc/invoke-command.ts
?? product-strategy/contracts/fixtures/wu-0a-14b/
?? product-strategy/contracts/wu-0a-14b-render-with-harness.md
?? proposals/14b-wu-0a-14b.md
?? src/contracts/temp-harness.ts
?? src/test/render-with-harness.tsx
?? src/test/temp-harness.test.tsx
```

### `git diff main --stat`

```
 src/ipc/invoke-command.ts | 33 ++++++++++++++++++++++++++++++++-
 1 file changed, 32 insertions(+), 1 deletion(-)
```

(Untracked WU-0A-14b files are not yet staged but match the ticket's code/test boundary; the diff stat correctly shows that the only existing-file modification is `src/ipc/invoke-command.ts`.)

### Bounded `invoke-command.ts` modification

The 33-line diff is purely additive:

| Section | Lines | Purpose |
| --- | --- | --- |
| `InvokeCommandFixtureResponses` type | 20-22 | new exported partial-record type |
| `activeFixtureResponses` module state | 24 | private registry, default `undefined` |
| `seedInvokeCommandFixtures` | 26-30 | setter |
| `clearInvokeCommandFixtures` | 32-34 | reset |
| `defaultInvokeShim` body wrap | 36-54 | conditional fixture lookup before `tauriInvoke` |

Every existing export (`TauriInvokeShim`, `InvokeCommand`, `createInvokeCommand`, `invokeCommand` singleton, `assertJsonSerializable`, `validateJsonValue`, `validateJsonObject`) is unchanged. The conditional inside `defaultInvokeShim` only activates when `activeFixtureResponses !== undefined`. This is a small additive registry hook, not a re-architecture. Diff size: 32 added lines (well under the rubric's ~30-line scrutiny threshold; the spillover is two single-line additions for the new exports plus a 16-line conditional).

### WU-0A-07 contract test still passes

`bun run test` output:

```
✓ src/test/harness-command.test.ts (10 tests) 14ms
```

All ten tests pass:

1. `keeps the TypeScript union aligned with the canonical command fixture` — passes.
2. `rejects alternate command forms from the shared fixture` — passes.
3. `keeps CommandError variants aligned with fixtures` — passes.
4. `resolves ping_runtime with the documented fixture response through the invoke shim` — passes.
5. `preserves get_harness_settings command and empty args while parsing HarnessSettings` — passes.
6. `preserves subscribe_workspace_events topic and channel registration args without a producer` — passes.
7. `rejects unknown commands before invoking the shim` — passes.
8. `rejects non-serializable arguments before invoking the shim` — passes.
9. `maps a rejected invoke shim to InvokeRejected` — passes.
10. `maps invalid fixture responses to ResponseDeserializationFailed` — passes.

These tests use `createInvokeCommand(shim)` with custom shims and never touch the singleton or the new seed registry, so the modification cannot influence them — confirming "inactive by default" semantics at the test level.

### No incidental edits to other WU implementation files

`git diff main --stat` shows exactly one file modified (`src/ipc/invoke-command.ts`). No edits to:

- WU-0A-01 (`src-tauri/src/lib.rs`, `src-tauri/src/main.rs`, `package.json`, scaffold).
- WU-0A-02 (`src/contracts/harness-settings.ts`, `src-tauri/src/contracts/harness_settings.rs`).
- WU-0A-03 (`src/contracts/local-storage-layout.ts`, etc.).
- WU-0A-04 (`src-tauri/src/app_state.rs`, etc.).
- WU-0A-05 (`src/contracts/event-topic.ts`, etc.).
- WU-0A-06 (`src/contracts/ipc-event.ts`, etc.).
- WU-0A-07 contract surface (`src/contracts/harness-command.ts` is unchanged; only `src/ipc/invoke-command.ts` got the seed-registry hook).
- WU-0A-08 (`src-tauri/src/ipc/subscribe_workspace_events.rs`, etc.).
- WU-0A-09 (`src/contracts/trace-context.ts`, etc.).
- WU-0A-10 (`src/contracts/backend-span-event.ts`, etc.).
- WU-0A-11 (`src/contracts/pane-id.ts`).
- WU-0A-12 (`src/state/shell-region-state.ts`).
- WU-0A-13 (`src/routes/workspace-route.tsx`, `src/components/AppShell.tsx`, etc.).
- WU-0A-14a (`src-tauri/src/test_harness/temp_harness.rs`, etc.).

### `package.json` / `vite.config.ts` / `playwright.config.ts` unchanged

`git diff main -- package.json vite.config.ts playwright.config.ts` returns empty. No new dependencies were declared and no test/build configuration was touched.

### `bun.lock` unchanged

`git diff main -- bun.lock` returns empty. No dependency graph change.

### No Rust file changes

`git diff main -- src-tauri/` is empty. `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings` all clean.

### Single-concern PR

The PR maps to one concern: introduce the `renderWithHarness(ui, seedName)` test utility plus its WU-0A-14a-derived `TempHarnessHandle` DTO mirror, fixtures, and contract documentation. The only out-of-boundary edit is the bounded fixture-seam hook in `src/ipc/invoke-command.ts`, which is required by the WU-0A-14b contract field ("Seeds the WU-0A-07 `invokeCommand` fixture shim with the command responses documented for the selected seed"). The hook is not a separate concern — it is the implementation of the required contract clause for components that import the singleton `invokeCommand`.

## Findings

None above LOW.

### WU-0A-14b-MULTI-CONCERN-F01 — `src/ipc/invoke-command.ts` edit crosses ticket code boundary (LOW, mirror of SCOPE-F01)

**Where:** `src/ipc/invoke-command.ts:20-54`

**Detail:** Same as `WU-0A-14b-SCOPE-F01`. The ticket's stated `Code boundary` is `src/test/render-with-harness.tsx`, but the implementation also modifies `src/ipc/invoke-command.ts` (a WU-0A-07-owned file). The modification is bounded (32 added lines), additive, and inactive unless seeded. Filed here as a multi-concern note because the cross-WU edit is the only signal that the PR touches anything outside its declared boundary.

**Impact:** None at the test or contract level. WU-0A-07's 10/10 tests pass unchanged. WU-0A-13's 17/17 workspace-route-shell tests pass unchanged.

**Recommendation:** Optional — accept as-is.

## Verdict

**LOW.** Single concern, single existing-file edit (`+32/-1`), no incidental edits to other WU implementation files, no `package.json`/`vite.config.ts`/`playwright.config.ts`/`bun.lock` changes, no Rust file edits, WU-0A-07 contract test continues to pass. The one cross-boundary edit is bounded, inactive-unless-seeded, and required by the WU contract field.
