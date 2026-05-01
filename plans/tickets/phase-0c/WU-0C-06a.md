# WU-0C-06a: BudgetUsageDraft DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-06a  
**Parent initiative:** BudgetLedger core service  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-06a: BudgetUsageDraft DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

BudgetUsageDraft {
  workspace_id: string,
  scope_type: BudgetScopeType,
  scope_id: string,
  input_tokens: i64,
  output_tokens: i64,
  cache_read_tokens: i64,
  cache_write_tokens: i64,
  latency_ms: i64,
  provider_cost_estimate: decimal,
  provider_state_id?: string,
  cache_prefix_hash?: string
}

validate_budget_usage_draft(draft: BudgetUsageDraft) -> Result<BudgetUsageDraft, BudgetError>
```

## Acceptance Criteria

- [ ] `BudgetUsageDraft` round-trips through Rust serde and TypeScript fixture JSON with all counter, provider, scope, and cache-prefix fields preserved.
- [ ] Calling `validate_budget_usage_draft(draft)` with valid non-negative counters returns the normalized DTO.
- [ ] Calling `validate_budget_usage_draft(draft)` with negative token, latency, or cost values returns `BudgetError::NegativeCounter`.
- [ ] DTO validation writes no BudgetLedger rows and evaluates no budget policy.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-16 `BudgetLedger` field taxonomy and WU-0B-17 `ProviderState` refs.

**Detailed dependency graph line, verbatim:** WU-0C-06a <- WU-0B-16, WU-0B-17

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-16
- WU-0B-17

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-06a-budget-usage-draft.md`, `src-tauri/src/contracts/budget_usage_draft.rs`, `src/contracts/budget-usage-draft.ts`

## Code Boundary

`src-tauri/src/budget/usage_draft.rs`, `src-tauri/src/contracts/budget_usage_draft.rs`, `src/contracts/budget-usage-draft.ts`, `src-tauri/tests/wu_0c_06a_budget_usage_draft_contract.rs`, `src/test/budget-usage-draft.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Usage draft DTO consumed by WU-0C-06 and render/budget integrations.
- Parallelizable with: WU-0C-04a, WU-0C-07a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
