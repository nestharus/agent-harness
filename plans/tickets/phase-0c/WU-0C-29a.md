# WU-0C-29a: OptimizerScheduleDecision DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-29a  
**Parent initiative:** Optimizer scheduler  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-29a: OptimizerScheduleDecision DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

OptimizerScheduleDecision {
  decision: "run" | "skip_no_work" | "skip_budget" | "skip_policy",
  request_ids: string[],
  budget_decision?: BudgetDecision,
  policy_decision?: PolicyDecision<JsonValue>
}

validate_optimizer_schedule_decision(decision: OptimizerScheduleDecision) -> Result<OptimizerScheduleDecision, OptimizerError>
```

## Acceptance Criteria

- [ ] `OptimizerScheduleDecision` round-trips through serde with decision, request IDs, budget decision, and policy decision preserved.
- [ ] Every decision variant is reachable through a documented fixture.
- [ ] Calling `validate_optimizer_schedule_decision(decision)` rejects `run` decisions with no request IDs and accepts skip decisions with documented empty-request fixtures.
- [ ] DTO validation launches no optimizer, drafts no edits, and mutates no graph truth.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-01, WU-0C-05; incoming WU-0B-25 `OptimizerRequest`.

**Detailed dependency graph line, verbatim:** WU-0C-29a <- WU-0C-01, WU-0C-05, WU-0B-25

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-25

**Intra-Phase 0C predecessors:**
- WU-0C-01
- WU-0C-05

## Test Boundary

`product-strategy/contracts/wu-0c-29a-optimizer-schedule-decision.md`, `src-tauri/src/contracts/optimizer_schedule_decision.rs`

## Code Boundary

`src-tauri/src/optimizer/schedule_decision.rs`, `src-tauri/src/contracts/optimizer_schedule_decision.rs`, `src-tauri/tests/wu_0c_29a_optimizer_schedule_decision_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Scheduler decision DTO consumed by WU-0C-29 and later reviewer sampling.
- Parallelizable with: WU-0C-31a, WU-0C-33a after their prerequisites.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
