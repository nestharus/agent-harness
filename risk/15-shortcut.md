# WU-0A-15 Risk Gate: Shortcut / Placeholder

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

Every contract behavior is backed by real disk I/O, not stubs. The fake executable is genuinely written under the canonical fixture root with `0o755` permissions; argv/transcript ref files are real JSON/text files written during install; `FixtureInstallFailed` is triggered by a real `fs::write`-onto-a-directory failure; `ArgvLogMissing` and `TranscriptRefMissing` arise from real `Path::exists()` checks against deliberately-skipped writes; `RealAgentsPathRejected` is enforced both structurally (string equality before any I/O) and post-install (canonical-path comparison). No `Command::new` / `std::process::Command` anywhere in the WU's source. No TODOs / `unimplemented!()` / placeholder strings.

## Verification

### Bin path is canonically under the temp harness fixture root

`fake_agents.rs:278-297` (`validate_installed_bin_path`):

```rust
let canonical_root = fs::canonicalize(fixture_root)?;
let canonical_bin = fs::canonicalize(bin_path)?;

if let Ok(real_agents) = fs::canonicalize(REAL_AGENTS_PATH) {
    if canonical_bin == real_agents {
        return Err(FakeAgentsFixtureError::RealAgentsPathRejected);
    }
}
if !canonical_bin.starts_with(canonical_root) {
    return Err(FakeAgentsFixtureError::FixtureInstallFailed);
}
```

This is a true canonicalize-then-compare, not string-prefix. The `success` test (`tests/fake_agents_fixture_contract.rs:155-163`) re-runs the same `fs::canonicalize(...).starts_with(...)` assertion from outside.

### `RealAgentsPathRejected` guard catches direct, indirect, and symlink-disguised attempts

`fake_agents.rs:299-308` (`is_real_agents_path`):

```rust
fn is_real_agents_path(path: &Path) -> bool {
    if path == Path::new(REAL_AGENTS_PATH) { return true; }
    match (fs::canonicalize(path), fs::canonicalize(REAL_AGENTS_PATH)) {
        (Ok(candidate), Ok(real_agents)) => candidate == real_agents,
        _ => false,
    }
}
```

This is invoked at `resolve_bin_path:211` against the requested `bin_path` *before* any write. The two-phase check covers:

- Direct string match (`/home/nes/.local/bin/agents` literally) — matches the `path == Path::new(REAL_AGENTS_PATH)` branch.
- Symlink that resolves to the real path — caught by `fs::canonicalize`.
- Indirect/relative path that resolves to the real path — also caught by `fs::canonicalize` (which resolves relative to cwd).

In addition, post-install `validate_installed_bin_path:287-291` re-canonicalizes the actual written path and re-checks against the canonical real-agents path. So even if a fixture root somehow contained a symlink pointing at the real binary, the post-install check would still fire `RealAgentsPathRejected`.

A third defense layer: `is_safe_relative_path` (line 233-236) rejects any non-`Component::Normal` (`..`, `.`, prefixes, root) before joining. That stops `..`-traversal from a relative bin path.

### `argv_log_ref` points to a REAL file written during install

`fake_agents.rs:66-72` resolves the argv-log path under the fixture root, then (unless `skip_argv_log_materialization` is set by a test-only manifest) calls `write_json_ref(&argv_log_ref, &manifest.recorded_argv)`, which `serde_json::to_string_pretty` then `fs::write`s. The post-write existence check at line 70 (`if !argv_log_ref.exists() { return Err(ArgvLogMissing) }`) is a real disk check. The `success` test (`tests/fake_agents_fixture_contract.rs:171-174`) reads the file back and asserts `["agents", "run", "--scenario", "success"]`.

### `stdout_ref` / `stderr_ref` are REAL written files

