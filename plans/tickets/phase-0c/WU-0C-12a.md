# WU-0C-12a: AgentSpawnResult DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-12a  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-12a: AgentSpawnResult DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

AgentSpawnResult {
  invocation_id: string,
  parent_invocation_id?: string,
  session_id?: string,
  stdout_ref: string,
  stderr_ref: string,
  exit_status?: i32,
  acceptance_state: "unknown" | "accepted" | "rejected" | "timed_out" | "ambiguous"
}

validate_agent_spawn_result(result: AgentSpawnResult) -> Result<AgentSpawnResult, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `AgentSpawnResult` round-trips through serde and preserves stdout/stderr refs, exit status, invocation IDs, optional session ID, and acceptance state.
- [ ] Every acceptance-state variant round-trips and is reachable through a WU-0A-15 fake `agents` scenario.
- [ ] Calling `validate_agent_spawn_result(result)` with valid refs returns the normalized DTO.
- [ ] Calling `validate_agent_spawn_result(result)` with empty invocation ID, stdout ref, or stderr ref returns the documented error.
- [ ] DTO validation does not spawn, wait for, or cancel subprocesses.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-11b; incoming WU-0A-15 `FakeAgentsFixture`.

**Detailed dependency graph line, verbatim:** WU-0C-12a <- WU-0C-11b, WU-0A-15

**Phase 0A upstream WUs:**
- WU-0A-15

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- WU-0C-11b

## Test Boundary

`product-strategy/contracts/wu-0c-12a-agent-spawn-result.md`, `src-tauri/src/contracts/agent_spawn_result.rs`

## Code Boundary

`src-tauri/src/agent_runner/spawn_result.rs`, `src-tauri/src/contracts/agent_spawn_result.rs`, `src-tauri/tests/wu_0c_12a_agent_spawn_result_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Spawn result DTO consumed by WU-0C-12 and the AgentRunnerClient facade.
- Parallelizable with: WU-0C-02, WU-0C-03, WU-0C-06a, WU-0C-09a, WU-0C-13a, WU-0C-13b, WU-0C-16a, WU-0C-16b.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
