# WU-0C-02a: PolicyGateDescriptor DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-02a  
**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-02a: PolicyGateDescriptor DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

PolicyGateDescriptor {
  gate_name: string,
  policy_version_field: string,
  input_schema_ref: string,
  output_schema_ref: string,
  decision_reason_codes: string[]
}

validate_policy_gate_descriptor(descriptor: PolicyGateDescriptor) -> Result<PolicyGateDescriptor, PolicyError>
```

## Acceptance Criteria

- [ ] `PolicyGateDescriptor` round-trips through Rust serde and TypeScript fixture JSON with stable gate name, policy version field, input schema ref, output schema ref, and reason-code list.
- [ ] Calling `validate_policy_gate_descriptor(descriptor)` with valid schema refs returns the normalized DTO.
- [ ] Calling `validate_policy_gate_descriptor(descriptor)` with an empty gate name, empty schema ref, or empty reason-code list returns the documented error variant.
- [ ] DTO validation does not execute policies, write audit rows, mutate PolicySet rows, or call provider/render/budget services.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-04 `PolicySet`.

**Detailed dependency graph line, verbatim:** WU-0C-02a <- WU-0B-04

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-04

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-02a-policy-gate-descriptor.md`, `src-tauri/src/contracts/policy_gate_descriptor.rs`, `src/contracts/policy-gate-descriptor.ts`

## Code Boundary

`src-tauri/src/policy/gate_descriptor.rs`, `src-tauri/src/contracts/policy_gate_descriptor.rs`, `src/contracts/policy-gate-descriptor.ts`, `src-tauri/tests/wu_0c_02a_policy_gate_descriptor_contract.rs`, `src/test/policy-gate-descriptor.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Gate descriptor DTO consumed by WU-0C-02, WU-0C-04, Tauri command routing, and downstream policy explain surfaces.
- Parallelizable with: WU-0C-06a, WU-0C-07a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24a.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
