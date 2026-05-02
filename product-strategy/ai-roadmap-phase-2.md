# Phase 2 AI-Optimized Roadmap - Lead Orchestrator Loop and Initial Curation

## Pipeline Reference

Phase 2 work units follow the same contract-first implementation pipeline used by the converged Phase 0A, Phase 0B, Phase 0C, and Phase 1 artifacts. The test agent sees only contracts, DTO signatures, command signatures, fixture specifications, and state-machine definitions. The code agent sees those tests plus implementation files. Each WU below is scoped as one single-concern PR.

- Phase 0 RCA is skipped unless an implementation attempt uncovers a concrete defect.
- Phase 1 research = gpt-high when hookpoints, provider semantics, or agent-runner behavior are uncertain.
- Phase 2 synthesis = gpt-high.
- Phase 2.5 existing-state risk profile = gpt-high.
- Phase 3 implementation proposal = gpt-high.
- Phase 4 risk = three independent claude-opus judgment passes plus gpt-high audit reconciliation.
- Phase 5 hookpoints = gpt-high.
- Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests.
- Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

Rules applied in this phase:
- D1: every distinct schema object, enum, service, adapter, state machine, method-bearing UI surface, and fixture pack is its own WU.
- D2: every method, enum, state, and transition declared in a Contract has binary acceptance criteria. WU-2-14 carries the deepest state machine and enumerates every state, valid transition, and invalid-transition rejection.
- D3: regression checks describe only checks actually performed during this proposer pass.
- D4: audit-history watch signals are tracked honestly in the Run Report.

## Phase 2 Scope

Phase 2 delivers the second feature phase: the lead orchestrator loop and initial optimizer curation. It consumes four upstream converged artifacts: Phase 0A runtime shell and fake agents, Phase 0B canonical durable schemas, Phase 0C shared engines/integration shells, and Phase 1 observable imposed-context services and UI surfaces.

Engineering order from `engineering-roadmap.md` lines 631-639:

1. VS-008 navigation tools over `AgentWalkState`.
2. VS-009 bounded orchestrator turns and advisory optimizer requests.
3. VS-010 context-management pipeline, summary refresh, and stale markers.

Phase 2 scope boundaries:
- Included: pack, unpack, focus, pin, unpin command schemas; walk-state transition validation; next-turn render invalidation; navigation tool audit; durable orchestrator turn lifecycle; `AgentRunnerClient`-mediated provider turn capability adapters; capture/commit transactions; advisory OptimizerRequest emission; compact detection; VS-010 turn decomposition, DetailRecord schema, detail injection routing, incremental summary update, full summary regeneration, stale-mark detection, per-task optimizer model dispatch, deterministic edit validation, merge and stale-base conflict classification, summary/stale UI, optimizer log UI, and first-class fixture packs per VS. VS-010 expansion is sourced from `proposal.md` Round 6 §1 Context-Management Model Assignment, `engineering-roadmap.md` r5 VS-010, `research/16-transcript-turn-decomposition.md`, `research/17-v4-per-task-model-assignment.md`, and Phase 0C r5 `SessionOverrideContract` WU-0C-N1..WU-0C-N3 and WU-0C-N5.
- Excluded: topology mutation, splits, merges, re-parenting, cross-reference creation, repack edits, worker dispatch, NEEDS_INPUT routing, reviewer sampling policy beyond existing deterministic hooks, recovery execution UI, and provider reroute/substitution. Those remain Phase 3+ work.
- Foreground OrchestratorBridge writes are bounded to GraphAction rows, evidence/provenance/audit rows, and advisory OptimizerRequest records. They never create or mutate GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, or OptimizerEdit graph truth.
- Optimizer behavior in this phase is limited to summary_regeneration and stale_mark drafts plus deterministic merge handling. Higher-consequence edit types are only represented by upstream schemas and remain disabled.

The requested round-6 monolithic input `product-strategy/ai-roadmap.md` was not present in this worktree. This draft uses the converged per-phase artifacts and the parent audit history instead, and records that gap in D3/D4.

## Work Unit Inventory

Total Phase 2 WUs: **50**.

| Value slice | WUs | Count |
|---|---|---:|
| VS-008: Navigate with Pack, Unpack, Focus, Pin, and Unpin | WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-07, WU-2-08, WU-2-09, WU-2-10, WU-2-11, WU-2-12 | 12 |
| VS-009: Run Bounded Orchestrator Turns with Advisory Optimizer Requests | WU-2-13, WU-2-14, WU-2-15, WU-2-16, WU-2-17, WU-2-18, WU-2-19, WU-2-20, WU-2-21, WU-2-22, WU-2-23, WU-2-24, WU-2-25, WU-2-26, WU-2-27, WU-2-28 | 16 |
| VS-010: Context-Management Pipeline, Refresh Summaries, and Mark Stale Nodes | WU-2-29, WU-2-30, WU-2-31, WU-2-32, WU-2-33, WU-2-34, WU-2-35, WU-2-36, WU-2-37, WU-2-38, WU-2-39, WU-2-40, WU-2-41, WU-2-42, WU-2-43, WU-2-44, WU-2-45, WU-2-46, WU-2-47, WU-2-48, WU-2-49, WU-2-50 | 22 |

| WU | Owns | Parent slice |
|---|---|---|
| WU-2-01 | PackToolCommandSchema | VS-008 |
| WU-2-02 | UnpackToolCommandSchema | VS-008 |
| WU-2-03 | FocusToolCommandSchema | VS-008 |
| WU-2-04 | PinToolCommandSchema | VS-008 |
| WU-2-05 | UnpinToolCommandSchema | VS-008 |
| WU-2-06 | NavigationToolResultDto | VS-008 |
| WU-2-07 | AgentWalkStateMutationService | VS-008 |
| WU-2-08 | WalkStateTransitionValidator | VS-008 |
| WU-2-09 | RenderInvalidationService | VS-008 |
| WU-2-10 | NavigationToolCallAuditEmitter | VS-008 |
| WU-2-11 | NavigationToolAffordanceAdapter | VS-008 |
| WU-2-12 | NavigationFixturePack | VS-008 |
| WU-2-13 | TurnLifecycleEventEnum | VS-009 |
| WU-2-14 | OrchestratorTurnStateMachine | VS-009 |
| WU-2-15 | TurnRenderPreparationService | VS-009 |
| WU-2-16 | OrchestratorBridgeService | VS-009 |
| WU-2-17 | TurnCaptureCommitTransactionService | VS-009 |
| WU-2-18 | AdvisoryOptimizerRequestEmitter | VS-009 |
| WU-2-19 | GraphActionTransactionService | VS-009 |
| WU-2-20 | CompactDetectionAvoidanceGuard | VS-009 |
| WU-2-21 | ParentInvocationPropagator | VS-009 |
| WU-2-22 | ClaudeTurnAdapter | VS-009 |
| WU-2-23 | CodexTurnAdapter | VS-009 |
| WU-2-24 | OpencodeTurnAdapter | VS-009 |
| WU-2-25 | TurnUIPaneSurface | VS-009 |
| WU-2-26 | OrchestratorTurnFixturePack | VS-009 |
| WU-2-27 | TurnLifecycleAuditEmitter | VS-009 |
| WU-2-28 | TurnProviderPreflightAdapter | VS-009 |
| WU-2-29 | OptimizerScopingRequestDto | VS-010 |
| WU-2-30 | OptimizerScopingService | VS-010 |
| WU-2-31 | OptimizerPromptSchemaDto | VS-010 |
| WU-2-32 | OptimizerResponseSchemaDto | VS-010 |
| WU-2-33 | SummaryRefreshRequestDto | VS-010 |
| WU-2-34 | OptimizerModelInvocationAdapter | VS-010 |
| WU-2-35 | DeterministicOptimizerEditValidator | VS-010 |
| WU-2-36 | OptimizerEditMergeService | VS-010 |
| WU-2-37 | ConflictOnStaleBaseClassifier | VS-010 |
| WU-2-38 | SummaryNodeStaleStateTransitionHandler | VS-010 |
| WU-2-39 | BackendStaleSignalEmitter | VS-010 |
| WU-2-40 | SummaryRegenerationUIComponent | VS-010 |
| WU-2-41 | StaleMarkerUIComponent | VS-010 |
| WU-2-42 | OptimizerLogUI | VS-010 |
| WU-2-43 | SummaryRefreshFixturePack | VS-010 |
| WU-2-44 | OptimizerEditAuditEmitter | VS-010 |
| WU-2-45 | DetailRecordSchemaDto | VS-010 |
| WU-2-46 | TurnDecompositionService | VS-010 |
| WU-2-47 | DetailInjectionRouterService | VS-010 |
| WU-2-48 | IncrementalSummaryUpdateService | VS-010 |
| WU-2-49 | FullSummaryRegenerationService | VS-010 |
| WU-2-50 | StaleMarkDetectionService | VS-010 |

## Phase 2 Work Units

### WU-2-01: PackToolCommandSchema

**Parent initiative:** VS-008: Pack tool command schema

**Contract:**
```text
contract_owner: PackToolCommandSchema
contract_kind: Navigation command DTO
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
PackToolCommand { node_id: OpaqueId<GraphNode>, reason: String, requested_depth?: u8, idempotency_key: String }.
```

**Test boundary:** product-strategy/contracts/wu-2-01-packtoolcommandschema.md; src-tauri/src/contracts/wu_2_01.rs; src/contracts/wu_2_01.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-01/

**Code boundary:** src-tauri/src/services/wu_2_01.rs; src-tauri/src/commands/wu_2_01.rs when IPC is declared; src/features/wu_2_01/**/* for UI WUs; src-tauri/tests/wu_2_01_contract.rs; src/test/wu_2_01.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** PackToolCommandSchema contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-02: UnpackToolCommandSchema

**Parent initiative:** VS-008: Unpack tool command schema

**Contract:**
```text
contract_owner: UnpackToolCommandSchema
contract_kind: Navigation command DTO
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
UnpackToolCommand { node_id: OpaqueId<GraphNode>, depth: u8, reason: String, max_child_count?: u16, idempotency_key: String }.
```

**Test boundary:** product-strategy/contracts/wu-2-02-unpacktoolcommandschema.md; src-tauri/src/contracts/wu_2_02.rs; src/contracts/wu_2_02.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-02/

**Code boundary:** src-tauri/src/services/wu_2_02.rs; src-tauri/src/commands/wu_2_02.rs when IPC is declared; src/features/wu_2_02/**/* for UI WUs; src-tauri/tests/wu_2_02_contract.rs; src/test/wu_2_02.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** UnpackToolCommandSchema contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-03: FocusToolCommandSchema

**Parent initiative:** VS-008: Focus tool command schema

**Contract:**
```text
contract_owner: FocusToolCommandSchema
contract_kind: Navigation command DTO
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
FocusToolCommand { node_id: OpaqueId<GraphNode>, reason: String, preserve_pins: bool, idempotency_key: String }.
```

**Test boundary:** product-strategy/contracts/wu-2-03-focustoolcommandschema.md; src-tauri/src/contracts/wu_2_03.rs; src/contracts/wu_2_03.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-03/

**Code boundary:** src-tauri/src/services/wu_2_03.rs; src-tauri/src/commands/wu_2_03.rs when IPC is declared; src/features/wu_2_03/**/* for UI WUs; src-tauri/tests/wu_2_03_contract.rs; src/test/wu_2_03.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** FocusToolCommandSchema contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-04, WU-2-05, WU-2-06.

### WU-2-04: PinToolCommandSchema

**Parent initiative:** VS-008: Pin tool command schema

**Contract:**
```text
contract_owner: PinToolCommandSchema
contract_kind: Navigation command DTO
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
PinToolCommand { node_id: OpaqueId<GraphNode>, reason: String, pin_scope: turn|initiative|session, idempotency_key: String }.
```

**Test boundary:** product-strategy/contracts/wu-2-04-pintoolcommandschema.md; src-tauri/src/contracts/wu_2_04.rs; src/contracts/wu_2_04.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-04/

**Code boundary:** src-tauri/src/services/wu_2_04.rs; src-tauri/src/commands/wu_2_04.rs when IPC is declared; src/features/wu_2_04/**/* for UI WUs; src-tauri/tests/wu_2_04_contract.rs; src/test/wu_2_04.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** PinToolCommandSchema contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-05, WU-2-06.

### WU-2-05: UnpinToolCommandSchema

**Parent initiative:** VS-008: Unpin tool command schema

**Contract:**
```text
contract_owner: UnpinToolCommandSchema
contract_kind: Navigation command DTO
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
UnpinToolCommand { node_id: OpaqueId<GraphNode>, reason: String, pin_scope: turn|initiative|session, idempotency_key: String }.
```

**Test boundary:** product-strategy/contracts/wu-2-05-unpintoolcommandschema.md; src-tauri/src/contracts/wu_2_05.rs; src/contracts/wu_2_05.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-05/

**Code boundary:** src-tauri/src/services/wu_2_05.rs; src-tauri/src/commands/wu_2_05.rs when IPC is declared; src/features/wu_2_05/**/* for UI WUs; src-tauri/tests/wu_2_05_contract.rs; src/test/wu_2_05.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** UnpinToolCommandSchema contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-06.

### WU-2-06: NavigationToolResultDto

**Parent initiative:** VS-008: Navigation tool result DTO

**Contract:**
```text
contract_owner: NavigationToolResultDto
contract_kind: Navigation result DTO
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
NavigationToolResult { tool_call_id, command_type, target_node_id, transition_state, walk_state_id, policy_decision_id?, budget_decision_id?, conflict_id?, rendered_next_turn: bool, denial_reason? }.
```

**Test boundary:** product-strategy/contracts/wu-2-06-navigationtoolresultdto.md; src-tauri/src/contracts/wu_2_06.rs; src/contracts/wu_2_06.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-06/

**Code boundary:** src-tauri/src/services/wu_2_06.rs; src-tauri/src/commands/wu_2_06.rs when IPC is declared; src/features/wu_2_06/**/* for UI WUs; src-tauri/tests/wu_2_06_contract.rs; src/test/wu_2_06.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** NavigationToolResultDto contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05.

### WU-2-07: AgentWalkStateMutationService

**Parent initiative:** VS-008: AgentWalkState consumer and extension service

**Contract:**
```text
contract_owner: AgentWalkStateMutationService
contract_kind: Method-bearing navigation service
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
AgentWalkStateMutationService.apply_navigation_command(command, base_walk_state_id, base_graph_snapshot_id) -> NavigationToolResult.
```

**Test boundary:** product-strategy/contracts/wu-2-07-agentwalkstatemutationservice.md; src-tauri/src/contracts/wu_2_07.rs; src/contracts/wu_2_07.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-07/

**Code boundary:** src-tauri/src/services/wu_2_07.rs; src-tauri/src/commands/wu_2_07.rs when IPC is declared; src/features/wu_2_07/**/* for UI WUs; src-tauri/tests/wu_2_07_contract.rs; src/test/wu_2_07.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `AgentWalkStateMutationService.apply_navigation_command` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-08.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** AgentWalkStateMutationService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-08: WalkStateTransitionValidator

**Parent initiative:** VS-008: Walk-state transition validator

