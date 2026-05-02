# Phase 1 Ticket Index

Source: `/home/nes/projects/agent-harness/worktrees/phase-1-ai-roadmap-r4/product-strategy/ai-roadmap-phase-1.md`.

This directory supersedes `worktrees/tickets-phase-1-r2/plans/tickets/phase-1/`. It regenerates all 56 Phase 1 tickets from the Phase 1 r4 Option A cascade artifact and preserves the 7-wave Parallelization Map.

## Work Unit Inventory

Total Phase 1 WUs: **56**.

| Value slice | WUs | Count |
|---|---|---:|
| Shared: Shared imposed-context scaffolding | WU-1-01 | 1 |
| VS-003: Capture Tool-Call Provenance and Audit Events | WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13 | 12 |
| VS-004: Gate Renders with Budget Ledgers and Cache Locality | WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19 | 6 |
| VS-001: Inspect Imposed Working-Set Renders | WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28 | 9 |
| VS-002: Enforce Summary Contracts on Visible Nodes | WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36 | 8 |
| VS-005: Inspect Configuration as Memory Semantics | WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44 | 8 |
| VS-006: Preflight Providers and Expose Capability Fingerprints | WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50, WU-1-56 | 7 |
| VS-007: Show Initiative Roots and Current Focus | WU-1-51, WU-1-52, WU-1-53, WU-1-54, WU-1-55 | 5 |

| WU | Owns | Parent slice |
|---|---|---|
| WU-1-01 | ImposedRenderLabel Enum | Shared |
| WU-1-02 | ToolCallNormalizedEvent DTO | VS-003 |
| WU-1-03 | ClaudeToolCallNormalizer | VS-003 |
| WU-1-04 | CodexToolCallNormalizer | VS-003 |
| WU-1-05 | OpencodeToolCallNormalizer | VS-003 |
| WU-1-06 | ToolCallProvenanceWriter | VS-003 |
| WU-1-07 | EvidenceBlobStore | VS-003 |
| WU-1-08 | SideEffectClassifier | VS-003 |
| WU-1-09 | ApprovalStateCapture | VS-003 |
| WU-1-10 | TranscriptIngestionAdapter | VS-003 |
| WU-1-11 | ParentInvocationPropagationService | VS-003 |
| WU-1-12 | ToolCallAuditEmitter | VS-003 |
| WU-1-13 | ToolCallDrilldownComponent | VS-003 |
| WU-1-14 | BudgetLedgerScopeWriter | VS-004 |
| WU-1-15 | TokenCostEstimator | VS-004 |
| WU-1-16 | CachePrefixHashConsumer | VS-004 |
| WU-1-17 | RenderBudgetGateConsumerAdapter | VS-004 |
| WU-1-18 | BlockedRenderSurface | VS-004 |
| WU-1-19 | CostDisplayPanel | VS-004 |
| WU-1-20 | WorkingSetSnapshotReader | VS-001 |
| WU-1-21 | WorkingSetSnapshotWriter | VS-001 |
| WU-1-22 | RenderEngineConsumerAdapter | VS-001 |
| WU-1-23 | RenderEvidencePointerService | VS-001 |
| WU-1-24 | RenderLabelClassifier | VS-001 |
| WU-1-25 | WorkingSetInspectorPane | VS-001 |
| WU-1-26 | TokenEstimateDisplay | VS-001 |
| WU-1-27 | CachePrefixDisplay | VS-001 |
| WU-1-28 | RenderAuditSubscription | VS-001 |
| WU-1-29 | SummaryContractValidationResult DTO | VS-002 |
| WU-1-30 | SummaryContractValidator | VS-002 |
| WU-1-31 | EvidenceLocatorValidator | VS-002 |
| WU-1-32 | SummaryTemplateRegistry | VS-002 |
| WU-1-33 | RenderLabelDecoratorService | VS-002 |
| WU-1-34 | InvalidRenderLabelDto | VS-002 |
| WU-1-35 | SummaryContractFixturePack | VS-002 |
| WU-1-36 | InvalidSummaryLabelComponent | VS-002 |
| WU-1-37 | ConfigurationInspectorService | VS-005 |
| WU-1-38 | EffectiveValueSourceResolver | VS-005 |
| WU-1-39 | EmptyGraphSimulator | VS-005 |
| WU-1-40 | ShapeExplanationGenerator | VS-005 |
| WU-1-41 | ConfigurationWarningWriter | VS-005 |
| WU-1-42 | ConfigurationOptimizerRequestEmitter | VS-005 |
| WU-1-43 | ConfigurationInspectorPane | VS-005 |
| WU-1-44 | ConfigurationAuditSubscription | VS-005 |
| WU-1-45 | RedactedProviderProbeService | VS-006 |
| WU-1-56 | FakeProviderProbeFixture | VS-006 |
| WU-1-46 | EntitlementObservationWriter | VS-006 |
| WU-1-47 | CapabilityMatrixPopulator | VS-006 |
| WU-1-48 | RouteEligibilityResolver | VS-006 |
| WU-1-49 | RouteDenialReasonClassifier | VS-006 |
| WU-1-50 | ProviderPreflightPane | VS-006 |
| WU-1-51 | InitiativeRootService | VS-007 |
| WU-1-52 | FocusPathService | VS-007 |
| WU-1-53 | AgentWalkStateConsumerAdapter | VS-007 |
| WU-1-54 | NotificationClassifier | VS-007 |
| WU-1-55 | SingleTabShellExtension | VS-007 |

