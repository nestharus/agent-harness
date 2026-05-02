# Phase 0C Ticket Index

**Source artifact:** `worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md`  
**Audit history:** `worktrees/phase-0c-ai-roadmap-r5/plans/audit/ai-roadmap-phase-0c.md`  
**Engineering scope:** `worktrees/engineering-roadmap-r5/product-strategy/engineering-roadmap.md` -> Phase 0C shared engines, Option A SessionOverrideContract cascade, and landed agent-runner session commands

This directory supersedes the prior r4-derived ticket set at `worktrees/tickets-phase-0c-r2/plans/tickets/phase-0c/`. The r5 cascade replaces the 77-WU Phase 0C r2 output with 76 tickets: WU-0C-N4 is dropped, and WU-0C-N3 is rescoped from direct DB/JSONL mutation to `AgentRunnerCliAdapter` over the landed `agents session` surface.

## Work Unit Inventory

Total Phase 0C WUs: **76**.

| Group | WUs | Count |
|---|---:|---:|
| PolicyEngine and deterministic gate substrate | WU-0C-01..WU-0C-04 plus WU-0C-02a/WU-0C-04a | 6 |
| BudgetLedger runtime service and gates | WU-0C-05..WU-0C-07 plus WU-0C-06a/WU-0C-07a | 5 |
| ConfigurationRegistry runtime | WU-0C-08..WU-0C-10 plus WU-0C-09a | 4 |
| Agent-runner subprocess supervisor family | WU-0C-11a..WU-0C-18 plus split DTO WUs | 21 |
| SessionOverrideContract and CLI adapter foundation | WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5 | 4 |
| Provider monitor and diagnostics | WU-0C-19..WU-0C-20 plus WU-0C-20a | 3 |
| RenderEngine core | WU-0C-21..WU-0C-26 plus split DTO WUs | 12 |
| Optimizer shell | WU-0C-27..WU-0C-29 plus WU-0C-29a/WU-0C-29b | 5 |
| Identity/conflict shell | WU-0C-29c..WU-0C-29d | 2 |
| Recovery shell | WU-0C-30..WU-0C-31 plus WU-0C-30a/WU-0C-31a | 4 |
| Hook/plugin boundary | WU-0C-32..WU-0C-33 plus WU-0C-33a | 3 |
| Tauri IPC, UI shell wiring, audit emit pipeline | WU-0C-34..WU-0C-37 plus split DTO WUs | 7 |

