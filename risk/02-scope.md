# WU-0A-02 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-02 boundary (HarnessSettings DTO + log-level union + `load_harness_settings` loader + documented `SettingsError` taxonomy + fixtures), or does it bleed into WU-0A-03 storage-layout containment, GraphStore migrations, provider configs, IPC command registration, app-state plumbing, or subprocess supervision?

## Findings

### In-scope, confirmed

- DTO fields match exactly: `workspace_id`, `storage_root`, `database_path`, `agent_runner_bin`, `log_level`, optional `profile_name` (`src-tauri/src/contracts/harness_settings.rs:5-13`, `src/contracts/harness-settings.ts:5-12`). `#[serde(deny_unknown_fields)]` on the Rust struct (`harness_settings.rs:4`) and the explicit allowed-key set in the TS parser (`harness-settings.ts:48-63`) honor the contract clause "Unknown DTO fields are rejected" (`product-strategy/contracts/wu-0a-02-…md:18`).
- Log-level union is exactly `trace|debug|info|warn|error` (Rust enum at `harness_settings.rs:15-23` with `rename_all = "lowercase"`; TS const tuple at `harness-settings.ts:1-3`). Negative tests reject `"verbose"` (`src-tauri/tests/harness_settings_contract.rs:143-146`, `src/test/harness-settings.test.ts:38-41,77`).
- `SettingsError` enumerates exactly the six documented variants (`harness_settings.rs:25-33`, `harness-settings.ts:14-23`); the `invalid-settings-error.json` round-trip fixture proves Rust serde and the TS parser both reject `"UnknownSettingsError"` (`harness_settings_contract.rs:177-182`, `harness-settings.test.ts:69`).
- Loader signature matches the contract verbatim (`src-tauri/src/settings.rs:27-31`): `cwd`, `&HashMap<String,String>` env overrides, optional config-file path string, `Result<HarnessSettings, SettingsError>`.
- Resolution order matches contract `…md:30-44`: empty defaults → optional config-file overlay → recognized env keys → unrecognized env keys ignored (`settings.rs:33-39, 48-60`) → required-field validation → `agent_runner_bin` file-existence check.
- All seven error reachability fixtures resolve to their documented variant (`harness_settings_contract.rs:92-120`), satisfying ticket acceptance criteria 3-8.

### Anti-scope honored

- Anti-scope from `proposals/02-wu-0a-02.md:20-24`:
  - No GraphStore schema or migrations (no `src-tauri/migrations/`, no SQL).
  - No provider config (no provider crates added; `Cargo.toml:16-21` only promotes `serde`/`serde_json` to main deps to support the contract types).
  - No subprocess supervision (`settings.rs` does not import `std::process` or `tokio::process`).
  - No IPC command registration: `src-tauri/src/lib.rs:5-10` still calls `tauri::Builder::default().setup(|_app| Ok(()))` with zero `generate_handler!`. `registered_command_count()` still returns `0` (`lib.rs:12-14`); the WU-0A-01 scaffold contract assertion is preserved.
  - No app-state container: no `app.manage(...)`, no `tauri::State`, no global statics.
  - No storage-layout derivation: `storage_root`/`database_path` are passed through as literal strings; the loader never canonicalizes or containment-checks them, deferring that to WU-0A-03 (proposal A2 at `proposals/02-wu-0a-02.md:39`).
- No default discovery of process env vars: `apply_env_overrides` (`settings.rs:48-60`) reads only the supplied `&HashMap`; no `std::env::var` calls anywhere in the new files. Matches A3 in the proposal.
- No shell UI changes: no edits under `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, or `src/ShellRoot.tsx`. Only additions are `src/contracts/harness-settings.ts` and `src/test/harness-settings.test.ts`.

### Adjacent-path additions (in-scope)

- `proposals/02-wu-0a-02.md`, `product-strategy/contracts/wu-0a-02-…md`, and `…/fixtures/wu-0a-02/*` are this WU's own contract artifacts.
- `product-strategy/contracts/fixtures/wu-0a-02/bin/fake-agents` is a fixture file used solely as the "exists" target for the `agent_runner_bin` resolution check, scoped to this WU's tests.
- `src-tauri/Cargo.toml:17-18` promotes `serde`/`serde_json` from dev-only to main deps because the contract types are now compiled into the lib crate. Necessary, minimal.
- `src-tauri/src/contracts/mod.rs:1` adds `pub mod harness_settings;`; `src-tauri/src/lib.rs:1-2` declares `pub mod contracts;` and `pub mod settings;`. Module wiring only.

### Potential concerns — none rising to MEDIUM

- INFO: `required_string` trims whitespace before emptiness check (`settings.rs:89-95`), so a whitespace-only value (e.g. `"   "`) maps to `EmptyWorkspaceId` etc. The contract speaks only of "empty", not "blank". This is a defensible reading but slightly stronger than the literal contract; no fixture exercises it, no behavior depends on it elsewhere. Worth noting; not a scope violation.
- INFO: `agent_runner_bin` is checked with `is_file()` (`settings.rs:73`) which silently treats permission errors / symlink failures as "not a file" and returns `MissingAgentRunnerBin`. The contract's "unresolved" wording covers this. Aligned with intent.

## Verdict

**LOW.** The implementation lands exactly the DTO, loader, error taxonomy, and fixtures called out by the WU contract and ticket. Anti-scope is honored end-to-end (no GraphStore, no providers, no IPC, no app-state, no storage-layout derivation, no ambient env reads), and the only adjacent edits are the minimal module wiring and dependency promotion that the new contract types make unavoidable.
