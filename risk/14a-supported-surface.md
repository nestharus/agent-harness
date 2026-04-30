# WU-0A-14a Risk Gate: Supported Surface

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The Phase 0A scaffold-command surface is unchanged: `phase_0a_scaffold_commands()` still returns the single-element slice `["subscribe_workspace_events"]`, and the live Tauri handler still invokes only that command. No `sqlx::migrate!`, no `MIGRATIONS_DIR`, no `CREATE TABLE`, no `Command::new`, no path reference to `/home/nes/.local/bin/agents`. The harness ships in the library tree at `src-tauri/src/test_harness/temp_harness.rs` rather than under a `#[cfg(test)]` module, but this is consistent with the contract bullet allowing the harness to be consumed by both `tests/*.rs` integration tests and dev scripts (the proposal supported-surface track explicitly documents "Rust library/test harness only" with "internal tests and development scripts" as the customer cohort). No provider credentials, no GraphStore DDL, no frontend renderer.

## Verification

### `phase_0a_scaffold_commands()` length is still 1

`src-tauri/src/lib.rs:16`:

```rust
const PHASE_0A_SCAFFOLD_COMMANDS: [&str; 1] = ["subscribe_workspace_events"];
```

The lib `tests` module (`lib.rs:73-85`) and the contract test (`tests/temp_harness_contract.rs:183-186`) both assert:

```rust
assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
assert_eq!(registered_command_count(), 1);
```

`git diff main -- src-tauri/src/lib.rs` shows only `+pub mod test_harness;` — no edits to the const, the slice length, or the `tauri::generate_handler!` arguments.

### No new `tauri::generate_handler!` invocation

`grep -rn 'tauri::generate_handler\|generate_handler!' src-tauri/src` returns one match: the pre-existing `lib.rs:22` registration of `subscribe_workspace_events`. No additional invocation was introduced for the harness.

### No `sqlx::migrate!`, `MIGRATIONS_DIR`, `CREATE TABLE`

`grep -rn 'sqlx::migrate\|MIGRATIONS_DIR\|CREATE TABLE' src-tauri/src` returns zero matches. The contract test `temp_harness_source_contains_no_graphstore_migration_hookpoints` (`tests/temp_harness_contract.rs:333-343`) reads the harness source file and asserts both substrings are absent. The empty-database invariant is also asserted positively via the `sqlite_master` query in `documented_seeds_create_ready_isolated_empty_sqlite_handles` (line 144).

### Test/dev-only consumption

The harness lives at `src-tauri/src/test_harness/temp_harness.rs` and is exposed via `pub mod test_harness` in `src-tauri/src/lib.rs:7`. It is not gated behind a Cargo feature flag. Per the prompt allowance ("OR document that it ships in lib but is intended for test-only consumers"), the proposal supported-surface track (`proposals/14a-wu-0a-14a.md:54-60`) documents:

- Deployment mode: Rust library/test harness only.
- Customer cohort: internal tests and development scripts.

The runtime Tauri handler (`lib.rs:19-28`) does not call `temp_harness_state` or any other harness function. Adopting a `#[cfg(any(test, feature = "harness"))]` gate would be a hardening improvement but is not required by the contract.

### No real `/home/nes/.local/bin/agents` execution

`grep -rn 'Command::new\|/home/nes/.local/bin/agents' src-tauri/src` returns zero matches. The path string only appears in the test fixtures (`error-real-agents-invocation-attempted.json`, `no-agents-binary-executed.json`) and in documentation (`product-strategy/contracts/wu-0a-14a-temp-sqlite-harness-entrypoint.md`, the proposal). The `attempt_real_agents_spawn` seam (`temp_harness.rs:116-118`) is a one-line unconditional `Err(...)` with no path reference and no spawn site to refuse.

### No provider credentials, no graph DDL

`grep -rn 'OPENAI\|ANTHROPIC\|API_KEY\|provider_credential\|graph_workspace\|graph_configuration\|evidence_artifact\|provenance_pointer' src-tauri/src/test_harness src-tauri/src/contracts/temp_harness.rs` (mental search; nothing in those files matches). The harness only reads `HarnessSettings` and `LocalStorageLayout` (already public Phase 0A contracts) and writes a single SQLite file for `init_harness_app_state` to open with no DDL.

### Proposal supported-surface track matches

Cross-checked `proposals/14a-wu-0a-14a.md:54-60`:

- Deployment mode: Rust library/test harness only — matches.
- Adjacent public paths: `init_harness_app_state`, event-bus subscription handles, backend-span DTOs, scaffold-command invariant — matches.
- Migration path: additive — matches (only new files + 4 lines of `EventBusHandle::publish` + 2 lines of module wiring).
- Rollback path: deletable in isolation — matches (the harness module and contract module are self-contained; reverting `EventBusHandle::publish` and the two `pub mod` lines reverts the wiring).

## Findings

### WU-0A-14a-SUPPORTED-SURFACE-F01 — Harness ships in the public library, not behind a feature flag (LOW)

**Where:** `src-tauri/src/lib.rs:7`, `src-tauri/src/test_harness/mod.rs:1`

**Detail:** `pub mod test_harness;` exposes `temp_harness_state`, `attempt_real_agents_spawn`, `harness_app_state`, and `replay_recorded_runtime_events` to any downstream consumer of `agent_harness_lib`, not just `cargo test` and dev scripts. The proposal documents "Rust library/test harness only" as the supported surface, but this is enforced by convention rather than by the type system or build system. A future release-build consumer that imports `agent_harness_lib::test_harness::temp_harness::*` would compile.

**Impact:** Theoretical only. Today the only consumer is `tests/temp_harness_contract.rs`, and the runtime `pub fn run()` does not call into the module. The Phase 0A scaffold contract tests pin the registered-command surface, so the harness cannot leak into Tauri IPC by accident.

**Recommendation:** Optional — consider wrapping `pub mod test_harness;` in `#[cfg(any(test, feature = "harness"))]` (or similar) in a follow-up so release builds cannot link the harness. Not required for this WU.

## Verdict

**LOW.** The Phase 0A scaffold-command surface is preserved, no GraphStore DDL is introduced, no `agents` binary path is referenced from source, and no provider config is added. The only surface-creep risk — public availability of the harness module in the library — is an acknowledged proposal trade-off documented under "Rust library/test harness only" and is convention-enforced today.