**Contract:**
```text
contract_owner: WalkStateTransitionValidator
contract_kind: State machine validator
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
WalkStateTransitionValidator.validate_transition(command_type, from_state, to_state, policy_decision, budget_decision, conflict_state) -> TransitionDecision.
```

**Test boundary:** product-strategy/contracts/wu-2-08-walkstatetransitionvalidator.md; src-tauri/src/contracts/wu_2_08.rs; src/contracts/wu_2_08.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-08/

**Code boundary:** src-tauri/src/services/wu_2_08.rs; src-tauri/src/commands/wu_2_08.rs when IPC is declared; src/features/wu_2_08/**/* for UI WUs; src-tauri/tests/wu_2_08_contract.rs; src/test/wu_2_08.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `WalkStateTransitionValidator.validate_transition` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] The validator accepts requested -> validated for a resolvable node and rejects requested -> applied_to_walk_state.
- [ ] The validator accepts validated -> applied_to_walk_state when policy and budget decisions are accepted.
- [ ] The validator accepts validated -> denied_policy when PolicyEngine denies the target node.
- [ ] The validator accepts validated -> denied_budget when BudgetGateService denies expansion or pin persistence.
- [ ] The validator accepts validated -> denied_conflict when ConflictRecord marks the target unresolved.
- [ ] The validator accepts applied_to_walk_state -> rendered_next_turn only after RenderInvalidationService records the next-turn invalidation.
- [ ] The validator rejects rendered_next_turn -> applied_to_walk_state, denied_policy -> applied_to_walk_state, denied_budget -> rendered_next_turn, and denied_conflict -> rendered_next_turn.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** WalkStateTransitionValidator contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-09: RenderInvalidationService

**Parent initiative:** VS-008: Next-turn render invalidation service

**Contract:**
```text
contract_owner: RenderInvalidationService
contract_kind: Method-bearing render invalidation service
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
RenderInvalidationService.mark_walk_state_dirty(walk_state_id, changed_node_ids, reason) -> RenderInvalidationRecord.
```

**Test boundary:** product-strategy/contracts/wu-2-09-renderinvalidationservice.md; src-tauri/src/contracts/wu_2_09.rs; src/contracts/wu_2_09.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-09/

**Code boundary:** src-tauri/src/services/wu_2_09.rs; src-tauri/src/commands/wu_2_09.rs when IPC is declared; src/features/wu_2_09/**/* for UI WUs; src-tauri/tests/wu_2_09_contract.rs; src/test/wu_2_09.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `RenderInvalidationService.mark_walk_state_dirty` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-07.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** RenderInvalidationService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-10: NavigationToolCallAuditEmitter

**Parent initiative:** VS-008: Tool-call audit emitter for navigation events

**Contract:**
```text
contract_owner: NavigationToolCallAuditEmitter
contract_kind: Method-bearing audit emitter
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
NavigationToolCallAuditEmitter.emit_navigation_event(result, trace_context) -> AuditEventRef.
```

**Test boundary:** product-strategy/contracts/wu-2-10-navigationtoolcallauditemitter.md; src-tauri/src/contracts/wu_2_10.rs; src/contracts/wu_2_10.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-10/

**Code boundary:** src-tauri/src/services/wu_2_10.rs; src-tauri/src/commands/wu_2_10.rs when IPC is declared; src/features/wu_2_10/**/* for UI WUs; src-tauri/tests/wu_2_10_contract.rs; src/test/wu_2_10.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `NavigationToolCallAuditEmitter.emit_navigation_event` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-07.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** NavigationToolCallAuditEmitter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-11: NavigationToolAffordanceAdapter

**Parent initiative:** VS-008: Claude-facing navigation affordance adapter

**Contract:**
```text
contract_owner: NavigationToolAffordanceAdapter
contract_kind: Method-bearing tool adapter
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
NavigationToolAffordanceAdapter.build_affordances(render_result, capability_fingerprint_id) -> NavigationToolManifest.
```

**Test boundary:** product-strategy/contracts/wu-2-11-navigationtoolaffordanceadapter.md; src-tauri/src/contracts/wu_2_11.rs; src/contracts/wu_2_11.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-11/

**Code boundary:** src-tauri/src/services/wu_2_11.rs; src-tauri/src/commands/wu_2_11.rs when IPC is declared; src/features/wu_2_11/**/* for UI WUs; src-tauri/tests/wu_2_11_contract.rs; src/test/wu_2_11.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `NavigationToolAffordanceAdapter.build_affordances` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] The adapter maps upstream errors to documented Phase 2 errors without string matching or provider-specific fallthrough.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-08.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** NavigationToolAffordanceAdapter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-12: NavigationFixturePack

**Parent initiative:** VS-008: Navigation fixture pack

**Contract:**
```text
contract_owner: NavigationFixturePack
contract_kind: Fixture pack
Consumes AgentWalkState and produces no GraphNode, GraphEdge, NodeRevision, SummaryContract, or IdentityEvent mutation.
All commands return NavigationToolResult and emit tool-call provenance/audit refs.
Every denial is deterministic and cites policy_decision_id or budget_decision_id.
NavigationFixturePack includes valid focus, denied budget unpack, denied conflict pack, pin persistence, unpin idempotency, and rendered_next_turn goldens.
```

**Test boundary:** product-strategy/contracts/wu-2-12-navigationfixturepack.md; src-tauri/src/contracts/wu_2_12.rs; src/contracts/wu_2_12.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-12/

**Code boundary:** src-tauri/src/services/wu_2_12.rs; src-tauri/src/commands/wu_2_12.rs when IPC is declared; src/features/wu_2_12/**/* for UI WUs; src-tauri/tests/wu_2_12_contract.rs; src/test/wu_2_12.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful command updates AgentWalkState only and leaves GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, OptimizerRequest, and OptimizerEdit unchanged.
- [ ] Policy denial, budget denial, and conflict denial each return NavigationToolResult with the correct denial field populated and no walk-state write committed.
- [ ] An accepted walk-state change marks the next orchestrator render dirty and does not mutate the already-rendered WorkingSetSnapshot.
- [ ] Every fixture named in the Contract is present as a stable file or fixture builder and has one positive and one negative contract assertion.
- [ ] Fixture data uses Phase 0A temp harness/fake agents and Phase 0B/0C/1 contracts; it does not invent alternate schemas.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-07, WU-2-08, WU-2-09, WU-2-10, WU-2-11.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-07, WU-0B-08, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-27, WU-0B-30.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-12, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36.

**Produces:** NavigationFixturePack contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06.

### WU-2-13: TurnLifecycleEventEnum

**Parent initiative:** VS-009: Turn lifecycle audit-event enum extension

**Contract:**
```text
contract_owner: TurnLifecycleEventEnum
contract_kind: Enum
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
TurnLifecycleEvent = turn_requested|snapshot_selected|render_created|launch_requested|session_accepted|tool_observed|capture_completed|commit_started|optimizer_enqueued|turn_completed|turn_blocked|turn_recovering|compact_detected.
```

**Test boundary:** product-strategy/contracts/wu-2-13-turnlifecycleeventenum.md; src-tauri/src/contracts/wu_2_13.rs; src/contracts/wu_2_13.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-13/

**Code boundary:** src-tauri/src/services/wu_2_13.rs; src-tauri/src/commands/wu_2_13.rs when IPC is declared; src/features/wu_2_13/**/* for UI WUs; src-tauri/tests/wu_2_13_contract.rs; src/test/wu_2_13.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] Lead-orchestrator provider routing preserves the Claude Code substrate by selecting Claude Sonnet 4.6, with Claude Haiku 4.5 only as an explicit degraded fallback, as constrained by `proposal.md` Round 4 §1 and `research/17-v4-per-task-model-assignment.md` §7.
- [ ] Worker-routing policy defaults sub-agent dispatch to MiniMax-M2.7 through the Phase 0C provider substrate, with MiniMax-M2.7-highspeed only as the documented routine-worker fallback and no silent lead-substrate switch.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** TurnLifecycleEventEnum contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-20, WU-2-21, WU-2-28.

### WU-2-14: OrchestratorTurnStateMachine

**Parent initiative:** VS-009: OrchestratorTurn lifecycle state machine

**Contract:**
```text
contract_owner: OrchestratorTurnStateMachine
contract_kind: State machine
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
OrchestratorTurnStateMachine.transition(turn_id, event, guard_inputs) -> OrchestratorTurnStateDecision for idle, snapshotting, rendering, launching_or_resuming, thinking, tool_pending, capturing, committing, optimizer_enqueue, complete, blocked, recovering.
```

**Test boundary:** product-strategy/contracts/wu-2-14-orchestratorturnstatemachine.md; src-tauri/src/contracts/wu_2_14.rs; src/contracts/wu_2_14.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-14/

**Code boundary:** src-tauri/src/services/wu_2_14.rs; src-tauri/src/commands/wu_2_14.rs when IPC is declared; src/features/wu_2_14/**/* for UI WUs; src-tauri/tests/wu_2_14_contract.rs; src/test/wu_2_14.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OrchestratorTurnStateMachine.transition` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] State idle is reachable only before a requested turn owns a graph_snapshot_id.
- [ ] State snapshotting is reachable after turn request creation and before WorkingSetSnapshot creation.
- [ ] State rendering is reachable only after snapshotting records identity, configuration, provider, and policy refs.
- [ ] State launching_or_resuming is reachable only after rendering succeeds within budget and route policy.
- [ ] State thinking is reachable only after the CLI adapter records session acceptance.
- [ ] State tool_pending is reachable only from thinking after a tool call or GraphAction draft is observed.
- [ ] State capturing is reachable from thinking final output or from tool_pending tool result/failure.
- [ ] State committing is reachable only after evidence and tool provenance writes are durable.
- [ ] State optimizer_enqueue is reachable only after bounded GraphAction validation passes.
- [ ] State complete is reachable only after queue writes and audit events are durable.
- [ ] State blocked is reachable from rendering, launching_or_resuming, thinking, tool_pending, capturing, or committing when user input or a hard gate is required.
- [ ] State recovering is reachable from any state on session loss, protocol corruption, provider lockout, entitlement change, sandbox denial, runtime loss, compact corruption, or storage conflict.
- [ ] Transition idle -> snapshotting succeeds with a requested turn and fails without a turn request.
- [ ] Transition snapshotting -> rendering succeeds after GraphSnapshot selection and fails if identity resolution is missing.
- [ ] Transition rendering -> launching_or_resuming succeeds for an accepted budget/provider route and fails when BudgetDecision is blocked.
- [ ] Transition rendering -> blocked succeeds for overfull required context, invalid configuration, insufficient provider state, or unreconciled corruption.
- [ ] Transition launching_or_resuming -> thinking succeeds only after Claude/Codex/opencode acceptance is captured.
- [ ] Transition thinking -> tool_pending succeeds on tool call detection and fails on plain final output.
- [ ] Transition tool_pending -> capturing succeeds after tool result, tool denial, or tool failure is captured.
- [ ] Transition thinking -> capturing succeeds on final model output and rejects when an unhandled tool call remains pending.
- [ ] Transition capturing -> committing succeeds only after evidence artifact, transcript locator, and tool provenance rows are durable.
- [ ] Transition committing -> optimizer_enqueue succeeds only after every GraphAction commit_state is committed or rejected.
- [ ] Transition optimizer_enqueue -> complete succeeds only after OptimizerRequest enqueue and AuditEvent emission complete.
- [ ] Invalid transitions idle -> thinking, rendering -> complete, tool_pending -> complete, committing -> thinking, complete -> committing, and blocked -> complete are rejected with exact error codes.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-13.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** OrchestratorTurnStateMachine contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-15: TurnRenderPreparationService

**Parent initiative:** VS-009: Snapshot and render preparation service

**Contract:**
```text
contract_owner: TurnRenderPreparationService
contract_kind: Method-bearing service
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
TurnRenderPreparationService.prepare_turn_render(turn_request) -> PreparedTurnRender with graph_snapshot_id, working_set_id, provider_state_id, budget_ledger_id.
```

**Test boundary:** product-strategy/contracts/wu-2-15-turnrenderpreparationservice.md; src-tauri/src/contracts/wu_2_15.rs; src/contracts/wu_2_15.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-15/

**Code boundary:** src-tauri/src/services/wu_2_15.rs; src-tauri/src/commands/wu_2_15.rs when IPC is declared; src/features/wu_2_15/**/* for UI WUs; src-tauri/tests/wu_2_15_contract.rs; src/test/wu_2_15.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `TurnRenderPreparationService.prepare_turn_render` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-14.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** TurnRenderPreparationService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-16: OrchestratorBridgeService

**Parent initiative:** VS-009: Central OrchestratorBridge service

**Contract:**
```text
contract_owner: OrchestratorBridgeService
contract_kind: Method-bearing orchestrator brain
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
OrchestratorBridgeService.run_bounded_turn(request) -> OrchestratorTurnResult, using RenderEngine, AgentRunnerClient, navigation tools, policy, budget, provider preflight, capture, and commit services.
```

**Test boundary:** product-strategy/contracts/wu-2-16-orchestratorbridgeservice.md; src-tauri/src/contracts/wu_2_16.rs; src/contracts/wu_2_16.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-16/

**Code boundary:** src-tauri/src/services/wu_2_16.rs; src-tauri/src/commands/wu_2_16.rs when IPC is declared; src/features/wu_2_16/**/* for UI WUs; src-tauri/tests/wu_2_16_contract.rs; src/test/wu_2_16.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OrchestratorBridgeService.run_bounded_turn` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-11, WU-2-14, WU-2-15, WU-2-17, WU-2-18, WU-2-20, WU-2-22, WU-2-23, WU-2-24, WU-2-28.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** OrchestratorBridgeService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-17: TurnCaptureCommitTransactionService

**Parent initiative:** VS-009: Turn capture and commit transaction service

**Contract:**
```text
contract_owner: TurnCaptureCommitTransactionService
contract_kind: Method-bearing transaction service
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
TurnCaptureCommitTransactionService.capture_and_commit(turn_id, session_capture, graph_action_drafts) -> TurnCommitResult.
```

**Test boundary:** product-strategy/contracts/wu-2-17-turncapturecommittransactionservice.md; src-tauri/src/contracts/wu_2_17.rs; src/contracts/wu_2_17.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-17/

**Code boundary:** src-tauri/src/services/wu_2_17.rs; src-tauri/src/commands/wu_2_17.rs when IPC is declared; src/features/wu_2_17/**/* for UI WUs; src-tauri/tests/wu_2_17_contract.rs; src/test/wu_2_17.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `TurnCaptureCommitTransactionService.capture_and_commit` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-14, WU-2-19, WU-2-27.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** TurnCaptureCommitTransactionService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-18: AdvisoryOptimizerRequestEmitter

**Parent initiative:** VS-009: Advisory optimizer request emitter

**Contract:**
```text
contract_owner: AdvisoryOptimizerRequestEmitter
contract_kind: Method-bearing queue emitter
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
AdvisoryOptimizerRequestEmitter.enqueue_from_turn(turn_id, request_payload, base_snapshot_id) -> OptimizerRequestRef with source_type orchestrator_turn.
```

**Test boundary:** product-strategy/contracts/wu-2-18-advisoryoptimizerrequestemitter.md; src-tauri/src/contracts/wu_2_18.rs; src/contracts/wu_2_18.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-18/

