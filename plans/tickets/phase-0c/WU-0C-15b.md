# WU-0C-15b: SessionTurnsRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-15b  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 3 from the Phase 0C r5 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-15b: SessionTurnsRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r5/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r5 cascade history
- Proposal cascade: `worktrees/proposal-r6/product-strategy/proposal.md` -> Option A SessionOverrideContract/agent-runner feature landing cascade where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r5/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-5 agent-runner feature cascade
- Shape reference: `worktrees/tickets-phase-0c-r2/plans/tickets/phase-0c/` -> superseded r4-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r5 artifact:

```text
schema_object: Rust struct

SessionTurnsRequest {
  invocation_id: string,
  session_id?: string,
  provider_name?: string,
  trace_ref?: string
}

validate_session_turns_request(request: SessionTurnsRequest) -> Result<SessionTurnsRequest, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `SessionTurnsRequest` round-trips through serde with invocation, session, provider, and trace refs preserved.
- [ ] Calling `validate_session_turns_request(request)` with valid invocation/session inputs returns the normalized request.
- [ ] Calling `validate_session_turns_request(request)` with missing trace/session evidence returns a documented missing-source state unless the fixture allows a substrate gap.
- [ ] DTO validation does not accept raw transcript paths, locator scripts, or provider-native JSONL records as input.
- [ ] DTO validation writes no transcript, evidence, tool provenance, worker, turn, question, or recovery rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-13b; incoming WU-0B-09 `EvidenceArtifact`.

**Detailed dependency graph line, verbatim:** WU-0C-15b <- WU-0C-13b, WU-0B-09

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09

**Intra-Phase 0C predecessors:**
- WU-0C-13b

## Test Boundary

`product-strategy/contracts/wu-0c-15b-session-turns-request.md`, `src-tauri/src/contracts/session_turns_request.rs`

## Code Boundary

`src-tauri/src/agent_runner/session_turns_request.rs`, `src-tauri/src/contracts/session_turns_request.rs`, `src-tauri/tests/wu_0c_15b_session_turns_request_contract.rs`

## Revision Rationale

Round 4 removes direct transcript-locator input from the general session-turn reader. Raw transcript location is owned by `agent-runner` and exposed to override consumers through WU-0C-N3's CLI adapter behind WU-0C-N1. r5 Option A leaves this DTO contract unchanged after the agent-runner locate/export/import-replace/pause-handshake/schema-probe features landed in the proposal-r6 / engineering-roadmap-r5 cascade.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Session turns request DTO consumed by WU-0C-15d and AgentRunnerClient.
- Parallelizable with: WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-15a, WU-0C-16.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
