# WU-0C-16a: ConfigSnapshotRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-16a  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-16a: ConfigSnapshotRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

ConfigSnapshotRequest {
  workspace_id: string,
  config_roots: string[],
  include_models: boolean,
  include_agents: boolean
}

validate_config_snapshot_request(request: ConfigSnapshotRequest) -> Result<ConfigSnapshotRequest, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `ConfigSnapshotRequest` round-trips through serde with workspace, roots, include-models, and include-agents fields preserved.
- [ ] Calling `validate_config_snapshot_request(request)` with valid roots returns the normalized request.
- [ ] Calling `validate_config_snapshot_request(request)` with path escape attempts returns `AgentRunnerError::ConfigPathRejected`.
- [ ] DTO validation reads no vendor credentials and writes no EvidenceArtifact or ProviderState rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0A-02 `HarnessSettings`, WU-0A-03 `LocalStorageLayout`.

**Detailed dependency graph line, verbatim:** WU-0C-16a <- WU-0A-02, WU-0A-03

**Phase 0A upstream WUs:**
- WU-0A-02
- WU-0A-03

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-16a-config-snapshot-request.md`, `src-tauri/src/contracts/config_snapshot_request.rs`

## Code Boundary

`src-tauri/src/agent_runner/config_snapshot_request.rs`, `src-tauri/src/contracts/config_snapshot_request.rs`, `src-tauri/tests/wu_0c_16a_config_snapshot_request_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Config snapshot request DTO consumed by WU-0C-16.
- Parallelizable with: WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-14a, WU-0C-14b.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
