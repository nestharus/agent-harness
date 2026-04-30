# WU-0A-03 — Supported-Surface Risk Review

**Gate:** Phase 4 supported-surface.
**Severity:** LOW.

## Question

Does deployment-mode + customer-cohort + adjacent-public-paths analysis match the WU-0A-03 supported-surface declaration? What is the blast radius for adjacent paths (the WU-0A-01 inert scaffold, WU-0A-02 settings loader, WU-0A-05 event-topic taxonomy, WU-0A-09 trace-context schema, WU-0A-11 pane-id taxonomy)? Migration / rollback / observability story?

## Supported-surface contract (proposal `proposals/03-wu-0a-03.md:26-34`)

- Deployment mode: local Phase 0A desktop scaffold.
- Customer cohort: developer/test harness consumers only.
- Public paths: Rust library consumers and TypeScript contract importers.
- Adjacent paths: WU-0A-02 settings loading remains string-preserving; existing event, trace, pane, shell, and Tauri command-free paths remain unchanged.
- Migration path: additive contract, storage module, fixtures, and tests.
- Rollback path: remove the added contract/storage files, fixtures, tests, and module declarations.
- Observability: contract tests expose layout derivation, every documented error variant, and strict TypeScript parsing.

## Findings

### Public surface matches the declaration

- Rust: `src-tauri/src/lib.rs:4` adds `pub mod storage;`; `src-tauri/src/contracts/mod.rs:3` adds `pub mod local_storage_layout;`. Public symbols introduced are limited to:
  - `agent_harness_lib::contracts::local_storage_layout::{LocalStorageLayout, StorageLayoutError}` (`src-tauri/src/contracts/local_storage_layout.rs:5-22`).
  - `agent_harness_lib::storage::{derive_local_storage_layout, validate_local_storage_layout}` (`src-tauri/src/storage.rs:11-13, 28-31`).
  - The four child-root constants (`EVIDENCE_ROOT_NAME`, `FIXTURE_ROOT_NAME`, `LOG_ROOT_NAME`, `TEMP_ROOT_NAME`) at `storage.rs:6-9` are private; the path normalizer and helpers at `storage.rs:63-106` are private. Consumers can only access the layout through the contract types and the two helpers.
- TypeScript: `src/contracts/local-storage-layout.ts` exports the `LocalStorageLayout` interface, `STORAGE_LAYOUT_ERRORS` const tuple, `StorageLayoutError` type, `parseLocalStorageLayout`, and `parseStorageLayoutError`. No default export, no module-side-effect code.
- The exposed surface is exactly what the contract calls "Rust library consumers and TypeScript contract importers."

### Adjacent paths remain unchanged

- WU-0A-01 inert scaffold:
  - `src-tauri/src/lib.rs:7-12` is unchanged in shape: `tauri::Builder::default().setup(|_app| Ok(())).run(...)`. Zero `generate_handler!` invocations; the WU-0A-01 contract assertion `phase_0a_registers_no_value_slice_commands` (`lib.rs:19-26`) still passes (`registered_command_count()` still returns `0` at `lib.rs:15-17`).
  - No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The visible application root and capability set are unchanged.
- WU-0A-02 settings loader:
  - No edits to `src-tauri/src/settings.rs` or `src-tauri/src/contracts/harness_settings.rs`. The settings DTO is *consumed* (the loader output feeds `derive_local_storage_layout`) but not modified — `storage.rs:3` imports `HarnessSettings` and `storage.rs:11-13` takes it by reference. Loader behavior remains string-preserving as A1 requires (`proposals/03-wu-0a-03.md:38`).
  - `src/contracts/harness-settings.ts` is unchanged.
- WU-0A-05 / WU-0A-09 / WU-0A-11: the `events`, `tracing`, and pane-id modules are untouched. `git diff` only shows the two single-line module additions plus the new files, so the prior taxonomies are byte-identical.
- `Cargo.toml` and `Cargo.lock` are unchanged (`git status` shows neither modified) — the serde derives reuse runtime deps already promoted by WU-0A-02.

