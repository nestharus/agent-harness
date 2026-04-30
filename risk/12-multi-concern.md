# WU-0A-12 — Multi-Concern PR Risk Review

**Gate:** Phase 4 multi-concern PR.
**Severity:** LOW.

## Question

Does the working tree contain only WU-0A-12 changes, or does it bundle work from another WU / unrelated concern that would force a multi-concern PR (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-12.md` "Single-concern PR" handoff note)?

## Findings

### Branch state

- Current branch: `impl-wu-0a-12`. HEAD == `main` (943b420). No committed delta yet — the implementer's work is staged in untracked working-tree files. The single-PR check therefore evaluates the union of `git diff main` and untracked additions.
- `git status` reports no modified tracked files; only the eight WU-0A-12 paths are listed under "Untracked files".

### Working-tree diff scope (vs. `main`)

`git ls-files -m -o --exclude-standard` lists exactly:

```
product-strategy/contracts/fixtures/wu-0a-12/canonical-states.json
product-strategy/contracts/fixtures/wu-0a-12/parse-errors.json
product-strategy/contracts/fixtures/wu-0a-12/shell-region-state-errors.json
product-strategy/contracts/wu-0a-12-shell-region-state.md
proposals/12-wu-0a-12.md
src/contracts/shell-region-state.ts
src/shell/shell-region-state.ts
src/test/shell-region-state.test.ts
```

All eight paths are WU-0A-12 artifacts — three TypeScript modules (DTO, parser, test) on the ticket's "Code boundary," three fixtures and the contract markdown on the ticket's "Test boundary," and the one expected proposal artifact (`proposals/12-wu-0a-12.md`). No file outside this set is added or modified.

### Zero Rust file changes

- `git diff main -- src-tauri/` is empty.
- `git ls-files -o --exclude-standard | grep -E '\.rs$|src-tauri'` returns nothing.
- `cargo test --manifest-path src-tauri/Cargo.toml` runs the same prior-WU test inventory (4 + 5 + 3 + 6 tests across `harness_settings_contract`, `local_storage_layout_contract`, `scaffold_contract`, `trace_context_contract`, plus the in-crate `phase_0a_registers_no_value_slice_commands`); no new tests appear, confirming no Rust edits.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check` and `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` are both clean (TS-only WU as the ticket prescribes).

### No incidental edits to prior-WU files

Specifically checked — all empty diffs:

- `git diff main -- src/shell/pane-id.ts` (WU-0A-11 metadata file).
- `git diff main -- src/contracts/pane-id.ts` (WU-0A-11 parser).
- `git diff main -- product-strategy/contracts/fixtures/wu-0a-11` (WU-0A-11 fixtures).
- `git diff main -- src/contracts/event-topic.ts src/contracts/ipc-event.ts src/contracts/trace-context.ts src/contracts/local-storage-layout.ts` (prior-WU contract files).
- `git diff main -- src/test` other than `shell-region-state.test.ts` — empty.

The WU-0A-12 test imports the WU-0A-11 fixture as a *read-only* peer (`src/test/shell-region-state.test.ts:3` reads `pane-ids.json`), which is the documented adjacent-path relationship in the proposal (`proposals/12-wu-0a-12.md:36`) and does not require any edit to WU-0A-11.

### No incidental edits elsewhere

`git status` shows no modifications to:

- `src/main.tsx`, `src/App.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`.
- `package.json`, `bun.lock`, `Cargo.toml`, `Cargo.lock`, `turbo.json`.
- Any other `proposals/*.md`, `risk/*.md`, or `product-strategy/contracts/*.md` outside the WU-0A-12 set.
- Any `tickets/` or `plans/` ref (those live on the orphan ticket branch).

The WU-0A-12 contract file `wu-0a-12-shell-region-state.md` and the three fixtures are new; all other contracts and fixtures are untouched.

### Verification commands

- `bun run lint` → clean.
- `bun run typecheck` → clean.
- `bun run test` → 8 files / 45 tests pass; `shell-region-state.test.ts` reports 21 tests.
- `cargo test --manifest-path src-tauri/Cargo.toml` → all prior-WU tests pass; no new tests.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check` → clean.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` → clean.

### Potential concerns — none rising to MEDIUM

- INFO: The WU-0A-12 work is uncommitted (HEAD == main on `impl-wu-0a-12`). For the multi-concern gate this is neutral — the diff scope is what matters, and the diff is single-concern. The implementer will need to create a single commit (or a small focused stack) for the WU-0A-12 PR before opening it; no other work is pending in the tree to leak into that commit.

## Verdict

**LOW.** Working tree against `main` contains only WU-0A-12 artifacts: the three TypeScript files on the ticket Code boundary, the contract markdown and three fixtures on the Test boundary, and the proposal artifact. Zero Rust diff. No incidental edits to prior-WU files (especially `src/shell/pane-id.ts` and `src/contracts/pane-id.ts`). Single-concern PR boundary holds.
