# WU-0C-17b: ProviderDiagnosticsResult DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-17b  
**Parent initiative:** CLI subprocess supervisor and provider diagnostics  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-17b: ProviderDiagnosticsResult DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

ProviderDiagnosticsResult {
  provider_state_id?: string,
  capability_fingerprint_id?: string,
  diagnostics_ref: string,
  route_denial_reasons: string[]
}

validate_provider_diagnostics_result(result: ProviderDiagnosticsResult) -> Result<ProviderDiagnosticsResult, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `ProviderDiagnosticsResult` round-trips through serde with provider state, capability fingerprint, diagnostics ref, and denial reasons preserved.
- [ ] Calling `validate_provider_diagnostics_result(result)` with valid diagnostics returns the normalized result.
- [ ] Route denial reason taxonomy values are preserved exactly; unknown values are retained as unknown observations.
- [ ] DTO validation mutates no ProviderState, EntitlementSnapshot, CapabilityFingerprint, or RecoveryAction rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-17 through WU-0B-19 provider/capability records.

**Detailed dependency graph line, verbatim:** WU-0C-17b <- WU-0B-17, WU-0B-18, WU-0B-19

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-17
- WU-0B-18
- WU-0B-19

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-17b-provider-diagnostics-result.md`, `src-tauri/src/contracts/provider_diagnostics_result.rs`

## Code Boundary

`src-tauri/src/agent_runner/provider_diagnostics_result.rs`, `src-tauri/src/contracts/provider_diagnostics_result.rs`, `src-tauri/tests/wu_0c_17b_provider_diagnostics_result_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Provider diagnostics result DTO consumed by WU-0C-17 and ProviderStateMonitor.
- Parallelizable with: WU-0C-23a, WU-0C-23b, WU-0C-24 after shared prerequisites are met.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
