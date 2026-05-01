# WU-0C-22a: RenderPolicy DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-22a  
**Parent initiative:** RenderEngine core  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-22a: RenderPolicy DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

RenderPolicy {
  render_policy_id: string,
  max_nodes: i64,
  max_evidence_items: i64,
  max_depth: i64,
  token_ceiling: i64,
  required_pin_behavior: "fail_closed" | "allow_exception"
}

validate_render_policy(policy: RenderPolicy) -> Result<RenderPolicy, RenderError>
```

## Acceptance Criteria

- [ ] `RenderPolicy` round-trips through Rust serde and TypeScript fixture JSON with all caps and required-pin behavior preserved.
- [ ] Every `required_pin_behavior` variant round-trips and is reachable through configuration fixtures.
- [ ] Calling `validate_render_policy(policy)` with positive caps returns the normalized DTO.
- [ ] Calling `validate_render_policy(policy)` with negative or zero caps returns `RenderError::InvalidRenderPolicy`.
- [ ] DTO validation creates no render snapshots, audit rows, or configuration provenance rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-21; incoming WU-0B-05 `GraphConfiguration`.

**Detailed dependency graph line, verbatim:** WU-0C-22a <- WU-0C-21, WU-0B-05

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-05

**Intra-Phase 0C predecessors:**
- WU-0C-21

## Test Boundary

`product-strategy/contracts/wu-0c-22a-render-policy.md`, `src-tauri/src/contracts/render_policy.rs`, `src/contracts/render-policy.ts`

## Code Boundary

`src-tauri/src/render/render_policy.rs`, `src-tauri/src/contracts/render_policy.rs`, `src/contracts/render-policy.ts`, `src-tauri/tests/wu_0c_22a_render_policy_contract.rs`, `src/test/render-policy.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Resolved render policy DTO consumed by WU-0C-22, privilege filtering, budget adapter, and RenderEngine core.
- Parallelizable with: WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-16.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
