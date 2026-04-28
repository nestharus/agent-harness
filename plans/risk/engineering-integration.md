# Engineering — Integration Risk Assessment

**Rating: LOW**

## Scope of assessment

I traced integration contracts across every declared phase boundary in `engineering-roadmap.md` (Phases 0A → 0B → 0C → 1 → 2 → 3 → 4 → 5 → 6 → 7), checked shared-schema usage across the 28 proposal data-model objects, audited the Tauri IPC and `agent-runner` substrate boundaries, and verified the dependency graph at lines 753-787 against the per-slice "Foundation dependencies" lines.

The roadmap's overall integration story is coherent. Phase 0 is treated as substrate (not as hidden delivery of value slices), shared engines (`RenderEngine`, `PolicyEngine`, `BudgetLedger`, `ProviderStateMonitor`, optimizer queue, identity/conflict shell, recovery-action writer) are explicitly placed in Phase 0 ahead of every consumer, the snapshot-walk-then-merge boundary is preserved across `GraphAction` (foreground) vs `OptimizerEdit` (background) usage, and the dependency graph at lines 753-787 is acyclic with every edge respecting phase ordering. Pushbacks P-1 through P-5 (lines 480-548) explicitly close the highest-risk integration seams that an empty worktree would otherwise create. Phase 0C's "Substrate Integration Contract" subsection (lines 594-604) and "Cross-slice contract: `OptimizerRequest` emission" subsection (lines 606-616) consolidate the two integration surfaces with the highest cross-slice fanout.

The findings below are observations about declarative completeness and minor coupling that the roadmap addresses through prose or pushbacks, not unresolved incoherences. Severity stays at LOW.

## Findings

### F-1. Phase 0B base-table list (lines 558-588) and the foundation table (line 27) are dual descriptions of the same Phase 0 schema ownership

**Severity: LOW**

