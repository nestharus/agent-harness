# WU-0C-23a: RenderPrivilegeFilterInput DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-23a  
**Parent initiative:** RenderEngine core  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-23a: RenderPrivilegeFilterInput DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

RenderPrivilegeFilterInput {
  graph_snapshot_id: string,
  node_ids: string[],
  evidence_pointer_ids: string[],
  target_actor: "orchestrator" | "worker",
  policy_set_id: string
}

validate_render_privilege_filter_input(input: RenderPrivilegeFilterInput) -> Result<RenderPrivilegeFilterInput, RenderError>
```

## Acceptance Criteria

- [ ] `RenderPrivilegeFilterInput` round-trips through Rust serde and TypeScript fixture JSON with snapshot, node, evidence, actor, and policy refs preserved.
- [ ] Calling `validate_render_privilege_filter_input(input)` with valid refs returns the normalized DTO.
- [ ] Calling `validate_render_privilege_filter_input(input)` with unknown actor or empty snapshot/policy refs returns the documented error.
- [ ] DTO validation writes no graph, evidence, working-set, policy decision, or audit rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-07 `GraphNode`, WU-0B-10 `ProvenancePointer`, WU-0B-11 `SummaryContract`, WU-0B-14 `GraphSnapshot`.

**Detailed dependency graph line, verbatim:** WU-0C-23a <- WU-0B-07, WU-0B-10, WU-0B-11, WU-0B-14

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-07
- WU-0B-10
- WU-0B-11
- WU-0B-14

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-23a-render-privilege-filter-input.md`, `src-tauri/src/contracts/render_privilege_filter_input.rs`, `src/contracts/render-privilege-filter-input.ts`

## Code Boundary

`src-tauri/src/render/privilege_filter_input.rs`, `src-tauri/src/contracts/render_privilege_filter_input.rs`, `src/contracts/render-privilege-filter-input.ts`, `src-tauri/tests/wu_0c_23a_render_privilege_filter_input_contract.rs`, `src/test/render-privilege-filter-input.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Privilege filter input DTO consumed by WU-0C-23.
- Parallelizable with: WU-0C-17a, WU-0C-17b, WU-0C-20a, WU-0C-24 after policy prerequisites.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
