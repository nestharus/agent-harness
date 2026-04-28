# Engineering Roadmap

## Technical Landscape Summary

`agent-harness` is currently a strategy-only worktree. `product-strategy/engineering-research.md` reports no harness application code: no `src-tauri/`, `src/`, `apps/`, `packages/`, `migrations/`, `tests/`, daemon, API service, or infrastructure directories exist. The only implementation-adjacent substrate is external: `/home/nes/.local/bin/agents` from `agent-runner`, plus the source and local configuration under `/home/nes/projects/agent-runner/` and `~/.config/oulipoly-agent-runner/`.

The reusable substrate is meaningful but bounded. `agent-runner` already provides multi-provider invocation, balancing, `--resume`, trace trees through `OULIPOLY_INVOCATION` / `OULIPOLY_PARENT_INVOCATION`, session ingestion into `session_turns`, provider diagnostics, quota windows, and session-id capture for Claude and Codex. The research also identifies useful upstream Tauri patterns: `tauriInvoke` / `Channel` wrappers, `#[tauri::command]`, app-state guarded backend commands, and subprocess composition. Those are patterns and external services, not harness code.

The harness itself must be built from scratch around the fixed substrate in `proposal.md`: Tauri v2, Bun, Turbo, React 19, TanStack Router/Query, Tailwind v4, Vitest, Playwright, Lefthook, Changesets, Commitlint, Rust, Tokio, and SQLite. The proposal's local SQLite data model spans `GraphWorkspace`, `GraphConfiguration`, `GraphNode`, `NodeRevision`, `GraphSnapshot`, `GraphEdge`, `IdentityEvent`, `SummaryContract`, `EvidenceArtifact`, `ProvenancePointer`, `WorkingSetSnapshot`, `GraphAction`, `AgentWalkState`, `WorkerSlice`, `WorkerRun`, `OrchestratorTurn`, `QuestionArtifact`, `ToolCallProvenance`, `OptimizerRequest`, `OptimizerEdit`, `ConflictRecord`, `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, `BudgetLedger`, `RecoveryAction`, `PolicySet`, and `AuditEvent`. None of those tables, migrations, repositories, commands, UI panes, or test fixtures exist yet.

The central engineering implication is that Phase 0 is not a small bootstrap. It must establish enough storage, render, policy, IPC, agent-runner integration, evidence, and test substrate that Phase 1 slices can be implemented as value slices instead of seven incompatible first drafts of the same core. This does not contradict the executive roadmap: the executive roadmap explicitly left Phase 0 to engineering and named local state plus no in-product compaction as pervasive constraints.

No `engineering-surfaces.md` was written. I found no problem surface under the operator's binary test because there is no existing harness code pattern that contradicts the proposal. I found no philosophy surface because the ordering tradeoffs are covered by existing principles, especially `P1`, `P3`, `P6`, `P7`, `P8`, `P12`, `P13`, `P14`, `P15`, and `P16`.

## Foundation Phase

### Phase 0: Technical Foundations

**Justification:** The shared infrastructure table in `engineering-research.md` shows that every executive slice depends on absent harness substrate. Phase 0 should include only non-operator-visible foundations required by two or more slices: the desktop/backend skeleton, graph store and migrations, render/policy interfaces, evidence and audit primitives, subprocess integration, IPC, test fixtures, and shared state machines. It should not include operator-visible inspectors, worker launch controls, optimizer curation features, reviewer sampling, recovery execution, or graph-topology mutation capabilities beyond schemas and validator boundaries needed by downstream slices.

**Effort scale:** `S` is less than one week; `M` is one to two weeks; `L` is two to four weeks; `XL` is four or more weeks.

