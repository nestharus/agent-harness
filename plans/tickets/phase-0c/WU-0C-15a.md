# WU-0C-15a: SessionTurnRef DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-15a  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-15a: SessionTurnRef DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

SessionTurnRef {
  turn_id: string,
  role: "user" | "assistant" | "tool" | "system",
  message_ref: string,
  tool_call_refs: string[],
  evidence_ref: string,
  source_offset?: string,
  source_hash?: string
}

validate_session_turn_ref(turn: SessionTurnRef) -> Result<SessionTurnRef, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `SessionTurnRef` round-trips through serde with turn, role, message, tool-call, evidence, source offset, and source hash refs preserved.
- [ ] Every role variant round-trips and is reachable through a documented transcript fixture.
- [ ] Calling `validate_session_turn_ref(turn)` with valid refs returns the normalized turn ref.
- [ ] Calling `validate_session_turn_ref(turn)` with empty turn ID, message ref, or evidence ref returns the documented error.
- [ ] DTO validation treats source offsets and hashes as opaque evidence and does not interpret Claude, Codex, opencode, or future CLI record formats.
- [ ] DTO validation creates no ToolCallProvenance, EvidenceArtifact, WorkerRun, OrchestratorTurn, or QuestionArtifact rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-09 `EvidenceArtifact`, WU-0B-20 `ToolCallProvenance`.

**Detailed dependency graph line, verbatim:** WU-0C-15a <- WU-0B-09, WU-0B-20

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-20

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-15a-session-turn-ref.md`, `src-tauri/src/contracts/session_turn_ref.rs`

## Code Boundary

`src-tauri/src/agent_runner/session_turn_ref.rs`, `src-tauri/src/contracts/session_turn_ref.rs`, `src-tauri/tests/wu_0c_15a_session_turn_ref_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split (identity/conflict shell + dep fixes) -> r3 Stitch Notes systematic enumeration (converged) -> r4 brownfield-cascade refactor: scope narrowed to consume SessionOverrideContract (WU-0C-N1) instead of reimplementing per-CLI session manipulation. r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Session turn reference DTO consumed by WU-0C-15c, tool-provenance later slices, worker evidence surfaces, and recovery.
- Parallelizable with: WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-13, WU-0C-16.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
