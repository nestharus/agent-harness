# WU-0C-37a: AuditEmitInput DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-37a  
**Parent initiative:** Audit event writer subscription and emit  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-37a: AuditEmitInput DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

AuditEmitInput {
  workspace_id: string,
  audit_event_draft: AuditEventDraft,
  trace_context_id?: string,
  emit_topic?: EventTopic
}

validate_audit_emit_input(input: AuditEmitInput) -> Result<AuditEmitInput, AuditEmitError>
```

## Acceptance Criteria

- [ ] `AuditEmitInput` round-trips through serde with workspace ID, audit event draft, trace context, and optional emit topic preserved.
- [ ] Calling `validate_audit_emit_input(input)` with valid draft refs returns the normalized DTO.
- [ ] Calling `validate_audit_emit_input(input)` with empty workspace ID or unknown emit topic returns the documented error.
- [ ] DTO validation appends no AuditEvent row and emits no IPC event.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-15 `AuditEvent`, WU-0A-09 `TraceContext`, WU-0A-05 `EventTopic`.

**Detailed dependency graph line, verbatim:** WU-0C-37a <- WU-0B-15, WU-0A-05, WU-0A-09

**Phase 0A upstream WUs:**
- WU-0A-05
- WU-0A-09

**Phase 0B upstream WUs:**
- WU-0B-15

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-37a-audit-emit-input.md`, `src-tauri/src/contracts/audit_emit_input.rs`

## Code Boundary

`src-tauri/src/audit/audit_emit_input.rs`, `src-tauri/src/contracts/audit_emit_input.rs`, `src-tauri/tests/wu_0c_37a_audit_emit_input_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Audit emit input DTO consumed by WU-0C-37 and Phase 0C service emitters.
- Parallelizable with: WU-0C-36a after WU-0C-35 if UI and audit files are disjoint.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
