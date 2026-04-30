# WU-0A-07 Contract: HarnessCommand Taxonomy and Typed Invoke Helper

## Ownership

This WU owns the shared command-name taxonomy and typed frontend invoke helper for three Phase 0A command names. It does not own Tauri command handler registration, Rust command implementations, `Channel` producers, event bus subscriptions, GraphStore, providers, agents, workers, budget accounting, audit persistence, or migrations.

The existing Phase 0A invariant remains unchanged: `registered_command_count() == 0`.

## HarnessCommand

`HarnessCommand` serializes as one of these exact strings:

- `get_harness_settings`
- `subscribe_workspace_events`
- `ping_runtime`

Alternate forms such as camelCase, kebab-case, uppercase, empty strings, and unknown names are rejected.

## Command Shapes

### get_harness_settings

```text
args: {}
response: HarnessSettings
```

The response is the WU-0A-02 `HarnessSettings` DTO. Empty args are preserved as an empty object.

### subscribe_workspace_events

```text
args: {
  topic: EventTopic,
  channelId: string
}

response: {
  subscribed: true,
  topic: EventTopic,
  channelId: string
}
```

The `topic` field reuses WU-0A-05 `EventTopic`. The helper only preserves and validates fixture-controlled registration arguments; no real event producer or Tauri channel is opened in Phase 0A.

### ping_runtime

```text
args: {}
response: {
  ok: true,
  command: "ping_runtime",
  runtime: "agent-harness",
  phase: "0A"
}
```

The ping response is fixture-owned in this WU because no Rust `ping_runtime` command handler is registered.

## TypeScript Helper

```text
invokeCommand<TResponse, TArgs>(
  command: HarnessCommand,
  args: TArgs
) -> Promise<TResponse>
```

The exported helper is typed by a command lookup so each command receives only its documented args and resolves only to its documented response shape. Production uses Tauri v2:

```ts
import { invoke } from "@tauri-apps/api/core";
```

Tests use an injected fixture invoke shim. The shim records `(command, args)` and either resolves with fixture data or rejects with a fixture error. Tests do not require a running Tauri runtime.

## CommandError

`CommandError` serializes as one of these exact strings:

- `UnknownCommand`
- `ArgumentSerializationFailed`
- `InvokeRejected`
- `ResponseDeserializationFailed`

Behavior:

- Undocumented command names reject with `UnknownCommand` before invoking the shim.
- Non-JSON-serializable args, including direct JS circular references and `BigInt`, reject with `ArgumentSerializationFailed` before invoking the shim.
- A rejected invoke shim rejects with `InvokeRejected`.
- A resolved invoke response that does not match the command-specific parser rejects with `ResponseDeserializationFailed`.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-07/`:

- `command-names.json`: all documented command names in canonical order.
- `alternate-command-names.json`: alternate and unknown forms that must be rejected.
- `get-harness-settings-happy-path.json`: empty args and `HarnessSettings` response.
- `subscribe-workspace-events-happy-path.json`: topic/channel args and ack response.
- `ping-runtime-happy-path.json`: empty args and ping response.
- `command-errors.json`: the four documented error strings in canonical order.
- `invalid-ping-runtime-response.json`: response parser rejection fixture.

The argument-non-serializable fixture is a direct JavaScript object in the Vitest suite, not a JSON file, because circular references and `BigInt` cannot be represented in JSON.

## Test Handoff

- Rust contract test: `src-tauri/tests/harness_command_contract.rs`.
- TypeScript contract and invoke helper test: `src/test/harness-command.test.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