### Blast radius

- Greenfield Phase 0A: no prior runtime users of `agent_harness_lib::storage` or `src/contracts/local-storage-layout`. The new public API is purely additive; nothing pre-existing depends on it.
- The `storage` module is a new top-level crate-internal namespace; reordering or renaming inside it cannot affect the WU-0A-01 / WU-0A-02 / WU-0A-05 / WU-0A-09 / WU-0A-11 namespaces.
- The fixtures live under `product-strategy/contracts/fixtures/wu-0a-03/`, namespaced by WU. They cannot collide with other WU fixture sets.
- No transitive dep changes; no feature flags toggled; no new bundle targets, icons, or Tauri permissions.

### Migration / rollback path

- Migration: none. No persistent state, no schema, no on-disk artifacts produced by derivation or validation. Both helpers are pure over their inputs (no `std::fs`, no `std::env::var`, no `Path::canonicalize`).
- Rollback: delete the added files (`src-tauri/src/storage.rs`, `src-tauri/src/contracts/local_storage_layout.rs`, `src-tauri/tests/local_storage_layout_contract.rs`, `src/contracts/local-storage-layout.ts`, `src/test/local-storage-layout.test.ts`, `proposals/03-wu-0a-03.md`, `product-strategy/contracts/wu-0a-03-local-storage-layout-dto.md`, `product-strategy/contracts/fixtures/wu-0a-03/`) and revert the two one-line additions in `src-tauri/src/lib.rs:4` and `src-tauri/src/contracts/mod.rs:3`. No stateful reconciliation needed.

### Observability

- DTO shape pinning: `src-tauri/tests/local_storage_layout_contract.rs:51-82` asserts the canonical fixture's exact key set and round-trips through serde; `src/test/local-storage-layout.test.ts:18-37` asserts the same on the TS side.
- Derivation success: `local_storage_layout_contract.rs:84-116` checks that `derive_local_storage_layout` preserves `storage_root` and `database_path`, produces four distinct child roots, and that every child root passes `Path::starts_with(storage_root)`.
- Every documented error variant: `local_storage_layout_contract.rs:118-152` loops `derive-errors.json` and `invalid-layouts.json` and asserts the matching variant per case.
- Round-trip and unknown-variant rejection: `local_storage_layout_contract.rs:154-188` covers Rust serde of all six variants plus the negative `invalid-storage-layout-error.json`; `local-storage-layout.test.ts:49-76` covers the TS parser side, including a `// @ts-expect-error` compile-time check at line 62.
- TS shape rejection: `local-storage-layout.test.ts:39-47` loops `invalid-layout-shapes.json` (missing field, extra field, wrong type) and asserts each throws `Invalid LocalStorageLayout`.

### Boundary observations (informational)

- INFO: `validate_local_storage_layout` is `pub` on the storage module surface (`storage.rs:28-31`). The `invalid-layouts.json` ticket criterion exercises configured-layout validation distinct from derivation, so the helper is a documented public symbol. Future WUs that re-validate layouts after deserialization (e.g. an app-state config loader) should consume it rather than re-encoding the lexical containment rules.
- INFO: `derive_local_storage_layout` is pure over its input — there is no ambient process-env or filesystem read — so callers are free to call it during cold-start without I/O concerns. Document this in any future caller guidance.

## Verdict

**LOW.** Public surface is exactly the declared library types + derivation + validation helper + parser functions, nothing else. The WU-0A-01 inert scaffold, WU-0A-02 settings loader, and the WU-0A-05 / WU-0A-09 / WU-0A-11 taxonomies are all untouched in shape and behavior. Migration is additive only, rollback is a clean revert, and every acceptance criterion is observable via at least one test that pins it to a fixture rather than to internal state.
