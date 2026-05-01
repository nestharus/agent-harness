# WU-0C-20a: ProviderStateMonitorResult DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-20a  
**Parent initiative:** ProviderStateMonitor  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-20a: ProviderStateMonitorResult DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

ProviderStateMonitorResult {
  probe_id: string,
  provider_state_id: string,
  entitlement_snapshot_ids: string[],
  capability_fingerprint_ids: string[],
  audit_event_draft: AuditEventDraft,
  redacted_evidence_ref: string,
  freshness_state: "fresh" | "stale" | "probe_failed" | "manual"
}

validate_provider_state_monitor_result(result: ProviderStateMonitorResult) -> Result<ProviderStateMonitorResult, ProviderMonitorError>
```

## Acceptance Criteria

- [ ] `ProviderStateMonitorResult` round-trips through Rust serde and TypeScript fixture JSON with probe, provider state, entitlement, capability, audit draft, evidence ref, and freshness fields preserved.
- [ ] Every freshness state variant round-trips and is reachable through a documented fixture.
- [ ] Calling `validate_provider_state_monitor_result(result)` with valid IDs returns the normalized DTO.
- [ ] Calling `validate_provider_state_monitor_result(result)` with empty probe ID, provider state ID, or evidence ref returns the documented error.
- [ ] DTO validation stores no secrets and writes no provider, entitlement, capability, evidence, or audit rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-19; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-17 `ProviderState`, WU-0B-18 `EntitlementSnapshot`, WU-0B-19 `CapabilityFingerprint`.

**Detailed dependency graph line, verbatim:** WU-0C-20a <- WU-0C-19, WU-0B-09, WU-0B-15, WU-0B-17, WU-0B-18, WU-0B-19

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-15
- WU-0B-17
- WU-0B-18
- WU-0B-19

**Intra-Phase 0C predecessors:**
- WU-0C-19

## Test Boundary

`product-strategy/contracts/wu-0c-20a-provider-state-monitor-result.md`, `src-tauri/src/contracts/provider_state_monitor_result.rs`, `src/contracts/provider-state-monitor-result.ts`

## Code Boundary

`src-tauri/src/provider_monitor/monitor_result.rs`, `src-tauri/src/contracts/provider_state_monitor_result.rs`, `src/contracts/provider-state-monitor-result.ts`, `src-tauri/tests/wu_0c_20a_provider_state_monitor_result_contract.rs`, `src/test/provider-state-monitor-result.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Provider state monitor result DTO consumed by WU-0C-20, RenderEngine, worker dispatch later slices, provider panel later slices, recovery, and reroute.
- Parallelizable with: WU-0C-22a, WU-0C-23a, WU-0C-23b, WU-0C-24 after WU-0C-19 complete.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
