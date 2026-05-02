# WU-0C-14a: AgentRunnerTraceEdge DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-14a  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-14a: AgentRunnerTraceEdge DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

AgentRunnerTraceEdge {
  invocation_id: string,
  parent_invocation_id?: string,
  session_id?: string,
  evidence_ref: string
}

validate_agent_runner_trace_edge(edge: AgentRunnerTraceEdge) -> Result<AgentRunnerTraceEdge, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `AgentRunnerTraceEdge` round-trips through serde with invocation, parent, session, and evidence refs preserved.
- [ ] Calling `validate_agent_runner_trace_edge(edge)` with valid refs returns the normalized edge.
- [ ] Calling `validate_agent_runner_trace_edge(edge)` with empty invocation or evidence ref returns the documented error.
- [ ] DTO validation treats trace refs as evidence/substrate only and writes no graph or worker rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-11b; incoming WU-0B-09 `EvidenceArtifact`.

**Detailed dependency graph line, verbatim:** WU-0C-14a <- WU-0C-11b, WU-0B-09

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09

**Intra-Phase 0C predecessors:**
- WU-0C-11b

## Test Boundary

`product-strategy/contracts/wu-0c-14a-agent-runner-trace-edge.md`, `src-tauri/src/contracts/agent_runner_trace_edge.rs`

## Code Boundary

`src-tauri/src/agent_runner/trace_edge.rs`, `src-tauri/src/contracts/agent_runner_trace_edge.rs`, `src-tauri/tests/wu_0c_14a_trace_edge_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Trace edge DTO consumed by WU-0C-14b and WU-0C-14.
- Parallelizable with: WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-16a, WU-0C-16b.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
