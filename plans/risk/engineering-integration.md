# Engineering — Integration Risk Assessment

**Rating: LOW**

## Scope of assessment

I traced integration contracts across every declared phase boundary in `engineering-roadmap.md` (Phases 0A → 0B → 0C → 1 → 2 → 3 → 4 → 5 → 6 → 7), checked shared-schema usage across the 28 proposal data-model objects, audited the Tauri IPC and `agent-runner` substrate boundaries, and re-verified the dependency graph at lines 684-716 against the per-slice "Foundation dependencies" lines.

The roadmap's overall integration story is coherent: Phase 0 is treated as substrate (not as hidden delivery of value slices), shared engines (`RenderEngine`, `PolicyEngine`, `BudgetLedger`, `ProviderStateMonitor`, optimizer queue, identity/conflict shell) are explicitly placed in Phase 0 ahead of every consumer, the snapshot-walk-then-merge boundary is preserved across `GraphAction` (foreground) vs `OptimizerEdit` (background) usage, and the dependency graph at lines 684-716 is acyclic with every edge respecting phase ordering. Pushbacks P-1 through P-5 explicitly close the highest-risk integration seams that an empty worktree would otherwise create.

The findings below are observations about declarative completeness, not unresolved incoherences. Severity stays at LOW.

## Findings

### F-1. Phase 0B base-table description list omits ~10 proposal objects that the catch-all foundation item silently covers

**Severity: LOW (declarative-completeness only)**

`engineering-roadmap.md` line 542-543 enumerates Phase 0B base tables as: "graph snapshots, working-set snapshots, summaries, evidence, provenance, audit events, budget ledgers, provider state, capability fingerprints, policy sets, agent walk state, optimizer requests/edits, identity events, conflict records, worker runs, question artifacts, and recovery actions."

The proposal data model (`proposal.md` lines 180-921) defines 28 schema objects. The Phase 0B sentence implicitly covers ~18; missing names from the sentence include `GraphWorkspace`, `GraphConfiguration`, `GraphNode`, `NodeRevision`, `GraphEdge`, `OrchestratorTurn`, `GraphAction`, `ToolCallProvenance`, `WorkerSlice` (vs the listed `WorkerRun`), and `EntitlementSnapshot` (vs the listed `ProviderState`).

The catch-all line 24 foundation item "SQLite GraphStore migration system and base schema ownership | VS-001 through VS-021 | ... | L" is the binding declaration — it claims schema ownership for every consumer. So no schema is actually orphaned, but the sub-phase narrative diverges from the table.

This matters for integration only because some downstream slices reference these omitted schemas as if Phase 0B already produced them:

- VS-001's `WorkingSetSnapshot` carries a `turn_id` that points to `OrchestratorTurn` (proposal lines 446, 587-611), and VS-001 produces real renders before VS-009 builds the orchestrator turn lifecycle. The roadmap leaves nullable-vs-synthetic-turn-id behavior implicit.
- VS-002 / VS-003 record summaries and tool provenance against `GraphNode` and `NodeRevision`, but those names do not appear in the Phase 0B sentence.
- VS-005 reads `GraphConfiguration` revisions; the Phase 0B sentence does not name it, only the line 24 catch-all does.
- VS-009's bounded `GraphAction` allow/deny set (proposal lines 473-511, 991) has no explicit Phase 0 owner; it is implied by `PolicyEngine` (Phase 0C, line 31, 547) plus the catch-all line 24 schema item.

**Recommendation:** No action required. The dual reading (catch-all foundation item + non-exhaustive sub-phase sentence) leaves the contract complete; LOW conditions below pin this down.

---

### F-2. `agent-runner` substrate integration is declared by feature, not consolidated into one Phase 0 contract

**Severity: LOW**

