# Phase 1 AI-Optimized Roadmap

## Pipeline Reference

Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

Test/code separation remains strict: the test agent sees contract files, DTO definitions, endpoint or command signatures, and fixture specifications only; the code agent sees those contracts plus the failing tests. Each WU below is scoped as one single-concern PR.

Rules carried into this phase:

- D1: each distinct schema object, enum, service, state machine, adapter, fixture pack, or method-bearing UI surface is its own WU.
- D2: every method, enum, and state machine declared in a Contract has binary acceptance criteria.
- D3: dependency regression checks describe only checks actually performed for this draft.
- D4: audit-history watch signals are tracked explicitly in the Run Report.

## Phase 1 Scope

Phase 1 delivers Observable Imposed Context, the first operator-visible feature phase after Phases 0A, 0B, and 0C. It consumes WU-0A shell/IPC/test substrate, WU-0B GraphStore and durable schema contracts, and WU-0C runtime engines including RenderEngine, PolicyEngine, BudgetGateService, ConfigurationRegistry runtime, ProviderStateMonitor, AgentRunnerClient facade, SessionOverrideContract, and AuditEmitPipeline. It does not redefine upstream schema tables or service engines.

Round 3 integrates the proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 SessionOverrideContract cascade. Phase 1 keeps transcript and provider work read-only unless a WU explicitly owns a durable Phase 1 row. It does not open, locate, parse, rewrite, truncate, append, or migrate per-CLI session JSONL files directly; raw session reads and any future session transcript mutation flow through WU-0C-N1 and its adapters. Phase 1 also does not reimplement `agent-runner` provider routing, account selection, quota balancing, auth refresh, `--resume` composition, cross-provider session porting, or session-id capture.

Engineering order inside Phase 1: VS-003 and VS-004 create the evidence/audit and budget/cache backbones; VS-001 accepts only after those backbones can populate real render evidence, token, prefix, provider, configuration, and audit fields; VS-002, VS-005, VS-006, and VS-007 lane in parallel where their dependencies are stable.

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

## Round 3 SessionOverrideContract Audit

Audit result over all 56 WUs: no WU in this Phase 1 artifact owns worker launch, worker-output reintegration, orchestrator turn execution, or session transcript mutation. The affected WUs are read-only transcript/provenance/provider surfaces that needed boundary language and cross-phase SessionOverrideContract dependencies.

| WU | r2 risk found | r3 refactor | SessionOverrideContract dependency |
|---|---|---|---|
| WU-1-03 | Claude normalizer could be read as parsing Claude transcript files directly. | Consumes normalized turn/hook/trace evidence only; no JSONL or storage lookup. | WU-0C-N2 only. |
| WU-1-04 | Codex normalizer could be read as parsing rollout/session files directly. | Consumes normalized turn/rollout/trace evidence only; no JSONL or storage lookup. | WU-0C-N2 only. |
| WU-1-05 | Opencode normalizer could be read as parsing provider-native session rows. | Consumes normalized hook/event/trace evidence only; no storage lookup. | WU-0C-N2 only. |
| WU-1-10 | Transcript ingestion could become a direct per-CLI transcript reader. | Reads through WU-0C-15d or WU-0C-N1 and records refusal evidence; no mutation fallback. | WU-0C-N1..WU-0C-N5; WU-0C-N3 supplies v1 adapter. |
| WU-1-12 | Audit emission could imply ownership of override lifecycle events. | Links existing override receipts/refusals only. | WU-0C-N5. |
| WU-1-13 | Drill-down could expose raw session storage. | Displays override evidence pointers only; no raw transcript bodies or mutation controls. | WU-0C-N5. |
| WU-1-23 | Render evidence pointers could become a second transcript locator. | Adds override-derived evidence as opaque pointers. | WU-0C-N2, WU-0C-N5. |
| WU-1-25 | Inspector pane could become a hidden session editor. | Displays override metadata only; no import/replace/reroute/resume controls. | WU-0C-N2, WU-0C-N5. |
| WU-1-28 | Audit subscription could expose override lifecycle controls. | Streams override metadata and acknowledges UI state only. | WU-0C-N5. |
| WU-1-45 | Provider probe could drift into routing/quota/session-storage ownership. | Redacted observation only; no account, quota, auth, resume, or storage decisions. | WU-0C-N4, WU-0C-N5 for read-only capability/refusal evidence. |
| WU-1-48 | Route eligibility could be mistaken for provider routing. | Returns harness policy/readiness eligibility only. | WU-0C-N4, WU-0C-N5. |
| WU-1-49 | Denial classification could recompute upstream adapter/provider refusals. | Labels observed denial/refusal categories only. | WU-0C-N4, WU-0C-N5. |
| WU-1-50 | Provider panel could expose routing or override controls. | Displays read-only preflight and override capability/refusal metadata. | WU-0C-N4, WU-0C-N5. |

Unaffected WUs: WU-1-01/02/06/07/08/09/11/14/15/16/17/18/19/20/21/22/24/26/27/29/30/31/32/33/34/35/36/37/38/39/40/41/42/43/44/46/47/51/52/53/54/55/56 do not encode per-CLI session storage, provider routing, quota, resume, cross-provider porting, or session mutation behavior. No WU scope collapsed enough to merge into a sibling.

## Phase 1 Work Units
### WU-1-01: ImposedRenderLabel Enum

**Parent initiative:** Shared: Shared imposed-context scaffolding

**Contract:**
```text
ImposedRenderLabel = valid | invalid_missing_evidence | invalid_conflict | invalid_stale | invalid_policy | needs_review | overfull_required_context | blocked_budget | blocked_provider | blocked_configuration.
```

**Test boundary:** product-strategy/contracts/wu-1-01-imposedrenderlabel-enum.md; src-tauri/src/contracts/wu_1_01.rs; src/contracts/wu_1_01.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_01.rs; src-tauri/src/commands/wu_1_01.rs when IPC is declared; src/features/wu_1_01/**/* for UI WUs; src-tauri/tests/wu_1_01_contract.rs; src/test/wu_1_01.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared object round-trips through Rust serde and TypeScript fixtures with every field or variant preserved.
- [ ] An unknown enum value, missing required field, or malformed opaque ID is rejected with a documented error variant.
- [ ] The contract fixture includes one positive case and one negative case for every declared enum or validation state.
- [ ] `enum round-trip` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `invalid label rejection` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-01, WU-0A-02, WU-0A-03, WU-0A-04, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-15, WU-0B-04, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** Shared render-label taxonomy for VS-001/VS-002/VS-004/VS-005/VS-006/VS-007.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-02: ToolCallNormalizedEvent DTO

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
ToolCallNormalizedEvent { tool_event_id?, evidence_id?, session_id, turn_id?, protocol, tool_call_id, tool_name, tool_input_hash, tool_result_id?, approval_state, side_effect_class, retry_semantics, state, parent_invocation_id?, correlation_key }.
```

**Test boundary:** product-strategy/contracts/wu-1-02-toolcallnormalizedevent-dto.md; src-tauri/src/contracts/wu_1_02.rs; src/contracts/wu_1_02.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_02.rs; src-tauri/src/commands/wu_1_02.rs when IPC is declared; src/features/wu_1_02/**/* for UI WUs; src-tauri/tests/wu_1_02_contract.rs; src/test/wu_1_02.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared object round-trips through Rust serde and TypeScript fixtures with every field or variant preserved.
- [ ] An unknown enum value, missing required field, or malformed opaque ID is rejected with a documented error variant.
- [ ] The contract fixture includes one positive case and one negative case for every declared enum or validation state.
- [ ] `serde/sqlx round-trip` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `protocol enum validation` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `state enum validation` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** Canonical normalized event consumed by all Phase 1 provenance writers.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-03: ClaudeToolCallNormalizer

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
normalize_claude_tool_call(normalized_turn_ref, session_id, invocation_ref) -> ToolCallNormalizedEvent.
```

**Test boundary:** product-strategy/contracts/wu-1-03-claudetoolcallnormalizer.md; src-tauri/src/contracts/wu_1_03.rs; src/contracts/wu_1_03.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_03.rs; src-tauri/src/commands/wu_1_03.rs when IPC is declared; src/features/wu_1_03/**/* for UI WUs; src-tauri/tests/wu_1_03_contract.rs; src/test/wu_1_03.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `normalize_claude_tool_call` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The normalizer consumes normalized `SessionTurnRef`, `TranscriptTurn`, hook, or trace evidence produced by WU-0C-15a..WU-0C-15d / WU-0C-N2 and never opens Claude JSONL paths, `state.db`, locator scripts, or provider-native session files.
- [ ] The normalizer never calls `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, or `append_turns`; read orchestration belongs to WU-1-10 and mutation belongs to WU-0C-N1 implementers.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-02.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N2.

**Produces:** Claude-shaped tool evidence normalized to WU-1-02.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

**Revision rationale:** Round 3 narrows this WU from any implied Claude transcript parser into a provider-shape normalizer over already captured, normalized evidence. Claude-owned storage layout, JSONL record shape, session lookup, and resume behavior stay behind `agent-runner` / SessionOverrideContract boundaries.

### WU-1-04: CodexToolCallNormalizer

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
normalize_codex_tool_call(normalized_turn_ref, session_id, invocation_ref) -> ToolCallNormalizedEvent.
```

**Test boundary:** product-strategy/contracts/wu-1-04-codextoolcallnormalizer.md; src-tauri/src/contracts/wu_1_04.rs; src/contracts/wu_1_04.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_04.rs; src-tauri/src/commands/wu_1_04.rs when IPC is declared; src/features/wu_1_04/**/* for UI WUs; src-tauri/tests/wu_1_04_contract.rs; src/test/wu_1_04.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `normalize_codex_tool_call` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The normalizer consumes normalized `SessionTurnRef`, `TranscriptTurn`, rollout, or trace evidence produced by WU-0C-15a..WU-0C-15d / WU-0C-N2 and never opens Codex JSONL paths, `state.db`, locator scripts, or provider-native session files.
- [ ] The normalizer never calls `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, or `append_turns`; read orchestration belongs to WU-1-10 and mutation belongs to WU-0C-N1 implementers.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-02.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N2.

**Produces:** Codex-shaped tool evidence normalized to WU-1-02.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

**Revision rationale:** Round 3 removes any direct Codex rollout/session-file interpretation from this WU. It only maps normalized evidence into the Phase 1 tool-call DTO; Codex session lookup, storage compatibility, resume acceptance, and cross-account portability remain upstream.

### WU-1-05: OpencodeToolCallNormalizer

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
normalize_opencode_tool_call(normalized_event_ref, session_id, invocation_ref) -> ToolCallNormalizedEvent.
```

**Test boundary:** product-strategy/contracts/wu-1-05-opencodetoolcallnormalizer.md; src-tauri/src/contracts/wu_1_05.rs; src/contracts/wu_1_05.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_05.rs; src-tauri/src/commands/wu_1_05.rs when IPC is declared; src/features/wu_1_05/**/* for UI WUs; src-tauri/tests/wu_1_05_contract.rs; src/test/wu_1_05.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `normalize_opencode_tool_call` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The normalizer consumes normalized hook/event/trace evidence and never opens opencode session rows, JSONL paths, `state.db`, locator scripts, or provider-native session files.
- [ ] The normalizer never calls `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, or `append_turns`; read orchestration belongs to WU-1-10 and mutation belongs to WU-0C-N1 implementers.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-02.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N2.

**Produces:** opencode-shaped tool evidence normalized to WU-1-02.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

**Revision rationale:** Round 3 keeps opencode support as evidence normalization only. Any provider-native storage interpretation or unsupported-session explanation is supplied by Phase 0C session/trace contracts, not by this normalizer.

### WU-1-06: ToolCallProvenanceWriter

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
write_tool_call_provenance(event: ToolCallNormalizedEvent) -> ToolCallProvenanceId; reconcile_tool_call(tool_event_id, result_ref) -> ToolCallProvenanceId.
```

**Test boundary:** product-strategy/contracts/wu-1-06-toolcallprovenancewriter.md; src-tauri/src/contracts/wu_1_06.rs; src/contracts/wu_1_06.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_06.rs; src-tauri/src/commands/wu_1_06.rs when IPC is declared; src/features/wu_1_06/**/* for UI WUs; src-tauri/tests/wu_1_06_contract.rs; src/test/wu_1_06.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `write_tool_call_provenance` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reconcile_tool_call` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-02, WU-1-03, WU-1-04, WU-1-05.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** Durable ToolCallProvenance rows linked to evidence and turns.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54.

### WU-1-07: EvidenceBlobStore

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
put_evidence_blob(source_type, source_uri, bytes, privilege_origin, correlation_key) -> EvidenceArtifactId; get_evidence_blob(evidence_id) -> EvidenceBlobRead.
```

**Test boundary:** product-strategy/contracts/wu-1-07-evidenceblobstore.md; src-tauri/src/contracts/wu_1_07.rs; src/contracts/wu_1_07.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_07.rs; src-tauri/src/commands/wu_1_07.rs when IPC is declared; src/features/wu_1_07/**/* for UI WUs; src-tauri/tests/wu_1_07_contract.rs; src/test/wu_1_07.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `put_evidence_blob` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `get_evidence_blob` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** Captured evidence blobs and EvidenceArtifact references for drill-down and summary validation.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-08: SideEffectClassifier

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
classify_side_effect(protocol, tool_name, normalized_input_hash, declared_capabilities) -> SideEffectClass.
```

**Test boundary:** product-strategy/contracts/wu-1-08-sideeffectclassifier.md; src-tauri/src/contracts/wu_1_08.rs; src/contracts/wu_1_08.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_08.rs; src-tauri/src/commands/wu_1_08.rs when IPC is declared; src/features/wu_1_08/**/* for UI WUs; src-tauri/tests/wu_1_08_contract.rs; src/test/wu_1_08.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `classify_side_effect` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `side_effect enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** read_only/local_write/external_write/process/network classification for tool records.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-09: ApprovalStateCapture

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
capture_approval_state(protocol, raw_event_ref, tool_call_id) -> ApprovalState.
```

**Test boundary:** product-strategy/contracts/wu-1-09-approvalstatecapture.md; src-tauri/src/contracts/wu_1_09.rs; src/contracts/wu_1_09.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_09.rs; src-tauri/src/commands/wu_1_09.rs when IPC is declared; src/features/wu_1_09/**/* for UI WUs; src-tauri/tests/wu_1_09_contract.rs; src/test/wu_1_09.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `capture_approval_state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `approval enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** Approval-state observation used by provenance and recovery.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-10: TranscriptIngestionAdapter

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
ingest_transcript(session_id, session_read_ref?) -> Vec<ToolCallNormalizedEvent>; mark_partial_ingest(session_id, reason) -> EvidenceArtifactId.
```

