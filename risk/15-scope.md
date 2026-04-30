# WU-0A-15 Risk Gate: Scope-Creep

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The implementation matches the WU-0A-15 contract field exactly. `FakeAgentsFixture` carries the eleven documented fields; `ChildAcceptanceState` covers the five documented variants; `FakeAgentsFixtureError` covers the six documented variants. `install_fake_agents_fixture(handle, scenario_name)` resolves the live `HarnessAppState` through `harness_app_state(&handle)`, scopes a fixture root under the WU-0A-14a `fixture_manifest_id`, and writes the fake executable plus argv/transcript refs to disk. Every documented scenario reaches the asserted `child_acceptance_state`, and every error variant is reachable through a documented fixture. All thirteen acceptance criteria are exercised by `src-tauri/tests/fake_agents_fixture_contract.rs`. No Phase 0B/0C/1+ behavior leaked in.

## Verification

### FakeAgentsFixture DTO shape

`src-tauri/src/contracts/fake_agents_fixture.rs:5-23` declares the struct with `#[serde(deny_unknown_fields)]` and exactly the eleven contract fields: `bin_path`, `scenario_name`, `argv_log_ref`, `stdin_ref`, `stdout_ref`, `stderr_ref`, `exit_status`, `oulipoly_invocation`, `parent_invocation_id`, `session_id`, `child_acceptance_state`. Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]`. The round-trip test (`tests/fake_agents_fixture_contract.rs:51-90`) parses `fixture-round-trip.json` (all 11 fields present), re-serializes back to the original `Value`, also tests an absent-optional fixture, and asserts an extra `extra: true` key is rejected.

### ChildAcceptanceState — five variants

`fake_agents_fixture.rs:25-33` defines `ChildAcceptanceState { Unknown, Accepted, Rejected, TimedOut, Ambiguous }` with `#[serde(rename_all = "snake_case")]`. `documented_child_acceptance_states_are_reachable` (`tests/fake_agents_fixture_contract.rs:125-143`) maps each variant to a documented scenario:

| State | Scenario |
| --- | --- |
| `unknown` | `unknown-state-fixture` |
| `accepted` | `success` |
| `rejected` | `rejected-fixture` |
| `timed_out` | `timeout` |
| `ambiguous` | `cancellation` |

Each scenario installs successfully and yields the asserted state.

### FakeAgentsFixtureError — six variants

`fake_agents_fixture.rs:35-43` defines exactly `UnknownScenario`, `FixtureInstallFailed`, `ArgvLogMissing`, `TranscriptRefMissing`, `RealAgentsPathRejected`, `InvalidChildAcceptanceState`. `fake-agents-fixture-errors.json` lists the same six strings in the same order. `fake_agents_error_variants_round_trip_and_unknown_is_rejected` (`tests/fake_agents_fixture_contract.rs:92-123`) round-trips each and rejects `"OtherError"`. `documented_error_inputs_reach_every_fake_agents_error_variant` (lines 237-263) iterates the six error fixtures and asserts each reaches its documented variant.

### Documented scenarios — four behavioral scenarios

| Scenario | Verification (`tests/fake_agents_fixture_contract.rs`) |
| --- | --- |
| `success` | `success_fixture_installs_under_temp_harness_root_and_materializes_refs` (line 145) — argv recorded, stdout/stderr ref files exist, `exit_status == 0`, `child_acceptance_state == Accepted` |
| `nonzero-exit` | `nonzero_cancellation_and_timeout_scenarios_materialize_contract_refs:194-204` — argv/stdin/stdout/stderr ref files exist, `exit_status == 42`, `child_acceptance_state == Rejected` |
| `cancellation` | same test, lines 206-218 — argv recorded, `child_acceptance_state == Ambiguous`, explicit `assert_ne!(... Accepted)` |
| `timeout` | same test, lines 220-235 — `child_acceptance_state == TimedOut`, stderr transcript ref preserved with content `"timeout stderr transcript"` |

### Acceptance-criteria mapping (13/13)

