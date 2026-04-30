# WU-0A-02 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-02 settings DTO + loader — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-02.md`) explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Concern enumeration

Files added/modified on the branch group cleanly under one logical concern: "ship the Phase 0A `HarnessSettings` DTO + `load_harness_settings` loader + `SettingsError` taxonomy with bilingual contract tests."

| Group | Files | Why required for this single concern |
|---|---|---|
| Rust contract types | `src-tauri/src/contracts/mod.rs`, `src-tauri/src/contracts/harness_settings.rs` | The Rust DTO, log-level enum, and error enum that the WU contract owns (`product-strategy/contracts/wu-0a-02-…md:7-58`). |
| Rust loader | `src-tauri/src/settings.rs` | `load_harness_settings(cwd, env_overrides, config_file)` per WU contract loader section (`…md:20-44`). |
| Rust module wiring + dep promotion | `src-tauri/src/lib.rs` (additions of `pub mod contracts; pub mod settings;`), `src-tauri/Cargo.toml` (promote `serde`/`serde_json` to runtime deps), `src-tauri/Cargo.lock` (refreshed) | Minimum needed to compile the new modules into the lib crate. |
| TypeScript contract types + parser | `src/contracts/harness-settings.ts` | The TS DTO, log-level union, error union, and parsers required by ticket criterion 1 and 9. |
| Rust contract test | `src-tauri/tests/harness_settings_contract.rs` | Fixture-backed coverage of the loader success path and every documented `SettingsError` variant. |
| TS contract test | `src/test/harness-settings.test.ts` | DTO parse/reject coverage and error-variant round-trip on the TS side. |
| Fixtures | `product-strategy/contracts/fixtures/wu-0a-02/{canonical-settings,config-input,env-overrides,env-overridden-settings,settings-errors,invalid-settings-error,invalid-config,error-*}.json`, `…/bin/fake-agents` | Canonical inputs/outputs for the success path and one fixture per documented error variant (proposal test-intent and contract `…md:59-70`). |
| Contract spec + proposal | `product-strategy/contracts/wu-0a-02-harness-settings-dto-and-settings-loader.md`, `proposals/02-wu-0a-02.md` | Phase-3 proposal and WU-owned contract document. |

## Severability check — can any group ship independently?

- **Rust contract types alone** (without the loader): would not satisfy ticket criteria 2-8 (loader behavior). The error enum has no observable use without the loader to emit it.
- **Rust loader alone** (without the contract types): does not compile — the loader returns `Result<HarnessSettings, SettingsError>`.
- **TypeScript DTO alone** (without Rust DTO + fixtures): would not satisfy criterion 1 (Rust → JSON → TS round-trip) because the canonical fixture is the round-trip evidence.
- **Fixtures alone**: cannot ship — they have no consumer.
- **Tests alone**: cannot ship — they reference symbols that only exist with the rest of the change.
- **Contract spec / proposal alone**: required workflow artifacts for this WU; not a separable concern.
- **`Cargo.toml`/`lib.rs`/`mod.rs` wiring**: cannot ship without the modules they declare; would break the build.

No group is independently shippable.

## Cross-WU contamination check

- No file references WU-0A-03 (`LocalStorageLayout`), WU-0A-04+, WU-0A-14, or any later phase ticket.
- No GraphStore migration files (`src-tauri/migrations/` is still absent), no provider crates added (`Cargo.toml:16-21` only adds `serde`), no subprocess code (no `std::process` or `tokio::process` imports), no `tauri::generate_handler!`, no `app.manage(...)`, no IPC commands. The WU-0A-01 scaffold contract assertion `phase_0a_registers_no_value_slice_commands` still passes (`src-tauri/src/lib.rs:17-23`).
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The user-facing shell is untouched.
- `proposals/` now contains `01-wu-0a-01.md` and `02-wu-0a-02.md`; no third proposal snuck in. `product-strategy/contracts/fixtures/` contains `wu-0a-01/` and `wu-0a-02/`; no third fixture set.
- `Cargo.lock` changes are limited to what `serde`/`serde_json` promotion implies (no surprise crate additions for unrelated WUs).

## Observations

- The `bin/fake-agents` fixture file is small (54 bytes, plain text — it is not actually executable). It exists solely as a `Path::is_file()` target for the `agent_runner_bin` resolution check, which only requires existence. Single-purpose fixture, on-concern.
- Both contract tests use shared helper functions (`fixture_path`, `read_json`) that live inside the Rust test file rather than in a new shared crate — keeps the change scoped to one concern instead of introducing a test-helpers module.

## Verdict

**LOW.** This is a single-concern PR for WU-0A-02. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, no group serves another WU, and there is no contamination from WU-0A-01 (untouched) or any later WU (no GraphStore, providers, IPC, app-state, subprocesses, or storage-layout work). The single-concern handoff note is honored.