**Test boundary:** product-strategy/contracts/wu-1-10-transcriptingestionadapter.md; src-tauri/src/contracts/wu_1_10.rs; src/contracts/wu_1_10.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_10.rs; src-tauri/src/commands/wu_1_10.rs when IPC is declared; src/features/wu_1_10/**/* for UI WUs; src-tauri/tests/wu_1_10_contract.rs; src/test/wu_1_10.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `ingest_transcript` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `mark_partial_ingest` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `ingest_transcript` reads session content only through WU-0C-15d normalized turn evidence or WU-0C-N1 `read_transcript` / `get_session_metadata`; it accepts `session_id` as the lookup key and does not accept filesystem transcript paths as product inputs.
- [ ] The adapter records unsupported schema, unsupported storage, busy session, missing session, and adapter refusal as evidence artifacts without attempting fallback per-CLI JSONL parsing.
- [ ] The adapter never calls WU-0C-N1 mutation methods (`replace_transcript`, `truncate_after`, `append_turns`) and never edits `agents` state rows or provider session files.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-03, WU-1-04, WU-1-05, WU-1-07.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37, WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5.

**Produces:** Session-derived evidence and normalized events.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54.

**Blocked-on:** v1 read behavior can use WU-0C-N3 after schema probe. v2 adapter migration is blocked on `agents session locate` and `agents session export`; this WU is not blocked on `agents session import-replace` or `agents pause-handshake` because Phase 1 transcript ingestion is read-only.

**Revision rationale:** Round 3 makes this the central Phase 1 reader over normalized session evidence. It no longer owns transcript path discovery, per-CLI JSONL parsing fallback, session storage mutation, resume composition, or provider/session porting.

### WU-1-11: ParentInvocationPropagationService

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
propagate_parent_invocation(child_invocation_ref, parent_invocation_ref) -> InvocationLineageRef.
```

**Test boundary:** product-strategy/contracts/wu-1-11-parentinvocationpropagationservice.md; src-tauri/src/contracts/wu_1_11.rs; src/contracts/wu_1_11.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_11.rs; src-tauri/src/commands/wu_1_11.rs when IPC is declared; src/features/wu_1_11/**/* for UI WUs; src-tauri/tests/wu_1_11_contract.rs; src/test/wu_1_11.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `propagate_parent_invocation` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.

**Produces:** OULIPOLY parent/child invocation linkage on provenance rows.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-12: ToolCallAuditEmitter

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
emit_tool_call_audit(tool_event_id, decision, reason_code) -> AuditEventId.
```

**Test boundary:** product-strategy/contracts/wu-1-12-toolcallauditemitter.md; src-tauri/src/contracts/wu_1_12.rs; src/contracts/wu_1_12.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_12.rs; src-tauri/src/commands/wu_1_12.rs when IPC is declared; src/features/wu_1_12/**/* for UI WUs; src-tauri/tests/wu_1_12_contract.rs; src/test/wu_1_12.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `emit_tool_call_audit` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] Tool-call audit events can link to WU-0C-N5 override registry records by opaque override ID when an upstream session read/refusal is the evidence source.
- [ ] The emitter never begins, commits, rolls back, quarantines, replaces, truncates, or appends session overrides; it only links existing evidence/audit refs.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-06, WU-1-08, WU-1-09, WU-1-11.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N5.

**Produces:** AuditEvent entries for tool-call capture, side effects, approvals, and reconciliation.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-33, WU-1-42, WU-1-48, WU-1-55.

**Revision rationale:** Round 3 allows audit surfaces to reference SessionOverrideStore refusal/receipt metadata without moving override lifecycle ownership into Phase 1 audit emission.

### WU-1-13: ToolCallDrilldownComponent

**Parent initiative:** VS-003: Capture Tool-Call Provenance and Audit Events

**Contract:**
```text
ToolCallDrilldownView { tool_event_id, evidence_id, protocol, approval_state, side_effect_class, retry_semantics, state, audit_event_ids }.
```

**Test boundary:** product-strategy/contracts/wu-1-13-toolcalldrilldowncomponent.md; src-tauri/src/contracts/wu_1_13.rs; src/contracts/wu_1_13.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_13.rs; src-tauri/src/commands/wu_1_13.rs when IPC is declared; src/features/wu_1_13/**/* for UI WUs; src-tauri/tests/wu_1_13_contract.rs; src/test/wu_1_13.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The view can show an override receipt/refusal pointer from WU-0C-N5 as linked evidence without rendering raw JSONL, filesystem paths, adapter temp paths, or provider-native transcript bodies.
- [ ] The component exposes no control that invokes `replace_transcript`, `truncate_after`, `append_turns`, provider reroute, resume, or session import/export.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-06, WU-1-07, WU-1-12.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-20, WU-0B-22, WU-0B-24, WU-0B-04, WU-0B-31, WU-0C-04, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N5.

**Produces:** Operator-visible provenance drill-down pane fragment.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-21, WU-1-28, WU-1-34, WU-1-43, WU-1-44, WU-1-49.

**Revision rationale:** Round 3 limits drill-down to evidence display. It may surface SessionOverrideContract receipts/refusals but does not inspect or mutate provider-owned session storage.

### WU-1-14: BudgetLedgerScopeWriter

**Parent initiative:** VS-004: Gate Renders with Budget Ledgers and Cache Locality

**Contract:**
```text
write_render_budget_scope(scope_type, scope_id, provider_state_id?, cache_prefix_hash?, budget_state, policy_action) -> BudgetLedgerId.
```

**Test boundary:** product-strategy/contracts/wu-1-14-budgetledgerscopewriter.md; src-tauri/src/contracts/wu_1_14.rs; src/contracts/wu_1_14.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_14.rs; src-tauri/src/commands/wu_1_14.rs when IPC is declared; src/features/wu_1_14/**/* for UI WUs; src-tauri/tests/wu_1_14_contract.rs; src/test/wu_1_14.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `write_render_budget_scope` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `budget_state enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `policy_action enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-04, WU-0B-15, WU-0B-21, WU-0B-24, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Render-scoped BudgetLedger rows for cost and policy decisions.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-15: TokenCostEstimator

**Parent initiative:** VS-004: Gate Renders with Budget Ledgers and Cache Locality

**Contract:**
```text
estimate_render_cost(render_result_id, provider_state_id?, model_id?) -> TokenCostEstimate.
```

**Test boundary:** product-strategy/contracts/wu-1-15-tokencostestimator.md; src-tauri/src/contracts/wu_1_15.rs; src/contracts/wu_1_15.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_15.rs; src-tauri/src/commands/wu_1_15.rs when IPC is declared; src/features/wu_1_15/**/* for UI WUs; src-tauri/tests/wu_1_15_contract.rs; src/test/wu_1_15.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `estimate_render_cost` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-04, WU-0B-15, WU-0B-21, WU-0B-24, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Input/output/cache token and cost estimate DTOs for renders.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-16: CachePrefixHashConsumer

**Parent initiative:** VS-004: Gate Renders with Budget Ledgers and Cache Locality

**Contract:**
```text
read_cache_prefix_hash(working_set_id) -> CachePrefixHash; compare_cache_prefix_hash(a,b) -> CacheLocalityResult.
```

**Test boundary:** product-strategy/contracts/wu-1-16-cacheprefixhashconsumer.md; src-tauri/src/contracts/wu_1_16.rs; src/contracts/wu_1_16.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_16.rs; src-tauri/src/commands/wu_1_16.rs when IPC is declared; src/features/wu_1_16/**/* for UI WUs; src-tauri/tests/wu_1_16_contract.rs; src/test/wu_1_16.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `read_cache_prefix_hash` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `compare_cache_prefix_hash` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-04, WU-0B-15, WU-0B-21, WU-0B-24, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Cache-prefix hash read and comparison service over Phase 0C hasher output.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-17: RenderBudgetGateConsumerAdapter

**Parent initiative:** VS-004: Gate Renders with Budget Ledgers and Cache Locality

**Contract:**
```text
gate_visible_render(working_set_id, token_estimate, budget_ledger_id) -> BudgetDecision.
```

**Test boundary:** product-strategy/contracts/wu-1-17-renderbudgetgateconsumeradapter.md; src-tauri/src/contracts/wu_1_17.rs; src/contracts/wu_1_17.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_17.rs; src-tauri/src/commands/wu_1_17.rs when IPC is declared; src/features/wu_1_17/**/* for UI WUs; src-tauri/tests/wu_1_17_contract.rs; src/test/wu_1_17.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `gate_visible_render` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-14, WU-1-15, WU-1-16.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-04, WU-0B-15, WU-0B-21, WU-0B-24, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Phase 1 adapter from visible render requests to BudgetGateService decisions.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

### WU-1-18: BlockedRenderSurface

**Parent initiative:** VS-004: Gate Renders with Budget Ledgers and Cache Locality

**Contract:**
```text
BlockedRenderSurface { working_set_id, budget_state, policy_action, reason_code, required_user_action? }.
```

**Test boundary:** product-strategy/contracts/wu-1-18-blockedrendersurface.md; src-tauri/src/contracts/wu_1_18.rs; src/contracts/wu_1_18.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_18.rs; src-tauri/src/commands/wu_1_18.rs when IPC is declared; src/features/wu_1_18/**/* for UI WUs; src-tauri/tests/wu_1_18_contract.rs; src/test/wu_1_18.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render budget states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-17.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-04, WU-0B-15, WU-0B-21, WU-0B-24, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Operator-visible blocked/warn/narrow-scope render state.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54.

### WU-1-19: CostDisplayPanel

**Parent initiative:** VS-004: Gate Renders with Budget Ledgers and Cache Locality

**Contract:**
```text
CostDisplayPanel { working_set_id, input_tokens, output_tokens, cache_read_tokens, cache_write_tokens, provider_cost_estimate, latency_ms }.
```

**Test boundary:** product-strategy/contracts/wu-1-19-costdisplaypanel.md; src-tauri/src/contracts/wu_1_19.rs; src/contracts/wu_1_19.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_19.rs; src-tauri/src/commands/wu_1_19.rs when IPC is declared; src/features/wu_1_19/**/* for UI WUs; src-tauri/tests/wu_1_19_contract.rs; src/test/wu_1_19.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render cost values` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `missing field rejection` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-14, WU-1-15, WU-1-16.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-04, WU-0B-15, WU-0B-21, WU-0B-24, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Visible token, cache, cost, and latency panel.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

### WU-1-20: WorkingSetSnapshotReader

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
get_working_set_snapshot(working_set_id) -> WorkingSetSnapshotView; list_recent_working_sets(workspace_id, limit) -> Vec<WorkingSetSnapshotSummary>.
```

**Test boundary:** product-strategy/contracts/wu-1-20-workingsetsnapshotreader.md; src-tauri/src/contracts/wu_1_20.rs; src/contracts/wu_1_20.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_20.rs; src-tauri/src/commands/wu_1_20.rs when IPC is declared; src/features/wu_1_20/**/* for UI WUs; src-tauri/tests/wu_1_20_contract.rs; src/test/wu_1_20.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `get_working_set_snapshot` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `list_recent_working_sets` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Read model over Phase 0B WorkingSetSnapshot plus render blob refs.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-21: WorkingSetSnapshotWriter

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
attach_render_inspection_metadata(working_set_id, evidence_pointer_ids, token_estimate, prefix_hash, audit_event_id) -> WorkingSetSnapshotId.
```

**Test boundary:** product-strategy/contracts/wu-1-21-workingsetsnapshotwriter.md; src-tauri/src/contracts/wu_1_21.rs; src/contracts/wu_1_21.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_21.rs; src-tauri/src/commands/wu_1_21.rs when IPC is declared; src/features/wu_1_21/**/* for UI WUs; src-tauri/tests/wu_1_21_contract.rs; src/test/wu_1_21.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `attach_render_inspection_metadata` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-20, WU-1-12, WU-1-17.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Phase 1 inspection metadata attached without redefining WorkingSetSnapshot.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-13, WU-1-28, WU-1-34, WU-1-43, WU-1-44, WU-1-49.

### WU-1-22: RenderEngineConsumerAdapter

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
load_visible_render(render_request_id) -> RenderResultDto; resolve_render_blob(rendered_blob_ref) -> RenderBlobRead.
```

**Test boundary:** product-strategy/contracts/wu-1-22-renderengineconsumeradapter.md; src-tauri/src/contracts/wu_1_22.rs; src/contracts/wu_1_22.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_22.rs; src-tauri/src/commands/wu_1_22.rs when IPC is declared; src/features/wu_1_22/**/* for UI WUs; src-tauri/tests/wu_1_22_contract.rs; src/test/wu_1_22.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `load_visible_render` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `resolve_render_blob` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-20, WU-1-21.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Adapter from Phase 0C RenderEngine output into the inspector.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-35, WU-1-50.

### WU-1-23: RenderEvidencePointerService

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
list_render_evidence(working_set_id) -> Vec<ProvenancePointerView>; open_render_evidence(provenance_id) -> EvidenceArtifactView.
```

**Test boundary:** product-strategy/contracts/wu-1-23-renderevidencepointerservice.md; src-tauri/src/contracts/wu_1_23.rs; src/contracts/wu_1_23.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_23.rs; src-tauri/src/commands/wu_1_23.rs when IPC is declared; src/features/wu_1_23/**/* for UI WUs; src-tauri/tests/wu_1_23_contract.rs; src/test/wu_1_23.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `list_render_evidence` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `open_render_evidence` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] Render evidence pointers can include WU-0C-N5 override receipt/refusal IDs and WU-0C-N2 session metadata refs without exposing raw per-CLI transcript paths or JSONL bodies.
- [ ] Opening override-derived evidence returns registered evidence and audit metadata only; it never locates sessions or calls SessionOverrideContract mutation methods.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-07, WU-1-20.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N2, WU-0C-N5.

**Produces:** Evidence pointer read path for imposed render rows.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

**Revision rationale:** Round 3 adds override-derived evidence as an opaque pointer type, not as a new transcript reader. The storage lookup and adapter compatibility decisions remain in WU-0C-N1..WU-0C-N5.

### WU-1-24: RenderLabelClassifier

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
classify_render_label(working_set_id, budget_decision?, provider_state?, configuration_report?) -> ImposedRenderLabel.
```

