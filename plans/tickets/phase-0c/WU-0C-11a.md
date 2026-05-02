# WU-0C-11a: AgentSpawnRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-11a  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-11a: AgentSpawnRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

AgentSpawnRequest {
  agents_binary: string,
  model: string,
  project_dir: string,
  prompt_file: string,
  stdin_ref?: string,
  env: map<string, string>,
  timeout_ms: u64,
  parent_invocation_id?: string,
  resume_session_id?: string,
  route_constraints_ref?: string
}

validate_spawn_request(request: AgentSpawnRequest) -> Result<AgentSpawnRequest, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `AgentSpawnRequest` round-trips through serde while preserving agents binary, model, project dir, prompt file, stdin ref, env, timeout, parent invocation, optional resume session, and route-constraints refs.
- [ ] The documented argv fixture renders as an `agents` invocation over model/project/prompt inputs; it does not render provider CLI commands such as `claude`, `codex`, or `opencode` directly.
- [ ] Calling `validate_spawn_request(request)` with valid inputs returns the normalized request.
- [ ] Calling `validate_spawn_request(request)` with empty `agents_binary`, `model`, `project_dir`, `prompt_file`, or zero timeout returns the documented error variant.
- [ ] Spawn request validation accepts only harness route constraints as opaque refs and never selects provider account, provider alias, quota window, auth profile, resume strategy, or cross-provider porting behavior.
- [ ] Spawn request validation does not parse stderr, launch agents, open transcript files, or write ProviderState, WorkerRun, OrchestratorTurn, QuestionArtifact, or RecoveryAction rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0A-02 `HarnessSettings`, WU-0A-03 `LocalStorageLayout`, WU-0A-09 `TraceContext`, WU-0B-19 `CapabilityFingerprint`.

**Detailed dependency graph line, verbatim:** WU-0C-11a <- WU-0A-02, WU-0A-03, WU-0A-09, WU-0B-19

**Phase 0A upstream WUs:**
- WU-0A-02
- WU-0A-03
- WU-0A-09

**Phase 0B upstream WUs:**
- WU-0B-19

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-11a-agent-spawn-request.md`, `src-tauri/src/contracts/agent_spawn_request.rs`

## Code Boundary

`src-tauri/src/agent_runner/spawn_request.rs`, `src-tauri/src/contracts/agent_spawn_request.rs`, `src-tauri/tests/wu_0c_11a_agent_spawn_request_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split (identity/conflict shell + dep fixes) -> r3 Stitch Notes systematic enumeration (converged) -> r4 brownfield-cascade refactor: scope narrowed to consume SessionOverrideContract (WU-0C-N1) instead of reimplementing per-CLI session manipulation. r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Spawn request DTO consumed by subprocess supervisor, hook payloads, plugin capability matrix, and AgentRunnerClient.
- Parallelizable with: WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-19, WU-0C-21, WU-0C-24, WU-0C-30.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
