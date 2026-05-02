# WU-0C-14b: AgentRunnerTraceTree DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-14b  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 3 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-14b: AgentRunnerTraceTree DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

AgentRunnerTraceTree {
  root_invocation_id: string,
  invocation_edges: AgentRunnerTraceEdge[],
  evidence_ref: string
}

validate_agent_runner_trace_tree(tree: AgentRunnerTraceTree) -> Result<AgentRunnerTraceTree, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `AgentRunnerTraceTree` round-trips through serde with root invocation, edge list, and evidence ref preserved.
- [ ] Calling `validate_agent_runner_trace_tree(tree)` with a connected parent-before-child fixture returns the normalized tree.
- [ ] Calling `validate_agent_runner_trace_tree(tree)` with a missing root edge or cycle returns the documented parse/validation error.
- [ ] DTO validation writes no GraphNode, WorkerRun, OrchestratorTurn, or RecoveryAction rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-14a; incoming WU-0B-09 `EvidenceArtifact`.

**Detailed dependency graph line, verbatim:** WU-0C-14b <- WU-0C-14a, WU-0B-09

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09

**Intra-Phase 0C predecessors:**
- WU-0C-14a

## Test Boundary

`product-strategy/contracts/wu-0c-14b-agent-runner-trace-tree.md`, `src-tauri/src/contracts/agent_runner_trace_tree.rs`

## Code Boundary

`src-tauri/src/agent_runner/trace_tree_dto.rs`, `src-tauri/src/contracts/agent_runner_trace_tree.rs`, `src-tauri/tests/wu_0c_14b_trace_tree_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Trace tree DTO consumed by WU-0C-14, AgentRunnerClient, worker board later surfaces, recovery, and subprocess event emission.
- Parallelizable with: WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-16a, WU-0C-16b.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