## Parallelization Map

Topological-level partition: every dependency of a WU in wave N has a wave number lower than N. File ownership still needs per-PR coordination, but there are no intra-wave dependency edges.

Round 3 re-derive: the topological waves remain 18+12+9+5+7+3+2 because SessionOverrideContract edges are cross-phase incoming gates, not new Phase 1-local edges.

Round 4 re-derive: removing the retired adapter dependency and former block-on annotations adds no Phase 1-local edge; waves remain 18+12+9+5+7+3+2 = 56.

| Wave | Work units | Count |
|---:|---|---:|
| 1 | WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56 | 18 |
| 2 | WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52 | 12 |
| 3 | WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54 | 9 |
| 4 | WU-1-12, WU-1-33, WU-1-42, WU-1-48, WU-1-55 | 5 |
| 5 | WU-1-13, WU-1-21, WU-1-28, WU-1-34, WU-1-43, WU-1-44, WU-1-49 | 7 |
| 6 | WU-1-22, WU-1-35, WU-1-50 | 3 |
| 7 | WU-1-25, WU-1-36 | 2 |

## Round 4 SessionOverrideContract Incoming Edges

Round 4 SessionOverrideContract incoming edges, matching Phase 0C-r5 outgoing declarations:

- (WU-0C-N2, WU-1-03) canonical transcript DTO shapes for Claude tool-call normalization; no raw JSONL parsing.
- (WU-0C-N2, WU-1-04) canonical transcript DTO shapes for Codex tool-call normalization; no raw JSONL parsing.
- (WU-0C-N2, WU-1-05) canonical transcript DTO shapes for opencode tool-call normalization; no raw session storage parsing.
- (WU-0C-N1, WU-1-10), (WU-0C-N2, WU-1-10), (WU-0C-N3, WU-1-10), (WU-0C-N5, WU-1-10) read-only transcript ingestion through SessionOverrideContract v2, schema probing, and refusal evidence.
- (WU-0C-N5, WU-1-12), (WU-0C-N5, WU-1-13), (WU-0C-N5, WU-1-28) override receipt/refusal audit and drill-down display.
- (WU-0C-N2, WU-1-23), (WU-0C-N5, WU-1-23), (WU-0C-N2, WU-1-25), (WU-0C-N5, WU-1-25) override-derived evidence pointer and inspector display.
- (WU-0C-N3, WU-1-45), (WU-0C-N5, WU-1-45), (WU-0C-N3, WU-1-48), (WU-0C-N5, WU-1-48), (WU-0C-N3, WU-1-49), (WU-0C-N5, WU-1-49), (WU-0C-N3, WU-1-50), (WU-0C-N5, WU-1-50) read-only provider preflight display of schema/refusal evidence.

Bidirectional consistency note: Phase 0C-r5 names WU-0C-N5 as feeding VS-001 evidence inspectors and VS-003 audit surfaces, and names v2-only SessionOverrideContract edges as feeding Phase 1 worker-launcher / worker-output-reintegration successors. Phase 1 r4 wires the existing VS-001/VS-003/VS-006 read-only surfaces. The worker-launcher and worker-output-reintegration edges are intentionally carried forward as outgoing/downstream edges because those WUs are not present in this 56-WU Phase 1 artifact.

No WUs in this 56-WU Phase 1 ticket set are blocked on agent-runner features; the relevant agent-runner session features have landed upstream, and Phase 1 remains read-only at those boundaries.

## Revision Families

- r4-affected WUs: WU-1-03, WU-1-04, WU-1-05, WU-1-10, WU-1-12, WU-1-13, WU-1-23, WU-1-25, WU-1-28, WU-1-45, WU-1-48, WU-1-49, WU-1-50.
- Byte-stable from r2 WUs: WU-1-01, WU-1-02, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19, WU-1-20, WU-1-21, WU-1-22, WU-1-24, WU-1-26, WU-1-27, WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36, WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44, WU-1-46, WU-1-47, WU-1-51, WU-1-52, WU-1-53, WU-1-54, WU-1-55, WU-1-56.

## Generation Notes

- One ticket file exists for each WU-1-01 through WU-1-56.
- Ticket acceptance criteria are copied verbatim from the Phase 1 r4 WU blocks.
- Dependency sections preserve the roadmap dependency lines verbatim and regroup the same WU IDs by Phase 0A, Phase 0B, Phase 0C, and intra-Phase 1 for implementation handoff.
- No commit or push is part of this ticket generation work.
