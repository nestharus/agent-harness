# WU-0A-07 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-07 boundary (the `HarnessCommand` command-name taxonomy with exactly three variants, the typed `invokeCommand<TResponse, TArgs>` helper, the four-variant `CommandError` taxonomy, and bilingual fixtures) or does it bleed into Tauri command registration, real `subscribe_workspace_events` producers / `Channel` lifecycles, real `ping_runtime` Rust handlers, real `get_harness_settings` wiring, GraphStore, providers, agents, optimizers, workers, questions, recovery, budget, audit, or any later-phase work the proposal anti-scope reserves (`proposals/07-wu-0a-07.md:47-55`)?

## Findings

### WU-0A-07-SCOPE-F01 — `HarnessCommand` has exactly three documented variants in both languages

`HarnessCommand` (`src-tauri/src/contracts/harness_command.rs:3-9`) declares exactly the three documented variants `GetHarnessSettings`, `SubscribeWorkspaceEvents`, `PingRuntime`, with `#[serde(rename_all = "snake_case")]` to pin the wire form to the contract strings (`product-strategy/contracts/wu-0a-07-harness-command.md:13-15`). The TypeScript mirror `HARNESS_COMMANDS` (`src/contracts/harness-command.ts:4-10`) is a `const` tuple of the same three strings deriving the union type. Both are loop-bound to `command-names.json` in `src-tauri/tests/harness_command_contract.rs:38-69` and `src/test/harness-command.test.ts:53-74`.

### WU-0A-07-SCOPE-F02 — `invokeCommand<TResponse, TArgs>` signature matches the WU contract field

The exported helper (`src/ipc/invoke-command.ts:15-18, 25-60, 62`) is typed as `<TCommand extends HarnessCommand>(command: TCommand, args: HarnessCommandArgsByCommand[TCommand]) => Promise<HarnessCommandResponseByCommand[TCommand]>`. This is the contract surface "TypeScript helper" block at `product-strategy/contracts/wu-0a-07-harness-command.md:62-68`, narrowed by per-command arg/response lookups from `src/contracts/harness-command.ts:60-70`. Production binds the default shim to Tauri v2 `invoke` (`src/ipc/invoke-command.ts:1, 20-23, 62`); tests inject a fixture shim through `createInvokeCommand(invokeShim)` (`src/test/harness-command.test.ts:24, 31-43, 118, 139-141, 160-162, 185, 198, 217, 233`). The contract Test Handoff line "Tests use an injected fixture invoke shim" (`product-strategy/contracts/wu-0a-07-harness-command.md:74-76`) is honored.

### WU-0A-07-SCOPE-F03 — All four `CommandError` variants implemented and bilaterally pinned

`CommandError` is a TypeScript `const` object plus `COMMAND_ERRORS` tuple deriving the union type (`src/contracts/harness-command.ts:12-26`) with the four documented strings in canonical order: `UnknownCommand`, `ArgumentSerializationFailed`, `InvokeRejected`, `ResponseDeserializationFailed`. The fixture `command-errors.json` carries the same canonical order, asserted in `src/test/harness-command.test.ts:89-111`. `parseCommandError` (`src/contracts/harness-command.ts:82-88`) rejects strings outside that union. The proposal anti-scope explicitly notes Rust does not own this runtime error taxonomy in Phase 0A (`proposals/07-wu-0a-07.md:45`); accordingly, Rust only carries `HarnessCommandError::UnknownCommand` for the parser surface (`src-tauri/src/contracts/harness_command.rs:11-14`). All four TS variants are reached by tests:
- `UnknownCommand`: `harness-command.test.ts:181-192` (passing `"unknown"` to `invokeCommand`) and `:76-87` (parser-side via `parseHarnessCommand` over `alternateCommandNames`).
- `ArgumentSerializationFailed`: `:194-211` (real circular reference and real `BigInt`).
- `InvokeRejected`: `:213-226` (real `rejectingShim` throwing `new Error("tauri unavailable")`).
- `ResponseDeserializationFailed`: `:228-245` (resolving shim returns `invalid-ping-runtime-response.json`).

### WU-0A-07-SCOPE-F04 — All eight acceptance criteria are exercised by tests

| AC | Verified by |
| --- | --- |
| Every `HarnessCommand` variant round-trips through Rust+TS fixtures | `src-tauri/tests/harness_command_contract.rs:34-69` (Rust serde round-trip + `parse_harness_command` parity) and `src/test/harness-command.test.ts:53-74` (TS union/fixture/parser parity). |
| `invokeCommand("ping_runtime", {})` resolves with the documented ping response | `src/test/harness-command.test.ts:113-132` against `ping-runtime-happy-path.json`. |
| `invokeCommand("get_harness_settings", {})` preserves command/empty args and parses `HarnessSettings` | `src/test/harness-command.test.ts:134-153` against `get-harness-settings-happy-path.json`; response parsed via `parseHarnessSettings` re-export from WU-0A-02 (`src/contracts/harness-command.ts:2, 156-157`). |
| `invokeCommand("subscribe_workspace_events", args)` preserves topic/channelId without a producer | `src/test/harness-command.test.ts:155-179` against `subscribe-workspace-events-happy-path.json`; only the fixture shim records the call — no producer is opened. |
| `invokeCommand("unknown", args)` rejects with `UnknownCommand` | `src/test/harness-command.test.ts:181-192`. |
| Non-serializable args reject with `ArgumentSerializationFailed` | `src/test/harness-command.test.ts:194-211` — circular ref + `BigInt`. |
| Rejected Tauri invoke returns `InvokeRejected` | `src/test/harness-command.test.ts:213-226`. |
| Mismatched response returns `ResponseDeserializationFailed` | `src/test/harness-command.test.ts:228-245`. |

