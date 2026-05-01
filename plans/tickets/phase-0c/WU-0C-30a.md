# WU-0C-30a: RecoveryActionDraft DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-30a  
**Parent initiative:** Recovery-action writer  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-30a: RecoveryActionDraft DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

RecoveryActionDraft {
  action_type: RecoveryActionType,
  cause: RecoveryCause,
  precondition_ref: string,
  worker_run_ref?: string,
  affected_session_ids: string[],
  affected_node_ids: string[],
  affected_edit_ids: string[],
  affected_provider_state_ids: string[],
  provider_failure_cause: ProviderFailureCause,
  side_effect_classification: SideEffectClassification,
  user_confirmation_state: UserConfirmationState,
  preserved_ref?: string,
  replayed_ref?: string,
  discarded_ref?: string
}

validate_recovery_action_draft(draft: RecoveryActionDraft) -> Result<RecoveryActionDraft, RecoveryError>
```

## Acceptance Criteria

- [ ] `RecoveryActionDraft` round-trips through serde with action, cause, precondition, affected refs, side-effect, confirmation, and preserved/replayed/discarded refs preserved.
- [ ] Every RecoveryAction action type, cause, provider failure cause, side-effect classification, and confirmation-state variant from WU-0B-31 round-trips through draft fixtures.
- [ ] Calling `validate_recovery_action_draft(draft)` with valid refs returns the normalized draft.
- [ ] Calling `validate_recovery_action_draft(draft)` with an empty precondition ref or invalid high-consequence confirmation state returns the documented error.
- [ ] DTO validation writes no RecoveryAction, AuditEvent, ProviderState, OptimizerEdit, or ConflictRecord rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-17 `ProviderState`, WU-0B-26 `OptimizerEdit`, WU-0B-27 `ConflictRecord`, WU-0B-29 `WorkerRun`, WU-0B-30 `QuestionArtifact`, WU-0B-31 `RecoveryAction`.

**Detailed dependency graph line, verbatim:** WU-0C-30a <- WU-0B-17, WU-0B-26, WU-0B-27, WU-0B-29, WU-0B-30, WU-0B-31

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-17
- WU-0B-26
- WU-0B-27
- WU-0B-29
- WU-0B-30
- WU-0B-31

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-30a-recovery-action-draft.md`, `src-tauri/src/contracts/recovery_action_draft.rs`

## Code Boundary

`src-tauri/src/recovery/action_draft.rs`, `src-tauri/src/contracts/recovery_action_draft.rs`, `src-tauri/tests/wu_0c_30a_recovery_action_draft_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Recovery action draft DTO consumed by WU-0C-30 and failed-resume handoff slices.
- Parallelizable with: WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
