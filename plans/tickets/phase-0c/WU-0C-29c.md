# WU-0C-29c: IdentityResolverRuntime

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-29c  
**Parent initiative:** Identity/conflict shell  
**Implementation wave:** Wave 4 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-29c: IdentityResolverRuntime`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust service object

IdentityResolverRuntime::record_identity_event(event: IdentityEvent) -> Result<IdentityEvent, IdentityConflictError>
IdentityResolverRuntime::latest_identity_event(logical_node_id: string) -> Result<Option<IdentityEvent>, IdentityConflictError>

Routing invariant:
- Phase 0C records identity shell events only; it does not move, split, merge, restore, delete, or forward graph topology.
```

## Acceptance Criteria

- [ ] Calling `record_identity_event(event)` with every WU-0B-13 `IdentityEvent` event-type variant writes exactly one durable IdentityEvent row and returns it with all Phase 0B fields preserved.
- [ ] Calling `record_identity_event(event)` with unknown node/version refs returns the documented error and writes no row.
- [ ] Calling `latest_identity_event(logical_node_id)` returns the newest event for that logical node or `None` for no events.
- [ ] The routing invariant holds across move, split, merge, forwarding, restore, and delete fixtures: no GraphNode, GraphEdge, NodeRevision, GraphSnapshot, or ConflictRecord row is mutated.
- [ ] The service emits only audit event drafts; durable audit writing is delegated to WU-0C-37.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-04; incoming WU-0B-07 `GraphNode`, WU-0B-08 `GraphEdge`, WU-0B-12 `NodeRevision`, WU-0B-13 `IdentityEvent`, WU-0B-14 `GraphSnapshot`, WU-0B-15 `AuditEvent`.

**Detailed dependency graph line, verbatim:** WU-0C-29c <- WU-0C-04, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-07
- WU-0B-08
- WU-0B-12
- WU-0B-13
- WU-0B-14
- WU-0B-15

**Intra-Phase 0C predecessors:**
- WU-0C-04

## Test Boundary

`product-strategy/contracts/wu-0c-29c-identity-resolver-runtime.md`, `src-tauri/src/contracts/identity_resolver_runtime.rs`

## Code Boundary

`src-tauri/src/identity/identity_resolver_runtime.rs`, `src-tauri/src/contracts/identity_resolver_runtime.rs`, `src-tauri/tests/wu_0c_29c_identity_resolver_runtime_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Identity resolver runtime shell consumed by VS-012, VS-013, VS-017, VS-018, VS-020, and VS-021.
- Parallelizable with: WU-0C-29d after WU-0C-04 if write files remain disjoint; WU-0C-31a, WU-0C-33a.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