**Test boundary:** product-strategy/contracts/wu-1-24-renderlabelclassifier.md; src-tauri/src/contracts/wu_1_24.rs; src/contracts/wu_1_24.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_24.rs; src-tauri/src/commands/wu_1_24.rs when IPC is declared; src/features/wu_1_24/**/* for UI WUs; src-tauri/tests/wu_1_24_contract.rs; src/test/wu_1_24.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `classify_render_label` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `label enum coverage` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-01, WU-1-17, WU-1-20, WU-1-23.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** valid/invalid/blocked labels for visible renders.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-18, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54.

### WU-1-25: WorkingSetInspectorPane

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
WorkingSetInspectorPane { working_set_id, graph_snapshot_id, target_cli, target_model, label, evidence_rows, token_estimate, prefix_hash, provider_state_id, configuration_explanation_ref, audit_event_ids }.
```

**Test boundary:** product-strategy/contracts/wu-1-25-workingsetinspectorpane.md; src-tauri/src/contracts/wu_1_25.rs; src/contracts/wu_1_25.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_25.rs; src-tauri/src/commands/wu_1_25.rs when IPC is declared; src/features/wu_1_25/**/* for UI WUs; src-tauri/tests/wu_1_25_contract.rs; src/test/wu_1_25.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render pane states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown label` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The pane can display override receipt/refusal status from WU-0C-N5 where imposed context evidence came from a session read, while hiding raw per-CLI session file paths and provider-native transcript bodies.
- [ ] The pane exposes no control for session import, transcript replacement, provider reroute, resume, or cross-provider porting.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-22, WU-1-23, WU-1-24.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N2, WU-0C-N5.

**Produces:** Main VS-001 operator pane answering what the model saw.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-36.

**Revision rationale:** Round 3 keeps the inspector as a display surface. It consumes override metadata only through registered evidence/audit pointers and cannot become a hidden session editor.

### WU-1-26: TokenEstimateDisplay

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
TokenEstimateDisplay { working_set_id, token_estimate, reasoning_budget_class, over_budget_flag }.
```

**Test boundary:** product-strategy/contracts/wu-1-26-tokenestimatedisplay.md; src-tauri/src/contracts/wu_1_26.rs; src/contracts/wu_1_26.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_26.rs; src-tauri/src/commands/wu_1_26.rs when IPC is declared; src/features/wu_1_26/**/* for UI WUs; src-tauri/tests/wu_1_26_contract.rs; src/test/wu_1_26.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render budget classes` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `missing token rejection` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-15, WU-1-20.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Token estimate badge/readout inside the inspector.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

### WU-1-27: CachePrefixDisplay

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
CachePrefixDisplay { working_set_id, prefix_hash, locality_status, compared_to_previous? }.
```

**Test boundary:** product-strategy/contracts/wu-1-27-cacheprefixdisplay.md; src-tauri/src/contracts/wu_1_27.rs; src/contracts/wu_1_27.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_27.rs; src-tauri/src/commands/wu_1_27.rs when IPC is declared; src/features/wu_1_27/**/* for UI WUs; src-tauri/tests/wu_1_27_contract.rs; src/test/wu_1_27.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render locality states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `missing prefix rejection` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-16, WU-1-20.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Cache locality readout inside the inspector.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-29, WU-1-40, WU-1-46, WU-1-52.

### WU-1-28: RenderAuditSubscription

**Parent initiative:** VS-001: Inspect Imposed Working-Set Renders

**Contract:**
```text
subscribe_render_audit(workspace_id, working_set_id?) -> Stream<AuditEventView>; acknowledge_render_audit_event(audit_event_id) -> Ack.
```

**Test boundary:** product-strategy/contracts/wu-1-28-renderauditsubscription.md; src-tauri/src/contracts/wu_1_28.rs; src/contracts/wu_1_28.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_28.rs; src-tauri/src/commands/wu_1_28.rs when IPC is declared; src/features/wu_1_28/**/* for UI WUs; src-tauri/tests/wu_1_28_contract.rs; src/test/wu_1_28.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `subscribe_render_audit` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `acknowledge_render_audit_event` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The stream can include SessionOverrideStore audit events from WU-0C-N5 by opaque override ID, operation, state, refusal reason, and evidence ref.
- [ ] Acknowledgement updates only the Phase 1 subscription/UI acknowledgement state and never commits, rolls back, quarantines, imports, replaces, truncates, or appends a session override.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-12, WU-1-20.
- Cross-phase incoming: WU-0A-01, WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-21, WU-0B-24, WU-0B-05, WU-0B-32, WU-0B-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-05, WU-0C-06a, WU-0C-06, WU-0C-07a, WU-0C-07, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N5.

**Produces:** Audit event stream filtered to imposed-context renders.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-13, WU-1-21, WU-1-34, WU-1-43, WU-1-44, WU-1-49.

**Revision rationale:** Round 3 permits render audit streams to surface SessionOverrideContract metadata while preserving Phase 1 as read-only with respect to transcript override lifecycle.

### WU-1-29: SummaryContractValidationResult DTO

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
SummaryContractValidationResult { summary_contract_id, validation_state, missing_field_names, invalid_evidence_locator_ids, stale_marker_ids, policy_reason_codes, reviewed_at }.
```

**Test boundary:** product-strategy/contracts/wu-1-29-summarycontractvalidationresult-dto.md; src-tauri/src/contracts/wu_1_29.rs; src/contracts/wu_1_29.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_29.rs; src-tauri/src/commands/wu_1_29.rs when IPC is declared; src/features/wu_1_29/**/* for UI WUs; src-tauri/tests/wu_1_29_contract.rs; src/test/wu_1_29.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared object round-trips through Rust serde and TypeScript fixtures with every field or variant preserved.
- [ ] An unknown enum value, missing required field, or malformed opaque ID is rejected with a documented error variant.
- [ ] The contract fixture includes one positive case and one negative case for every declared enum or validation state.
- [ ] `serde/sqlx round-trip` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `validation_state enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-01.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Single result object for summary contract validation.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-40, WU-1-46, WU-1-52.

### WU-1-30: SummaryContractValidator

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
validate_summary_contract(summary_contract_id, graph_snapshot_id) -> SummaryContractValidationResult.
```

**Test boundary:** product-strategy/contracts/wu-1-30-summarycontractvalidator.md; src-tauri/src/contracts/wu_1_30.rs; src/contracts/wu_1_30.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_30.rs; src-tauri/src/commands/wu_1_30.rs when IPC is declared; src/features/wu_1_30/**/* for UI WUs; src-tauri/tests/wu_1_30_contract.rs; src/test/wu_1_30.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `validate_summary_contract` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-29.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Machine-checkable validation for visible node summaries.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-31, WU-1-41, WU-1-47, WU-1-54.

### WU-1-31: EvidenceLocatorValidator

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
validate_evidence_locator(provenance_id, evidence_locator) -> EvidenceLocatorValidation.
```

**Test boundary:** product-strategy/contracts/wu-1-31-evidencelocatorvalidator.md; src-tauri/src/contracts/wu_1_31.rs; src/contracts/wu_1_31.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_31.rs; src-tauri/src/commands/wu_1_31.rs when IPC is declared; src/features/wu_1_31/**/* for UI WUs; src-tauri/tests/wu_1_31_contract.rs; src/test/wu_1_31.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `validate_evidence_locator` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-07, WU-1-29.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Evidence locator verification for summary claims.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-41, WU-1-47, WU-1-54.

### WU-1-32: SummaryTemplateRegistry

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
resolve_summary_template(node_kind, template_source, configuration_id) -> SummaryTemplateRef.
```

**Test boundary:** product-strategy/contracts/wu-1-32-summarytemplateregistry.md; src-tauri/src/contracts/wu_1_32.rs; src/contracts/wu_1_32.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_32.rs; src-tauri/src/commands/wu_1_32.rs when IPC is declared; src/features/wu_1_32/**/* for UI WUs; src-tauri/tests/wu_1_32_contract.rs; src/test/wu_1_32.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `resolve_summary_template` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `template_source enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Template source lookup without owning GraphConfiguration.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-33: RenderLabelDecoratorService

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
decorate_render_label(summary_contract_id, base_label) -> ImposedRenderLabel.
```

**Test boundary:** product-strategy/contracts/wu-1-33-renderlabeldecoratorservice.md; src-tauri/src/contracts/wu_1_33.rs; src/contracts/wu_1_33.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_33.rs; src-tauri/src/commands/wu_1_33.rs when IPC is declared; src/features/wu_1_33/**/* for UI WUs; src-tauri/tests/wu_1_33_contract.rs; src/test/wu_1_33.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `decorate_render_label` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-01, WU-1-29, WU-1-30, WU-1-31.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Render-label enrichment from summary contract validation.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-12, WU-1-42, WU-1-48, WU-1-55.

### WU-1-34: InvalidRenderLabelDto

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
InvalidRenderLabelDto { node_id, summary_contract_id, label, reason_code, evidence_pointer_ids, stale_marker_ids, can_unpack }.
```

**Test boundary:** product-strategy/contracts/wu-1-34-invalidrenderlabeldto.md; src-tauri/src/contracts/wu_1_34.rs; src/contracts/wu_1_34.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_34.rs; src-tauri/src/commands/wu_1_34.rs when IPC is declared; src/features/wu_1_34/**/* for UI WUs; src-tauri/tests/wu_1_34_contract.rs; src/test/wu_1_34.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared object round-trips through Rust serde and TypeScript fixtures with every field or variant preserved.
- [ ] An unknown enum value, missing required field, or malformed opaque ID is rejected with a documented error variant.
- [ ] The contract fixture includes one positive case and one negative case for every declared enum or validation state.
- [ ] `serde round-trip` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `label enum validation` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-01, WU-1-29, WU-1-33.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** UI payload for invalid/stale summary labels.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-13, WU-1-21, WU-1-28, WU-1-43, WU-1-44, WU-1-49.

### WU-1-35: SummaryContractFixturePack

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
SummaryContractFixturePack provides valid, invalid_missing_evidence, invalid_conflict, invalid_stale, invalid_policy, and needs_review fixtures.
```

**Test boundary:** product-strategy/contracts/wu-1-35-summarycontractfixturepack.md; src-tauri/src/contracts/wu_1_35.rs; src/contracts/wu_1_35.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_35.rs; src-tauri/src/commands/wu_1_35.rs when IPC is declared; src/features/wu_1_35/**/* for UI WUs; src-tauri/tests/wu_1_35_contract.rs; src/test/wu_1_35.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The fixture pack contains one valid and one invalid fixture for every declared state or enum path.
- [ ] Fixtures resolve only through declared contract IDs and do not depend on implementation-only files.
- [ ] Invalid fixtures fail with documented contract errors, not panics or silent coercion.
- [ ] `fixture state coverage` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `invalid fixture rejection` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-34.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Contract-only fixtures for VS-002 tests.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-22, WU-1-50.

### WU-1-36: InvalidSummaryLabelComponent

**Parent initiative:** VS-002: Enforce Summary Contracts on Visible Nodes

**Contract:**
```text
InvalidSummaryLabelComponent { label, reason_code, can_unpack, evidence_pointer_count, stale_marker_count }.
```

**Test boundary:** product-strategy/contracts/wu-1-36-invalidsummarylabelcomponent.md; src-tauri/src/contracts/wu_1_36.rs; src/contracts/wu_1_36.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_36.rs; src-tauri/src/commands/wu_1_36.rs when IPC is declared; src/features/wu_1_36/**/* for UI WUs; src-tauri/tests/wu_1_36_contract.rs; src/test/wu_1_36.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render label states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown label` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-34, WU-1-35.
- Cross-phase incoming: WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-09, WU-0B-10, WU-0B-11, WU-0B-05, WU-0B-32, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Operator-visible invalid/stale label on visible nodes.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-25.

### WU-1-37: ConfigurationInspectorService

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
inspect_configuration(workspace_id, configuration_id, graph_snapshot_id?) -> ConfigurationInspectionReport.
```

**Test boundary:** product-strategy/contracts/wu-1-37-configurationinspectorservice.md; src-tauri/src/contracts/wu_1_37.rs; src/contracts/wu_1_37.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_37.rs; src-tauri/src/commands/wu_1_37.rs when IPC is declared; src/features/wu_1_37/**/* for UI WUs; src-tauri/tests/wu_1_37_contract.rs; src/test/wu_1_37.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `inspect_configuration` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `inspection_state enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Top-level inspection report over ConfigurationRegistry read API.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-38: EffectiveValueSourceResolver

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
resolve_effective_value_sources(configuration_id, keys) -> Vec<EffectiveValueSource>.
```

**Test boundary:** product-strategy/contracts/wu-1-38-effectivevaluesourceresolver.md; src-tauri/src/contracts/wu_1_38.rs; src/contracts/wu_1_38.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_38.rs; src-tauri/src/commands/wu_1_38.rs when IPC is declared; src/features/wu_1_38/**/* for UI WUs; src-tauri/tests/wu_1_38_contract.rs; src/test/wu_1_38.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `resolve_effective_value_sources` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `source enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Source attribution for defaults, inherited values, user values, and recovered values.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-39: EmptyGraphSimulator

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
simulate_empty_graph(configuration_id) -> EmptyGraphSimulation.
```

**Test boundary:** product-strategy/contracts/wu-1-39-emptygraphsimulator.md; src-tauri/src/contracts/wu_1_39.rs; src/contracts/wu_1_39.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_39.rs; src-tauri/src/commands/wu_1_39.rs when IPC is declared; src/features/wu_1_39/**/* for UI WUs; src-tauri/tests/wu_1_39_contract.rs; src/test/wu_1_39.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `simulate_empty_graph` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Initial roots, node kinds, templates, caps, provider routes, optimizer scope, and indexes for an empty graph.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-45, WU-1-51, WU-1-53, WU-1-56.

