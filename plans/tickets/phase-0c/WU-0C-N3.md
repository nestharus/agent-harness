# WU-0C-N3: AgentRunnerCliAdapter (SessionOverrideContract v2 implementation)

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N3  
**Parent initiative:** SessionOverrideContract v2 implementation  
**Implementation wave:** Wave 3 from the Phase 0C r5 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N3: AgentRunnerCliAdapter (SessionOverrideContract v2 implementation)`
- Audit: `worktrees/phase-0c-ai-roadmap-r5/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r5 cascade history
- Proposal cascade: `worktrees/proposal-r6/product-strategy/proposal.md` -> Option A SessionOverrideContract/agent-runner feature landing cascade where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r5/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-5 agent-runner feature cascade
- Shape reference: `worktrees/tickets-phase-0c-r2/plans/tickets/phase-0c/` -> superseded r4-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r5 artifact:

```text
schema_object: Rust service object implementing SessionOverrideContract

AgentRunnerCliAdapter::new(agents_binary, evidence_writer, audit_writer, recovery_writer) -> AgentRunnerCliAdapter

Implements:
- schema_version_probe -> agents session schema-probe
- locate_session -> agents session locate <id> [--json]
- read_transcript -> agents session export <id> [--format canonical-jsonl]
- replace_transcript -> agents session import-replace <id> --from-file <path> [--preimage-sha256 <hex>]
- truncate_after / append_turns -> read_transcript + canonical JSONL edit + replace_transcript
- get_session_metadata -> agents session locate <id> --json, mapped to the metadata subset

Atomic mid-session override:
- acquire agents session pause-handshake <id> [--ttl-ms <ms>]
- use the returned token and import-replace with --preimage-sha256
- release with agents session resume-handshake <id> --token <token>
```

## Acceptance Criteria

- [ ] Each trait method invokes the corresponding `agents session` subcommand and maps stdout/stderr JSON into the WU-0C-N2 DTOs and `SessionOverrideError` variants.
- [ ] `unsupported-storage` exit/error code from locate/export/import surfaces as `SessionOverrideError::UnsupportedStorage` and performs no harness-side transcript mutation.
- [ ] `session-busy` exit code 13 from pause-handshake or import-replace surfaces as `SessionOverrideError::SessionBusy`.
- [ ] `--preimage-sha256` mismatch surfaces as `SessionOverrideError::PreimageMismatch`.
- [ ] `pause-handshake` lease TTL is respected for atomic mid-session override, and `resume-handshake` is always called on adapter drop when a lease token is held.
- [ ] Adapter construction calls `agents session schema-probe` and refuses operation if the output is malformed, required feature flags are absent, or `safe_for_import_replace` is false.
- [ ] `truncate_after` and `append_turns` compose `read_transcript`, a canonical JSONL boundary edit, and `replace_transcript`; if `agent-runner` gains finer-grained surfaces later, only this adapter mapping changes.
- [ ] The fake `agents` binary fixture covers success, unsupported storage, busy lease, preimage mismatch, unsafe schema probe, malformed JSON, and missing subcommand cases.
- [ ] The adapter records invocation evidence through WU-0B-09, override/audit receipts through WU-0B-15-compatible drafts, and write-failure recovery metadata through WU-0B-31 refs.
- [ ] The adapter never opens or writes `agent-runner` `state.db`, provider transcript files, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, or cross-provider migration settings directly.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-N1, WU-0C-N2; incoming WU-0A-15 `FakeAgentsFixture`, Phase 0B WU-0B-09 `EvidenceArtifact` for `agents` invocation logs, WU-0B-15 `AuditEvent` for override receipts, and WU-0B-31 `RecoveryAction` for write failures.

**Detailed dependency graph line, verbatim:** WU-0C-N3 <- WU-0C-N1, WU-0C-N2, WU-0A-15, WU-0B-09, WU-0B-15, WU-0B-31

**Phase 0A upstream WUs:**
- WU-0A-15

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-15
- WU-0B-31

**Intra-Phase 0C predecessors:**
- WU-0C-N1
- WU-0C-N2

## Test Boundary

`src-tauri/tests/agent_runner_cli_adapter_contract.rs`

## Code Boundary

`src-tauri/src/session_override/agent_runner_cli_adapter.rs`, contract test, integration test against a fake `agents` binary fixture reusing the WU-0A-15 fake_agents pattern.

## Revision Rationale

r4 introduced WU-0C-N3 as a v1 `AgentRunnerDbAdapter` with direct `state.db` and JSONL writes pinned by WU-0C-N4. r5 Option A rescopes it to the v2 `AgentRunnerCliAdapter` after agent-runner feature requests landed in the proposal-r6 / engineering-roadmap-r5 cascade (commits #14-#23). The schema probe is now the `agents session schema-probe` call inside this adapter rather than a separate WU. Classification: externally driven `fix-created-family` gen 0. This clears the former blocked-on annotations because the required agent-runner features landed.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Shipping `SessionOverrideContract` implementation consumed by VS-010, VS-012, VS-018, VS-020, and VS-021.
- Parallelizable with: WU-0C-04, WU-0C-12, WU-0C-13, WU-0C-14b, WU-0C-15b, WU-0C-16, WU-0C-N5, WU-0C-22, WU-0C-25a, WU-0C-26a, WU-0C-31a, and WU-0C-37 after WU-0C-N1/N2 land.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
