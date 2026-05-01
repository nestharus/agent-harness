# WU-0C-17a: ProviderDiagnosticsRequest DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-17a  
**Parent initiative:** CLI subprocess supervisor and provider diagnostics  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-17a: ProviderDiagnosticsRequest DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

ProviderDiagnosticsRequest {
  workspace_id: string,
  provider_state_id?: string,
  config_snapshot_ref?: string
}

validate_provider_diagnostics_request(request: ProviderDiagnosticsRequest) -> Result<ProviderDiagnosticsRequest, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `ProviderDiagnosticsRequest` round-trips through serde with workspace, provider state, and config snapshot refs preserved.
- [ ] Calling `validate_provider_diagnostics_request(request)` with valid refs returns the normalized request.
- [ ] Calling `validate_provider_diagnostics_request(request)` with empty workspace ID returns the documented error.
- [ ] DTO validation mutates no ProviderState, EntitlementSnapshot, CapabilityFingerprint, or RecoveryAction rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-16b; incoming WU-0B-17 through WU-0B-19 provider/capability records.

**Detailed dependency graph line, verbatim:** WU-0C-17a <- WU-0C-16b, WU-0B-17, WU-0B-18, WU-0B-19

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-17
- WU-0B-18
- WU-0B-19

**Intra-Phase 0C predecessors:**
- WU-0C-16b

## Test Boundary

`product-strategy/contracts/wu-0c-17a-provider-diagnostics-request.md`, `src-tauri/src/contracts/provider_diagnostics_request.rs`

## Code Boundary

`src-tauri/src/agent_runner/provider_diagnostics_request.rs`, `src-tauri/src/contracts/provider_diagnostics_request.rs`, `src-tauri/tests/wu_0c_17a_provider_diagnostics_request_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Provider diagnostics request DTO consumed by WU-0C-17.
- Parallelizable with: WU-0C-23a, WU-0C-23b, WU-0C-24 after shared prerequisites are met.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
