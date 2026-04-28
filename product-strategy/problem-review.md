# Stage 1 Problem Alignment Review - Round 3

## Unaddressed axes

None. The proposal engages all 18 problem axes.

## Misalignments

None.

## Constraint-driven blind spots

None.

## Surface discovery

No new problem surfaces found. The new proposal mechanisms for configuration and provider state are covered by the existing problem definition in §17 and §18, and they do not expose an additional proposal-originated or emergent surface that needs expansion.

## Axes aligned

These axes pass problem alignment and should proceed to Stage 2 philosophy review.

- **§1 Effective Working Set vs. Nominal Context Window** - Aligned. The proposal treats working-set quality as an effective reasoning budget, not vendor context capacity, in `Design Commitments / Imposed Working Set`; `WorkingSetSnapshot`; `Working-Set Policy`; and the render budget gates in `PolicyEngine`. Required pins, eviction order, token/node/evidence/depth caps, and `overfull_required_context` engage the noisy-`W` failure mode directly.

- **§2 Summary Contract: What Unpacking Must Faithfully Reveal** - Aligned. `Design Commitments / Summary Contract`, `SummaryContract`, `ProvenancePointer`, `Summary Regeneration`, and deterministic summary-contract gates define machine-checkable focus, status, uncertainty, blockers, evidence pointers, stale markers, omitted detail, and unpack affordances. Invalid summaries are surfaced as invalid state rather than fluent replacement prose.

- **§3 Concurrent Optimizer Mutation Under Foreground Walking** - Aligned. `Snapshot-Walk-Then-Merge`, `GraphSnapshot`, bounded `GraphAction`, advisory `OptimizerRequest`, `OptimizerEdit`, `ConflictRecord`, `Orchestrator Turn`, and `Optimizer Cycle` preserve a stable per-turn snapshot while allowing background edits to merge only at boundaries with explicit conflicts.

- **§4 Stable Identity Across Summary Regeneration and Topology Reshape** - Aligned. `Stable Identity`, `GraphNode`, `IdentityEvent`, `GraphEdge`, `GraphSnapshot`, and `OptimizerEdit` treat identity as a durable contract across summary regeneration, split, merge, re-parenting, forwarding, and recovery rather than as current path or prose.

- **§5 Working-Set Policy: Pinning, Eviction, and Recursive Unpack** - Aligned. `Working-Set Policy`, `AgentWalkState`, `WorkingSetSnapshot`, and `Pack, Unpack, and Focus Tools` define pins, active detail, visible summaries, evidence affordances, eviction order, recursive bounds, denial states, and budget failure behavior. Configuration participation in `W` does not regress the axis; it makes policy causes inspectable.

- **§6 Hierarchical Packing Without Bounded-Depth Precedent** - Aligned. `Working-Set Policy`, recursive unpack bounds, `GraphEdge` containment plus `packs_into` / `unpacks_to`, `AgentWalkState.unpacked_stack`, and optimizer `repack` edits engage live, bounded traversal of arbitrarily deep packed graph state while preserving identity and conflict semantics.

- **§7 Cross-CLI Rendering Asymmetry** - Aligned. `Cross-CLI Adaptation`, `RenderEngine`, `CapabilityFingerprint`, `WorkerRun`, `Sub-Agent Dispatch`, and the `Constraints and What Is Lost` entry for cross-CLI differences explicitly avoid a uniform-context fiction and expose weaker injection, tool interception, resume, entitlement, runtime, and sandbox surfaces.

- **§8 Sub-Agent Supervision via Subgraph Slices** - Aligned. `WorkerSlice`, `WorkerRun`, `Sub-Agent Dispatch and Reintegration`, `Provenance Beside Summaries`, and `WorkerDispatcher` make delegation a graph-slice operation with assigned roots, allowed adjacency, overlap policy, write scope, trace capture, staged reintegration, and conflict records instead of final-response paste.

- **§9 User-Question Routing as Graph-State Routing** - Aligned. `Questions as Continuations`, `QuestionArtifact`, `NEEDS_INPUT Routing`, and `WorkerSlice` attach questions to worker, slice, render, blocked output, node, and correlation key, and require child acceptance before treating an answer as resumed work.