`engineering-roadmap.md` line 558-588 enumerates Phase 0B durable objects as a 27-bullet list including `GraphWorkspace`, `GraphConfiguration`, `GraphNode`, `GraphEdge`, `NodeRevision`, `GraphSnapshot`, `SummaryContract`, `ProvenancePointer`, `EvidenceArtifact`, `ToolCallProvenance`, `IdentityEvent`, `WorkingSetSnapshot`, `GraphAction`, `AgentWalkState`, `OrchestratorTurn`, `OptimizerRequest`, `OptimizerEdit`, `ConflictRecord`, `BudgetLedger`, `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, `WorkerSlice`, `WorkerRun`, `QuestionArtifact`, `RecoveryAction`, `AuditEvent`, `PolicySet`, and `ConfigurationRegistry`.

The foundation table line 27 ("SQLite GraphStore migration system and base schema ownership | VS-001 through VS-021 | ... | L") makes the same commitment in catch-all form. The two descriptions agree: every proposal schema object is owned in Phase 0B.

The 28th proposal object — `ConfigurationRegistry` — is named in the Phase 0B bullet list (line 588) and again as foundation table line 32 ("`ConfigurationRegistry` skeleton and read API over `GraphConfiguration`"). It is technically not a SQLite table; it is the read API over the `GraphConfiguration` rows. The roadmap correctly distinguishes the schema (line 588 lists the registry alongside data-model bullets) from the read path (line 32) without contradiction.

Downstream slices reference these schemas as if Phase 0B already produced them, which is consistent with the dual ownership declaration:

- VS-001's `WorkingSetSnapshot` (proposal lines 438-471) carries `turn_id`, `provider_state_id`, `capability_fingerprint_id`, and `configuration_explanation_ref`. All four target schemas are in Phase 0B.
- VS-002 / VS-003 record summaries and tool provenance against `GraphNode`, `NodeRevision`, `EvidenceArtifact`, `ProvenancePointer`, `ToolCallProvenance` — all owned in Phase 0B.
- VS-009's bounded `GraphAction` allow/deny set (proposal lines 473-511) is owned by the Phase 0C `PolicyEngine` (line 31, line 592), reading the Phase 0B `GraphAction` schema.

**Recommendation:** No action required.

---

### F-2. `agent-runner` substrate integration is consolidated by Phase 0C's "Substrate Integration Contract" subsection (lines 594-604)

**Severity: NONE (positive finding)**

The substrate boundary spans seven distinct integration patterns (per `engineering-research.md` lines 41-58). Phase 0C lines 594-604 declare the unified contract:

- Subprocess spawn semantics for `/home/nes/.local/bin/agents`, including project directory, model selection, prompt-file invocation, stdout/stderr capture, cancellation, timeout, and child-acceptance accounting.
- `OULIPOLY_INVOCATION` and `OULIPOLY_PARENT_INVOCATION` propagation, including stderr parsing and trace correlation.
- Session-id capture per CLI: Claude `--session-id`, Codex `thread_id` / thread-start output, opencode session-row mapping when available.
- Trace stitching through `agents trace --json`, treated as evidence/substrate rather than canonical graph truth.
- Read-only ingestion from `session_turns`, with explicit handling of missing transcript locators and opencode gaps.
- Awareness of `providers.toml`, `sessions.toml`, model configuration, and agents configuration files, with redaction and no harness ownership of vendor credentials.
- "A single wrapper API for active subprocess management, passive trace/state/config reads, provider diagnostics, resume attempts, and test fakes, so VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021 do not invent separate agent-runner readers."

The foundation table line 35 ("CLI subprocess supervisor around `/home/nes/.local/bin/agents`") names a subset of consumers. The "Needed by" list on line 35 does include VS-016 (which uses `agents trace --json`), so the prior round's concern about VS-016 omission is resolved. Patterns 4-6 above (trace consumption, state.db reads, config reads) are now covered by the explicit "passive trace/state/config reads" wording in the line 604 wrapper API description.

**Recommendation:** No action required.

---

### F-3. `OptimizerRequest` lifecycle is owned in Phase 0 with the full taxonomy frozen before any emitter ships (lines 606-616)

**Severity: NONE (positive finding)**

The proposal's `OptimizerRequest.source_type` enum (proposal line 672) is `orchestrator_turn`, `worker_output`, `user_surface`, `backend_signal`, `recovery_action`. The roadmap's Phase 0C "Cross-slice contract: `OptimizerRequest` emission" subsection (lines 606-616) explicitly declares:

- `orchestrator_turn` is emitted by VS-009 (Phase 2).
- `worker_output` is emitted by VS-018 (Phase 5).
- `user_surface` is emitted by VS-005 (Phase 1) and VS-011 (Phase 3).
- `backend_signal` is emitted by VS-010 (Phase 2).
- `recovery_action` is emitted by VS-020 (Phase 6).

Line 616 commits to the contract guarantee: "stable fields, stable enum values, advisory-only semantics, no direct graph mutation by the emitter, and uniform validation gates across emitters. Later slices may add emission sites only by using this contract, not by adding source-specific queue variants."

The foundation table line 39 ("Optimizer queue, `OptimizerRequest` / `OptimizerEdit` store, and merge-validation shell | VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-018, VS-019, VS-020 | ... | L") binds the Phase 0 store ownership. The full `request_type` taxonomy (proposal line 676: 11 enum values) needs to land in Phase 0B's `OptimizerRequest` migration so VS-011, VS-014, VS-018, and VS-020 do not require a schema migration to add their request types.

**Recommendation:** No action required.

---

### F-4. Reviewer sampling (VS-019) → recovery (VS-020) edge is acknowledged in prose and as a directed event edge in the dependency graph (line 781)

**Severity: LOW**

The proposal lists reviewer sampling triggers (proposal lines 1198-1207) including "anomaly-triggered cases, such as missing evidence, high uncertainty, repeated failed resumes, or protocol mismatches" and "provider anomalies, such as repeated route denials, provider feature mismatches, or recovery that proposes changing the execution contract."

VS-019 (Phase 6 position 1) ships before VS-020 (Phase 6 position 2). The dependency graph at lines 779-781 lists:

- `VS-010, VS-018, VS-003, VS-004 -> VS-019` (line 779).
- `VS-020 recovery-anomaly events -> VS-019 reviewer sampling triggers` (line 781).
- `VS-003, VS-006, VS-013, VS-017, VS-018 -> VS-020` (line 783).

Phase 6 prose at lines 670-673 explains the resolution: "VS-019 can ship its core sampler first, then subscribe to VS-020 recovery-anomaly events once the recovery surface emits them; this adds the explicit `VS-020 -> VS-019` event edge without changing the Phase 6 sequence." Line 787 confirms: "VS-019's core sampler can ship before VS-020, then activate recovery-anomaly sampling once VS-020 emits the events. The only executive-order change is moving VS-013 before VS-012 inside Phase 3."

VS-019's foundation dependencies at line 432 explicitly include "RecoveryAction event contract" — i.e., the contract is owned in Phase 0 (foundation table line 43, "`RecoveryAction` schema, side-effect classification taxonomy, and audit linkage") so VS-019 can declare the subscription without VS-020 being live.

The integration contract is therefore: VS-019 v1 ships consuming `OptimizerEdit` reviewer-sampling triggers; VS-020 ships emitting recovery-anomaly events to the same Phase 0 contract; VS-019 v1.1 activates the second subscription. This is coherent.

**Recommendation:** No action required.

---

### F-5. `ConfigurationRegistry` skeleton/read API is in Phase 0C; VS-005 builds the operator-visible inspector

**Severity: LOW**

The proposal places `ConfigurationRegistry` as a top-level component (proposal lines 162-167). `WorkingSetSnapshot` carries `configuration_id` and `configuration_explanation_ref` (proposal lines 446, 462). `RenderEngine` "applies working-set policy, summary contract validation, configuration defaults, ... configuration explanation" (proposal line 124).

The roadmap separates the read path from the operator-visible surface:

- Foundation table line 32: "`ConfigurationRegistry` skeleton and read API over `GraphConfiguration` | VS-001, VS-002, VS-004, VS-005, VS-009, VS-010, VS-011, VS-015, VS-020". The skeleton + read API is in Phase 0.
- VS-005 description line 148: "It does not create the registry itself; Phase 0 owns the registry skeleton and read path."
- VS-005 description line 150: "Foundation dependencies: GraphStore, `ConfigurationRegistry` skeleton/read API, PolicyEngine, RenderEngine core, audit writer, IPC/UI shell, test fixtures."

So between VS-001 acceptance and VS-005 acceptance, VS-001's renderer reads `GraphConfiguration` rows through the Phase 0 registry read API and records `configuration_id` plus a basic `configuration_explanation_ref` against that read path. VS-005 then layers the empty-graph simulator, source-attribution UI, and shape-explanation surface on top of the same read API. No schema rewrite or DTO migration is required.

The Phase 1 engineering ordering (lines 622-628) places VS-001 ahead of VS-005:

```
1. VS-003 backend evidence/audit spine and VS-004 budget/cache ledger primitives.
2. VS-001 working-set render inspection, using the real evidence/budget/provider/config fields.
3. VS-002 summary contract validation and invalid/stale render labels.
4. VS-005 configuration inspector, warning emission, advisory optimizer-request creation, and empty-graph simulation over the Phase 0 registry read API.
5. VS-006 provider preflight panel and capability fingerprints over the Phase 0 provider contracts.
6. VS-007 initiative roots and current focus UI.
```

Line 629 closes the integration boundary: "The `ConfigurationRegistry` skeleton/read API and provider-state contracts are already Phase 0 substrate; VS-005 and VS-006 deliver the operator-visible surfaces, warning emission, and panel behavior."

**Recommendation:** No action required.

---

### F-6. Phase 0A → 0B → 0C → 1 sub-phase ordering is compositionally coherent

**Severity: NONE (positive finding)**

Cross-checking the operator's Phase 0 sub-phase coherence requirement explicitly:

- **0A (line 552-554) produces the right host for 0B.** Phase 0A delivers Tauri/Turbo/React shell, Rust/Tokio backend skeleton, app state, settings, logging, IPC/event streams, single-tab UI shell, test harness, fake `agents`, basic CI commands. This matches foundation table lines 25, 26, 37, 44, 45, 46. No 0B item depends on a 0C engine.
- **0B (line 556-588) needs 0A's host and produces the canonical schema.** The Phase 0B description line 558 says "Implement repository boundaries and fixture builders, but keep features inert unless a value slice turns them on." 0B only needs the SQLite access from 0A's "Rust/Tokio harness backend skeleton ... local storage roots" (line 26).
- **0C (line 590-616) plugs correctly into 0A's shell with 0B's schema.** Phase 0C builds RenderEngine core interfaces, PolicyEngine gate framework, BudgetLedger core service, `ConfigurationRegistry` skeleton/read API, CLI subprocess supervisor, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, identity/conflict shell, recovery-action writer, Tauri IPC commands/Channel event streams, and seeded UI panes. Every engine references Phase 0B schema (RenderEngine reads `GraphSnapshot`, `WorkingSetSnapshot`, `SummaryContract`, `GraphConfiguration`; PolicyEngine reads `PolicySet`, `BudgetLedger`, `ProviderState`; optimizer queue reads `OptimizerRequest` / `OptimizerEdit` rows; identity/conflict shell reads `IdentityEvent` / `ConflictRecord`; recovery-action writer reads `RecoveryAction`).
- **0C engines have stable shapes by Phase 1's start.** Every Phase 1 slice's "Foundation dependencies" line (lines 68, 90, 110, 130, 150, 170, 190) names a subset of {Tauri scaffold, GraphStore, RenderEngine core, PolicyEngine, BudgetLedger core, ProviderState/CapabilityFingerprint records, evidence/audit, CLI supervisor, hook/MCP/plugin scaffold, IPC, UI shell, fixtures}, all delivered by 0A/0B/0C.

Schema objects with multi-phase consumers carry the same identity model, lifecycle, and access semantics across the phases that touch them, because the schema is owned in Phase 0B and accessed through Phase 0C engines:

- `WorkingSetSnapshot`: VS-001 (Phase 1), VS-008/VS-009 (Phase 2), VS-015 (Phase 4), VS-017/VS-020 (Phase 5/6).
- `OptimizerEdit`: VS-010 (Phase 2), VS-011/VS-012/VS-013/VS-014 (Phase 3), VS-018 (Phase 5), VS-019 (Phase 6).
- `WorkerRun`: VS-015 (Phase 4), VS-016/VS-017/VS-018 (Phase 5), VS-019/VS-020 (Phase 6), VS-021 (Phase 7).
- `RecoveryAction` (per Pushback P-4): VS-017 (Phase 5), VS-020 (Phase 6), VS-021 (Phase 7).
- `ProviderState` / `CapabilityFingerprint` (per Pushback P-5): VS-001/VS-006 (Phase 1), VS-015 (Phase 4), VS-016/VS-017 (Phase 5), VS-020/VS-021 (Phase 6/7).
- `ConflictRecord`: VS-013 (Phase 3), VS-018 (Phase 5), VS-020 (Phase 6).

**Recommendation:** No action required.

---

### F-7. Tauri IPC boundary is declared at the phase level with seeded panes plugging stable command shapes

**Severity: NONE (positive finding)**

The proposal's `UserSurface` (proposal lines 174-178, 1155-1172) lists 11 graph-addressed panes: Initiative map, Current focus, Working set inspector, Configuration inspector, Provider state panel, Question queue, Worker board, Optimizer log, Evidence drill-down, Cost surface, Recovery surface.

The roadmap establishes the Tauri IPC boundary in three places:

- Phase 0A description line 554: "Tauri IPC/event streams, frontend stack scaffolding, single-tab UI shell".
- Phase 0C description line 592: "Tauri IPC commands/Channel event streams, and seeded UI panes."
- Foundation table line 37: "Tauri IPC commands and Channel event stream layer | VS-001, VS-005, VS-006, VS-007, VS-010, VS-016, VS-017, VS-020, VS-021 | Pattern only from agent-runner `invoke` / `Channel` | M".

Phase 1 surfaces exactly the inspectors that Phase 0C's seeded panes are placeholders for (VS-001 working-set inspector → working-set pane; VS-005 configuration inspector → configuration pane; VS-006 provider preflight panel → provider state panel; VS-007 initiative roots → initiative map). The contract at the phase boundary is coherent: Phase 0C produces seeded shells with stable IPC command names; Phase 1 fills in the engine outputs those commands return. Each Phase 2-7 slice's Foundation dependencies line names "IPC" or "IPC/UI shell" where IPC is consumed (e.g., VS-009 line 230, VS-015 line 350, VS-016 line 370, VS-017 line 392, VS-020 line 452, VS-021 line 472).

**Recommendation:** No action required.

---

### F-8. Pushbacks P-1 through P-5 close the highest-fanout integration seams

**Severity: NONE (positive finding)**

The five applied pushbacks (lines 480-548) align engineering ordering with cross-slice integration contracts:

- **P-1 (lines 482-492)** keeps Phase 0 as XL substrate so Phase 1 slices do not invent seven incompatible first drafts of `RenderEngine`, `PolicyEngine`, `GraphStore`, `BudgetLedger`, provider state, and UI shell.
- **P-2 (lines 494-506)** keeps VS-001 first user-visible but blocks acceptance until VS-003 evidence/audit and VS-004 budget/cache primitives produce real `evidence_pointer_ids`, `token_estimate`, `cache_prefix_hash`, `provider_state_id`, and `configuration_explanation_ref` fields. This avoids a `WorkingSetSnapshot` schema migration after VS-001 ships.
- **P-3 (lines 508-520)** moves VS-013 conflict / identity mechanics before VS-012 topology edits inside Phase 3 so topology edits can fail closed against a real conflict state machine.
- **P-4 (lines 522-534)** lands the `RecoveryAction` schema, side-effect taxonomy, and audit linkage in Phase 0, with a failed-resume handoff path required before VS-017 acceptance. Without this, VS-017 would write ad hoc error states that VS-020 would later need to migrate.
- **P-5 (lines 536-548)** lands `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, and the denial-reason taxonomy in Phase 0 so VS-001 render route display, VS-004 budget interpretation, VS-009 orchestrator launch, VS-015 worker dispatch, VS-017 continuation, VS-020 recovery, and VS-021 reroute share one schema rather than retrofitting after VS-006 ships its panel.

