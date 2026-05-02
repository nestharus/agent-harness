# WU-0C-07a: BudgetCheckRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-07a  
**Parent initiative:** BudgetLedger core service  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-07a: BudgetCheckRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

BudgetCheckRequest {
  workspace_id: string,
  scope_type: BudgetScopeType,
  scope_id: string,
  projected_input_tokens: i64,
  projected_output_tokens: i64,
  projected_cache_read_tokens: i64,
  projected_cache_write_tokens: i64,
  provider_state_id?: string,
  policy_set_id: string
}

validate_budget_check_request(request: BudgetCheckRequest) -> Result<BudgetCheckRequest, BudgetError>
```

## Acceptance Criteria

- [ ] `BudgetCheckRequest` round-trips through Rust serde and TypeScript fixture JSON with all projected usage and policy fields preserved.
- [ ] Calling `validate_budget_check_request(request)` with valid projected usage returns the normalized request.
- [ ] Calling `validate_budget_check_request(request)` with negative projected usage returns `BudgetError::NegativeCounter`.
- [ ] Calling `validate_budget_check_request(request)` with empty workspace, scope, or policy set ID returns the documented error.
- [ ] DTO validation writes no BudgetLedger rows and performs no policy evaluation.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-04 `PolicySet` and WU-0B-16 `BudgetLedger` field taxonomy.

**Detailed dependency graph line, verbatim:** WU-0C-07a <- WU-0B-04, WU-0B-16

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-04
- WU-0B-16

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-07a-budget-check-request.md`, `src-tauri/src/contracts/budget_check_request.rs`, `src/contracts/budget-check-request.ts`

## Code Boundary

`src-tauri/src/budget/check_request.rs`, `src-tauri/src/contracts/budget_check_request.rs`, `src/contracts/budget-check-request.ts`, `src-tauri/tests/wu_0c_07a_budget_check_request_contract.rs`, `src/test/budget-check-request.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Budget check request DTO consumed by WU-0C-07 and render budget adapters.
- Parallelizable with: WU-0C-04a, WU-0C-06a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