### WU-1-40: ShapeExplanationGenerator

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
explain_graph_shape(graph_snapshot_id, configuration_id) -> ShapeExplanation.
```

**Test boundary:** product-strategy/contracts/wu-1-40-shapeexplanationgenerator.md; src-tauri/src/contracts/wu_1_40.rs; src/contracts/wu_1_40.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_40.rs; src-tauri/src/commands/wu_1_40.rs when IPC is declared; src/features/wu_1_40/**/* for UI WUs; src-tauri/tests/wu_1_40_contract.rs; src/test/wu_1_40.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `explain_graph_shape` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-37, WU-1-38.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Current graph shape explanation traced to configuration/evidence/user actions.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-46, WU-1-52.

### WU-1-41: ConfigurationWarningWriter

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
write_configuration_warning(configuration_id, graph_snapshot_id, reason_code) -> AuditEventId.
```

**Test boundary:** product-strategy/contracts/wu-1-41-configurationwarningwriter.md; src-tauri/src/contracts/wu_1_41.rs; src/contracts/wu_1_41.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_41.rs; src-tauri/src/commands/wu_1_41.rs when IPC is declared; src/features/wu_1_41/**/* for UI WUs; src-tauri/tests/wu_1_41_contract.rs; src/test/wu_1_41.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `write_configuration_warning` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-37, WU-1-40.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Audit-backed warning emission for ambiguous or invalid memory semantics.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-47, WU-1-54.

### WU-1-42: ConfigurationOptimizerRequestEmitter

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
emit_configuration_optimizer_request(configuration_warning_id, target_node_ids) -> OptimizerRequestId.
```

**Test boundary:** product-strategy/contracts/wu-1-42-configurationoptimizerrequestemitter.md; src-tauri/src/contracts/wu_1_42.rs; src/contracts/wu_1_42.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_42.rs; src-tauri/src/commands/wu_1_42.rs when IPC is declared; src/features/wu_1_42/**/* for UI WUs; src-tauri/tests/wu_1_42_contract.rs; src/test/wu_1_42.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `emit_configuration_optimizer_request` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-41.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Advisory user_surface OptimizerRequest for configuration concerns.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-12, WU-1-33, WU-1-48, WU-1-55.

### WU-1-43: ConfigurationInspectorPane

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
ConfigurationInspectorPane { configuration_id, inspection_state, effective_sources, empty_graph_simulation, shape_explanation, warning_ids, optimizer_request_ids }.
```

**Test boundary:** product-strategy/contracts/wu-1-43-configurationinspectorpane.md; src-tauri/src/contracts/wu_1_43.rs; src/contracts/wu_1_43.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_43.rs; src-tauri/src/commands/wu_1_43.rs when IPC is declared; src/features/wu_1_43/**/* for UI WUs; src-tauri/tests/wu_1_43_contract.rs; src/test/wu_1_43.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render inspection states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Operator-visible configuration-as-memory pane.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-13, WU-1-21, WU-1-28, WU-1-34, WU-1-44, WU-1-49.

### WU-1-44: ConfigurationAuditSubscription

**Parent initiative:** VS-005: Inspect Configuration as Memory Semantics

**Contract:**
```text
subscribe_configuration_audit(workspace_id, configuration_id?) -> Stream<AuditEventView>.
```

**Test boundary:** product-strategy/contracts/wu-1-44-configurationauditsubscription.md; src-tauri/src/contracts/wu_1_44.rs; src/contracts/wu_1_44.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_44.rs; src-tauri/src/commands/wu_1_44.rs when IPC is declared; src/features/wu_1_44/**/* for UI WUs; src-tauri/tests/wu_1_44_contract.rs; src/test/wu_1_44.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `subscribe_configuration_audit` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-41, WU-1-42.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-05, WU-0B-32, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-21, WU-0B-25, WU-0B-26, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-08, WU-0C-09a, WU-0C-09, WU-0C-10, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-27, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Audit subscription for configuration inspection events.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-13, WU-1-21, WU-1-28, WU-1-34, WU-1-43, WU-1-49.

### WU-1-45: RedactedProviderProbeService

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
run_redacted_provider_probe(provider, cli, workspace_id) -> ProviderProbeObservation.
```

**Test boundary:** product-strategy/contracts/wu-1-45-redactedproviderprobeservice.md; src-tauri/src/contracts/wu_1_45.rs; src/contracts/wu_1_45.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_45.rs; src-tauri/src/commands/wu_1_45.rs when IPC is declared; src/features/wu_1_45/**/* for UI WUs; src-tauri/tests/wu_1_45_contract.rs; src/test/wu_1_45.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] `run_redacted_provider_probe` succeeds for an available provider and returns a `ProviderProbeObservation` without credential material.
- [ ] `run_redacted_provider_probe` fails for a denied or unreachable provider with a documented denial reason and no unrelated provider, budget, or graph mutation.
- [ ] Secret values in CLI args, environment variables, stderr, stdout, and audit metadata are redacted before observation storage.
- [ ] A probe attempt emits exactly one audit event carrying provider, CLI, workspace, result state, and redacted evidence references.
- [ ] The probe does not select provider accounts, rebalance quota lanes, refresh auth, mutate provider configuration, compose `--resume`, port sessions, or infer per-CLI session storage support.
- [ ] Any session-override compatibility shown beside provider state comes from WU-0C-N4/WU-0C-N5 evidence, not probe-owned JSONL or `state.db` inspection.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N4, WU-0C-N5.

**Produces:** Runtime provider probe service that stores no secret material.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-51, WU-1-53, WU-1-56.

**Revision rationale:** Round 3 keeps provider probing observational. The probe can report redacted readiness but cannot become a replacement for `agent-runner` routing, quota, resume, or session storage ownership.

### WU-1-56: FakeProviderProbeFixture

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
FakeProviderProbeFixture yields ready, degraded, blocked, stale, and probe_failed ProviderProbeObservation fixtures without touching real credentials.
```

**Test boundary:** product-strategy/contracts/wu-1-56-fakeproviderprobefixture.md; src-tauri/src/contracts/wu_1_56.rs; src/contracts/wu_1_56.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_56.rs; src-tauri/src/commands/wu_1_56.rs when IPC is declared; src/features/wu_1_56/**/* for UI WUs; src-tauri/tests/wu_1_56_contract.rs; src/test/wu_1_56.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] `ready fixture` yields a `ProviderProbeObservation` with ready state, valid freshness metadata, and no credential material.
- [ ] `degraded fixture` yields a `ProviderProbeObservation` with degraded state, degraded evidence, and no credential material.
- [ ] `blocked fixture` yields a `ProviderProbeObservation` with blocked state, denial reason, and no credential material.
- [ ] `stale fixture` yields a `ProviderProbeObservation` with stale state, stale timestamp evidence, and no credential material.
- [ ] `probe_failed fixture` yields a `ProviderProbeObservation` with probe_failed state, failure evidence, and no credential material.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Contract-only fake provider probe fixtures for WU-1-45 service tests and VS-006 fixture-backed tests; no downstream VS foundation edge.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53.

### WU-1-46: EntitlementObservationWriter

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
write_entitlement_observation(provider_state_id, model_id, feature_matrix, evidence_id) -> EntitlementSnapshotId.
```

**Test boundary:** product-strategy/contracts/wu-1-46-entitlementobservationwriter.md; src-tauri/src/contracts/wu_1_46.rs; src/contracts/wu_1_46.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_46.rs; src-tauri/src/commands/wu_1_46.rs when IPC is declared; src/features/wu_1_46/**/* for UI WUs; src-tauri/tests/wu_1_46_contract.rs; src/test/wu_1_46.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `write_entitlement_observation` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `entitlement_state enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-45.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Durable entitlement observations from redacted probes.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-52.

### WU-1-47: CapabilityMatrixPopulator

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
populate_capability_matrix(provider_state_id, entitlement_snapshot_ids) -> CapabilityFingerprintId.
```

**Test boundary:** product-strategy/contracts/wu-1-47-capabilitymatrixpopulator.md; src-tauri/src/contracts/wu_1_47.rs; src/contracts/wu_1_47.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_47.rs; src-tauri/src/commands/wu_1_47.rs when IPC is declared; src/features/wu_1_47/**/* for UI WUs; src-tauri/tests/wu_1_47_contract.rs; src/test/wu_1_47.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `populate_capability_matrix` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `route_state enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-45, WU-1-46.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** CapabilityFingerprint population over provider and entitlement state.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-54.

### WU-1-48: RouteEligibilityResolver

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
resolve_route_eligibility(workload_requirements, capability_fingerprint_id) -> RouteEligibility.
```

**Test boundary:** product-strategy/contracts/wu-1-48-routeeligibilityresolver.md; src-tauri/src/contracts/wu_1_48.rs; src/contracts/wu_1_48.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_48.rs; src-tauri/src/commands/wu_1_48.rs when IPC is declared; src/features/wu_1_48/**/* for UI WUs; src-tauri/tests/wu_1_48_contract.rs; src/test/wu_1_48.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `resolve_route_eligibility` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] Route eligibility returns harness policy/readiness decisions only; it never chooses concrete provider accounts, quota windows, auth profiles, resume strategies, session ports, or per-CLI storage adapters.
- [ ] Workload requirements may include a need for session override capability by opaque WU-0C-N4/WU-0C-N5 evidence refs, but the resolver does not probe or mutate sessions directly.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-47.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N4, WU-0C-N5.

**Produces:** Workload-specific provider route eligibility decisions.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-12, WU-1-33, WU-1-42, WU-1-55.

**Revision rationale:** Round 3 clarifies that this WU gates provider readiness for UI/future launches without owning provider routing. `agent-runner` still performs actual routing, balancing, resume composition, and session portability.

### WU-1-49: RouteDenialReasonClassifier

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
classify_route_denial(route_eligibility, provider_probe_observation) -> Vec<RouteDenialReason>.
```

**Test boundary:** product-strategy/contracts/wu-1-49-routedenialreasonclassifier.md; src-tauri/src/contracts/wu_1_49.rs; src/contracts/wu_1_49.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_49.rs; src-tauri/src/commands/wu_1_49.rs when IPC is declared; src/features/wu_1_49/**/* for UI WUs; src-tauri/tests/wu_1_49_contract.rs; src/test/wu_1_49.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `classify_route_denial` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] Denial reasons distinguish harness preflight policy/capability denials from `agent-runner` routing, quota, auth, resume, session-porting, and SessionOverrideContract refusal reasons.
- [ ] Session override refusal reasons are referenced by opaque WU-0C-N5 registry/evidence refs and are not recomputed from provider-native files.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-45, WU-1-48.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N4, WU-0C-N5.

**Produces:** Explicit denial reasons for provider panel and future launch gates.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-13, WU-1-21, WU-1-28, WU-1-34, WU-1-43, WU-1-44.

**Revision rationale:** Round 3 prevents denial classification from absorbing session override or provider routing internals. It labels observed refusal categories while leaving source-of-truth decisions in Phase 0C services and `agent-runner`.

### WU-1-50: ProviderPreflightPane

**Parent initiative:** VS-006: Preflight Providers and Expose Capability Fingerprints

**Contract:**
```text
ProviderPreflightPane { provider_state_id, entitlement_snapshot_ids, capability_fingerprint_id, route_state, denial_reasons, freshness, confidence }.
```

**Test boundary:** product-strategy/contracts/wu-1-50-providerpreflightpane.md; src-tauri/src/contracts/wu_1_50.rs; src/contracts/wu_1_50.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_50.rs; src-tauri/src/commands/wu_1_50.rs when IPC is declared; src/features/wu_1_50/**/* for UI WUs; src-tauri/tests/wu_1_50_contract.rs; src/test/wu_1_50.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render provider states` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The pane can display session-override capability/refusal metadata from WU-0C-N4/WU-0C-N5 as read-only evidence and never exposes raw transcript paths, provider-native JSONL, or adapter temp files.
- [ ] The pane offers no account picker, quota-balancing control, auth refresh control, resume/import control, provider reroute command, or SessionOverrideContract mutation command.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0A-15, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-17, WU-0B-18, WU-0B-19, WU-0B-28, WU-0B-29, WU-0B-31, WU-0B-16, WU-0B-04, WU-0B-15, WU-0C-04, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18, WU-0C-19, WU-0C-20a, WU-0C-20, WU-0C-30a, WU-0C-30, WU-0C-31a, WU-0C-31, WU-0C-32, WU-0C-33a, WU-0C-33, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.
- Cross-phase incoming (SessionOverrideContract): WU-0C-N4, WU-0C-N5.

**Produces:** Operator-visible provider and capability preflight panel.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-22, WU-1-35.

**Revision rationale:** Round 3 keeps the provider panel as read-only preflight. It may explain when future worker/orchestrator actions are blocked on `agent-runner` session features, but it does not implement those features.

### WU-1-51: InitiativeRootService

**Parent initiative:** VS-007: Show Initiative Roots and Current Focus

**Contract:**
```text
list_initiative_roots(workspace_id, graph_snapshot_id) -> Vec<InitiativeRootView>.
```

**Test boundary:** product-strategy/contracts/wu-1-51-initiativerootservice.md; src-tauri/src/contracts/wu_1_51.rs; src/contracts/wu_1_51.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_51.rs; src-tauri/src/commands/wu_1_51.rs when IPC is declared; src/features/wu_1_51/**/* for UI WUs; src-tauri/tests/wu_1_51_contract.rs; src/test/wu_1_51.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `list_initiative_roots` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-23, WU-0B-21, WU-0B-30, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0B-31, WU-0B-15, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Root initiative selector data over GraphStore.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-53, WU-1-56.

### WU-1-52: FocusPathService

**Parent initiative:** VS-007: Show Initiative Roots and Current Focus

**Contract:**
```text
get_current_focus_path(workspace_id, walk_state_id) -> FocusPathView.
```

**Test boundary:** product-strategy/contracts/wu-1-52-focuspathservice.md; src-tauri/src/contracts/wu_1_52.rs; src/contracts/wu_1_52.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_52.rs; src-tauri/src/commands/wu_1_52.rs when IPC is declared; src/features/wu_1_52/**/* for UI WUs; src-tauri/tests/wu_1_52_contract.rs; src/test/wu_1_52.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `get_current_focus_path` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-51.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-23, WU-0B-21, WU-0B-30, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0B-31, WU-0B-15, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Current focus path derived from AgentWalkState and graph containment.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46.

### WU-1-53: AgentWalkStateConsumerAdapter

**Parent initiative:** VS-007: Show Initiative Roots and Current Focus

**Contract:**
```text
read_agent_walk_state(workspace_id, orchestrator_id) -> AgentWalkStateView; subscribe_walk_state(workspace_id) -> Stream<AgentWalkStateView>.
```

**Test boundary:** product-strategy/contracts/wu-1-53-agentwalkstateconsumeradapter.md; src-tauri/src/contracts/wu_1_53.rs; src/contracts/wu_1_53.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_53.rs; src-tauri/src/commands/wu_1_53.rs when IPC is declared; src/features/wu_1_53/**/* for UI WUs; src-tauri/tests/wu_1_53_contract.rs; src/test/wu_1_53.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `read_agent_walk_state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `subscribe_walk_state` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: none.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-23, WU-0B-21, WU-0B-30, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0B-31, WU-0B-15, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Read/subscription adapter over Phase 0B AgentWalkState.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-56.