Together, P-2, P-4, and P-5 prevent the three most likely "phase assumes a different system" failure modes for an empty worktree. P-3 prevents the topology vs identity ordering hazard. P-1 prevents seven Phase 1 prototypes from each inventing incompatible substrate.

**Recommendation:** No action required.

---

## Summary table

| ID  | Finding                                                                                                            | Severity |
| --- | ------------------------------------------------------------------------------------------------------------------ | -------- |
| F-1 | Phase 0B schema list (lines 558-588) and foundation table line 27 are dual descriptions of the same Phase 0 ownership | LOW      |
| F-2 | `agent-runner` substrate integration is consolidated under Phase 0C's "Substrate Integration Contract" subsection (lines 594-604) | NONE     |
| F-3 | `OptimizerRequest` taxonomy is frozen in Phase 0 with all five `source_type` emitters declared (lines 606-616)     | NONE     |
| F-4 | VS-019 reviewer-sampling integration with VS-020 recovery is declared as a dependency edge (line 781) and Phase 6 prose (lines 670-673) | LOW      |
| F-5 | `ConfigurationRegistry` skeleton/read API in Phase 0C; VS-001 reads via Phase 0; VS-005 builds operator-visible inspector | LOW      |
| F-6 | Phase 0A → 0B → 0C → 1 sub-phase ordering is compositionally coherent                                              | NONE     |
| F-7 | Tauri IPC boundary is declared at the phase level with seeded panes plugging stable command shapes                 | NONE     |
| F-8 | Pushbacks P-1 through P-5 close the highest-fanout integration seams                                               | NONE     |

