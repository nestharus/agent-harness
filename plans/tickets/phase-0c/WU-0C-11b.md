# WU-0C-11b: OulipolyInvocationRef DTO and Parser

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-11b  
**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-11b: OulipolyInvocationRef DTO and Parser`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

OulipolyInvocationRef {
  invocation_id: string,
  parent_invocation_id?: string,
  raw_stderr_ref: string
}

parse_oulipoly_invocation(stderr_ref: string) -> Result<OulipolyInvocationRef, AgentRunnerError>
```

## Acceptance Criteria

- [ ] `OulipolyInvocationRef` round-trips through serde while preserving invocation, parent invocation, and raw stderr refs.
- [ ] Calling `parse_oulipoly_invocation(stderr_ref)` against stderr containing `OULIPOLY_INVOCATION` returns the parsed invocation ID.
- [ ] Calling `parse_oulipoly_invocation(stderr_ref)` against stderr containing `OULIPOLY_PARENT_INVOCATION` returns both child and parent IDs.
- [ ] Calling `parse_oulipoly_invocation(stderr_ref)` with missing invocation markers returns `AgentRunnerError::InvocationMarkerMissing` and does not fabricate IDs.
- [ ] Parser output is substrate evidence only and writes no graph, worker, turn, provider, recovery, or audit rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0A-09 `TraceContext` and stderr evidence refs.

**Detailed dependency graph line, verbatim:** WU-0C-11b <- WU-0A-09

**Phase 0A upstream WUs:**
- WU-0A-09

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-11b-oulipoly-invocation-ref.md`, `src-tauri/src/contracts/oulipoly_invocation_ref.rs`

## Code Boundary

`src-tauri/src/agent_runner/oulipoly_invocation.rs`, `src-tauri/src/contracts/oulipoly_invocation_ref.rs`, `src-tauri/tests/wu_0c_11b_oulipoly_invocation_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Invocation reference parser consumed by subprocess supervisor, trace reader, hook payloads, plugin capability matrix, and AgentRunnerClient.
- Parallelizable with: WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-19, WU-0C-21, WU-0C-24, WU-0C-30.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