The substrate boundary spans at least seven distinct integration patterns (per `engineering-research.md` lines 41-58 and the proposal's "Fixed Substrate" line 18):

1. Subprocess spawn of `agents -m <model> -p <project> -f <prompt-file>` with stdout/stderr semantics.
2. `OULIPOLY_INVOCATION` / `OULIPOLY_PARENT_INVOCATION` propagation and stderr parsing.
3. Forced `--session-id` capture for Claude vs `thread.started` parsing for Codex.
4. `agents trace --json` invocation tree consumption.
5. Read access to `~/.local/share/oulipoly-agent-runner/state.db` (`session_turns`).
6. Read access to `~/.config/oulipoly-agent-runner/providers.toml`, `models/*.toml`, and `sessions.toml`.
7. `agents resume -m <model> --session-id <sid> -f <answer.md>` for `QuestionArtifact` continuation, with child-acceptance distinct from wrapper attempt.

The roadmap declares the substrate boundary in two places:

- Foundation table line 34: "CLI subprocess supervisor around `/home/nes/.local/bin/agents` | VS-001, VS-003, VS-006, VS-009, VS-015, VS-017, VS-020, VS-021 | ... process lifecycle, prompt files, stderr invocation capture, cancellation, and child acceptance | M".
- Phase 0C line 547: "CLI subprocess supervisor around `agents`".

The "Needed by" column on line 34 does not include VS-016 (which uses `agents trace --json` per VS-016 description and engineering-research line 120) and does not separate read-only state.db / providers.toml access (used implicitly by VS-006 preflight per `engineering-research.md` line 110) from active subprocess spawn. Patterns 4, 5, 6 above are not consolidated under any single Phase 0 item.

This is not an incoherence — every individual slice description (VS-006 line 152-169, VS-009 line 211-229, VS-015 line 331-349, VS-016 line 351-369, VS-017 line 371-389, VS-020 line 431-449, VS-021 line 451-469) names the specific substrate pattern it consumes, and `engineering-research.md` lines 39-93 catalogues each pattern with evidence paths. But the integration contract for substrate is distributed across slices rather than declared once in Phase 0.

The risk this would produce — multiple slices independently inventing their own `agent-runner` reader — is mitigated by Pushback P-5 (provider/capability schema in Phase 0) and by the explicit dependency graph edges from VS-009 / VS-015 / VS-017 / VS-020 / VS-021 onto the same substrate items.

**Recommendation:** No action required at the roadmap level. Implementation-phase research/proposal work for the supervisor slice should split read-only substrate reads (state.db, providers.toml, trace --json) from active subprocess management; this is implementation-phase detail per the engineering risk rubric and not a roadmap-level redo.

---

### F-3. `OptimizerRequest` lifecycle is owned in Phase 0 but request-creation surface lands in five non-adjacent slices

**Severity: LOW**

The proposal's `OptimizerRequest.source_type` enum (proposal line 672) is `orchestrator_turn`, `worker_output`, `user_surface`, `backend_signal`, `recovery_action`. The roadmap distributes request-emission across:

- VS-009 (Phase 2): `orchestrator_turn` source.
- VS-010 (Phase 2): consumer (advisory queue drain into `OptimizerEdit`).
- VS-011 (Phase 3): `backend_signal` for configuration warnings.
- VS-014 (Phase 3): `backend_signal` for poison/quarantine triage.
- VS-018 (Phase 5): `worker_output` source (worker reintegration staging).
- VS-020 (Phase 6): `recovery_action` source.

The Phase 0 foundation item line 38 "Optimizer queue, `OptimizerRequest` / `OptimizerEdit` store, and merge-validation shell | ... | L" claims ownership of the request store. This is the right placement — the schema and queue arrive before any source/sink wires up.

The integration concern: each emitter populates `request_type` from a different subset of the enum (`consider_summary_refresh`, `consider_stale_mark`, `consider_cross_reference`, `consider_repack`, `consider_split`, `consider_merge`, `consider_reparent`, `consider_provenance_repair`, `consider_quarantine`, `consider_configuration_warning`, `consider_provider_route_warning`). The Phase 0 store must accommodate the full taxonomy at creation time so VS-014 / VS-018 / VS-020 do not require a schema migration. The roadmap does not explicitly state this, but the foundation item description includes "merge-validation shell", implying full-taxonomy validation up front.

**Recommendation:** No action required.

---

### F-4. Reviewer sampling (VS-019) does not declare a recovery-flow consumer edge to VS-020

**Severity: LOW**

The proposal lists reviewer sampling triggers (proposal lines 1198-1207) including: "anomaly-triggered cases, such as missing evidence, high uncertainty, repeated failed resumes, or protocol mismatches" and "recovery anomalies, such as ... recovery that proposes changing the execution contract". VS-019 (Phase 6 position 1) ships before VS-020 (Phase 6 position 2). The dependency graph at line 710 lists `VS-010, VS-018, VS-003, VS-004 -> VS-019` (no edge from recovery), and line 712 lists `VS-003, VS-006, VS-013, VS-017, VS-018 -> VS-020` (no edge to reviewer).

So at strict edge-graph reading, recovery never consumes the reviewer sampler. In practice, both slices share `PolicySet`, `BudgetLedger`, and `AuditEvent` (Phase 6 description line 604: "They share `PolicySet`, `BudgetLedger`, `AuditEvent`, and UI flags; recovery may trigger reviewer sampling. Keep one policy owner.").

The roadmap acknowledges the integration in Phase 6 prose but not in the dependency graph. This is consistent with the rubric's exclusion of detailed contract signatures, and the prose pin ("Keep one policy owner.") is a coherent owner-assignment.

**Recommendation:** No action required.

---

### F-5. `ConfigurationRegistry` is consumed by VS-001's RenderEngine but built in VS-005

**Severity: LOW**

The proposal places `ConfigurationRegistry` as a top-level component (proposal lines 162-167). `WorkingSetSnapshot` carries `configuration_id` and `configuration_explanation_ref` (proposal lines 451, 462). `RenderEngine` "applies working-set policy, summary contract validation, configuration defaults, privilege labels, cost constraints, provider state, and capability fingerprints" (proposal line 124).

Phase 0C builds RenderEngine core interfaces (line 547). VS-001 (Phase 1, ordered third within Phase 1 per line 553-559: VS-003 → VS-004 → VS-001) ships before VS-005 (Phase 1, fourth). VS-005 is the slice that builds `ConfigurationRegistry`, the empty-graph simulator, the source-attribution map, and the shape-explanation surface.

So between VS-001 acceptance and VS-005 completion, the renderer either reads `GraphConfiguration` rows directly (without registry abstraction), records `configuration_id` without resolving its full effective-value source map, or the Phase 0C "RenderEngine core" includes a minimal config reader that VS-005 later upgrades.

The roadmap does not declare which of those three approaches applies. However, the integration is not incoherent: `GraphConfiguration` schema is in Phase 0B (covered by line 24 catch-all foundation item), the registry's role is specifically *inspection* (configuration provenance for the user), and the renderer can produce valid `WorkingSetSnapshot` rows with `configuration_id` set without the registry's empty-graph simulation being live yet.

**Recommendation:** No action required. The slice that is operator-visible (VS-005's inspector + simulator) is correctly distinguished from the schema (Phase 0B) and the read path (RenderEngine core, Phase 0C).

---

### F-6. Phase 0A → 0B → 0C → 1 sub-phase ordering is compositionally coherent

**Severity: NONE (positive finding)**

Cross-checking the operator's Phase 0 sub-phase coherence requirement explicitly:

- **0A produces the right host for 0B.** Phase 0A delivers Tauri/Turbo/React shell, Rust/Tokio backend skeleton, app state, settings, logging, IPC/event streams, UI shell, test harness, and fake `agents` (line 539 + foundation table lines 25, 26, 36, 44, 45). Phase 0B's migration infrastructure and base GraphStore tables (line 542-543 + foundation table line 27) need exactly that backend skeleton plus SQLite access to land. No 0B item depends on a 0C engine.
- **0C plugs correctly into 0A's shell with 0B's tables.** Phase 0C's RenderEngine core, PolicyEngine, CLI supervisor, provider probe adapters, hook/MCP/plugin abstraction, optimizer queue shell, identity/conflict shell, and seeded UI panes (line 547 + foundation table lines 30, 31, 32, 33, 34, 35, 38, 39, 41, 42, 43) every reference Phase 0B schema (e.g., RenderEngine reads `GraphSnapshot`, `WorkingSetSnapshot`, `SummaryContract`, `GraphConfiguration`; PolicyEngine reads `PolicySet`, `BudgetLedger`, `ProviderState`; optimizer queue reads `OptimizerRequest` / `OptimizerEdit` rows; identity/conflict shell reads `IdentityEvent` / `ConflictRecord`).
- **0C engines have stable shapes by Phase 1's start.** Every Phase 1 slice's "Foundation dependencies" list (lines 67, 87, 107, 127, 147, 167, 187) names a subset of {Tauri scaffold, GraphStore, RenderEngine core, PolicyEngine, BudgetLedger core, ProviderState/CapabilityFingerprint records, evidence/audit, CLI supervisor, hook/MCP/plugin scaffold, IPC, UI shell, fixtures}, all delivered by 0A/0B/0C.

The 18-axis coverage and 10-subsystem-plus-6-cross-cutting-concerns mapping referenced by the operator carries through: schema objects with multi-phase consumers (`WorkingSetSnapshot` in VS-001/VS-008/VS-009/VS-015/VS-017/VS-020, `OptimizerEdit` in VS-010/VS-011/VS-012/VS-013/VS-014/VS-018/VS-019, `WorkerRun` in VS-015/VS-016/VS-017/VS-018/VS-019/VS-020/VS-021, `RecoveryAction` per Pushback P-4 in VS-017/VS-020/VS-021, `ProviderState`/`CapabilityFingerprint` per Pushback P-5 in VS-001/VS-006/VS-015/VS-016/VS-017/VS-020/VS-021, `ConflictRecord` in VS-013/VS-018/VS-020) all carry the same identity model, lifecycle, and access semantics across the phases that touch them, because the schema is owned in Phase 0B and accessed through Phase 0C engines.

**Recommendation:** No action required.

---

### F-7. Tauri IPC boundary is declared at the phase level

**Severity: NONE (positive finding)**

The proposal's `UserSurface` (lines 174-178, 1155-1172) lists 11 graph-addressed panes: Initiative map, Current focus, Working set inspector, Configuration inspector, Provider state panel, Question queue, Worker board, Optimizer log, Evidence drill-down, Cost surface, Recovery surface.

The roadmap establishes the Tauri IPC boundary in Phase 0A ("IPC/event streams", line 539), Phase 0C ("seeded UI panes", line 547), and foundation table line 36 ("Tauri IPC commands and Channel event stream layer | VS-001, VS-005, VS-006, VS-007, VS-010, VS-016, VS-017, VS-020, VS-021"). The "Needed by" list on line 36 omits some operator-visible slices (VS-002, VS-003, VS-004, VS-008, VS-009, VS-011, VS-012, VS-013, VS-014, VS-015, VS-018, VS-019), but that omission reads as the "everyone needs the IPC layer" being implicit in the substrate rather than a contract gap. Each Phase 1+ slice's Foundation dependencies line names "IPC" or "IPC/UI shell" where IPC is consumed.

Phase 1 surfaces exactly the inspectors that Phase 0C's seeded panes are placeholders for (VS-001 working-set inspector → working-set pane; VS-005 configuration inspector → configuration pane; VS-006 provider preflight panel → provider pane; VS-007 initiative roots → initiative map). The contract at the phase boundary is coherent: Phase 0C produces seeded shells with stable IPC command names; Phase 1 fills in the engine outputs those commands return.

**Recommendation:** No action required.

---

## Summary table

| ID  | Finding                                                                                                            | Severity |
| --- | ------------------------------------------------------------------------------------------------------------------ | -------- |
| F-1 | Phase 0B description list omits ~10 proposal objects but the line 24 catch-all schema item covers them              | LOW      |
| F-2 | `agent-runner` substrate integration is per-slice rather than consolidated in Phase 0                              | LOW      |
| F-3 | `OptimizerRequest` request-creation surface lands in five non-adjacent slices over a Phase 0 store                 | LOW      |
| F-4 | VS-019 reviewer-sampling integration with VS-020 recovery is acknowledged in prose, not as a graph edge            | LOW      |
| F-5 | `ConfigurationRegistry` is consumed by VS-001 RenderEngine but built in VS-005 within Phase 1                      | LOW      |
| F-6 | Phase 0A → 0B → 0C → 1 sub-phase ordering is compositionally coherent                                              | NONE     |
| F-7 | Tauri IPC boundary is declared at the phase level with seeded panes plugging stable command shapes                 | NONE     |

## What LOW requires

These conditions must hold for the LOW rating to remain valid. They are concrete and verifiable against the artifact.

1. **Phase 0B base-table set is exhaustive.** When implementation begins, the Phase 0B migration set covers all 28 proposal data-model objects, including `GraphWorkspace`, `GraphConfiguration`, `GraphNode`, `NodeRevision`, `GraphEdge`, `OrchestratorTurn`, `GraphAction`, `ToolCallProvenance`, `WorkerSlice`, and `EntitlementSnapshot` even though they are not named in the line 542-543 sentence. The line 24 foundation table item "SQLite GraphStore migration system and base schema ownership | VS-001 through VS-021" is the binding declaration.

2. **`OptimizerRequest.request_type` taxonomy is fully accommodated up front.** The Phase 0 optimizer queue / request store accepts every `request_type` value the proposal defines (line 676), so VS-011, VS-014, VS-018, and VS-020 do not require schema migrations to add their request types.

3. **`RecoveryAction` schema and side-effect taxonomy land in Phase 0 per Pushback P-4.** VS-017 (Phase 5) writes durable `failed_resume` handoff records into the schema before VS-020 (Phase 6) ships the operator-visible recovery surface.

4. **`ProviderState`, `EntitlementSnapshot`, and `CapabilityFingerprint` schemas land in Phase 0 per Pushback P-5.** VS-001's render route display, VS-004's budget interpretation, VS-009's orchestrator launch, VS-015's worker dispatch, and VS-017's continuation read from the same schema VS-006 later inspects.

5. **VS-013 conflict / identity mechanics land before VS-012 topology edits per Pushback P-3.** The dependency-graph edge `VS-008, VS-010, VS-013 -> VS-012` (line 698) is preserved at implementation time.

6. **VS-001 acceptance lands behind VS-003 evidence/audit and VS-004 budget per Pushback P-2.** The `WorkingSetSnapshot` rows VS-001 produces include real `evidence_pointer_ids`, `token_estimate`, `cache_prefix_hash`, `provider_state_id`, and `configuration_explanation_ref` fields rather than placeholders that later require migration.

7. **Phase 0C engine interfaces are stable by Phase 1 start.** RenderEngine core, PolicyEngine gate framework, CLI supervisor, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, and identity/conflict shell publish stable shapes (DTOs, event names, command names) before VS-001/VS-002/VS-003/VS-004 begin acceptance work.

8. **Tauri IPC command shapes for each user-surface pane are seeded in Phase 0C.** Each of the 11 panes from `proposal.md` lines 1155-1171 has a placeholder command + Channel event name in Phase 0C, so Phase 1+ slices fill engine outputs into stable command shapes rather than minting new commands per slice.

9. **`agent-runner` substrate access is wrapped by named Phase 0 components.** Subprocess spawn (CLI supervisor), trace consumption (`agents trace --json`), state.db reads (`session_turns`, exhausted accounts), config reads (providers.toml, models/*.toml, sessions.toml), and resume client (`agents resume ... -f answer.md`) each have a single owner so VS-009 / VS-015 / VS-016 / VS-017 / VS-020 / VS-021 read substrate through the same boundary.

10. **The dependency graph at lines 684-714 stays acyclic and respects phase ordering.** Every edge in the graph goes from an earlier-phase slice (or earlier-position same-phase slice) to a later one; no slice in Phase N depends on a slice in Phase N+K.