**Code boundary:** src-tauri/src/services/wu_2_18.rs; src-tauri/src/commands/wu_2_18.rs when IPC is declared; src/features/wu_2_18/**/* for UI WUs; src-tauri/tests/wu_2_18_contract.rs; src/test/wu_2_18.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `AdvisoryOptimizerRequestEmitter.enqueue_from_turn` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-17.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** AdvisoryOptimizerRequestEmitter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-19: GraphActionTransactionService

**Parent initiative:** VS-009: Foreground GraphAction transaction service

**Contract:**
```text
contract_owner: GraphActionTransactionService
contract_kind: Method-bearing transaction service
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
GraphActionTransactionService.validate_and_commit_actions(turn_id, actions) -> GraphActionCommitBatch.
```

**Test boundary:** product-strategy/contracts/wu-2-19-graphactiontransactionservice.md; src-tauri/src/contracts/wu_2_19.rs; src/contracts/wu_2_19.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-19/

**Code boundary:** src-tauri/src/services/wu_2_19.rs; src-tauri/src/commands/wu_2_19.rs when IPC is declared; src/features/wu_2_19/**/* for UI WUs; src-tauri/tests/wu_2_19_contract.rs; src/test/wu_2_19.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `GraphActionTransactionService.validate_and_commit_actions` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-14.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** GraphActionTransactionService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-20: CompactDetectionAvoidanceGuard

**Parent initiative:** VS-009: Compact detection and avoidance guard

**Contract:**
```text
contract_owner: CompactDetectionAvoidanceGuard
contract_kind: Method-bearing guard
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
CompactDetectionAvoidanceGuard.evaluate(session_capture, render_budget, cli_config) -> CompactGuardDecision.
```

**Test boundary:** product-strategy/contracts/wu-2-20-compactdetectionavoidanceguard.md; src-tauri/src/contracts/wu_2_20.rs; src/contracts/wu_2_20.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-20/

**Code boundary:** src-tauri/src/services/wu_2_20.rs; src-tauri/src/commands/wu_2_20.rs when IPC is declared; src/features/wu_2_20/**/* for UI WUs; src-tauri/tests/wu_2_20_contract.rs; src/test/wu_2_20.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `CompactDetectionAvoidanceGuard.evaluate` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** CompactDetectionAvoidanceGuard contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-21, WU-2-28.

### WU-2-21: ParentInvocationPropagator

**Parent initiative:** VS-009: Parent invocation propagation service

**Contract:**
```text
contract_owner: ParentInvocationPropagator
contract_kind: Method-bearing propagation service
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
ParentInvocationPropagator.attach_parent_invocation(turn_id, oulipoly_invocation_ref, trace_tree) -> ParentInvocationAttachment.
```

**Test boundary:** product-strategy/contracts/wu-2-21-parentinvocationpropagator.md; src-tauri/src/contracts/wu_2_21.rs; src/contracts/wu_2_21.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-21/

**Code boundary:** src-tauri/src/services/wu_2_21.rs; src-tauri/src/commands/wu_2_21.rs when IPC is declared; src/features/wu_2_21/**/* for UI WUs; src-tauri/tests/wu_2_21_contract.rs; src/test/wu_2_21.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `ParentInvocationPropagator.attach_parent_invocation` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** ParentInvocationPropagator contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-28.

### WU-2-22: ClaudeTurnAdapter

**Parent initiative:** VS-009: Claude CLI turn adapter

**Contract:**
```text
contract_owner: ClaudeTurnAdapter
contract_kind: Provider capability adapter over AgentRunnerClient
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
ClaudeTurnAdapter.request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession using Phase 0C AgentRunnerClient invocation/resume evidence.
Claude-specific behavior is limited to observed capability, hook, and tool-surface differences after AgentRunnerClient acceptance.
No direct `claude` command execution, Claude session storage assumption, provider/account route selection, resume composition, or session-id capture is owned here.
```

**Test boundary:** product-strategy/contracts/wu-2-22-claudeturnadapter.md; src-tauri/src/contracts/wu_2_22.rs; src/contracts/wu_2_22.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-22/

**Code boundary:** src-tauri/src/services/wu_2_22.rs; src-tauri/src/commands/wu_2_22.rs when IPC is declared; src/features/wu_2_22/**/* for UI WUs; src-tauri/tests/wu_2_22_contract.rs; src/test/wu_2_22.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `ClaudeTurnAdapter.request_turn` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] The adapter maps upstream errors to documented Phase 2 errors without string matching or provider-specific fallthrough.
- [ ] Launch, resume, provider routing, provider/account selection, session-id capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `claude` directly or infer Claude storage paths is rejected.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-13.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** ClaudeTurnAdapter capability contract and tests over AgentRunnerClient evidence; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-23: CodexTurnAdapter

**Parent initiative:** VS-009: Codex CLI turn adapter

**Contract:**
```text
contract_owner: CodexTurnAdapter
contract_kind: Provider capability adapter over AgentRunnerClient
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
CodexTurnAdapter.request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession using Phase 0C AgentRunnerClient invocation/resume evidence.
Codex-specific behavior is limited to normalized post-acceptance rollout/tool-surface semantics.
No direct `codex` command execution, thread storage assumption, provider/account route selection, resume composition, or session-id capture is owned here.
```

**Test boundary:** product-strategy/contracts/wu-2-23-codexturnadapter.md; src-tauri/src/contracts/wu_2_23.rs; src/contracts/wu_2_23.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-23/

**Code boundary:** src-tauri/src/services/wu_2_23.rs; src-tauri/src/commands/wu_2_23.rs when IPC is declared; src/features/wu_2_23/**/* for UI WUs; src-tauri/tests/wu_2_23_contract.rs; src/test/wu_2_23.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `CodexTurnAdapter.request_turn` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] The adapter maps upstream errors to documented Phase 2 errors without string matching or provider-specific fallthrough.
- [ ] Launch, resume, provider routing, provider/account selection, thread/session capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `codex` directly or infer Codex thread storage is rejected.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-13.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** CodexTurnAdapter capability contract and tests over AgentRunnerClient evidence; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-24: OpencodeTurnAdapter

**Parent initiative:** VS-009: Opencode CLI turn adapter

**Contract:**
```text
contract_owner: OpencodeTurnAdapter
contract_kind: Provider capability adapter over AgentRunnerClient
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
OpencodeTurnAdapter.request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession using Phase 0C AgentRunnerClient invocation/resume evidence.
Opencode-specific behavior is limited to typed substrate-gap and tool-surface evidence observed after AgentRunnerClient acceptance.
No direct `opencode` command execution, session-row mapping, provider/account route selection, resume composition, or session-id capture is owned here.
```

**Test boundary:** product-strategy/contracts/wu-2-24-opencodeturnadapter.md; src-tauri/src/contracts/wu_2_24.rs; src/contracts/wu_2_24.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-24/

**Code boundary:** src-tauri/src/services/wu_2_24.rs; src-tauri/src/commands/wu_2_24.rs when IPC is declared; src/features/wu_2_24/**/* for UI WUs; src-tauri/tests/wu_2_24_contract.rs; src/test/wu_2_24.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OpencodeTurnAdapter.request_turn` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] The adapter maps upstream errors to documented Phase 2 errors without string matching or provider-specific fallthrough.
- [ ] Launch, resume, provider routing, provider/account selection, session-row mapping, session-id capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `opencode` directly or infer opencode storage rows is rejected.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-13.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** OpencodeTurnAdapter capability contract and tests over AgentRunnerClient evidence; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-25: TurnUIPaneSurface

**Parent initiative:** VS-009: Turn lifecycle UI pane surface

**Contract:**
```text
contract_owner: TurnUIPaneSurface
contract_kind: Method-bearing UI surface
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
TurnUIPaneSurface renders current turn state, captured artifacts, advisory optimizer requests, compact guard state, and blocked/recovering causes.
```

**Test boundary:** product-strategy/contracts/wu-2-25-turnuipanesurface.md; src-tauri/src/contracts/wu_2_25.rs; src/contracts/wu_2_25.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-25/

**Code boundary:** src-tauri/src/services/wu_2_25.rs; src-tauri/src/commands/wu_2_25.rs when IPC is declared; src/features/wu_2_25/**/* for UI WUs; src-tauri/tests/wu_2_25_contract.rs; src/test/wu_2_25.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] The UI renders loading, empty, happy, denied/blocked, failed, and stale-data states without issuing backend mutations from render-only interactions.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-14, WU-2-16, WU-2-17, WU-2-18, WU-2-20.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** TurnUIPaneSurface contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-26: OrchestratorTurnFixturePack

**Parent initiative:** VS-009: Orchestrator turn fixture pack

**Contract:**
```text
contract_owner: OrchestratorTurnFixturePack
contract_kind: Fixture pack
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
OrchestratorTurnFixturePack includes happy path, tool path, blocked render, compact detected, failed capture, recovered turn, and optimizer request fixtures.
Provider-launch fixtures assert `agents` / WU-0C-18 AgentRunnerClient invocation, normalized session evidence, and provider-specific capability/tool-surface differences only.
Direct provider CLI command, resume, session-row, thread-storage, or JSONL-storage fixtures are outside this fixture pack.
```

**Test boundary:** product-strategy/contracts/wu-2-26-orchestratorturnfixturepack.md; src-tauri/src/contracts/wu_2_26.rs; src/contracts/wu_2_26.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-26/

**Code boundary:** src-tauri/src/services/wu_2_26.rs; src-tauri/src/commands/wu_2_26.rs when IPC is declared; src/features/wu_2_26/**/* for UI WUs; src-tauri/tests/wu_2_26_contract.rs; src/test/wu_2_26.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] Every fixture named in the Contract is present as a stable file or fixture builder and has one positive and one negative contract assertion.
- [ ] Fixture data uses Phase 0A temp harness/fake agents and Phase 0B/0C/1 contracts, and fixture assertions prove WU-2-22, WU-2-23, and WU-2-24 consume `agents` / WU-0C-18 AgentRunnerClient invocation and normalized session evidence, not direct provider CLI command/resume fixtures.
- [ ] Provider-specific fixture variants are limited to observed capability, hook, tool-surface, rollout, and substrate-gap differences after AgentRunnerClient acceptance.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-13, WU-2-14, WU-2-15, WU-2-16, WU-2-17, WU-2-18, WU-2-19, WU-2-20, WU-2-21, WU-2-22, WU-2-23, WU-2-24, WU-2-25, WU-2-27, WU-2-28.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** OrchestratorTurnFixturePack contract and tests; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-27: TurnLifecycleAuditEmitter

**Parent initiative:** VS-009: Turn lifecycle audit emitter

**Contract:**
```text
contract_owner: TurnLifecycleAuditEmitter
contract_kind: Method-bearing audit emitter
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
TurnLifecycleAuditEmitter.emit_turn_event(turn_id, lifecycle_event, refs) -> AuditEventRef.
```

**Test boundary:** product-strategy/contracts/wu-2-27-turnlifecycleauditemitter.md; src-tauri/src/contracts/wu_2_27.rs; src/contracts/wu_2_27.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-27/

**Code boundary:** src-tauri/src/services/wu_2_27.rs; src-tauri/src/commands/wu_2_27.rs when IPC is declared; src/features/wu_2_27/**/* for UI WUs; src-tauri/tests/wu_2_27_contract.rs; src/test/wu_2_27.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `TurnLifecycleAuditEmitter.emit_turn_event` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-13.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** TurnLifecycleAuditEmitter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21, WU-2-28.

### WU-2-28: TurnProviderPreflightAdapter

**Parent initiative:** VS-009: Provider preflight adapter for orchestrator turns

**Contract:**
```text
contract_owner: TurnProviderPreflightAdapter
contract_kind: Method-bearing provider adapter
Consumes stable GraphSnapshot and WorkingSetSnapshot records for one foreground turn.
Creates bounded GraphAction records and advisory OptimizerRequest records only.
No foreground path may mutate graph topology, summaries, node revisions, identity, or optimizer edits.
TurnProviderPreflightAdapter.preflight_orchestrator_route(turn_request, provider_requirements) -> TurnRouteDecision.
```

**Test boundary:** product-strategy/contracts/wu-2-28-turnproviderpreflightadapter.md; src-tauri/src/contracts/wu_2_28.rs; src/contracts/wu_2_28.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-28/

**Code boundary:** src-tauri/src/services/wu_2_28.rs; src-tauri/src/commands/wu_2_28.rs when IPC is declared; src/features/wu_2_28/**/* for UI WUs; src-tauri/tests/wu_2_28_contract.rs; src/test/wu_2_28.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `TurnProviderPreflightAdapter.preflight_orchestrator_route` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] A successful bounded turn records exactly one OrchestratorTurn row, at least one WorkingSetSnapshot ref, and zero topology mutations.
- [ ] Every GraphAction action_type outside record_model_output, record_tool_provenance, attach_audit_note, emit_user_facing_output, and create_optimizer_request is rejected.
- [ ] Every OptimizerRequest emitted by this WU family has source_type orchestrator_turn and advisory_state queued at creation.
- [ ] Any detected /compact or equivalent CLI compaction marks the session corrupt or recovering and prevents complete state from being reached.
- [ ] The adapter maps upstream errors to documented Phase 2 errors without string matching or provider-specific fallthrough.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-09, WU-0B-10, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-20, WU-0B-21, WU-0B-22, WU-0B-23, WU-0B-24, WU-0B-25, WU-0B-27.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28, WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56.

**Produces:** TurnProviderPreflightAdapter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-13, WU-2-20, WU-2-21.

### WU-2-29: OptimizerScopingRequestDto

**Parent initiative:** VS-010: Optimizer scoping request DTO

**Contract:**
```text
contract_owner: OptimizerScopingRequestDto
contract_kind: DTO
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerScopingRequest { optimizer_request_ids, changed_node_ids, base_graph_snapshot_id, budget_scope_id, configuration_id, stale_signal_refs, max_nodes, max_evidence_items }.
```

**Test boundary:** product-strategy/contracts/wu-2-29-optimizerscopingrequestdto.md; src-tauri/src/contracts/wu_2_29.rs; src/contracts/wu_2_29.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-29/

**Code boundary:** src-tauri/src/services/wu_2_29.rs; src-tauri/src/commands/wu_2_29.rs when IPC is declared; src/features/wu_2_29/**/* for UI WUs; src-tauri/tests/wu_2_29_contract.rs; src/test/wu_2_29.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerScopingRequestDto contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-33.

### WU-2-30: OptimizerScopingService

**Parent initiative:** VS-010: Optimizer scoping service

**Contract:**
```text
contract_owner: OptimizerScopingService
contract_kind: Method-bearing service
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerScopingService.scope_cycle(request) -> OptimizerScope with bounded nodes, evidence refs, summary contracts, budget decision, and merge base.
```

**Test boundary:** product-strategy/contracts/wu-2-30-optimizerscopingservice.md; src-tauri/src/contracts/wu_2_30.rs; src/contracts/wu_2_30.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-30/

**Code boundary:** src-tauri/src/services/wu_2_30.rs; src-tauri/src/commands/wu_2_30.rs when IPC is declared; src/features/wu_2_30/**/* for UI WUs; src-tauri/tests/wu_2_30_contract.rs; src/test/wu_2_30.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OptimizerScopingService.scope_cycle` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Optimizer scoping invokes any model-backed task only through WU-2-34 OptimizerModelInvocationAdapter with task_class `optimizer_scoping` routed to MiniMax-M2.7 per `proposal.md` Round 4 §1 and `research/17-v4-per-task-model-assignment.md` §7.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-29.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerScopingService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-31: OptimizerPromptSchemaDto

