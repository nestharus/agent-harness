# WU-0C-15d: SessionTurnsReader

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-15d  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 5 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-15d: SessionTurnsReader`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust service object

SessionTurnsReader::read_session_turns(request: SessionTurnsRequest) -> Result<SessionTurnsRead, AgentRunnerError>
```

## Acceptance Criteria

- [ ] Calling `read_session_turns(request)` for a fake `agents` normalized session-turn fixture returns stable turn refs with evidence refs and source hashes.
- [ ] Calling `read_session_turns(request)` for a fixture with parent/sidechain/compaction metadata preserves that metadata through opaque refs without parsing provider-native JSONL.
- [ ] Calling `read_session_turns(request)` for missing transcript-location evidence returns explicit `missing_locator_state` rather than failing silently.
- [ ] Calling `read_session_turns(request)` for a substrate-gap fixture records explicit `substrate_gap_state` and does not fabricate turn refs.
- [ ] The reader is read-only and does not create ToolCallProvenance, EvidenceArtifact, WorkerRun, OrchestratorTurn, QuestionArtifact, or SessionOverrideStore rows.
- [ ] The reader never calls `replace_transcript`, `truncate_after`, or `append_turns`; session mutation is exclusive to WU-0C-N1 implementers.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-13, WU-0C-15a, WU-0C-15b, WU-0C-15c; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-20 `ToolCallProvenance`.

**Detailed dependency graph line, verbatim:** WU-0C-15d <- WU-0C-13, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0B-09, WU-0B-20

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-20

**Intra-Phase 0C predecessors:**
- WU-0C-13
- WU-0C-15a
- WU-0C-15b
- WU-0C-15c

## Test Boundary

`product-strategy/contracts/wu-0c-15d-session-turns-reader.md`, `src-tauri/src/contracts/session_turns_reader.rs`

## Code Boundary

`src-tauri/src/agent_runner/session_turns_reader.rs`, `src-tauri/src/contracts/session_turns_reader.rs`, `src-tauri/tests/wu_0c_15d_session_turns_reader_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split (identity/conflict shell + dep fixes) -> r3 Stitch Notes systematic enumeration (converged) -> r4 brownfield-cascade refactor: scope narrowed to consume SessionOverrideContract (WU-0C-N1) instead of reimplementing per-CLI session manipulation. r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Read-only session turn ingestion contract consumed by tool provenance later slices, worker evidence surfaces, and recovery.
- Parallelizable with: WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-16.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
