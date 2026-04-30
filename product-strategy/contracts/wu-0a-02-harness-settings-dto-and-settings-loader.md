# WU-0A-02 Contract: HarnessSettings DTO and Settings Loader

## Ownership

This WU owns the Phase 0A settings DTO, log-level union, settings-loader merge behavior, and settings-loader error taxonomy. It does not own storage layout derivation, database opening, process spawning, provider credentials, app-state construction, or IPC command registration.

## DTO

`HarnessSettings` serializes as a JSON object with exactly these fields:

- `workspace_id`: string
- `storage_root`: string
- `database_path`: string
- `agent_runner_bin`: string
- `log_level`: `"trace" | "debug" | "info" | "warn" | "error"`
- `profile_name`: optional string

Unknown DTO fields are rejected by the Rust contract type and the TypeScript parser.

## Loader

```text
load_harness_settings(
  cwd: string,
  env_overrides: map<string, string>,
  config_file?: string
) -> Result<HarnessSettings, SettingsError>
```

Resolution order:

1. Start with empty settings.
2. If `config_file` is provided, resolve it relative to `cwd` unless it is absolute, read it as JSON, and deserialize the object fields listed above.
3. Overlay recognized env override keys:
   - `HARNESS_WORKSPACE_ID` -> `workspace_id`
   - `HARNESS_STORAGE_ROOT` -> `storage_root`
   - `HARNESS_DATABASE_PATH` -> `database_path`
   - `HARNESS_AGENT_RUNNER_BIN` -> `agent_runner_bin`
   - `HARNESS_LOG_LEVEL` -> `log_level`
   - `HARNESS_PROFILE_NAME` -> `profile_name`
4. Ignore unrecognized env override keys.
5. Validate required values.

`agent_runner_bin` remains serialized as the supplied string, but the loader checks that it resolves to an existing file under `cwd` when relative. Empty or unresolved paths return `MissingAgentRunnerBin`.

## Errors

`SettingsError` serializes as one of these exact strings:

- `EmptyWorkspaceId`
- `EmptyStorageRoot`
- `EmptyDatabasePath`
- `MissingAgentRunnerBin`
- `InvalidLogLevel`
- `ConfigFileUnreadable`

Unknown error strings are rejected by Rust serde and by the TypeScript parser.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-02/`:

- `canonical-settings.json`: canonical DTO shape with optional `profile_name`.
- `config-input.json`: config-file source for loader success.
- `env-overrides.json`: env overlay source for loader success.
- `env-overridden-settings.json`: expected loader success result after merge.
- `settings-errors.json`: all documented error variants.
- `invalid-settings-error.json`: unknown error string rejection fixture.
- `error-*.json`: input cases that must reach each `SettingsError` variant.
- `invalid-config.json`: malformed JSON for `ConfigFileUnreadable`.

## Test Handoff

- Rust contract test: `src-tauri/tests/harness_settings_contract.rs`.
- TypeScript contract test: `src/test/harness-settings.test.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
