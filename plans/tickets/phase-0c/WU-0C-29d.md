# WU-0C-29d: ConflictRecordWriterShell

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-29d  
**Parent initiative:** Identity/conflict shell  
**Implementation wave:** Wave 5 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-29d: ConflictRecordWriterShell`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust service object

ConflictRecordWriterShell::record_conflict(record: ConflictRecord) -> Result<ConflictRecord, IdentityConflictError>
ConflictRecordWriterShell::mark_conflict_state(conflict_record_id: string, state: ConflictState, reason: string) -> Result<ConflictRecord, IdentityConflictError>

Routing invariant:
- Phase 0C records conflict workflow shell state only; it does not resolve conflicts, merge edits, mutate topology, or launch optimizer/recovery execution.
```

## Acceptance Criteria

- [ ] Calling `record_conflict(record)` with every WU-0B-27 conflict-type variant writes exactly one durable ConflictRecord row and returns it with all Phase 0B fields preserved.
- [ ] Calling `record_conflict(record)` with unknown identity, edit, snapshot, provider, worker, question, or budget refs returns the documented error and writes no row.
- [ ] Calling `mark_conflict_state(id, state, reason)` exercises every WU-0B-27 documented workflow state transition and rejects invalid transitions with a documented error.
- [ ] The routing invariant holds across identity, content, summary, configuration, edge, worker-overlap, question-route, tool-protocol, provider-state, and budget conflict fixtures: no GraphNode, GraphEdge, NodeRevision, OptimizerEdit, IdentityEvent, RecoveryAction, or AuditEvent row is mutated.
- [ ] The service emits only audit event drafts; durable audit writing is delegated to WU-0C-37.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-04, WU-0C-29c; incoming WU-0B-13 `IdentityEvent`, WU-0B-15 `AuditEvent`, WU-0B-25 `OptimizerRequest`, WU-0B-26 `OptimizerEdit`, WU-0B-27 `ConflictRecord`, WU-0B-29 `WorkerRun`, WU-0B-30 `QuestionArtifact`, WU-0B-31 `RecoveryAction`.

**Detailed dependency graph line, verbatim:** WU-0C-29d <- WU-0C-04, WU-0C-29c, WU-0B-13, WU-0B-15, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-29, WU-0B-30, WU-0B-31

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-13
- WU-0B-15
- WU-0B-25
- WU-0B-26
- WU-0B-27
- WU-0B-29
- WU-0B-30
- WU-0B-31

**Intra-Phase 0C predecessors:**
- WU-0C-04
- WU-0C-29c

## Test Boundary

`product-strategy/contracts/wu-0c-29d-conflict-record-writer-shell.md`, `src-tauri/src/contracts/conflict_record_writer_shell.rs`

## Code Boundary

`src-tauri/src/identity/conflict_record_writer_shell.rs`, `src-tauri/src/contracts/conflict_record_writer_shell.rs`, `src-tauri/tests/wu_0c_29d_conflict_record_writer_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Conflict-record write shell consumed by VS-012, VS-013, VS-017, VS-018, VS-020, and VS-021.
- Parallelizable with: WU-0C-31a, WU-0C-33a after WU-0C-04/WU-0C-29c.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
