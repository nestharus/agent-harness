# WU-0C-09a: ConfigurationRevisionDraft DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-09a  
**Parent initiative:** ConfigurationRegistry runtime  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-09a: ConfigurationRevisionDraft DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

ConfigurationRevisionDraft {
  workspace_id: string,
  base_configuration_id: string,
  changes: JsonObject,
  actor: ActorRef,
  reason: string
}

validate_configuration_revision_draft(draft: ConfigurationRevisionDraft) -> Result<ConfigurationRevisionDraft, ConfigurationError>
```

## Acceptance Criteria

- [ ] `ConfigurationRevisionDraft` round-trips through Rust serde and TypeScript fixture JSON with workspace, base configuration, changes, actor, and reason preserved.
- [ ] Calling `validate_configuration_revision_draft(draft)` with valid changes returns the normalized draft.
- [ ] Calling `validate_configuration_revision_draft(draft)` with empty workspace, base configuration, or reason returns the documented `ConfigurationError` variant.
- [ ] DTO validation writes no GraphConfiguration, GraphWorkspace, OptimizerRequest, AuditEvent, or IPC event rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-05 `GraphConfiguration`, WU-0B-06 `GraphWorkspace`, WU-0B-15 `AuditEvent` draft shape.

**Detailed dependency graph line, verbatim:** WU-0C-09a <- WU-0B-05, WU-0B-06, WU-0B-15

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-05
- WU-0B-06
- WU-0B-15

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-09a-configuration-revision-draft.md`, `src-tauri/src/contracts/configuration_revision_draft.rs`, `src/contracts/configuration-revision-draft.ts`

## Code Boundary

`src-tauri/src/configuration/revision_draft.rs`, `src-tauri/src/contracts/configuration_revision_draft.rs`, `src/contracts/configuration-revision-draft.ts`, `src-tauri/tests/wu_0c_09a_configuration_revision_draft_contract.rs`, `src/test/configuration-revision-draft.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Configuration revision draft DTO consumed by WU-0C-09 and later configuration inspector surfaces.
- Parallelizable with: WU-0C-04a, WU-0C-06a, WU-0C-07a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