| WU | Owns | Parent initiative |
|---|---|---|
| WU-0C-01 | `PolicyDecision` DTO | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-02a | `PolicyGateDescriptor` DTO | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-02 | `PolicyRegistry` | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-03 | `PolicyValidationReport` | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-04a | `PolicyGateInput` DTO | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-04 | `PolicyEngine::evaluate_gate` | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-05 | `BudgetDecision` DTO | BudgetLedger core service |
| WU-0C-06a | `BudgetUsageDraft` DTO | BudgetLedger core service |
| WU-0C-06 | `BudgetLedgerAccountingService` | BudgetLedger core service |
| WU-0C-07a | `BudgetCheckRequest` DTO | BudgetLedger core service |
| WU-0C-07 | `BudgetGateService` | BudgetLedger core service |
| WU-0C-08 | `ConfigurationRegistryRuntimeRead` | ConfigurationRegistry runtime read API |
| WU-0C-09a | `ConfigurationRevisionDraft` DTO | ConfigurationRegistry runtime write API |
| WU-0C-09 | `ConfigurationRegistryWriteApi` | ConfigurationRegistry runtime write API |
| WU-0C-10 | `ConfigurationProvenanceResolver` | ConfigurationRegistry provenance resolver |
| WU-0C-11a | `AgentSpawnRequest` DTO | CLI subprocess supervisor |
| WU-0C-11b | `OulipolyInvocationRef` DTO and parser | CLI subprocess supervisor |
| WU-0C-12a | `AgentSpawnResult` DTO | CLI subprocess supervisor |
| WU-0C-12 | `AgentSubprocessSupervisor` spawn/cancel/timeout operation | CLI subprocess supervisor |
| WU-0C-13a | `SessionCaptureRequest` DTO | CLI subprocess supervisor |
| WU-0C-13b | `AgentRunnerSessionCapture` DTO | CLI subprocess supervisor |
| WU-0C-13 | `AgentSessionCapture` | CLI subprocess supervisor |
| WU-0C-14a | `AgentRunnerTraceEdge` DTO | CLI subprocess supervisor |
| WU-0C-14b | `AgentRunnerTraceTree` DTO | CLI subprocess supervisor |
| WU-0C-14 | `AgentTraceTreeReader` | CLI subprocess supervisor |
| WU-0C-15a | `SessionTurnRef` DTO | CLI subprocess supervisor |
| WU-0C-15b | `SessionTurnsRequest` DTO | CLI subprocess supervisor |
| WU-0C-15c | `SessionTurnsRead` DTO | CLI subprocess supervisor |
| WU-0C-15d | `SessionTurnsReader` | CLI subprocess supervisor |
| WU-0C-16a | `ConfigSnapshotRequest` DTO | CLI subprocess supervisor |
| WU-0C-16b | `AgentRunnerConfigSnapshot` DTO | CLI subprocess supervisor |
| WU-0C-16 | `AgentRunnerConfigSnapshotReader` | CLI subprocess supervisor |
| WU-0C-17a | `ProviderDiagnosticsRequest` DTO | CLI subprocess supervisor and provider diagnostics |
| WU-0C-17b | `ProviderDiagnosticsResult` DTO | CLI subprocess supervisor and provider diagnostics |
| WU-0C-17 | `ProviderDiagnosticsAdapter` | CLI subprocess supervisor and provider diagnostics |
| WU-0C-18 | `AgentRunnerClient` facade | Unified agent-runner wrapper API |
| WU-0C-N2 | `TranscriptTurn`, `SessionLocation`, `SessionMetadata`, and related override DTOs | SessionOverrideContract foundation |
| WU-0C-N1 | `SessionOverrideContract` trait and TypeScript DTO surface | SessionOverrideContract foundation |
| WU-0C-N3 | `AgentRunnerCliAdapter` | SessionOverrideContract v2 implementation |
| WU-0C-N5 | `SessionOverrideStore` registry | SessionOverrideContract audit/override registry |
| WU-0C-19 | `ProviderProbeRequest` DTO | ProviderStateMonitor |
| WU-0C-20a | `ProviderStateMonitorResult` DTO | ProviderStateMonitor |
| WU-0C-20 | `ProviderStateMonitor` | ProviderStateMonitor |
| WU-0C-21 | `RenderRequestDto` | RenderEngine core |
| WU-0C-22a | `RenderPolicy` DTO | RenderEngine core |
| WU-0C-22 | `RenderPolicyResolver` | RenderEngine core |
| WU-0C-23a | `RenderPrivilegeFilterInput` DTO | RenderEngine core |
| WU-0C-23b | `RenderPrivilegeFilterResult` DTO | RenderEngine core |
| WU-0C-23 | `RenderPrivilegeFilter` | RenderEngine core |
| WU-0C-24a | `CachePrefixInput` DTO | RenderEngine core and cache locality |
| WU-0C-24 | `CachePrefixHasher` | RenderEngine core and cache locality |
| WU-0C-25a | `RenderBudgetRequest` DTO | RenderEngine plus BudgetLedger integration |
| WU-0C-25 | `RenderBudgetGateAdapter` | RenderEngine plus BudgetLedger integration |
| WU-0C-26a | `RenderResultDto` | RenderEngine core |
| WU-0C-26 | `RenderEngine::render_working_set` | RenderEngine core |
| WU-0C-27 | `OptimizerQueueService` | Optimizer queue shell |
| WU-0C-28 | `OptimizerCycleStateMachine` | Optimizer cycle shell |
| WU-0C-29a | `OptimizerScheduleDecision` DTO | Optimizer scheduler |
| WU-0C-29b | `OptimizerCycleLease` DTO | Optimizer scheduler |
| WU-0C-29 | `OptimizerScheduler` | Optimizer scheduler |
| WU-0C-29c | `IdentityResolverRuntime` | Identity/conflict shell |
| WU-0C-29d | `ConflictRecordWriterShell` | Identity/conflict shell |
| WU-0C-30a | `RecoveryActionDraft` DTO | Recovery-action writer |
| WU-0C-30 | `RecoveryActionWriter` | Recovery-action writer |
| WU-0C-31a | `RecoveryProcessorDecision` DTO | RecoveryAction processor skeleton |
| WU-0C-31 | `RecoveryActionProcessorSkeleton` | RecoveryAction processor skeleton |
| WU-0C-32 | `HookPayloadEnvelope` | Hook/MCP/plugin injection scaffold |
| WU-0C-33a | `PluginCapability` DTO | Hook/MCP/plugin capability boundary |
| WU-0C-33 | `PluginCapabilityMatrix` | Hook/MCP/plugin capability boundary |
| WU-0C-34 | `TauriIpcCommandRouter` | Tauri IPC layer scaffolding |
| WU-0C-35a | `DomainEventDraft` DTO | Channel event emission backbone |
| WU-0C-35 | `IpcEventEmissionBackbone` | Channel event emission backbone |
| WU-0C-36a | `PaneRouteBinding` DTO | UI shell wiring |
| WU-0C-36 | `UiShellPaneRouter` | UI shell wiring |
| WU-0C-37a | `AuditEmitInput` DTO | Audit event writer subscription and emit |
| WU-0C-37 | `AuditEmitPipeline` | Audit event writer subscription and emit |

