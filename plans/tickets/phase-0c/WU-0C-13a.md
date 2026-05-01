# WU-0C-13a: SessionCaptureRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-13a  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-13a: SessionCaptureRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

SessionCaptureRequest {
  invocation_id: string,
  stdout_ref?: string,
  stderr_ref?: string,
  trace_ref?: string,
  agent_runner_session_ref?: string
}

validate_session_capture_request(request: SessionCaptureRequest) -> Result<SessionCaptureRequest, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `SessionCaptureRequest` round-trips through serde with invocation, stdout/stderr, trace, and agent-runner session refs preserved.
- [ ] Calling `validate_session_capture_request(request)` with a documented `agents` output/source combination returns the normalized request.
- [ ] Calling `validate_session_capture_request(request)` with missing source refs returns `AgentRunnerError::MissingCaptureSource` unless the fixture allows `substrate_gap`.
- [ ] DTO validation does not run transcript locator scripts, inspect provider-specific JSONL, choose provider/account routes, or compose resume commands.
- [ ] DTO validation writes no session, worker, turn, question, or recovery rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-11a, WU-0C-11b; incoming WU-0A-15 `FakeAgentsFixture`.

**Detailed dependency graph line, verbatim:** WU-0C-13a <- WU-0C-11a, WU-0C-11b, WU-0A-15

**Phase 0A upstream WUs:**
- WU-0A-15

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- WU-0C-11a
- WU-0C-11b

## Test Boundary

`product-strategy/contracts/wu-0c-13a-session-capture-request.md`, `src-tauri/src/contracts/session_capture_request.rs`

## Code Boundary

`src-tauri/src/agent_runner/session_capture_request.rs`, `src-tauri/src/contracts/session_capture_request.rs`, `src-tauri/tests/wu_0c_13a_session_capture_request_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split (identity/conflict shell + dep fixes) -> r3 Stitch Notes systematic enumeration (converged) -> r4 brownfield-cascade refactor: scope narrowed to consume SessionOverrideContract (WU-0C-N1) instead of reimplementing per-CLI session manipulation. r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Session capture request DTO consumed by WU-0C-13.
- Parallelizable with: WU-0C-12a, WU-0C-14a, WU-0C-14b, WU-0C-16a, WU-0C-16b.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
