# WU-0A-03 — Scope Risk Review

**Gate:** Phase 4 scope.
**Severity:** LOW.

## Question

Does the implementation stay inside the WU-0A-03 boundary (the `LocalStorageLayout` DTO + `derive_local_storage_layout` helper + 6-variant `StorageLayoutError` taxonomy + bilingual fixtures), or does it bleed into GraphStore migrations, SQLite opening, app-state construction, IPC commands, real directory creation, symlink resolution, filesystem canonicalization, or settings-loading work the proposal anti-scope reserves for later WUs (`proposals/03-wu-0a-03.md:20-25`)?

## Findings

### In-scope, confirmed

- Rust `LocalStorageLayout` declares exactly the six documented fields (`storage_root`, `database_path`, `evidence_root`, `fixture_root`, `log_root`, `temp_root`) with `#[serde(deny_unknown_fields)]` (`src-tauri/src/contracts/local_storage_layout.rs:3-12`). Matches contract DTO clause (`product-strategy/contracts/wu-0a-03-local-storage-layout-dto.md:9-18`).
- Rust `StorageLayoutError` encodes exactly the six documented variants in canonical order (`local_storage_layout.rs:14-22`); default serde produces the PascalCase wire strings the contract specifies (`…dto.md:42-49`).
- `derive_local_storage_layout(settings)` signature matches the contract verbatim (`src-tauri/src/storage.rs:11-13`): takes `&HarnessSettings`, returns `Result<LocalStorageLayout, StorageLayoutError>`. Preserves `settings.storage_root` and `settings.database_path` literally; derives `evidence_root`/`fixture_root`/`log_root`/`temp_root` from the four stable child names `evidence`/`fixtures`/`logs`/`tmp` (`storage.rs:6-21`, contract `…dto.md:31-35`).
- `validate_local_storage_layout` enforces every documented containment failure: storage-root parent-traversal escape → `StorageRootEscapesWorkspace`; each non-storage path → its specific child error variant (`storage.rs:28-61`, contract `…dto.md:26-37`).
- TypeScript mirror: `LocalStorageLayout` interface with the same six fields (`src/contracts/local-storage-layout.ts:1-8`), `STORAGE_LAYOUT_ERRORS` const tuple in canonical order (`local-storage-layout.ts:10-19`), `parseLocalStorageLayout` rejects extra/missing/wrong-type fields via an explicit allowed-key set (`local-storage-layout.ts:37-65`), `parseStorageLayoutError` rejects unknown strings (`local-storage-layout.ts:29-35,67-69`).

### Anti-scope honored

The proposal anti-scope (`proposals/03-wu-0a-03.md:21-25`) bars: GraphStore schema/migrations, providers, app-state, SQLite opening, subprocess supervision, IPC registration, real directory creation, symlink resolution, filesystem canonicalization, ambient env reads. Verified:

- No `tauri::generate_handler!` invocation; `src-tauri/src/lib.rs:8-12` still uses `tauri::Builder::default().setup(|_app| Ok(()))`. `registered_command_count()` still returns `0` and the WU-0A-01 assertion `phase_0a_registers_no_value_slice_commands` still passes (`lib.rs:15-26`).
- No filesystem mutation: `storage.rs` imports only `std::path::{Component, Path, PathBuf}` and the in-crate contract types — no `std::fs`, no `tokio::fs`, no `std::process`, no SQL, no migration files. Path containment is purely lexical (`storage.rs:84-106`).
- No symlink/canonicalization: `normalize_workspace_path` is a pure component walker that resolves `.` and `..` lexically and rejects parent traversal that escapes the implicit root. No `Path::canonicalize`, no `fs::read_link`. Matches proposal A2 (`proposals/03-wu-0a-03.md:39`).
- No ambient env reads: no `std::env::var` calls; `derive_local_storage_layout` is pure over its `&HarnessSettings` input.
- No app-state container: no `app.manage(...)`, no `tauri::State`, no global statics.
- No GraphStore migration files (`src-tauri/migrations/` still absent), no provider crates added (`Cargo.toml` and `Cargo.lock` unchanged).

### Adjacent-path additions (in-scope)

- `src-tauri/src/lib.rs:4`: adds `pub mod storage;` next to the existing `pub mod contracts; pub mod events; pub mod settings; pub mod tracing;`. One-line module declaration. Required to expose the helper to the integration test crate.
- `src-tauri/src/contracts/mod.rs:3`: adds `pub mod local_storage_layout;` between `harness_settings` and `trace_context`. One-line module declaration.
- No `Cargo.toml` / `Cargo.lock` churn — the serde derives reuse the runtime deps already promoted by WU-0A-02.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The shell, capabilities, and bundle config are untouched.

### Potential concerns — none rising to MEDIUM

- INFO: `validate_local_storage_layout` is exported `pub` (`storage.rs:28-31`), beyond the single `derive_local_storage_layout` named in the WU Contract field. It exists to satisfy the `invalid-layouts.json` ticket criterion (re-validating *configured* layouts after deserialization) — the success-path derivation funnels through it via `derive_local_storage_layout` (`storage.rs:23`). Loop-bound to fixtures in `src-tauri/tests/local_storage_layout_contract.rs:137-152`. On-concern.
- INFO: `Display` for `StorageLayoutError` synthesizes the wire string via `serde_json::to_string` (`local_storage_layout.rs:24-29`). Convenience for `?` interop; ties human form to wire form. Not a scope expansion.

## Verdict

**LOW.** Implementation lands exactly the DTO, derivation helper, error taxonomy, and bilingual fixtures the WU contract and ticket call out. Anti-scope is honored end-to-end (no fs mutation, no canonicalization, no symlinks, no ambient env, no SQL/migrations, no IPC, no app-state, no providers); the only adjacent edits are the two unavoidable one-line module wiring lines.