**Parent initiative:** VS-010: Optimizer prompt schema DTO

**Contract:**
```text
contract_owner: OptimizerPromptSchemaDto
contract_kind: DTO
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerPromptSchema { scope_id, actor_model, graph_snapshot_ref, target_nodes, current_summaries, evidence_refs, configuration_refs, allowed_edit_types, privilege_labels, budget_caps }.
```

**Test boundary:** product-strategy/contracts/wu-2-31-optimizerpromptschemadto.md; src-tauri/src/contracts/wu_2_31.rs; src/contracts/wu_2_31.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-31/

**Code boundary:** src-tauri/src/services/wu_2_31.rs; src-tauri/src/commands/wu_2_31.rs when IPC is declared; src/features/wu_2_31/**/* for UI WUs; src-tauri/tests/wu_2_31_contract.rs; src/test/wu_2_31.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-30.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerPromptSchemaDto contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-32: OptimizerResponseSchemaDto

**Parent initiative:** VS-010: Optimizer response schema DTO

**Contract:**
```text
contract_owner: OptimizerResponseSchemaDto
contract_kind: DTO
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerResponseSchema { scope_id, proposed_edits, cited_evidence_ids, stale_markers, rejected_request_ids, uncertainty_notes, token_usage }.
```

**Test boundary:** product-strategy/contracts/wu-2-32-optimizerresponseschemadto.md; src-tauri/src/contracts/wu_2_32.rs; src/contracts/wu_2_32.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-32/

**Code boundary:** src-tauri/src/services/wu_2_32.rs; src-tauri/src/commands/wu_2_32.rs when IPC is declared; src/features/wu_2_32/**/* for UI WUs; src-tauri/tests/wu_2_32_contract.rs; src/test/wu_2_32.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-31.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerResponseSchemaDto contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-33: SummaryRefreshRequestDto

**Parent initiative:** VS-010: Summary refresh request DTO

**Contract:**
```text
contract_owner: SummaryRefreshRequestDto
contract_kind: DTO
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
SummaryRefreshRequest { target_node_id, base_revision_id, base_summary_contract_id, stale_reasons, evidence_ids, configuration_refs, requested_by }.
```

**Test boundary:** product-strategy/contracts/wu-2-33-summaryrefreshrequestdto.md; src-tauri/src/contracts/wu_2_33.rs; src/contracts/wu_2_33.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-33/

**Code boundary:** src-tauri/src/services/wu_2_33.rs; src-tauri/src/commands/wu_2_33.rs when IPC is declared; src/features/wu_2_33/**/* for UI WUs; src-tauri/tests/wu_2_33_contract.rs; src/test/wu_2_33.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Every field or enum variant has at least one positive fixture and one invalid or missing-field rejection fixture.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** SummaryRefreshRequestDto contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29.

### WU-2-34: OptimizerModelInvocationAdapter

**Parent initiative:** VS-010: Optimizer model invocation adapter

**Contract:**
```text
contract_owner: OptimizerModelInvocationAdapter
contract_kind: Method-bearing invocation adapter
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerModelInvocationAdapter.invoke_optimizer(task_class, prompt_schema, budget_scope) -> OptimizerResponseSchema through AgentRunnerClient with per-task model dispatch.
Dispatch policy: MiniMax-M2.7 for turn_decomposition, detail_injection_routing, incremental_summary_update, full_summary_regeneration, stale_mark_detection, cross_reference_discovery, repack_planning, optimizer_scoping, and sub_agent_provider_routing; Claude Opus 4.7 for conflict_on_stale_base; Claude Sonnet 4.6 for lead_orchestrator_provider_routing; GPT-5.5 for reviewer_sampling; deterministic_only tasks receive no model call.
Source basis: proposal.md Round 4 §1 Context-Management Model Assignment; research/15-v2-long-context-model-benchmarks.md; research/16-transcript-turn-decomposition.md; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-34-optimizermodelinvocationadapter.md; src-tauri/src/contracts/wu_2_34.rs; src/contracts/wu_2_34.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-34/

**Code boundary:** src-tauri/src/services/wu_2_34.rs; src-tauri/src/commands/wu_2_34.rs when IPC is declared; src/features/wu_2_34/**/* for UI WUs; src-tauri/tests/wu_2_34_contract.rs; src/test/wu_2_34.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OptimizerModelInvocationAdapter.invoke_optimizer` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] The adapter maps upstream errors to documented Phase 2 errors without string matching or provider-specific fallthrough.
- [ ] The adapter respects the per-task assignment matrix exactly: MiniMax-M2.7 for routine bounded optimizer classes, Claude Opus 4.7 for conflict-on-stale-base, Claude Sonnet 4.6 for lead-orchestrator provider routing, GPT-5.5 for reviewer sampling, and deterministic-only for render/validator/merge classes.
- [ ] No primary or fallback dispatch path can select GLM, Gemini, Qwen, Mistral, or DeepSeek; attempts to configure those providers for these task classes fail closed with a documented provider_excluded error.
- [ ] MiniMax-M2.7-highspeed is accepted only as a routine MiniMax fallback when both the matrix and the calling WU contract name it; it is not used for conflict arbitration, reviewer sampling, lead-orchestrator substrate routing, or stale-mark ambiguity where the WU requires MiniMax-M2.7 only.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-31.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerModelInvocationAdapter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-35: DeterministicOptimizerEditValidator

**Parent initiative:** VS-010: Deterministic optimizer edit validator

**Contract:**
```text
contract_owner: DeterministicOptimizerEditValidator
contract_kind: Method-bearing validator
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
DeterministicOptimizerEditValidator.validate_response(response_schema, scope) -> OptimizerEditValidationBatch.
```

**Test boundary:** product-strategy/contracts/wu-2-35-deterministicoptimizereditvalidator.md; src-tauri/src/contracts/wu_2_35.rs; src/contracts/wu_2_35.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-35/

**Code boundary:** src-tauri/src/services/wu_2_35.rs; src-tauri/src/commands/wu_2_35.rs when IPC is declared; src/features/wu_2_35/**/* for UI WUs; src-tauri/tests/wu_2_35_contract.rs; src/test/wu_2_35.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `DeterministicOptimizerEditValidator.validate_response` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] DeterministicOptimizerEditValidator is deterministic-only and never calls WU-2-34 or any LLM provider; tests fail if a provider route is requested.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-30, WU-2-32, WU-2-33.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** DeterministicOptimizerEditValidator contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-36: OptimizerEditMergeService

**Parent initiative:** VS-010: Optimizer edit merge service for summary and stale edits

**Contract:**
```text
contract_owner: OptimizerEditMergeService
contract_kind: Method-bearing merge service
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerEditMergeService.merge_validated_edit(edit_id, current_snapshot_id) -> OptimizerMergeResult.
```

**Test boundary:** product-strategy/contracts/wu-2-36-optimizereditmergeservice.md; src-tauri/src/contracts/wu_2_36.rs; src/contracts/wu_2_36.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-36/

**Code boundary:** src-tauri/src/services/wu_2_36.rs; src-tauri/src/commands/wu_2_36.rs when IPC is declared; src/features/wu_2_36/**/* for UI WUs; src-tauri/tests/wu_2_36_contract.rs; src/test/wu_2_36.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OptimizerEditMergeService.merge_validated_edit` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] OptimizerEditMergeService is deterministic-only and never calls WU-2-34 or any LLM provider; merge outcomes are derived from validated edits, base snapshots, hashes, policy, and conflict records.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-35, WU-2-37, WU-2-38.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerEditMergeService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-37: ConflictOnStaleBaseClassifier

**Parent initiative:** VS-010: Conflict-on-stale-base classifier

**Contract:**
```text
contract_owner: ConflictOnStaleBaseClassifier
contract_kind: Method-bearing classifier
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
ConflictOnStaleBaseClassifier.classify(edit_id, base_snapshot_id, current_snapshot_id) -> ConflictClassification.
```

**Test boundary:** product-strategy/contracts/wu-2-37-conflictonstalebaseclassifier.md; src-tauri/src/contracts/wu_2_37.rs; src/contracts/wu_2_37.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-37/

**Code boundary:** src-tauri/src/services/wu_2_37.rs; src-tauri/src/commands/wu_2_37.rs when IPC is declared; src/features/wu_2_37/**/* for UI WUs; src-tauri/tests/wu_2_37_contract.rs; src/test/wu_2_37.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `ConflictOnStaleBaseClassifier.classify` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Conflict-on-stale-base ambiguity dispatches only through WU-2-34 OptimizerModelInvocationAdapter with task_class `conflict_on_stale_base` routed to Claude Opus 4.7; unavailable arbitration fails closed unless the documented GPT-5.5 human-gated fallback is explicitly enabled.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-35.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** ConflictOnStaleBaseClassifier contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-38: SummaryNodeStaleStateTransitionHandler

**Parent initiative:** VS-010: SummaryNode stale-state transition handler

**Contract:**
```text
contract_owner: SummaryNodeStaleStateTransitionHandler
contract_kind: State machine
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
SummaryNodeStaleStateTransitionHandler.apply_stale_transition(node_id, transition, refs) -> SummaryStaleStateDecision for fresh, stale, refresh_queued, refreshed, conflict_on_stale_base.
```

**Test boundary:** product-strategy/contracts/wu-2-38-summarynodestalestatetransitionhandler.md; src-tauri/src/contracts/wu_2_38.rs; src/contracts/wu_2_38.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-38/

**Code boundary:** src-tauri/src/services/wu_2_38.rs; src-tauri/src/commands/wu_2_38.rs when IPC is declared; src/features/wu_2_38/**/* for UI WUs; src-tauri/tests/wu_2_38_contract.rs; src/test/wu_2_38.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `SummaryNodeStaleStateTransitionHandler.apply_stale_transition` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Summary stale_state fresh -> stale succeeds when newer evidence, configuration, or base snapshot mismatch is detected.
- [ ] Summary stale_state stale -> refresh_queued succeeds when BackendStaleSignalEmitter enqueues a backend_signal OptimizerRequest.
- [ ] Summary stale_state refresh_queued -> refreshed succeeds only after a merged summary_regeneration OptimizerEdit targets the same node.
- [ ] Summary stale_state stale -> conflict_on_stale_base succeeds when the merge classifier opens a ConflictRecord.
- [ ] Invalid transitions fresh -> refreshed, refreshed -> refresh_queued, conflict_on_stale_base -> refreshed, and refresh_queued -> fresh are rejected.
- [ ] Stale-state decisions are deterministic-first over TTL, content hash, dependency hash, and evidence invalidation; ambiguity may call WU-2-34 only with task_class `stale_mark_detection` routed to MiniMax-M2.7.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-33, WU-2-35, WU-2-50.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** SummaryNodeStaleStateTransitionHandler contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-39: BackendStaleSignalEmitter

**Parent initiative:** VS-010: Backend stale-signal optimizer request emitter

**Contract:**
```text
contract_owner: BackendStaleSignalEmitter
contract_kind: Method-bearing queue emitter
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
BackendStaleSignalEmitter.enqueue_backend_signal(target_node_id, stale_reasons, base_snapshot_id) -> OptimizerRequestRef with source_type backend_signal.
```

**Test boundary:** product-strategy/contracts/wu-2-39-backendstalesignalemitter.md; src-tauri/src/contracts/wu_2_39.rs; src/contracts/wu_2_39.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-39/

**Code boundary:** src-tauri/src/services/wu_2_39.rs; src-tauri/src/commands/wu_2_39.rs when IPC is declared; src/features/wu_2_39/**/* for UI WUs; src-tauri/tests/wu_2_39_contract.rs; src/test/wu_2_39.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `BackendStaleSignalEmitter.enqueue_backend_signal` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-29, WU-2-33.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** BackendStaleSignalEmitter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-40: SummaryRegenerationUIComponent

**Parent initiative:** VS-010: Summary regeneration UI component

**Contract:**
```text
contract_owner: SummaryRegenerationUIComponent
contract_kind: Method-bearing UI component
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
SummaryRegenerationUIComponent renders pending, validated, merged, rejected, conflicted, and reverted summary regeneration state.
```

**Test boundary:** product-strategy/contracts/wu-2-40-summaryregenerationuicomponent.md; src-tauri/src/contracts/wu_2_40.rs; src/contracts/wu_2_40.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-40/

**Code boundary:** src-tauri/src/services/wu_2_40.rs; src-tauri/src/commands/wu_2_40.rs when IPC is declared; src/features/wu_2_40/**/* for UI WUs; src-tauri/tests/wu_2_40_contract.rs; src/test/wu_2_40.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] The UI renders loading, empty, happy, denied/blocked, failed, and stale-data states without issuing backend mutations from render-only interactions.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-36, WU-2-38, WU-2-49.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** SummaryRegenerationUIComponent contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-41: StaleMarkerUIComponent

**Parent initiative:** VS-010: Stale marker UI component

**Contract:**
```text
contract_owner: StaleMarkerUIComponent
contract_kind: Method-bearing UI component
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
StaleMarkerUIComponent renders stale reasons, evidence age, base snapshot, and conflict-on-stale-base markers without treating them as graph truth before merge.
```

**Test boundary:** product-strategy/contracts/wu-2-41-stalemarkeruicomponent.md; src-tauri/src/contracts/wu_2_41.rs; src/contracts/wu_2_41.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-41/

**Code boundary:** src-tauri/src/services/wu_2_41.rs; src-tauri/src/commands/wu_2_41.rs when IPC is declared; src/features/wu_2_41/**/* for UI WUs; src-tauri/tests/wu_2_41_contract.rs; src/test/wu_2_41.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] The UI renders loading, empty, happy, denied/blocked, failed, and stale-data states without issuing backend mutations from render-only interactions.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-38, WU-2-39.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** StaleMarkerUIComponent contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-42: OptimizerLogUI

**Parent initiative:** VS-010: Optimizer log UI

**Contract:**
```text
contract_owner: OptimizerLogUI
contract_kind: Method-bearing UI surface
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerLogUI renders advisory requests plus accepted, ignored, converted, merged, rejected, conflicted, and reverted optimizer-owned edits.
```

**Test boundary:** product-strategy/contracts/wu-2-42-optimizerlogui.md; src-tauri/src/contracts/wu_2_42.rs; src/contracts/wu_2_42.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-42/

**Code boundary:** src-tauri/src/services/wu_2_42.rs; src-tauri/src/commands/wu_2_42.rs when IPC is declared; src/features/wu_2_42/**/* for UI WUs; src-tauri/tests/wu_2_42_contract.rs; src/test/wu_2_42.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] The UI renders loading, empty, happy, denied/blocked, failed, and stale-data states without issuing backend mutations from render-only interactions.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-36, WU-2-39, WU-2-44.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerLogUI contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-43: SummaryRefreshFixturePack

**Parent initiative:** VS-010: Summary refresh fixture pack

**Contract:**
```text
contract_owner: SummaryRefreshFixturePack
contract_kind: Fixture pack
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
SummaryRefreshFixturePack includes queued request, scoped snapshot, turn decomposition, DetailRecord validation, detail injection routing, incremental summary update, requires_full_regen, full regeneration, stale detection, valid summary regeneration, stale mark, validation failure, stale-base conflict, reviewer-sampled hold, and merge-success fixtures.
SessionOverrideContract coverage includes WU-0C-N1 fake adapter success/refusal, WU-0C-N3 schema-probe/safe-import gating, preimage mismatch, session busy, unsupported storage, and WU-0C-N5 override receipt propagation.
Direct JSONL mutation fixtures are forbidden outside WU-0C-N3 adapter fixtures.
```

**Test boundary:** product-strategy/contracts/wu-2-43-summaryrefreshfixturepack.md; src-tauri/src/contracts/wu_2_43.rs; src/contracts/wu_2_43.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-43/

**Code boundary:** src-tauri/src/services/wu_2_43.rs; src-tauri/src/commands/wu_2_43.rs when IPC is declared; src/features/wu_2_43/**/* for UI WUs; src-tauri/tests/wu_2_43_contract.rs; src/test/wu_2_43.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass; stale bases open or reference a ConflictRecord and never overwrite current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.
- [ ] Every fixture named in the Contract is present as a stable file or fixture builder with one positive and one negative contract assertion, using Phase 0A temp harness/fake agents and Phase 0B/0C/1 contracts without alternate schemas.
- [ ] Fixture coverage includes the full turn-decomposition -> DetailRecord validation -> detail injection routing -> incremental summary update -> full regeneration or stale-mark-detection pipeline, including one no-fit new-node creation request and one ambiguous parked detail.
- [ ] Fixture coverage asserts WU-2-34 per-task model routing for MiniMax-M2.7 routine tasks, Claude Opus 4.7 conflict arbitration, Claude Sonnet 4.6 lead routing, GPT-5.5 reviewer sampling, and excluded-provider failures for GLM/Gemini/Qwen/Mistral/DeepSeek.
- [ ] Fixture coverage includes WU-0C-N1 fake adapter success/refusal, WU-0C-N3 schema-probe/safe-import gating, preimage mismatch, session busy, unsupported storage, and WU-0C-N5 override receipt propagation, with no direct provider JSONL open/truncate/rewrite/append/locate fixture outside WU-0C-N3.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-29, WU-2-30, WU-2-31, WU-2-32, WU-2-33, WU-2-34, WU-2-35, WU-2-36, WU-2-37, WU-2-38, WU-2-39, WU-2-40, WU-2-41, WU-2-42, WU-2-44, WU-2-45, WU-2-46, WU-2-47, WU-2-48, WU-2-49, WU-2-50.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37, WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** SummaryRefreshFixturePack contract and tests; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-44: OptimizerEditAuditEmitter

**Parent initiative:** VS-010: Optimizer edit audit emitter

**Contract:**
```text
contract_owner: OptimizerEditAuditEmitter
contract_kind: Method-bearing audit emitter
Consumes queued OptimizerRequest and immutable GraphSnapshot data.
Creates optimizer-owned summary_regeneration or stale_mark OptimizerEdit drafts only.
Merge outcomes are merged, conflicted, rejected, or deferred; current orchestrator turn is never mutated.
OptimizerEditAuditEmitter.emit_optimizer_edit_event(edit_id, lifecycle_state, refs) -> AuditEventRef.
```

**Test boundary:** product-strategy/contracts/wu-2-44-optimizereditauditemitter.md; src-tauri/src/contracts/wu_2_44.rs; src/contracts/wu_2_44.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-44/

**Code boundary:** src-tauri/src/services/wu_2_44.rs; src-tauri/src/commands/wu_2_44.rs when IPC is declared; src/features/wu_2_44/**/* for UI WUs; src-tauri/tests/wu_2_44_contract.rs; src/test/wu_2_44.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `OptimizerEditAuditEmitter.emit_optimizer_edit_event` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, and stale idempotency keys are rejected with exact documented error variants.
- [ ] The WU can merge as a single-concern PR without enabling a later Phase 2 or Phase 3+ behavior by accident.
- [ ] The implementation records TraceContext or AuditEvent refs for every accepted, rejected, denied, blocked, or failed operation path declared by the contract.
- [ ] The implementation does not redefine upstream Phase 0A, Phase 0B, Phase 0C, or Phase 1 DTO names, enum values, repository ownership, or service boundaries.
- [ ] The optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2; cross_reference, repack, split, merge, reparent, provenance_repair, and poison_quarantine drafts are rejected or left disabled.
- [ ] Every optimizer draft cites base_graph_snapshot_id, configuration_id, evidence_ids, and expected_invariants before validation can pass.
- [ ] A stale base opens or references a ConflictRecord and never overwrites current graph truth silently.
- [ ] Merged edits become visible only to a later snapshot or render, never to the in-flight orchestrator turn.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-35, WU-2-36, WU-2-37, WU-2-38, WU-2-39, WU-2-48, WU-2-49, WU-2-50.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** OptimizerEditAuditEmitter contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-45: DetailRecordSchemaDto

**Parent initiative:** VS-010: DetailRecord canonical schema DTO

**Contract:**
```text
contract_owner: DetailRecordSchemaDto
contract_kind: DTO
Consumes WU-0C-N2-derived canonical `TranscriptTurn` evidence, immutable transcript turn bundles, tool-call provenance refs, evidence pointers, graph/entity refs, and base GraphSnapshot refs.
Creates canonical DetailRecord DTOs only; no graph node, edge, summary, or optimizer edit is merged by this WU.
DetailRecord { detail_id, turn_id, detail_type, scope, subject_refs, content, raw_span_refs, evidence_ids, confidence, source_fidelity, temporal, entities, relationships, intent, candidate_node_ids, routing_reason, dedupe_key, base_graph_snapshot_id }.
raw_span_refs and turn refs are canonical TranscriptTurn/source-offset references derived from WU-0C-N2 DTOs; provider-native JSONL bodies and mutable transcript handles are rejected as DTO input.
detail_type enum: observation, decision, constraint, tool_event, artifact, blocker, question, action_item, summary_delta, conflict_signal, entity_fact, relationship_fact.
scope enum: user, assistant, tool, system, optimizer, worker.
source_fidelity enum: verbatim, paraphrase, inference, proposal.
Source basis: proposal.md Round 6 §1 Context-Management Model Assignment; engineering-roadmap.md r5 VS-010; Phase 0C r5 WU-0C-N2; research/16-transcript-turn-decomposition.md §1.3; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-45-detailrecordschemadto.md; src-tauri/src/contracts/wu_2_45.rs; src/contracts/wu_2_45.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-45/

