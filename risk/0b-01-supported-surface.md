# WU-0B-01 Supported-Surface Gate

**Severity:** LOW

## Verdict

`phase_0a_scaffold_commands()` still returns exactly `["subscribe_workspace_events"]` (length 1). The sqlx `migrate` feature is **not** enabled in `Cargo.toml`, and the renamed scaffold-contract test still asserts that rejection. No new Tauri commands. No Phase 0B domain tables — only `schema_versions`. No provider credentials, no `agents` invocation introduced by this WU. The only new dependency is `sha2 = "0.10.9"`, pinned, and the only new line in `Cargo.lock`'s package-deps section is `+ "sha2"` (no transitive crates added — `sha2` was already pulled in transitively via `sqlx`). The proposal's Supported Surface section is present and matches the actual blast radius.

## Per-Item Findings

### WU-0B-01-SURFACE-F01 — `phase_0a_scaffold_commands()` still length 1, still `["subscribe_workspace_events"]` (PASS)

`src-tauri/src/lib.rs:31-37` defines `phase_0a_scaffold_commands` and `registered_command_count`. The internal `tests` module at `:78-85` asserts:

```rust
assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
assert_eq!(registered_command_count(), phase_0a_scaffold_commands().len());
```

The contract test `tauri_bootstrap_is_inert_and_command_free` (`src-tauri/tests/scaffold_contract.rs:139-187`) is unchanged from `main` and asserts the same allowlist plus the absence of `get_harness_settings` / `ping_runtime` registrations. Both pass under `cargo test`.

### WU-0B-01-SURFACE-F02 — No new Tauri commands (PASS)

The diff contains no `#[tauri::command]` additions and no edits to `src-tauri/src/commands/`. The only `lib.rs` change is `+ pub mod graphstore;`. `git diff main -- src-tauri/src/lib.rs` shows exactly one inserted line; the `tauri::generate_handler![...]` block is byte-identical.

### WU-0B-01-SURFACE-F03 — sqlx `migrate` feature not enabled (PASS)

`grep -n 'features.*=.*"migrate"' src-tauri/Cargo.toml` returns no matches. `grep -n 'migrate' src-tauri/Cargo.toml` returns no matches. The sqlx feature list at `src-tauri/Cargo.toml:21` is `["sqlite", "runtime-tokio", "macros"]` — unchanged from `main`. The renamed scaffold-contract test (`cargo_manifest_declares_phase_0a_runtime_dependencies_without_sqlx_migrate_feature`, `src-tauri/tests/scaffold_contract.rs:64-137`) still asserts at `:117-122`:

```rust
assert!(!features.iter().any(|feature| feature.as_str() == Some("migrate")), "WU-0A-01 must not enable sqlx migration ownership");
```

so both the runtime feature surface and the contract assertion are preserved.

### WU-0B-01-SURFACE-F04 — Only `schema_versions` durable table; no Phase 0B domain tables (PASS)

`grep -rn 'CREATE TABLE' src-tauri/migrations/` returns exactly:

```
src-tauri/migrations/0001_schema_versions.sql:1:CREATE TABLE IF NOT EXISTS schema_versions (
```

No `graph_workspace`, `graph_node`, `graph_edge`, or any other Phase 0B domain table appears anywhere on the branch. The contract test `shipped_wu_0b_01_migration_emits_only_schema_versions` (`graphstore_migrations_contract.rs:531-549`) is a runtime guard: after running the shipped `src-tauri/migrations` directory against a fresh temp pool it asserts `durable_tables == vec!["schema_versions"]` (filtering `sqlite_*` internals).

### WU-0B-01-SURFACE-F05 — No provider credentials, no `agents` invocation (PASS)

`grep -rn 'agents'` against `src-tauri/` finds only **pre-existing** Phase 0A references in `src-tauri/src/test_harness/{fake_agents.rs, temp_harness.rs}`, `src-tauri/tests/fake_agents_fixture_contract.rs`, etc. None of those files are in the WU-0B-01 diff, and the new files (`graphstore/migrations.rs`, `graphstore_migrations.rs` contract, the new fixtures, `graphstore_migrations_contract.rs`) contain zero `agents` references. No env-var reads of `OPENAI_API_KEY`/`ANTHROPIC_API_KEY` and no provider config introduced.

### WU-0B-01-SURFACE-F06 — `sha2` is the only new dep, pinned, transitive blast radius zero (PASS)

`git diff main -- src-tauri/Cargo.toml` shows exactly one inserted line: `+sha2 = "0.10.9"`. `git diff main -- src-tauri/Cargo.lock` is 12 lines total and adds exactly one entry to the `agent-harness` package's `dependencies` array (`+ "sha2"`). No new `[[package]]` blocks are added — `sha2 0.10.9` was already a transitive dep of `sqlx` / `sqlx-sqlite` and is now also a direct dep. The 0.10.x line is the current stable series of a widely-used and well-audited crate. PASS.

### WU-0B-01-SURFACE-F07 — Proposal Supported Surface track matches reality (PASS)

`proposals/0b-01-wu-0b-01.md:95-103` declares:

- Deployment mode: Rust library + contract tests — matches.
- Customer cohort: later Phase 0B WUs and internal Rust harness tests — matches (no UI / no Tauri-command surface).
- Adjacent paths: WU-0A SQLite pool, temp harness, scaffold-command count, contract fixtures — matches the actual touchpoints (`temp_harness_state` is reused in `temp_pool` in the contract test; the scaffold-command count is asserted unchanged).
- Blast radius: additive GraphStore module + contract DTO module + migration SQL + fixtures + tests + one bounded scaffold-contract assertion update — matches `git diff --stat`.
- Migration path: fresh DB applies 0001; existing DB without `schema_versions` bootstraps via the same runner — matches the runner logic at `migrations.rs:140-142` (`schema_versions_exists` returns false → empty `applied` map → bootstrap is applied normally).
- Rollback path: WU-0B-01 supports no destructive rollback; remove module + migration file before release if rejected — matches.
- Observability: contract tests inspect rows / report vectors / exact errors / enum round-trips / table absence — matches.

### WU-0B-01-SURFACE-N01 — Nit: assumption A4 cites a ticket clause that is not literally in the ticket text (NIT)

The proposal's assumption A4 says "Hand-rolled migrations are acceptable because the ticket explicitly allows them. Evidence: ticket notes either `sqlx::migrate` or hand-rolled is fine if justified." The ticket as visible in `tickets-phase-0b:plans/tickets/phase-0b/WU-0B-01.md` contains no such "either/or" clause; it leaves the choice implicit. The hand-rolled choice is still defensible because (a) the source WU contract names a runtime API rather than a sqlx-macro and (b) the design avoids enabling the `migrate` feature that the Phase 0A scaffold-contract test still rejects. No action required — the choice is sound; the citation is just slightly loose.

## Conclusion

Severity **LOW**. Every supported-surface invariant the gate cares about is preserved: command count = 1, no new commands, sqlx `migrate` feature still off and still asserted off, no Phase 0B domain tables, no provider credentials, no `agents` invocation, single bounded `sha2` dep. The proposal's Supported Surface section accurately describes the change.
