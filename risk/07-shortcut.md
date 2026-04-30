# WU-0A-07 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-07's purpose (a typed bilingual command-name taxonomy, a typed invoke helper that maps documented errors with no runtime side effects, and bilingual fixtures that pin the wire surface)? Are TODOs, stubbed branches, weak shims, fake error throws, or fall-through `_` arms hiding incomplete work?

## Shortcuts examined

### WU-0A-07-SHORTCUT-F01 — Test seam is dependency injection, not a runtime feature flag

The helper exposes `createInvokeCommand(invokeShim?: TauriInvokeShim)` (`src/ipc/invoke-command.ts:25-60`) that closes over an injected shim, defaulting to `defaultInvokeShim` which calls `tauriInvoke` from `@tauri-apps/api/core` (`:1, 20-23, 62`). Production exports `invokeCommand` bound to the default shim (`:62`). Tests instantiate their own `createInvokeCommand(shim)` per test (`src/test/harness-command.test.ts:118, 139-141, 160-162, 185, 198, 217, 233`) — there is no environment variable, no `process.env.NODE_ENV` branch, no `if (testing)` runtime gate, no `vi.mock`. The seam is real DI; nothing about the production binding changes between test and prod. **Justified.**

### WU-0A-07-SHORTCUT-F02 — `ArgumentSerializationFailed` is reached with real non-serializable inputs

`src/test/harness-command.test.ts:194-211` constructs an actual circular reference (`circularArgs.self = circularArgs`) and an actual `BigInt(1)` and feeds them to `invokeCommand`. The validator at `src/ipc/invoke-command.ts:64-123` walks the value: a `WeakSet` traps the circular ref (`:95-97`) and the `bigint`/`function`/`symbol`/`undefined` arms throw `TypeError` (`:83-87`); after the structural walk `JSON.stringify(value)` runs as a final guard (`:66`). Both inputs trigger the helper's `ArgumentSerializationFailed` path before the shim is ever consulted (asserted by `expect(calls).toEqual([])`, `:210`). **Real failure, not a fake throw.**

### WU-0A-07-SHORTCUT-F03 — `InvokeRejected` is reached with a shim that actually rejects

`rejectingShim` (`src/test/harness-command.test.ts:38-43`) records the call then `throw error;` inside an `async` function — the returned promise genuinely rejects. The test at `:213-226` builds the shim with `new Error("tauri unavailable")`, asserts the helper rejects with `CommandError.InvokeRejected`, and asserts the shim was called once with `command: "ping_runtime", args: {}` (`:220-225`). The helper's `try { await invokeShim(...) } catch (error) { throw new CommandErrorFailure(InvokeRejected, ...) }` (`src/ipc/invoke-command.ts:42-49`) wraps the real rejection. **Real rejection, not a stub returning `Err`.**

### WU-0A-07-SHORTCUT-F04 — `ResponseDeserializationFailed` is reached with a response that fails the documented parser

`product-strategy/contracts/fixtures/wu-0a-07/invalid-ping-runtime-response.json` is `{ "ok": false, "command": "pingRuntime", "runtime": "agent-harness" }` — three independent contract violations: missing `phase` key, `command` is camelCase rather than `"ping_runtime"`, and `ok` is `false`. Any one would fail `parsePingRuntimeResponse` (`src/contracts/harness-command.ts:129-149`): the `hasExactKeys` check (`:130`) catches the missing `phase`, and the literal-equality block (`:134-141`) catches the wrong `command` and `ok`. The test at `src/test/harness-command.test.ts:228-245` resolves the shim with this fixture, asserts `ResponseDeserializationFailed`, and asserts the shim *was* called (`expect(calls).toEqual([{ command: "ping_runtime", args: {} }])`, `:239-244`) — confirming the failure happened in the parser after the shim resolved, not earlier. **Genuine parser failure, not a sentinel.**

### WU-0A-07-SHORTCUT-F05 — `UnknownCommand` check happens before the shim is called

`createInvokeCommand` runs `parseHarnessCommand(command)` as the first statement of the returned function (`src/ipc/invoke-command.ts:30`); `parseHarnessCommand` throws `CommandErrorFailure(UnknownCommand)` for any string outside `HARNESS_COMMANDS` (`src/contracts/harness-command.ts:72-80`). The test at `src/test/harness-command.test.ts:181-192` passes `"unknown" as HarnessCommand` and asserts `expect(calls).toEqual([])` — no shim invocation. The same statement order also runs before `assertJsonSerializable` (`src/ipc/invoke-command.ts:33`) and before the shim await (`:43`), so an unknown command short-circuits before either side effect. **Justified.**

### WU-0A-07-SHORTCUT-F06 — TS error union is exhaustive — no `_` / `default` fall-through swallows unexpected errors