**Code boundary:** src-tauri/src/services/wu_2_45.rs; src-tauri/src/commands/wu_2_45.rs when IPC is declared; src/features/wu_2_45/**/* for UI WUs; src-tauri/tests/wu_2_45_contract.rs; src/test/wu_2_45.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] Unknown enum values, missing required refs, malformed opaque IDs, invalid confidence values, and stale idempotency or dedupe keys are rejected with exact documented error variants.
- [ ] Every detail_type, scope, and source_fidelity variant has one positive fixture and one invalid or missing-field rejection fixture.
- [ ] Every DetailRecord fixture includes at least one raw_span_ref or evidence_id; records with neither are rejected.
- [ ] raw_span_refs and turn refs validate as WU-0C-N2-derived canonical TranscriptTurn/source-offset evidence; non-canonical provider transcript pointers are rejected.
- [ ] Provider-native JSONL bodies, mutable transcript handles, file descriptors, SQLite handles, and adapter parser objects are rejected as DetailRecord input.
- [ ] detail_id and dedupe_key are stable across equivalent inputs and differ for non-equivalent turn spans.
- [ ] The WU can merge as a single-concern PR without enabling turn decomposition, routing, summary update, regeneration, or stale detection behavior by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: none.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37, WU-0C-N2.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** DetailRecordSchemaDto contract and tests; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-29, WU-2-33.

### WU-2-46: TurnDecompositionService

**Parent initiative:** VS-010: Turn decomposition service

**Contract:**
```text
contract_owner: TurnDecompositionService
contract_kind: Method-bearing service
Consumes canonical WU-0C-N2 `TranscriptTurn` / bundle evidence from WU-0C-N1 reads or normalized Phase 0C session-turn evidence, plus tool-call provenance and evidence refs.
Creates DetailRecord[] candidates only; no graph node, edge, summary, or optimizer edit is merged by this WU.
TurnDecompositionService.decompose_turn_bundle(bundle_ref, max_adjacent_turns, base_graph_snapshot_id) -> DetailRecord[].
If decomposition emits canonical synthetic turns or packed transcript deltas for session visibility, session write-back must call WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript` with WU-0C-N2 preconditions and propagate WU-0C-N5 receipts/refusals.
The service never opens, parses, truncates, rewrites, appends, or locates provider-native JSONL directly.
Source basis: proposal.md Round 6 §1 Context-Management Model Assignment; engineering-roadmap.md r5 VS-010; Phase 0C r5 WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5; research/16-transcript-turn-decomposition.md §§1 and 7.7; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-46-turndecompositionservice.md; src-tauri/src/contracts/wu_2_46.rs; src/contracts/wu_2_46.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-46/

**Code boundary:** src-tauri/src/services/wu_2_46.rs; src-tauri/src/commands/wu_2_46.rs when IPC is declared; src/features/wu_2_46/**/* for UI WUs; src-tauri/tests/wu_2_46_contract.rs; src/test/wu_2_46.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `TurnDecompositionService.decompose_turn_bundle` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] The service accepts one turn or a bounded adjacent-turn batch from one bundle stream and rejects non-adjacent, cross-session, or unbounded transcript input.
- [ ] Accepted turn input is WU-0C-N2 canonical TranscriptTurn / bundle evidence from WU-0C-N1 reads or normalized Phase 0C session-turn evidence; raw provider JSONL input is rejected.
- [ ] Each emitted DetailRecord includes detail_id, turn_id, detail_type, source_fidelity, raw_span_refs or evidence_ids, dedupe_key, and base_graph_snapshot_id.
- [ ] Missing evidence pointers, unsupported detail_type values, malformed tool-call provenance, or prompt output that fails WU-2-45 validation causes rejection or retry with a smaller batch.
- [ ] Model dispatch goes only through WU-2-34 OptimizerModelInvocationAdapter with task_class `turn_decomposition` routed to MiniMax-M2.7, with MiniMax-M2.7-highspeed only as the routine fallback.
- [ ] The service never dispatches to GLM, Gemini, Qwen, Mistral, DeepSeek, Claude Opus, Claude Sonnet, or GPT for turn decomposition.
- [ ] When decomposition emits canonical synthetic turns or packed transcript deltas for session visibility, write-back calls only WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript` with WU-0C-N2 preconditions.
- [ ] WU-0C-N5 override receipts and refusal reasons are propagated into DetailRecord evidence or rejection output without being rewritten or swallowed.
- [ ] The service never opens, locates, parses, truncates, rewrites, or appends provider-native JSONL directly.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-34, WU-2-45.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37, WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** TurnDecompositionService contract and tests; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-35, WU-2-50.

### WU-2-47: DetailInjectionRouterService

**Parent initiative:** VS-010: Detail injection router service

**Contract:**
```text
contract_owner: DetailInjectionRouterService
contract_kind: Method-bearing service
Consumes DetailRecord[] candidates, graph node summaries, entity indices, dependency hashes, and base GraphSnapshot refs.
Creates InjectionPlan proposals only; graph mutation remains a later optimizer/topology merge concern.
DetailInjectionRouterService.route_details(detail_records, candidate_node_scope, base_graph_snapshot_id) -> InjectionPlan[].
Optional session-visible output path emits canonical turns or a packed transcript delta, then calls WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript`, and returns the WU-0C-N5 receipt/refusal.
The router never opens, truncates, rewrites, appends, or locates per-CLI JSONL directly.
InjectionPlan { base_graph_snapshot_id, detail_id, action, target_node_id?, target_revision_id?, new_node_template?, edge_candidate?, rationale, evidence_ids, expected_invariants }.
action enum: append_to_node, create_node, create_edge_candidate, park, defer, quarantine, drop.
Source basis: proposal.md Round 6 §1 Context-Management Model Assignment; engineering-roadmap.md r5 VS-010; Phase 0C r5 WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5; research/16-transcript-turn-decomposition.md §§2.1-2.7; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-47-detailinjectionrouterservice.md; src-tauri/src/contracts/wu_2_47.rs; src/contracts/wu_2_47.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-47/

**Code boundary:** src-tauri/src/services/wu_2_47.rs; src-tauri/src/commands/wu_2_47.rs when IPC is declared; src/features/wu_2_47/**/* for UI WUs; src-tauri/tests/wu_2_47_contract.rs; src/test/wu_2_47.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `DetailInjectionRouterService.route_details` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] Exact graph node ID, entity ID, file path, symbol, issue ID, URL, and dependency-hash matches are considered before any model-backed routing call.
- [ ] High-confidence deterministic matches emit append_to_node plans without invoking WU-2-34.
- [ ] Ambiguous routing calls WU-2-34 only with task_class `detail_injection_routing` routed to MiniMax-M2.7; low-confidence results are parked or deferred instead of escalated to a premium model.
- [ ] No-fit durable, evidence-backed, type-valid details emit create_node plans; unsupported or privilege-unsafe records emit drop or quarantine plans.
- [ ] create_edge_candidate output is proposal-only and cannot create GraphEdge truth in Phase 2.
- [ ] Graph/topology mutation remains proposal-only; no InjectionPlan can create or update GraphNode, GraphEdge, NodeRevision, SummaryContract, IdentityEvent, or OptimizerEdit truth in this WU.
- [ ] Optional session-visible output emits only canonical turns or a packed transcript delta and writes only through WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript`.
- [ ] WU-0C-N5 override receipts/refusals for session-visible output are returned to callers and attached to evidence refs without being converted into silent success.
- [ ] The router never opens, locates, truncates, rewrites, or appends per-CLI JSONL directly.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-30, WU-2-34, WU-2-45, WU-2-46.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37, WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** DetailInjectionRouterService contract and tests; no unlisted downstream behavior.
**Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.
**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-37, WU-2-38.

### WU-2-48: IncrementalSummaryUpdateService

**Parent initiative:** VS-010: Incremental summary update service