### WU-1-54: NotificationClassifier

**Parent initiative:** VS-007: Show Initiative Roots and Current Focus

**Contract:**
```text
classify_focus_notification(focus_item, provider_state?, budget_state?, recovery_state?) -> NotificationClass.
```

**Test boundary:** product-strategy/contracts/wu-1-54-notificationclassifier.md; src-tauri/src/contracts/wu_1_54.rs; src/contracts/wu_1_54.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_54.rs; src-tauri/src/commands/wu_1_54.rs when IPC is declared; src/features/wu_1_54/**/* for UI WUs; src-tauri/tests/wu_1_54_contract.rs; src/test/wu_1_54.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs.
- [ ] Every declared enum or state-machine value is reached by at least one fixture or input path.
- [ ] Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records.
- [ ] `classify_focus_notification` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `notification enum` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-52, WU-1-53.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-23, WU-0B-21, WU-0B-30, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0B-31, WU-0B-15, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Action-needed vs passive-progress classification for current focus.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47.

### WU-1-55: SingleTabShellExtension

**Parent initiative:** VS-007: Show Initiative Roots and Current Focus

**Contract:**
```text
SingleTabShellExtension { initiative_roots, focus_path, notification_class, active_pane_id }.
```

**Test boundary:** product-strategy/contracts/wu-1-55-singletabshellextension.md; src-tauri/src/contracts/wu_1_55.rs; src/contracts/wu_1_55.ts when TypeScript DTOs are present.

**Code boundary:** src-tauri/src/services/wu_1_55.rs; src-tauri/src/commands/wu_1_55.rs when IPC is declared; src/features/wu_1_55/**/* for UI WUs; src-tauri/tests/wu_1_55_contract.rs; src/test/wu_1_55.test.tsx for UI WUs.

**Acceptance criteria:**
- [ ] The declared view model renders from seeded IPC fixtures without requiring backend implementation details beyond the contract.
- [ ] Every declared UI state, label, or notification class appears in at least one fixture-backed render test.
- [ ] A fixture payload with an unknown state or missing required ID is rejected before rendering misleading UI.
- [ ] `render notification classes` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] `reject unknown class` has a binary fixture assertion covering its valid path and its documented invalid or denied path.
- [ ] The WU can merge as a single-concern PR without enabling another Phase 1 value slice by accident.

**Pipeline phases:** Phase 0 RCA skipped unless defect; Phase 1 research = gpt-high when implementation hookpoints are uncertain; Phase 2 synthesis = gpt-high; Phase 2.5 existing-state risk profile = gpt-high; Phase 3 proposal = gpt-high; Phase 4 risk gates = three independent claude-opus passes plus gpt-high audit reconciliation; Phase 5 hookpoints = gpt-high; Phase 6a contract handoff = orchestrator; Phase 6b tests = gpt-high with contract-only access; Phase 6c code = gpt-high with contracts plus tests; Phase 7 CodeRabbit = tool; Phase 8 PR gates = test-audit/commit-hygiene gpt-high plus multi-concern/justification claude-opus; Phase 9 draft PR; Phase 10 human promotion.

**Dependencies:**
- Phase 1 internal: WU-1-51, WU-1-52, WU-1-53, WU-1-54.
- Cross-phase incoming: WU-0A-05, WU-0A-06, WU-0A-07, WU-0A-08, WU-0A-09, WU-0A-10, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14a, WU-0A-14b, WU-0B-01, WU-0B-02, WU-0B-03, WU-0B-06, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-14, WU-0B-23, WU-0B-21, WU-0B-30, WU-0B-28, WU-0B-29, WU-0B-25, WU-0B-26, WU-0B-31, WU-0B-15, WU-0C-21, WU-0C-22a, WU-0C-22, WU-0C-23a, WU-0C-23b, WU-0C-23, WU-0C-24a, WU-0C-24, WU-0C-25a, WU-0C-25, WU-0C-26a, WU-0C-26, WU-0C-34, WU-0C-35a, WU-0C-35, WU-0C-36a, WU-0C-36, WU-0C-37a, WU-0C-37.

**Produces:** Single-tab UI shell extension for roots and current focus.

**Parallelizable with:** same topological wave after file ownership coordination: WU-1-12, WU-1-33, WU-1-42, WU-1-48.

## Dependency Graph

Cross-phase incoming dependencies are listed explicitly on every WU above. The local Phase 1 graph is:

```text
WU-1-01 <- none :: ImposedRenderLabel Enum
WU-1-02 <- none :: ToolCallNormalizedEvent DTO
WU-1-03 <- WU-1-02 :: ClaudeToolCallNormalizer
WU-1-04 <- WU-1-02 :: CodexToolCallNormalizer
WU-1-05 <- WU-1-02 :: OpencodeToolCallNormalizer
WU-1-06 <- WU-1-02, WU-1-03, WU-1-04, WU-1-05 :: ToolCallProvenanceWriter
WU-1-07 <- none :: EvidenceBlobStore
WU-1-08 <- none :: SideEffectClassifier
WU-1-09 <- none :: ApprovalStateCapture
WU-1-10 <- WU-1-03, WU-1-04, WU-1-05, WU-1-07 :: TranscriptIngestionAdapter
WU-1-11 <- none :: ParentInvocationPropagationService
WU-1-12 <- WU-1-06, WU-1-08, WU-1-09, WU-1-11 :: ToolCallAuditEmitter
WU-1-13 <- WU-1-06, WU-1-07, WU-1-12 :: ToolCallDrilldownComponent
WU-1-14 <- none :: BudgetLedgerScopeWriter
WU-1-15 <- none :: TokenCostEstimator
WU-1-16 <- none :: CachePrefixHashConsumer
WU-1-17 <- WU-1-14, WU-1-15, WU-1-16 :: RenderBudgetGateConsumerAdapter
WU-1-18 <- WU-1-17 :: BlockedRenderSurface
WU-1-19 <- WU-1-14, WU-1-15, WU-1-16 :: CostDisplayPanel
WU-1-20 <- none :: WorkingSetSnapshotReader
WU-1-21 <- WU-1-20, WU-1-12, WU-1-17 :: WorkingSetSnapshotWriter
WU-1-22 <- WU-1-20, WU-1-21 :: RenderEngineConsumerAdapter
WU-1-23 <- WU-1-07, WU-1-20 :: RenderEvidencePointerService
WU-1-24 <- WU-1-01, WU-1-17, WU-1-20, WU-1-23 :: RenderLabelClassifier
WU-1-25 <- WU-1-22, WU-1-23, WU-1-24 :: WorkingSetInspectorPane
WU-1-26 <- WU-1-15, WU-1-20 :: TokenEstimateDisplay
WU-1-27 <- WU-1-16, WU-1-20 :: CachePrefixDisplay
WU-1-28 <- WU-1-12, WU-1-20 :: RenderAuditSubscription
WU-1-29 <- WU-1-01 :: SummaryContractValidationResult DTO
WU-1-30 <- WU-1-29 :: SummaryContractValidator
WU-1-31 <- WU-1-07, WU-1-29 :: EvidenceLocatorValidator
WU-1-32 <- none :: SummaryTemplateRegistry
WU-1-33 <- WU-1-01, WU-1-29, WU-1-30, WU-1-31 :: RenderLabelDecoratorService
WU-1-34 <- WU-1-01, WU-1-29, WU-1-33 :: InvalidRenderLabelDto
WU-1-35 <- WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-34 :: SummaryContractFixturePack
WU-1-36 <- WU-1-34, WU-1-35 :: InvalidSummaryLabelComponent
WU-1-37 <- none :: ConfigurationInspectorService
WU-1-38 <- none :: EffectiveValueSourceResolver
WU-1-39 <- none :: EmptyGraphSimulator
WU-1-40 <- WU-1-37, WU-1-38 :: ShapeExplanationGenerator
WU-1-41 <- WU-1-37, WU-1-40 :: ConfigurationWarningWriter
WU-1-42 <- WU-1-41 :: ConfigurationOptimizerRequestEmitter
WU-1-43 <- WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42 :: ConfigurationInspectorPane
WU-1-44 <- WU-1-41, WU-1-42 :: ConfigurationAuditSubscription
WU-1-45 <- none :: RedactedProviderProbeService
WU-1-56 <- none :: FakeProviderProbeFixture
WU-1-46 <- WU-1-45 :: EntitlementObservationWriter
WU-1-47 <- WU-1-45, WU-1-46 :: CapabilityMatrixPopulator
WU-1-48 <- WU-1-47 :: RouteEligibilityResolver
WU-1-49 <- WU-1-45, WU-1-48 :: RouteDenialReasonClassifier
WU-1-50 <- WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49 :: ProviderPreflightPane
WU-1-51 <- none :: InitiativeRootService
WU-1-52 <- WU-1-51 :: FocusPathService
WU-1-53 <- none :: AgentWalkStateConsumerAdapter
WU-1-54 <- WU-1-52, WU-1-53 :: NotificationClassifier
WU-1-55 <- WU-1-51, WU-1-52, WU-1-53, WU-1-54 :: SingleTabShellExtension
```

Cross-phase incoming graph by value slice:

### Incoming for Shared

- WU-0A-01 -> Shared
- WU-0A-02 -> Shared
- WU-0A-03 -> Shared
- WU-0A-04 -> Shared
- WU-0A-05 -> Shared
- WU-0A-06 -> Shared
- WU-0A-07 -> Shared
- WU-0A-08 -> Shared
- WU-0A-09 -> Shared
- WU-0A-10 -> Shared
- WU-0B-01 -> Shared
- WU-0B-02 -> Shared
- WU-0B-03 -> Shared
- WU-0B-15 -> Shared
- WU-0B-04 -> Shared
- WU-0C-34 -> Shared
- WU-0C-35a -> Shared
- WU-0C-35 -> Shared
- WU-0C-37a -> Shared
- WU-0C-37 -> Shared

### Incoming for VS-001

- WU-0A-01 -> VS-001
- WU-0A-05 -> VS-001
- WU-0A-06 -> VS-001
- WU-0A-07 -> VS-001
- WU-0A-08 -> VS-001
- WU-0A-09 -> VS-001
- WU-0A-10 -> VS-001
- WU-0A-11 -> VS-001
- WU-0A-12 -> VS-001
- WU-0A-13 -> VS-001
- WU-0A-14a -> VS-001
- WU-0A-14b -> VS-001
- WU-0A-15 -> VS-001
- WU-0B-01 -> VS-001
- WU-0B-02 -> VS-001
- WU-0B-03 -> VS-001
- WU-0B-06 -> VS-001
- WU-0B-07 -> VS-001
- WU-0B-08 -> VS-001
- WU-0B-12 -> VS-001
- WU-0B-14 -> VS-001
- WU-0B-09 -> VS-001
- WU-0B-10 -> VS-001
- WU-0B-11 -> VS-001
- WU-0B-15 -> VS-001
- WU-0B-16 -> VS-001
- WU-0B-17 -> VS-001
- WU-0B-18 -> VS-001
- WU-0B-19 -> VS-001
- WU-0B-21 -> VS-001
- WU-0B-24 -> VS-001
- WU-0B-05 -> VS-001
- WU-0B-32 -> VS-001
- WU-0B-04 -> VS-001
- WU-0C-08 -> VS-001
- WU-0C-09a -> VS-001
- WU-0C-09 -> VS-001
- WU-0C-10 -> VS-001
- WU-0C-05 -> VS-001
- WU-0C-06a -> VS-001
- WU-0C-06 -> VS-001
- WU-0C-07a -> VS-001
- WU-0C-07 -> VS-001
- WU-0C-21 -> VS-001
- WU-0C-22a -> VS-001
- WU-0C-22 -> VS-001
- WU-0C-23a -> VS-001
- WU-0C-23b -> VS-001
- WU-0C-23 -> VS-001
- WU-0C-24a -> VS-001
- WU-0C-24 -> VS-001
- WU-0C-25a -> VS-001
- WU-0C-25 -> VS-001
- WU-0C-26a -> VS-001
- WU-0C-26 -> VS-001
- WU-0C-11a -> VS-001
- WU-0C-11b -> VS-001
- WU-0C-12a -> VS-001
- WU-0C-12 -> VS-001
- WU-0C-13a -> VS-001
- WU-0C-13b -> VS-001
- WU-0C-13 -> VS-001
- WU-0C-14a -> VS-001
- WU-0C-14b -> VS-001
- WU-0C-14 -> VS-001
- WU-0C-15a -> VS-001
- WU-0C-15b -> VS-001
- WU-0C-15c -> VS-001
- WU-0C-15d -> VS-001
- WU-0C-16a -> VS-001
- WU-0C-16b -> VS-001
- WU-0C-16 -> VS-001
- WU-0C-17a -> VS-001
- WU-0C-17b -> VS-001
- WU-0C-17 -> VS-001
- WU-0C-18 -> VS-001
- WU-0C-19 -> VS-001
- WU-0C-20a -> VS-001
- WU-0C-20 -> VS-001
- WU-0C-32 -> VS-001
- WU-0C-33a -> VS-001
- WU-0C-33 -> VS-001
- WU-0C-34 -> VS-001
- WU-0C-35a -> VS-001
- WU-0C-35 -> VS-001
- WU-0C-36a -> VS-001
- WU-0C-36 -> VS-001
- WU-0C-37a -> VS-001
- WU-0C-37 -> VS-001

### Incoming for VS-002