| Foundation item | Needed by | Currently exists? | Effort |
|---|---|---:|---:|
| Tauri/Turbo/React monorepo scaffold with fixed frontend/tooling substrate | VS-001 through VS-021 | No; research says no `src-tauri/`, `apps/`, `packages/`, or package metadata exists, while `proposal.md` fixes the stack | M |
| Rust/Tokio harness backend skeleton, app state, settings, local storage roots | VS-001 through VS-021 | No; only agent-runner has analogous Tauri/Rust patterns | M |
| SQLite GraphStore migration system and base schema ownership | VS-001 through VS-021 | No; research says no `migrations/` and proposal schema spans lines 180-920 | L |
| Evidence blob store and provenance locator interfaces | VS-001, VS-002, VS-003, VS-010, VS-014, VS-016, VS-018, VS-019, VS-020 | No; agent-runner transcript locators are optional evidence sources, not harness provenance | M |
| Append-only audit writer and `AuditEvent` base table | VS-001 through VS-021, directly called out for VS-003, VS-006, VS-010, VS-020 | No | M |
| RenderEngine core interfaces and snapshot DTOs | VS-001, VS-002, VS-004, VS-005, VS-006, VS-007, VS-008, VS-009, VS-014, VS-015, VS-017, VS-020 | No; research names `RenderEngine` as absent and cross-cutting | L |
| PolicyEngine deterministic gate framework with versioned decisions | VS-002, VS-003, VS-004, VS-006, VS-008, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-015, VS-017, VS-018, VS-019, VS-020, VS-021 | No; research warns individual validators would conflict | L |
| `ConfigurationRegistry` skeleton and read API over `GraphConfiguration` | VS-001, VS-002, VS-004, VS-005, VS-009, VS-010, VS-011, VS-015, VS-020 | No; proposal names it as a top-level component, but the operator-visible inspector remains VS-005 | M |
| BudgetLedger core tables and token/cache accounting interfaces | VS-001, VS-004, VS-008, VS-009, VS-010, VS-015, VS-016, VS-019, VS-020, VS-021 | No; agent-runner quota state is provider telemetry only | M |
| ProviderStateMonitor, `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, and denial-reason taxonomy | VS-001, VS-006, VS-015, VS-016, VS-017, VS-020, VS-021 | Partial external substrate only; agent-runner has provider config, quota, resume config, diagnostics | M |
| CLI subprocess supervisor around `/home/nes/.local/bin/agents` | VS-001, VS-003, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, VS-021 | Partial external substrate only; harness still needs process lifecycle, prompt files, env-var propagation, session-id capture, trace stitching, cancellation, and child acceptance | M |
| Hook/MCP/plugin injection scaffold and capability boundary abstraction | VS-001, VS-003, VS-006, VS-008, VS-009, VS-015, VS-017, VS-018 | No; research says no harness MCP or hook/plugin code exists | L |
| Tauri IPC commands and Channel event stream layer | VS-001, VS-005, VS-006, VS-007, VS-010, VS-016, VS-017, VS-020, VS-021 | Pattern only from agent-runner `invoke` / `Channel` | M |
| `AgentWalkState` schema and navigation state service shell | VS-001, VS-007, VS-008, VS-009, VS-012, VS-013 | No | M |
| Optimizer queue, `OptimizerRequest` / `OptimizerEdit` store, and merge-validation shell | VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-018, VS-019, VS-020 | No | L |
| Identity resolver and `ConflictRecord` base workflow | VS-012, VS-013, VS-017, VS-018, VS-020, VS-021 | No | L |
| WorkerSlice / WorkerRun schema and state contract, without launch UI | VS-015, VS-016, VS-017, VS-018, VS-019, VS-020, VS-021 | Partial external invocation substrate only | M |
| `QuestionArtifact` schema and continuation-envelope contract, without full queue UI | VS-017, VS-020, VS-021 | Partial external resume substrate only | M |
| `RecoveryAction` schema, side-effect classification taxonomy, and audit linkage | VS-003, VS-006, VS-013, VS-017, VS-018, VS-020, VS-021 | No | M |
| Single-tab UI shell, route/layout primitives, seeded graph-addressed panes | VS-001, VS-005, VS-006, VS-007, VS-010, VS-016, VS-017, VS-019, VS-020, VS-021 | No | M |
| Test harness and fixtures: temp SQLite DBs, fake `agents`, provider configs, transcript samples, UI seeds | VS-001 through VS-021 | No; research says no tests exist and `sqlite3` CLI cannot be assumed | L |
| Logging/tracing infrastructure for backend spans, subprocess correlation, and UI event streams | VS-001, VS-003, VS-006, VS-009, VS-015, VS-017, VS-020, VS-021 | Partial external trace substrate only | M |

**Total foundation effort:** XL, roughly 12-20 weeks serial equivalent. The serial sum is larger than a normal bootstrap because the worktree is empty and the value slices share storage, render, policy, IPC, and test surfaces. Best-case foundation parallelization is possible if ownership is explicit: one lane for desktop/tooling, one for SQLite schema/migrations, one for subprocess/provider fixtures, one for UI shell, and one for tests. The dependency graph is acyclic if Phase 0 is treated as substrate, not as hidden delivery of value slices.

## Initiative Assessments

### VS-001: Inspect Imposed Working-Set Renders (Phase 1)

**Executive priority:** Phase 1, position 1.

**Effort:** L after Phase 0; XL if the RenderEngine, graph snapshot DTOs, budget ledger fields, provider fingerprint records, and IPC shell are not built in Phase 0.

**Effort reasoning:** `engineering-research.md` says this slice has no runnable harness code and is missing the graph SQLite schema, render pipeline, token estimator, cache prefix hash, provider/capability snapshot, working-set inspector UI, and audit writes. It can reuse agent-runner invocation IDs, trace, provider selection, configured models, and persistent state, plus the Tauri invoke/Channel pattern from agent-runner. This is not a small file change; it introduces the first graph render and the first "what did the model see?" UI surface.

**Risk:** High.

**Risk factors:** New-technology risk from the imposed-context render engine with no surveyed exact precedent (`problem.md` §15); provider-specific risk because Claude, Codex, and opencode need different render shapes; cache-locality risk because `prefix_hash` and token estimates affect correctness (`P12`); substrate risk because it depends on agent-runner invocation and trace semantics.

**What exists:** Planning artifacts define `WorkingSetSnapshot`, `GraphSnapshot`, `RenderEngine`, `BudgetLedger`, and the UI inspector. agent-runner provides invocation and trace evidence.

**What is new:** The actual renderer, snapshot store, render blob storage, token/cache estimator, IPC commands, inspector pane, and audit integration.

**Foundation dependencies:** Tauri scaffold, GraphStore, evidence/provenance interfaces, RenderEngine core, BudgetLedger core, `ProviderState` / `EntitlementSnapshot` / `CapabilityFingerprint` records, IPC, UI shell, fake `agents`, logging.

**Acceptance prerequisite:** VS-001 remains the first operator-visible Phase 1 target, but it is not accepted until VS-003's evidence/audit backbone and VS-004's budget/cache primitives are real enough for `WorkingSetSnapshot` rows to include evidence pointers, token estimates, cache-prefix hashes, provider-state references, configuration-explanation references, and audit links without later migration.

**Parallelizable with:** Partial with VS-002, VS-003, VS-004, VS-005, VS-006, and VS-007 only after stable schema and DTO boundaries. It shares `RenderEngine`, `WorkingSetSnapshot`, evidence pointers, budget fields, capability fingerprints, configuration attribution, and UI shell.

### VS-002: Enforce Summary Contracts on Visible Nodes (Phase 1)

**Executive priority:** Phase 1, position 2.

**Effort:** M.

**Effort reasoning:** Research says no code directly implements contracts. The slice needs contract schema, validators, summary template registry, invalid render labels, evidence locator validation, and unpack affordances. It follows the proposal-defined `SummaryContract` object and PolicyEngine gate pattern, so after Phase 0 it is a bounded validation/render-label slice rather than a full optimizer.

**Risk:** Medium.

**Risk factors:** Contract correctness risk because missing evidence must invalidate summaries rather than emit fluent prose; integration risk with VS-003 evidence/provenance IDs; privilege and poison state must survive renders; schema migration risk is lower if Phase 0 owns base tables.

**What exists:** `SummaryContract` fields and validation states in proposal; roadmap states `valid`, `invalid_missing_evidence`, `invalid_conflict`, `invalid_stale`, `invalid_policy`, `needs_review`.

**What is new:** Validators, template-source handling, evidence locator checks, UI invalid/stale labels, and render blocking/labeling behavior.

**Foundation dependencies:** GraphStore, evidence/provenance interfaces, RenderEngine core, PolicyEngine, UI shell, test fixtures.

**Parallelizable with:** Yes with VS-003 if `EvidenceArtifact` and `ProvenancePointer` contracts are fixed first. Partial with VS-001, VS-004, VS-005, and VS-007 because they share render labels, configuration sources, and node-status UI.

### VS-003: Capture Tool-Call Provenance and Audit Events (Phase 1)

**Executive priority:** Phase 1, position 3.

**Effort:** L.

**Effort reasoning:** Research says this slice needs a cross-CLI tool event normalizer, evidence blob store, audit table, protocol correlation keys, side-effect classifier, and approval-state capture. It can reuse agent-runner trace, transcript locators, `OULIPOLY_INVOCATION`, and parent invocation propagation. The missing work crosses storage, subprocess ingestion, policy, and UI drill-down.

**Risk:** High.

**Risk factors:** Provider/protocol-specific risk because Claude, Codex, and opencode tool shapes are asymmetric; opencode lacks configured local turn ingestion; recovery risk because orphaned and deferred tool calls must be classified correctly; substrate risk from agent-runner transcript and trace semantics.

**What exists:** Proposal objects `ToolCallProvenance`, `EvidenceArtifact`, `GraphAction`, and `AuditEvent`; agent-runner trace and transcript locator substrate.

**What is new:** Normalized tool-call lifecycle capture, side-effect classification, approval state, evidence blobs, audit writer, and drill-down UI.

**Foundation dependencies:** Evidence store, audit writer, CLI supervisor, hook/MCP/plugin scaffold, PolicyEngine, IPC, test transcript fixtures.

**Parallelizable with:** Mostly yes with VS-004, VS-005, VS-006, and VS-007 once the audit writer API is stable. Partial with VS-001 because render provenance pointers and evidence drill-down share UI.

### VS-004: Gate Renders with Budget Ledgers and Cache Locality (Phase 1)

**Executive priority:** Phase 1, position 4.

**Effort:** M.

**Effort reasoning:** Research says this needs harness budget ledger schema, token/cost estimator, cache-prefix hash, policy gates for render/worker/optimizer/reviewer, and cost UI. It can reuse agent-runner quota windows and provider exhaustion state, but those are explicitly not the full budget truth. After Phase 0, this is a ledger/gate/UI slice with clear downstream consumers.

**Risk:** Medium.

**Risk factors:** External dependency risk from provider pricing, quota windows, and prompt-cache TTL behavior; integration risk because budget gates touch renders, workers, optimizer, reviewers, and recovery; cost-as-correctness risk if estimates are wrong enough to allow runaway behavior.

**What exists:** Proposal defines `BudgetLedger`; agent-runner records quota and exhausted-account telemetry.

**What is new:** Per-scope budget records, token/cache estimation, policy transitions, cost surface, blocked render handling.

**Foundation dependencies:** GraphStore, BudgetLedger core, ProviderState records, PolicyEngine, RenderEngine core, IPC/UI shell, provider fixtures.

**Parallelizable with:** Mostly yes with VS-003 and VS-007. Partial with VS-001, VS-002, VS-005, and VS-006 because render result states, configuration caps, provider quota interpretation, and UI badges overlap.

### VS-005: Inspect Configuration as Memory Semantics (Phase 1)

**Executive priority:** Phase 1, position 5.

**Effort:** M.

**Effort reasoning:** Research says this needs effective value source tracking, empty-graph simulator, shape explanation, configuration validation UI, warning emission, advisory optimizer-request creation, and audit events. The Phase 0 `ConfigurationRegistry` skeleton and read API already exist so VS-001's RenderEngine can consume configuration from day one; VS-005 turns that substrate into the operator-visible inspection and warning surface. The agent-runner model/provider config locations can inform only the provider/model subset.

**Risk:** Medium.

**Risk factors:** Configuration semantics risk because defaults and user-configured values change graph truth; integration risk with RenderEngine, SummaryContract templates, ProviderState policy, and BudgetLedger caps; UX risk because the inspection must be operational state, not help text.

**What exists:** Proposal defines `GraphConfiguration`, effective value sources, and inspection lifecycle.

**What is new:** Operator-visible configuration inspector, default/inherited/source maps, empty-graph simulation, shape explanation, configuration-warning emission, advisory `OptimizerRequest` creation for user-surface configuration concerns, and inspector-pane audit events. It does not create the registry itself; Phase 0 owns the registry skeleton and read path.

**Foundation dependencies:** GraphStore, `ConfigurationRegistry` skeleton/read API, PolicyEngine, RenderEngine core, audit writer, IPC/UI shell, test fixtures.

**Parallelizable with:** Mostly yes with VS-003. Partial with VS-001, VS-002, VS-004, VS-006, and VS-007 because configuration fields affect render attribution, summary templates, budget caps, provider routing, and focus labels.

### VS-006: Preflight Providers and Expose Capability Fingerprints (Phase 1)

**Executive priority:** Phase 1, position 6.

**Effort:** M.

**Effort reasoning:** Research says agent-runner provides provider configs, quota scripts, auth refresh delegation, session capture/resume configs, diagnostics, and model pools. Missing harness work is `ProviderStateMonitor`, redacted probes, entitlement snapshots, capability registry, route denial reasons, and UI badges. This is a new service with strong substrate analogues.

**Risk:** Medium.

**Risk factors:** Provider-specific risk across auth, billing, quota, entitlement, runtime, sandbox, network, and resume features; external dependency risk from Anthropic/OpenAI/z.ai limits; substrate risk because agent-runner provider state may evolve; security risk around redaction and not storing secrets.

**What exists:** agent-runner provider configuration, quota and diagnostic substrate; Phase 0 owns proposal-defined `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, and route-denial taxonomy contracts.