| AC | Verification |
| --- | --- |
| AC1 (DTO round-trip) | `fake_agents_fixture_round_trips_stable_field_names` |
| AC2 (every state round-trips + reachable) | round-trip + `documented_child_acceptance_states_are_reachable` |
| AC3 (`success` under fixture root, not real path) | `success_fixture_installs_under_temp_harness_root_and_materializes_refs` (canonical comparison + `assert_ne!` against `REAL_AGENTS_PATH`) |
| AC4 (`success` records argv, writes stdout/stderr, exit 0, accepted) | same test |
| AC5 (`nonzero-exit` records argv/stdin/stdout/stderr + nonzero exit) | `nonzero_cancellation_and_timeout_scenarios_materialize_contract_refs:194-204` |
| AC6 (`cancellation` records argv, returns ambiguous) | same test, lines 206-218 |
| AC7 (`timeout` returns timed_out + stderr preserved) | same test, lines 220-235 |
| AC8 (`unknown` → `UnknownScenario`) | `documented_error_inputs_reach_every_fake_agents_error_variant` |
| AC9 (`FixtureInstallFailed`) | same test (`install-failure` scenario) |
| AC10 (`ArgvLogMissing`) | same test (`missing-argv-log`) |
| AC11 (`TranscriptRefMissing`) | same test (`missing-transcript-ref`) |
| AC12 (`RealAgentsPathRejected`) | same test (`real-agents-path`) |
| AC13 (`InvalidChildAcceptanceState`) | same test (`invalid-child-acceptance-state`) |

### Phase boundary

No Phase 0B+ behavior:

- No Tauri command added — `phase_0a_scaffold_commands()` length is still 1, asserted by `fake_agents_fixture_adds_no_commands_and_no_process_execution` (lines 265-280).
- No `sqlx::migrate!`, no migrations, no graph DDL.
- No `Command::new` / `std::process::Command` in either `fake_agents.rs` or `fake_agents_fixture.rs` (asserted by source-grep self-test at lines 276-279, and confirmed by external grep).
- No provider config / credentials.

### Code/test boundary alignment

Ticket code boundary: `src-tauri/src/test_harness/fake_agents.rs`, `src-tauri/src/contracts/fake_agents_fixture.rs`, `src-tauri/tests/fake_agents_fixture_contract.rs`. Test boundary: `product-strategy/contracts/wu-0a-15-fake-agents-fixture.md`, `src-tauri/src/contracts/fake_agents_fixture.rs`, `product-strategy/contracts/fixtures/wu-0a-15/*.json`. All listed files exist with the documented purpose, plus the two minimal one-line `mod.rs` wirings (`src-tauri/src/contracts/mod.rs`, `src-tauri/src/test_harness/mod.rs`) needed to expose the modules. No incidental files added.

## Findings

None above LOW.

### WU-0A-15-SCOPE-F01 — Test-only scenario manifests live in the canonical fixture directory (LOW, accepted)

**Where:** `product-strategy/contracts/fixtures/wu-0a-15/errors/error-fixture-install-failed.json`, `errors/error-argv-log-missing.json`, `errors/error-transcript-ref-missing.json`

**Detail:** Three error scenarios drive their failure modes via test-only manifest flags (`force_install_failure`, `skip_argv_log_materialization`, `skip_transcript_materialization`) read by `ScenarioManifest` (`fake_agents.rs:38-40`). These flags exist in production code paths but are only set by test fixtures; without them, no real-world scenario manifest sets them. The proposal's Assumption A3 explicitly admits this ("scenario manifests may drive test-only error paths").

**Impact:** None — this is the documented mechanism for exercising error variants without a separate test-only API surface, and the failures themselves are real I/O failures (see shortcut gate). The scaffold-command invariant is still asserted at length 1.

**Recommendation:** Accept as-is.

## Verdict

**LOW.** Eleven DTO fields, five `ChildAcceptanceState` variants, six `FakeAgentsFixtureError` variants, four behavioral scenarios, thirteen acceptance criteria — all present, all wired to documented fixtures, all exercised by tests. No Phase 0B+ leakage.
