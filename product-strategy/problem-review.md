# Stage 1 Problem Alignment Review - Round 7

## Round scope

Round 7 was triggered by a stack-correction edit to `proposal.md`: the prior hallucinated frontend stack list ("Bun, Turbo, React 19, TanStack Router SPA, TanStack Query 5, Tailwind v4, xo-typescript ESLint, Prettier, Vitest, Playwright, Lefthook, Changesets, Commitlint") was removed, and all concrete stack content was deferred to `/DECISIONS.md` (D2 application substrate from `~/projects/agent-runner`; D3–D17 frontend SPA substrate from `~/projects/server-manager` research, including D7–D7e and D8–D11 SPA cascade; D18 model assignments). Tailwind is now explicitly REJECTED. Frontend = Solid 1.9.x. Backend = `rusqlite`, NOT `sqlx`.

The Fixed Substrate section now references substrate **by category** only (Tauri v2 desktop shell, Rust/Tokio backend, local SQLite database, frontend SPA) with explicit pointers to DECISIONS.md. No problem-aligned design commitment, data-model entry, lifecycle, governance gate, observability surface, AI/ML actor, constraint, or non-goal was rewritten.

Coverage scan and per-axis alignment analysis were re-run against the corrected proposal. Results match round 3–6: all 18 axes remain engaged and aligned. No axis depends on a specific frontend framework, package manager, build tool, lint/format tool, test runner, type-checker, ORM, or migration runner — every axis is about long-lived agentic operation under graph-based context engineering, which is framework-agnostic at the level the proposal describes. The stack correction therefore does not regress, redirect, or weaken any axis treatment.

## Unaddressed axes

None. The proposal engages all 18 problem axes.

## Misalignments

None. The corrected Fixed Substrate references DECISIONS.md categorically without altering any per-axis mechanism. Every prior-round alignment finding remains valid because the substrate-by-category posture leaves the harness's graph, working-set, optimizer, reviewer, identity, provenance, cross-CLI adaptation, provider-state, configuration, and recovery semantics unchanged.

## Constraint-driven blind spots

None. The proposal's Fixed Substrate (lines 18–20) honestly acknowledges that concrete versions, crates, and packages are roadmap- and DECISIONS-layer concerns and not enumerated in the proposal, and the Constraints and What Is Lost section continues to enumerate what is sacrificed (single user, local canonical state, snapshot-walk delay, contract-invalid summaries, cross-CLI asymmetry, configuration explicitness, provider observation rather than ownership, no `/compact`, sampled reviewer coverage). The deferral to DECISIONS.md is itself an honest scope boundary, not a blind spot in problem-axis treatment.

## Axes aligned (Stage 2 handoff)

All 18 axes pass problem alignment and proceed to philosophy review. The round 3 alignment evidence remains in force; the stack correction did not touch the mechanisms cited there. Brief reaffirmation per axis:

- **§1 Effective Working Set vs. Nominal Context Window** — Aligned. `Imposed Working Set`, `WorkingSetSnapshot`, `Working-Set Policy`, render budget gates, required pins, eviction order, and `overfull_required_context` continue to treat working-set quality as an effective reasoning budget rather than vendor context capacity. Untouched by stack deferral.

- **§2 Summary Contract** — Aligned. `SummaryContract`, `ProvenancePointer`, `Summary Regeneration`, deterministic summary-contract gates, and `validation_state` enumerations preserve machine-checkable focus, status, uncertainty, blockers, evidence pointers, stale markers, omitted detail, and unpack affordances. Untouched by stack deferral.

- **§3 Concurrent Optimizer Mutation Under Foreground Walking** — Aligned. `Snapshot-Walk-Then-Merge`, `GraphSnapshot`, bounded `GraphAction`, advisory `OptimizerRequest`, `OptimizerEdit`, `ConflictRecord`, `Orchestrator Turn`, and `Optimizer Cycle` keep per-turn views deterministic with optimistic merge and explicit conflicts. Untouched by stack deferral.

- **§4 Stable Identity Across Summary Regeneration and Topology Reshape** — Aligned. `Stable Identity`, `GraphNode`, `IdentityEvent`, `GraphEdge.forwards_to`, snapshot-local identity resolution, and optimizer-owned topology edits keep identity as a durable contract across regeneration, split, merge, re-parenting, forwarding, and recovery. Untouched by stack deferral.

- **§5 Working-Set Policy** — Aligned. `Working-Set Policy`, `AgentWalkState`, `WorkingSetSnapshot`, `Pack, Unpack, and Focus Tools`, eviction order, recursive bounds, and denial states preserve pin/evict/recursion discipline. Untouched by stack deferral.

- **§6 Hierarchical Packing Without Bounded-Depth Precedent** — Aligned. Recursive unpack bounds, `GraphEdge` containment plus `packs_into` / `unpacks_to`, `AgentWalkState.unpacked_stack`, and optimizer `repack` edits keep arbitrarily deep packed graph state navigable under live operation. Untouched by stack deferral.

- **§7 Cross-CLI Rendering Asymmetry** — Aligned. `Cross-CLI Adaptation`, `RenderEngine`, `CapabilityFingerprint`, `WorkerRun`, and the cross-CLI Constraints/Lost entry keep CLI capability differences explicit. Untouched by stack deferral.

- **§8 Sub-Agent Supervision via Subgraph Slices** — Aligned. `WorkerSlice`, `WorkerRun`, `Sub-Agent Dispatch and Reintegration`, `WorkerDispatcher`, overlap policy, and staged reintegration keep delegation a graph-slice operation. Untouched by stack deferral.

