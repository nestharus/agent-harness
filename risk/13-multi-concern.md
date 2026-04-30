# WU-0A-13 Risk Gate: Multi-Concern PR

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The PR is single-concern: every changed and newly added file is part of WU-0A-13 (route shell). There are no incidental edits to WU-0A-01 through WU-0A-12 implementation files, no `bun.lock` change, no `package.json` / `vite.config.ts` / `playwright.config.ts` / `vitest.config.ts` / `tsconfig.json` change, and no Rust diff. The three modifications outside the strict ticket boundary (`src/router.tsx`, `src/styles.css`, `src/test/setup.ts`) are each the minimum required to make the new routes runnable and testable.

## Verification

### Full diff inventory

`git diff main --name-only` (modified):
```
src/router.tsx
src/styles.css
src/test/setup.ts
```

Untracked (added by this WU):
```
product-strategy/contracts/fixtures/wu-0a-13/route-fixtures.json
product-strategy/contracts/fixtures/wu-0a-13/shell-region-state-round-trips.json
product-strategy/contracts/fixtures/wu-0a-13/side-effect-absence.json
product-strategy/contracts/wu-0a-13-workspace-route-shell.md
proposals/13-wu-0a-13.md
src/contracts/workspace-route-shell.ts
src/routes/workspace.$workspaceId.tsx
src/routes/workspace.$workspaceId.node.$nodeId.tsx
src/shell/workspace-shell.tsx
src/test/workspace-route-shell.test.tsx
tests-e2e/workspace-route-shell.spec.ts
```

(`risk/13-*.md` are gate artifacts, not part of the implementation PR.)

Every entry is on the WU-0A-13 boundary. There is no driver / WU-0A-14 / out-of-scope file in this list.

### Diff stat

```
src/router.tsx    |  15 ++++++--
src/styles.css    | 105 +++++++++++++++++++++++++
src/test/setup.ts |   5 +++
3 files changed, 122 insertions(+), 3 deletions(-)
```

Tiny, additive footprint outside of the strict boundary — see "off-boundary edits" below.

### `bun.lock` is unchanged

```
git diff main -- bun.lock   →   (empty)
```

`bun install` only repopulated `node_modules`; the lockfile was not modified.

### Build / config files unchanged

```
git diff main -- package.json vite.config.ts vitest.config.ts playwright.config.ts tsconfig.json   →   (empty)
```

No version bumps, no plugin additions, no path-alias rewrites, no test runner reconfigurations.

### Rust is untouched

```
git diff main -- src-tauri/   →   (empty)
```

### No edits to prior WU implementation files

Verified each of the WU-0A-01 .. 0A-12 source files is unchanged:

- `src/main.tsx`, `src/App.tsx`, `src/ShellRoot.tsx` — WU-0A-01: untouched.
- `src/contracts/local-storage-layout.ts`, `src/contracts/harness-settings.ts` — WU-0A-02 / 0A-03: untouched.
- Rust scaffold (WU-0A-04, 0A-05, 0A-06): untouched.
- `src/contracts/harness-command.ts`, `src/ipc/*` — WU-0A-07 / 0A-08: untouched.
- `src/contracts/event-topic.ts`, `src/contracts/ipc-event.ts`, `src/contracts/subscribe-workspace-events.ts` — WU-0A-08 / 0A-09: untouched.
- `src/contracts/trace-context.ts`, `src/contracts/backend-span-event.ts` — WU-0A-10: untouched.
- `src/contracts/pane-id.ts`, `src/shell/pane-id.ts` — WU-0A-11: untouched (`git diff main` empty for both).
- `src/contracts/shell-region-state.ts`, `src/shell/shell-region-state.ts` — WU-0A-12: untouched.

Only the additive `src/contracts/workspace-route-shell.ts` and `src/shell/workspace-shell.tsx` are introduced.

### Off-boundary edits (justification)

| File | Change | Justification |
| --- | --- | --- |
| `src/router.tsx` | Added `Outlet` import, swapped root component to a `RootOutlet`, imported the two new route factories, added them to `rootRoute.addChildren([...])`. | Required wiring — TanStack Router needs the routes registered to render them. The root-component swap is the minimal way to keep `/` rendering `ShellRoot` while letting workspace routes mount their own component. The existing scaffold smoke test (`src/test/scaffold.test.tsx`) still passes. |
| `src/styles.css` | Appended `.workspace-shell*` rules at end of file, plus a media query. | Additive only; no existing rule modified. |
| `src/test/setup.ts` | Added a `window.scrollTo` polyfill via `Object.defineProperty`. | TanStack Router calls `scrollTo` on every navigation; jsdom does not implement it. Without the polyfill every WU-0A-13 component test would throw. The change is two lines and applies harmlessly to all tests. |

None of these is a multi-concern signal; each maps directly to making WU-0A-13 runnable.

### Single-PR scope per ticket

The ticket "Handoff notes" (line 39-41 of the WU-0A-13 ticket) states "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR." The diff confirms compliance.

## Findings

None.

## Verdict

**LOW.** Single-concern PR. Diff is exactly the WU-0A-13 surface plus the three minimum-mechanical wiring touches required to make the new routes mount and the new component render in jsdom. No lockfile change, no config change, no Rust change, no edits to prior WU files.
