# WU-0A-01 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise the WU's underlying purpose (a stable, contract-pinned, inert scaffold that downstream Phase 0A WUs can attach to)? Are TODOs, deferred behaviors, or weak assertions hiding incomplete work?

## Shortcuts examined

### S1 — Single root package instead of multi-app monorepo

`package.json:7` declares `"workspaces": ["."]`, with both frontend and `src-tauri/` colocated at the root. Justified by A2 in `proposals/01-wu-0a-01.md:62-64`: the ticket's code boundary names `src/**/*` and `src-tauri/...` directly. The contract `wu-0a-01-…md:9-13` says "Bun workspace at the repository root", "React 19 SPA under `src/`", "Tauri v2 under `src-tauri/`". A multi-app split would have *violated* the contract, not honored it. **Justified.**

### S2 — Contract test asserts JSON subset, not full lockfile equality

`src-tauri/tests/scaffold_contract.rs:22-40` walks expected-fixture keys and demands they exist with matching values in the actual config; extra keys in the implementation are allowed. This is intentional (proposal tradeoff `proposals/01-wu-0a-01.md:28`).

The subset is still load-bearing:
- Pinned dep versions for `react`, `react-dom`, `@tanstack/*`, `@tauri-apps/*`, `tailwindcss`, `vite`, `vitest`, `turbo`, `typescript`, `@playwright/test`, `@testing-library/react` are asserted via `package.shape.json:15-33`.
- The seven user-facing scripts are asserted exactly (`package.shape.json:6-14`).
- Turbo task graph (`turbo.shape.json:3-49`) and Tauri config skeleton (`tauri.conf.shape.json`) are asserted as values, not just keys.
- Cargo manifest has its own targeted asserts on `tauri==2.10.3`, `tokio==1.52.1`, `sqlx==0.8.6` plus negative `migrate` feature (`scaffold_contract.rs:75-121`).

Lockfile equality would have been brittle (transitive churn) without buying purpose-fit. **Justified.**

### S3 — sqlx is included with no migrations

`src-tauri/Cargo.toml:19` declares `sqlx 0.8.6` with `default-features = false, features = ["sqlite","runtime-tokio","macros"]`, and notably *without* `migrate`. No `src-tauri/migrations/` directory exists (the contract permits the directory to be empty or absent — `contract:49`). Justified by A3 (sqlx-readiness ≠ schema ownership) and enforced negatively by `scaffold_contract.rs:116-121,123-132`. **Justified.**

### S4 — Phase 0A registers zero Tauri commands

The contract permits "Phase 0A-owned inert bootstrap commands" (ticket acceptance criterion 6) but does not require any. The implementation registers none, and `registered_command_count()` returning `0` (`src-tauri/src/lib.rs:9-11`) is the compile-time evidence Phase 0B/0C tests can build on. Conservative shortcut, but inside the latitude allowed by A4. **Justified.**

### S5 — `bundle.targets` is `["deb"]`

`src-tauri/tauri.conf.json:28-30` produces only a Debian package. The contract fixture pins this exact value (`tauri.conf.shape.json:27-30`), so it is a declared decision, not an unstated shortcut. Aligns with the Linux-only verification harness in the proposal supported-surface track. **Declared.**

### S6 — `index.html:9` has no `data-testid` on the static markup

The `app-root` testid is rendered by `src/ShellRoot.tsx:3` after React mounts. The Vitest smoke (`src/test/scaffold.test.tsx:12`) and Playwright e2e (`tests-e2e/scaffold.spec.ts:9`) both find it, so the contract is met dynamically. Not a hidden shortcut.

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, or `unimplemented!()` markers in any new source under `src/`, `src-tauri/src/`, or config files.
- `src-tauri/src/lib.rs:5` uses `.expect("error while running Tauri application")` which is the standard Tauri scaffold pattern; not a deferred error path.
- `setup(|_app| Ok(()))` (`lib.rs:4`) is intentionally empty — proposal anti-scope explicitly excludes setup logic.
- No commented-out code or stub modules anywhere in `src-tauri/src/` or `src/`.

## Verdict

**LOW.** Every shortcut is named in the proposal tradeoffs or assumption register, and each has a counterpart in the contract test that defends purpose-fit (pinned versions, missing `migrate` feature, missing `generate_handler!`, empty `migrations/`, no value-slice commands). No deferred work hidden in TODOs, no stubbed modules.