- **§9 User-Question Routing as Graph-State Routing** — Aligned. `Questions as Continuations`, `QuestionArtifact`, `NEEDS_INPUT Routing`, child acceptance, and the resume state machine attach questions to slice/worker/render/blocked output and demand acceptance before treating answers as resumed work. Untouched by stack deferral.

- **§10 Tool-Call Protocol State as First-Class Graph Provenance** — Aligned. `Provenance Beside Summaries`, `EvidenceArtifact`, `ToolCallProvenance`, `GraphAction.record_tool_provenance`, deterministic tool-protocol gates, and `Fact Extraction` preserve protocol IDs, results, approval state, retry semantics, side-effect class, and source protocol. Untouched by stack deferral.

- **§11 Workflow-Reviewer Reliability for Graph Mutations** — Aligned. `WorkflowReviewer`, `PolicyEngine`, `Deterministic Gates`, `Reviewer Sampling`, the `Workflow Reviewer` AI-use section, and explicit reviewer cannot/can boundaries keep reviewer output as fallible evidence with deterministic gates carrying hard guarantees. Untouched by stack deferral.

- **§12 Instruction Hierarchy and Memory Poisoning Becoming Graph Poisoning** — Aligned. `Privilege and Poisoning Controls`, `GraphNode.privilege_origin`, `GraphNode.trust_state`, `SummaryContract.poison_risk`, `ProvenancePointer.privilege_transform`, quarantine states, and optimizer validation engage poisoning as a graph-promotion and imposed-context risk. Untouched by stack deferral.

- **§13 Cost, Latency, and Resource Tails of Graph-Walking Workloads** — Aligned. `Cost as Correctness`, `BudgetLedger`, render/optimizer/reviewer budget gates, cache prefix hashes on `WorkingSetSnapshot`, and `policy_action` thresholds treat cost as operational correctness. Untouched by stack deferral.

- **§14 Multi-Workstream Legibility for the Agent and the User** — Aligned. `UserSurface`, `Observability`, initiative-root states, question queue, worker board, optimizer log, recovery surface, provider panel, configuration inspector, and action-needed/passive separation keep the shared graph as the multi-workstream model. Untouched by stack deferral.

- **§15 Imposed Working Context Without Surveyed Precedent** — Aligned. `Imposed Working Set`, `GraphStore` as canonical graph, `RenderEngine`, bounded `GraphAction`, advisory `OptimizerRequest`, optimizer-owned curation, `No In-Product Compaction`, and the corresponding non-goal/constraint show the stronger render/provenance/audit/recovery guarantees imposition demands. Untouched by stack deferral.

- **§16 Recovery and Resume Surfaces Are Not Neutral** — Aligned. `RecoveryAction`, `Recovery`, `QuestionArtifact`, `ToolCallProvenance`, `WorkerRun.acceptance_state`, `Provider Preflight and Routing`, recovery gates, and explicit preserved/replayed/discarded refs distinguish neutral vs. side-effecting state changes. Untouched by stack deferral.

- **§17 Graph and Memory Configuration Overhead** — Aligned. `Configuration as Memory Semantics`, `GraphConfiguration`, `ConfigurationRegistry`, `WorkingSetSnapshot.configuration_explanation_ref`, `OptimizerEdit.configuration_refs`, `Configuration Inspection and Validation`, configuration gates, anomaly-triggered reviewer sampling, and configuration-accountability audit make schema, templates, indexes, render caps, optimizer cadence, memory policy, provider routing defaults, and field meanings versioned causes of graph shape and failure attribution. Untouched by stack deferral.

- **§18 Provider, Account, and Entitlement Friction** — Aligned. `Provider State as Observable State`, `ProviderStateMonitor`, `ProviderState`, `EntitlementSnapshot`, extended `CapabilityFingerprint`, `WorkerSlice.required_provider_features`, `Provider Preflight and Routing`, `route_denial_reasons`, provider-aware `BudgetLedger`, provider-caused `RecoveryAction`, and provider accountability audit make auth, billing, quota, entitlement, runtime, network, sandbox, feature support, freshness, and confidence pre-routing state. Untouched by stack deferral.

## Surface discovery

No new problem surfaces found.

The stack correction is a process-discipline event at the proposer/roadmap layer (recovering canonical stack from `~/projects/agent-runner` and `~/projects/server-manager` research, replacing v0 hallucinations), not a new harness-runtime difficulty. None of the stack-correction artifacts (Solid vs. React, vanilla CSS vs. Tailwind, rusqlite vs. sqlx, Oxlint/Oxfmt vs. ESLint/Prettier, Bun + Vite vs. Bun + Turbo, tsgo vs. tsc, `@solidjs/router` vs. TanStack Router) introduce a difficulty that problem.md does not already cover at the architectural level it describes.

The fact that v0 carried a hallucinated stack list and was corrected by recovering DECISIONS.md is itself an instance of the durable-fact-from-hallucination dynamic in §12 (memory poisoning) at the design-process level, but problem.md is scoped to the harness's runtime behavior, not its build/planning process. No new axis is required, and no emergent depth within an existing axis is revealed by the stack correction.

`problem-surfaces.md` was not created.

## Files written

- `/home/nes/projects/agent-harness/product-strategy/problem-review.md` (this file).

`/home/nes/projects/agent-harness/product-strategy/problem-surfaces.md` was intentionally not written; the stack correction surfaced no new harness-runtime problem axes.
