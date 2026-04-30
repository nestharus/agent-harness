# WU-0A-08 Contract: SubscribeWorkspaceEvents IPC Command

## Command

```text
subscribe_workspace_events(
  workspace_id: string,
  topic: EventTopic,
  channel: Channel<IpcEvent<Value>>
) -> Result<subscription_id: string, SubscribeWorkspaceEventsError>
```

The registered Tauri command name is exactly `subscribe_workspace_events`, aligned with `HarnessCommand::SubscribeWorkspaceEvents`.

## Subscription

```text
Subscription {
  subscription_id: string,
  workspace_id: string,
  topic: EventTopic,
  channel: Channel<IpcEvent<Value>>
}
```

`subscription_id` must be non-empty and unique across calls in a process. The event bus must retain subscription metadata keyed by `subscription_id` so tests can verify the requested `workspace_id` and `topic`.

## Channel

The command accepts Tauri v2 `Channel<IpcEvent<Value>>`. Contract tests use a recording channel fixture that captures any sent `IpcEvent<Value>` bodies. This command does not send domain payloads itself.

## Errors

`SubscribeWorkspaceEventsError` serializes as one of these exact strings:

- `EmptyWorkspaceId`
- `UnknownTopic`
- `ChannelUnavailable`
- `AppStateUnavailable`

## Scaffold Command Policy

Phase 0A permits exactly one scaffold command registration:

```json
["subscribe_workspace_events"]
```

No value-slice commands are registered by this WU.

## Inert Topics

Every WU-0A-05 `EventTopic` variant can be subscribed to. Non-runtime topics remain inert in Phase 0A; a successful non-runtime subscription records zero domain payloads during the fixture observation window.

## Fixtures

- `event-topic-happy-paths.json`: one success case for every documented `EventTopic`.
- `error-empty-workspace-id.json`: `EmptyWorkspaceId`.
- `error-unknown-topic.json`: `UnknownTopic`.
- `error-channel-unavailable.json`: `ChannelUnavailable`.
- `error-app-state-unavailable.json`: `AppStateUnavailable`.
- `inert-non-runtime-topic.json`: successful non-runtime subscription with zero payloads in the observation window.
- `subscription-id-shape.json`: non-empty unique id expectations.
- `canonical-args.json`: TypeScript DTO fixture for command args.
- `canonical-response.json`: TypeScript DTO fixture for the string subscription response.
- `subscribe-workspace-events-errors.json`: error taxonomy fixture.
- `invalid-shapes.json`: malformed TypeScript DTO inputs.
