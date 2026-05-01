# WU-0C-24a: CachePrefixInput DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-24a  
**Parent initiative:** RenderEngine core and cache locality  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-24a: CachePrefixInput DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

CachePrefixInput {
  target_cli: string,
  target_model: string,
  policy_set_id: string,
  configuration_id: string,
  graph_snapshot_id: string,
  stable_prefix_parts: string[]
}

validate_cache_prefix_input(input: CachePrefixInput) -> Result<CachePrefixInput, RenderError>
```

## Acceptance Criteria

- [ ] `CachePrefixInput` round-trips through Rust serde and TypeScript fixture JSON with target CLI, target model, policy set, configuration, snapshot, and stable prefix parts preserved.
- [ ] Calling `validate_cache_prefix_input(input)` with valid refs and non-empty stable prefix parts returns the normalized DTO.
- [ ] Calling `validate_cache_prefix_input(input)` with an empty stable prefix returns `RenderError::EmptyPrefix`.
- [ ] DTO validation reads no provider credentials and writes no BudgetLedger or WorkingSetSnapshot rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-04 `PolicySet`, WU-0B-05 `GraphConfiguration`, WU-0B-14 `GraphSnapshot`.

**Detailed dependency graph line, verbatim:** WU-0C-24a <- WU-0B-04, WU-0B-05, WU-0B-14

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-04
- WU-0B-05
- WU-0B-14

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-24a-cache-prefix-input.md`, `src-tauri/src/contracts/cache_prefix_input.rs`, `src/contracts/cache-prefix-input.ts`

## Code Boundary

`src-tauri/src/render/cache_prefix_input.rs`, `src-tauri/src/contracts/cache_prefix_input.rs`, `src/contracts/cache-prefix-input.ts`, `src-tauri/tests/wu_0c_24a_cache_prefix_input_contract.rs`, `src/test/cache-prefix-input.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Cache prefix input DTO consumed by WU-0C-24 and render budget accounting.
- Parallelizable with: WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-30a.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
