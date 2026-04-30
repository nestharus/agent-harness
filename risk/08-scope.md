# WU-0A-08 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-08 boundary (the `subscribe_workspace_events(workspace_id, topic, channel)` Tauri IPC command, its four-variant `SubscribeWorkspaceEventsError` taxonomy, the workspace/topic subscription association on the WU-0A-04 event bus, the inert non-runtime topic invariant, and the scaffold-command allowlist update) or does it leak into Phase 0B+ work the proposal anti-scope reserves (`proposals/08-wu-0a-08.md:39-41`)?

## Findings

### WU-0A-08-SCOPE-F01 — Code/test boundary matches the ticket

Ticket (`plans/tickets/phase-0a/WU-0A-08.md`) declares the code boundary as `src-tauri/src/commands/subscribe_workspace_events.rs`, `src-tauri/src/events/subscription.rs`, `src-tauri/src/contracts/subscribe_workspace_events.rs`, `src/contracts/subscribe-workspace-events.ts`, `src-tauri/tests/subscribe_workspace_events_contract.rs`, `src/test/subscribe-workspace-events.test.ts`, and the test boundary as `product-strategy/contracts/wu-0a-08-subscribe-workspace-events.md` plus `product-strategy/contracts/fixtures/wu-0a-08/*.json`. Every one of those files is present (`git status --porcelain`), and the only modified files are the unavoidable scaffold-contract and module-wiring updates (`src-tauri/src/{lib.rs,app_state.rs,commands/mod.rs,contracts/mod.rs,events/mod.rs}` plus the two scaffold tests).

### WU-0A-08-SCOPE-F02 — `subscribe_workspace_events` is registered as the only Phase 0A command

`src-tauri/src/lib.rs:21-23` registers exactly one handler:

```rust
.invoke_handler(tauri::generate_handler![
    commands::subscribe_workspace_events::subscribe_workspace_events
])
```

The Tauri attribute on the handler (`src-tauri/src/commands/subscribe_workspace_events.rs:12`) plus the snake_case command symbol pin the registered name to `subscribe_workspace_events` — aligned with `HarnessCommand::SubscribeWorkspaceEvents` (`src-tauri/src/contracts/harness_command.rs`) and the contract's wire form (`product-strategy/contracts/wu-0a-08-subscribe-workspace-events.md:13`). `phase_0a_scaffold_commands()` returns exactly `["subscribe_workspace_events"]` (`src-tauri/src/lib.rs:15, 29-31`).

### WU-0A-08-SCOPE-F03 — Every documented `EventTopic` variant is exercised happy-path

`product-strategy/contracts/fixtures/wu-0a-08/event-topic-happy-paths.json` enumerates all 10 variants from the WU-0A-05 taxonomy (`src-tauri/src/contracts/event_topic.rs:5-16`): `graph`, `render`, `provider`, `optimizer`, `worker`, `question`, `recovery`, `budget`, `audit`, `runtime`. The Rust integration test at `src-tauri/tests/subscribe_workspace_events_contract.rs:152-182` loops over the fixture, calls `subscribe_workspace_events_with_state` per case, and asserts both `Ok(subscription_id)` and that `state.event_bus.subscription_metadata(&subscription_id)` round-trips the requested `workspace_id` and `topic`. **Full coverage.**

### WU-0A-08-SCOPE-F04 — `subscription_id` is non-empty and recorded against (workspace_id, topic)

`subscribe_workspace_events_with_state` (`src-tauri/src/commands/subscribe_workspace_events.rs:47-49`) returns `state.event_bus.register_subscription(...)`. `EventBusHandle::register_subscription` (`src-tauri/src/app_state.rs:53-69`) constructs a `Subscription` via `Subscription::new` (`src-tauri/src/events/subscription.rs:31-44`), pulling `subscription_id` from `next_subscription_id` (`:71-79` — `format!("sub-{nanos}-{counter}")` with a process-wide `AtomicU64` counter), then inserts the subscription keyed by id into `Arc<Mutex<HashMap<...>>>` on the bus (`app_state.rs:34, 41-42, 63-67`). `subscription_metadata` (`app_state.rs:71-77`) returns the `(subscription_id, workspace_id, topic)` triple. The id-shape test (`subscribe_workspace_events_contract.rs:184-220`) asserts non-empty, `sub-` prefix, uniqueness across `call_count` calls, and metadata association.

### WU-0A-08-SCOPE-F05 — All four `SubscribeWorkspaceEventsError` variants are implemented and tested

`SubscribeWorkspaceEventsError` (`src-tauri/src/contracts/subscribe_workspace_events.rs:14-20`) has exactly the four documented variants. Reachability (`src-tauri/tests/subscribe_workspace_events_contract.rs:222-284`) consumes per-error fixtures:

