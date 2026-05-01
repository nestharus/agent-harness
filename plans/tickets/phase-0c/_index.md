# Phase 0C Ticket Index

**Source artifact:** `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md`  
**Audit history:** `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md`  
**Engineering scope:** `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared engines and SessionOverrideContract cascade

This directory supersedes the prior r3-derived ticket set at `worktrees/tickets-phase-0c/plans/tickets/phase-0c/`. The r4 cascade replaces the 72-ticket Phase 0C output with 77 tickets, adding WU-0C-N1..WU-0C-N5 and refactoring the affected agent-runner/session WUs around `SessionOverrideContract`.

## Work Unit Inventory

Total Phase 0C WUs: **77**.

| Group | WUs | Count |
|---|---:|---:|
| PolicyEngine and deterministic gate substrate | WU-0C-01..WU-0C-04 plus WU-0C-02a/WU-0C-04a | 6 |
| BudgetLedger runtime service and gates | WU-0C-05..WU-0C-07 plus WU-0C-06a/WU-0C-07a | 5 |
| ConfigurationRegistry runtime | WU-0C-08..WU-0C-10 plus WU-0C-09a | 4 |
| Agent-runner subprocess supervisor family | WU-0C-11a..WU-0C-18 plus split DTO WUs | 21 |
| SessionOverrideContract and v1 adapter foundation | WU-0C-N1..WU-0C-N5 | 5 |
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
| WU-0C-N4 | `AgentRunnerSchemaProbe` | SessionOverrideContract v1 compatibility probe |
| WU-0C-N3 | `AgentRunnerDbAdapter` v1 | SessionOverrideContract v1 adapter |
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

## Parallelization Map

Parallel groups are derived by topological depth from the graph above. No wave contains a direct or transitive dependency edge between members.

| Wave | WUs | Max concurrent worktrees | Notes |
|---:|---|---:|---|
| 1 | WU-0C-01, WU-0C-02a, WU-0C-04a, WU-0C-05, WU-0C-06a, WU-0C-07a, WU-0C-08, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-15a, WU-0C-16a, WU-0C-16b, WU-0C-17b, WU-0C-N2, WU-0C-19, WU-0C-21, WU-0C-23a, WU-0C-24a, WU-0C-30a, WU-0C-35a, WU-0C-36a, WU-0C-37a | 23 | External 0A/0B-only DTOs, primitive service DTOs, and session-override DTOs. |
| 2 | WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-10, WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-14a, WU-0C-17a, WU-0C-N1, WU-0C-N4, WU-0C-20a, WU-0C-22a, WU-0C-23b, WU-0C-24, WU-0C-29a, WU-0C-30, WU-0C-35 | 19 | First derived DTOs, leaf services, SessionOverrideContract trait, and schema probe. |
| 3 | WU-0C-04, WU-0C-12, WU-0C-13, WU-0C-14b, WU-0C-15b, WU-0C-16, WU-0C-N5, WU-0C-22, WU-0C-25a, WU-0C-26a, WU-0C-31a, WU-0C-37 | 12 | Policy, agent-runner basics, override registry, render policy resolver, render budget/result DTOs, audit emit. |
| 4 | WU-0C-07, WU-0C-14, WU-0C-15c, WU-0C-N3, WU-0C-23, WU-0C-29c, WU-0C-32 | 7 | Budget gate, trace reader, session override v1 adapter, privilege filter, identity shell, hook payload. |
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

## Blocked-on Annotation Summary

Per-WU annotations copied from the Phase 0C r4 artifact:

- WU-0C-N2: **Blocked-on:** None for the DTO contract. v2 population fields are placeholders until `agents session locate/export/import-replace`, pause-handshake, and schema-version probe land.
- WU-0C-N1: **Blocked-on:** None for v1 trait acceptance. v2 adapter migration is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe.
- WU-0C-N4: **Blocked-on:** v1 ships with local table/binary probing. Replacement by an upstream supported-surface probe is blocked on `agents schema-version probe` or equivalent.
- WU-0C-N3: **Blocked-on:** v1 ships now under schema-version pinning and idle-only writes. v2 swap-later is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe. Atomic mid-session override remains blocked on `agents pause-handshake`.
- WU-0C-N5: **Blocked-on:** None for the v1 registry. v2 adapter-kind activation is blocked on `agents session locate/export/import-replace` and schema-version probe.

Value-slice Stitch Notes annotations copied from the Phase 0C r4 artifact:

- VS-010: **Blocked-on:** v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-010 uses WU-0C-N3 v1 idle-only write-back.
- VS-012: **Blocked-on:** v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-012 uses WU-0C-N3 v1 idle-only replacement.
- VS-018: **Blocked-on:** v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-018 may stage accepted output and uses WU-0C-N3 only when session-idle.
- VS-020: **Blocked-on:** recovery actions that need live mid-session replacement require `agents pause-handshake`; v2 replacement requires `agents session import-replace`.
- VS-021: **Blocked-on:** v2 reroute/write-back migration requires `agents session locate/export/import-replace` and schema-version probe.

## Regeneration Notes

- One ticket file exists for each WU ID in the r4 Work Unit Inventory.
- Acceptance criteria, contract blocks, dependency lines, test boundaries, code boundaries, produces lines, parallelization lines, and blocked-on annotations are sourced from `ai-roadmap-phase-0c.md`.
- Per-ticket dependency categories use the union of each WU's local `Dependencies` line and the r4 Detailed Phase 0C edges so Round 4 ledger edges such as WU-0C-31 -> WU-0C-N5 are visible.
- No commit or push is part of this ticket-regeneration work.
