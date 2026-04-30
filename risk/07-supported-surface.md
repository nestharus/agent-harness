# WU-0A-07 — Supported-Surface Risk Review

**Gate:** Phase 4 supported surface.
**Severity:** LOW.

## Question

Does the implementation respect the Phase 0A invariant `registered_command_count() == 0`, and does it land only the supported surface promised in `proposals/07-wu-0a-07.md:57-65`: a TS-import surface for `src/contracts/harness-command.ts` and `src/ipc/invoke-command.ts`, a Rust-import surface for `agent_harness_lib::contracts::harness_command`, no Tauri runtime registration, no real `subscribe_workspace_events` producer, no domain side effects from importing the helper, and no `tauri.conf.json` allowlist creep?

## Findings

### WU-0A-07-SURFACE-F01 — `registered_command_count() == 0` invariant still holds

`src-tauri/src/lib.rs:17-19` keeps `pub fn registered_command_count() -> usize { 0 }` unchanged. The unit test `tests::phase_0a_registers_no_value_slice_commands` (`src-tauri/src/lib.rs:21-29`) and the WU-0A-04 contract test `harness_app_state_contract.rs` line `assert_eq!(agent_harness_lib::registered_command_count(), 0);` both still pass under `cargo test`. `git diff main -- src-tauri/src/lib.rs` shows the only change is the single line `pub mod commands;`, which is a module declaration with no behavioral effect — `commands/mod.rs` itself contains no items.

### WU-0A-07-SURFACE-F02 — Tauri `Builder` invoke handler stays empty

`src-tauri/src/lib.rs:11-14` is `tauri::Builder::default().setup(|_app| Ok(())).run(tauri::generate_context!())`. There is no `.invoke_handler(...)` call, no `tauri::generate_handler![...]` macro, and no `app.manage(...)` invocation. `grep -rn 'invoke_handler\|generate_handler' src-tauri` returns only the *negative assertion* in the existing scaffold contract test `src-tauri/tests/scaffold_contract.rs:162` which fails the build if `lib.rs` ever contains `generate_handler![`. That test (`tauri_bootstrap_is_inert_and_command_free`, `:135-165`) still passes — confirming the bootstrap remains inert.

### WU-0A-07-SURFACE-F03 — `src-tauri/src/commands/mod.rs` has no handler functions

`src-tauri/src/commands/mod.rs` is one line: `// Future WUs add Tauri command handlers here. Phase 0A registers zero commands.` No `pub fn`, no `#[tauri::command]`, no exported items. The module exists only so future WUs have a stable home; importing it triggers nothing.

### WU-0A-07-SURFACE-F04 — No entries added to `tauri.conf.json` command allowlist

`git diff main -- src-tauri/tauri.conf.json` returns no output; the file is unchanged. Inspecting the current `src-tauri/tauri.conf.json` confirms there is no command allowlist / capabilities block at all (only `productName`, `version`, `identifier`, `build`, `app.windows`, `app.security`, `bundle`). The ticket does not introduce one. Running `cargo test --manifest-path src-tauri/Cargo.toml` still passes the `workspace_manifests_match_phase_0a_contract_fixtures` assertion (`src-tauri/tests/scaffold_contract.rs:43-62`) which compares the live `tauri.conf.json` to the WU-0A-01 shape fixture.

### WU-0A-07-SURFACE-F05 — No real domain event producer for `subscribe_workspace_events`

There is no `tauri::ipc::Channel`, no `app_handle.emit*`, no event-bus structure, no producer task, and no global registry anywhere in the diff. The fixture-owned ack `{ subscribed: true, topic, channelId }` (`product-strategy/contracts/fixtures/wu-0a-07/subscribe-workspace-events-happy-path.json`) is a JSON file, not a runtime construct. The TS test at `src/test/harness-command.test.ts:155-179` exercises the path through a `resolvingShim` that records calls and returns the fixture — no producer is started, no subscription state is held. The proposal anti-scope line "no `subscribe_workspace_events` event producer or `Channel` lifecycle" (`proposals/07-wu-0a-07.md:50`) is honored.

### WU-0A-07-SURFACE-F06 — Typed invoke helper has no auto-registration side effects

