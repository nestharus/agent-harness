# WU-0A-01 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-01 scaffold boundary, or does it bleed into Phase 0B (GraphStore migrations, schema), Phase 0C (provider configs, real `agents`), or value-slice features?

## Findings

### In-scope, confirmed

- Workspace is a single Bun root package matching the ticket's `src/` + `src-tauri/` code boundary (`package.json:7`, A2 in proposal `proposals/01-wu-0a-01.md:62-64`).
- The seven required local-quality commands exist exactly and only at `package.json:11-17` (`dev`, `build`, `test`, `test:e2e`, `lint`, `typecheck`, `tauri:dev`); supplementary `frontend:*`/`tauri:*`/`test:unit` scripts at `package.json:18-27` are private Turbo task entrypoints, not new operator surface.
- Tauri shell is inert: `src-tauri/src/lib.rs:2-7` calls `tauri::Builder::default()` with an empty `setup` closure and zero `generate_handler!` invocations. `registered_command_count()` returns `0` (`src-tauri/src/lib.rs:9-11`) and the contract test enforces the absence of `generate_handler![` (`src-tauri/tests/scaffold_contract.rs:161-164`).
- No GraphStore migrations: `src-tauri/migrations/` directory is not present at all (allowed — contract `wu-0a-01-…md:49` says it "may exist only as an empty placeholder"). The Rust contract test asserts emptiness if it ever appears (`scaffold_contract.rs:123-132`).
- sqlx is declared with `sqlite, runtime-tokio, macros` features and explicitly without `migrate` (`src-tauri/Cargo.toml:19`; enforced negatively at `scaffold_contract.rs:116-121`). This matches A3 ("sqlx-ready means dependency readiness, not schema ownership").
- No provider credentials, OAuth flow, or API-key scaffolding: no env wiring, no `dotenv`, no provider crates in `Cargo.toml`. `vite.config.ts:13` whitelists only `VITE_` and `TAURI_` prefixes — no provider envs.
- No `agents` subprocess invocation: no `Command`, `process`, or `spawn` calls in `src-tauri/src/`. `lib.rs` is 21 lines including a self-test.
- Frontend renders one inert root route (`src/router.tsx:5-15`) and one shell component with no fetch/IPC/`invoke` (`src/ShellRoot.tsx:1-13`). `src/App.tsx:7-16` wires only the TanStack Router/Query providers required by the contract.

### Adjacent-path additions (in-scope)

- `proposals/01-wu-0a-01.md` and `product-strategy/contracts/wu-0a-01-…md` plus `…/fixtures/wu-0a-01/*.shape.json` are explicitly the WU's own contract artifacts.
- `src-tauri/capabilities/default.json:8-10` registers only `core:default` for the `main` window — the minimum Tauri v2 needs to open a window. No filesystem/shell/dialog/IPC permissions.
- `LICENSE` was already present pre-branch; not introduced by this WU.

### Potential concerns — none rising to MEDIUM

- INFO: `bundle.targets` is `["deb"]` (`src-tauri/tauri.conf.json:28-30`). Linux-only bundle output is consistent with the "local developer workstation" supported surface and the verification harness, but it is a soft scope choice — a future Phase 0A WU may need to add other targets. Not a scope violation.
- INFO: `src-tauri/icons/icon.png` is a single PNG; Tauri build accepted it for `.deb`. Not a scope violation.

## Verdict

**LOW.** The implementation is a faithful inert scaffold. No GraphStore migration files, no provider credential paths, no `agents` invocation, no value-slice command registration, and no operator-visible feature claim beyond the visible application root. Anti-Scope from `proposals/01-wu-0a-01.md:31-37` is honored end-to-end and is enforced negatively by the contract test.
