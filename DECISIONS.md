# DECISIONS.md (DRAFT for agent-harness — pending user approval)

Append-only log of architectural and stack decisions. Each entry: **D#** — date — title — context — decision — consequences — citations.

This draft consolidates the evidence from `~/projects/agent-runner` (Tauri+Rust reference) and `~/projects/server-manager/research/07+08-frontend-stack.md` (frontend SPA reference) per the directive: "Frontend SPA defers entirely to server-manager research; agent-runner only governs Tauri+Rust." Tailwind explicitly excluded. Type-checker = tsgo (server-manager preference, beta).

Source evidence:
- `tmp/scratch/session-stack-research/findings-r5.md` (server-manager frontend research extract)
- `tmp/scratch/session-stack-research/findings-r6.md` (agent-runner application stack extract)
- `tmp/scratch/session-stack-research/findings-r3.md` (recovered .claude2 session decisions; model assignments only)

---

## D1 — Workflow compliance is mandatory

**Context:** Prior implementation passes (Phase 0A + Phase 0B) bundled multiple pipeline phases into single dispatches and used the local Agent (Task) tool instead of the `agents` CLI. The user wiped 38 commits because workflow non-compliance compromised every test/contract relationship.

**Decision:**
1. All Work Unit implementation goes through the `implementation-pipeline-orchestrator` (`claude-opus`) at `~/ai/agents/implementation-pipeline-orchestrator.md`.
2. All sub-dispatches use the `agents` CLI per `~/ai/workflows/agents-cli.md`.
3. Test writer and code writer are separate `agents` invocations (load-bearing rule).
4. Three `process-tree-auditor` runs per WU (Phase 4 join, Phase 6 join, Phase 8 join) are mandatory.
5. Violation policy: Tier-1 rewind (force-push main, autonomous), Tier-2 split, Tier-3 shrink. Recorded in DECISIONS.md.
6. Only two surviving human gates: Phase 2.5 problem-map review, and any sub-agent NEEDS_INPUT carrying a new-value question.

**Consequences:**
- No bundled phase prompts. No local Agent tool dispatches.
- Every WU produces process-tree audit evidence proving 6b/6c separation.
- Pipeline non-compliance triggers automatic rewind, no asking permission first.

**Citations:** `~/ai/workflows/implementation-pipeline.md`, `~/ai/agents/implementation-pipeline-orchestrator.md`, `~/projects/agent-harness/AGENTS.md`.

---

## D2 — Application shell + backend reference: ~/projects/agent-runner

**Context:** v0 proposal claimed "frontend inherits the server-manager reference stack" — a hallucinated reference (server-manager is Python/Discord daemon with no frontend stack). Real reference for the desktop app shell is `~/projects/agent-runner` (Tauri v2 + Rust + SolidJS).

**Decision:** Adopt agent-runner's actual Tauri+Rust substrate verbatim:

| Item | Adopted version (from agent-runner) |
|---|---|
| Tauri (Rust crate) | `2` (locked `2.10.2`), with `devtools` feature; `test` feature for dev-deps |
| Tauri build crate | `2` (locked `2.5.5`) |
| `@tauri-apps/api` | `~2.10.1` |
| `@tauri-apps/cli` | `~2.10.0` |
| Rust edition | `2024` |
| Tokio | `1` (locked `1.49.0`), `sync` feature; mpsc channels for async work |
| **Database driver** | **`rusqlite 0.38` with `bundled` feature** — NOT sqlx |
| **Migrations** | **Hand-rolled `ensure_*_schema` + `ALTER TABLE` in Rust** — NOT sqlx migrate |
| Connection pool | None — single `rusqlite::Connection` per app (pool not justified yet) |
| clap | `4` with `derive` feature |
| serde / serde_json | `1` / `1`, `derive` feature |
| serde_yml | `0.0.12` |
| toml | `1` |
| chrono | `0.4` with `serde` feature |
| uuid | `1` with `v4` feature |
| sha2 | `0.10` |
| regex | `1` |
| dirs | `6` |
| tempfile | `3` |
| libc | `0.2` |
| signal-hook | `0.3` |
| nix | `0.29` with `fs` feature |
| getrandom | `0.2` |

