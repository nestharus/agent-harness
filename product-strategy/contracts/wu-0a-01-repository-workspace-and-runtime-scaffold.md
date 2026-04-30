# WU-0A-01 Contract: Repository Workspace and Runtime Scaffold

## Ownership

This WU owns repository scaffold shape only. It owns no durable schema object, no GraphStore table, no provider credential flow, no real `agents` subprocess invocation, and no operator-visible feature claim beyond a running shell.

## Workspace Shape

- Package manager: Bun workspace at the repository root.
- Task runner: Turbo.
- Frontend: React 19 SPA under `src/`, built with Vite.
- Backend shell: Tauri v2 under `src-tauri/`.
- Backend runtime: Rust + Tokio.
- Local database readiness: sqlx dependency compiled for SQLite/Tokio readiness, with no migrations.
- Styling: Tailwind v4 CSS-first setup using `@import "tailwindcss"` and `@theme`.
- Routing/query: TanStack Router and TanStack Query providers are present in the shell.
- Shell behavior: one inert root route renders a visible application root and does not require GraphStore tables, provider credentials, or `agents`.

## Local Quality Commands

The following commands are the supported Phase 0A local surface and must exist exactly:

- `bun run dev`
- `bun run build`
- `bun run test`
- `bun run test:e2e`
- `bun run lint`
- `bun run typecheck`
- `bun run tauri:dev`

`bun run lint` and `bun run typecheck` run through Turbo and cover both TypeScript/frontend and Rust/Tauri surfaces. `bun run build` produces frontend artifacts and invokes the Tauri build path.

## Canonical Fixtures

The canonical shape fixtures live under `product-strategy/contracts/fixtures/wu-0a-01/`:

- `package.shape.json`: required root package scripts and pinned dependency versions.
- `turbo.shape.json`: required Turbo task graph shape.
- `tauri.conf.shape.json`: required Tauri v2 app/build/bundle shape.

The contract test asserts the implementation matches these fixtures for the keys owned by this WU.

## Tauri Bootstrap Contract

- `src-tauri/Cargo.toml` declares pinned `tauri`, `tokio`, and `sqlx` dependencies matching the fixture-owned expectations.
- `sqlx` uses SQLite/Tokio readiness features and does not require or declare migration files.
- `src-tauri/src/main.rs` delegates to `agent_harness_lib::run()`.
- `src-tauri/src/lib.rs` exposes an inert `run()` bootstrap and registers no value-slice Tauri commands.
- `src-tauri/migrations/` may exist only as an empty placeholder directory; migration files are not allowed.

## Test Handoff

- Rust contract test: `src-tauri/tests/scaffold_contract.rs`.
- Vitest smoke test: `src/test/scaffold.test.tsx`.
- Playwright e2e test: `tests-e2e/scaffold.spec.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