**What is new:** Redacted probes over the Phase 0 provider contracts, entitlement observations, capability matrix population, route eligibility logic, operator-visible denial reasons, and provider panel.

**Foundation dependencies:** ProviderStateMonitor records, `ProviderState` / `EntitlementSnapshot` / `CapabilityFingerprint` schemas, denial-reason taxonomy, CLI supervisor, audit writer, PolicyEngine, IPC/UI shell, fake provider probes.

**Parallelizable with:** Mostly yes with VS-002, VS-003, and VS-007. Partial with VS-001, VS-004, and VS-005 because render route display, provider cost interpretation, and provider-routing defaults share state.

### VS-007: Show Initiative Roots and Current Focus (Phase 1)

**Executive priority:** Phase 1, position 7.

**Effort:** M.

**Effort reasoning:** Research says the target stack and agent-runner IPC pattern are the only reusable pieces. The missing pieces are graph root model, focus path service, single-tab UI shell, notification classes, and graph status subscriptions. After Phase 0, this is mostly a user-surface slice over existing graph and walk-state records.

**Risk:** Medium.

**Risk factors:** Integration risk with `GraphNode` lifecycle states, `AgentWalkState`, `WorkingSetSnapshot`, optimizer stale markers, worker states, and recovery states; UX risk because `P13` requires action-needed vs passive progress separation; lower technical novelty than render/optimizer/concurrency slices.

**What exists:** Proposal-defined user surface regions and `AgentWalkState`; no code.

**What is new:** Initiative/focus UI, status selectors, subscriptions, notification classification, and seeded fixtures.

**Foundation dependencies:** UI shell, GraphStore, `AgentWalkState`, IPC, audit writer, seeded SQLite fixtures.

**Parallelizable with:** Mostly yes with VS-003, VS-004, and VS-006. Partial with VS-001, VS-002, and VS-005 because the UI shell, node status labels, configuration warnings, and working-set focus panes overlap.

### VS-008: Navigate with Pack, Unpack, Focus, Pin, and Unpin (Phase 2)

**Executive priority:** Phase 2, position 1.

**Effort:** M.

**Effort reasoning:** Research says agent-runner can launch/resume the orchestrator but has no graph navigation tools. The slice needs a tool API exposed to Claude, walk-state persistence, policy validation, next-turn render integration, and audit events. It follows established `AgentWalkState` and `RenderEngine` boundaries from Phase 0 / Phase 1 and does not mutate topology.

**Risk:** Medium.

**Risk factors:** Integration risk with RenderEngine, `AgentWalkState`, budget denial, conflict denial, and tool-call provenance; provider-specific risk in how Claude tool affordances are exposed; correctness risk around preserving `P5` agent-owned focus without crossing into optimizer-owned curation.

**What exists:** Proposal-defined pack/unpack/focus/pin/unpin tool states and `AgentWalkState`.

**What is new:** Tool command schema, policy enforcement, state transitions, next-render invalidation, and tool affordance tests.

**Foundation dependencies:** `AgentWalkState`, RenderEngine, PolicyEngine, BudgetLedger, CLI injection scaffold, audit writer, test fixtures.

**Parallelizable with:** Partial with VS-009 because the tools run inside orchestrator turns. Mostly yes with VS-010 if render invalidation and stale labels are coordinated.

### VS-009: Run Bounded Orchestrator Turns with Advisory Optimizer Requests (Phase 2)

**Executive priority:** Phase 2, position 2.

**Effort:** L.

**Effort reasoning:** Research says agent-runner supplies `agents -m`, project working directory, prompt file invocation, top-level resume, and trace. Missing harness work includes `OrchestratorBridge`, subprocess supervisor, turn state machine, capture/commit transaction, advisory queue, and optimizer request UI. It is a multi-integration slice across renderer, provenance, budget, provider preflight, optimizer queue, and no-compact discipline.

**Risk:** High.

**Risk factors:** Substrate risk from `--resume`, trace, `OULIPOLY_INVOCATION`, and session-id capture semantics; provider-specific risk because Claude resume and Codex rollout semantics differ; concurrency risk because foreground `GraphAction` must remain narrow while optimizer requests queue; recovery risk around blocked/failed turns.

**What exists:** agent-runner invocation/resume/trace; proposal-defined `OrchestratorTurn`, `GraphAction`, `OptimizerRequest`, and lifecycle.

**What is new:** Durable turn orchestration, capture/commit transactions, advisory request queueing, `/compact` detection/avoidance, and UI state.

**Foundation dependencies:** CLI supervisor, RenderEngine, PolicyEngine, BudgetLedger, ProviderStateMonitor, evidence/audit, optimizer queue, IPC, fake `agents`.

**Parallelizable with:** Partial with VS-008 and VS-010 because they share turn/tool state, `OptimizerRequest`, and audit events.

### VS-010: Refresh Summaries and Mark Stale Nodes (Phase 2)

**Executive priority:** Phase 2, position 3.

**Effort:** L.

**Effort reasoning:** Research says `glm` is configured and agent-runner can invoke it, but the harness lacks optimizer queue, scoped snapshot rendering, edit drafts, deterministic validators, merge attempts, and optimizer log UI. This is the first slice where background optimizer edits touch graph state the orchestrator walks, even if only for summary/stale edit types.

**Risk:** High.

**Risk factors:** Concurrency risk from snapshot-walk-then-merge; new-technology risk because live optimizer curation over imposed graph state has no surveyed prior art; external dependency risk from z.ai / `glm` availability and cost; cache-locality risk if summary regeneration churns stable prefixes.

**What exists:** Proposal-defined `OptimizerRequest`, `OptimizerEdit`, optimizer cycle, and summary regeneration limitations; configured `glm` model.

**What is new:** Optimizer scoping, prompt/response schema, edit validation, merge attempts, conflict-on-stale-base behavior, summary/stale UI.

**Foundation dependencies:** Optimizer queue/edit store, RenderEngine, SummaryContract validators, evidence/provenance, BudgetLedger, PolicyEngine, ConflictRecord base, IPC/UI shell.

**Parallelizable with:** Mostly yes with VS-008 if render invalidation boundaries are fixed. Partial with VS-009 because orchestrator turns enqueue advisory requests consumed by the optimizer.

