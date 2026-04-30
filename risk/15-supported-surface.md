# WU-0A-15 Risk Gate: Supported-Surface

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The WU adds zero Tauri commands, zero TypeScript files, zero migrations, and zero subprocess execution. `phase_0a_scaffold_commands()` is unchanged at length 1 (`["subscribe_workspace_events"]`). The `/home/nes/.local/bin/agents` literal appears only in (a) the structural rejection seam (`REAL_AGENTS_PATH` constant), (b) test source, (c) fixture JSON, and (d) the proposal/contract documentation. The supported-surface track in the proposal matches: Rust library/test-harness only, additive-only blast radius, internal Rust contract tests as the customer cohort.

## Verification

### `phase_0a_scaffold_commands()` invariant

`src-tauri/src/lib.rs:30-36` defines `phase_0a_scaffold_commands()` and `registered_command_count()`. `git diff main -- src-tauri/src/lib.rs` returns no diff — the scaffold is unchanged. The lib.rs internal test (`src-tauri/src/lib.rs:75-83`) asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`, and `tests/fake_agents_fixture_contract.rs:265-280` re-asserts the same invariant from outside the crate:

```rust
assert_eq!(
    agent_harness_lib::phase_0a_scaffold_commands(),
    ["subscribe_workspace_events"]
);
assert_eq!(agent_harness_lib::registered_command_count(), 1);
```

`cargo test` reports both assertions green.

### NO new Tauri commands

External grep across the WU source files (`fake_agents.rs`, `fake_agents_fixture.rs`) for `#[tauri::command]`, `tauri::generate_handler`, `invoke_handler` returns zero matches. The two `mod.rs` edits add only `pub mod fake_agents;` and `pub mod fake_agents_fixture;` — no command registration.

### NO real `/home/nes/.local/bin/agents` execution

External grep across `src-tauri/src/` for `/home/nes/.local/bin/agents` returns exactly one source-code match: `src-tauri/src/test_harness/fake_agents.rs:14` (`const REAL_AGENTS_PATH: &str = "/home/nes/.local/bin/agents";`), which is the structural rejection seam. The constant is consumed only by `is_real_agents_path` (line 299-308) and the post-install canonical check (line 287-291) — both of which reject, never spawn. The other matches in the repo are confined to:

- `src-tauri/tests/fake_agents_fixture_contract.rs:14` — same constant, used in `assert_ne!` checks.
- `product-strategy/contracts/fixtures/wu-0a-15/**/*.json` — fixture JSON.
- `product-strategy/contracts/wu-0a-15-fake-agents-fixture.md`, `proposals/15-wu-0a-15.md` — documentation.

No execution surface.

### NO TypeScript file changes

`git diff main --stat -- 'src/**' 'apps/**' 'packages/**'` returns empty. No `.ts` / `.tsx` files modified or added.

### NO `Command::new` / `std::process::Command` execution

External grep across `src-tauri/src/test_harness/fake_agents.rs` and `src-tauri/src/contracts/fake_agents_fixture.rs` for `Command::new`, `std::process::`, `process::Command` returns zero matches. The contract test self-asserts the same:

```rust
let source = fs::read_to_string(...fake_agents.rs)?;
assert!(!source.contains("Command::new"));
assert!(!source.contains("std::process::Command"));
```

### NO graph DDL, NO migrations, NO provider credentials

External grep across the WU source for `sqlx::migrate`, `MIGRATIONS_DIR`, `CREATE TABLE`, `CREATE INDEX`, provider/credential/secret tokens returns zero matches in the WU files. `git diff main -- src-tauri/Cargo.toml` is empty — no new dependencies. No `migrations/` directory created.

### Cargo.toml + tauri.conf.json + bun.lock unchanged

- `git diff main -- src-tauri/Cargo.toml` → empty.
- `git diff main -- src-tauri/tauri.conf.json` → empty.
- `git diff main -- bun.lock` → empty.

### Proposal supported-surface track matches

`proposals/15-wu-0a-15.md` Supported Surface section (lines 67-75) declares deployment mode as "Rust library/test harness only", customer cohort as "internal Rust contract tests and future development scripts", blast radius as "additive Rust contract/test-harness modules plus contract documentation and fixtures", and rollback path as removing the proposal/contract/fixtures/Rust modules/test/`mod.rs` declarations. This matches what was actually shipped.

### `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `bun run lint`, `bun run typecheck`, `bun run test` all green

Verification commands run without errors or warnings. `cargo fmt --check` returns silently (no diff). Clippy with `-D warnings` finishes clean. The new contract test suite reports `7 passed; 0 failed`.

## Findings

None above LOW.

## Verdict

**LOW.** The supported-surface envelope is preserved exactly. No commands added, no TypeScript touched, no migrations, no execution, no dependency changes, and the only references to `/home/nes/.local/bin/agents` are the structural rejection seam plus tests and documentation.
