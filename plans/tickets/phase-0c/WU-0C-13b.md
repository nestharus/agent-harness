# WU-0C-13b: AgentRunnerSessionCapture DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-13b  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-13b: AgentRunnerSessionCapture DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

AgentRunnerSessionCapture {
  invocation_id: string,
  session_id?: string,
  provider_name?: string,
  capture_method: "agents_reported_session" | "trace_session" | "state_db_session" | "substrate_gap",
  source_ref: string,
  capture_state: "captured" | "missing" | "ambiguous" | "substrate_gap"
}

validate_agent_runner_session_capture(capture: AgentRunnerSessionCapture) -> Result<AgentRunnerSessionCapture, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `AgentRunnerSessionCapture` round-trips through serde with all fields preserved.
- [ ] Every capture method and capture state variant round-trips and is reachable through a documented `agents` fixture.
- [ ] Calling `validate_agent_runner_session_capture(capture)` with valid capture state/session combinations returns the normalized DTO.
- [ ] Calling `validate_agent_runner_session_capture(capture)` with `capture_state = captured` and no `session_id` returns the documented error.
- [ ] Calling `validate_agent_runner_session_capture(capture)` with `provider_name` present preserves it as observed runner evidence and does not use it to select or reroute accounts.
- [ ] DTO validation writes no WorkerRun, OrchestratorTurn, QuestionArtifact, or RecoveryAction rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-11b; incoming WU-0A-15 `FakeAgentsFixture`.

**Detailed dependency graph line, verbatim:** WU-0C-13b <- WU-0C-11b, WU-0A-15

**Phase 0A upstream WUs:**
- WU-0A-15

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- WU-0C-11b

## Test Boundary

`product-strategy/contracts/wu-0c-13b-agent-runner-session-capture.md`, `src-tauri/src/contracts/agent_runner_session_capture.rs`

## Code Boundary

`src-tauri/src/agent_runner/session_capture_dto.rs`, `src-tauri/src/contracts/agent_runner_session_capture.rs`, `src-tauri/tests/wu_0c_13b_agent_session_capture_dto_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split (identity/conflict shell + dep fixes) -> r3 Stitch Notes systematic enumeration (converged) -> r4 brownfield-cascade refactor: scope narrowed to consume SessionOverrideContract (WU-0C-N1) instead of reimplementing per-CLI session manipulation. r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Captured session DTO consumed by WU-0C-13, WU-0C-15b, AgentRunnerClient, WorkerRun later slices, and question routing.
- Parallelizable with: WU-0C-12a, WU-0C-14a, WU-0C-14b, WU-0C-16a, WU-0C-16b.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