### VS-011: Simulate Configuration Effects and Request Shape Repair (Phase 3)

**Executive priority:** Phase 3, position 1.

**Effort:** M.

**Effort reasoning:** Research says only agent-runner config can inform provider/model parts. Missing work is deeper config anomaly detection, shape-repair request generation, and optimizer request linkage for repairs beyond VS-005's operator-visible warning emission. It builds directly on VS-005 configuration inspection and VS-010 optimizer request handling.

**Risk:** Medium.

**Risk factors:** Configuration semantics risk because warnings must not silently rewrite graph truth; integration risk with `OptimizerRequest` taxonomy, audit events, and PolicyEngine; lower provider risk except where provider-routing warnings are generated.

**What exists:** `GraphConfiguration` inspection lifecycle and warning-to-advisory-request design.

**What is new:** Expanded warning categories, anomaly detectors, shape-repair request flow over the Phase 0 `OptimizerRequest` contract, and UI linkage from simulated configuration effects to optimizer-owned repair consideration.

**Foundation dependencies:** ConfigurationRegistry, OptimizerRequest store, PolicyEngine, audit writer, UI shell.

**Parallelizable with:** Partial with VS-013 and VS-012 because warnings may become conflicts or topology edits. Mostly yes with VS-014 if PolicyEngine ownership is coordinated.

### VS-013: Resolve Snapshot-Merge Conflicts and Identity Forwarding (Phase 3)

**Executive priority:** Phase 3, position 3 in the executive sequence; engineering order position 2, before VS-012.

**Effort:** L.

**Effort reasoning:** Research says agent-runner trace can identify session/invocation relationships but not graph conflicts. Missing work is identity resolver, optimistic merge engine, conflict table, resolution state machine, and conflict UI. This is required by topology edits, worker reintegration, question routes, and recovery.

**Risk:** High.

**Risk factors:** Concurrency risk from optimizer-vs-agent races on shared graph state; new-technology risk from snapshot-walk-then-merge semantics; schema risk around identity maps and conflict records; recovery risk because conflicts become recovery inputs.

**What exists:** Proposal-defined `ConflictRecord`, `IdentityEvent`, snapshot semantics, and fail-closed recovery references.

**What is new:** Merge precondition checks, stale-base conflict detection, forwarding-map resolver, conflict state machine, and user/orchestrator conflict surfaces.

**Foundation dependencies:** GraphStore, Identity resolver base, ConflictRecord base, OptimizerEdit store, PolicyEngine, audit writer, concurrency fixtures.

**Parallelizable with:** Partial with VS-011 and VS-014. No with VS-012 unless one team owns the merge/identity API and the other builds only clients against it.

### VS-012: Repack Hierarchy and Discover Cross-References (Phase 3)

**Executive priority:** Phase 3, position 2 in the executive sequence; engineering order position 3, after VS-013's conflict/identity mechanics are in place.

**Effort:** L.

**Effort reasoning:** Research says there is no graph-topology substrate; missing work includes cross-reference discovery, repack/split/merge/reparent operations, containment edge updates, identity forwarding, and conflict creation. This is a significant schema and merge-behavior slice with no code analogue in the current repo.

**Risk:** High.

**Risk factors:** New-technology risk from unbounded hierarchical packing without bounded-depth precedent; concurrency risk from topology edits against snapshots; identity risk across split/merge/reparent; policy risk if cross-references or repacks traverse poisoned or unresolved nodes.

**What exists:** Proposal-defined `GraphEdge`, `IdentityEvent`, `OptimizerEdit` edit types, and recursive unpack bounds.

**What is new:** Topology edit operations, cross-reference discovery, repack planning, identity forwarding application, conflict generation, and UI inspection.

**Foundation dependencies:** GraphStore, OptimizerEdit store, Identity resolver, ConflictRecord workflow, PolicyEngine, evidence/provenance, topology invariant tests.

**Parallelizable with:** Not safely parallelizable with VS-013 without tight coordination. VS-012 design and fixture work may begin while VS-013 is underway, but topology mutation should not merge before the VS-013 identity resolver and conflict state machine exist. Partial with VS-011 and VS-014 due shared `OptimizerEdit`, `GraphEdge`, conflict, render traversal, and policy surfaces.

### VS-014: Quarantine Poisoned or Privilege-Unsafe Graph Content (Phase 3)

**Executive priority:** Phase 3, position 4.

**Effort:** L.

**Effort reasoning:** Research says raw CLI transcripts and tool outputs can become evidence inputs, but the harness lacks privilege labeling, quarantine policy, render traversal blocks, poison-risk validators, and UI quarantine labels. It touches SummaryContract, RenderEngine, PolicyEngine, OptimizerEdit, and evidence.

**Risk:** High.

**Risk factors:** Security and prompt-injection risk; privilege-origin preservation risk; integration risk with optimizer edits, render traversal, and reviewer sampling; recovery risk if quarantine state must be reverted or audited.

**What exists:** Proposal-defined privilege origin, trust state, poison risk, privilege controls, deterministic gates.

**What is new:** Quarantine edit type behavior, privilege transforms, render blocks, validator fixtures, and drill-down UI.

**Foundation dependencies:** Evidence/provenance, SummaryContract validators, PolicyEngine, RenderEngine, OptimizerEdit store, audit writer, UI shell.

**Parallelizable with:** Mostly yes with VS-011 if gate APIs are stable. Partial with VS-012 and VS-013 because quarantine can block traversal, conflict with topology edits, and feed recovery hooks.

### VS-015: Dispatch Provider-Aware Worker Slices (Phase 4)

**Executive priority:** Phase 4, only slice.

**Effort:** L.

**Effort reasoning:** Research says agent-runner can invoke configured models, load balance, capture invocation UUIDs, resume sessions, and trace children. Missing harness work includes `WorkerSlice` schema, slice validator, provider preflight integration, write-scope enforcement, launch acceptance tracking, and worker evidence ingestion. This is the first full worker launch slice and spans graph, provider, budget, subprocess, evidence, and policy systems.

**Risk:** High.

**Risk factors:** Provider-specific risk across Claude/Codex/opencode injection and resume surfaces; substrate risk from agent-runner version compatibility and `OULIPOLY_INVOCATION`; concurrency risk from overlapping write scopes; external dependency risk from quotas and entitlements; cost risk from sub-agent fanout.

**What exists:** agent-runner invocation/resume/trace; proposal-defined `WorkerSlice`, `WorkerRun`, and lifecycle.

**What is new:** Slice validation, provider-aware route approval, prompt/render for worker slices, acceptance tracking, evidence ingestion, and launch UI/control path.

**Foundation dependencies:** WorkerSlice/WorkerRun schema, ProviderStateMonitor, BudgetLedger, CLI supervisor, hook/MCP/plugin scaffold, RenderEngine, PolicyEngine, evidence/audit, fake `agents`.

**Parallelizable with:** No same-phase pair. Cross-phase, it can consume VS-006 provider preflight and VS-004 budget ledger if those APIs are stable.

### VS-016: Show the Worker Board and Trace Evidence (Phase 5)

**Executive priority:** Phase 5, position 1.

**Effort:** M.

**Effort reasoning:** Research says agent-runner `trace --json` can show invocation tree, status, timing, session/transcript state. Missing work is worker board UI, trace-to-worker mapping, budget and fingerprint badges, raw evidence drill-down, and lost/ambiguous acceptance handling. After VS-015, this is mostly a graph-addressed UI and trace mapping slice.

**Risk:** Medium.

**Risk factors:** Integration risk with `WorkerRun` state and evidence locators; substrate risk where transcript locators are missing or opencode ingestion is weaker; UX risk around ambiguous acceptance and blocked/running/failed distinctions.

**What exists:** agent-runner trace JSON capability; proposal user surface includes worker board and evidence drill-down.

**What is new:** Board filters, status selectors, trace/evidence mapping, capability/budget badges, and ambiguity states.

**Foundation dependencies:** WorkerRun records, evidence/provenance, BudgetLedger, CapabilityFingerprint, IPC/UI shell, trace fixtures.

