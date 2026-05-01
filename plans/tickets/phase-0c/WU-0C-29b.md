# WU-0C-29b: OptimizerCycleLease DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-29b  
**Parent initiative:** Optimizer scheduler  
**Implementation wave:** Wave 7 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-29b: OptimizerCycleLease DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

OptimizerCycleLease {
  lease_id: string,
  workspace_id: string,
  optimizer_request_id: string,
  cycle_id: string,
  claimed_at: Timestamp,
  expires_at: Timestamp
}

validate_optimizer_cycle_lease(lease: OptimizerCycleLease) -> Result<OptimizerCycleLease, OptimizerError>
```

## Acceptance Criteria

- [ ] `OptimizerCycleLease` round-trips through serde with lease, workspace, request, cycle, claim, and expiry fields preserved.
- [ ] Calling `validate_optimizer_cycle_lease(lease)` with `expires_at > claimed_at` returns the normalized lease.
- [ ] Calling `validate_optimizer_cycle_lease(lease)` with empty IDs or expired-at-before-claimed-at returns the documented error.
- [ ] DTO validation does not claim requests, transition cycles, draft edits, or mutate graph truth.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-28; incoming WU-0B-25 `OptimizerRequest`.

**Detailed dependency graph line, verbatim:** WU-0C-29b <- WU-0C-28, WU-0B-25

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-25

**Intra-Phase 0C predecessors:**
- WU-0C-28

## Test Boundary

`product-strategy/contracts/wu-0c-29b-optimizer-cycle-lease.md`, `src-tauri/src/contracts/optimizer_cycle_lease.rs`

## Code Boundary

`src-tauri/src/optimizer/cycle_lease.rs`, `src-tauri/src/contracts/optimizer_cycle_lease.rs`, `src-tauri/tests/wu_0c_29b_optimizer_cycle_lease_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Optimizer cycle lease DTO consumed by WU-0C-29.
- Parallelizable with: WU-0C-31a, WU-0C-33a after their prerequisites.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