`src/ipc/invoke-command.ts:62` exports `invokeCommand = createInvokeCommand()`, which constructs a closure over `defaultInvokeShim`. The closure is *not* invoked at import time. The default shim itself wraps `tauriInvoke` from `@tauri-apps/api/core` (`:1, 20-23`), which is a function reference; importing the module does not call it. There are no top-level `await`, no `useEffect`, no module side effects. Tests instantiate `createInvokeCommand(shim)` with their own shim and never call the production `invokeCommand` export — confirmed by `expect(calls).toEqual(...)` assertions on the local `calls` arrays, never on a global. Importing `src/contracts/harness-command.ts` likewise has no side effects (only `const`, `type`, `class`, and `function` declarations).

### WU-0A-07-SURFACE-F07 — Proposal supported-surface track matches implementation

Proposal Supported Surface block (`proposals/07-wu-0a-07.md:57-65`):

| Promise | Verified |
| --- | --- |
| Deployment mode: local Phase 0A Tauri scaffold | No deploy/runtime change; `cargo test` and `bun run test` pass. |
| Customer cohort: internal contract consumers and future IPC slices | Public exports are `HARNESS_COMMANDS`, `HarnessCommand` (TS+Rust), `CommandError`, `CommandErrorFailure`, `parse*`, `createInvokeCommand`, `invokeCommand`, `parse_harness_command`, `HarnessCommandError` — all internal contract surface. |
| Public paths: TS imports of `src/contracts/harness-command.ts` and `src/ipc/invoke-command.ts`, plus Rust imports of `agent_harness_lib::contracts::harness_command` | Files exist; Rust path is reachable through the `pub mod harness_command;` declaration in `src-tauri/src/contracts/mod.rs:3` and the existing `pub mod contracts;` in `lib.rs:3`. |
| Adjacent paths in blast radius: WU-0A-02 `HarnessSettings`, WU-0A-05 `EventTopic`, contract module declarations, and `bun run *` / Cargo checks | Re-exports / imports limited to those (`src/contracts/harness-command.ts:1-2`); `src-tauri/src/contracts/mod.rs` and `src-tauri/src/lib.rs` only add module declarations; lint/typecheck/test runs all green. |
| Migration path: additive taxonomy, helper, fixtures, and tests | All new files are additive; the two modified files only add `pub mod` lines. |
| Rollback path: remove WU-0A-07 files and module declarations | Confirmed — undoing the four-line edits to `mod.rs` files plus deleting the new files restores `main`. |
| Observability: Rust contract tests and Vitest tests expose drift | `harness_command_contract.rs` (Rust) and `harness-command.test.ts` (TS) both loop over fixtures. |

### WU-0A-07-SURFACE-F08 — No new dependencies; no Cargo / npm churn

`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock` returns no output. The implementation reuses already-imported `serde`, `serde_json`, `@tauri-apps/api`, and `vitest`. The `cargo_manifest_declares_phase_0a_runtime_dependencies_without_migrations` assertion (`src-tauri/tests/scaffold_contract.rs:65-133`) still passes.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 55/55 |
| `cargo test --manifest-path src-tauri/Cargo.toml` | ✓ all suites; `phase_0a_registers_no_value_slice_commands ok`; `tauri_bootstrap_is_inert_and_command_free ok`; `harness_command_contract.rs` 2/2 |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | ✓ |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | ✓ |

## Verdict

**LOW.** The Phase 0A invariant `registered_command_count() == 0` is preserved both by direct verification of the function body and by the existing `tauri_bootstrap_is_inert_and_command_free` and `phase_0a_registers_no_value_slice_commands` tests, which still pass. The Tauri `Builder` invoke handler stays empty. `src-tauri/src/commands/mod.rs` is a placeholder comment with no functions. No `tauri.conf.json` allowlist entries were added. No event producer or `Channel` lifecycle exists. The typed invoke helper's import is side-effect-free; the production `invokeCommand` export is a closure that does nothing until called and never auto-registers handlers. The proposal's supported-surface track exactly matches the shipped surface, and no Cargo/npm dependencies were churned.
