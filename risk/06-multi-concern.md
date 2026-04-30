# WU-0A-06 — Multi-Concern PR Risk Review

**Gate:** Phase 4 multi-concern.
**Severity:** LOW.

## Question

Does the WU-0A-06 change set bundle a single concern (the `IpcEvent<T>` envelope, builder, raw-topic helper, error taxonomy, fixtures, contract tests, and the unavoidable module declarations), or does it incidentally bleed into WU-0A-02 / WU-0A-03 / WU-0A-05 / WU-0A-09 / WU-0A-11 surfaces, dependency manifests, or unrelated config files?

## Findings

### WU-0A-06-MULTI-CONCERN-F01 — Diff against `main` covers only this WU's files

`git diff main --stat` reports two committed-edit lines (mod.rs declarations):

```
 src-tauri/src/contracts/mod.rs | 1 +
 src-tauri/src/events/mod.rs    | 1 +
 2 files changed, 2 insertions(+)
```

`git ls-files --others --exclude-standard` lists 13 untracked files, all WU-0A-06-scoped:

```
product-strategy/contracts/fixtures/wu-0a-06/build-errors.json
product-strategy/contracts/fixtures/wu-0a-06/build-success.json
product-strategy/contracts/fixtures/wu-0a-06/canonical-event.json
product-strategy/contracts/fixtures/wu-0a-06/invalid-events.json
product-strategy/contracts/fixtures/wu-0a-06/ipc-event-errors.json
product-strategy/contracts/fixtures/wu-0a-06/minimal-event.json
product-strategy/contracts/wu-0a-06-ipc-event.md
proposals/06-wu-0a-06.md
src-tauri/src/contracts/ipc_event.rs
src-tauri/src/events/ipc_event.rs
src-tauri/tests/ipc_event_contract.rs
src/contracts/ipc-event.ts
src/test/ipc-event.test.ts
```

Every path is either under a WU-0A-06 directory, named `*ipc-event*` / `*ipc_event*`, or sits in the proposal/contract namespace for this WU. **Single-concern.**

### WU-0A-06-MULTI-CONCERN-F02 — `mod.rs` edits are exactly the unavoidable wiring

```
diff --git a/src-tauri/src/contracts/mod.rs b/src-tauri/src/contracts/mod.rs
@@ -1,4 +1,5 @@
 pub mod event_topic;
 pub mod harness_settings;
+pub mod ipc_event;
 pub mod local_storage_layout;
 pub mod trace_context;
diff --git a/src-tauri/src/events/mod.rs b/src-tauri/src/events/mod.rs
@@ -1 +1,2 @@
+pub mod ipc_event;
 pub mod topic;
```

Both diffs are pure additions of a single `pub mod ipc_event;` line in alphabetical position. No reordering of existing modules, no rename, no visibility change to the WU-0A-02/03/05/09/11 modules. **Necessary single-line wiring, not a bundled concern.**

### WU-0A-06-MULTI-CONCERN-F03 — No incidental edits to prior-WU files

Verified by `git diff main` showing only the two `mod.rs` lines:

- WU-0A-01 (`src-tauri/src/lib.rs`, scaffold contract, package.json, tauri.conf.json): unchanged. The `phase_0a_registers_no_value_slice_commands` test still passes.
- WU-0A-02 (`src-tauri/src/contracts/harness_settings.rs`, `src-tauri/src/settings/`, harness-settings fixtures, `src/contracts/harness-settings.ts`, `src/test/harness-settings.test.ts`): unchanged.
- WU-0A-03 (`src-tauri/src/contracts/local_storage_layout.rs`, `src-tauri/src/storage/`, layout fixtures, `src/contracts/local-storage-layout.ts`): unchanged.
- WU-0A-05 (`src-tauri/src/contracts/event_topic.rs`, `src-tauri/src/events/topic.rs`, event-topic fixtures, `src/contracts/event-topic.ts`, `src/test/event-topic.test.ts`): unchanged. WU-0A-06 *consumes* `parse_event_topic` via `use crate::events::topic::parse_event_topic;` (`src-tauri/src/events/ipc_event.rs:8`) without editing it.
- WU-0A-09 (`src-tauri/src/contracts/trace_context.rs`, trace fixtures, `src/contracts/trace-context.ts`): unchanged. The optional `trace_context_id` in `IpcEvent` is preserved as an opaque string; no import of the trace-context module.
- WU-0A-11 (`src-tauri/src/contracts/pane_id.rs` if present, pane-id fixtures, `src/contracts/pane-id.ts`): unchanged.

### WU-0A-06-MULTI-CONCERN-F04 — No dependency / build-config drift

`git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock` returns no output. Existing `serde` / `serde_json` runtime deps (introduced in earlier WUs) cover the new derives; no crate added. Likewise `tsconfig.json`, `vite.config.ts`, `vitest.config.ts`, `playwright.config.ts`, `eslint.config.js`, `turbo.json`, `.prettierrc`, and `src-tauri/tauri.conf.json` are all untouched. **No bundled tooling concern.**

### WU-0A-06-MULTI-CONCERN-F05 — Test boundaries align with the ticket

Ticket test boundary: `product-strategy/contracts/wu-0a-06-ipc-event.md`, `src-tauri/src/contracts/ipc_event.rs`, `src/contracts/ipc-event.ts`, `product-strategy/contracts/fixtures/wu-0a-06/*.json`. Ticket code boundary: `src-tauri/src/events/ipc_event.rs`, `src-tauri/src/contracts/ipc_event.rs`, `src/contracts/ipc-event.ts`, `src-tauri/tests/ipc_event_contract.rs`, `src/test/ipc-event.test.ts`. The 13 untracked files plus the two `mod.rs` lines are exactly the union of these boundaries plus the proposal markdown. No file lands outside the ticket boundary.

### WU-0A-06-MULTI-CONCERN-F06 — Verification commands all pass with no warnings

| Command | Result |
| --- | --- |
| `bun run lint` | ✓ |
| `bun run typecheck` | ✓ |
| `bun run test` | ✓ 24/24 vitest |
| `cargo test --manifest-path src-tauri/Cargo.toml` | ✓ all suites green; the 4 `tests/ipc_event_contract.rs` cases plus the 1 unit test pass; the WU-0A-01 / 02 / 03 / 05 / 09 contract suites are unchanged and still pass |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | ✓ no diff |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | ✓ no warnings |

No incidental edit could regress earlier WUs because their test suites still pass byte-for-byte unchanged.

## Verdict

**LOW.** The change set is single-concern WU-0A-06: 13 new files (proposal, contract md, six fixtures, two Rust sources, one Rust integration test, one TS source, one TS test) plus two single-line `mod.rs` declarations. No incidental edits to WU-0A-01/02/03/05/09/11, no dependency or build-config drift, no global config touched. The mod.rs additions are exactly what the proposal's blast radius declared as required.