**Consequences:**
- All Phase 0B WUs that reference `sqlx` (every single migration WU) need rewriting to use `rusqlite + bundled`.
- All Phase 0B WUs that reference sqlx's migrate runner need rewriting to use the agent-runner-style hand-rolled `ensure_*_schema` pattern.
- Connection pool absent → no `SqlitePool`; the harness uses `rusqlite::Connection` per app, with mutex/RwLock if cross-thread access is needed.
- Pre-existing v0/r3/r6 prior claims of "sqlx" + "migrate feature" are FABRICATIONS; the agent-runner reality is rusqlite + hand-rolled.

**Citations:**
- `/home/nes/projects/agent-runner/src-tauri/Cargo.toml` lines 4, 11–28
- `/home/nes/projects/agent-runner/src-tauri/Cargo.lock` (Tauri 2.10.2, Tokio 1.49, rusqlite 0.38)
- `/home/nes/projects/agent-runner/src-tauri/src/state/db.rs` (hand-rolled migration runner)
- `/home/nes/projects/agent-runner/README.md` Tech Stack table
- Anti-evidence: `~/projects/agent-harness/product-strategy/proposal.md:19` (v0 hallucination)

---

## D3 — Frontend SPA reference: ~/projects/server-manager research (07+08)

**Context:** Server-manager is the canonical reference for the SPA inside the agent-harness Tauri webview. server-manager itself does not yet have a frontend implementation; its `research/07-frontend-stack.md` and `research/08-frontend-stack.md` are the forward-looking research records, not D#-adopted decisions.

**Decision:** Inherit every converged server-manager item; for items still in CANDIDATE state, agent-harness picks one and documents the choice here, deferring future re-alignment to when/if server-manager makes its own D# adoption.

**Consequences:** Every existing agent-harness ticket that referenced React 19 + Tailwind + ESLint + Prettier + Bun-as-fronted-config + Turbo + TanStack Router + Vitest + Playwright + Lefthook + Changesets + Commitlint must be re-evaluated against this DECISIONS.md before any orchestrator dispatch.

**Citations:** `~/projects/server-manager/research/07-frontend-stack.md`, `~/projects/server-manager/research/08-frontend-stack.md`, `tmp/scratch/session-stack-research/findings-r5.md`.

---

## D4 — Tailwind is NOT in the technical stack

**Context:** v0 proposal listed Tailwind v4 as inherited substrate. server-manager research explicitly rejects Tailwind by user preference and team direction. agent-runner uses Tailwind for its own SPA, but per D3 the SPA stack defers to server-manager.

**Decision:** Tailwind (any version) is excluded from agent-harness. Styling = vanilla CSS + CSS Modules + native CSS features (cascade layers, custom properties, container queries, `:has()`, OKLCH, subgrid).

**Consequences:**
- Every ticket / contract / component fixture that imports Tailwind classes is invalid and must be regenerated.
- Tailwind Variants and tailwind-merge (used by agent-runner) are also out.

**Citations:**
- `~/projects/server-manager/research/07-frontend-stack.md` lines 14–15 ("Tailwind v4, Park UI, Next.js, Remix, and the `ui-designer` reference are rejected as dated")
- `~/projects/server-manager/research/08-frontend-stack.md` lines 14–17 ("Tailwind is rejected for this project by user preference and team direction")
- `~/projects/server-manager/research/08-frontend-stack.md` lines 22–24 ("Styling should be vanilla CSS: CSS Modules, native CSS features, layers, custom-property tokens, and small local utilities rather than Tailwind")

---

## D5 — Type-checker: tsgo (beta), TS 5.x retained as fallback

**Context:** server-manager research prefers tsgo / `@typescript/native-preview`. agent-runner uses stable `tsc --noEmit`. User chose tsgo.

