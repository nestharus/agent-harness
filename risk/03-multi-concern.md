# WU-0A-03 — Multi-Concern Risk Review

**Gate:** Phase 8 multi-concern (single-concern PR check).
**Severity:** LOW.

## Question

Does this branch belong together as one PR — the WU-0A-03 `LocalStorageLayout` DTO + `derive_local_storage_layout` helper + `StorageLayoutError` taxonomy + bilingual fixtures — or are there severable chunks that should split out? The ticket handoff (`tickets-phase-0a:plans/tickets/phase-0a/WU-0A-03.md`) explicitly says "Single-concern PR: this ticket maps to one PR. Do not bundle other tickets into the same PR."

## Concern enumeration

Files added/modified on the branch (relative to the WU-0A-11 parent commit) group cleanly under one logical concern: "ship the Phase 0A `LocalStorageLayout` DTO + `derive_local_storage_layout` helper + `StorageLayoutError` taxonomy with bilingual contract tests."

| Group | Files | Why required for this single concern |
|---|---|---|
| Rust contract types | `src-tauri/src/contracts/local_storage_layout.rs`, `src-tauri/src/contracts/mod.rs` (one-line addition at line 3) | The Rust DTO + 6-variant error enum that the WU contract owns (`product-strategy/contracts/wu-0a-03-local-storage-layout-dto.md:9-49`). |
| Rust storage module | `src-tauri/src/storage.rs`, `src-tauri/src/lib.rs` (one-line addition at line 4) | `derive_local_storage_layout(settings)` and the configured-layout `validate_local_storage_layout` per WU contract derivation section (`…dto.md:20-38`). |
| TypeScript contract types + parser | `src/contracts/local-storage-layout.ts` | TS DTO + error union + parsers required by ticket criteria 1, 7. |
| Rust contract test | `src-tauri/tests/local_storage_layout_contract.rs` | Fixture-backed coverage of DTO shape, derivation success, every documented `StorageLayoutError` variant (both derivation and configured-layout paths), and serde round-trip. |
| TS contract test | `src/test/local-storage-layout.test.ts` | DTO parse/reject coverage and error-variant round-trip on the TS side, including a `// @ts-expect-error` compile-time check. |
| Fixtures | `product-strategy/contracts/fixtures/wu-0a-03/{canonical-layout,derive-success,derive-errors,invalid-layouts,invalid-layout-shapes,storage-layout-errors,invalid-storage-layout-error}.json` | Canonical inputs/outputs for the success path and one fixture per documented error variant (proposal test-intent and contract `…dto.md:53-63`). |
| Contract spec + proposal | `product-strategy/contracts/wu-0a-03-local-storage-layout-dto.md`, `proposals/03-wu-0a-03.md` | Phase-3 proposal and WU-owned contract document. |

## Severability check — can any group ship independently?

- **Rust contract types alone** (without `storage.rs`): would not satisfy ticket criteria 2-6 (derivation behavior). The error enum has no observable use without the helper to emit it.
- **Rust storage module alone** (without the contract types): does not compile — `derive_local_storage_layout` returns `Result<LocalStorageLayout, StorageLayoutError>`.
- **TypeScript DTO + parser alone** (without Rust DTO + fixtures): would not satisfy criterion 1 (Rust → JSON → TS round-trip) because the canonical fixture is the cross-language pin.
- **Fixtures alone**: cannot ship — they have no consumer.
- **Tests alone**: cannot ship — they reference symbols that only exist with the rest of the change.
- **Contract spec / proposal alone**: required workflow artifacts for this WU; not a separable concern.
- **`lib.rs` / `contracts/mod.rs` wiring**: cannot ship without the modules they declare; would break the build.

No group is independently shippable.

## Cross-WU contamination check

- No file references WU-0A-04, WU-0A-06+, WU-0A-14, or any later phase ticket.
- No GraphStore migration files (`src-tauri/migrations/` is still absent), no provider crates added (`Cargo.toml` and `Cargo.lock` are unchanged on this branch — `git status` shows neither modified), no subprocess code (no `std::process` / `tokio::process` imports in `storage.rs`), no `tauri::generate_handler!`, no `app.manage(...)`, no IPC commands. The WU-0A-01 scaffold contract assertion `phase_0a_registers_no_value_slice_commands` still passes (`src-tauri/src/lib.rs:19-26`).
- No edits to `src-tauri/src/settings.rs` or `src-tauri/src/contracts/harness_settings.rs` — WU-0A-02 settings loader is byte-identical. `storage.rs:3` only *imports* `HarnessSettings` from the existing module to satisfy the helper's input type.
- No edits to `src/contracts/harness-settings.ts` or any WU-0A-02 fixture under `product-strategy/contracts/fixtures/wu-0a-02/`.
- WU-0A-05 / WU-0A-09 / WU-0A-11 modules untouched: no edits to `src-tauri/src/events/`, `src-tauri/src/tracing/`, the pane-id module, or any of the `event-topic`/`trace-context`/`pane-id` TS contracts and fixtures.
- No edits to `src/main.tsx`, `src/App.tsx`, `src/router.tsx`, `src/ShellRoot.tsx`, `src/styles.css`, `index.html`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `src-tauri/tauri.conf.json`, or `src-tauri/capabilities/default.json`. The user-facing shell and capability set are untouched.
- `proposals/` adds only `03-wu-0a-03.md`; no other proposal snuck in. `product-strategy/contracts/fixtures/` adds only `wu-0a-03/`; no cross-WU fixtures touched.

## Observations

- The split between `contracts/local_storage_layout.rs` (DTO + error types) and `storage.rs` (derivation + validation helpers) mirrors the WU-0A-02 split (`contracts/harness_settings.rs` + `settings.rs`). Two files keep the change scoped to one concern instead of inventing a shared module.
- Both contract tests use shared fixture-loop assertion patterns (`derive_layout_rejects_documented_settings_errors` and `configured_layout_validation_rejects_paths_outside_storage_root` in `local_storage_layout_contract.rs:118-152`), so adding a future error variant only requires updating the fixture; no new variant could be silently absent.
- The two helper paths through `validate_local_storage_layout` (called once from `derive_local_storage_layout` and once directly from the configured-layout test) keep derivation and configured-layout containment behaviorally identical.

## Verdict

**LOW.** This is a single-concern PR for WU-0A-03. Every file directly serves the ticket's contract or acceptance criteria, no group is independently shippable, no group serves another WU, and there is no contamination from WU-0A-01 / WU-0A-02 / WU-0A-05 / WU-0A-09 / WU-0A-11 (all untouched) or any later WU (no GraphStore, providers, optimizers, workers, questions, recovery, budget, audit, IPC, app-state, subprocesses, payload schemas, real directory creation, symlink resolution, or filesystem canonicalization). The single-concern handoff note is honored.
