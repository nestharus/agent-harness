# WU-0A-07 — Multi-Concern Risk Review

**Gate:** Phase 4 multi-concern PR.
**Severity:** LOW.

## Question

Is the change set strictly single-concern (only WU-0A-07 — `HarnessCommand` taxonomy + typed invoke helper)? Are there incidental edits to other WU files (WU-0A-01/02/03/04/05/06/09/11/12), unbounded `Cargo.toml` / `package.json` / `tauri.conf.json` changes, or any cross-WU refactors bundled into this slice?

## Findings

### WU-0A-07-MULTI-F01 — Tracked diff against `main` is two single-line module declarations

`git diff main --stat`:

```
 src-tauri/src/contracts/mod.rs | 1 +
 src-tauri/src/lib.rs           | 1 +
 2 files changed, 2 insertions(+)
```

`git diff main -- src-tauri/src/contracts/mod.rs src-tauri/src/lib.rs` shows only:

- `src-tauri/src/contracts/mod.rs`: `+pub mod harness_command;` inserted alphabetically.
- `src-tauri/src/lib.rs`: `+pub mod commands;` inserted alphabetically.

Both are unavoidable wiring lines — declaring the new contract module and the new (empty) commands module. No behavioral edits to either file.

### WU-0A-07-MULTI-F02 — All untracked additions sit inside the WU-0A-07 boundary

`git ls-files --others --exclude-standard` enumerates exactly the files the ticket lists in its Test boundary, Code boundary, and proposal/contract artifacts:

| Untracked file | Boundary |
| --- | --- |
| `proposals/07-wu-0a-07.md` | Proposal artifact for this WU. |
| `product-strategy/contracts/wu-0a-07-harness-command.md` | Test boundary. |
| `product-strategy/contracts/fixtures/wu-0a-07/alternate-command-names.json` | Test boundary fixtures. |
| `product-strategy/contracts/fixtures/wu-0a-07/command-errors.json` | Test boundary fixtures. |
| `product-strategy/contracts/fixtures/wu-0a-07/command-names.json` | Test boundary fixtures. |
| `product-strategy/contracts/fixtures/wu-0a-07/get-harness-settings-happy-path.json` | Test boundary fixtures. |
| `product-strategy/contracts/fixtures/wu-0a-07/invalid-ping-runtime-response.json` | Test boundary fixtures. |
| `product-strategy/contracts/fixtures/wu-0a-07/ping-runtime-happy-path.json` | Test boundary fixtures. |
| `product-strategy/contracts/fixtures/wu-0a-07/subscribe-workspace-events-happy-path.json` | Test boundary fixtures. |
| `src/contracts/harness-command.ts` | Code boundary. |
| `src/ipc/invoke-command.ts` | Code boundary. |
| `src/test/harness-command.test.ts` | Code boundary. |
| `src-tauri/src/commands/mod.rs` | Code boundary. |
| `src-tauri/src/contracts/harness_command.rs` | Code boundary. |
| `src-tauri/tests/harness_command_contract.rs` | Code boundary. |

Every untracked file maps to a ticket boundary entry. No stray files.

### WU-0A-07-MULTI-F03 — No incidental edits to other WUs' contract / fixture / test files

`git diff main` touches only `src-tauri/src/contracts/mod.rs` and `src-tauri/src/lib.rs`. Spot-checked the prior-WU files for accidental edits:

- WU-0A-01: `src-tauri/tests/scaffold_contract.rs` — untouched (`git diff main -- src-tauri/tests/scaffold_contract.rs` is empty).
- WU-0A-02: `src/contracts/harness-settings.ts`, `src-tauri/src/contracts/harness_settings.rs`, `src-tauri/src/settings/` — untouched. The new `harness-command.ts` only *imports* `parseHarnessSettings`, `HarnessSettings`; it does not modify the source files.
- WU-0A-03: `src/contracts/local-storage-layout.ts` and Rust mirror — untouched.
- WU-0A-04: `src-tauri/src/contracts/harness_app_state.rs`, `src-tauri/tests/harness_app_state_contract.rs` — untouched.
- WU-0A-05: `src/contracts/event-topic.ts`, `src-tauri/src/contracts/event_topic.rs` — untouched. The new `harness-command.ts` only *imports* `EventTopic`, `EVENT_TOPICS`.
- WU-0A-06: `src/contracts/ipc-event.ts`, `src-tauri/src/contracts/ipc_event.rs`, `src-tauri/src/events/ipc_event.rs` — untouched.
- WU-0A-09 / 0A-11 / 0A-12: no overlap with this slice; untouched.

### WU-0A-07-MULTI-F04 — `Cargo.toml` and `tauri.conf.json` have no edits

- `git diff main -- src-tauri/Cargo.toml` — empty.
- `git diff main -- src-tauri/Cargo.lock` — empty.
- `git diff main -- src-tauri/tauri.conf.json` — empty.
- `git diff main -- package.json bun.lock turbo.json` — empty.

No new dependencies, no allowlist edits, no version bumps, no script changes. The `cargo_manifest_declares_phase_0a_runtime_dependencies_without_migrations` and `workspace_manifests_match_phase_0a_contract_fixtures` assertions still pass.

### WU-0A-07-MULTI-F05 — No cross-WU refactors bundled in

The diff introduces no rename, no signature change, no reformat, and no extraction in any prior-WU file. The TS file `src/contracts/harness-command.ts` *re-uses* WU-0A-02's `parseHarnessSettings` and WU-0A-05's `EventTopic` / `EVENT_TOPICS` via plain imports (`src/contracts/harness-command.ts:1-2`); both source files are untouched.

### WU-0A-07-MULTI-F06 — Branch state is uncommitted but cleanly bounded

The implementer left the change as one tracked diff (the two `pub mod` lines) plus the untracked WU-0A-07 directory and two new files inside `src-tauri/src/`. `git log main..HEAD --oneline` returns no entries — there is no commit on the branch yet. The reviewer instructions explicitly forbid committing or pushing, so this is the expected state at gate-review time. When the implementer commits, the staged set should be exactly the 17 files enumerated in F01 + F02 (15 untracked + 2 modified) — no other files appear in the working tree (`git status` is clean apart from these).

### Verification commands

| Command | Result |
| --- | --- |
| `git diff main --stat` | 2 files, 2 insertions — both single-line `pub mod` declarations. |
| `git diff main -- src-tauri/src/lib.rs` | only `+pub mod commands;`. |
| `git diff main -- src-tauri/src/contracts/mod.rs` | only `+pub mod harness_command;`. |
| `git diff main -- src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json package.json bun.lock turbo.json` | empty. |
| `git ls-files --others --exclude-standard` | 15 files, all inside the WU-0A-07 boundary. |
| `git log main..HEAD --oneline` | empty (uncommitted). |

## Verdict

**LOW.** The PR is strictly single-concern. The two tracked file edits are both single-line `pub mod` declarations needed to wire in the new modules — the absolute minimum diff. All 15 untracked additions map directly to entries in the ticket's Test boundary, Code boundary, or proposal/contract artifact list. No prior WU's files were modified, no `Cargo.toml` / `Cargo.lock` / `tauri.conf.json` / `package.json` / `bun.lock` / `turbo.json` churn, no cross-WU refactor bundled in. Re-uses of WU-0A-02 (`HarnessSettings`) and WU-0A-05 (`EventTopic`) are import-only.