### WU-0A-07-SCOPE-F05 — Anti-scope honored

Anti-scope (`proposals/07-wu-0a-07.md:47-55`) bars: real Tauri command registration at the runtime invoke handler, real `subscribe_workspace_events` event producer or `Channel` lifecycle, real `ping_runtime` Rust handler, real `get_harness_settings` handler wiring, GraphStore tables, migrations, provider credentials, agent invocation, and domain event producers. Verified:

- No `tauri::generate_handler!`, no `.invoke_handler(...)`, no `app.manage(...)`. `src-tauri/src/lib.rs` still calls `tauri::Builder::default().setup(|_app| Ok(())).run(...)` (`src-tauri/src/lib.rs:9-15`); the only diff against `main` is `pub mod commands;` (`git diff main -- src-tauri/src/lib.rs`).
- `src-tauri/src/commands/mod.rs` is a single comment line, no handler functions: `// Future WUs add Tauri command handlers here. Phase 0A registers zero commands.` (`src-tauri/src/commands/mod.rs:1`).
- No event producer or Tauri `Channel`: `subscribe_workspace_events` is exercised exclusively through the fixture shim (`src/test/harness-command.test.ts:155-179`); no `tauri::ipc::Channel`, `app_handle.emit`, or event-bus reference anywhere in the diff.
- No Rust `ping_runtime` or `get_harness_settings` handlers: the response parsers in `src/contracts/harness-command.ts:113-149` validate fixture-owned shapes; no `#[tauri::command]` exists in the diff.
- No GraphStore, providers, agents, optimizers, workers, questions, recovery, budget, audit, or migration changes.
- The TypeScript `parseHarnessCommandResponse` switch (`src/contracts/harness-command.ts:151-163`) is exhaustive over `HarnessCommand` with explicit per-command arms; no fall-through routes a real production response through a no-op parser.

### WU-0A-07-SCOPE-F06 — No Phase 0B/0C/1+ behavior added

- No real workspace event producer, no `Channel` constructor invocations, no IPC subscription wiring; `subscribe_workspace_events` returns a fixture-owned ack object (`product-strategy/contracts/fixtures/wu-0a-07/subscribe-workspace-events-happy-path.json`) only when the shim resolves with that fixture.
- No real `ping_runtime` health implementation; the response is a fixture-owned `{ ok: true, command: "ping_runtime", runtime: "agent-harness", phase: "0A" }` shape (`product-strategy/contracts/fixtures/wu-0a-07/ping-runtime-happy-path.json`).
- No additional command-allowlist entries in `tauri.conf.json` (`git diff main -- src-tauri/tauri.conf.json` is empty).
- No new Cargo or npm dependencies (`git diff main -- src-tauri/Cargo.toml package.json bun.lock src-tauri/Cargo.lock` is empty).

### Adjacent-path additions (in-scope)

- `src-tauri/src/contracts/mod.rs:3` — single-line `pub mod harness_command;` next to existing module declarations.
- `src-tauri/src/lib.rs:2` — single-line `pub mod commands;` declaring the new (empty) commands module.
- `src-tauri/src/commands/mod.rs` — single-line placeholder comment, no handler functions.

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ (eslint + cargo clippy via turbo, FULL TURBO cache hit) |
| `bun run typecheck` | ✓ (tsc + cargo check via turbo) |
| `bun run test` | ✓ 55/55 vitest tests pass; `src/test/harness-command.test.ts` 10/10 |
| `cargo test --manifest-path src-tauri/Cargo.toml` | ✓ all suites pass; `tests/harness_command_contract.rs` 2/2; `tests::phase_0a_registers_no_value_slice_commands` ok |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | ✓ no diff |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** The implementation lands the documented `HarnessCommand` taxonomy with exactly the three required variants, the typed `invokeCommand` helper with the contract signature, and all four `CommandError` variants — every one of them reached by tests with real (not faked) failure modes. All eight acceptance criteria are exercised. Anti-scope is honored end-to-end: no Tauri command handlers registered, no event producer or `Channel`, no Rust `ping_runtime` / `get_harness_settings` handler, no GraphStore/providers/agents/etc., no `tauri.conf.json` allowlist edits, no new dependencies. The only adjacent edits are unavoidable single-line module declarations.