`fake_agents.rs:321-340` (`materialize_optional_text_ref`): when the manifest declares a ref and `skip_materialization == false`, it `fs::create_dir_all(parent)` and `fs::write(&resolved, content)`. The `success` test (lines 175-186) and the nonzero/cancellation/timeout test (lines 201-204, 218, 230-235) read these files back from disk via `Path::exists()` and (for timeout) `fs::read_to_string`.

### `FixtureInstallFailed` is a REAL file-write failure

`fake_agents.rs:271-276` (`force_fixture_install_failure`):

```rust
let blocked_path = fixture_root.join("install-failure-blocker");
fs::create_dir_all(&blocked_path)?;
fs::write(&blocked_path, b"this write targets a directory")?
```

Creates a directory at a path, then attempts to `fs::write` to that *same path* — Linux returns `EISDIR`, which propagates as `FixtureInstallFailed`. This is genuine OS-level I/O failure, not a stub `Err(...)`.

### `ArgvLogMissing` / `TranscriptRefMissing` are REAL missing-file checks

- `ArgvLogMissing` (line 70-72): existence is checked via `argv_log_ref.exists()` against the actual filesystem. The `missing-argv-log` test fixture skips the write step, so the file genuinely is not present on disk when the check runs.
- `TranscriptRefMissing` (line 342-350, called at lines 93-94): `Path::new(ref_path).exists()` against the actual disk. The `missing-transcript-ref` fixture sets `skip_transcript_materialization: true` so `fs::write` is never called for the declared refs.

These are not stub `Err(...)` shortcuts; they observe real disk state.

### `InvalidChildAcceptanceState` is reached via real fixture-JSON inspection

`fake_agents.rs:182-201` (`load_scenario_manifest` + `validate_child_acceptance_state_value`): the manifest is parsed as a generic `serde_json::Value`, then the `child_acceptance_state` string is inspected against the documented allow-list (`unknown` | `accepted` | `rejected` | `timed_out` | `ambiguous`). The `error-invalid-child-acceptance-state.json` fixture supplies `"handshake_maybe"` — a real JSON string outside the allow-list — which fails the check and returns `InvalidChildAcceptanceState`. The pre-validation step is necessary because raw `serde_json::from_value` failure would otherwise be coerced into `FixtureInstallFailed`; the inspect-then-deserialize ordering preserves error fidelity. Reading the `Value` and comparing the string is a genuine data check on real fixture content, not an artificial branch.

(Note: this is closer to "explicit allow-list inspection on real JSON" than "let serde reject the string"; both arrive at the same correct error code, and the implementation choice is documented under proposal Assumption A3.)

### `exit_status` is whatever the manifest declares

`fake_agents.rs:103` and `fake_executable_body:264-269` both pull `exit_status` from `ScenarioManifest`. Tests assert exact values (success: 0, nonzero-exit: 42, cancellation: 130, timeout: 124) — no forced default.

### NO binary execution

External grep over `src-tauri/src/test_harness/fake_agents.rs` and `src-tauri/src/contracts/fake_agents_fixture.rs` for `Command::new`, `std::process::`, `process::Command` returns zero matches. The test `fake_agents_fixture_adds_no_commands_and_no_process_execution` (lines 265-280) re-asserts this with a source-grep self-check. The fake executable is *written* (with `0o755` permissions on Unix at lines 249-258) but never spawned or invoked — `fake_executable_body:264-269` is just `format!("#!/bin/sh\n# WU-0A-15 fake agents fixture: {}\nexit {}\n", ...)` and is content for `fs::write`.

### No TODOs / `unimplemented!()` / placeholder code

External grep across the WU's source files for `TODO`, `FIXME`, `unimplemented!`, `todo!`, `panic!`, `unreachable!` returns zero hits.

## Findings

None above LOW.

## Verdict

**LOW.** Every contract surface is backed by real disk I/O. The path safety guard canonicalizes (catching direct, indirect, and symlink attempts), the install-failure scenario triggers a real OS-level write error, the missing-ref scenarios genuinely leave files off-disk, and there is no subprocess execution anywhere in the WU. No stubs, no TODOs.
