# WU-0C-N4: AgentRunnerSchemaProbe

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N4  
**Parent initiative:** SessionOverrideContract v1 compatibility probe  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N4: AgentRunnerSchemaProbe`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust service object

SchemaProbe {
  agents_binary_ref: string,
  agents_version_or_commit: string,
  state_db_path: string,
  supported: boolean,
  supported_range: string,
  table_fingerprints: map<string, string>,
  storage_capabilities: map<string, "read_write" | "read_only" | "unsupported">,
  refusal_reason?: SessionOverrideError
}

AgentRunnerSchemaProbe::probe(state_db_path: string, agents_binary: string) -> Result<SchemaProbe, SessionOverrideError>
```

## Acceptance Criteria

- [ ] Calling `probe(state_db_path, agents_binary)` against a fixture with `invocations`, `session_turns`, `session_chains`, and `session_chain_segments` matching the pinned range returns `supported = true`.
- [ ] The probe validates required columns for `invocations.session_id`, `invocations.session_capture_method`, `invocations.resume_acceptance_status`, `session_turns.provider_name`, `session_turns.session_id`, `session_turns.turn_id`, `session_turns.parent_turn_id`, `session_turns.is_sidechain`, `session_turns.is_compaction_boundary`, `session_turns.source_file`, `session_chains.chain_id`, and `session_chain_segments.ended_at`.
- [ ] Missing required tables, missing required columns, incompatible indexes, or unreadable DB files return `SessionOverrideError::UnsupportedSchema` or `DbFailure` before any write-capability is reported.
- [ ] Unknown or unparseable `agents_binary` version/commit returns `UnsupportedSchema` unless the fixture explicitly marks a test-only fake binary.
- [ ] The probe reports `claude_code_jsonl` and `codex_jsonl` storage as `read_write`, `read_only`, or `unsupported` only from the pinned fixture capability table; it does not infer support heuristically from path names.
- [ ] Probe output includes deterministic table fingerprints for all required tables and indexes so audit records can cite the checked surface.
- [ ] The service opens SQLite read-only and performs no schema migration, `PRAGMA user_version` update, table creation, transcript write, or agent-runner config edit.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-N2; incoming WU-0A-03 `LocalStorageLayout`, WU-0A-15 `FakeAgentsFixture`.

**Detailed dependency graph line, verbatim:** WU-0C-N4 <- WU-0C-N2, WU-0A-03, WU-0A-15

**Phase 0A upstream WUs:**
- WU-0A-03
- WU-0A-15

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- WU-0C-N2

## Test Boundary

`product-strategy/contracts/wu-0c-n4-agent-runner-schema-probe.md`, `src-tauri/src/contracts/agent_runner_schema_probe.rs`

## Code Boundary

`src-tauri/src/session_override/schema_probe.rs`, `src-tauri/src/contracts/agent_runner_schema_probe.rs`, `src-tauri/tests/wu_0c_n4_agent_runner_schema_probe_contract.rs`

## Blocked-on

**Blocked-on:** v1 ships with local table/binary probing. Replacement by an upstream supported-surface probe is blocked on `agents schema-version probe` or equivalent.

## Revision Rationale

Phase 0C r4 brownfield-cascade addition: introduced by SessionOverrideContract axiom from proposal-r5 + engineering-roadmap-r4. Owns one of the 5 new contract concerns (trait / DTOs / v1 adapter / schema probe / override registry). r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Risk gates LOW/LOW/LOW (decomp / coverage / dep). Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Runtime compatibility probe consumed by WU-0C-N3 and v1 refusal paths in WU-0C-N1 tests.
- Parallelizable with: WU-0C-12a, WU-0C-13a, WU-0C-14a, WU-0C-16b after WU-0C-N2 lands.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
- Preserve the `**Blocked-on:**` annotation above in implementation handoffs and PR descriptions until the named `agents` features land.
