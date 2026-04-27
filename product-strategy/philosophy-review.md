# Philosophy Alignment Review

## Round-2 focus

The round-1 structural concern is resolved.

The proposal now defines `GraphAction` as a bounded foreground graph-adjacent write in **Data Model / GraphAction**. Its `action_type` set is limited to `record_model_output`, `record_tool_provenance`, `attach_audit_note`, `emit_user_facing_output`, and `create_optimizer_request`. Its allowed effects are evidence, tool-call provenance, audit events, and advisory optimizer requests. Its disallowed effects explicitly include creating, updating, deleting, splitting, merging, re-parenting, repacking, or forwarding `GraphNode` identity; modifying `GraphEdge`, including containment, `cross_ref`, `packs_into`, and `unpacks_to`; modifying `NodeRevision`, `SummaryContract`, or `IdentityEvent`; regenerating summaries; marking stale state as graph truth; repairing provenance; quarantining nodes; or mutating topology.

That explicit allow/deny boundary preserves `P5 -- Agent-Owned Focus, Optimizer-Owned Curation`: the orchestrator can record foreground trace and request focus through walk-state tools, but it cannot curate graph topology, summaries, cross-references, repacks, splits, merges, re-parents, provenance repair, or quarantine state.

The proposal also defines `OptimizerRequest` in **Data Model / OptimizerRequest** as an advisory request asking the optimizer to consider graph curation. It states that the request is input to the optimizer queue, not a graph mutation; does not authorize the source actor to decide that nodes, revisions, edges, summaries, or topology changes should exist; and can only become graph truth through a separate `OptimizerEdit` with its own scope, evidence, validation, reviewer state, audit trail, and actor attribution.

That pathway preserves `P1 -- Imposed Context, Visible to the User` and `P5`: user-visible topology or summary changes are attributed to `OptimizerEdit`, not laundered through the foreground orchestrator. **Data Model / OptimizerEdit**, **Operational Lifecycle / Orchestrator Turn**, **Governance / Deterministic Gates**, **User Surface**, and **AI and ML Use / Orchestrator** all repeat the same boundary: accepted topology or summary changes remain optimizer-authored decisions in the user surface.

The `Orchestrator Turn` lifecycle is coherent with the new pathway. `tool_pending` may hold a tool call, walk-state tool, or bounded `GraphAction`; `committing -> optimizer_enqueue` occurs only after bounded `GraphAction` records pass policy and `OptimizerRequest` artifacts are queued as advisory input; `optimizer_enqueue -> complete` writes queues and audit events. There is no remaining ambiguous foreground mutation path.

The **AI and ML Use / Orchestrator** output list is now consistent with `P5`. It lists bounded `GraphAction` records for turn output, tool provenance, audit notes, user-facing output, and advisory `OptimizerRequest` creation, and its limitations explicitly exclude direct creation or mutation of graph topology, summaries, cross-references, node revisions, identity events, repacks, splits, merges, re-parents, provenance repairs, and quarantine state.

## Violations

None.

## Ungrounded decisions

None.

## Structural concerns

None.

## Embodiment summary

- **§1 Effective Working Set vs. Nominal Context Window:** Embodied through **Imposed Working Set**, `WorkingSetSnapshot`, required pins, eviction order, and render invalidation when required context exceeds budget. This follows `P1`, `P2`, and `P12`.
- **§2 Summary Contract:** Embodied through **Summary Contract**, `SummaryContract`, `ProvenancePointer`, invalid summary states, and **Summary Regeneration**. This follows `P3` and `P7`.
- **§3 Concurrent Optimizer Mutation:** Embodied through **Snapshot-Walk-Then-Merge**, immutable `GraphSnapshot`, append-only `OptimizerEdit`, `ConflictRecord`, and turn-boundary visibility. This follows `P1`, `P5`, `P6`, and `P14`.
- **§4 Stable Identity:** Embodied through stable opaque IDs, `IdentityEvent`, forwarding maps, optimizer-owned topology edits, and snapshot-scoped identity resolution. This follows `P1`, `P4`, `P5`, and `P15`.
- **§5 Working-Set Policy:** Embodied through `AgentWalkState`, pack/unpack/focus/pin tools that modify walk state rather than graph topology, and the bounded `GraphAction` boundary. This follows `P2` and `P5`.
- **§6 Hierarchical Packing:** Embodied through recursive unpack bounds, containment and packing edges, stable identity across topology changes, and optimizer-owned repack edits. This follows `P2`, `P4`, and `P6`.
- **§7 Cross-CLI Rendering Asymmetry:** Embodied through **Cross-CLI Adaptation**, CLI-specific render paths, and `CapabilityFingerprint`. This follows `P8` and `P13`.
- **§8 Sub-Agent Supervision:** Embodied through `WorkerSlice`, `WorkerRun`, staged reintegration, overlap policy, and advisory `OptimizerRequest` for curation-affecting candidates. This follows `P5`, `P7`, `P8`, and `P10`.
- **§9 User-Question Routing:** Embodied through `QuestionArtifact`, `NEEDS_INPUT Routing`, child acceptance tracking, and refusal to treat answers as ordinary orchestrator chat. This follows `P11` and `P14`.
- **§10 Tool-Call Protocol Provenance:** Embodied through `EvidenceArtifact`, `ToolCallProvenance`, deterministic protocol gates, and evidence-linked summaries. This follows `P7`, `P10`, and `P14`.
- **§11 Workflow-Reviewer Reliability:** Embodied through `WorkflowReviewer` as evidence only, deterministic gates, reviewer sampling, and explicit reviewer limitations. This follows `P10` and `P12`.
- **§12 Graph Poisoning:** Embodied through privilege origins, `trust_state`, quarantine, provenance preservation, and privilege/poisoning controls. This follows `P1`, `P7`, `P10`, and `P15`.
- **§13 Cost and Resource Tails:** Embodied through **Cost as Correctness**, `BudgetLedger`, budget-gated optimizer/reviewer/worker behavior, and cache-prefix observability. This follows `P12`.
- **§14 Multi-Workstream Legibility:** Embodied through the single-tab `UserSurface`, initiative/status panes, global question queue, optimizer log, evidence drill-down, cost surface, and notification separation. This follows `P1`, `P13`, `P14`, and `P16`.
- **§15 Imposed Context Precedent Gap:** Embodied through graph-derived renders, visible imposed working sets, local graph canonicality, no in-product compaction, bounded foreground actions, and optimizer-owned curation. This follows `P1`, `P5`, `P9`, and `P15`.
- **§16 Recovery and Resume Surfaces:** Embodied through `RecoveryAction`, recovery preflight, explicit preserved/replayed/discarded records, failed-resume handling, audit events, and user-visible recovery surfaces. This follows `P14` and `P15`.

No new philosophical surfaces were found.
