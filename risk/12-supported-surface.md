# WU-0A-12 — Supported-Surface Risk Review

**Gate:** Phase 4 supported surface.
**Severity:** LOW.

## Question

Does the implementation only add the public surface declared in the proposal's "Supported Surface" track (`proposals/12-wu-0a-12.md:31-39`) — TypeScript contract importers and shell view-model consumers in the local Phase 0A desktop scaffold — without expanding into IPC commands, GraphStore tables, provider credentials, agents invocation, router state, stores, or IPC subscription wiring (`proposals/12-wu-0a-12.md:24-29`, `tickets-phase-0a:plans/tickets/phase-0a/WU-0A-12.md`)?

## Findings

### Declared supported surface

Proposal "Supported Surface" track (`proposals/12-wu-0a-12.md:31-39`):

- Deployment mode: local Phase 0A desktop scaffold.
- Customer cohort: developer/test harness consumers only.
- Public paths: TypeScript contract importers and shell view-model consumers.
- Adjacent paths: `PaneId` parsing remains owned by WU-0A-11; router, stores, IPC, and Rust contracts remain unchanged.
- Migration path: additive DTO, parser, fixtures, and unit tests.
- Rollback path: remove the WU-0A-12 contract, DTO, fixtures, and tests.
- Observability: Vitest verifies parser success/error behavior and fixture parity. Parser is pure with no router/store/IPC dependency.

### Public surface lands on the declared paths only

New TypeScript exports (`src/contracts/shell-region-state.ts:4-12, 18-26, 28, 90`):

- `SHELL_REGION_STATE_ERRORS` (readonly tuple of five strings).
- `ShellRegionStateError` (string-literal union).
- `ShellRegionStateParseFailure` (interface `{ kind }`).
- `ShellRegionStateParseResult` (sum type).
- `parseShellRegionState(input: unknown): ShellRegionStateParseResult`.

DTO export (`src/shell/shell-region-state.ts:3-10`):

- `ShellRegionState` (interface).

These map cleanly onto "TypeScript contract importers and shell view-model consumers." No public re-exports (no `index.ts` barrel introduced), no namespace seeding beyond the two files documented in the ticket "Code boundary."

### Parser is pure

`src/contracts/shell-region-state.ts` imports only:

- `parsePaneId` from `./pane-id` (sibling pure parser, `src/contracts/pane-id.ts:37-62`).
- `type ShellRegionState` from `../shell/shell-region-state` (interface only — no runtime).

No imports of router, Zustand or Redux store, Tauri `invoke` / `listen`, IPC client, fetch, filesystem, environment, or worker APIs. The function reads only its `input` parameter and returns a value object — no module-scope mutable state, no side effects. The proposal's "Parser purity assumption" track (`proposals/12-wu-0a-12.md:54`) is enforced by import-boundary review and the dedicated test at `src/test/shell-region-state.test.ts:102-114`.

### No new IPC commands; Rust diff is empty

- `git diff main -- src-tauri/` is empty. `git ls-files -o --exclude-standard | grep -E '\.rs$|src-tauri'` returns nothing.
- `src-tauri/src/lib.rs:14-16` still defines `pub fn registered_command_count() -> usize { 0 }`. The `phase_0a_registers_no_value_slice_commands` Rust unit test at `src-tauri/src/lib.rs:21-26` still asserts `registered_command_count() == 0`, and `tauri_bootstrap_is_inert_and_command_free` (`src-tauri/tests/scaffold_contract.rs:136-160`) still passes (`cargo test --manifest-path src-tauri/Cargo.toml` is green). The WU-0A-01 invariant holds.
- No `tauri::generate_handler!` invocation, no Tauri command attribute (`#[tauri::command]`) added anywhere.
- No new IPC topic registered: `EVENT_TOPICS` (`src/contracts/event-topic.ts`) is unchanged; no `IpcEvent` envelope construction in this WU.

### No GraphStore tables, providers, or `agents` invocation

- No SQL / migration file added (`product-strategy/contracts/fixtures/wu-0a-03/local-storage-layout.json` and the GraphStore root remain untouched).
- No provider client / credential code: no `keytar`, `keyring`, OAuth, or HTTP client added; no environment variable read.
- No agent / worker code: no spawn, no `tauri::AppHandle`, no Rust thread, no Tokio task; the WU is data-shape only.
- Grep across `src/` for `ShellRegionState`: only the three WU-0A-12 files reference it; nothing in `src/main.tsx`, `src/App.tsx`, `src/ShellRoot.tsx`, or any feature module imports it. The DTO is not yet wired into a consumer — consistent with "shell view-model consumers" being a future-WU concern.

### Migration / rollback / observability

- Migration: pure additive — three new TypeScript files, three new fixture files, one contract markdown, one proposal markdown. No edits to existing files.
- Rollback: deleting the eight new paths leaves the tree byte-identical to `main` (HEAD == main; tree changes are entirely in untracked files).
- Observability: `src/test/shell-region-state.test.ts` runs 21 deterministic Vitest cases covering canonical round-trip, pane variant coverage, error-table coverage, type-level enforcement (`// @ts-expect-error` lines 27-33, 84-87), and parser-purity assumption documentation. Reported in `bun run test` output: "✓ src/test/shell-region-state.test.ts (21 tests) 6 ms".

### Potential concerns — none rising to MEDIUM

- INFO: The DTO is currently unused outside its own tests. That is the *expected* shape for a Phase 0A data-shape WU — the proposal's "shell view-model consumers" surface is a forward-looking declaration. Acceptable.
- INFO: Adjacent path "WU-0A-11 owns `PaneId`" is honored: WU-0A-12 imports `parsePaneId` and the `PaneId` type only; nothing in `src/contracts/pane-id.ts` or `src/shell/pane-id.ts` is modified (`git diff main -- src/contracts/pane-id.ts src/shell/pane-id.ts` empty).

## Verdict

**LOW.** Public surface is exactly the DTO interface, the parser, the error union, the parse-result sum type, and the canonical fixtures — all on the paths declared in the proposal. Parser is pure (no router/store/IPC imports), no new IPC commands are registered (`registered_command_count()` still returns 0 and the inert-bootstrap invariant still passes), no GraphStore / provider / agent surface is touched, and the WU is fully reversible by deleting the new files.