**Parallelizable with:** Partial with VS-017 and VS-018 because all three share `WorkerRun` state transitions, evidence ingestion, and worker UI.

### VS-017: Route NEEDS_INPUT Answers as Worker Continuations (Phase 5)

**Executive priority:** Phase 5, position 2.

**Effort:** L.

**Effort reasoning:** Research says agent-runner supports non-interactive `resume` with answer payload and owner lookup. Missing work is question parser/envelope, global action-needed queue, answer payload writer, resume acceptance watcher, and failed-resume recovery handoff. The exact resume pattern is locked by `philosophy.md` `P11`.

**Risk:** High.

**Risk factors:** Provider-specific risk because resume acceptance differs by CLI; substrate risk because wrapper-recorded attempts do not prove child acceptance; recovery risk for `failed_resume`; graph-routing risk if answers attach to the wrong slice or stale graph state.

**What exists:** agent-runner resume substrate; proposal-defined `QuestionArtifact` and routing lifecycle.

**What is new:** NEEDS_INPUT envelope parsing, queue UI, answer artifact writer, acceptance watcher, and failed-resume integration.

**Foundation dependencies:** QuestionArtifact schema, WorkerRun records, CLI supervisor, ProviderStateMonitor, RecoveryAction skeleton, evidence/audit, UI queue.

**Acceptance prerequisite:** VS-017 is not accepted until a failed-resume handoff path writes durable `RecoveryAction` records with side-effect class, affected worker/session/question refs, audit linkage, and enough preserved/replayed/discarded placeholders for VS-020 to surface later without migration. Full operator-visible recovery remains VS-020.

**Parallelizable with:** Partial with VS-016 and VS-018. It shares `WorkerRun`, `QuestionArtifact`, evidence ingestion, blocked/needs-input state, and recovery handoff.

### VS-018: Stage Worker Output for Graph Reintegration (Phase 5)

**Executive priority:** Phase 5, position 3.

**Effort:** L.

**Effort reasoning:** Research says agent-runner trace/transcripts can provide worker output evidence, but the harness lacks reintegration staging schema, mapper from worker output to candidates, conflict detection, optimizer request generation, and UI staging surface. This touches worker outputs, provenance, conflict records, optimizer requests, and policy gates.

**Risk:** High.

**Risk factors:** Concurrency risk from worker outputs overlapping foreground or optimizer edits; evidence risk because final output cannot be trusted without trace; policy risk because workers cannot directly mutate topology, summaries, identity, quarantine, or focus; reviewer/recovery integration risk for high-consequence candidates.

**What exists:** Proposal-defined reintegration boundaries and worker output outputs; agent-runner evidence substrate.

**What is new:** Staged candidate model, parser/mappers, conflict-on-overlap logic, advisory optimizer request generation, and staging UI.

**Foundation dependencies:** WorkerSlice/WorkerRun records, evidence/provenance, ConflictRecord workflow, OptimizerRequest store, PolicyEngine, audit writer, UI shell.

**Parallelizable with:** Partial with VS-016 and VS-017 due shared worker state and evidence/session correlation.

### VS-019: Sample Reviewers over High-Consequence Edits (Phase 6)

**Executive priority:** Phase 6, position 1.

**Effort:** M.

**Effort reasoning:** Research says `gpt-high` is configured through codex/codex2 and agent-runner can invoke it. Missing work is sampling policy, reviewer prompt/evidence packer, reviewer result schema, and integration into optimizer/reintegration/provider anomaly gates. This is a policy/evidence integration slice once optimizer, reintegration, budget, and provenance exist.

**Risk:** Medium.

**Risk factors:** External dependency risk from OpenAI/Codex availability and rate limits; reviewer fallibility risk because reviewer output is evidence, not ground truth; cost risk from trace length and sample rate; integration risk with deterministic gates.

**What exists:** Proposal-defined `WorkflowReviewer`, reviewer sampling rules, and deterministic limits.

**What is new:** Sampling decisions, prompt packer, result schema, UI flags, budget-ledger integration, and a recovery-anomaly trigger hook that subscribes to VS-020 events once recovery accounting ships.

**Foundation dependencies:** PolicyEngine, BudgetLedger, evidence/provenance, OptimizerEdit store, Worker reintegration staging, RecoveryAction event contract, CLI supervisor, audit writer.

**Parallelizable with:** Partial with VS-020 because both share `PolicySet`, `BudgetLedger`, `AuditEvent`, UI flags, and recovery may trigger reviewer sampling.

### VS-020: Surface Recovery Preflight and State Accounting (Phase 6)

**Executive priority:** Phase 6, position 2.

**Effort:** L.

**Effort reasoning:** Research says agent-runner resume attempts, trace warnings, quota/provider diagnostics, and session ingestion can supply evidence. Missing work is `RecoveryAction` schema behavior, preflight engine, side-effect classification, recovery UI, and graph/session/provider reconciliation. This joins graph, provider, session, question, worker, and tool protocol state.

**Risk:** High.

**Risk factors:** Recovery risk because deferred recording and deferred execution differ; provider-specific risk across resume/session stores; concurrency risk where recovery touches conflicts, worker state, and graph snapshots; side-effect risk for replay/cancel/rollback operations.

**What exists:** Proposal-defined `RecoveryAction` and lifecycle; agent-runner diagnostics and resume evidence.

**What is new:** Preflight engine, preserved/replayed/discarded accounting, side-effect classifier, recovery surface, and reconciliation transactions.

**Foundation dependencies:** RecoveryAction skeleton, evidence/audit, ProviderStateMonitor, ConflictRecord workflow, QuestionArtifact, WorkerRun, ToolCallProvenance, BudgetLedger, UI shell.

**Parallelizable with:** Partial with VS-019 only with coordinated policy/audit/budget ownership.

### VS-021: Reroute or Substitute Failed Provider/Worker Runs Explicitly (Phase 7)

**Executive priority:** Phase 7, only slice.

**Effort:** M.

**Effort reasoning:** Research says agent-runner already has multi-provider pools, provider quota/exhaustion, resume strategies, and diagnostics. Missing work is alternate route comparator, substitution planner, user confirmation contract, reroute audit events, and changed-contract UI. After VS-006, VS-015, and VS-020, this is mostly recovery-policy extension and UI.

**Risk:** High.

**Risk factors:** Provider-specific risk around capability fingerprints, entitlement changes, sandbox/network failures, and resume semantics; external dependency risk from provider rate limits and auth/billing state; recovery risk because fresh substitution must not be represented as successful resume; cost risk from changed provider/model route.

**What exists:** agent-runner routing and diagnostics substrate; proposal-defined provider-aware recovery constraints.

**What is new:** Route comparison, substitution planning, confirmation flow, contract-diff display, and audit linkage.

**Foundation dependencies:** ProviderStateMonitor, CapabilityFingerprint, RecoveryAction, WorkerRun, BudgetLedger, PolicyEngine, audit writer, UI shell.

**Parallelizable with:** No same-phase pair. It depends on VS-020 and should remain last in the dependency graph.

## Pushback Summary

The executive ordering is strategically coherent, and this revision applies five cost-driven sequencing adjustments. These do not drop or change any value slice; they prevent duplicate schema/render/policy work and avoid building user-visible surfaces on temporary contracts.

### Pushback P-1: Expand Phase 0 before Phase 1 acceptance

**Status:** Applied.

**Executive ordering:** Phase 1 slices begin after an engineering-defined Phase 0 placeholder with no detailed scope.

**Engineering recommendation:** Treat Phase 0 as an XL substrate phase containing GraphStore/migrations, evidence/audit backbone, RenderEngine core, PolicyEngine, BudgetLedger core, `ProviderState` / `EntitlementSnapshot` / `CapabilityFingerprint` records, CLI supervisor with subprocess management, `OULIPOLY_INVOCATION` / parent-invocation propagation, session-id capture and trace stitching, Tauri IPC layer, frontend stack scaffolding and UI shell, and test fixtures.

