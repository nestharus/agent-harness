# WU-0A-02 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does deployment-mode + customer-cohort + adjacent-public-paths analysis match the WU-0A-02 supported-surface declaration? What is the blast radius for adjacent paths (the WU-0A-01 inert scaffold)? Migration / rollback / observability story?

## Supported-surface contract (proposal `proposals/02-wu-0a-02.md:26-34`)

- Deployment mode: local Phase 0A desktop scaffold.
- Customer cohort: developer/test harness consumers only.
- Public paths: Rust library consumers (`agent_harness_lib::settings`, `agent_harness_lib::contracts::harness_settings`) and TypeScript contract importers (`src/contracts/harness-settings.ts`).
- Adjacent paths: existing inert Tauri bootstrap and React shell remain command-free and provider-free.
- Migration path: additive files and module wiring only.
- Rollback path: remove the added contract/settings files and module declarations.
- Observability: contract tests expose loader success and every documented error variant.

## Findings

### Public surface matches the declaration

- Rust: `src-tauri/src/lib.rs:1-2` adds `pub mod contracts;` and `pub mod settings;`. Public symbols introduced are limited to `HarnessSettings`, `HarnessLogLevel`, `SettingsError` (`src-tauri/src/contracts/harness_settings.rs:5-33`) and `load_harness_settings` (`src-tauri/src/settings.rs:27-31`). No `pub` items beyond these in the new modules.
- TypeScript: `src/contracts/harness-settings.ts` exports the `HARNESS_LOG_LEVELS` tuple, `HarnessLogLevel` type, `HarnessSettings` interface, `SETTINGS_ERRORS` tuple, `SettingsError` type, and the two parser functions. No default export, no module-side-effect code.
- The exposed surface is exactly what the contract calls "Rust library consumers and TypeScript contract importers."

### Adjacent inert scaffold remains command-free and provider-free

- `src-tauri/src/lib.rs:5-10` is unchanged in shape: `tauri::Builder::default().setup(|_app| Ok(())).run(...)`. Zero `generate_handler!` invocations; the WU-0A-01 contract assertion about no value-slice commands is preserved.
- `registered_command_count()` still returns `0` (`lib.rs:12-14`); the `phase_0a_registers_no_value_slice_commands` test (`lib.rs:17-23`) still passes.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The visible application root and its capability set are unchanged.
- No new env-var prefixes in `vite.config.ts`; the loader does not exfiltrate env into the bundle (it only reads an explicit `&HashMap` passed by the caller — `settings.rs:48-60`).
- No new bundle targets, no new icons, no new Tauri permissions.

### Blast radius

- Greenfield Phase 0A: no prior runtime users of `agent_harness_lib` or `src/contracts`. The new public API is additive; nothing pre-existing depends on it.
- The `Cargo.toml` change (`src-tauri/Cargo.toml:16-21`) promotes `serde` and `serde_json` from dev-only to runtime deps. Both versions match what was already locked, so `Cargo.lock` updates are non-breaking. No transitive feature flags toggled.
- The fixtures live under `product-strategy/contracts/fixtures/wu-0a-02/`, namespaced by WU. They cannot collide with WU-0A-01 fixtures.

### Migration / rollback path

- Migration: none. No persistent state, no schema, no on-disk artifacts produced by the loader. The function is pure over its inputs (with one filesystem read for the optional config file and one `is_file()` probe for `agent_runner_bin`).
- Rollback: delete the added files (`src-tauri/src/settings.rs`, `src-tauri/src/contracts/`, `src/contracts/harness-settings.ts`, the two test files, the proposal, the contract MD, the fixtures directory) and revert `lib.rs:1-2`, `contracts/mod.rs`, and the `Cargo.toml` dep promotions. No stateful reconciliation needed.

### Observability

- Loader success path: `harness_settings_contract.rs:67-84` asserts `load_harness_settings` returns the expected merged settings.
- Every documented `SettingsError` variant: `harness_settings_contract.rs:86-121` loops the seven `error-*.json` fixtures and asserts the matching variant.
- Round-trip and unknown-variant rejection: `harness_settings_contract.rs:149-183` covers Rust serde; `src/test/harness-settings.test.ts:56-70` covers TS parser.
- DTO shape pinning: `harness_settings_contract.rs:36-65` asserts the canonical fixture's exact key set; `harness-settings.test.ts:16-31` asserts the same on the TS side.
- Log-level union: Rust at `harness_settings_contract.rs:124-147`, TS at `harness-settings.test.ts:72-83` (with a `// @ts-expect-error` for compile-time enforcement).

### Boundary observations (informational)

- INFO: `load_harness_settings` reads the supplied `config_file` from disk via `fs::read_to_string` (`settings.rs:44`). Callers control the path; no ambient discovery. Document this in any future caller guidance so callers do not accidentally pass user-controlled paths without sandboxing.
- INFO: Promoting `serde_json` to a runtime dep slightly enlarges the lib binary surface but matches what later WUs (IPC payloads, GraphStore JSON columns) will need anyway. Tracked as expected, not as drift.

## Verdict

**LOW.** Public surface is the declared library types + loader function + parser functions, nothing else. The WU-0A-01 inert scaffold is untouched in shape and behavior — still command-free, provider-free, capability-minimal. Migration is additive only, rollback is a clean revert, and every acceptance criterion is observable via at least one test that pins it to a fixture rather than to internal state.
