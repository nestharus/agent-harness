# WU-0C-26a: RenderResultDto

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-26a  
**Parent initiative:** RenderEngine core  
**Implementation wave:** Wave 3 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-26a: RenderResultDto`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

RenderResultDto {
  working_set_id: string,
  graph_snapshot_id: string,
  configuration_id: string,
  rendered_blob_ref: string,
  token_estimate: i64,
  cache_prefix_hash: string,
  provider_state_id?: string,
  capability_fingerprint_id?: string,
  configuration_explanation_ref: string,
  audit_event_draft: AuditEventDraft
}

validate_render_result(result: RenderResultDto) -> Result<RenderResultDto, RenderError>
```

## Acceptance Criteria

- [ ] `RenderResultDto` round-trips through Rust serde and TypeScript fixture JSON with working set, snapshot, configuration, blob, token, cache, provider, capability, configuration explanation, and audit draft fields preserved.
- [ ] Calling `validate_render_result(result)` with valid refs returns the normalized DTO.
- [ ] Calling `validate_render_result(result)` with empty working set, rendered blob, cache prefix, or configuration explanation ref returns the documented error.
- [ ] DTO validation writes no WorkingSetSnapshot, BudgetLedger, EvidenceArtifact, or AuditEvent rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-21, WU-0C-22a, WU-0C-24; incoming WU-0B-15 `AuditEvent`, WU-0B-17 `ProviderState`, WU-0B-19 `CapabilityFingerprint`, WU-0B-21 `WorkingSetSnapshot`.

**Detailed dependency graph line, verbatim:** WU-0C-26a <- WU-0C-21, WU-0C-22a, WU-0C-24, WU-0B-15, WU-0B-17, WU-0B-19, WU-0B-21

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-15
- WU-0B-17
- WU-0B-19
- WU-0B-21

**Intra-Phase 0C predecessors:**
- WU-0C-21
- WU-0C-22a
- WU-0C-24

## Test Boundary

`product-strategy/contracts/wu-0c-26a-render-result-dto.md`, `src-tauri/src/contracts/render_result.rs`, `src/contracts/render-result.ts`

## Code Boundary

`src-tauri/src/render/render_result.rs`, `src-tauri/src/contracts/render_result.rs`, `src/contracts/render-result.ts`, `src-tauri/tests/wu_0c_26a_render_result_contract.rs`, `src/test/render-result.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Render result DTO consumed by WU-0C-26, IPC, audit, and Phase 1+ render consumers.
- Parallelizable with: WU-0C-25 after WU-0C-21 and WU-0C-24 complete.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