**Decision:** Pin `@typescript/native-preview@7.0.0-dev.x` for type-check (`tsgo --noEmit`). Retain stable TypeScript 5.9.x as runtime/peer where tooling depends on the JS API. CI gate runs tsgo.

**Consequences:**
- tsgo is beta; expect occasional pinning bumps.
- Tooling that imports the TypeScript JS API (Vite plugins, etc.) keeps using stable TS 5.x.

**Citations:** `~/projects/server-manager/research/08-frontend-stack.md` lines 99–101, 327–328.

---

## D6 — Lint + format: Oxlint + Oxfmt (server-manager preference)

**Context:** agent-runner uses Biome. server-manager research prefers Oxlint + Oxfmt + `@nkzw/oxlint-config` (note: `@nkzw/oxlint-config` is React-shaped; if Solid is chosen we override). Per D3, server-manager wins.

**Decision:** Oxlint for linting, Oxfmt for formatting. Both are beta in strict terms; pin versions and expect pinning bumps. ESLint, Prettier, Biome, xo-typescript: NOT used.

**Consequences:**
- Drop Biome from any inherited agent-runner template.
- `@nkzw/oxlint-config` is React-shaped; if final framework is Solid (D7 below), use a Solid-appropriate Oxlint config or write a project-local one.

**Citations:** `~/projects/server-manager/research/08-frontend-stack.md` lines 70–80, 184–187, 327–328.

---

## D7 — Frontend framework: SolidJS 1.9.x (Candidate 1)

**Context:** server-manager research presented two non-converged candidates (Solid 1.9.x SPA in 07, React 19 + React Compiler in 08). agent-runner already runs Solid 1.9.11 in production, matching Candidate 1 and the "safe production posture" line in 08.

**Decision:** Adopt SolidJS `^1.9.11` for the agent-harness SPA. Solid 2.0 (beta) is OUT of v1; revisit when Solid 2 reaches stable. The framework choice cascades into D7a–D7e below.

**Consequences:**
- All v0 / prior-ticket references to React 19 are invalid and must be regenerated.
- Solid ecosystem breadth is smaller than React's; we accept that trade.
- Solid 2 transition is the main 12-month risk; manage by pinning Solid 1.9.x and revisiting when Solid 2 is GA.

**Citations:** `~/projects/agent-runner/package.json` (Solid 1.9.11), `~/projects/server-manager/research/07-frontend-stack.md:30, 121-124`, `~/projects/server-manager/research/08-frontend-stack.md:36-43, 282-284`.

---

## D7a — Router: @solidjs/router

**Decision:** `@solidjs/router` `^0.15.x`. TanStack Solid Router is OUT (server-manager 08 explicitly bans for the Solid path).

**Citations:** `~/projects/server-manager/research/08-frontend-stack.md:18-19, 63-68`.

---

## D7b — Data fetching: TanStack Solid Query 5

**Decision:** `@tanstack/solid-query` `^5.x`. (Confirms D13.) Solid signals/stores cover local UI state; TanStack Store only if a downstream package needs it.

**Citations:** `~/projects/agent-runner/package.json:20`, `~/projects/server-manager/research/08-frontend-stack.md:88-95`.

---

## D7c — Forms: TanStack Solid Form + Standard Schema validation

**Decision:** `@tanstack/solid-form` `^1.29.x`. Validation through Standard Schema (Zod / Valibot adapter as needed). Drop RHF (React-only), JSON Forms / RJSF (alternate-renderer paths not pursued for v1).

**Citations:** `~/projects/server-manager/research/07-frontend-stack.md:48-50, 91-100`, `~/projects/server-manager/research/08-frontend-stack.md:91-94`.

---

## D7d — Component primitives: Ark UI Solid

**Decision:** `@ark-ui/solid` `^5.31.x` (matches agent-runner). Underlying Zag.js machines flow through Ark. Mantine, Chakra, shadcn, Park UI: NOT used.

