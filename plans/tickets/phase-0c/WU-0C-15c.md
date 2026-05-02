# WU-0C-15c: SessionTurnsRead DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-15c  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 4 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-15c: SessionTurnsRead DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

SessionTurnsRead {
  invocation_id: string,
  session_id?: string,
  turn_refs: SessionTurnRef[],
  provider_name?: string,
  missing_locator_state?: string,
  substrate_gap_state?: string
}

validate_session_turns_read(read: SessionTurnsRead) -> Result<SessionTurnsRead, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `SessionTurnsRead` round-trips through serde with invocation, session, provider, turn refs, missing-locator state, and substrate-gap state preserved.
- [ ] Calling `validate_session_turns_read(read)` with documented turn refs returns the normalized DTO.
- [ ] Calling `validate_session_turns_read(read)` with `substrate_gap_state` does not require fabricated turn refs.
- [ ] Calling `validate_session_turns_read(read)` with an empty invocation ID returns the documented error.
- [ ] DTO validation does not expose raw provider transcript paths or provider-native JSONL line bodies.
- [ ] DTO validation writes no ToolCallProvenance, EvidenceArtifact, WorkerRun, OrchestratorTurn, or QuestionArtifact rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-15a, WU-0C-15b; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-20 `ToolCallProvenance`.

**Detailed dependency graph line, verbatim:** WU-0C-15c <- WU-0C-15a, WU-0C-15b, WU-0B-09, WU-0B-20

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-20

**Intra-Phase 0C predecessors:**
- WU-0C-15a
- WU-0C-15b

## Test Boundary

`product-strategy/contracts/wu-0c-15c-session-turns-read.md`, `src-tauri/src/contracts/session_turns_read.rs`

## Code Boundary

`src-tauri/src/agent_runner/session_turns_read.rs`, `src-tauri/src/contracts/session_turns_read.rs`, `src-tauri/tests/wu_0c_15c_session_turns_read_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split (identity/conflict shell + dep fixes) -> r3 Stitch Notes systematic enumeration (converged) -> r4 brownfield-cascade refactor: scope narrowed to consume SessionOverrideContract (WU-0C-N1) instead of reimplementing per-CLI session manipulation. r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Session turns read DTO consumed by WU-0C-15d, tool provenance later slices, worker evidence surfaces, and recovery.
- Parallelizable with: WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-16.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
