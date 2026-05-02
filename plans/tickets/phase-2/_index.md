# Phase 2 Ticket Index

Source: `product-strategy/ai-roadmap-phase-2.md` from Phase 2 r5.

This directory supersedes the prior `tickets-phase-2-r4/plans/tickets/phase-2/` ticket directory. Phase 2 r5 is an audit-driven minor edit-pass from `proposal.md` Round 6, `engineering-roadmap.md` Round 5, and Phase 0C r5 `SessionOverrideContract` WU-0C-N1..WU-0C-N3 plus WU-0C-N5. It preserves the 50-WU count and the 11-wave Parallelization Map while retaining the r4 AgentRunnerClient and SessionOverrideContract boundaries for WU-2-22/23/24/26/43/45/46/47, removing stale block-on annotations, and dropping the obsolete schema-probe split edge.

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

## Regeneration Notes

- Full regeneration target: one ticket per WU, `WU-2-01.md` through `WU-2-50.md`.
- WU-2-22, WU-2-23, and WU-2-24 are provider capability adapters over WU-0C-18 AgentRunnerClient evidence, with no direct per-CLI launch/resume fallback.
- WU-2-26 fixture coverage asserts `agents` / AgentRunnerClient invocation and normalized session evidence instead of direct provider CLI fixtures.
- WU-2-43, WU-2-45, WU-2-46, and WU-2-47 carry the targeted WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5 SessionOverrideContract dependencies from the r5 cascade.
- WU-2-46 and WU-2-47 must use WU-0C-N1 write methods, WU-0C-N3 schema-probe/safe-import gating, and WU-0C-N5 receipts/refusals for session-visible output; direct provider JSONL mutation stays forbidden.
- No Phase 2 WU is blocked on agent-runner features in r5.
- Do not commit or push from ticket generation work.