- WU-0A-11 -> VS-002
- WU-0A-12 -> VS-002
- WU-0A-13 -> VS-002
- WU-0A-14a -> VS-002
- WU-0A-14b -> VS-002
- WU-0B-01 -> VS-002
- WU-0B-02 -> VS-002
- WU-0B-03 -> VS-002
- WU-0B-06 -> VS-002
- WU-0B-07 -> VS-002
- WU-0B-08 -> VS-002
- WU-0B-12 -> VS-002
- WU-0B-14 -> VS-002
- WU-0B-09 -> VS-002
- WU-0B-10 -> VS-002
- WU-0B-11 -> VS-002
- WU-0B-05 -> VS-002
- WU-0B-32 -> VS-002
- WU-0B-04 -> VS-002
- WU-0B-15 -> VS-002
- WU-0C-04 -> VS-002
- WU-0C-08 -> VS-002
- WU-0C-09a -> VS-002
- WU-0C-09 -> VS-002
- WU-0C-10 -> VS-002
- WU-0C-21 -> VS-002
- WU-0C-22a -> VS-002
- WU-0C-22 -> VS-002
- WU-0C-23a -> VS-002
- WU-0C-23b -> VS-002
- WU-0C-23 -> VS-002
- WU-0C-24a -> VS-002
- WU-0C-24 -> VS-002
- WU-0C-25a -> VS-002
- WU-0C-25 -> VS-002
- WU-0C-26a -> VS-002
- WU-0C-26 -> VS-002
- WU-0C-36a -> VS-002
- WU-0C-36 -> VS-002
- WU-0C-37a -> VS-002
- WU-0C-37 -> VS-002

### Incoming for VS-003

- WU-0A-05 -> VS-003
- WU-0A-06 -> VS-003
- WU-0A-07 -> VS-003
- WU-0A-08 -> VS-003
- WU-0A-09 -> VS-003
- WU-0A-10 -> VS-003
- WU-0A-14a -> VS-003
- WU-0A-14b -> VS-003
- WU-0A-15 -> VS-003
- WU-0B-01 -> VS-003
- WU-0B-02 -> VS-003
- WU-0B-03 -> VS-003
- WU-0B-09 -> VS-003
- WU-0B-10 -> VS-003
- WU-0B-15 -> VS-003
- WU-0B-20 -> VS-003
- WU-0B-22 -> VS-003
- WU-0B-24 -> VS-003
- WU-0B-04 -> VS-003
- WU-0B-31 -> VS-003
- WU-0C-04 -> VS-003
- WU-0C-11a -> VS-003
- WU-0C-11b -> VS-003
- WU-0C-12a -> VS-003
- WU-0C-12 -> VS-003
- WU-0C-13a -> VS-003
- WU-0C-13b -> VS-003
- WU-0C-13 -> VS-003
- WU-0C-14a -> VS-003
- WU-0C-14b -> VS-003
- WU-0C-14 -> VS-003
- WU-0C-15a -> VS-003
- WU-0C-15b -> VS-003
- WU-0C-15c -> VS-003
- WU-0C-15d -> VS-003
- WU-0C-16a -> VS-003
- WU-0C-16b -> VS-003
- WU-0C-16 -> VS-003
- WU-0C-17a -> VS-003
- WU-0C-17b -> VS-003
- WU-0C-17 -> VS-003
- WU-0C-18 -> VS-003
- WU-0C-30a -> VS-003
- WU-0C-30 -> VS-003
- WU-0C-31a -> VS-003
- WU-0C-31 -> VS-003
- WU-0C-32 -> VS-003
- WU-0C-33a -> VS-003
- WU-0C-33 -> VS-003
- WU-0C-34 -> VS-003
- WU-0C-35a -> VS-003
- WU-0C-35 -> VS-003
- WU-0C-37a -> VS-003
- WU-0C-37 -> VS-003

### Incoming for VS-004

- WU-0A-05 -> VS-004
- WU-0A-06 -> VS-004
- WU-0A-07 -> VS-004
- WU-0A-08 -> VS-004
- WU-0A-11 -> VS-004
- WU-0A-12 -> VS-004
- WU-0A-13 -> VS-004
- WU-0A-14a -> VS-004
- WU-0A-14b -> VS-004
- WU-0B-01 -> VS-004
- WU-0B-02 -> VS-004
- WU-0B-03 -> VS-004
- WU-0B-16 -> VS-004
- WU-0B-17 -> VS-004
- WU-0B-18 -> VS-004
- WU-0B-19 -> VS-004
- WU-0B-04 -> VS-004
- WU-0B-15 -> VS-004
- WU-0B-21 -> VS-004
- WU-0B-24 -> VS-004
- WU-0B-28 -> VS-004
- WU-0B-29 -> VS-004
- WU-0B-25 -> VS-004
- WU-0B-26 -> VS-004
- WU-0C-04 -> VS-004
- WU-0C-08 -> VS-004
- WU-0C-09a -> VS-004
- WU-0C-09 -> VS-004
- WU-0C-10 -> VS-004
- WU-0C-05 -> VS-004
- WU-0C-06a -> VS-004
- WU-0C-06 -> VS-004
- WU-0C-07a -> VS-004
- WU-0C-07 -> VS-004
- WU-0C-21 -> VS-004
- WU-0C-22a -> VS-004
- WU-0C-22 -> VS-004
- WU-0C-23a -> VS-004
- WU-0C-23b -> VS-004
- WU-0C-23 -> VS-004
- WU-0C-24a -> VS-004
- WU-0C-24 -> VS-004
- WU-0C-25a -> VS-004
- WU-0C-25 -> VS-004
- WU-0C-26a -> VS-004
- WU-0C-26 -> VS-004
- WU-0C-34 -> VS-004
- WU-0C-35a -> VS-004
- WU-0C-35 -> VS-004
- WU-0C-36a -> VS-004
- WU-0C-36 -> VS-004
- WU-0C-37a -> VS-004
- WU-0C-37 -> VS-004

### Incoming for VS-005

- WU-0A-05 -> VS-005
- WU-0A-06 -> VS-005
- WU-0A-07 -> VS-005
- WU-0A-08 -> VS-005
- WU-0A-09 -> VS-005
- WU-0A-10 -> VS-005
- WU-0A-11 -> VS-005
- WU-0A-12 -> VS-005
- WU-0A-13 -> VS-005
- WU-0A-14a -> VS-005
- WU-0A-14b -> VS-005
- WU-0B-01 -> VS-005
- WU-0B-02 -> VS-005
- WU-0B-03 -> VS-005
- WU-0B-05 -> VS-005
- WU-0B-32 -> VS-005
- WU-0B-06 -> VS-005
- WU-0B-07 -> VS-005
- WU-0B-08 -> VS-005
- WU-0B-12 -> VS-005
- WU-0B-14 -> VS-005
- WU-0B-21 -> VS-005
- WU-0B-25 -> VS-005
- WU-0B-26 -> VS-005
- WU-0B-04 -> VS-005
- WU-0B-15 -> VS-005
- WU-0C-04 -> VS-005
- WU-0C-08 -> VS-005
- WU-0C-09a -> VS-005
- WU-0C-09 -> VS-005
- WU-0C-10 -> VS-005
- WU-0C-21 -> VS-005
- WU-0C-22a -> VS-005
- WU-0C-22 -> VS-005
- WU-0C-23a -> VS-005
- WU-0C-23b -> VS-005
- WU-0C-23 -> VS-005
- WU-0C-24a -> VS-005
- WU-0C-24 -> VS-005
- WU-0C-25a -> VS-005
- WU-0C-25 -> VS-005
- WU-0C-26a -> VS-005
- WU-0C-26 -> VS-005
- WU-0C-27 -> VS-005
- WU-0C-34 -> VS-005
- WU-0C-35a -> VS-005
- WU-0C-35 -> VS-005
- WU-0C-36a -> VS-005
- WU-0C-36 -> VS-005
- WU-0C-37a -> VS-005
- WU-0C-37 -> VS-005

### Incoming for VS-006

- WU-0A-05 -> VS-006
- WU-0A-06 -> VS-006
- WU-0A-07 -> VS-006
- WU-0A-08 -> VS-006
- WU-0A-09 -> VS-006
- WU-0A-10 -> VS-006
- WU-0A-11 -> VS-006
- WU-0A-12 -> VS-006
- WU-0A-13 -> VS-006
- WU-0A-14a -> VS-006
- WU-0A-14b -> VS-006
- WU-0A-15 -> VS-006
- WU-0B-01 -> VS-006
- WU-0B-02 -> VS-006
- WU-0B-03 -> VS-006
- WU-0B-17 -> VS-006
- WU-0B-18 -> VS-006
- WU-0B-19 -> VS-006
- WU-0B-28 -> VS-006
- WU-0B-29 -> VS-006
- WU-0B-31 -> VS-006
- WU-0B-16 -> VS-006
- WU-0B-04 -> VS-006
- WU-0B-15 -> VS-006
- WU-0C-04 -> VS-006
- WU-0C-21 -> VS-006
- WU-0C-22a -> VS-006
- WU-0C-22 -> VS-006
- WU-0C-23a -> VS-006
- WU-0C-23b -> VS-006
- WU-0C-23 -> VS-006
- WU-0C-24a -> VS-006
- WU-0C-24 -> VS-006
- WU-0C-25a -> VS-006
- WU-0C-25 -> VS-006
- WU-0C-26a -> VS-006
- WU-0C-26 -> VS-006
- WU-0C-11a -> VS-006
- WU-0C-11b -> VS-006
- WU-0C-12a -> VS-006
- WU-0C-12 -> VS-006
- WU-0C-13a -> VS-006
- WU-0C-13b -> VS-006
- WU-0C-13 -> VS-006
- WU-0C-14a -> VS-006
- WU-0C-14b -> VS-006
- WU-0C-14 -> VS-006
- WU-0C-15a -> VS-006
- WU-0C-15b -> VS-006
- WU-0C-15c -> VS-006
- WU-0C-15d -> VS-006
- WU-0C-16a -> VS-006
- WU-0C-16b -> VS-006
- WU-0C-16 -> VS-006
- WU-0C-17a -> VS-006
- WU-0C-17b -> VS-006
- WU-0C-17 -> VS-006
- WU-0C-18 -> VS-006
- WU-0C-19 -> VS-006
- WU-0C-20a -> VS-006
- WU-0C-20 -> VS-006
- WU-0C-30a -> VS-006
- WU-0C-30 -> VS-006
- WU-0C-31a -> VS-006
- WU-0C-31 -> VS-006
- WU-0C-32 -> VS-006
- WU-0C-33a -> VS-006
- WU-0C-33 -> VS-006
- WU-0C-34 -> VS-006
- WU-0C-35a -> VS-006
- WU-0C-35 -> VS-006
- WU-0C-36a -> VS-006
- WU-0C-36 -> VS-006
- WU-0C-37a -> VS-006
- WU-0C-37 -> VS-006

### Incoming for VS-007

- WU-0A-05 -> VS-007
- WU-0A-06 -> VS-007
- WU-0A-07 -> VS-007
- WU-0A-08 -> VS-007
- WU-0A-09 -> VS-007
- WU-0A-10 -> VS-007
- WU-0A-11 -> VS-007
- WU-0A-12 -> VS-007
- WU-0A-13 -> VS-007
- WU-0A-14a -> VS-007
- WU-0A-14b -> VS-007
- WU-0B-01 -> VS-007
- WU-0B-02 -> VS-007
- WU-0B-03 -> VS-007
- WU-0B-06 -> VS-007
- WU-0B-07 -> VS-007
- WU-0B-08 -> VS-007
- WU-0B-12 -> VS-007
- WU-0B-14 -> VS-007
- WU-0B-23 -> VS-007
- WU-0B-21 -> VS-007
- WU-0B-30 -> VS-007
- WU-0B-28 -> VS-007
- WU-0B-29 -> VS-007
- WU-0B-25 -> VS-007
- WU-0B-26 -> VS-007
- WU-0B-31 -> VS-007
- WU-0B-15 -> VS-007
- WU-0C-21 -> VS-007
- WU-0C-22a -> VS-007
- WU-0C-22 -> VS-007
- WU-0C-23a -> VS-007
- WU-0C-23b -> VS-007
- WU-0C-23 -> VS-007
- WU-0C-24a -> VS-007
- WU-0C-24 -> VS-007
- WU-0C-25a -> VS-007
- WU-0C-25 -> VS-007
- WU-0C-26a -> VS-007
- WU-0C-26 -> VS-007
- WU-0C-34 -> VS-007
- WU-0C-35a -> VS-007
- WU-0C-35 -> VS-007
- WU-0C-36a -> VS-007
- WU-0C-36 -> VS-007
- WU-0C-37a -> VS-007
- WU-0C-37 -> VS-007

### Round 3 SessionOverrideContract incoming overlay

The Phase 0C-r4 outgoing declarations add WU-0C-N1..WU-0C-N5 as prerequisites for downstream session read/write consumers. Phase 1 has no worker-launcher or worker-output-reintegration WU in this artifact; those remain downstream VS-015/VS-018 work. The relevant Phase 1-local edges are:

- WU-0C-N2 -> WU-1-03, WU-1-04, WU-1-05 for canonical `TranscriptTurn` / session metadata fixture shapes used by provider-shape normalizers.
- WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5 -> WU-1-10 for read-only transcript ingestion through `read_transcript` / metadata and v1 adapter refusal evidence.
- WU-0C-N5 -> WU-1-12, WU-1-13, WU-1-28 for audit stream and drill-down references to override receipts/refusals.
- WU-0C-N2, WU-0C-N5 -> WU-1-23, WU-1-25 for imposed-context evidence pointers and inspector display of override-derived metadata.
- WU-0C-N4, WU-0C-N5 -> WU-1-45, WU-1-48, WU-1-49, WU-1-50 for read-only provider preflight display of adapter capability/refusal evidence.

Bidirectional check against Phase 0C r4: WU-0C-N5 explicitly feeds VS-001 evidence inspectors and VS-003 audit surfaces; WU-0C-N1..WU-0C-N5 explicitly feed Phase 1 worker-launcher / worker-output-reintegration successors. This Phase 1 r3 artifact wires the VS-001/VS-003/VS-006 read-only portions now and leaves VS-015/VS-018 mutation dependencies to their owning future phase artifacts.

## Critical Path

Longest Phase 1-local acceptance path:

