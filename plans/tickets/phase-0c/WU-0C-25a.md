# WU-0C-25a: RenderBudgetRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-25a  
**Parent initiative:** RenderEngine plus BudgetLedger integration  
**Implementation wave:** Wave 3 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-25a: RenderBudgetRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

RenderBudgetRequest {
  render_request: RenderRequestDto,
  projected_token_estimate: i64,
  projected_cache_read_tokens: i64,
  projected_cache_write_tokens: i64,
  cache_prefix_hash: string,
  provider_state_id?: string
}

validate_render_budget_request(request: RenderBudgetRequest) -> Result<RenderBudgetRequest, RenderError>
```

## Acceptance Criteria

- [ ] `RenderBudgetRequest` round-trips through Rust serde and TypeScript fixture JSON with render request, projected tokens, cache tokens, cache prefix hash, and provider state preserved.
- [ ] Calling `validate_render_budget_request(request)` with non-negative projected usage returns the normalized DTO.
- [ ] Calling `validate_render_budget_request(request)` with negative projected tokens returns `RenderError::BudgetInputInvalid`.
- [ ] DTO validation writes no BudgetLedger, WorkingSetSnapshot, or AuditEvent rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-21, WU-0C-24; incoming WU-0B-17 `ProviderState`.

**Detailed dependency graph line, verbatim:** WU-0C-25a <- WU-0C-21, WU-0C-24, WU-0B-17

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-17

**Intra-Phase 0C predecessors:**
- WU-0C-21
- WU-0C-24

## Test Boundary

`product-strategy/contracts/wu-0c-25a-render-budget-request.md`, `src-tauri/src/contracts/render_budget_request.rs`, `src/contracts/render-budget-request.ts`

## Code Boundary

`src-tauri/src/render/budget_request.rs`, `src-tauri/src/contracts/render_budget_request.rs`, `src/contracts/render-budget-request.ts`, `src-tauri/tests/wu_0c_25a_render_budget_request_contract.rs`, `src/test/render-budget-request.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Render budget request DTO consumed by WU-0C-25 and RenderEngine core.
- Parallelizable with: WU-0C-23 after WU-0C-07 and WU-0C-21 complete.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