Removed in r5: WU-0C-N4 (`AgentRunnerSchemaProbe`) has no ticket file. The r4 schema-probe WU existed only to protect the v1 direct `state.db` / JSONL adapter; schema compatibility is now the `agents session schema-probe` call owned by WU-0C-N3 and enforced by its construction-time `safe_for_import_replace` acceptance criterion.

## Parallelization Map

Parallel groups are derived by topological depth from the graph above. No wave contains a direct or transitive dependency edge between members. Wave counts sum to 76.

| Wave | WUs | Max concurrent worktrees | Notes |
|---:|---|---:|---|
| 1 | WU-0C-01, WU-0C-02a, WU-0C-04a, WU-0C-05, WU-0C-06a, WU-0C-07a, WU-0C-08, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-15a, WU-0C-16a, WU-0C-16b, WU-0C-17b, WU-0C-N2, WU-0C-19, WU-0C-21, WU-0C-23a, WU-0C-24a, WU-0C-30a, WU-0C-35a, WU-0C-36a, WU-0C-37a | 23 | External 0A/0B-only DTOs, primitive service DTOs, and session-override DTOs. |
| 2 | WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-10, WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-14a, WU-0C-17a, WU-0C-N1, WU-0C-20a, WU-0C-22a, WU-0C-23b, WU-0C-24, WU-0C-29a, WU-0C-30, WU-0C-35 | 18 | First derived DTOs, leaf services, and SessionOverrideContract trait. |
| 3 | WU-0C-04, WU-0C-12, WU-0C-13, WU-0C-14b, WU-0C-15b, WU-0C-16, WU-0C-N3, WU-0C-N5, WU-0C-22, WU-0C-25a, WU-0C-26a, WU-0C-31a, WU-0C-37 | 13 | Policy, agent-runner basics, CLI override adapter, override registry, render policy resolver, render budget/result DTOs, audit emit. |
| 4 | WU-0C-07, WU-0C-14, WU-0C-15c, WU-0C-23, WU-0C-29c, WU-0C-32 | 6 | Budget gate, trace reader, privilege filter, identity shell, hook payload. |
| 5 | WU-0C-15d, WU-0C-25, WU-0C-27, WU-0C-29d, WU-0C-33a | 5 | Session turns service, render budget adapter, optimizer queue, conflict writer, plugin capability DTO. |
| 6 | WU-0C-17, WU-0C-26, WU-0C-28, WU-0C-33 | 4 | Provider diagnostics, render core, optimizer cycle, plugin matrix. |
| 7 | WU-0C-18, WU-0C-29b | 2 | Agent facade and optimizer lease DTO. |
| 8 | WU-0C-20, WU-0C-29 | 2 | Provider monitor and optimizer scheduler. |
| 9 | WU-0C-31, WU-0C-34 | 2 | Recovery processor and command router join service surfaces. |
| 10 | WU-0C-36 | 1 | UI shell routing joins commands and events. |

Parallel write rule:

- Do not let same-wave WUs edit the same generated Rust contract index, TypeScript contract index, Tauri command registration module, or event-topic registry in parallel.
- If codegen produces shared indexes, land per-WU generated files independently and reserve shared-index updates for a small integration patch after the wave.
- No same-wave WU shares a migration file; WU-0C-N5 is the only round-4 WU that may require a small override-ledger persistence addition, and it is not a canonical workspace session-id table.

## Blocked Summary

No WUs blocked on agent-runner features (all landed). All prior blocked annotations from tickets-phase-0c-r2 were removed; WU-0C-N3 now directly maps to `agents session locate`, `export`, `import-replace`, `pause-handshake`, `resume-handshake`, and `schema-probe`.

## Regeneration Notes

- One ticket file exists for each active WU ID in the r5 Work Unit Inventory: 76 files plus this index.
- WU-0C-N4 is represented only by the removed-in-r5 note above.
- WU-0C-N3's contract, acceptance criteria, dependency lines, test boundary, code boundary, produces line, and parallelization line are sourced from `ai-roadmap-phase-0c.md` r5.
- r5-touched SessionOverrideContract and agent-runner tickets cite the Option A proposal-r6 / engineering-roadmap-r5 cascade and the landed agent-runner features.
- Unchanged WUs preserve the tickets-phase-0c-r2 ticket body byte-for-byte except where r5 removal of blocked annotations or active-WU references required a targeted edit.
- No commit or push is part of this ticket-regeneration work.
