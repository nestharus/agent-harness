# WU-0A-08 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-08's purpose (a real Tauri command that genuinely registers `(workspace_id, topic, channel)` subscriptions on the WU-0A-04 event bus, returns a real non-empty unique `subscription_id`, and exercises every documented error variant via real failure modes — not stubbed `Err` returns)? Are TODOs, hardcoded ids, no-op channel seams, or fall-through arms hiding incomplete work?

## Shortcuts examined

### WU-0A-08-SHORTCUT-F01 — The registered command does real subscription work, not a stub

`subscribe_workspace_events` (`src-tauri/src/commands/subscribe_workspace_events.rs:12-20`) is a `#[tauri::command]` that delegates to `subscribe_workspace_events_with_optional_state`. The latter (`:31-50`) validates state presence, validates `workspace_id`, parses the raw topic string, validates channel presence, then calls `state.event_bus.register_subscription(workspace_id, topic, channel)` — an actual mutation of the bus's `Arc<Mutex<HashMap<String, Subscription>>>` (`src-tauri/src/app_state.rs:53-69`). The return value is the `subscription_id` produced inside `Subscription::new`. There is no `todo!()`, no `unimplemented!()`, no hardcoded `"sub-stub"` early return. Test traffic flows through the same pure helpers used by the runtime command (`tests/subscribe_workspace_events_contract.rs:152-318` calls `subscribe_workspace_events_with_state` / `_with_optional_state`). **Justified.**

### WU-0A-08-SHORTCUT-F02 — `subscription_id` is a real unique non-empty value

`next_subscription_id` (`src-tauri/src/events/subscription.rs:71-79`) reads `SystemTime::now().duration_since(UNIX_EPOCH).as_nanos()` and atomically fetch-adds a process-wide `AtomicU64` counter (`:11`), returning `format!("sub-{nanos}-{counter}")`. The atomic counter alone guarantees uniqueness inside a process even if the system clock is frozen — id construction does not depend on the timestamp being monotonically distinct. The id-shape test (`tests/subscribe_workspace_events_contract.rs:184-220`) calls subscribe twice, asserts both ids start with `sub-`, both are non-empty, and the deduped vector has length 2. The format mirrors WU-0A-06's `evt-{nanos}-{counter}` style declared in the proposal (`proposals/08-wu-0a-08.md:18-20`). **Real, not hardcoded.**

### WU-0A-08-SHORTCUT-F03 — Subscription registry actually stores subscriptions, not a no-op

`EventBusHandle` (`src-tauri/src/app_state.rs:32-35`) carries `subscriptions: Arc<Mutex<HashMap<String, Subscription>>>`. `register_subscription` (`:53-69`) does three real things: (a) calls `self.subscribe()` to obtain a fresh `broadcast::Receiver` (`:59`), (b) constructs a `Subscription` that owns both the channel handle and the receiver (`:60`), and (c) inserts it into the map keyed by id (`:63-67`). `subscription_metadata` (`:71-77`) reads the map under lock. `subscription_count` (`:79-84`) reports the map size. The integration test relies on this storage end-to-end (`tests/subscribe_workspace_events_contract.rs:171-179, 207-209, 300-306`) — every retrieved metadata entry has the expected workspace_id and topic. **Real registry, not a no-op.**

### WU-0A-08-SHORTCUT-F04 — `ChannelUnavailable` is exercised via a missing channel argument

The pure-helper signature accepts `Option<WorkspaceEventChannel>` (`src-tauri/src/commands/subscribe_workspace_events.rs:35`). The `#[tauri::command]` wrapper (`:12-20`) wraps the runtime channel in `Some(channel)` so production never hits the `None` arm, while `subscribe_workspace_events_with_state` accepts `Option<...>` so the contract test can pass `None` (`tests/subscribe_workspace_events_contract.rs:259-267`). The error is reached at `commands/subscribe_workspace_events.rs:45` via `channel.ok_or(SubscribeWorkspaceEventsError::ChannelUnavailable)?` — not a hand-thrown sentinel. The fixture `error-channel-unavailable.json` records the absence with `channel_available: false` and `expected_error: "ChannelUnavailable"`. **Real missing-channel path, exposed as a typed `Option`.**

### WU-0A-08-SHORTCUT-F05 — `AppStateUnavailable` is exercised via missing managed state

The `#[tauri::command]` form binds `state: State<'_, HarnessAppState>` (`commands/subscribe_workspace_events.rs:17`), so the runtime path receives whatever the Tauri builder `manage(...)` provides. The pure helper accepts `Option<&HarnessAppState>` (`:32`) and the test seam `subscribe_workspace_events_with_optional_state` (`:31-50`) returns `Err(AppStateUnavailable)` at `:37` when state is `None`. The contract test (`tests/subscribe_workspace_events_contract.rs:269-281`) reaches the error by passing `None` for state. The fixture `error-app-state-unavailable.json` records `app_state_available: false`. **Real `Option<&HarnessAppState>` discriminator, not a stub branch.**