**Implementation cost of executive ordering:** If VS-001 through VS-007 are built directly from the empty worktree, each slice will invent its own DTOs, schema migrations, render result states, audit calls, and UI event streams. Research explicitly warns that `RenderEngine`, `PolicyEngine`, `GraphStore`, `BudgetLedger`, provider state, and UI shell are shared surfaces across most slices.

**Value lost by engineering ordering:** Operators wait longer before seeing the first working-set inspector.

**Recommendation:** Apply the engineering ordering. The delay buys one canonical substrate instead of seven incompatible Phase 1 prototypes.

### Pushback P-2: Build provenance/audit and budget gate backends before declaring VS-001 complete

**Status:** Applied.

**Executive ordering:** VS-001 inspect renders appears before VS-003 provenance/audit and VS-004 budget/cache.

**Engineering recommendation:** Keep VS-001 as the first operator-visible target, but implement the VS-003 evidence/audit backbone and VS-004 budget/cache ledger fields before VS-001 acceptance.

**Implementation cost of executive ordering:** `WorkingSetSnapshot` includes evidence pointers, token estimate, cache prefix hash, provider state, and auditability. If the inspector ships before those IDs and ledgers exist, it will show a temporary render shape and later require migration of snapshot rows, UI drill-down, test goldens, and policy decisions.

**Value lost by engineering ordering:** The earliest inspector milestone may show less UI sooner, because backend evidence and budget plumbing must land first.

**Recommendation:** Apply the engineering ordering inside Phase 1: develop VS-001 UI in parallel, but do not mark it complete without VS-003/VS-004 backend primitives.

### Pushback P-3: Implement VS-013 conflict/identity mechanics before VS-012 topology edits

**Status:** Applied.

**Executive ordering:** VS-012 repack/cross-reference appears before VS-013 conflict and identity forwarding in Phase 3.

**Engineering recommendation:** Build VS-013's identity resolver, optimistic merge engine, and `ConflictRecord` state machine before enabling VS-012 topology edit types.

**Implementation cost of executive ordering:** Research marks VS-012 / VS-013 as "No without tight coordination": VS-012 creates topology edits, while VS-013 defines identity forwarding and conflict semantics those edits require. Building VS-012 first either produces throwaway merge logic or risks topology changes that cannot fail closed.

**Value lost by engineering ordering:** Operators wait longer for cross-reference discovery and hierarchy repack.

**Recommendation:** Move VS-013 before VS-012 in the engineering phase structure. VS-012 can begin design and fixture work in parallel, but topology mutation should not merge before conflict/identity semantics exist.

### Pushback P-4: Pull a recovery skeleton into Phase 0 / Phase 5 prerequisites

**Status:** Applied.

**Executive ordering:** Full recovery accounting is VS-020 in Phase 6, after NEEDS_INPUT routing and worker reintegration.

**Engineering recommendation:** Keep operator-visible recovery in VS-020, but implement `RecoveryAction` schema, side-effect taxonomy, and audit linkage in Phase 0, and require a failed-resume handoff path before VS-017 acceptance.

**Implementation cost of executive ordering:** VS-017 has an explicit `failed_resume` state and research says child acceptance is not guaranteed by wrapper attempts. Without recovery records, failed continuations become ad hoc error states that later need migration into VS-020's preserved/replayed/discarded accounting.

**Value lost by engineering ordering:** VS-017 may take longer because it must write durable recovery handoff records even though full recovery UI arrives later.

**Recommendation:** Apply the engineering ordering. This preserves the executive value order while preventing question-routing failures from becoming unstructured dead ends.

### Pushback P-5: Treat provider fingerprints as an early contract, not just a VS-006 UI feature

**Status:** Applied.

**Executive ordering:** VS-006 is one Phase 1 slice among seven.

**Engineering recommendation:** Define `ProviderState`, `EntitlementSnapshot`, and `CapabilityFingerprint` schemas and denial-reason taxonomy in Phase 0, then deliver the operator-visible preflight panel as VS-006.

**Implementation cost of executive ordering:** VS-001 render route display, VS-004 budget interpretation, VS-009 orchestrator launch, VS-015 worker dispatch, VS-017 continuation, VS-020 recovery, and VS-021 reroute all consume provider/capability records. If VS-006 owns the first real schema late in Phase 1, earlier slices either fake route state or need retrofits.

**Value lost by engineering ordering:** The VS-006 panel itself is not visible earlier; only its schema contract is.

**Recommendation:** Apply the engineering ordering. It makes provider readiness a shared contract while preserving VS-006 as the user-facing capability slice.

## Engineering Phase Structure

### Phase 0A: Repository and Runtime Skeleton

Initialize the fixed Tauri/Turbo/React/Rust/Tokio/SQLite stack, backend app state, settings, logging, Tauri IPC/event streams, frontend stack scaffolding, single-tab UI shell, test harness, fake `agents`, and basic CI-quality local commands. This phase has no operator-visible value beyond a running shell. It is the host substrate for the XL Phase 0 work: later sub-phases should be able to add GraphStore migrations, shared engines, subprocess supervision, provider fixtures, and seeded panes without changing the repository shape.

### Phase 0B: Canonical State and Contracts

Create migration infrastructure and the complete base GraphStore schema. Implement repository boundaries and fixture builders, but keep features inert unless a value slice turns them on. Phase 0B owns the durable objects and contracts below:

- `GraphWorkspace`: root local container for the single user's harness graph, active configuration, active policy set, storage root, and current graph version.
- `GraphConfiguration`: versioned graph and memory configuration, including schema profile, summary templates, render policy, memory policy, provider routing policy, and effective-value sources.
- `GraphNode`: stable logical object the agent can see, unpack, reference, assign, derive from, quarantine, archive, or delete.
- `GraphEdge`: typed relationship between graph nodes, including containment, evidence, blockers, worker ownership, cross-references, forwarding, and recovery relationships.
- `NodeRevision`: immutable content revision for a node, supported by evidence and valid over explicit graph-version bounds.
- `GraphSnapshot`: immutable read view used for renders, optimizer drafts, worker slices, configuration inspections, conflicts, and recovery preflight.
- `SummaryContract`: machine-checkable packed-node summary contract with evidence pointers, stale markers, omitted-detail classes, poison risk, and validation state.
- `ProvenancePointer`: claim-level pointer from derived graph content to evidence locators with derivation type, confidence, and privilege-transform data.
- `EvidenceArtifact`: raw or normalized source content such as CLI transcripts, tool results, command output, user messages, worker output, optimizer prompts, reviewer output, or external documents.
- `ToolCallProvenance`: protocol-sensitive tool-call lifecycle, approval state, side-effect class, retry semantics, and reconciliation state.
- `IdentityEvent`: durable record of identity-preserving topology changes such as move, split, merge, forwarding, restore, or delete.
- `WorkingSetSnapshot`: exact rendered context imposed on an orchestrator or worker, including evidence pointers, budget fields, provider state, capability fingerprint, configuration explanation, and rendered blob reference.
- `GraphAction`: bounded foreground graph-adjacent write for turn output, tool provenance, audit notes, user-facing output, or advisory optimizer-request creation.
- `AgentWalkState`: current orchestrator focus, pinned nodes, unpacked stack, pending focus requests, and graph snapshot used by renders.
- `OrchestratorTurn`: durable foreground turn record linking session, graph snapshot, working set, input/output evidence, tool events, graph actions, optimizer requests, and cost ledger.
- `OptimizerRequest`: advisory request asking the optimizer to consider curation, without authorizing direct graph mutation.
- `OptimizerEdit`: optimizer-authored draft mutation against a base snapshot, with deterministic validation, reviewer state, merge state, and result graph version.
- `ConflictRecord`: explicit record for optimistic-merge failures across identity, content, summary, configuration, edge, worker-overlap, question-route, tool-protocol, provider-state, or budget conflicts.
- `BudgetLedger`: cost, latency, token, cache, provider-cost, and budget-policy record for workspace, initiative, turn, worker, optimizer, reviewer, or render scopes.
- `ProviderState`: redacted observable provider/CLI/account/runtime state, including auth, billing, quota, network, runtime, sandbox, checked locations, freshness, and confidence.
- `EntitlementSnapshot`: point-in-time feature and model availability for a provider/account/runtime, including feature matrix, entitlement state, required network/runtime, probe method, and evidence.
- `CapabilityFingerprint`: visible per-CLI/session capability contract with injection, resume, tool-interception, context-strength, provider, entitlement, route-state, denial-reason, asymmetry, and observed-failure fields.
- `WorkerSlice`: bounded subgraph assignment and write-scope contract for a sub-agent, without launch UI.
- `WorkerRun`: provider-level execution record tying worker slice state to `agent-runner` invocation, session, acceptance, provider, resume, and cost state.
- `QuestionArtifact`: durable continuation record for `NEEDS_INPUT` routing, answer payloads, exact worker resume, child acceptance, and failed-resume handoff.
- `RecoveryAction`: auditable recovery plan or operation with preconditions, affected sessions/nodes/edits/providers, provider failure cause, reroute candidates, side-effect classification, confirmation state, result state, and preserved/replayed/discarded refs.
- `AuditEvent`: append-only decision and state-change event tying actor, policy, configuration, provider state, inputs, outputs, decision, and reason code together.
- `PolicySet`: versioned deterministic governance bundle for summary, render, identity, privilege, tool protocol, budget, reviewer, recovery, configuration, and provider rules.
- `ConfigurationRegistry`: Phase 0 read API and provenance resolver over `GraphConfiguration` rows, with no operator-visible inspector yet.

