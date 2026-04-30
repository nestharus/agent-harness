# WU-0A-01 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does deployment mode + customer cohort + adjacent-public-paths analysis match the ticket's "no operator-visible feature claim beyond a running shell" constraint? What is the blast radius for unchanged adjacent paths? Migration / rollback / observability story?

## Supported-surface contract (proposal `proposals/01-wu-0a-01.md:39-47`)

- Deployment mode: local developer workstation only (Bun, Vite, Playwright, Cargo, Tauri CLI).
- Customer cohort: internal developers staging later Phase 0A slices.
- Adjacent paths: planning/research files remain read-only; future GraphStore paths represented only by an empty migration directory.
- Blast radius: package/task config affects all local quality commands; no existing runtime surface in this greenfield worktree.
- Migration: none (no durable schema).
- Rollback: delete the added scaffold + contract artifacts.
- Observability: command exit codes, Vitest DOM assertion, Playwright visible-root assertion, Rust contract test, Cargo check/clippy, Tauri startup/build logs.

## Findings

### Operator-visible surface matches "running shell"

- `src/ShellRoot.tsx:1-13` renders a single `<main data-testid="app-root">` with the heading "Agent Harness" and one descriptive paragraph. No buttons, forms, IPC calls, or feature flags.
- One inert route at `/` (`src/router.tsx:9-13`); root and index components are both `ShellRoot`. No second route, no nav, no error boundary that promises retry semantics.
- Tauri window title "Agent Harness" with reasonable defaults (`src-tauri/tauri.conf.json:13-19`). No tray icon, no menus, no global shortcuts.
- Capability set is `["core:default"]` only (`src-tauri/capabilities/default.json:8-10`) — minimum needed to open a window. No `fs`, `shell`, `dialog`, `os`, `process`, or `path` permissions, so the shell cannot reach OS resources beyond rendering.
- No API surface: zero invocations of `@tauri-apps/api` in `src/`. Zero `generate_handler!` in `src-tauri/`.

### Adjacent-path blast radius

- This is a greenfield worktree (proposal A1, `proposals/01-wu-0a-01.md:59-61`). No prior runtime surface to break. The only pre-existing files were planning/research artifacts, `LICENSE`, and `.gitignore` — none of which are functionally altered.
- The `.gitignore` adds standard Node/Tauri/test ignore patterns (`/.gitignore:1-22`); this is additive and does not occlude existing tracked content.
- Local quality commands now exist; absence of prior commands means no regression risk for them.

### Migration / rollback path

- No durable schema, no data migrations, no provider state files. `src-tauri/migrations/` is absent; if added later it must remain empty per the contract test (`src-tauri/tests/scaffold_contract.rs:123-132`). Rollback is `git revert` of the WU's commits; no state reconciliation.

### Observability

- Each acceptance criterion is observable through a contract test or local command:
  - Command success/failure: the seven scripts in `package.json:11-17` route through Turbo and surface non-zero exit on error.
  - Inert shell render: `src/test/scaffold.test.tsx:12-14` (Vitest + jsdom).
  - Inert route serves: `tests-e2e/scaffold.spec.ts:9-11` (Playwright + headless Chromium).
  - Scaffold shape: `src-tauri/tests/scaffold_contract.rs:42-165` covers manifest subset, dependency pinning, and command-free Tauri bootstrap.
- The Phase 0A risk profile is "N/A for existing runtime behavior" (`proposals/01-wu-0a-01.md:51`); no oncall surface, no SLOs to keep.

### Boundary observations (informational)

- INFO: `app.security.csp` is `null` (`src-tauri/tauri.conf.json:23-24`), pinned by the contract fixture (`tauri.conf.shape.json:22-24`). For a Tauri v2 desktop app loading a local `frontendDist`, disabling CSP is the documented default; web-attack surface is bounded by the absent capability set. Not a supported-surface drift, but worth flagging for future WUs that introduce remote content or webview navigation — those will need a non-null CSP.
- INFO: `bundle.targets: ["deb"]` (`tauri.conf.json:28-30`) is the only declared deployment artifact format. Other-platform bundles are out-of-surface for this WU; future workstation-portability WUs can extend.
- INFO: `vite.config.ts:13` exposes only `VITE_` and `TAURI_` env prefixes — provider envs (e.g., `OPENAI_API_KEY`) cannot leak into the bundle from `.env` accidentally.

## Verdict

**LOW.** Operator-visible surface is exactly one visible application root, observable by Vitest + Playwright + Tauri startup. No unintended public surface (no IPC commands, no FS permissions, no second route, no provider envs). Migration/rollback/observability are appropriate for a greenfield, schema-free, credential-free scaffold.