**Contract:**
```text
contract_owner: IncrementalSummaryUpdateService
contract_kind: Method-bearing service
Consumes existing SummaryContract, selected current node detail inventory, new DetailRecord[], invalidated detail refs, evidence pointers, conflicts, and summary template refs.
Creates summary_regeneration OptimizerEdit candidates and an explicit requires_full_regen signal; merge remains WU-2-36.
IncrementalSummaryUpdateService.update_summary_incrementally(node_id, base_revision_id, base_summary_contract_id, detail_records, invalidated_detail_refs, drift_threshold) -> IncrementalSummaryUpdateResult.
IncrementalSummaryUpdateResult { summary_contract_candidate, changed_claims, removed_claims, requires_full_regen, drift_reason?, evidence_ids, expected_invariants }.
Source basis: proposal.md Round 4 §1 Context-Management Model Assignment; engineering-roadmap.md r3 VS-010; research/16-transcript-turn-decomposition.md §§3.3-3.5; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-48-incrementalsummaryupdateservice.md; src-tauri/src/contracts/wu_2_48.rs; src/contracts/wu_2_48.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-48/

**Code boundary:** src-tauri/src/services/wu_2_48.rs; src-tauri/src/commands/wu_2_48.rs when IPC is declared; src/features/wu_2_48/**/* for UI WUs; src-tauri/tests/wu_2_48_contract.rs; src/test/wu_2_48.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `IncrementalSummaryUpdateService.update_summary_incrementally` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] The prompt input includes existing SummaryContract plus selected current detail inventory, new DetailRecord[] records, invalidated details, evidence pointer table, known conflicts, local template, and budget caps; old summary plus new facts alone is rejected.
- [ ] Every new, changed, or removed claim cites evidence_ids or a documented invalidation reason.
- [ ] requires_full_regen is true when drift_threshold is exceeded, evidence support becomes too diffuse, identity/topology/evidence repair changes exceed incremental scope, or the model reports uncertainty that cannot be validated.
- [ ] Model dispatch goes only through WU-2-34 with task_class `incremental_summary_update` routed to MiniMax-M2.7, with MiniMax-M2.7-highspeed only as the routine fallback.
- [ ] Unsupported claims, uncited deletions, missing evidence, or summary contract validation failures reject the candidate and either retry with smaller deltas or trigger WU-2-49 full regeneration.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-33, WU-2-34, WU-2-45, WU-2-47.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** IncrementalSummaryUpdateService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-36, WU-2-41.

### WU-2-49: FullSummaryRegenerationService

**Parent initiative:** VS-010: Full summary regeneration service

**Contract:**
```text
contract_owner: FullSummaryRegenerationService
contract_kind: Method-bearing service
Consumes a bounded graph node, source detail inventory, evidence pointer table, current SummaryContract, stale marks, conflicts, and summary template refs.
Creates full summary_regeneration OptimizerEdit candidates only; merge remains WU-2-36.
FullSummaryRegenerationService.regenerate_summary(node_id, base_revision_id, trigger, bounded_node_payload) -> FullSummaryRegenerationResult.
trigger enum: requires_full_regen, stale_mark_elevation, evidence_repair, identity_or_topology_change, template_change.
Node payload must fit below the 200K total prompt budget for summary + cross_references + full_content plus output reserve.
Source basis: proposal.md Round 4 §1 Context-Management Model Assignment; engineering-roadmap.md r3 VS-010 and VS-012 node-size axiom; research/16-transcript-turn-decomposition.md §§3.2-3.5; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-49-fullsummaryregenerationservice.md; src-tauri/src/contracts/wu_2_49.rs; src/contracts/wu_2_49.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-49/

**Code boundary:** src-tauri/src/services/wu_2_49.rs; src-tauri/src/commands/wu_2_49.rs when IPC is declared; src/features/wu_2_49/**/* for UI WUs; src-tauri/tests/wu_2_49_contract.rs; src/test/wu_2_49.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `FullSummaryRegenerationService.regenerate_summary` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] The service runs only when triggered by requires_full_regen, stale-mark elevation, evidence repair, identity_or_topology_change, or template_change.
- [ ] Payloads at or above the 200K total prompt budget are rejected and routed to later repack planning instead of a larger-context premium model.
- [ ] The model input is grounded in source detail inventory and evidence pointers rather than recursive summary-only summarization.
- [ ] Model dispatch goes only through WU-2-34 with task_class `full_summary_regeneration` routed to MiniMax-M2.7, with MiniMax-M2.7-highspeed only as the routine fallback.
- [ ] Every regenerated claim cites evidence_ids and every dropped claim cites evidence invalidation, scope removal, conflict, or supersession.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-33, WU-2-34, WU-2-48, WU-2-50.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** FullSummaryRegenerationService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-44.

### WU-2-50: StaleMarkDetectionService

**Parent initiative:** VS-010: Stale-mark detection service

**Contract:**
```text
contract_owner: StaleMarkDetectionService
contract_kind: Method-bearing service
Consumes SummaryContract refs, detail inventory hashes, evidence pointer hashes, dependency hashes, TTL policy, configuration hashes, conflict refs, and base GraphSnapshot refs.
Creates SummaryStaleStateDecision and stale_mark OptimizerEdit candidates only; state transitions remain WU-2-38 and merge remains WU-2-36.
StaleMarkDetectionService.detect_stale_marks(node_ids, base_graph_snapshot_id, deterministic_inputs) -> StaleMarkDetectionBatch.
stale label enum: current, possibly_stale, stale_missing_evidence, stale_new_detail, stale_conflict, stale_identity_change, stale_topology_change, stale_template_change, needs_full_regen.
Source basis: proposal.md Round 4 §1 Context-Management Model Assignment; engineering-roadmap.md r3 VS-010; research/16-transcript-turn-decomposition.md §§4.1-4.9; research/17-v4-per-task-model-assignment.md §7.
```

**Test boundary:** product-strategy/contracts/wu-2-50-stalemarkdetectionservice.md; src-tauri/src/contracts/wu_2_50.rs; src/contracts/wu_2_50.ts when TypeScript DTOs are present; product-strategy/contracts/fixtures/wu-2-50/

**Code boundary:** src-tauri/src/services/wu_2_50.rs; src-tauri/src/commands/wu_2_50.rs when IPC is declared; src/features/wu_2_50/**/* for UI WUs; src-tauri/tests/wu_2_50_contract.rs; src/test/wu_2_50.test.tsx when UI is present

**Acceptance criteria:**
- [ ] The declared contract round-trips through Rust serde and TypeScript fixtures with every field, enum value, optional ref, and opaque ID preserved.
- [ ] The `StaleMarkDetectionService.detect_stale_marks` method has binary tests for its happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant.
- [ ] TTL, content-hash, evidence-pointer-hash, dependency-hash, configuration-hash, and conflict-ref checks run before any model-backed classification.
- [ ] Deterministic current, possibly_stale, stale_new_detail, stale_missing_evidence, stale_conflict, stale_identity_change, stale_topology_change, stale_template_change, and needs_full_regen labels are emitted without model calls when inputs are conclusive.
- [ ] Ambiguous stale classification calls WU-2-34 only with task_class `stale_mark_detection` routed to MiniMax-M2.7; if MiniMax-M2.7 is unavailable, the decision remains queued or conservatively stale rather than dispatching to another model.
- [ ] The service favors conservative stale labeling over false-current output when model confidence or evidence support is below threshold.
- [ ] Detection output feeds WU-2-38 state transitions and never mutates SummaryContract truth directly.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = separate gpt-high with contract-only access; Phase 6c code = separate gpt-high with contracts plus tests; Phase 7 CodeRabbit; Phase 8 PR review; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 2 internal: WU-2-33, WU-2-34.
- Cross-phase incoming from Phase 0A: WU-0A-01, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b.
- Cross-phase incoming from Phase 0B: WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-04, WU-0B-05, WU-0B-07, WU-0B-08, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15, WU-0B-16, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-32.
- Cross-phase incoming from Phase 0C: WU-0C-04, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b, WU-0C-29, WU-0C-29d, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming from Phase 1: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-24, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44.

**Produces:** StaleMarkDetectionService contract and tests; no unlisted downstream behavior.

**Parallelizable with:** topological peers that do not share files; primary candidates in the same slice are WU-2-35, WU-2-46.

## Dependency Graph

Internal Phase 2 graph in topological order:

- Wave 1 - leaf DTOs, enums, guards, and adapters that depend only on upstream phases: WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-13, WU-2-20, WU-2-21, WU-2-28, WU-2-29, WU-2-33, WU-2-45
- Wave 2 - first validators, turn adapters, scoping, lifecycle audit, and stale signal: WU-2-08, WU-2-14, WU-2-22, WU-2-23, WU-2-24, WU-2-27, WU-2-30, WU-2-39
- Wave 3 - mutation service, tool affordances, render preparation, graph action, and prompt schema: WU-2-07, WU-2-11, WU-2-15, WU-2-19, WU-2-31
- Wave 4 - invalidation, navigation audit, turn commit, response schema, and optimizer model invocation: WU-2-09, WU-2-10, WU-2-17, WU-2-32, WU-2-34
- Wave 5 - navigation fixtures, request enqueue, optimizer validation, turn decomposition, and stale-mark detection: WU-2-12, WU-2-18, WU-2-35, WU-2-46, WU-2-50
- Wave 6 - central bridge, stale classification, stale transition handling, and detail injection routing: WU-2-16, WU-2-37, WU-2-38, WU-2-47
- Wave 7 - turn UI, optimizer merge, stale marker UI, and incremental summary update: WU-2-25, WU-2-36, WU-2-41, WU-2-48
- Wave 8 - turn fixtures and full summary regeneration: WU-2-26, WU-2-49
- Wave 9 - summary regeneration UI and optimizer audit: WU-2-40, WU-2-44
- Wave 10 - optimizer log: WU-2-42
- Wave 11 - summary refresh and context-management fixtures: WU-2-43

Explicit internal edges:

- WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-08 -> WU-2-07
- WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05 -> WU-2-08
- WU-2-07 -> WU-2-09
- WU-2-07 -> WU-2-10
- WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-08 -> WU-2-11
- WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-07, WU-2-08, WU-2-09, WU-2-10, WU-2-11 -> WU-2-12
- WU-2-13 -> WU-2-14
- WU-2-14 -> WU-2-15
- WU-2-11, WU-2-14, WU-2-15, WU-2-17, WU-2-18, WU-2-20, WU-2-22, WU-2-23, WU-2-24, WU-2-28 -> WU-2-16
- WU-2-14, WU-2-19, WU-2-27 -> WU-2-17
- WU-2-17 -> WU-2-18
- WU-2-14 -> WU-2-19
- WU-2-13 -> WU-2-22
- WU-2-13 -> WU-2-23
- WU-2-13 -> WU-2-24
- WU-2-14, WU-2-16, WU-2-17, WU-2-18, WU-2-20 -> WU-2-25
- WU-2-13, WU-2-14, WU-2-15, WU-2-16, WU-2-17, WU-2-18, WU-2-19, WU-2-20, WU-2-21, WU-2-22, WU-2-23, WU-2-24, WU-2-25, WU-2-27, WU-2-28 -> WU-2-26
- WU-2-13 -> WU-2-27
- WU-2-29 -> WU-2-30
- WU-2-30 -> WU-2-31
- WU-2-31 -> WU-2-32
- WU-2-31 -> WU-2-34
- WU-2-30, WU-2-32, WU-2-33 -> WU-2-35
- WU-2-35, WU-2-37, WU-2-38 -> WU-2-36
- WU-2-35 -> WU-2-37
- WU-2-33, WU-2-34 -> WU-2-50
- WU-2-34, WU-2-45 -> WU-2-46
- WU-2-30, WU-2-34, WU-2-45, WU-2-46 -> WU-2-47
- WU-2-33, WU-2-34, WU-2-45, WU-2-47 -> WU-2-48
- WU-2-33, WU-2-34, WU-2-48, WU-2-50 -> WU-2-49
- WU-2-33, WU-2-35, WU-2-50 -> WU-2-38
- WU-2-29, WU-2-33 -> WU-2-39
- WU-2-36, WU-2-38, WU-2-49 -> WU-2-40
- WU-2-38, WU-2-39 -> WU-2-41
- WU-2-36, WU-2-39, WU-2-44 -> WU-2-42
- WU-2-29, WU-2-30, WU-2-31, WU-2-32, WU-2-33, WU-2-34, WU-2-35, WU-2-36, WU-2-37, WU-2-38, WU-2-39, WU-2-40, WU-2-41, WU-2-42, WU-2-44, WU-2-45, WU-2-46, WU-2-47, WU-2-48, WU-2-49, WU-2-50 -> WU-2-43
- WU-2-35, WU-2-36, WU-2-37, WU-2-38, WU-2-39, WU-2-48, WU-2-49, WU-2-50 -> WU-2-44

Cross-phase incoming edges are enumerated per WU in the Dependencies block above and systematically restated in Stitch Notes below.

## Critical Path

Longest Phase 2 critical path:

1. WU-2-01..WU-2-06 navigation command/result contracts
2. WU-2-08 WalkStateTransitionValidator
3. WU-2-07 AgentWalkStateMutationService
4. WU-2-11 NavigationToolAffordanceAdapter
5. WU-2-14 OrchestratorTurnStateMachine
6. WU-2-15 TurnRenderPreparationService
7. WU-2-17 TurnCaptureCommitTransactionService
8. WU-2-18 AdvisoryOptimizerRequestEmitter
9. WU-2-16 OrchestratorBridgeService
10. WU-2-25 TurnUIPaneSurface
11. WU-2-26 OrchestratorTurnFixturePack

Optimizer branch critical path: WU-2-29 -> WU-2-30 -> WU-2-31 -> WU-2-34 -> WU-2-46 -> WU-2-47 -> WU-2-48 -> WU-2-49 -> WU-2-44 -> WU-2-42 -> WU-2-43. This branch can start in parallel after upstream foundations, but merge behavior waits for the same OptimizerRequest, BudgetLedger, evidence, audit, and per-task model-dispatch APIs consumed by VS-009 and Phase 0C OptimizerQueueService/OptimizerCycleStateMachine/OptimizerScheduler.

## Parallelization Map

| Topological level | Work units | Max concurrent worktrees | Notes |
|---|---|---:|---|
| Wave 1 - leaf DTOs, enums, guards, and adapters that depend only on upstream phases | WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-13, WU-2-20, WU-2-21, WU-2-28, WU-2-29, WU-2-33, WU-2-45 | 13 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 2 - first validators, turn adapters, scoping, lifecycle audit, and stale signal | WU-2-08, WU-2-14, WU-2-22, WU-2-23, WU-2-24, WU-2-27, WU-2-30, WU-2-39 | 8 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 3 - mutation service, tool affordances, render preparation, graph action, and prompt schema | WU-2-07, WU-2-11, WU-2-15, WU-2-19, WU-2-31 | 5 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 4 - invalidation, navigation audit, turn commit, response schema, and optimizer model invocation | WU-2-09, WU-2-10, WU-2-17, WU-2-32, WU-2-34 | 5 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 5 - navigation fixtures, request enqueue, optimizer validation, turn decomposition, and stale-mark detection | WU-2-12, WU-2-18, WU-2-35, WU-2-46, WU-2-50 | 5 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 6 - central bridge, stale classification, stale transition handling, and detail injection routing | WU-2-16, WU-2-37, WU-2-38, WU-2-47 | 4 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 7 - turn UI, optimizer merge, stale marker UI, and incremental summary update | WU-2-25, WU-2-36, WU-2-41, WU-2-48 | 4 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 8 - turn fixtures and full summary regeneration | WU-2-26, WU-2-49 | 2 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 9 - summary regeneration UI and optimizer audit | WU-2-40, WU-2-44 | 2 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges; coordinate shared contract index files before parallel PRs. |
| Wave 10 - optimizer log | WU-2-42 | 1 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges. |
| Wave 11 - summary refresh and context-management fixtures | WU-2-43 | 1 | Re-derived as longest-path-from-leaf+1; no intra-wave dependency edges. |

## Run Report

