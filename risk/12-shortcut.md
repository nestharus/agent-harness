# WU-0A-12 — Shortcut / Placeholder Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Does any shipped code rely on placeholders, TODOs, half-implemented stubs, blanket `unknown`/`any` passthrough, fall-through error swallowing, or weak validation that would let the parser silently admit invalid `ShellRegionState` inputs and break the WU contract (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-12.md`, `product-strategy/contracts/wu-0a-12-shell-region-state.md`)?

## Findings

### No placeholders / TODOs / unimplemented stubs

`src/shell/shell-region-state.ts:1-10` is a pure interface declaration. `src/contracts/shell-region-state.ts:1-98` contains no `TODO`, `FIXME`, `XXX`, `unimplemented`, `throw new Error("not implemented")`, `any`, or `as any` token (verified by inspection). Test file is `src/test/shell-region-state.test.ts:1-115` — fixtures-driven and free of `.skip` / `.todo`. Proposal and contract files are committed-quality prose with no "?" / "TBD" markers.

### Parser does real shape validation, not blanket passthrough

`parseShellRegionState` at `src/contracts/shell-region-state.ts:28-88` validates each documented field with strict checks before constructing the result:

- Input shape: `typeof input !== "object" || input === null || Array.isArray(input)` → `EmptyWorkspaceId` (line 29-31). Null and arrays — both `typeof === "object"` — are rejected explicitly.
- `workspaceId`: `typeof === "string"` AND `length !== 0` (line 35). Empty string AND non-string both fail per the contract row "Empty or non-string `workspaceId` returns `EmptyWorkspaceId`" (`…wu-0a-12-shell-region-state.md:64`).
- `activePane`: `typeof === "string"` (line 39) AND delegated through `parsePaneId` (line 43-46). Both the non-string and unknown-string branches return `UnknownPaneId`, matching `…wu-0a-12-shell-region-state.md:65`. The delegation reuses WU-0A-11's exact-match parser (`src/contracts/pane-id.ts:37-62`), so no trim / case-fold / aliasing is introduced.
- `actionNeededCount`: `typeof === "number"` AND `Number.isFinite` AND `>= 0` (lines 48-54). NaN and ±Infinity are rejected because `Number.isFinite` returns `false` for them. Maps to `NegativeActionNeededCount` consistently with the contract row "Negative or non-number `actionNeededCount` returns `NegativeActionNeededCount`" (`…wu-0a-12-shell-region-state.md:66`).
- `passiveProgressCount`: same triple check (lines 56-62) against `NegativePassiveProgressCount`.
- `runtimeConnected`: strict `typeof === "boolean"` (line 64). Truthy strings like `"true"` and numerics like `0`/`1` are rejected with `InvalidRuntimeConnected`. Covered explicitly by the `parse-errors.json` "invalid runtime connected" case using `"true"` (`fixtures/wu-0a-12/parse-errors.json:67-81`).
- `selectedNodeId`: optional — `undefined` accepted (lines 68-73), string accepted, all other types rejected.
- The success branch (lines 75-87) reconstructs the value field-by-field with `selectedNodeId` only present when it was a string, preserving the contract's "omitted when no shell node is selected" rule (`…wu-0a-12-shell-region-state.md:22`) and producing a clean `ShellRegionState` rather than echoing the raw input.

### Error variants are exhaustive — no fall-through that swallows unknown errors

There is no catch-all or default branch that maps unknown failures to a generic error. Every guard returns through the `failure(kind)` helper (`src/contracts/shell-region-state.ts:90-97`) with one of the five documented variants. The five variants are also pinned at runtime by the `SHELL_REGION_STATE_ERRORS` `as const` tuple (lines 4-10) and reconciled against `shell-region-state-errors.json` by the alignment test (`src/test/shell-region-state.test.ts:74-92`), so adding a sixth value fails the build.

The success branch is the only `ok: true` exit; if any guard fires, the function returns immediately, so no "fall-through to success on unrecognized shape" exists.

### Counters validated as non-negative finite numbers

The counter checks (lines 48-54, 56-62) reject:

- non-`number` types (`typeof !== "number"`),
- non-finite numbers (`!Number.isFinite(...)` — catches `NaN`, `Infinity`, `-Infinity`),
- negative numbers (`< 0`).

The acceptance criteria require non-negative numeric counters; integer-only is not required by the ticket or contract, so accepting `0.5` is contract-compliant. The `parse-errors.json` fixtures pin `-1` for both counters (`fixtures/wu-0a-12/parse-errors.json:35-65`).

### `runtimeConnected` validated as strict boolean

Line 64: `if (typeof record.runtimeConnected !== "boolean")` — rejects strings, numbers, `null`, `undefined`, objects, arrays. The "invalid runtime connected" fixture uses the string `"true"` to pin truthy-string rejection (`fixtures/wu-0a-12/parse-errors.json:67-81`), exactly satisfying the `InvalidRuntimeConnected` acceptance criterion (`WU-0A-12.md` "Non-boolean `runtimeConnected` returns `ShellRegionStateError::InvalidRuntimeConnected`.").

### Potential concern — does not rise above LOW

- NIT (lines 68-73): When `selectedNodeId` is present and is *not* a string and *not* `undefined` (e.g., `42`, `null`, `true`), the parser returns `UnknownPaneId`. The contract enumerates exactly five errors and is silent on this case (`…wu-0a-12-shell-region-state.md:54-69`); routing the failure through `UnknownPaneId` is misleading because `selectedNodeId` is not a pane id. No fixture exercises this path, so it does not break any acceptance criterion or test, and no shipped caller can reach it (the parser is unwired). Acceptable for Phase 0A scaffold; worth tightening if the WU later registers an `InvalidSelectedNodeId` variant or if a caller starts feeding raw IPC payloads. Not a shortcut against the contract — the contract simply does not pin behavior here.

### Verification commands

- `bun run lint` — clean (`turbo run lint:eslint lint:rust`).
- `bun run typecheck` — clean (`tsc --noEmit` and `cargo check`).
- `bun run test` — 8 files / 45 tests pass; `src/test/shell-region-state.test.ts` reports 21 tests in 6 ms.
- `cargo test --manifest-path src-tauri/Cargo.toml` — same prior-WU test inventory passes; no new tests introduced (TS-only WU).
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check` — clean.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` — clean.

## Verdict

**LOW.** Parser does field-by-field strict-shape validation, every documented error variant is reachable and asserted by a fixture, counters are validated as non-negative finite numbers, `runtimeConnected` is checked as a strict boolean, and there is no fall-through case that swallows unknown errors. No TODOs, no `any`/`unknown` passthrough, no skipped tests. The single nit — routing a non-string `selectedNodeId` to `UnknownPaneId` — is undocumented behavior the contract leaves open and is not exercised by any fixture or caller.