## What LOW requires

These conditions must hold for the LOW rating to remain valid. They are concrete and verifiable against the artifact.

1. **Phase 0B base-table set is exhaustive.** The Phase 0B migration set covers all 28 proposal data-model objects named in lines 558-588 (including `GraphWorkspace`, `GraphConfiguration`, `GraphNode`, `NodeRevision`, `GraphEdge`, `OrchestratorTurn`, `GraphAction`, `ToolCallProvenance`, `WorkerSlice`, `EntitlementSnapshot`). The foundation table line 27 ("SQLite GraphStore migration system and base schema ownership | VS-001 through VS-021") is the binding declaration.

2. **`OptimizerRequest.request_type` taxonomy is fully accommodated up front.** The Phase 0 optimizer queue / request store accepts every `request_type` value the proposal defines (line 676: 11 values), so VS-009, VS-010, VS-011, VS-014, VS-018, and VS-020 do not require schema migrations to add their request types. The Phase 0C cross-slice contract guarantee at line 616 ("stable fields, stable enum values, advisory-only semantics, no direct graph mutation by the emitter") is upheld.

3. **`RecoveryAction` schema and side-effect taxonomy land in Phase 0 per Pushback P-4.** VS-017 (Phase 5) writes durable `failed_resume` handoff records into the Phase 0 schema before VS-020 (Phase 6) ships the operator-visible recovery surface. VS-017 acceptance prerequisite (line 392) is enforced.