| Check | Result | Evidence |
|---|---|---|
| D1 per-object granularity | PASS in proposer draft | 50 WUs preserved; WU-2-22..24 remain separate provider capability adapters routing launch/resume through WU-0C-18 AgentRunnerClient. No new WUs were added for round 5. |
| D2 binary criteria | PASS in proposer draft | Affected WUs carry 8-12 binary criteria each. WU-2-26 and WU-2-43 fixture criteria now assert AgentRunnerClient and SessionOverrideContract boundaries without direct provider CLI or JSONL mutation fixtures. |
| D3 regression check | PASS in proposer draft | Read Phase 2 r4, Phase 2 r5 audit instructions, proposal r6, engineering-roadmap r5, and Phase 0C r5 WU-0C-N1..WU-0C-N3 plus WU-0C-N5. Re-derived the Parallelization Map as unchanged because round 5 removes stale block-on annotations and the dropped schema-probe split edge, adding no Phase 2 internal dependency edges. |
| D4 watch-signal compliance | HONEST LOW | Round 5 is a minor externally-driven fix-created-family gen 0 edit-pass from the proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 SessionOverrideContract cascade. `dependency-encoding-family` remains LOW after removing the dropped schema-probe split edge; `parallelization-map-family` remains LOW with unchanged 11-wave topology. |

### Rule D1

D1 audit count: 50 rows = 50 WUs.

| WU | D1 owner | Classification |
|---|---|---|
| WU-2-01 | PackToolCommandSchema | Navigation command DTO |
| WU-2-02 | UnpackToolCommandSchema | Navigation command DTO |
| WU-2-03 | FocusToolCommandSchema | Navigation command DTO |
| WU-2-04 | PinToolCommandSchema | Navigation command DTO |
| WU-2-05 | UnpinToolCommandSchema | Navigation command DTO |
| WU-2-06 | NavigationToolResultDto | Navigation result DTO |
| WU-2-07 | AgentWalkStateMutationService | Method-bearing navigation service |
| WU-2-08 | WalkStateTransitionValidator | State machine validator |
| WU-2-09 | RenderInvalidationService | Method-bearing render invalidation service |
| WU-2-10 | NavigationToolCallAuditEmitter | Method-bearing audit emitter |
| WU-2-11 | NavigationToolAffordanceAdapter | Method-bearing tool adapter |
| WU-2-12 | NavigationFixturePack | Fixture pack |
| WU-2-13 | TurnLifecycleEventEnum | Enum |
| WU-2-14 | OrchestratorTurnStateMachine | State machine |
| WU-2-15 | TurnRenderPreparationService | Method-bearing service |
| WU-2-16 | OrchestratorBridgeService | Method-bearing orchestrator brain |
| WU-2-17 | TurnCaptureCommitTransactionService | Method-bearing transaction service |
| WU-2-18 | AdvisoryOptimizerRequestEmitter | Method-bearing queue emitter |
| WU-2-19 | GraphActionTransactionService | Method-bearing transaction service |
| WU-2-20 | CompactDetectionAvoidanceGuard | Method-bearing guard |
| WU-2-21 | ParentInvocationPropagator | Method-bearing propagation service |
| WU-2-22 | ClaudeTurnAdapter | Provider capability adapter over AgentRunnerClient |
| WU-2-23 | CodexTurnAdapter | Provider capability adapter over AgentRunnerClient |
| WU-2-24 | OpencodeTurnAdapter | Provider capability adapter over AgentRunnerClient |
| WU-2-25 | TurnUIPaneSurface | Method-bearing UI surface |
| WU-2-26 | OrchestratorTurnFixturePack | Fixture pack |
| WU-2-27 | TurnLifecycleAuditEmitter | Method-bearing audit emitter |
| WU-2-28 | TurnProviderPreflightAdapter | Method-bearing provider adapter |
| WU-2-29 | OptimizerScopingRequestDto | DTO |
| WU-2-30 | OptimizerScopingService | Method-bearing service |
| WU-2-31 | OptimizerPromptSchemaDto | DTO |
| WU-2-32 | OptimizerResponseSchemaDto | DTO |
| WU-2-33 | SummaryRefreshRequestDto | DTO |
| WU-2-34 | OptimizerModelInvocationAdapter | Method-bearing invocation adapter |
| WU-2-35 | DeterministicOptimizerEditValidator | Method-bearing validator |
| WU-2-36 | OptimizerEditMergeService | Method-bearing merge service |
| WU-2-37 | ConflictOnStaleBaseClassifier | Method-bearing classifier |
| WU-2-38 | SummaryNodeStaleStateTransitionHandler | State machine |
| WU-2-39 | BackendStaleSignalEmitter | Method-bearing queue emitter |
| WU-2-40 | SummaryRegenerationUIComponent | Method-bearing UI component |
| WU-2-41 | StaleMarkerUIComponent | Method-bearing UI component |
| WU-2-42 | OptimizerLogUI | Method-bearing UI surface |
| WU-2-43 | SummaryRefreshFixturePack | Fixture pack |
| WU-2-44 | OptimizerEditAuditEmitter | Method-bearing audit emitter |
| WU-2-45 | DetailRecordSchemaDto | DTO |
| WU-2-46 | TurnDecompositionService | Method-bearing service |
| WU-2-47 | DetailInjectionRouterService | Method-bearing service |
| WU-2-48 | IncrementalSummaryUpdateService | Method-bearing service |
| WU-2-49 | FullSummaryRegenerationService | Method-bearing service |
| WU-2-50 | StaleMarkDetectionService | Method-bearing service |

### Rule D2

- Every command DTO declares field-level positive and negative fixture criteria.
- Every service declares accepted, denied/blocked, failed, and no-out-of-scope-mutation criteria.
- WU-2-08 declares navigation tool-state transition criteria and invalid transition rejections.
- WU-2-14 declares every OrchestratorTurn lifecycle state, every valid transition, and six invalid transition classes.
- WU-2-38 declares SummaryNode stale-state transition criteria and invalid transition rejections.
- WU-2-34 declares exact per-task dispatch criteria and excluded-provider failures for GLM, Gemini, Qwen, Mistral, and DeepSeek.
- WU-2-45 declares every DetailRecord field and enum variant with positive and negative fixtures, and rejects provider-native JSONL bodies or mutable transcript handles as input.
- WU-2-46 and WU-2-47 declare canonical TranscriptTurn/session-write-back boundaries through WU-0C-N1/N2/N3/N5 and forbid direct per-CLI JSONL operations.
- WU-2-48, WU-2-49, and WU-2-50 declare binary model-routing, deterministic-prefilter, failure, and no-out-of-scope-mutation criteria and remain clean in round 5.
- UI WUs declare loading, empty, happy, denied/blocked, failed, and stale-data states.

### Rule D3

- Checked line-count and inventory existence for Phase 0A, Phase 0B, Phase 0C, Phase 1, proposal, engineering roadmap, and audit histories.
- Checked Phase 2 r4 roadmap and Phase 2 r5 audit instructions for the required minor edit-pass scope.
- Checked proposal r6, engineering-roadmap r5, and Phase 0C r5 WU-0C-N1..WU-0C-N3 plus WU-0C-N5 for SessionOverrideContract dependency and write-back boundaries.
- Re-derived the 11-wave Parallelization Map as unchanged because no Phase 2 internal edges changed.
- Checked Phase 0C Stitch Notes outgoing to Phase 2+ for WU-0C-N1..WU-0C-N3 plus WU-0C-N5 -> Phase 2 turn-decomposition/detail-injection expectations.
- Did not dispatch the 3-gate risk loop; that is explicitly separate after proposer lands.

### Rule D4

| Watch signal | Phase 2 treatment | Residual risk |
|---|---|---|
| bundling-family | Five navigation command schemas are five WUs; provider capability adapters remain three WUs; optimizer prompt/response/request DTOs are split; DetailRecordSchemaDto and each VS-010 pipeline service are separate WUs; fixture packs are WU-2-12, WU-2-26, WU-2-43. | LOW if reviewers agree no DTO/service pair remains bundled. |
| state-machine-criteria-family | WU-2-08, WU-2-14, and WU-2-38 have explicit transition criteria and invalid-transition rejections. | LOW-MEDIUM because WU-2-14 is deep and should receive reviewer attention. |
| dependency-encoding-family | Every WU has explicit WU-0A/WU-0B/WU-0C/WU-1 IDs; Stitch Notes enumerate incoming pairs. Round 5 removes the dropped schema-probe split edge and keeps WU-0C-N1/N2/N3/N5 only on WU-2-43/45/46/47 as directed; WU-2-48/49/50 remain clean. | LOW after targeted SessionOverrideContract edge pass. |
| parallelization-map-family | Parallelization Map re-derived as unchanged 11-wave topology because round 5 adds no Phase 2 internal edges. | LOW. |
| fix-created-family | Round 5 is fix-created-family gen 0, externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 SessionOverrideContract. This pass drops stale block-on annotations and the obsolete schema-probe split edge without adding speculative WUs. | LOW. |

### Self-classification

- Round 5 externally-driven minor edit-pass: preserved 50 WUs and the 11-wave map; retained the r4 SessionOverrideContract criteria for WU-2-22/23/24/26/43/45/46/47; removed stale block-on annotations and the dropped schema-probe split edge after agent-runner feature requests landed.

## Stitch Notes

### Incoming From Phase 0A

- (WU-0A-01, VS-008)
- (WU-0A-02, VS-008)
- (WU-0A-03, VS-008)
- (WU-0A-04, VS-008)
- (WU-0A-05, VS-008)
- (WU-0A-06, VS-008)
- (WU-0A-07, VS-008)
- (WU-0A-08, VS-008)
- (WU-0A-09, VS-008)
- (WU-0A-10, VS-008)
- (WU-0A-11, VS-008)
- (WU-0A-12, VS-008)
- (WU-0A-13, VS-008)
- (WU-0A-14a, VS-008)
- (WU-0A-14b, VS-008)
- (WU-0A-15, VS-008)
- (WU-0A-01, VS-009)
- (WU-0A-02, VS-009)
- (WU-0A-03, VS-009)
- (WU-0A-04, VS-009)
- (WU-0A-05, VS-009)
- (WU-0A-06, VS-009)
- (WU-0A-07, VS-009)
- (WU-0A-08, VS-009)
- (WU-0A-09, VS-009)
- (WU-0A-10, VS-009)
- (WU-0A-14a, VS-009)
- (WU-0A-14b, VS-009)
- (WU-0A-15, VS-009)
- (WU-0A-01, VS-010)
- (WU-0A-03, VS-010)
- (WU-0A-04, VS-010)
- (WU-0A-05, VS-010)
- (WU-0A-06, VS-010)
- (WU-0A-07, VS-010)
- (WU-0A-08, VS-010)
- (WU-0A-09, VS-010)
- (WU-0A-10, VS-010)
- (WU-0A-11, VS-010)
- (WU-0A-12, VS-010)
- (WU-0A-13, VS-010)
- (WU-0A-14a, VS-010)
- (WU-0A-14b, VS-010)

### Incoming From Phase 0B

- (WU-0B-01, VS-008)
- (WU-0B-02, VS-008)
- (WU-0B-03, VS-008)
- (WU-0B-04, VS-008)
- (WU-0B-07, VS-008)
- (WU-0B-08, VS-008)
- (WU-0B-11, VS-008)
- (WU-0B-14, VS-008)
- (WU-0B-15, VS-008)
- (WU-0B-16, VS-008)
- (WU-0B-20, VS-008)
- (WU-0B-21, VS-008)
- (WU-0B-22, VS-008)
- (WU-0B-23, VS-008)
- (WU-0B-27, VS-008)
- (WU-0B-30, VS-008)
- (WU-0B-01, VS-009)
- (WU-0B-02, VS-009)
- (WU-0B-03, VS-009)
- (WU-0B-04, VS-009)
- (WU-0B-09, VS-009)
- (WU-0B-10, VS-009)
- (WU-0B-14, VS-009)
- (WU-0B-15, VS-009)
- (WU-0B-16, VS-009)
- (WU-0B-17, VS-009)
- (WU-0B-18, VS-009)
- (WU-0B-19, VS-009)
- (WU-0B-20, VS-009)
- (WU-0B-21, VS-009)
- (WU-0B-22, VS-009)
- (WU-0B-23, VS-009)
- (WU-0B-24, VS-009)
- (WU-0B-25, VS-009)
- (WU-0B-27, VS-009)
- (WU-0B-01, VS-010)
- (WU-0B-02, VS-010)
- (WU-0B-03, VS-010)
- (WU-0B-04, VS-010)
- (WU-0B-05, VS-010)
- (WU-0B-07, VS-010)
- (WU-0B-08, VS-010)
- (WU-0B-09, VS-010)
- (WU-0B-10, VS-010)
- (WU-0B-11, VS-010)
- (WU-0B-12, VS-010)
- (WU-0B-13, VS-010)
- (WU-0B-14, VS-010)
- (WU-0B-15, VS-010)
- (WU-0B-16, VS-010)
- (WU-0B-21, VS-010)
- (WU-0B-25, VS-010)
- (WU-0B-26, VS-010)
- (WU-0B-27, VS-010)
- (WU-0B-32, VS-010)

### Incoming From Phase 0C

VS-009 provider capability adapters WU-2-22..24 preserve incoming WU-0C-11a..18 through WU-0C-18 AgentRunnerClient; they do not add WU-0C-N* edges. VS-010 pipeline WUs retain reliance on WU-0C-27 OptimizerQueueService, WU-0C-28 OptimizerCycleStateMachine, and WU-0C-29 OptimizerScheduler for queued cycle execution. Round 5 keeps WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5 only where SessionOverrideContract write-back, DTO evidence, schema-probe gating, fixtures, receipts, or refusals are consumed: WU-2-43, WU-2-45, WU-2-46, and WU-2-47. WU-2-48, WU-2-49, and WU-2-50 remain clean.

