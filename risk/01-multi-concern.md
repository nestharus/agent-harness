# WU-0A-01 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-01 scaffold — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-01.md`) explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Concern enumeration

Files added on the branch group cleanly under one logical concern: "stand up the Phase 0A inert repository + runtime scaffold with contract evidence." Each file's purpose:

| Group | Files | Why required for this single concern |
|---|---|---|
| Workspace + task runner | `package.json`, `bun.lock`, `turbo.json`, `tsconfig.json`, `.prettierrc`, `eslint.config.js`, `.gitignore` | Define the Bun workspace and the seven local quality commands the ticket requires (`tickets/.../WU-0A-01.md` acceptance criteria). |
| Frontend SPA | `index.html`, `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `vite.config.ts`, `vitest.config.ts` | The React 19 + TanStack Router/Query inert shell required by contract `wu-0a-01-…md:11-17`. |
| Frontend tests | `src/test/setup.ts`, `src/test/scaffold.test.tsx`, `tests-e2e/scaffold.spec.ts`, `playwright.config.ts` | Vitest smoke + Playwright e2e required by ticket criteria 3 and 4 (`bun run test`, `bun run test:e2e`). |
| Tauri shell | `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/build.rs`, `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`, `src-tauri/tauri.conf.json`, `src-tauri/capabilities/default.json`, `src-tauri/icons/icon.png` | The Rust + Tokio + sqlx-ready inert Tauri v2 shell required by contract `wu-0a-01-…md:43-49`. |
| Contract artifacts | `product-strategy/contracts/wu-0a-01-…md`, `product-strategy/contracts/fixtures/wu-0a-01/{package,turbo,tauri.conf}.shape.json`, `src-tauri/tests/scaffold_contract.rs` | The fixtures + Rust contract test that pin scaffold shape (proposal test-intent "Contract fixture shape"). |
| Proposal | `proposals/01-wu-0a-01.md` | The Phase 3 proposal for this WU. |

## Severability check — can any group ship independently?

- **Workspace + task runner alone**: would not satisfy any acceptance criterion (no shell to render, no Tauri to build, no tests to run). Severing it would break the other groups' commands.
- **Frontend without Tauri**: ticket acceptance 6 (`bun run tauri:dev`) would fail; contract `wu-0a-01-…md:43-48` requires the Tauri bootstrap. Not severable.
- **Tauri without frontend**: `tauri.conf.json:7-11` references `dist/` produced by `vite build`; `bun run build` would fail. Not severable.
- **Contract artifacts without scaffold**: the Rust contract test (`src-tauri/tests/scaffold_contract.rs`) asserts files that only exist once the scaffold lands. Splitting it earlier or later changes nothing — the fixtures and test exist precisely to constrain *this* PR.
- **Proposal**: required by the workflow gate (`~/ai/workflows/implementation-pipeline.md` Phase 3 output) for this WU; not a separate concern.

No group is independently shippable, and no group serves a second WU's acceptance criteria. The branch contains zero files that are unrelated to the scaffold (e.g., no docs PR, no unrelated dependency bumps, no parallel WU drift).

## Cross-WU contamination check

- No code references `WU-0A-02` … `WU-0A-14` or any later phase ticket.
- No `agents` subprocess scaffolding, no provider client crates, no GraphStore migration files — confirming this PR is not silently picking up WU-0A-02+ work.
- `proposals/` contains only `01-wu-0a-01.md`. `product-strategy/contracts/fixtures/` contains only the `wu-0a-01/` directory.

## Verdict

**LOW.** This is a single-concern PR for the WU-0A-01 scaffold. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, and there is no contamination from adjacent WUs. The single-concern handoff note is honored.