```text
WU-1-02 -> WU-1-03 -> WU-1-06 -> WU-1-12 -> WU-1-14 -> WU-1-17 -> WU-1-20 -> WU-1-21 -> WU-1-22 -> WU-1-24 -> WU-1-25
```

This path runs from normalized CLI tool evidence through provenance/audit, budget gate adaptation, WorkingSetSnapshot inspection metadata, RenderEngine consumption, render label classification, and the inspector pane. It is the acceptance path for VS-001 because VS-001 must show real evidence, token, cache, provider, configuration, and audit fields rather than placeholders.

Round 2 re-check: WU-1-56 is a Wave 1 contract-only fixture pack for VS-006 tests and does not add a runtime edge to the VS-001 acceptance path, so the critical path is unchanged.

Round 3 re-check: SessionOverrideContract adds cross-phase prerequisites to WU-1-10 and read-only evidence/audit/display WUs, but no new Phase 1-local dependency edge. The Phase 1-local critical path remains unchanged; upstream readiness now additionally requires WU-0C-N1..WU-0C-N5 before transcript-ingestion and override-derived evidence display are accepted.

Phase 0 critical incoming gates for this path: WU-0B-09, WU-0B-10, WU-0B-15, WU-0B-16, WU-0B-17, WU-0B-19, WU-0B-20, WU-0B-21, WU-0C-04, WU-0C-05..WU-0C-07, WU-0C-21..WU-0C-26, WU-0C-34..WU-0C-37, and the Phase 0A IPC/UI/test substrate WU-0A-05..WU-0A-15.

## Parallelization Map

Topological-level partition: every dependency of a WU in wave N has a wave number lower than N. File ownership still needs per-PR coordination, but there are no intra-wave dependency edges.

Round 3 re-derive: the topological waves remain 18+12+9+5+7+3+2 because SessionOverrideContract edges are cross-phase incoming gates, not new Phase 1-local edges. Wave reshuffling is therefore external: WU-1-10, WU-1-12, WU-1-13, WU-1-23, WU-1-25, WU-1-28, WU-1-45, WU-1-48, WU-1-49, and WU-1-50 now wait for the named WU-0C-N* prerequisites before their tests can be accepted, but their relative Phase 1 ordering is unchanged.

| Wave | Work units | Count |
|---:|---|---:|
| 1 | WU-1-01, WU-1-02, WU-1-07, WU-1-08, WU-1-09, WU-1-11, WU-1-14, WU-1-15, WU-1-16, WU-1-20, WU-1-32, WU-1-37, WU-1-38, WU-1-39, WU-1-45, WU-1-51, WU-1-53, WU-1-56 | 18 |
| 2 | WU-1-03, WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52 | 12 |
| 3 | WU-1-06, WU-1-10, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54 | 9 |
| 4 | WU-1-12, WU-1-33, WU-1-42, WU-1-48, WU-1-55 | 5 |
| 5 | WU-1-13, WU-1-21, WU-1-28, WU-1-34, WU-1-43, WU-1-44, WU-1-49 | 7 |
| 6 | WU-1-22, WU-1-35, WU-1-50 | 3 |
| 7 | WU-1-25, WU-1-36 | 2 |

## Run Report

| Rule | Result | Evidence |
|---|---|---|
| D1 per-object granularity | Applied | 56 WUs; service objects, DTOs, enum taxonomies, adapters, UI components, and fixture packs are separate rather than bundled per VS. WU-1-45 now owns only the runtime service and WU-1-56 owns FakeProviderProbeFixture. |
| D2 binary criteria | Applied | Each WU has method, enum, state-machine, or fixture-state criteria plus binary fixture assertions generated from its Contract. |
| D3 regression check | Performed for this draft only | I checked WU count, internal dependency IDs, topological wave acyclicity, SessionOverrideContract cross-phase overlay, and explicit blocked-on annotations by text scan. I did not run external reviewer tools or implementation tests because this is a roadmap artifact. |
| D4 watch-signal compliance | Covered with residual review risk | bundling-family remains closed by the WU-1-45/WU-1-56 split; state-machine-criteria-family remains addressed by per-method criteria; dependency-encoding-family is updated for WU-0C-N1..WU-0C-N5 incoming edges; fix-created-family reopens at generation 0 for this externally-driven Round 3 cascade from proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4. |

### Rule D1 - Per-Schema-Object and Per-Service Granularity

D1 audit count: 56 rows = 56 WUs in the inventory.

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

| Metric | Value |
|---|---:|
| Total Phase 1 WUs | 56 |
| Topological waves | 7 |
| Maximum same-wave concurrency | 18 |
| Cross-phase upstream phases consumed | 3 |

### Self-classification

- Round 2 brownfield: split WU-1-45 RedactedProviderProbeService from WU-1-56 FakeProviderProbeFixture, moved the canonical D1 ownership audit into Run Report, placed WU-1-56 in Wave 1, and left downstream Stitch Notes unchanged because fixture WUs do not become Phase 2+ foundation edges.
- Round 3 brownfield: classification `fix-created-family` gen 0, externally driven by proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4. No WU was removed or merged. Phase 1 r2 contains no worker-launcher, worker-output-reintegration, or orchestrator-bridge mutation WU, so the refactor narrows read-only transcript/provenance/provider surfaces and records downstream blockers instead of collapsing WUs.

## Stitch Notes

### Incoming From Phase 0A

- (WU-0A-01, VS-001)
- (WU-0A-05, VS-001)
- (WU-0A-06, VS-001)
- (WU-0A-07, VS-001)
- (WU-0A-08, VS-001)
- (WU-0A-09, VS-001)
- (WU-0A-10, VS-001)
- (WU-0A-11, VS-001)
- (WU-0A-12, VS-001)
- (WU-0A-13, VS-001)
- (WU-0A-14a, VS-001)
- (WU-0A-14b, VS-001)
- (WU-0A-15, VS-001)
- (WU-0A-11, VS-002)
- (WU-0A-12, VS-002)
- (WU-0A-13, VS-002)
- (WU-0A-14a, VS-002)
- (WU-0A-14b, VS-002)
- (WU-0A-05, VS-003)
- (WU-0A-06, VS-003)
- (WU-0A-07, VS-003)
- (WU-0A-08, VS-003)
- (WU-0A-09, VS-003)
- (WU-0A-10, VS-003)
- (WU-0A-14a, VS-003)
- (WU-0A-14b, VS-003)
- (WU-0A-15, VS-003)
- (WU-0A-05, VS-004)
- (WU-0A-06, VS-004)
- (WU-0A-07, VS-004)
- (WU-0A-08, VS-004)
- (WU-0A-11, VS-004)
- (WU-0A-12, VS-004)
- (WU-0A-13, VS-004)
- (WU-0A-14a, VS-004)
- (WU-0A-14b, VS-004)
- (WU-0A-05, VS-005)
- (WU-0A-06, VS-005)
- (WU-0A-07, VS-005)
- (WU-0A-08, VS-005)
- (WU-0A-09, VS-005)
- (WU-0A-10, VS-005)
- (WU-0A-11, VS-005)
- (WU-0A-12, VS-005)
- (WU-0A-13, VS-005)
- (WU-0A-14a, VS-005)
- (WU-0A-14b, VS-005)
- (WU-0A-05, VS-006)
- (WU-0A-06, VS-006)
- (WU-0A-07, VS-006)
- (WU-0A-08, VS-006)
- (WU-0A-09, VS-006)
- (WU-0A-10, VS-006)
- (WU-0A-11, VS-006)
- (WU-0A-12, VS-006)
- (WU-0A-13, VS-006)
- (WU-0A-14a, VS-006)
- (WU-0A-14b, VS-006)
- (WU-0A-15, VS-006)
- (WU-0A-05, VS-007)
- (WU-0A-06, VS-007)
- (WU-0A-07, VS-007)
- (WU-0A-08, VS-007)
- (WU-0A-09, VS-007)
- (WU-0A-10, VS-007)
- (WU-0A-11, VS-007)
- (WU-0A-12, VS-007)
- (WU-0A-13, VS-007)
- (WU-0A-14a, VS-007)
- (WU-0A-14b, VS-007)

### Incoming From Phase 0B

- (WU-0B-01, VS-001)
- (WU-0B-02, VS-001)
- (WU-0B-03, VS-001)
- (WU-0B-06, VS-001)
- (WU-0B-07, VS-001)
- (WU-0B-08, VS-001)
- (WU-0B-12, VS-001)
- (WU-0B-14, VS-001)
- (WU-0B-09, VS-001)
- (WU-0B-10, VS-001)
- (WU-0B-11, VS-001)
- (WU-0B-15, VS-001)
- (WU-0B-16, VS-001)
- (WU-0B-17, VS-001)
- (WU-0B-18, VS-001)
- (WU-0B-19, VS-001)
- (WU-0B-21, VS-001)
- (WU-0B-24, VS-001)
- (WU-0B-05, VS-001)
- (WU-0B-32, VS-001)
- (WU-0B-04, VS-001)
- (WU-0B-01, VS-002)
- (WU-0B-02, VS-002)
- (WU-0B-03, VS-002)
- (WU-0B-06, VS-002)
- (WU-0B-07, VS-002)
- (WU-0B-08, VS-002)
- (WU-0B-12, VS-002)
- (WU-0B-14, VS-002)
- (WU-0B-09, VS-002)
- (WU-0B-10, VS-002)
- (WU-0B-11, VS-002)
- (WU-0B-05, VS-002)
- (WU-0B-32, VS-002)
- (WU-0B-04, VS-002)
- (WU-0B-15, VS-002)
- (WU-0B-01, VS-003)
- (WU-0B-02, VS-003)
- (WU-0B-03, VS-003)
- (WU-0B-09, VS-003)
- (WU-0B-10, VS-003)
- (WU-0B-15, VS-003)
- (WU-0B-20, VS-003)
- (WU-0B-22, VS-003)
- (WU-0B-24, VS-003)
- (WU-0B-04, VS-003)
- (WU-0B-31, VS-003)
- (WU-0B-01, VS-004)
- (WU-0B-02, VS-004)
- (WU-0B-03, VS-004)
- (WU-0B-16, VS-004)
- (WU-0B-17, VS-004)
- (WU-0B-18, VS-004)
- (WU-0B-19, VS-004)
- (WU-0B-04, VS-004)
- (WU-0B-15, VS-004)
- (WU-0B-21, VS-004)
- (WU-0B-24, VS-004)
- (WU-0B-28, VS-004)
- (WU-0B-29, VS-004)
- (WU-0B-25, VS-004)
- (WU-0B-26, VS-004)
- (WU-0B-01, VS-005)
- (WU-0B-02, VS-005)
- (WU-0B-03, VS-005)
- (WU-0B-05, VS-005)
- (WU-0B-32, VS-005)
- (WU-0B-06, VS-005)
- (WU-0B-07, VS-005)
- (WU-0B-08, VS-005)
- (WU-0B-12, VS-005)
- (WU-0B-14, VS-005)
- (WU-0B-21, VS-005)
- (WU-0B-25, VS-005)
- (WU-0B-26, VS-005)
- (WU-0B-04, VS-005)
- (WU-0B-15, VS-005)
- (WU-0B-01, VS-006)
- (WU-0B-02, VS-006)
- (WU-0B-03, VS-006)
- (WU-0B-17, VS-006)
- (WU-0B-18, VS-006)
- (WU-0B-19, VS-006)
- (WU-0B-28, VS-006)
- (WU-0B-29, VS-006)
- (WU-0B-31, VS-006)
- (WU-0B-16, VS-006)
- (WU-0B-04, VS-006)
- (WU-0B-15, VS-006)
- (WU-0B-01, VS-007)
- (WU-0B-02, VS-007)
- (WU-0B-03, VS-007)
- (WU-0B-06, VS-007)
- (WU-0B-07, VS-007)
- (WU-0B-08, VS-007)
- (WU-0B-12, VS-007)
- (WU-0B-14, VS-007)
- (WU-0B-23, VS-007)
- (WU-0B-21, VS-007)
- (WU-0B-30, VS-007)
- (WU-0B-28, VS-007)
- (WU-0B-29, VS-007)
- (WU-0B-25, VS-007)
- (WU-0B-26, VS-007)
- (WU-0B-31, VS-007)
- (WU-0B-15, VS-007)

### Incoming From Phase 0C