### Phase 0C: Shared Engines and Integration Shells

Implement RenderEngine core interfaces, PolicyEngine gate framework, BudgetLedger core service, `ConfigurationRegistry` skeleton/read API, CLI subprocess supervisor around `agents`, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, identity/conflict shell, recovery-action writer, Tauri IPC commands/Channel event streams, and seeded UI panes. This phase should include golden render fixtures per CLI shape and no value-slice-specific claims such as "workers can launch" or "optimizer refreshes summaries."

#### Substrate Integration Contract

Phase 0C owns the unified agent-runner contract the rest of the harness depends on:

- Subprocess spawn semantics for `/home/nes/.local/bin/agents`, including project directory, model selection, prompt-file invocation, stdout/stderr capture, cancellation, timeout, and child-acceptance accounting.
- `OULIPOLY_INVOCATION` and `OULIPOLY_PARENT_INVOCATION` propagation, including stderr parsing and trace correlation when a subprocess produces nested invocations.
- Session-id capture per CLI: Claude `--session-id`, Codex `thread_id` / thread-start output, and opencode session-row mapping when available.
- Trace stitching through `agents trace --json`, with invocation trees treated as evidence/substrate rather than canonical graph truth.
- Read-only ingestion from agent-runner `session_turns`, including explicit handling of missing transcript locators and opencode gaps.
- Awareness of `providers.toml`, `sessions.toml`, model configuration files, and agents configuration files, with redaction and no harness ownership of vendor credentials.
- A single wrapper API for active subprocess management, passive trace/state/config reads, provider diagnostics, resume attempts, and test fakes, so VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021 do not invent separate agent-runner readers.

#### Cross-slice contract: `OptimizerRequest` emission

Phase 0C freezes the `OptimizerRequest` schema and full request taxonomy before any emitter ships. The optimizer cycle in Phase 2 consumes these records, but every source remains advisory-only:

- `orchestrator_turn` is emitted by VS-009 when a bounded foreground turn requests curation.
- `worker_output` is emitted by VS-018 when reintegration staging identifies graph-candidate work.
- `user_surface` is emitted by VS-005 / VS-011 when configuration inspection or shape-repair surfaces request optimizer consideration.
- `backend_signal` is emitted by VS-010 for stale/summary signals that should enter the optimizer queue.
- `recovery_action` is emitted by VS-020 when recovery accounting suggests optimizer-owned repair or curation.

The contract guarantees stable fields, stable enum values, advisory-only semantics, no direct graph mutation by the emitter, and uniform validation gates across emitters. Later slices may add emission sites only by using this contract, not by adding source-specific queue variants.

### Phase 1: Observable Imposed Context

Engineering order inside the executive phase:

1. VS-003 backend evidence/audit spine and VS-004 budget/cache ledger primitives.
2. VS-001 working-set render inspection, using the real evidence/budget/provider/config fields.
3. VS-002 summary contract validation and invalid/stale render labels.
4. VS-005 configuration inspector, warning emission, advisory optimizer-request creation, and empty-graph simulation over the Phase 0 registry read API.
5. VS-006 provider preflight panel and capability fingerprints over the Phase 0 provider contracts.
6. VS-007 initiative roots and current focus UI.

VS-003, VS-004, VS-005, VS-006, and VS-007 can have parallel backend/UI lanes once the Phase 0 DTOs are frozen, but VS-001 acceptance should wait for VS-003 evidence/audit and VS-004 budget/cache primitives. The `ConfigurationRegistry` skeleton/read API and provider-state contracts are already Phase 0 substrate; VS-005 and VS-006 deliver the operator-visible surfaces, warning emission, and panel behavior.

### Phase 2: Lead Orchestrator Loop and Initial Curation

Engineering order:

1. VS-008 navigation tools over `AgentWalkState`.
2. VS-009 bounded orchestrator turns and advisory optimizer requests.
3. VS-010 summary refresh and stale markers.

VS-008 and VS-009 share tool/turn handling, so their state machine should be owned by one lane. VS-010 can start optimizer prompt/schema work in parallel, but merge behavior must consume the same `OptimizerRequest`, budget, evidence, and audit APIs.

### Phase 3: Topology Safety and Graph Integrity

Engineering order with pushback applied:

1. VS-011 configuration warnings to advisory optimizer requests.
2. VS-013 snapshot-merge conflicts and identity forwarding.
3. VS-012 repack hierarchy and discover cross-references.
4. VS-014 poison/privilege quarantine.

VS-012 design and fixtures can run during VS-013, but topology mutations should not be enabled before identity forwarding and conflict semantics fail closed.

### Phase 4: Worker Slice Dispatch

Deliver VS-015 after Phase 3. Worker dispatch should consume provider fingerprints, budget gates, write-scope policy, render snapshots, and evidence/audit records rather than creating worker-specific substitutes.

### Phase 5: Worker Legibility and Reintegration

Engineering order:

1. VS-016 worker board and trace evidence.
2. VS-017 NEEDS_INPUT continuations, with durable failed-resume handoff to recovery records.
3. VS-018 staged worker reintegration.

These three can partially overlap only if `WorkerRun` state transitions and evidence/session correlation are frozen.

### Phase 6: Governance Sampling and Recovery Accounting

Engineering order:

1. VS-019 reviewer sampling over high-consequence edits.
2. VS-020 recovery preflight and state accounting.

They share `PolicySet`, `BudgetLedger`, `AuditEvent`, and UI flags; recovery may trigger reviewer sampling. Keep one policy owner. VS-019 can ship its core sampler first, then subscribe to VS-020 recovery-anomaly events once the recovery surface emits them; this adds the explicit `VS-020 -> VS-019` event edge without changing the Phase 6 sequence.

### Phase 7: Provider-Aware Recovery Closure

Deliver VS-021 last. It depends on provider fingerprints, worker dispatch, and recovery accounting, and it should not be pulled earlier because fresh substitution and reroute are only honest after preserved/replayed/discarded accounting exists.

## Parallelization Map