### WU-0A-08-SHORTCUT-F06 — `EmptyWorkspaceId` and `UnknownTopic` short-circuit before subscription registration

Order in `subscribe_workspace_events_with_optional_state` (`commands/subscribe_workspace_events.rs:31-50`):

1. State presence (`:37`) → `AppStateUnavailable`.
2. `workspace_id.trim().is_empty()` (`:40-42`) → `EmptyWorkspaceId`.
3. `parse_subscribe_workspace_events_topic(topic.as_ref())?` (`:44`) → `UnknownTopic`.
4. `channel.ok_or(...)?` (`:45`) → `ChannelUnavailable`.
5. `state.event_bus.register_subscription(...)` (`:47-49`).

Validation runs first; nothing is inserted into the bus's `subscriptions` map on error. **Early-return validation, not register-then-rollback.**

### WU-0A-08-SHORTCUT-F07 — Inert non-runtime test waits a real observation window

The fixture `inert-non-runtime-topic.json` carries `observation_window_ms: 25`. The test (`tests/subscribe_workspace_events_contract.rs:286-318`) actually awaits `tokio::time::sleep(Duration::from_millis(case.observation_window_ms))` (`:308`) between subscribe and assertion — not an immediate `assert_eq!(0, recorded.len())`. The recording channel (`:83-107`) is a real `Channel::new` with a closure that pushes any `InvokeResponseBody::{Json,Raw}` body it observes; if a producer fired during the sleep the `Vec<Value>` would grow. Window is short (25 ms) but documented in the fixture and serves the goal of catching synchronous post-subscribe emissions. **Real wait, real channel.**

### WU-0A-08-SHORTCUT-F08 — Recording channel does not swallow real serialization

The recording channel (`tests/subscribe_workspace_events_contract.rs:83-107`) calls `Channel::new` from `tauri::ipc` — the same constructor a real frontend would receive — and routes both `InvokeResponseBody::Json` and `InvokeResponseBody::Raw` payloads into the recorded `Vec<Value>`. JSON bodies go through `serde_json::from_str(&raw)` (`:90`), so any malformed JSON would error rather than silently drop. Raw bodies are recorded as a `Value::Array` of byte numbers (`:92-97`). No fake `Channel<IpcEvent<Value>>` test seam exists in the production code path. **Justified.**

### WU-0A-08-SHORTCUT-F09 — No TODOs / unimplemented stubs in shipped code

```
$ rg -n "todo!|unimplemented!|TODO|FIXME|XXX|HACK" src-tauri/src/commands/subscribe_workspace_events.rs src-tauri/src/contracts/subscribe_workspace_events.rs src-tauri/src/events/subscription.rs src-tauri/src/app_state.rs src-tauri/src/lib.rs
```
returns no matches. The four `expect("event-bus subscription registry should not be poisoned")` and `expect("runtime …")` strings in `app_state.rs:65, 74, 82` and `lib.rs:40-46, 68-69` are panic messages on infeasible mutex / I/O failures, not work-deferral markers.

### WU-0A-08-SHORTCUT-F10 — `parse_subscribe_workspace_events_topic` does not swallow `EmptyTopic` silently into `Ok`

`parse_subscribe_workspace_events_topic` (`src-tauri/src/contracts/subscribe_workspace_events.rs:22-26`) maps any `EventTopicError` (both `UnknownTopic` and `EmptyTopic`) to `SubscribeWorkspaceEventsError::UnknownTopic`. This is consistent with the contract: an empty topic string is not in the documented `EventTopic` lowercase taxonomy, and the WU only declares four error variants. The mapping does *not* return `Ok` on empty input. **Justified.**

### WU-0A-08-SHORTCUT-F11 — TS parsers exact-key/exact-type validate; no blanket `as` casts

`parseSubscribeWorkspaceEventsArgs` (`src/contracts/subscribe-workspace-events.ts:18-39`) checks record kind, exact key set (sorted) of `["channel", "topic", "workspace_id"]`, non-empty `workspace_id`, non-empty `channel`, and `topic` membership in `EVENT_TOPICS` via `isEventTopic`. `parseSubscribeWorkspaceEventsResponse` (`:41-47`) requires non-empty string. `parseSubscribeWorkspaceEventsError` (`:49-55`) checks union membership against `SUBSCRIBE_WORKSPACE_EVENTS_ERRORS`. Negative coverage is fixture-driven by `invalid-shapes.json` (8 cases — non-object, empty workspace, unknown topic, empty channel, extra key, empty response, object response, unknown error) at `src/test/subscribe-workspace-events.test.ts:46-67`. No `as` cast bypasses validation. **Justified.**

## Verdict

**LOW.** Every error variant is reached via a real failure mode (missing state, missing channel, empty workspace string, parser rejection of `"not-a-topic"`). The subscription registry is a real `HashMap` behind a `Mutex`, not a no-op. `subscription_id` is a process-unique nanos+atomic-counter combination, not hardcoded. The inert non-runtime test waits a documented window and uses a real recording channel that would catch any stray emit. No TODOs, no swallowing fall-through arms, no fake channel seams in production code.
