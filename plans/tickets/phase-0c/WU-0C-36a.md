# WU-0C-36a: PaneRouteBinding DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-36a  
**Parent initiative:** UI shell wiring  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-36a: PaneRouteBinding DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: TypeScript DTO

PaneRouteBinding {
  paneId: PaneId,
  command?: HarnessCommand,
  eventTopics: EventTopic[],
  shellRegionStateSelector: string
}

validatePaneRouteBinding(binding: PaneRouteBinding): Result<PaneRouteBinding, UiShellError>
```

## Acceptance Criteria

- [ ] `PaneRouteBinding` fixture JSON parses with exact pane ID, command, event topics, and selector fields.
- [ ] Calling `validatePaneRouteBinding(binding)` accepts every seeded Phase 0C PaneId binding.
- [ ] Calling `validatePaneRouteBinding(binding)` with an unknown pane, command, topic, or empty selector returns the documented error.
- [ ] DTO validation does not route commands, subscribe events, render panes, or query GraphStore.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0A-05 `EventTopic`, WU-0A-07 `HarnessCommand`, WU-0A-11 `PaneId`, WU-0A-12 `ShellRegionState`.

**Detailed dependency graph line, verbatim:** WU-0C-36a <- WU-0A-05, WU-0A-07, WU-0A-11, WU-0A-12

**Phase 0A upstream WUs:**
- WU-0A-05
- WU-0A-07
- WU-0A-11
- WU-0A-12

**Phase 0B upstream WUs:**
- none

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-36a-pane-route-binding.md`, `src/contracts/pane-route-binding.ts`, fixtures under `product-strategy/contracts/fixtures/wu-0c-36a/`

## Code Boundary

`src/shell/pane-route-binding.ts`, `src/contracts/pane-route-binding.ts`, `src/test/pane-route-binding.test.ts`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Pane route binding DTO consumed by WU-0C-36 and later seeded UI panes.
- Parallelizable with: WU-0C-37a after WU-0C-35 if UI and audit files are disjoint.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