| Variant | Reached by |
| --- | --- |
| `EmptyWorkspaceId` | `error-empty-workspace-id.json` (`workspace_id == ""`) → `commands/subscribe_workspace_events.rs:40-42`. |
| `UnknownTopic` | `error-unknown-topic.json` (`"not-a-topic"`) → `parse_subscribe_workspace_events_topic` maps `EventTopicError` to `UnknownTopic` (`contracts/subscribe_workspace_events.rs:22-26`). |
| `ChannelUnavailable` | Test passes `Some(state)` and `None` channel (`tests/...:259-267`) → `commands/...:45`. |
| `AppStateUnavailable` | Test passes `None` state via `subscribe_workspace_events_with_optional_state` (`tests/...:269-281`) → `commands/...:37`. |

The TypeScript mirror (`src/contracts/subscribe-workspace-events.ts:9-14`) and fixture (`subscribe-workspace-events-errors.json`) carry the same four-string union, validated by `src/test/subscribe-workspace-events.test.ts:36, 41-43`.

### WU-0A-08-SCOPE-F06 — Inert non-runtime topic test asserts zero domain payloads in the documented window

The fixture `inert-non-runtime-topic.json` selects `topic: "graph"` with `observation_window_ms: 25` and `expected_domain_payload_count: 0`. The test at `src-tauri/tests/subscribe_workspace_events_contract.rs:286-318` (a) subscribes against a real `EventBusHandle`, (b) confirms the subscription is registered, (c) sleeps `Duration::from_millis(25)` via `tokio::time::sleep`, and (d) asserts `recorded.lock().len() == 0`. The recording channel (`tests/...:83-107`) uses a real `Channel::new` with a closure that pushes any `InvokeResponseBody::Json` / `Raw` body it receives — so a stray emit on the broadcast bus would be observable. **Real observation window, real channel, no producer fires.**

### WU-0A-08-SCOPE-F07 — Scaffold-contract assertion updates are coherent

`phase_0a_scaffold_commands()` (`src-tauri/src/lib.rs:15, 29-31`) is the single source of truth for the allowlist. The two prior scaffold-contract assertions are updated to match:

- `src-tauri/tests/scaffold_contract.rs:136-184` (`tauri_bootstrap_is_inert_and_command_free`) now asserts `phase_0a_scaffold_commands() == ["subscribe_workspace_events"]`, `lib.rs` contains `tauri::generate_handler![` and the qualified `commands::subscribe_workspace_events::subscribe_workspace_events` symbol, and rejects `commands::get_harness_settings` and `commands::ping_runtime` value-slice command paths. The previous `!lib_content.contains("generate_handler![")` (zero-command rule) is replaced with `!lib_content.contains("generate_handler![]")` (no-empty-handler rule), preserving the no-empty-registration intent.
- `src-tauri/tests/harness_app_state_contract.rs:139-146` updates the count assertion to compare `registered_command_count() == phase_0a_scaffold_commands().len()` and asserts the list equals `["subscribe_workspace_events"]`. Both anchor on the same allowlist.

The `lib.rs` unit test (`src-tauri/src/lib.rs:73-83`) duplicates the same allowlist assertion. All three sites move in lock-step against `phase_0a_scaffold_commands()`.

### WU-0A-08-SCOPE-F08 — Anti-scope honored

Anti-scope (`proposals/08-wu-0a-08.md:39-41`): no GraphStore queries/tables, no migrations, no provider credentials, no `agents` invocation, no optimizer cycles, no graph mutation event producers, no runtime backend-span producers. Verified:

- No new SQL or schema in the diff (`src-tauri/src/app_state.rs` adds the in-memory subscription registry but the `db: SqlitePool` is unchanged from WU-0A-04).
- `src-tauri/src/commands/subscribe_workspace_events.rs` only validates inputs, parses the topic, and calls `register_subscription` — no `state.event_bus.sender.send(...)`, no producer wiring, no `agents` invocation.
- `Subscription` (`events/subscription.rs:22-28`) holds the broadcast receiver but never reads from it. No actor or task is spawned to pump the receiver into the channel.
- No edits to `Cargo.toml`, `tauri.conf.json`, `package.json`, or `bun.lock` (`git diff main --stat`).

### Verification commands

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ (eslint + cargo clippy via turbo, FULL TURBO cache hit) |
| `bun run typecheck` | ✓ (tsc + cargo check via turbo) |
| `bun run test` | ✓ 57/57 vitest tests; `src/test/subscribe-workspace-events.test.ts` 2/2 |
| `cargo test --manifest-path src-tauri/Cargo.toml` | ✓ all suites pass; `tests/subscribe_workspace_events_contract.rs` 5/5; `tests/scaffold_contract.rs` 3/3; `tests/harness_app_state_contract.rs` 7/7 |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | ✓ no diff |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | ✓ no warnings |

## Verdict

**LOW.** The implementation lands exactly the WU-0A-08 contract surface — one Tauri command registered as `subscribe_workspace_events`, every `EventTopic` variant covered happy-path, all four `SubscribeWorkspaceEventsError` variants reachable, real `(workspace_id, topic)` association on the event bus, and a documented inert non-runtime observation window. Scaffold-contract assertion updates anchor on a single allowlist and continue to reject value-slice commands (`get_harness_settings`, `ping_runtime`). No anti-scope leakage.