| Phase | Parallelizable pairs | Shared resources / constraints |
|---|---|---|
| 1 | VS-002 + VS-003 with fixed evidence/provenance schema | Summary validation consumes evidence; stable `EvidenceArtifact` and `ProvenancePointer` contracts required |
| 1 | VS-003 + VS-004, VS-003 + VS-005, VS-003 + VS-006, VS-003 + VS-007 mostly parallel | Shared audit writer API; otherwise provenance, budget, config, provider, and focus schemas are separable |
| 1 | VS-002 + VS-006 mostly parallel | Summary contracts and provider fingerprints meet only at render labels and policy aggregation |
| 1 | VS-004 + VS-007, VS-006 + VS-007 mostly parallel | Separate UI panes; notification classification and route-denial states must be coordinated |
| 1 | All VS-001 pairs partial | VS-001 shares RenderEngine, `WorkingSetSnapshot`, provenance pointers, budget fields, configuration attribution, capability fingerprints, and UI shell with nearly every Phase 1 slice |
| 1 | VS-004 + VS-005, VS-004 + VS-006, VS-005 + VS-006 partial | Shared `GraphConfiguration.render_policy_ref`, provider routing defaults, cost interpretation, and provider badges |
| 2 | VS-008 + VS-010 mostly parallel | Navigation changes working-set shape; optimizer summary refresh mutates graph between turns; coordinate render invalidation |
| 2 | VS-008 + VS-009 partial | Shared tool/turn state machine |
| 2 | VS-009 + VS-010 partial | Shared `OptimizerRequest` lifecycle and audit events |
| 3 | VS-011 + VS-014 mostly parallel | Both use PolicyEngine and optimizer requests; distinct feature surfaces |
| 3 | VS-011 + VS-012, VS-011 + VS-013 partial | Shared request/edit/conflict taxonomy |
| 3 | VS-012 + VS-013 not parallel without tight coordination | VS-012 topology edits require VS-013 identity and conflict semantics |
| 3 | VS-012 + VS-014, VS-013 + VS-014 partial | Quarantine can block traversal and conflict with topology/identity edits |
| 4 | None | Only VS-015 |
| 5 | VS-016 + VS-017, VS-016 + VS-018, VS-017 + VS-018 partial | Shared `WorkerRun`, `QuestionArtifact`, blocked/needs-input transitions, and evidence ingestion |
| 6 | VS-019 + VS-020 partial | Shared `PolicySet`, `BudgetLedger`, `AuditEvent`, recovery-triggered reviewer sampling, and UI flags |
| 7 | None | Only VS-021 |

## Critical Path

The longest dependency chain is:

Phase 0 GraphStore / RenderEngine / PolicyEngine / Evidence / Budget / Provider / CLI supervisor / tests (XL, roughly 12-20 weeks serial equivalent) -> VS-003 provenance and VS-004 budget/cache primitives (L/M, needed before VS-001 acceptance) -> VS-001 inspect renders (L, first operator-visible target) -> VS-009 bounded orchestrator turns (L) -> VS-010 summary/stale optimizer (L) -> VS-013 identity/conflict (L) -> VS-015 worker dispatch (L) -> VS-017 NEEDS_INPUT continuations with failed-resume handoff and VS-018 reintegration staging (L/L) -> VS-020 recovery preflight (L) -> VS-021 provider-aware reroute/substitution (M).

The chain is acyclic: downstream worker, question, reintegration, reviewer, and recovery slices consume graph/provenance/provider/budget/conflict contracts but do not redefine them.

## Effort Summary

This table uses approximate serial ranges from the T-shirt scale. Parallel effort assumes clean ownership boundaries and a frozen Phase 0 schema contract; it is not a calendar commitment.

| Phase | Initiatives | Serial effort | Parallel effort, best case |
|---|---:|---:|---:|
| 0 | 22 foundation items | XL, roughly 12-20 weeks serial equivalent | 6-10 weeks with 4-5 lanes |
| 1 | 7 value slices | 1 L + 6 M/L mix, roughly 10-16 weeks | 5-8 weeks with backend/UI lanes and schema freeze |
| 2 | 3 value slices | M + L + L, roughly 5-10 weeks | 4-7 weeks |
| 3 | 4 value slices | M + L + L + L, roughly 7-14 weeks | 5-9 weeks if VS-013 owns merge semantics |
| 4 | 1 value slice | L, roughly 2-4 weeks | 2-4 weeks |
| 5 | 3 value slices | M + L + L, roughly 5-10 weeks | 4-7 weeks |
| 6 | 2 value slices | M + L, roughly 3-6 weeks | 3-5 weeks |
| 7 | 1 value slice | M, roughly 1-2 weeks | 1-2 weeks |
| **Total** | **21 value slices + Phase 0** | **45-82 weeks serial equivalent** | **30-52 weeks best-case parallel equivalent** |

## Risk Summary

| Slice | Effort | Risk | Main risk factors |
|---|---:|---:|---|
| VS-001 | L | High | Imposed render novelty, cross-CLI render differences, cache locality, agent-runner trace dependence |
| VS-002 | M | Medium | Contract validity, evidence pointer dependence, privilege/poison labels in render |
| VS-003 | L | High | Cross-CLI tool protocol asymmetry, opencode ingestion gap, side-effect classification |
| VS-004 | M | Medium | Provider quota/pricing uncertainty, prompt-cache TTL, budget gates across many actors |
| VS-005 | M | Medium | Configuration semantics, empty-graph simulation accuracy, default/source attribution |
| VS-006 | M | Medium | Provider auth/billing/quota/entitlement/runtime probes, redaction, agent-runner compatibility |
| VS-007 | M | Medium | Multi-workstream legibility, notification classification, shared UI shell |
| VS-008 | M | Medium | Tool affordance integration, walk-state vs topology boundary, budget/conflict denial |
| VS-009 | L | High | Resume/session capture, subprocess lifecycle, no-compact discipline, foreground action boundary |
| VS-010 | L | High | Optimizer concurrency, `glm` availability/cost, summary drift, cache churn |
| VS-011 | M | Medium | Warning taxonomy, advisory-only shape repair, configuration policy integration |
| VS-012 | L | High | Hierarchical packing novelty, topology mutation, identity forwarding, merge conflicts |
| VS-013 | L | High | Snapshot-walk-then-merge semantics, conflict records, identity drift, recovery linkage |
| VS-014 | L | High | Prompt injection / graph poisoning, privilege transforms, quarantine traversal blocks |
| VS-015 | L | High | Provider-aware worker launch, trace/acceptance capture, write-scope overlap, sub-agent cost |
| VS-016 | M | Medium | Trace mapping, ambiguous acceptance, missing transcript locators, worker-state UI |
| VS-017 | L | High | Resume acceptance, exact question correlation, failed-resume recovery, provider differences |
| VS-018 | L | High | Worker output trust boundary, conflict-on-overlap, staged graph candidates, optimizer handoff |
| VS-019 | M | Medium | Reviewer fallibility, sample-rate cost, deterministic-gate precedence, provider availability |
| VS-020 | L | High | Recovery side effects, graph/session/provider reconciliation, replay vs recording classification |
| VS-021 | M | High | Changed execution contracts, provider reroute semantics, fresh substitution honesty, quotas |

## Dependency Graph

The engineering dependency graph is:

Phase 0 foundations -> Phase 1 observable context.

VS-001, VS-002, VS-004 -> VS-008.

VS-001, VS-003, VS-004, VS-006 -> VS-009.

VS-002, VS-003, VS-004, VS-005 -> VS-010.

VS-005, VS-010 -> VS-011.

VS-009, VS-010 -> VS-013.

VS-008, VS-010, VS-013 -> VS-012.

VS-002, VS-003, VS-010 -> VS-014.

VS-006, VS-004, VS-009, VS-007, VS-013 -> VS-015.

VS-015 -> VS-016.

VS-015, VS-006 -> VS-017.

VS-015, VS-010, VS-013, VS-003 -> VS-018.

VS-010, VS-018, VS-003, VS-004 -> VS-019.

VS-020 recovery-anomaly events -> VS-019 reviewer sampling triggers.

VS-003, VS-006, VS-013, VS-017, VS-018 -> VS-020.

VS-006, VS-015, VS-020 -> VS-021.

This graph has no cycles at the acceptance-prerequisite level. The `VS-020 -> VS-019` recovery edge is an event-subscription addition inside Phase 6: VS-019's core sampler can ship before VS-020, then activate recovery-anomaly sampling once VS-020 emits the events. The only executive-order change is moving VS-013 before VS-012 inside Phase 3 and treating selected schemas/engines as Phase 0 foundations rather than hidden work inside the first slice that happens to need them.
