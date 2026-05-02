# WU-0C-35a: DomainEventDraft DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-35a  
**Parent initiative:** Channel event emission backbone  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-35a: DomainEventDraft DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

DomainEventDraft {
  workspace_id: string,
  topic: EventTopic,
  payload: JsonValue,
  trace_context_id?: string,
  audit_event_ref?: string
}

validate_domain_event_draft(draft: DomainEventDraft) -> Result<DomainEventDraft, IpcEventError>
```

## Acceptance Criteria

- [ ] `DomainEventDraft` round-trips through Rust serde and TypeScript fixture JSON with workspace, topic, payload, trace context, and audit ref preserved.
- [ ] Calling `validate_domain_event_draft(draft)` with every documented EventTopic returns the normalized DTO.
- [ ] Calling `validate_domain_event_draft(draft)` with unknown topic or empty workspace ID returns the documented error.
- [ ] DTO validation does not emit events, subscribe listeners, or decide domain semantics.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0A-05 `EventTopic`, WU-0A-09 `TraceContext`, WU-0B-15 `AuditEvent`.

**Detailed dependency graph line, verbatim:** WU-0C-35a <- WU-0A-05, WU-0A-09, WU-0B-15

**Phase 0A upstream WUs:**
- WU-0A-05
- WU-0A-09

**Phase 0B upstream WUs:**
- WU-0B-15

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-35a-domain-event-draft.md`, `src-tauri/src/contracts/domain_event_draft.rs`, `src/contracts/domain-event-draft.ts`

## Code Boundary

`src-tauri/src/ipc/domain_event_draft.rs`, `src-tauri/src/contracts/domain_event_draft.rs`, `src/contracts/domain-event-draft.ts`, `src-tauri/tests/wu_0c_35a_domain_event_draft_contract.rs`, `src/test/domain-event-draft.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Domain event draft DTO consumed by WU-0C-35 and all event-emitting Phase 0C services.
- Parallelizable with: WU-0C-34 after shared registration files are coordinated.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
