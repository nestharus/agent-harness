# WU-0A-02 — Shortcut Risk Review

**Gate:** Phase 4 shortcut.
**Severity:** LOW.

## Question

Do the chosen shortcuts compromise WU-0A-02's purpose (a contract-pinned settings DTO + loader with every documented `SettingsError` reachable from fixture-backed tests)? Are TODOs, stubbed branches, weak assertions, or deferred behaviors hiding incomplete work?

## Shortcuts examined

### S1 — `agent_runner_bin` validated only by `Path::is_file()`

`src-tauri/src/settings.rs:72-75` resolves the path against `cwd` if relative, then checks `is_file()`. There is no canonicalization, no executable-bit check, no symlink-target validation. Justified: the contract (`product-strategy/contracts/wu-0a-02-…md:44`) only requires "resolves to an existing file under `cwd` when relative"; executability and `PATH` semantics are explicitly outside this WU. Both `error-missing-agent-runner-bin-empty.json` and `error-missing-agent-runner-bin-unresolved.json` exercise the negative cases, and `bin/fake-agents` is a real on-disk file used by the success path. **Justified.**

### S2 — Empty-string check uses `trim().is_empty()`

`settings.rs:89-95` (`required_string`) treats whitespace-only values as empty. The contract uses the word "empty" without further qualification (`contract:48-55`, ticket criteria 3-6). No fixture exercises a whitespace-only value, so the trim adds defensive coverage without changing the documented behavior. Mild over-strictness rather than under-implementation; not a hidden shortcut. **Justified.**

### S3 — `ConfigFileUnreadable` collapses I/O errors and JSON parse errors

`settings.rs:42-46` (`read_config_file`) maps both `fs::read_to_string` failures and `serde_json::from_str` failures to `SettingsError::ConfigFileUnreadable`. The contract treats both as the same variant ("ConfigFileUnreadable when `config_file` is present but cannot be parsed by the settings loader" — ticket criterion 8), so the collapse is the documented behavior, not a shortcut. The `invalid-config.json` fixture drives the JSON-parse arm; an absent path drives the I/O arm. **Justified.**

### S4 — `RawHarnessSettings` uses `deny_unknown_fields`, also routing to `ConfigFileUnreadable`

`settings.rs:16-25`. The contract DTO clause ("Unknown DTO fields are rejected by the Rust contract type and the TypeScript parser" — `contract:18`) covers this. An unknown key in a config file therefore produces `ConfigFileUnreadable` rather than a silent drop. Stricter than necessary but contract-aligned; no test gap. **Justified.**

### S5 — `Display` impl for `SettingsError` derives string from `serde_json`

`harness_settings.rs:35-40` writes the variant name by serializing through `serde_json` and trimming the surrounding quotes. Convenience implementation that ties the human-readable form to the on-the-wire form, so they cannot drift. Not a shortcut around correctness. **Justified.**

### S6 — TS parser uses an inline allowed-key list rather than zod/typia

`src/contracts/harness-settings.ts:41-74`. The parser hand-rolls field-by-field shape checks and an exact-key-set comparison (sorted, conditionally including `profile_name`). The proposal anti-scope intentionally avoids new runtime deps; tests at `src/test/harness-settings.test.ts:33-54` cover unknown-field rejection, wrong-type rejection, and bad-union rejection. **Justified.**

### S7 — Loader does not check existence of `storage_root` or `database_path`

`settings.rs:62-87` validates emptiness only; no filesystem probe. Justified by proposal A2 (`proposals/02-wu-0a-02.md:39`): containment and storage-layout validation belong to WU-0A-03. Not a hidden shortcut, an explicit deferral. **Justified.**

## Hidden-incomplete-work check

- No `TODO`, `FIXME`, `XXX`, `HACK`, `unimplemented!()`, or `todo!()` markers in `src-tauri/src/settings.rs`, `src-tauri/src/contracts/harness_settings.rs`, `src/contracts/harness-settings.ts`, or the new tests.
- No commented-out branches; no `#[allow(dead_code)]`; no `#[ignore]` tests.
- All seven fixture-driven error cases are looped through in `harness_settings_contract.rs:92-120` with a per-fixture assertion message — no silent skips.
- `parseSettingsError` and `parseHarnessSettings` both throw on bad input rather than returning sentinels (`harness-settings.ts:25-39`), so test failures cannot mask via undefined returns.
- `RawHarnessSettings::default()` (used when `config_file` is `None` — `settings.rs:35`) yields all `None`s, which is correctly rejected by `required_string`. No silent fallback to placeholder values.

## Verdict

**LOW.** Every shortcut is either documented in the contract, justified by the proposal anti-scope/assumptions, or compensated by negative fixtures. No deferred work hidden in TODOs, no stubbed enum arms, no weak assertions: each error variant is reachable, each variant is round-tripped through both Rust serde and the TS parser, and the success path is asserted against an explicit expected-result fixture rather than a self-referential snapshot.
