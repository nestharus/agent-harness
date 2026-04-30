# WU-0B-02 Supported-Surface Gate

**Severity:** LOW

## Verdict

`phase_0a_scaffold_commands()` still returns exactly `["subscribe_workspace_events"]` (length 1). No new Tauri commands. No new migration file — `src-tauri/migrations/` still contains only `0001_schema_versions.sql` (the WU-0B-01 bootstrap). No durable domain table created — the contract test introspects `sqlite_master` after exercising the codec and asserts the result is empty. No new top-level Cargo dependency: `git diff main -- src-tauri/Cargo.toml` and `git diff main -- src-tauri/Cargo.lock` are both empty. No provider credentials, no `agents` invocation. The proposal's Supported Surface section is present and matches the actual blast radius.

## Per-Item Findings

### WU-0B-02-SURFACE-F01 — `phase_0a_scaffold_commands()` length still 1, still `["subscribe_workspace_events"]` (PASS)

`src-tauri/src/lib.rs` is byte-identical to `main` (no entries in `git diff main -- src-tauri/src/lib.rs`). The internal `tests` module at `:78-85` still asserts:

```rust
assert_eq!(phase_0a_scaffold_commands(), ["subscribe_workspace_events"]);
assert_eq!(registered_command_count(), phase_0a_scaffold_commands().len());
```

The Phase 0A `tauri_bootstrap_is_inert_and_command_free` contract test (`src-tauri/tests/scaffold_contract.rs`) is unchanged from `main` and asserts the same allowlist. The new WU-0B-02 contract test additionally re-asserts the same allowlist at `src-tauri/tests/graphstore_prelude_contract.rs:269` as a coverage canary. All three independent assertions pass under `cargo test`.

### WU-0B-02-SURFACE-F02 — No new Tauri commands (PASS)

`git diff main` shows zero `#[tauri::command]` additions. The diff to `src-tauri/src/lib.rs` is empty, so the `tauri::generate_handler![...]` block is byte-identical to `main`. No edits to `src-tauri/src/commands/` (the path doesn't exist on this branch — commands are inline in `lib.rs`). The new WU-0B-02 source files (`graphstore/prelude.rs`, `contracts/graphstore_prelude.rs`) declare only types, methods, and `pub fn validate_record_meta` — none of which are Tauri-wired.

### WU-0B-02-SURFACE-F03 — No new migration file (PASS)

`ls src-tauri/migrations/` returns exactly `0001_schema_versions.sql`. `git diff main -- src-tauri/migrations/` is empty. This WU is type-only and the proposal's anti-scope explicitly says "No durable domain tables and no migration file" (`proposals/0b-02-wu-0b-02.md:77`). The implementation honours this: there is no SQL DDL string in the new production source. PASS.

### WU-0B-02-SURFACE-F04 — No durable domain table created (PASS — runtime-asserted)

`grep -nE 'CREATE TABLE' src-tauri/src/graphstore/prelude.rs src-tauri/src/contracts/graphstore_prelude.rs` returns zero matches. The only `CREATE TABLE` strings introduced by this WU are inside the contract test (`src-tauri/tests/graphstore_prelude_contract.rs:179, 220`) and a `CREATE TEMP TABLE` (`:253`), all running against `sqlite::memory:` pools that are dropped at test end. The contract test `prelude_exports_no_durable_domain_table_or_value_slice_command` (`graphstore_prelude_contract.rs:242-270`) is the runtime guard:

```rust
let tables: Vec<(String,)> =
    sqlx::query_as("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
        .fetch_all(&pool)
        .await
        .expect("sqlite_master should be readable");
assert_eq!(tables, Vec::<(String,)>::new());
```

The pool used here is a fresh `sqlite::memory:` pool to which only a `TEMP TABLE` was attached and then the codec was exercised. `TEMP` tables don't appear in `sqlite_master WHERE type = 'table'`, so the assertion proves the codec exercise itself did not create any persistent table. PASS.

### WU-0B-02-SURFACE-F05 — No new Cargo dependency (PASS)

`git diff main -- src-tauri/Cargo.toml` is empty. `git diff main -- src-tauri/Cargo.lock` is empty. The implementation re-uses already-present crates: `serde`, `serde_json`, and `sqlx` (with the existing `sqlite`, `runtime-tokio`, `macros` features). `std::marker::PhantomData` and `std::hash::{Hash, Hasher}` are stdlib. No new top-level dep was needed because `serde_json` was already pulled in by the WU-0A scaffold. PASS — this is the minimum-possible dependency blast radius.

### WU-0B-02-SURFACE-F06 — No provider credentials, no `agents` invocation (PASS)

`grep -rnE 'OPENAI_API_KEY|ANTHROPIC_API_KEY|provider' src-tauri/src/graphstore/ src-tauri/src/contracts/graphstore_prelude.rs src-tauri/tests/graphstore_prelude_contract.rs` returns zero matches. `grep -rn 'agents'` against the new files returns zero matches. The new code surface contains no env-var reads, no subprocess launches, no HTTP calls, no provider config. It is a pure-types-and-validators module plus its contract.

### WU-0B-02-SURFACE-F07 — Proposal Supported Surface section matches reality (PASS)

`proposals/0b-02-wu-0b-02.md:84-92` declares:

- Deployment mode: Rust library and contract tests — matches; no UI, no Tauri command, no CLI.
- Customer cohort: later Phase 0B schema WUs and internal Rust harness tests — matches; the prelude is `pub` only inside `agent_harness_lib::graphstore` and re-exported through `agent_harness_lib::contracts::graphstore_prelude`.
- Adjacent paths: GraphStore module exports, contract module declarations, SQLx SQLite usage, WU-0A temp harness, and Phase 0A scaffold command count — matches the `mod.rs` wiring (`graphstore/mod.rs`, `contracts/mod.rs`) and the test's allowlist re-assertion.
- Blast radius: additive prelude + contract module + fixtures + proposal + tests — matches `git diff main --stat`.
- Migration path: none; this WU exports types and helpers only — matches (no migration file, no schema change).
- Rollback path: remove the added prelude module, contract module, fixtures, proposal, and tests — matches; nothing on disk persists.
- Observability: contract tests assert validation, OpaqueId rejection, error codes, JSON SQLx round-trips, timestamp ordering, command-count stability, and absence of durable domain tables — matches the six tests in `graphstore_prelude_contract.rs`.

### WU-0B-02-SURFACE-N01 — Nit: `Timestamp` is a bare `String` type alias (NIT)

`prelude.rs:12` defines `pub type Timestamp = String;`. The proposal A1 acknowledges this trade-off: "UTC `Z` RFC3339 strings are sufficient for Phase 0B metadata ordering. Invalidated by a revised contract requiring arbitrary RFC3339 offsets or typed chrono values." The validator does enforce the exact `YYYY-MM-DDTHH:MM:SSZ` shape at construction time, so the contract still holds. The risk is purely internal: a caller can move a non-validated `String` into a `RecordMeta.created_at` slot if they bypass `validate_record_meta`. Same shape as the OpaqueId-bypass-deserialize nit in the shortcut report; no action required for this WU.

## Conclusion

Severity **LOW**. Every supported-surface invariant the gate cares about is preserved: command count = 1, no new commands, no new migration file, no durable domain table (runtime-asserted), zero new dependencies, no provider credentials, no `agents` invocation. The proposal's Supported Surface section accurately describes the blast radius.
