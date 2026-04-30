# WU-0A-15 Contract: FakeAgentsFixture Subprocess Double

## Ownership

This WU owns the Rust `FakeAgentsFixture` DTO and `install_fake_agents_fixture(handle, scenario_name)` test-harness builder. It installs fake executable scenarios under a temp harness fixture root and materializes argv/transcript reference files. It does not own Tauri commands, frontend code, GraphStore migrations, provider configuration, or subprocess execution.

## DTO

`FakeAgentsFixture` serializes as a JSON object with exactly these fields:

- `bin_path`: required string.
- `scenario_name`: required string.
- `argv_log_ref`: required string.
- `stdin_ref`: optional string.
- `stdout_ref`: optional string.
- `stderr_ref`: optional string.
- `exit_status`: required integer.
- `oulipoly_invocation`: optional string.
- `parent_invocation_id`: optional string.
- `session_id`: optional string.
- `child_acceptance_state`: required string enum, one of `unknown`, `accepted`, `rejected`, `timed_out`, or `ambiguous`.

Unknown DTO fields are rejected by Rust serde.

## Entrypoint

```text
install_fake_agents_fixture(
  handle: TempHarnessHandle,
  scenario_name: string
) -> Result<FakeAgentsFixture, FakeAgentsFixtureError>
```

`scenario_name` must be present in the internal scenario registry. The builder resolves the live `HarnessAppState` from `TempHarnessHandle`, derives a fixture root under `storage_layout.fixture_root/<fixture_manifest_id>`, writes the fake executable and ref files, validates declared refs, and returns a populated `FakeAgentsFixture`.

The documented valid scenarios are:

- `success`
- `nonzero-exit`
- `cancellation`
- `timeout`
- `unknown-state-fixture`
- `rejected-fixture`

The name `unknown` is intentionally absent from the registry and returns `UnknownScenario`.

## Path Safety

The default executable path is:

```text
<temp harness fixture_root>/<fixture_manifest_id>/fake-agents-<scenario_name>
```

The builder rejects `/home/nes/.local/bin/agents` structurally and does not write to that path. After writing a fake executable, it canonicalizes the fixture root and bin path and asserts the bin path remains under the temp harness fixture root and is not equal to the canonical real agents path if that real path exists.

## Errors

`FakeAgentsFixtureError` serializes as one of these exact strings:

- `UnknownScenario`
- `FixtureInstallFailed`
- `ArgvLogMissing`
- `TranscriptRefMissing`
- `RealAgentsPathRejected`
- `InvalidChildAcceptanceState`

Behavior:

- Unknown scenario names return `UnknownScenario`.
- Fixture-root or file write failures return `FixtureInstallFailed`.
- Declared-but-missing argv log refs return `ArgvLogMissing`.
- Declared-but-missing stdout/stderr refs return `TranscriptRefMissing`.
- Any scenario requesting `/home/nes/.local/bin/agents` returns `RealAgentsPathRejected`.
- Unknown child acceptance state strings in fixture JSON return `InvalidChildAcceptanceState`.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-15/`:

- `scenario-registry.json`: documented scenario names and manifest paths.
- `manifests/success.json`
- `manifests/nonzero-exit.json`
- `manifests/cancellation.json`
- `manifests/timeout.json`
- `manifests/unknown-state-fixture.json`
- `manifests/rejected-fixture.json`
- `fixture-round-trip.json`: canonical DTO fixture with all 11 fields present.
- `bin-path-canonicalization.json`: asserts `success` bin path is under the temp harness fixture root.
- `real-agents-path-rejected.json`: scenario requesting `/home/nes/.local/bin/agents`.
- `fake-agents-fixture-errors.json`: all documented error variants.
- `errors/error-unknown-scenario.json`
- `errors/error-fixture-install-failed.json`
- `errors/error-argv-log-missing.json`
- `errors/error-transcript-ref-missing.json`
- `errors/error-real-agents-path-rejected.json`
- `errors/error-invalid-child-acceptance-state.json`

## Test Handoff

Rust contract test: `src-tauri/tests/fake_agents_fixture_contract.rs`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
