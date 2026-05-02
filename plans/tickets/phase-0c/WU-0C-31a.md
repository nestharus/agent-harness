# WU-0C-31a: RecoveryProcessorDecision DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-31a  
**Parent initiative:** RecoveryAction processor skeleton  
**Implementation wave:** Wave 3 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-31a: RecoveryProcessorDecision DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

RecoveryProcessorDecision {
  recovery_action_id: string,
  processor_state: "detected" | "classified" | "preflight" | "requires_user" | "blocked_execution_deferred",
  next_required_input?: string,
  audit_event_draft?: AuditEventDraft
}

validate_recovery_processor_decision(decision: RecoveryProcessorDecision) -> Result<RecoveryProcessorDecision, RecoveryError>
```

## Acceptance Criteria

- [ ] `RecoveryProcessorDecision` round-trips through serde with action, state, next input, and audit draft fields preserved.
- [ ] Every processor state variant is reachable through a documented fixture.
- [ ] Calling `validate_recovery_processor_decision(decision)` with valid state/input combinations returns the normalized DTO.
- [ ] Calling `validate_recovery_processor_decision(decision)` with `processor_state = requires_user` and no next required input returns the documented error.
- [ ] DTO validation executes no rollback, retry, resume, cancel, quarantine, substitution, or audit append side effects.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-30; incoming WU-0B-15 `AuditEvent`, WU-0B-31 `RecoveryAction`.

**Detailed dependency graph line, verbatim:** WU-0C-31a <- WU-0C-30, WU-0B-15, WU-0B-31

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-15
- WU-0B-31

**Intra-Phase 0C predecessors:**
- WU-0C-30

## Test Boundary

`product-strategy/contracts/wu-0c-31a-recovery-processor-decision.md`, `src-tauri/src/contracts/recovery_processor_decision.rs`

## Code Boundary

`src-tauri/src/recovery/processor_decision.rs`, `src-tauri/src/contracts/recovery_processor_decision.rs`, `src-tauri/tests/wu_0c_31a_recovery_processor_decision_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Recovery processor decision DTO consumed by WU-0C-31 and failed-resume UX slices.
- Parallelizable with: WU-0C-29a, WU-0C-29b, WU-0C-33a after prerequisites.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