- (WU-0C-08, VS-001)
- (WU-0C-09a, VS-001)
- (WU-0C-09, VS-001)
- (WU-0C-10, VS-001)
- (WU-0C-05, VS-001)
- (WU-0C-06a, VS-001)
- (WU-0C-06, VS-001)
- (WU-0C-07a, VS-001)
- (WU-0C-07, VS-001)
- (WU-0C-21, VS-001)
- (WU-0C-22a, VS-001)
- (WU-0C-22, VS-001)
- (WU-0C-23a, VS-001)
- (WU-0C-23b, VS-001)
- (WU-0C-23, VS-001)
- (WU-0C-24a, VS-001)
- (WU-0C-24, VS-001)
- (WU-0C-25a, VS-001)
- (WU-0C-25, VS-001)
- (WU-0C-26a, VS-001)
- (WU-0C-26, VS-001)
- (WU-0C-11a, VS-001)
- (WU-0C-11b, VS-001)
- (WU-0C-12a, VS-001)
- (WU-0C-12, VS-001)
- (WU-0C-13a, VS-001)
- (WU-0C-13b, VS-001)
- (WU-0C-13, VS-001)
- (WU-0C-14a, VS-001)
- (WU-0C-14b, VS-001)
- (WU-0C-14, VS-001)
- (WU-0C-15a, VS-001)
- (WU-0C-15b, VS-001)
- (WU-0C-15c, VS-001)
- (WU-0C-15d, VS-001)
- (WU-0C-16a, VS-001)
- (WU-0C-16b, VS-001)
- (WU-0C-16, VS-001)
- (WU-0C-17a, VS-001)
- (WU-0C-17b, VS-001)
- (WU-0C-17, VS-001)
- (WU-0C-18, VS-001)
- (WU-0C-19, VS-001)
- (WU-0C-20a, VS-001)
- (WU-0C-20, VS-001)
- (WU-0C-32, VS-001)
- (WU-0C-33a, VS-001)
- (WU-0C-33, VS-001)
- (WU-0C-34, VS-001)
- (WU-0C-35a, VS-001)
- (WU-0C-35, VS-001)
- (WU-0C-36a, VS-001)
- (WU-0C-36, VS-001)
- (WU-0C-37a, VS-001)
- (WU-0C-37, VS-001)
- (WU-0C-04, VS-002)
- (WU-0C-08, VS-002)
- (WU-0C-09a, VS-002)
- (WU-0C-09, VS-002)
- (WU-0C-10, VS-002)
- (WU-0C-21, VS-002)
- (WU-0C-22a, VS-002)
- (WU-0C-22, VS-002)
- (WU-0C-23a, VS-002)
- (WU-0C-23b, VS-002)
- (WU-0C-23, VS-002)
- (WU-0C-24a, VS-002)
- (WU-0C-24, VS-002)
- (WU-0C-25a, VS-002)
- (WU-0C-25, VS-002)
- (WU-0C-26a, VS-002)
- (WU-0C-26, VS-002)
- (WU-0C-36a, VS-002)
- (WU-0C-36, VS-002)
- (WU-0C-37a, VS-002)
- (WU-0C-37, VS-002)
- (WU-0C-04, VS-003)
- (WU-0C-11a, VS-003)
- (WU-0C-11b, VS-003)
- (WU-0C-12a, VS-003)
- (WU-0C-12, VS-003)
- (WU-0C-13a, VS-003)
- (WU-0C-13b, VS-003)
- (WU-0C-13, VS-003)
- (WU-0C-14a, VS-003)
- (WU-0C-14b, VS-003)
- (WU-0C-14, VS-003)
- (WU-0C-15a, VS-003)
- (WU-0C-15b, VS-003)
- (WU-0C-15c, VS-003)
- (WU-0C-15d, VS-003)
- (WU-0C-16a, VS-003)
- (WU-0C-16b, VS-003)
- (WU-0C-16, VS-003)
- (WU-0C-17a, VS-003)
- (WU-0C-17b, VS-003)
- (WU-0C-17, VS-003)
- (WU-0C-18, VS-003)
- (WU-0C-30a, VS-003)
- (WU-0C-30, VS-003)
- (WU-0C-31a, VS-003)
- (WU-0C-31, VS-003)
- (WU-0C-32, VS-003)
- (WU-0C-33a, VS-003)
- (WU-0C-33, VS-003)
- (WU-0C-34, VS-003)
- (WU-0C-35a, VS-003)
- (WU-0C-35, VS-003)
- (WU-0C-37a, VS-003)
- (WU-0C-37, VS-003)
- (WU-0C-04, VS-004)
- (WU-0C-08, VS-004)
- (WU-0C-09a, VS-004)
- (WU-0C-09, VS-004)
- (WU-0C-10, VS-004)
- (WU-0C-05, VS-004)
- (WU-0C-06a, VS-004)
- (WU-0C-06, VS-004)
- (WU-0C-07a, VS-004)
- (WU-0C-07, VS-004)
- (WU-0C-21, VS-004)
- (WU-0C-22a, VS-004)
- (WU-0C-22, VS-004)
- (WU-0C-23a, VS-004)
- (WU-0C-23b, VS-004)
- (WU-0C-23, VS-004)
- (WU-0C-24a, VS-004)
- (WU-0C-24, VS-004)
- (WU-0C-25a, VS-004)
- (WU-0C-25, VS-004)
- (WU-0C-26a, VS-004)
- (WU-0C-26, VS-004)
- (WU-0C-34, VS-004)
- (WU-0C-35a, VS-004)
- (WU-0C-35, VS-004)
- (WU-0C-36a, VS-004)
- (WU-0C-36, VS-004)
- (WU-0C-37a, VS-004)
- (WU-0C-37, VS-004)
- (WU-0C-04, VS-005)
- (WU-0C-08, VS-005)
- (WU-0C-09a, VS-005)
- (WU-0C-09, VS-005)
- (WU-0C-10, VS-005)
- (WU-0C-21, VS-005)
- (WU-0C-22a, VS-005)
- (WU-0C-22, VS-005)
- (WU-0C-23a, VS-005)
- (WU-0C-23b, VS-005)
- (WU-0C-23, VS-005)
- (WU-0C-24a, VS-005)
- (WU-0C-24, VS-005)
- (WU-0C-25a, VS-005)
- (WU-0C-25, VS-005)
- (WU-0C-26a, VS-005)
- (WU-0C-26, VS-005)
- (WU-0C-27, VS-005)
- (WU-0C-34, VS-005)
- (WU-0C-35a, VS-005)
- (WU-0C-35, VS-005)
- (WU-0C-36a, VS-005)
- (WU-0C-36, VS-005)
- (WU-0C-37a, VS-005)
- (WU-0C-37, VS-005)
- (WU-0C-04, VS-006)
- (WU-0C-21, VS-006)
- (WU-0C-22a, VS-006)
- (WU-0C-22, VS-006)
- (WU-0C-23a, VS-006)
- (WU-0C-23b, VS-006)
- (WU-0C-23, VS-006)
- (WU-0C-24a, VS-006)
- (WU-0C-24, VS-006)
- (WU-0C-25a, VS-006)
- (WU-0C-25, VS-006)
- (WU-0C-26a, VS-006)
- (WU-0C-26, VS-006)
- (WU-0C-11a, VS-006)
- (WU-0C-11b, VS-006)
- (WU-0C-12a, VS-006)
- (WU-0C-12, VS-006)
- (WU-0C-13a, VS-006)
- (WU-0C-13b, VS-006)
- (WU-0C-13, VS-006)
- (WU-0C-14a, VS-006)
- (WU-0C-14b, VS-006)
- (WU-0C-14, VS-006)
- (WU-0C-15a, VS-006)
- (WU-0C-15b, VS-006)
- (WU-0C-15c, VS-006)
- (WU-0C-15d, VS-006)
- (WU-0C-16a, VS-006)
- (WU-0C-16b, VS-006)
- (WU-0C-16, VS-006)
- (WU-0C-17a, VS-006)
- (WU-0C-17b, VS-006)
- (WU-0C-17, VS-006)
- (WU-0C-18, VS-006)
- (WU-0C-19, VS-006)
- (WU-0C-20a, VS-006)
- (WU-0C-20, VS-006)
- (WU-0C-30a, VS-006)
- (WU-0C-30, VS-006)
- (WU-0C-31a, VS-006)
- (WU-0C-31, VS-006)
- (WU-0C-32, VS-006)
- (WU-0C-33a, VS-006)
- (WU-0C-33, VS-006)
- (WU-0C-34, VS-006)
- (WU-0C-35a, VS-006)
- (WU-0C-35, VS-006)
- (WU-0C-36a, VS-006)
- (WU-0C-36, VS-006)
- (WU-0C-37a, VS-006)
- (WU-0C-37, VS-006)
- (WU-0C-21, VS-007)
- (WU-0C-22a, VS-007)
- (WU-0C-22, VS-007)
- (WU-0C-23a, VS-007)
- (WU-0C-23b, VS-007)
- (WU-0C-23, VS-007)
- (WU-0C-24a, VS-007)
- (WU-0C-24, VS-007)
- (WU-0C-25a, VS-007)
- (WU-0C-25, VS-007)
- (WU-0C-26a, VS-007)
- (WU-0C-26, VS-007)
- (WU-0C-34, VS-007)
- (WU-0C-35a, VS-007)
- (WU-0C-35, VS-007)
- (WU-0C-36a, VS-007)
- (WU-0C-36, VS-007)
- (WU-0C-37a, VS-007)
- (WU-0C-37, VS-007)

Round 3 SessionOverrideContract incoming edges, matching Phase 0C-r4 outgoing declarations:

- (WU-0C-N2, WU-1-03) canonical transcript DTO shapes for Claude tool-call normalization; no raw JSONL parsing.
- (WU-0C-N2, WU-1-04) canonical transcript DTO shapes for Codex tool-call normalization; no raw JSONL parsing.
- (WU-0C-N2, WU-1-05) canonical transcript DTO shapes for opencode tool-call normalization; no raw session storage parsing.
- (WU-0C-N1, WU-1-10), (WU-0C-N2, WU-1-10), (WU-0C-N3, WU-1-10), (WU-0C-N4, WU-1-10), (WU-0C-N5, WU-1-10) read-only transcript ingestion through SessionOverrideContract and v1 adapter/refusal evidence.
- (WU-0C-N5, WU-1-12), (WU-0C-N5, WU-1-13), (WU-0C-N5, WU-1-28) override receipt/refusal audit and drill-down display.
- (WU-0C-N2, WU-1-23), (WU-0C-N5, WU-1-23), (WU-0C-N2, WU-1-25), (WU-0C-N5, WU-1-25) override-derived evidence pointer and inspector display.
- (WU-0C-N4, WU-1-45), (WU-0C-N5, WU-1-45), (WU-0C-N4, WU-1-48), (WU-0C-N5, WU-1-48), (WU-0C-N4, WU-1-49), (WU-0C-N5, WU-1-49), (WU-0C-N4, WU-1-50), (WU-0C-N5, WU-1-50) read-only provider preflight display of adapter capability/refusal evidence.

Bidirectional consistency note: Phase 0C-r4 names WU-0C-N5 as feeding VS-001 evidence inspectors and VS-003 audit surfaces, and names WU-0C-N1..WU-0C-N5 as feeding Phase 1 worker-launcher / worker-output-reintegration successors. Phase 1 r3 wires the existing VS-001/VS-003/VS-006 read-only surfaces. The worker-launcher and worker-output-reintegration edges are intentionally carried forward as outgoing/downstream blockers because those WUs are not present in this 56-WU Phase 1 artifact.

### Outgoing To Phase 2+

- (VS-001, VS-008) consumes Phase 1 foundations: WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28
- (VS-001, VS-009) consumes Phase 1 foundations: WU-1-20, WU-1-21, WU-1-22, WU-1-23, WU-1-24, WU-1-25, WU-1-26, WU-1-27, WU-1-28
- (VS-002, VS-008) consumes Phase 1 foundations: WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36
- (VS-002, VS-010) consumes Phase 1 foundations: WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36
- (VS-002, VS-014) consumes Phase 1 foundations: WU-1-29, WU-1-30, WU-1-31, WU-1-32, WU-1-33, WU-1-34, WU-1-35, WU-1-36
- (VS-003, VS-009) consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13
- (VS-003, VS-010) consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13
- (VS-003, VS-014) consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13
- (VS-003, VS-018) consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13. **Blocked-on:** accepted worker-output session write-back must consume WU-0C-N1..WU-0C-N5; v2 adapter migration needs `agents session locate / export / import-replace`; atomic mid-session reintegration needs `agents pause-handshake`.
- (VS-003, VS-019) consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13
- (VS-003, VS-020) consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13
- (VS-004, VS-008) consumes Phase 1 foundations: WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19
- (VS-004, VS-009) consumes Phase 1 foundations: WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19
- (VS-004, VS-010) consumes Phase 1 foundations: WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19
- (VS-004, VS-015) consumes Phase 1 foundations: WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19
- (VS-004, VS-019) consumes Phase 1 foundations: WU-1-14, WU-1-15, WU-1-16, WU-1-17, WU-1-18, WU-1-19
- (VS-005, VS-010) consumes Phase 1 foundations: WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44
- (VS-005, VS-011) consumes Phase 1 foundations: WU-1-37, WU-1-38, WU-1-39, WU-1-40, WU-1-41, WU-1-42, WU-1-43, WU-1-44
- (VS-006, VS-009) consumes Phase 1 foundations: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50
- (VS-006, VS-015) consumes Phase 1 foundations: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50. **Blocked-on:** the downstream worker launcher remains thin and must spawn `agents -m <model> -p <project> -f <prompt>` while capturing the spawned `session_id` via the `agents` `--session-id` forced-flag mechanism and storing it on `WorkerRun`; v2 SessionOverrideContract adapter migration needs `agents session locate / export / import-replace`.
- (VS-006, VS-017) consumes Phase 1 foundations: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50
- (VS-006, VS-020) consumes Phase 1 foundations: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50
- (VS-006, VS-021) consumes Phase 1 foundations: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50
- (VS-007, VS-015) consumes Phase 1 foundations: WU-1-51, WU-1-52, WU-1-53, WU-1-54, WU-1-55

Engineering-roadmap dependency rows encoded here:

- VS-001, VS-002, VS-004 -> VS-008.
- VS-001, VS-003, VS-004, VS-006 -> VS-009.
- VS-002, VS-003, VS-004, VS-005 -> VS-010.
- VS-005, VS-010 -> VS-011, with this Phase 1 artifact owning only the VS-005 side.
- VS-002, VS-003, VS-010 -> VS-014, with this Phase 1 artifact owning only VS-002 and VS-003 sides.
- VS-006, VS-004, VS-009, VS-007, VS-013 -> VS-015, with this Phase 1 artifact owning VS-004, VS-006, and VS-007 sides.
- VS-015, VS-006 -> VS-017, with this Phase 1 artifact owning only the VS-006 side.
- VS-015, VS-010, VS-013, VS-003 -> VS-018, with this Phase 1 artifact owning only the VS-003 side.
- VS-010, VS-018, VS-003, VS-004 -> VS-019, with this Phase 1 artifact owning VS-003 and VS-004 sides.
- VS-003, VS-006, VS-013, VS-017, VS-018 -> VS-020, with this Phase 1 artifact owning VS-003 and VS-006 sides.
- VS-006, VS-015, VS-020 -> VS-021, with this Phase 1 artifact owning only the VS-006 side.

### Non-Ownership Notes

- Phase 1 does not redefine GraphStore tables, Phase 0B durable schema objects, or Phase 0C runtime engines.
- Phase 1 does not launch orchestrator turns, mutate topology, run optimizer edit cycles, dispatch workers, route NEEDS_INPUT continuations, sample reviewers, execute recovery, rewrite session transcripts, or reroute providers as product workflows.
- Phase 1 may emit advisory OptimizerRequest records from configuration inspection only through WU-1-42 and the Phase 0C OptimizerQueueService contract.
- Provider probes must stay redacted and must not take ownership of vendor credential stores.
- Worker-launcher WUs and worker-output-reintegration WUs are absent from this Phase 1 artifact. When downstream artifacts introduce them, launchers must not manipulate session storage paths and reintegration WUs that mutate orchestrator session content must depend on WU-0C-N1 and the active adapter (WU-0C-N3 until v2).