4. **`ProviderState`, `EntitlementSnapshot`, and `CapabilityFingerprint` schemas land in Phase 0 per Pushback P-5.** VS-001's render route display, VS-004's budget interpretation, VS-009's orchestrator launch, VS-015's worker dispatch, VS-017's continuation, VS-020's recovery, and VS-021's reroute all read from the same schema VS-006 later populates with redacted probes.

5. **VS-013 conflict / identity mechanics land before VS-012 topology edits per Pushback P-3.** The dependency graph edge `VS-008, VS-010, VS-013 -> VS-012` (line 767) is preserved at implementation time. VS-012 topology mutation does not merge before VS-013's identity resolver and conflict state machine exist.

6. **VS-001 acceptance lands behind VS-003 evidence/audit and VS-004 budget per Pushback P-2.** The `WorkingSetSnapshot` rows VS-001 produces include real `evidence_pointer_ids`, `token_estimate`, `cache_prefix_hash`, `provider_state_id`, and `configuration_explanation_ref` fields rather than placeholders that later require migration. The Phase 1 engineering order at lines 622-628 (VS-003+VS-004 backends → VS-001 → VS-002 → VS-005 → VS-006 → VS-007) is preserved.

7. **Phase 0C engine interfaces are stable by Phase 1 start.** RenderEngine core, PolicyEngine gate framework, CLI supervisor, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, identity/conflict shell, and recovery-action writer publish stable shapes (DTOs, event names, command names) before VS-001/VS-002/VS-003/VS-004 begin acceptance work.