**Citations:** `~/projects/agent-runner/package.json:16`, `~/projects/server-manager/research/07-frontend-stack.md:42-44`, `~/projects/server-manager/research/08-frontend-stack.md:94-96`.

---

## D7e — State: Solid signals + stores; no Redux / Zustand / Jotai

**Decision:** Solid `createSignal` / `createStore` / `createMemo` / `createResource` for app state. TanStack Solid Query 5 owns server state. No Redux Toolkit, Zustand, Jotai, MobX, etc.

**Citations:** `~/projects/server-manager/research/08-frontend-stack.md:93-95`.

---

## D8 — Build: Vite 7

**Context:** agent-runner uses Vite 7.3.1. server-manager research recommends Vite for both candidates.

**Decision:** Vite `^7.3.1`. Plugin set depends on D7 (`vite-plugin-solid` for Solid path, `@vitejs/plugin-react` + `@rolldown/plugin-babel` + `reactCompilerPreset` for React path).

**Consequences:** WebView build target = `esnext` (matches agent-runner's `vite.config.ts:20`).

**Citations:** `~/projects/agent-runner/vite.config.ts:20`, `~/projects/agent-runner/package.json:35`, `~/projects/server-manager/research/07-frontend-stack.md:42-46`, `~/projects/server-manager/research/08-frontend-stack.md:96-98`.

---

## D9 — Test runner: Vitest 4 + jsdom 28

**Context:** server-manager research did not address test runner. agent-runner uses Vitest 4 + jsdom 28. Per D3 server-manager wins where it has a position; where it doesn't (test runner), agent-harness picks independently. Vitest is the obvious default and matches Vite ergonomics.

**Decision:** Vitest `^4.0.18` + jsdom `^28.1.0`. Run `vitest run` in CI.

**Consequences:** Defer Bun test, Jest, etc.

**Citations:** `~/projects/agent-runner/package.json:11, 32, 37`, `~/projects/agent-runner/vitest.config.ts:7`.

---

## D10 — E2E runner: Playwright 1.58

**Context:** server-manager research did not address E2E. agent-runner uses Playwright 1.58.

**Decision:** Playwright `^1.58.2`. Dev server invocation: `bun run dev` (matches agent-runner's `playwright.config.ts:19`).

**Citations:** `~/projects/agent-runner/package.json:12`, `~/projects/agent-runner/playwright.config.ts:19`.

---

## D11 — Package manager + runtime: Bun 1.2+

**Context:** server-manager research did not address package manager. agent-runner uses Bun (lockfile `bun.lock`, `bunx tauri build`, CI `setup-bun`).

**Decision:** Bun `1.2+`. Lockfile: `bun.lock`. Frontend script invocation via `bunx`.

**Consequences:** No `package-lock.json` / `pnpm-lock.yaml` / `yarn.lock`. v0's "Turbo" claim is dropped (no monorepo runner — agent-runner doesn't use one).

**Citations:** `~/projects/agent-runner/README.md:14, 884`, `~/projects/agent-runner/bun.lock`, `~/projects/agent-runner/.github/workflows/ci.yml`.

---

## D12 — Component primitives: see D7d (Ark UI Solid)

**Status:** Folded into D7d. Kept as cross-reference.

---

## D13 — Data fetching: TanStack Query 5

**Context:** Both server-manager candidates and agent-runner use TanStack Query (Solid or React adapter per D7).

**Decision:** TanStack Query `^5.x` for whichever adapter D7 selects.

**Citations:** `~/projects/agent-runner/package.json:20`, `~/projects/server-manager/research/07+08`.

---

## D14 — Distribution: Linux (.deb) + macOS (.dmg) only — Windows REJECTED

**Context:** agent-runner DECISIONS D-006 explicitly rejects Windows.

**Decision:** Bundle outputs are deb (Linux) and dmg (macOS). Windows is out. No Windows shim, no `#[cfg(unix)]` gates outside what agent-runner already gates.

**Citations:** `~/projects/agent-runner/DECISIONS.md:134, 136`.

---

## D15 — Tauri updater signing: deferred (out of scope for v1)

**Decision:** No updater signing secrets in this project's release pipeline yet, matching agent-runner's current posture. Re-evaluate when/if auto-update is added.

**Citations:** `~/projects/agent-runner/docs/security-audit.md:122`.

---

## D16 — CI: GitHub Actions (PR check + manual release dispatch)

**Decision:** Mirror agent-runner's CI shape: a `ci.yml` PR check (Bun + lint + type-check + Vitest + Rust fmt/clippy/test) and a `release.yml` manual `workflow_dispatch` for builds.

**Citations:** `~/projects/agent-runner/.github/workflows/ci.yml`, `~/projects/agent-runner/.github/workflows/release.yml`.

---

## D17 — NOT in the stack (explicit anti-list, sourced)

The following were claimed by v0 / inherited downstream but are NOT actually approved. Tickets that reference them must be regenerated:

| Item | Why excluded |
|---|---|
| Tailwind (any version) + Tailwind Variants + tailwind-merge | server-manager research explicitly rejects (D4) |
| Turbo / Nx | not used by agent-runner; not in server-manager research |
| sqlx + sqlx migrate | agent-runner uses rusqlite + hand-rolled migrations (D2) |
| ESLint + Prettier + xo-typescript | superseded by Oxlint + Oxfmt (D6) |
| Biome | superseded by Oxlint + Oxfmt (D6) |
| Lefthook + Husky + pre-commit | not used by agent-runner; not in server-manager research |
| Changesets + semantic-release | not used by agent-runner; not in server-manager research |
| Commitlint | not used by agent-runner; not in server-manager research |
| TanStack Router (for Solid path) | server-manager rejects for Solid; for React path TBD |
| Next.js / Remix / vinext / Million.js / Preact / Inferno / Rari / Park UI | server-manager research rejects |
| React Server Components / `<ViewTransition>` | server-manager 08 marks `<ViewTransition>` canary-only out-of-scope |

---

## D18 through D-N (model assignments — recovered from session r3)

Carried over verbatim from `tmp/scratch/session-stack-research/findings-r3.md`:

- **Orchestrator (implementation-pipeline-orchestrator)**: `claude-opus`
- **Proposer / builder / auditor**: `gpt-high` (gpt-5.5 + reasoning=high)
- **Judge / 4-gate reviewer**: `claude-opus`
- **Context-management workhorse (8 tasks)**: MiniMax-M2.7
- **Conflict resolution only**: Claude Opus 4.7
- **Lead substrate-locked**: Sonnet 4.6
- **Reviewer substrate-locked**: GPT-5.5
- **Excluded (verbatim user rejection)**: Gemini ("bad at anything that's not visual"), GLM-5.1 ("too rate limited"), Qwen ("not good at summaries"), Mistral ("too weak"), DeepSeek (lost the comparison to MiniMax-M2.7)

User's verbatim acceptance: "Ok. Now that we have settled on the models you can continue on with tasks."

**Citations:** `tmp/scratch/session-stack-research/findings-r3.md`, turn `391656b4-4ddf-4d0c-9e96-5f526eeb6bd7` (2026-04-30T05:13:14.546Z).

---

## Open items

None. D7 and its cascade (D7a–D7e) are settled. This DECISIONS.md is the canonical stack reference for agent-harness implementation work.

**Mandatory next step:** every existing agent-harness planning artifact (proposal, engineering-roadmap, every Phase 0A–7 AI-roadmap branch, every tickets-phase-N branch) must be re-evaluated against this DECISIONS.md before any orchestrator dispatch. Pre-existing references to React 19, Tailwind v4, sqlx + sqlx migrate, Bun + Turbo, ESLint + Prettier + xo-typescript, Biome, Lefthook, Changesets, Commitlint, TanStack Router, vinext, Next.js, Million.js, Park UI, Mantine, shadcn, etc. are invalid and require regeneration through the proper roadmap → ticket workflow before WUs derived from them can run.