`parseHarnessCommandResponse` (`src/contracts/harness-command.ts:151-163`) is a switch over `command: TCommand extends HarnessCommand` with explicit cases for all three commands and *no* `default` arm. TypeScript treats the union as exhausted; if a future variant is added without a parser, the switch fails to compile. The four `CommandError` mappings in `src/ipc/invoke-command.ts:32-58` are direct `try/catch` wrappers — `parseHarnessCommand → UnknownCommand` (via thrown `CommandErrorFailure`), `assertJsonSerializable → ArgumentSerializationFailed`, `await shim → InvokeRejected`, `parseHarnessCommandResponse → ResponseDeserializationFailed`. There is no catch-all that re-labels arbitrary errors. Unhandled `CommandErrorFailure` thrown from `parseHarnessCommand` propagates with its original `kind: UnknownCommand` (caught by the test's `.toMatchObject({ kind: UnknownCommand })`, `harness-command.test.ts:46-50`). **No swallowing.**

### WU-0A-07-SHORTCUT-F07 — Rust `HarnessCommand` parser rejects unknown variants (round-trip negative test)

`parse_harness_command` (`src-tauri/src/contracts/harness_command.rs:16-23`) returns `Err(HarnessCommandError::UnknownCommand)` for any non-canonical string. `#[serde(rename_all = "snake_case")]` (`:4`) makes serde reject the same alternates. The negative test at `src-tauri/tests/harness_command_contract.rs:71-91` loops over `alternate-command-names.json` (camelCase, kebab-case, uppercase, empty string, the literal `"unknown"`) and asserts both `parse_harness_command(&raw) == Err(UnknownCommand)` *and* `serde_json::from_value::<HarnessCommand>(...) is_err()`. Both paths reject. **Justified.**

### WU-0A-07-SHORTCUT-F08 — `assertJsonSerializable` walks the value and also calls `JSON.stringify`

`src/ipc/invoke-command.ts:64-123` runs a structural walk (`validateJsonValue` → `validateJsonObject`) that catches BigInt / function / symbol / non-finite number / circular ref / non-plain-prototype objects / symbol keys, *then* runs `JSON.stringify(value)` as a final belt-and-braces check (`:66`). The structural walk catches inputs `JSON.stringify` would silently coerce (functions and `undefined` get stripped without error in plain `JSON.stringify`). The `validateJsonObject` rejects non-plain prototypes (`:109-112`), so `Map`/`Set`/`Date`/class instances are rejected at the contract boundary instead of leaking through as their default JSON forms. Symbol keys are explicitly rejected (`:114-116`). **Defense-in-depth, not a stub.**

### WU-0A-07-SHORTCUT-F09 — `CommandErrorFailure` carries `kind`, `command`, and `cause` — no information loss

`src/contracts/harness-command.ts:28-38` extends `Error` with `kind: CommandError`, optional `command: string`, and forwards `cause` via the `Error` constructor's options bag. The helper passes the original error as `cause` for the three wrapping arms (`src/ipc/invoke-command.ts:36-38, 45-48, 53-57`), so the underlying Tauri rejection or parser failure remains inspectable. Tests assert via `toMatchObject({ name: "CommandErrorFailure", kind })` (`harness-command.test.ts:46-50`), so failures still surface a typed envelope rather than a raw string. **Justified.**

### WU-0A-07-SHORTCUT-F10 — Response parsers do exact-key validation, not blanket `as` casts

`parseEmptyCommandArgs` (`src/contracts/harness-command.ts:90-96`), `parseSubscribeWorkspaceEventsArgs` (`:98-111`), `parseSubscribeWorkspaceEventsAck` (`:113-127`), `parsePingRuntimeResponse` (`:129-149`) all validate object kind, exact key set (sorted, including all required keys), per-field types, and (for ping) literal field values. `parseHarnessCommandResponse` (`:151-163`) routes by command to the documented parser, including delegating `get_harness_settings` to the WU-0A-02 `parseHarnessSettings` re-export (`:2, 156-157`). No `as TResponse` cast bypasses validation. **Justified.**

### WU-0A-07-SHORTCUT-F11 — Module placeholder is documented, not a TODO

`src-tauri/src/commands/mod.rs:1` is a single comment: `// Future WUs add Tauri command handlers here. Phase 0A registers zero commands.` There is no `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented!()`, or `todo!()` marker anywhere in the diff (`grep -r 'TODO\|FIXME\|unimplemented\|todo' src/contracts/harness-command.ts src/ipc/invoke-command.ts src/test/harness-command.test.ts src-tauri/src/contracts/harness_command.rs src-tauri/src/commands/mod.rs src-tauri/tests/harness_command_contract.rs` returns nothing). The comment documents the deliberate emptiness required by anti-scope. **Justified.**

## Hidden-incomplete-work check

- No `#[allow(dead_code)]`, no `#[ignore]`, no `it.skip`, no `it.todo`, no `vi.mock`, no `vi.spyOn`. The test seam is pure dependency injection (`createInvokeCommand(shim)`).
- Both `// @ts-expect-error` lines (`harness-command.test.ts:63-64, 99-100`) are compile-time assertions that strings outside the unions are rejected, not silenced lints.
- All four `CommandError` variants have a reachability test (F02–F05).
- `bun run lint`, `bun run typecheck`, `bun run test`, `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features -- -D warnings` are all green.

## Verdict

**LOW.** Every shortcut examined is either a deliberate, contract-documented choice (DI test seam, fixture-owned ping/subscribe responses, snake_case serde) or a real validation/drift detector (exact-key parser, `assertJsonSerializable` structural walk, `// @ts-expect-error` compile-time assertions, exhaustive switch). All four `CommandError` paths are exercised with genuine failure modes — circular ref / `BigInt`, real promise rejection, real parser failure on a fixture with three independent contract violations — not faked sentinels. The Rust parser rejects every alternate form by both the function and serde paths. No TODOs, no skipped tests, no swallowing fall-throughs, no information loss in error wrapping.