8. **Tauri IPC command shapes for each user-surface pane are seeded in Phase 0C.** Each of the 11 panes from `proposal.md` lines 1155-1171 has a placeholder command + Channel event name in Phase 0C ("seeded UI panes" at line 592), so Phase 1+ slices fill engine outputs into stable command shapes rather than minting new commands per slice.

9. **`agent-runner` substrate access is wrapped by the unified Phase 0C wrapper API (line 604).** Subprocess spawn (CLI supervisor), trace consumption (`agents trace --json`), state.db reads (`session_turns`, exhausted accounts), config reads (providers.toml, models/*.toml, sessions.toml), and resume client (`agents resume ... -f answer.md`) each have a single owner so VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021 read substrate through the same boundary rather than inventing separate readers.

10. **The dependency graph at lines 753-787 stays acyclic and respects phase ordering.** Every edge in the graph goes from an earlier-phase slice (or earlier-position same-phase slice) to a later one; no slice in Phase N depends on a slice in Phase N+K. The `VS-020 -> VS-019` recovery-anomaly event edge inside Phase 6 (line 781) is the only same-phase reverse edge and is acceptable because VS-019 v1 ships subscribing-only and activates when VS-020 emits.

11. **Snapshot-walk-then-merge boundary is preserved across all phases.** Foreground `GraphAction` (proposal lines 473-511) only records evidence/tool/audit/output/advisory-request effects in Phase 1+ slices. `OptimizerEdit` (proposal lines 691-718) is the only mutation path for summaries, topology, identity, provenance repair, and quarantine, owned by Phase 2+ optimizer slices. No phase blurs this boundary.
