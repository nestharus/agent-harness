# Problem Alignment Review

## Unaddressed axes

None.

## Misalignments

None.

## Constraint-driven blind spots

None.

## Axes aligned (for Stage 2 handoff)

- **§1 Effective Working Set vs. Nominal Context Window** — The proposal engages the reasoning-quality problem rather than treating nominal window size as sufficient. The **Imposed Working Set** commitment, `WorkingSetSnapshot`, `RenderEngine`, effective-reasoning budgets, required pins, eviction behavior, and render invalidation for noisy or overfull context all aim at keeping each imposed `W` small, focused, and faithful.

- **§2 Summary Contract: What Unpacking Must Faithfully Reveal** — The proposal engages the summary-only-loop and summary-drift difficulty through **Summary Contract**, `SummaryContract`, `ProvenancePointer`, evidence-linked summaries, invalid summary states, and **Summary Regeneration**. It treats summaries as machine-checkable contracts over evidence, not fluent prose that can silently diverge from unpacked detail.

- **§3 Concurrent Optimizer Mutation Under Foreground Walking** — The proposal engages shared-state coordination through **Snapshot-Walk-Then-Merge**, immutable `GraphSnapshot` reads, append-only `OptimizerEdit` drafts, `ConflictRecord`, the **Optimizer Cycle**, and **Audit and Reversibility**. The new `GraphAction` path is bounded to foreground artifact recording and advisory request creation, while `OptimizerRequest` is explicitly non-mutating and only a possible input to later optimizer-owned edits. This keeps foreground walking deterministic without pretending background curation stops.

- **§4 Stable Identity Across Summary Regeneration and Topology Reshape** — The proposal engages identity drift through **Stable Identity**, opaque stable IDs, `IdentityEvent`, forwarding maps, snapshot-scoped identity resolution, and split/merge/re-parent semantics. `GraphAction` is explicitly forbidden from creating or modifying `GraphNode`, `GraphEdge`, `NodeRevision`, `SummaryContract`, or `IdentityEvent`, and `OptimizerRequest` cannot authorize topology or identity changes; identity-affecting changes must flow through `OptimizerEdit` and merge validation.

- **§5 Working-Set Policy: Pinning, Eviction, and Recursive Unpack** — The proposal engages persistent working-set selection through **Imposed Working Set**, `AgentWalkState`, `WorkingSetSnapshot`, explicit working-set layers, eviction order, required pins, recursive unpack bounds, and pack/unpack/focus tool states. `GraphAction` records may cite walk state but do not mutate topology, while `OptimizerRequest` can only ask the optimizer to consider repack, split, merge, reparent, or related curation, preserving the boundary between agent focus control and graph curation.

- **§6 Hierarchical Packing Without Bounded-Depth Precedent** — The proposal engages live nested graph topology through containment and `packs_into` / `unpacks_to` edge types, recursive unpack bounds, `AgentWalkState` depth tracking, optimizer-owned repack edits, and topology-preserving identity events. It recognizes that hierarchy is graph state maintained under policy rather than a static folder layout.

- **§7 Cross-CLI Rendering Asymmetry** — The proposal engages CLI asymmetry through **Cross-CLI Adaptation**, `CapabilityFingerprint`, CLI-specific render paths in `RenderEngine`, worker limitation disclosures, and UI exposure of weaker context injection, tool interception, or resume guarantees. It does not collapse Claude, Codex, and opencode into a fictional common capability model.

- **§8 Sub-Agent Supervision via Subgraph Slices** — The proposal engages delegation and reintegration as graph operations through `WorkerSlice`, `WorkerRun`, **Sub-Agent Dispatch and Reintegration**, staged worker outputs, overlap policy, acceptance tracking, reintegration states, and `ConflictRecord`. Worker outputs and worker-originated `OptimizerRequest` artifacts are staged inputs, not direct graph truth; curation-affecting reintegration still requires optimizer-owned `OptimizerEdit`, which addresses overlapping slice mutation and parent supervision.

- **§9 User-Question Routing as Graph-State Routing** — The proposal engages question routing through **Questions as Continuations**, `QuestionArtifact`, **NEEDS_INPUT Routing**, render/slice/session correlation keys, child acceptance tracking, and refusal to substitute lead-chat append for worker resume. Questions are durable continuations tied to blocked graph work, not ordinary messages.

- **§10 Tool-Call Protocol State as First-Class Graph Provenance** — The proposal engages protocol-sensitive tool state through **Provenance Beside Summaries**, `EvidenceArtifact`, `ToolCallProvenance`, deterministic tool-protocol gates, `GraphAction` allowed effects for recording tool provenance, and evidence-linked summaries. It keeps pending calls, results, IDs, approval state, and retry semantics as graph provenance rather than collapsing them into prose.

- **§11 Workflow-Reviewer Reliability for Graph Mutations** — The proposal engages reviewer fallibility through `WorkflowReviewer`, `PolicyEngine`, **Deterministic Gates**, **Reviewer Sampling**, and explicit limits on reviewer authority. Reviewer output can flag or add confidence but cannot bypass deterministic validation, create missing evidence, or become ground truth.

- **§12 Instruction Hierarchy and Memory Poisoning Becoming Graph Poisoning** — The proposal engages graph poisoning through `privilege_origin`, `trust_state`, poison quarantine, `ProvenancePointer.privilege_transform`, **Privilege and Poisoning Controls**, optimizer limitations, and deterministic gates. It treats optimizer-promoted content as dangerous unless provenance, privilege labels, and quarantine rules remain attached.

- **§13 Cost, Latency, and Resource Tails of Graph-Walking Workloads** — The proposal engages cost as operational correctness through **Cost as Correctness**, `BudgetLedger`, token and latency tracking, cache prefix hashes, budget-gated render / optimizer / reviewer / worker behavior, sampled reviewer use, and optimizer cadence narrowing. It treats excessive maintenance cost as a correctness failure for long-lived operation.

- **§14 Multi-Workstream Legibility for the Agent and the User** — The proposal engages multi-initiative legibility through `UserSurface`, initiative roots, current-focus and working-set inspectors, question queue, worker board, optimizer log, cost surface, recovery surface, graph-addressed observability, and separation of action-needed notifications from passive progress.

- **§15 Imposed Working Context Without Surveyed Precedent** — The proposal engages the imposed-context novelty directly through **Imposed Working Set**, graph-derived renders, local graph canonicality, **No In-Product Compaction**, the boundary between agent-owned focus tools and optimizer-owned curation, bounded foreground `GraphAction`, and advisory-only `OptimizerRequest`. It acknowledges that the harness, not the model's retrieval discretion, owns the rendered working set.

- **§16 Recovery and Resume Surfaces Are Not Neutral** — The proposal engages recovery as state-changing graph work through `RecoveryAction`, recovery preflight snapshots, preserved / replayed / discarded records, failed-resume states, child acceptance checks, recovery UI surfaces, `ToolCallProvenance`, and conflict records. Recovery is audited as graph state transition rather than treated as a neutral CLI session reload.