- **§10 Tool-Call Protocol State as First-Class Graph Provenance** - Aligned. `Provenance Beside Summaries`, `EvidenceArtifact`, `ToolCallProvenance`, `GraphAction`, deterministic tool-protocol gates, and `Fact Extraction` preserve protocol IDs, tool results, approval state, retry semantics, side-effect class, and source protocol as evidence rather than collapsing them into prose.

- **§11 Workflow-Reviewer Reliability for Graph Mutations** - Aligned. `WorkflowReviewer`, `PolicyEngine`, `Deterministic Gates`, `Reviewer Sampling`, and the `Workflow Reviewer` AI-use section keep reviewer output as fallible evidence, with deterministic schema, provenance, tool-protocol, privilege, identity, budget, configuration, and provider gates carrying the hard guarantees.

- **§12 Instruction Hierarchy and Memory Poisoning Becoming Graph Poisoning** - Aligned. `Privilege and Poisoning Controls`, `GraphNode.privilege_origin`, `GraphNode.trust_state`, `SummaryContract.poison_risk`, `ProvenancePointer.privilege_transform`, quarantine states, and optimizer validation address poisoning as a graph-promotion and imposed-context risk.

- **§13 Cost, Latency, and Resource Tails of Graph-Walking Workloads** - Aligned. `Cost as Correctness`, `BudgetLedger`, render/optimizer/reviewer budget gates, cache prefix hashes on `WorkingSetSnapshot`, optimizer cadence narrowing, reviewer sampling, and worker-dispatch approval thresholds treat cost and latency as operational correctness controls.

- **§14 Multi-Workstream Legibility for the Agent and the User** - Aligned. `UserSurface`, `Observability`, `GraphWorkspace`, initiative-root states, question queue, worker board, optimizer log, recovery surface, provider panel, configuration inspector, and action-needed/passive notification separation engage the shared graph as the multi-workstream model for both agent and user.

- **§15 Imposed Working Context Without Surveyed Precedent** - Aligned. `Imposed Working Set`, `GraphStore` as canonical graph, `RenderEngine`, bounded `GraphAction`, advisory `OptimizerRequest`, optimizer-owned curation, `No In-Product Compaction`, and the corresponding non-goal/constraint show the proposal understands that imposed context requires stronger render, provenance, audit, and recovery guarantees than consulted memory.

- **§16 Recovery and Resume Surfaces Are Not Neutral** - Aligned. `RecoveryAction`, `Recovery`, `QuestionArtifact`, `ToolCallProvenance`, `WorkerRun.acceptance_state`, `Provider Preflight and Routing`, and recovery gates distinguish preserved, replayed, discarded, deferred-recording, deferred-execution, provider-caused, and rerouted state instead of treating resume as neutral session restoration.

- **§17 Graph and Memory Configuration Overhead** - Aligned. The new design treatment engages the configuration-shapes-memory-semantics difficulty, not just setup UX. `Configuration as Memory Semantics`, `GraphConfiguration`, `ConfigurationRegistry`, `WorkingSetSnapshot.configuration_explanation_ref`, `OptimizerEdit.configuration_refs`, `Configuration Inspection and Validation`, configuration gates, reviewer anomaly sampling, and configuration/accountability audit make schema, templates, index scopes, render caps, optimizer cadence, memory policy, provider routing defaults, and field meanings versioned causes of graph shape, render shape, optimizer behavior, and failure attribution.

- **§18 Provider, Account, and Entitlement Friction** - Aligned. The new design treatment engages provider-state-as-observable-state, not just error handling. `Provider State as Observable State`, `ProviderStateMonitor`, `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, `WorkerSlice.required_provider_features`, `Provider Preflight and Routing`, route denial reasons, provider-aware `BudgetLedger`, provider-caused `RecoveryAction`, and provider accountability audit make auth, billing, quota, entitlement, runtime, network, sandbox, feature support, freshness, and confidence pre-routing state that affects workload eligibility and recovery.