- (WU-0C-04, VS-008)
- (WU-0C-05, VS-008)
- (WU-0C-06a, VS-008)
- (WU-0C-06, VS-008)
- (WU-0C-07a, VS-008)
- (WU-0C-07, VS-008)
- (WU-0C-11a, VS-008)
- (WU-0C-11b, VS-008)
- (WU-0C-12a, VS-008)
- (WU-0C-12, VS-008)
- (WU-0C-13a, VS-008)
- (WU-0C-13b, VS-008)
- (WU-0C-13, VS-008)
- (WU-0C-14a, VS-008)
- (WU-0C-14b, VS-008)
- (WU-0C-14, VS-008)
- (WU-0C-15a, VS-008)
- (WU-0C-15b, VS-008)
- (WU-0C-15c, VS-008)
- (WU-0C-15d, VS-008)
- (WU-0C-16a, VS-008)
- (WU-0C-16b, VS-008)
- (WU-0C-16, VS-008)
- (WU-0C-17a, VS-008)
- (WU-0C-17b, VS-008)
- (WU-0C-17, VS-008)
- (WU-0C-18, VS-008)
- (WU-0C-21, VS-008)
- (WU-0C-22a, VS-008)
- (WU-0C-22, VS-008)
- (WU-0C-23a, VS-008)
- (WU-0C-23b, VS-008)
- (WU-0C-23, VS-008)
- (WU-0C-24a, VS-008)
- (WU-0C-24, VS-008)
- (WU-0C-25a, VS-008)
- (WU-0C-25, VS-008)
- (WU-0C-26a, VS-008)
- (WU-0C-26, VS-008)
- (WU-0C-32, VS-008)
- (WU-0C-33a, VS-008)
- (WU-0C-33, VS-008)
- (WU-0C-37a, VS-008)
- (WU-0C-37, VS-008)
- (WU-0C-04, VS-009)
- (WU-0C-05, VS-009)
- (WU-0C-06a, VS-009)
- (WU-0C-06, VS-009)
- (WU-0C-07a, VS-009)
- (WU-0C-07, VS-009)
- (WU-0C-08, VS-009)
- (WU-0C-09a, VS-009)
- (WU-0C-09, VS-009)
- (WU-0C-10, VS-009)
- (WU-0C-11a, VS-009)
- (WU-0C-11b, VS-009)
- (WU-0C-12a, VS-009)
- (WU-0C-12, VS-009)
- (WU-0C-13a, VS-009)
- (WU-0C-13b, VS-009)
- (WU-0C-13, VS-009)
- (WU-0C-14a, VS-009)
- (WU-0C-14b, VS-009)
- (WU-0C-14, VS-009)
- (WU-0C-15a, VS-009)
- (WU-0C-15b, VS-009)
- (WU-0C-15c, VS-009)
- (WU-0C-15d, VS-009)
- (WU-0C-16a, VS-009)
- (WU-0C-16b, VS-009)
- (WU-0C-16, VS-009)
- (WU-0C-17a, VS-009)
- (WU-0C-17b, VS-009)
- (WU-0C-17, VS-009)
- (WU-0C-18, VS-009)
- (WU-0C-19, VS-009)
- (WU-0C-20a, VS-009)
- (WU-0C-20, VS-009)
- (WU-0C-21, VS-009)
- (WU-0C-22a, VS-009)
- (WU-0C-22, VS-009)
- (WU-0C-23a, VS-009)
- (WU-0C-23b, VS-009)
- (WU-0C-23, VS-009)
- (WU-0C-24a, VS-009)
- (WU-0C-24, VS-009)
- (WU-0C-25a, VS-009)
- (WU-0C-25, VS-009)
- (WU-0C-26a, VS-009)
- (WU-0C-26, VS-009)
- (WU-0C-27, VS-009)
- (WU-0C-28, VS-009)
- (WU-0C-29a, VS-009)
- (WU-0C-29b, VS-009)
- (WU-0C-29, VS-009)
- (WU-0C-32, VS-009)
- (WU-0C-33a, VS-009)
- (WU-0C-33, VS-009)
- (WU-0C-34, VS-009)
- (WU-0C-35a, VS-009)
- (WU-0C-35, VS-009)
- (WU-0C-37a, VS-009)
- (WU-0C-37, VS-009)
- (WU-0C-04, VS-010)
- (WU-0C-05, VS-010)
- (WU-0C-06a, VS-010)
- (WU-0C-06, VS-010)
- (WU-0C-07a, VS-010)
- (WU-0C-07, VS-010)
- (WU-0C-08, VS-010)
- (WU-0C-09a, VS-010)
- (WU-0C-09, VS-010)
- (WU-0C-10, VS-010)
- (WU-0C-21, VS-010)
- (WU-0C-22a, VS-010)
- (WU-0C-22, VS-010)
- (WU-0C-23a, VS-010)
- (WU-0C-23b, VS-010)
- (WU-0C-23, VS-010)
- (WU-0C-24a, VS-010)
- (WU-0C-24, VS-010)
- (WU-0C-25a, VS-010)
- (WU-0C-25, VS-010)
- (WU-0C-26a, VS-010)
- (WU-0C-26, VS-010)
- (WU-0C-27, VS-010)
- (WU-0C-28, VS-010)
- (WU-0C-29a, VS-010)
- (WU-0C-29b, VS-010)
- (WU-0C-29, VS-010)
- (WU-0C-29d, VS-010)
- (WU-0C-34, VS-010)
- (WU-0C-35a, VS-010)
- (WU-0C-35, VS-010)
- (WU-0C-36a, VS-010)
- (WU-0C-36, VS-010)
- (WU-0C-37a, VS-010)
- (WU-0C-37, VS-010)
- (WU-0C-N1, VS-010)
- (WU-0C-N2, VS-010)
- (WU-0C-N3, VS-010)
- (WU-0C-N5, VS-010)

### Incoming From Phase 1

- (WU-1-12, VS-008)
- (WU-1-14, VS-008)
- (WU-1-15, VS-008)
- (WU-1-16, VS-008)
- (WU-1-17, VS-008)
- (WU-1-18, VS-008)
- (WU-1-19, VS-008)
- (WU-1-20, VS-008)
- (WU-1-21, VS-008)
- (WU-1-22, VS-008)
- (WU-1-23, VS-008)
- (WU-1-24, VS-008)
- (WU-1-25, VS-008)
- (WU-1-26, VS-008)
- (WU-1-27, VS-008)
- (WU-1-28, VS-008)
- (WU-1-29, VS-008)
- (WU-1-30, VS-008)
- (WU-1-31, VS-008)
- (WU-1-32, VS-008)
- (WU-1-33, VS-008)
- (WU-1-34, VS-008)
- (WU-1-35, VS-008)
- (WU-1-36, VS-008)
- (WU-1-02, VS-009)
- (WU-1-03, VS-009)
- (WU-1-04, VS-009)
- (WU-1-05, VS-009)
- (WU-1-06, VS-009)
- (WU-1-07, VS-009)
- (WU-1-08, VS-009)
- (WU-1-09, VS-009)
- (WU-1-10, VS-009)
- (WU-1-11, VS-009)
- (WU-1-12, VS-009)
- (WU-1-13, VS-009)
- (WU-1-14, VS-009)
- (WU-1-15, VS-009)
- (WU-1-16, VS-009)
- (WU-1-17, VS-009)
- (WU-1-18, VS-009)
- (WU-1-19, VS-009)
- (WU-1-20, VS-009)
- (WU-1-21, VS-009)
- (WU-1-22, VS-009)
- (WU-1-23, VS-009)
- (WU-1-24, VS-009)
- (WU-1-25, VS-009)
- (WU-1-26, VS-009)
- (WU-1-27, VS-009)
- (WU-1-28, VS-009)
- (WU-1-45, VS-009)
- (WU-1-46, VS-009)
- (WU-1-47, VS-009)
- (WU-1-48, VS-009)
- (WU-1-49, VS-009)
- (WU-1-50, VS-009)
- (WU-1-56, VS-009)
- (WU-1-02, VS-010)
- (WU-1-03, VS-010)
- (WU-1-04, VS-010)
- (WU-1-05, VS-010)
- (WU-1-06, VS-010)
- (WU-1-07, VS-010)
- (WU-1-08, VS-010)
- (WU-1-09, VS-010)
- (WU-1-10, VS-010)
- (WU-1-11, VS-010)
- (WU-1-12, VS-010)
- (WU-1-13, VS-010)
- (WU-1-14, VS-010)
- (WU-1-15, VS-010)
- (WU-1-16, VS-010)
- (WU-1-17, VS-010)
- (WU-1-18, VS-010)
- (WU-1-19, VS-010)
- (WU-1-20, VS-010)
- (WU-1-21, VS-010)
- (WU-1-24, VS-010)
- (WU-1-29, VS-010)
- (WU-1-30, VS-010)
- (WU-1-31, VS-010)
- (WU-1-32, VS-010)
- (WU-1-33, VS-010)
- (WU-1-34, VS-010)
- (WU-1-35, VS-010)
- (WU-1-36, VS-010)
- (WU-1-37, VS-010)
- (WU-1-38, VS-010)
- (WU-1-39, VS-010)
- (WU-1-40, VS-010)
- (WU-1-41, VS-010)
- (WU-1-42, VS-010)
- (WU-1-43, VS-010)
- (WU-1-44, VS-010)

### Outgoing To Phase 3+

- (WU-2-01..WU-2-06 navigation command/result contracts, VS-015)
- (WU-2-01..WU-2-06 navigation command/result contracts, VS-017)
- (WU-2-07 AgentWalkStateMutationService, VS-015)
- (WU-2-07 AgentWalkStateMutationService, VS-017)
- (WU-2-08 WalkStateTransitionValidator, VS-012)
- (WU-2-08 WalkStateTransitionValidator, VS-013)
- (WU-2-09 RenderInvalidationService, VS-014)
- (WU-2-09 RenderInvalidationService, VS-015)
- (WU-2-10 NavigationToolCallAuditEmitter, VS-018)
- (WU-2-11 NavigationToolAffordanceAdapter, VS-015)
- (WU-2-13 TurnLifecycleEventEnum, VS-015)
- (WU-2-13 TurnLifecycleEventEnum, VS-016)
- (WU-2-13 TurnLifecycleEventEnum, VS-017)
- (WU-2-14 OrchestratorTurnStateMachine, VS-015)
- (WU-2-14 OrchestratorTurnStateMachine, VS-016)
- (WU-2-14 OrchestratorTurnStateMachine, VS-017)
- (WU-2-14 OrchestratorTurnStateMachine, VS-020)
- (WU-2-16 OrchestratorBridgeService, VS-015)
- (WU-2-16 OrchestratorBridgeService, VS-017)
- (WU-2-17 TurnCaptureCommitTransactionService, VS-018)
- (WU-2-18 AdvisoryOptimizerRequestEmitter, VS-011)
- (WU-2-18 AdvisoryOptimizerRequestEmitter, VS-018)
- (WU-2-18 AdvisoryOptimizerRequestEmitter, VS-020)
- (WU-2-19 GraphActionTransactionService, VS-014)
- (WU-2-20 CompactDetectionAvoidanceGuard, VS-017)
- (WU-2-20 CompactDetectionAvoidanceGuard, VS-020)
- (WU-2-21 ParentInvocationPropagator, VS-015)
- (WU-2-22..WU-2-24 AgentRunnerClient-mediated provider capability adapters, VS-015)
- (WU-2-22..WU-2-24 AgentRunnerClient-mediated provider capability adapters, VS-016)
- (WU-2-25 TurnUIPaneSurface, VS-020)
- (WU-2-29 OptimizerScopingRequestDto, VS-011)
- (WU-2-29 OptimizerScopingRequestDto, VS-012)
- (WU-2-30 OptimizerScopingService, VS-011)
- (WU-2-30 OptimizerScopingService, VS-014)
- (WU-2-31 OptimizerPromptSchemaDto, VS-011)
- (WU-2-31 OptimizerPromptSchemaDto, VS-018)
- (WU-2-32 OptimizerResponseSchemaDto, VS-011)
- (WU-2-32 OptimizerResponseSchemaDto, VS-018)
- (WU-2-35 DeterministicOptimizerEditValidator, VS-012)
- (WU-2-35 DeterministicOptimizerEditValidator, VS-013)
- (WU-2-35 DeterministicOptimizerEditValidator, VS-014)
- (WU-2-36 OptimizerEditMergeService, VS-012)
- (WU-2-36 OptimizerEditMergeService, VS-013)
- (WU-2-36 OptimizerEditMergeService, VS-014)
- (WU-2-36 OptimizerEditMergeService, VS-018)
- (WU-2-36 OptimizerEditMergeService, VS-019)
- (WU-2-37 ConflictOnStaleBaseClassifier, VS-012)
- (WU-2-37 ConflictOnStaleBaseClassifier, VS-013)
- (WU-2-38 SummaryNodeStaleStateTransitionHandler, VS-014)
- (WU-2-38 SummaryNodeStaleStateTransitionHandler, VS-018)
- (WU-2-39 BackendStaleSignalEmitter, VS-011)
- (WU-2-40 SummaryRegenerationUIComponent, VS-014)
- (WU-2-41 StaleMarkerUIComponent, VS-014)
- (WU-2-42 OptimizerLogUI, VS-019)
- (WU-2-44 OptimizerEditAuditEmitter, VS-020)
- (WU-2-45 DetailRecordSchemaDto, VS-011)
- (WU-2-45 DetailRecordSchemaDto, VS-018)
- (WU-2-43 SummaryRefreshFixturePack, VS-012)
- (WU-2-43 SummaryRefreshFixturePack, VS-018)
- (WU-2-46 TurnDecompositionService, VS-012)
- (WU-2-46 TurnDecompositionService, VS-018)
- (WU-2-46 TurnDecompositionService, VS-019)
- (WU-2-47 DetailInjectionRouterService, VS-012)
- (WU-2-47 DetailInjectionRouterService, VS-018)
- (WU-2-48 IncrementalSummaryUpdateService, VS-014)
- (WU-2-48 IncrementalSummaryUpdateService, VS-018)
- (WU-2-48 IncrementalSummaryUpdateService, VS-019)
- (WU-2-49 FullSummaryRegenerationService, VS-014)
- (WU-2-49 FullSummaryRegenerationService, VS-018)
- (WU-2-49 FullSummaryRegenerationService, VS-019)
- (WU-2-50 StaleMarkDetectionService, VS-014)
- (WU-2-50 StaleMarkDetectionService, VS-018)

Engineering-roadmap dependency rows encoded here:

- VS-008 foundations feed later worker dispatch, recovery, and topology/conflict safety through walk-state and navigation audit boundaries.
- VS-009 foundations feed worker launch/recovery phases through the durable turn state machine, AgentRunnerClient-mediated provider capability adapters, compact guard, parent invocation propagation, and advisory OptimizerRequest emission.
- WU-2-46 TurnDecompositionService feeds VS-012 repack planning with canonical transcript turns and detail decomposition; any packed transcript replacement remains WU-0C-N1-mediated and carries WU-0C-N5 receipts/refusals.
- WU-2-47 DetailInjectionRouterService feeds VS-012 with SessionOverrideContract receipts/refusals for packed transcript or canonical-turn write-back; graph/topology mutation remains proposal-only.
- WU-2-46 and WU-2-47 feed VS-018 worker-output reintegration through the same canonical turns, detail routing, and WU-0C-N1 write-back boundary.
- WU-2-43 SummaryRefreshFixturePack feeds VS-012 and VS-018 fixture flow-through for WU-0C-N1 fake adapter success/refusal and WU-0C-N3 idle-only write-back/refusal cases.
- VS-010 foundations feed topology safety, summary/evidence integrity, worker reintegration, cost surfaces, and recovery through optimizer scoping, DetailRecord extraction, detail routing, incremental and full summary maintenance, stale detection, deterministic validation, merge, stale-base conflict, and optimizer log surfaces. WU-2-45..WU-2-50 outgoing edges remain especially relevant to VS-018 staged worker output reintegration because worker output can arrive as DetailRecord-like evidence that must route, summarize, mark stale, and merge through the same optimizer pipeline.

### Non-Ownership Notes

- Phase 2 does not redefine Phase 0B durable schema tables or Phase 0C runtime engines.
- Phase 2 does not own topology mutation, split/merge/reparent behavior, cross-reference creation, repacking, provenance repair, poison quarantine, reviewer sampling orchestration, worker dispatch, question continuation routing, recovery execution, or provider reroute/substitution.
- Phase 2 navigation tools mutate AgentWalkState only; optimizer curation remains optimizer-owned.
- Phase 2 optimizer edits are limited to summary_regeneration and stale_mark behavior; high-consequence edit types stay disabled until Phase 3+.
