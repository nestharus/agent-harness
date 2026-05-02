# WU-0C-04a: PolicyGateInput DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-04a  
**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-04a: PolicyGateInput DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct + TypeScript DTO

PolicyGateInput {
  gate_name: string,
  policy_set_id: string,
  input_ref: string,
  payload: JsonValue
}

validate_policy_gate_input(input: PolicyGateInput) -> Result<PolicyGateInput, PolicyError>
```

## Acceptance Criteria

- [ ] `PolicyGateInput` round-trips through Rust serde and TypeScript fixture JSON with `gate_name`, `policy_set_id`, `input_ref`, and `payload` preserved.
- [ ] Calling `validate_policy_gate_input(input)` with valid input returns the normalized DTO.
- [ ] Calling `validate_policy_gate_input(input)` with empty gate name, policy set ID, or input ref returns the documented `PolicyError` variant.
- [ ] Validation does not read PolicySet rows, evaluate policies, or emit audit drafts.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-04 `PolicySet` ID taxonomy and WU-0B-15 `AuditEvent` draft shape only.

**Detailed dependency graph line, verbatim:** WU-0C-04a <- WU-0B-04, WU-0B-15

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-04
- WU-0B-15

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-04a-policy-gate-input.md`, `src-tauri/src/contracts/policy_gate_input.rs`, `src/contracts/policy-gate-input.ts`

## Code Boundary

`src-tauri/src/policy/gate_input.rs`, `src-tauri/src/contracts/policy_gate_input.rs`, `src/contracts/policy-gate-input.ts`, `src-tauri/tests/wu_0c_04a_policy_gate_input_contract.rs`, `src/test/policy-gate-input.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Policy gate input DTO consumed by WU-0C-04 and downstream gate adapters.
- Parallelizable with: WU-0C-06a, WU-0C-07a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
