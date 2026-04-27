# Product Philosophy

This document defines the principles and anti-goals that govern design decisions for `agent-harness`. Every design decision in the proposal must be traceable to one or more principles. When principles tension against each other, the tension must be surfaced and resolved — not silently resolved by the proposer.

---

## Principle 1: Imposed Context, Visible to the User

The harness's optimizer decides what the orchestrator sees on each turn. The orchestrator does not get to ignore that render or argue with it; it walks what the harness gives it. Imposing context is a feature, not a bug — it is what makes a long-lived multi-initiative orchestrator possible at all. But because the optimizer's authority is unilateral, every imposition must be visible to the user: which nodes are pinned this turn, which are unpacked, what the optimizer just changed, what evidence justified each summary, and what topology decisions were made on the user's behalf.

Anti-goal: a context-rewriting loop that silently shapes the orchestrator's beliefs. The optimizer may impose; it must not hide what it imposed.

---

## Principle 2: Working Set Is a Bounded Discipline, Not a Token Budget

Each rendered working set `W` is small, focused, and faithful — sized by the agent's effective reasoning capacity, not by the model's nominal context window. Render quality matters more than render volume. A render that fits but is full of distractors is a render that has failed.

Pack/unpack/focus tool calls, recursive-unpack depth, and eviction policy are first-class design surfaces, not implementation details. The harness pins what is essential, evicts what is not, and refuses to grow `W` past the point where the agent reasons reliably.

Anti-goal: maximizing how much context "fits." If `W` grows because there is room to grow, the architecture has lost the discipline that justified its existence.

---

## Principle 3: Summary Contract as a Hard Invariant

Every summary node enforces a contract that lets the agent decide whether to unpack. A contract specifies the fields a summary must contain (focus, decision status, uncertainty, evidence pointers, open blockers, stale markers, unpack affordances) and the predictability guarantee (what the summary says about a node faithfully predicts what unpacking will reveal). Contract conformance is checked by the optimizer and observable by the user.

Free-form summaries are not allowed at scale. Summary drift is the failure mode the contract exists to prevent. When a summary cannot be regenerated under the contract — because the underlying content is too divergent, contradictory, or evidence is missing — the contract surfaces that fact rather than producing a confident summary.

Anti-goal: prose-quality summarization without machine-checkable structure. Aesthetic summaries that drift silently are worse than ugly summaries that flag their own staleness.

---

## Principle 4: Stable Identity Decoupled from Summary Text and Parent Location

Every graph node has a stable identifier that survives summary regeneration, splits, merges, re-parents, and topology refactors. References (cross-references, question routes, worker slice assignments, walk history) name nodes by ID, not by current summary text or current parent path.

When the optimizer merges two nodes, both prior IDs forward to the merged identity until references catch up. When a node is split, the original ID resolves to a deterministic continuation chosen by the split mechanic. When a node is moved, its ID does not change. Identity drift is treated as a corruption, not a normal occurrence.

Anti-goal: identity-as-content-hash or identity-as-path. Either binds the ID to mutable state and breaks the invariant the harness depends on.

---

## Principle 5: Agent-Owned Focus, Optimizer-Owned Curation

The agent decides what to focus on by calling pack/unpack/focus tools. The optimizer decides what is in the graph at all — what gets summarized, cross-referenced, repacked, split, merged, re-parented. The two roles do not bleed: the optimizer does not directly steer the agent's attention; the agent does not directly rewrite the graph.

This division creates two distinct loops with two distinct accountabilities. The agent is accountable for its walk and its decisions inside `W`. The optimizer is accountable for the graph's fidelity and shape. Each loop's failures surface independently to the user.

Anti-goal: an optimizer that effectively forces agent attention by rewriting `W` into shapes that compel specific actions. This collapses two roles into one and removes the agent's accountability.

---

## Principle 6: Coordinated Concurrency, Not Race-Tolerant Concurrency

The optimizer mutates the graph while the agent walks it. This concurrency is essential — sleep-time-only optimization is too slow for live operation, and offline-only optimization defeats the architecture. But concurrent mutation requires explicit coordination: the harness commits to a documented semantic (snapshot-walk-then-merge, or another contract that gives the agent a stable view per turn), and the optimizer's writes are visible to the agent's next turn rather than appearing mid-thought.

Background mutation is not race-tolerant. The harness either acknowledges non-determinism by versioning what the agent sees, or it documents the coordination model that makes the agent's view stable across a turn boundary. Silent races between optimizer and agent are not acceptable.

Anti-goal: "experimental" concurrency where the agent's view changes mid-thought. If the system cannot give the agent a stable view per turn, it must say so loudly rather than rely on the user not to notice.

---

## Principle 7: Provenance Preserved Beside Summaries

Raw content is preserved separately from derived summaries. Tool outputs, sub-agent transcripts, file reads, search results, and pasted artifacts are stored as evidence nodes; summary nodes that reference them carry provenance pointers. Unpacking a summary surfaces its evidence on demand. The graph does not rely on its summaries to be lossless.

Provenance is not a parallel audit log. It is part of the graph contract: every summary derives from cited evidence; every cross-reference traces to evidence. When the optimizer regenerates a summary, the new summary's evidence pointers update; old evidence does not vanish from the graph.

Anti-goal: summaries that stand alone with no recoverable evidence. A summary the user cannot trace back is a summary the user cannot trust.

---

## Principle 8: Cross-CLI Adaptation, Not Lowest Common Denominator

The orchestrator runs on `claude`; sub-agents are dispatched across `claude`, `codex`, and `opencode` through `agents`. The harness uses each CLI's strongest available injection surface: rich hooks where they exist (Claude `UserPromptSubmit`, Codex hooks, opencode plugin transforms), MCP for portable tool/resource provision, wrapper-layer staging for launch and resume, and headless API for the cases where the harness must own the request entirely. Mechanism asymmetry is visible to the user as a per-CLI capability fingerprint, not hidden behind a uniform façade.

The harness does not degrade to MCP-only or wrapper-only because that would be uniformly weak. Nor does it pretend a Codex worker has Claude-level injection control when it does not.

Anti-goal: pretending the three CLIs are interchangeable. They are not. The architecture treats them as peers with different affordances and surfaces the differences rather than pretending they don't exist.

---

## Principle 9: No In-Product Compaction

The harness replaces `/compact` and equivalent in-product compaction mechanisms entirely. Compaction is not a baseline, fallback, or interoperable alternative. The graph is the harness's compaction story: imposed working set + pack/unpack + summary contracts + provenance + background optimizer.

When a CLI runs `/compact` autonomously (e.g., Claude Code's auto-compact at ~95% capacity), the harness either prevents it or treats the resulting state as corrupt. The harness does not engineer around `/compact`; it engineers so that `/compact` never fires.

Anti-goal: a hybrid where `/compact` and the harness's optimizer both operate on the same session. They produce contradictory state, and the resulting failure modes are not debuggable.

---

## Principle 10: Reviewer Evidence, Not Reviewer Ground Truth

Workflow-reviewer agents (gpt-high, per `~/ai/agents/workflow-reviewer.md`) check sub-agent process adherence and may flag optimizer edits, summary drift, or worker reintegration anomalies. Their output is one evidence channel, not authoritative judgment. The harness combines reviewer signals with non-LLM evidence: provenance diffs, schema validation, tool-call protocol checks, contract-format checks, and policy-violation rules.

Reviewers run on a sampling discipline appropriate to cost. Every-mutation review is rarely justified. On-completion-of-pipeline review with anomaly-triggered escalation is the default. False positives and false negatives are expected; the harness does not assume reviewer agreement equals correctness.

Anti-goal: treating reviewer approval as proof. A reviewer that has agreed cannot be the sole guardrail against poisoning, drift, or misroute.

---

## Principle 11: Questions Are Graph-State Continuations

Sub-agent `NEEDS_INPUT` returns become graph nodes (or pointers from graph nodes to question artifacts on disk per `~/ai/conventions/agent-questions-and-session-graph.md`). They preserve the worker session, the blocked output, the slice the worker held, the render that produced the question, and the correlation key needed to resume exact blocked work.

User answers are continuations of specific blocked work, not chat replies to the orchestrator. The harness routes them via `agents resume -m <model> --session-id <sid> -f <answer-payload>` (or the CLI-specific equivalent) and waits for child acceptance, not just the wrapper-recorded attempt.

Anti-goal: collapsing questions into the orchestrator's transcript so they look like normal conversation. The orchestrator may surface a question in its own render, but the question's authoritative state lives on the worker's session and the graph node that points at it.

---

## Principle 12: Cost as a Correctness Control

Graph maintenance must stay below the context savings it provides. Optimizer cadence, scope, and conflict-handling are tuned against measured token, latency, and cache-locality costs — not against intuition about how often summaries should regenerate. When the optimizer's cost on a representative workload exceeds tolerable bounds, the harness reduces optimizer activity rather than tolerating runaway cost.

Sub-agent dispatch, reviewer passes, and human-wait cache decay are budgeted explicitly. The user can see real-time cost per initiative and per optimizer pass, and the harness stops or warns when budgets are exceeded.

Anti-goal: optimizer features that look principled but explode cost on real workloads. Cost is correctness because an architecture that bankrupts the user is not actually working.

---

## Principle 13: Multi-Workstream Legibility for the User

The user sees the graph at a level that distinguishes active, blocked, recovering, stale, and waiting-for-user state. Initiative-rooted subgraphs are scannable. Blocked questions surface in a global queue, not buried inside the orchestrator's transcript. Optimizer edits are inspectable. The user can drill from initiative root → focus subgraph → unpacked node → evidence without losing place.

Notification volume separates action-needed (a worker question routed to the user) from passive progress (a summary regenerated, an initiative auto-paused). The user is not interrupted for FYI events; the user is interrupted for events that require their attention.

Anti-goal: collapsing the graph into a single chat scroll. Even though the orchestrator-as-UI is one tab, the underlying state is structured and the surface must reflect that structure.

---

## Principle 14: Recovery Boundaries Are Explicit

Every recovery action — resume, rollback, optimizer-edit revert, worker cancellation, session restoration — is an explicit, named operation with a documented effect on graph and session state. The harness shows the user what was preserved, what was replayed, and what was discarded. Silent recovery is forbidden; the user is told when a resume failed silently and a fresh agent was substituted.

The harness distinguishes deferred recording (the operation already happened; only the record is delayed — safe to retry) from deferred execution (the operation has not happened; preconditions may have changed — re-validation required). Cross-subsystem effects are classified by consequence at design time; high-consequence deferred effects require operator confirmation on retry, low-consequence effects auto-retry.

Anti-goal: invisible recovery. A user who does not know what was restored and what was lost cannot trust subsequent state.

---

## Principle 15: Local Control of Graph and Provenance

The context graph, evidence storage, and audit trail live under the user's filesystem (or a local SQLite the harness owns). Cross-references are stable and exportable. The graph's data model is documented well enough that a replacement harness could be built from the stored data without reverse-engineering. No critical state lives only inside a vendor's session store.

The harness ties into `agent-runner`'s SQLite (`~/.local/share/oulipoly-agent-runner/state.db`) for session ingestion and trace, but the harness's own graph state is its own source of truth. CLI session JSONL files are evidence sources, not the canonical graph.

Anti-goal: making the harness's value depend on a vendor whose disappearance would destroy the user's accumulated graph. Local control is what makes "very long running" actually possible — the user's history must outlast any one CLI's lifecycle.

---

## Principle 16: Single User, Single Tab, One Orchestrator

The harness serves one user with one long-lived orchestrator at a time. The UI is a single tab — no sidebar-of-many-chats, no multi-user collaboration, no shared workspaces. Multiple initiatives are managed by the one orchestrator walking many subgraphs, not by many orchestrators in many tabs.

This shapes scope. Multi-tenant features, real-time collaboration, project-level access control, and team dashboards are out of scope. The harness is a personal long-term thinking instrument, not a SaaS.

Anti-goal: feature creep toward team collaboration. A second user is a different product.

---

## Principle Interactions

Principles reinforce each other in predictable ways:

- **1 + 13 (imposed + legible)**: imposed context is acceptable only if it is visible. The user's trust in the optimizer depends on the user's ability to see and audit what the optimizer imposed. Legibility is what makes imposition tolerable.
- **2 + 3 (bounded + contract)**: a bounded working set is only useful if the summaries the agent walks are faithful. Contract enforcement (3) is what lets the agent trust that an unpacked node will not surprise it; bounded `W` (2) is what makes the contract feasible.
- **3 + 4 (contract + identity)**: summary contracts depend on stable identity. A summary that says "node X is the active initiative" is useless if X resolves to a different node next turn.
- **5 + 1 (agent-owned focus + imposed context)**: tension resolved at the boundary. The optimizer imposes the *shape* of `W`; the agent picks its *focus* within that shape via pack/unpack tools. Each loop has its own scope.
- **6 + 4 (coordination + identity)**: concurrent mutation is safe only if identity is preserved across the optimizer's writes. If a merge changes node IDs the agent just remembered, the agent's plan corrupts even with a snapshot-walk-then-merge contract.
- **7 + 3 (provenance + contract)**: provenance is what makes contract enforcement possible. The optimizer cannot regenerate a summary that conforms to the contract without traceable evidence to derive from.
- **8 + 11 (cross-CLI adaptation + question routing)**: cross-CLI mechanism asymmetry directly shapes how questions return. A worker on opencode emits questions through plugin-transformed messages; a worker on Claude returns through `Task`/`Agent` final messages; both must land as graph nodes via the same NEEDS_INPUT envelope, but the wire-level mechanism differs.
- **9 + 7 (no /compact + provenance)**: replacing `/compact` is only safe if the harness's own discipline produces equal or stronger preservation. Provenance pointers and explicit pack/unpack achieve this; lossy summarization without provenance does not.
- **10 + 12 (reviewer evidence + cost discipline)**: reviewer passes are expensive. Sampling rather than every-mutation review is what reconciles thoroughness with cost. The reviewer's role as evidence-not-ground-truth is what makes sampling tolerable.
- **13 + 14 (legibility + explicit recovery)**: a recovery surface is part of the legible state. The user sees not just "what is" but "what just changed and why" — including changes from recovery actions.
- **15 + 14 (local control + explicit recovery)**: local control is what makes recovery auditable. A graph stored in a vendor's session store cannot be inspected, rolled back, or replayed by the harness.
- **16 + 13 (single tab + legibility)**: single-tab does not mean single-render. A single tab can host an initiative grid, a focus pane, a blocked-questions queue, a cost surface, an optimizer-edit log — all backed by the same graph.
- **6 + 12 (coordination + cost)**: coordination semantics affect cost. A "lock the graph during agent turns" semantic is correct but expensive; a snapshot semantic is cheaper but requires versioning. The harness picks based on measured workload, not on which semantic sounds cleanest.

---

## Principle Tensions

Some principles can pull in different directions in specific contexts:

- **1 vs 5 (imposed context vs agent-owned focus)**: the optimizer imposes; the agent owns its attention within the imposition. The tension resolves at the granularity boundary — imposition is over the *shape and contents* of the graph, focus is over *which subset of the graph* is currently in `W`. If the optimizer imposes too tightly (forcing specific unpacks), it has crossed into the agent's role; if the agent reshapes nodes mid-turn (rather than just packing/unpacking), it has crossed into the optimizer's role.
- **6 vs 12 (concurrency vs cost)**: stronger coordination semantics (locks, snapshots, append-only with merge) are more expensive than weaker ones (none, optimistic with retry). The harness measures and picks the weakest semantic that holds the agent's view stable per turn — but not weaker.
- **10 vs 12 (reviewer everywhere vs cost)**: more review catches more violations and costs more. The harness samples reviewers (anomaly-triggered, on-completion, sampled-mutations) rather than running them over every graph edit. This trades coverage for cost; the user can tune the rate.
- **2 vs 3 (bounded `W` vs faithful summary)**: a tightly bounded `W` may force summaries to omit detail the agent later needs (premature pack). A loose `W` lets summaries be more permissive but reproduces context-bloat. The harness manages this through recursive-unpack depth controls and pinning policy: the agent unpacks deeper when reasoning warrants it, the optimizer pre-warms deep unpack when initiative root signals indicate the agent will need detail soon.
- **7 vs 12 (provenance vs cost)**: preserving raw content for every summary is expensive. The harness compresses provenance but does not delete it — old evidence is repacked deeper rather than discarded. Cost discipline applies to render and optimizer overhead; provenance integrity is a hard floor.
- **8 vs 16 (cross-CLI adaptation vs single-tab simplicity)**: per-CLI capability differences can leak into the user's mental model. The harness exposes a single "render quality fingerprint" per worker rather than per-mechanism details unless the user opts into deeper inspection.
- **9 vs platform reality**: the harness wants to suppress in-product `/compact`, but Claude Code's auto-compact at ~95% is a platform behavior the harness must engineer around — by keeping rendered prefixes short enough that auto-compact never fires, by using `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` to push the threshold, or by detecting and rejecting compacted state. The principle is upheld through prevention, not through fighting the platform.
- **13 vs 14 (legibility vs explicit recovery)**: surfacing every recovery action would create alert fatigue (PM-14 in v4 problem map: notifications and questions trade detection against attention). The harness distinguishes routine recovery (read-only, auto-acknowledged) from consequential recovery (consequence-graduated; consequential cases require user confirmation).
- **15 vs platform reality**: agent-runner already owns session ingestion in its SQLite; the harness's local-control principle does not mean re-implementing what agent-runner provides. It means the harness's graph and provenance are independent of agent-runner's session store, even though agent-runner is read for sub-agent trace.

---

## Anti-goals

These are things the system explicitly does not pursue:

- **General-purpose agent framework.** The harness is a single-user desktop orchestrator built on a specific architectural pattern. It is not a toolkit for building arbitrary agent applications.
- **Multi-tenant SaaS or web service.** No multi-user, no team dashboards, no shared workspaces. One user, one machine, one orchestrator.
- **Replacing `agent-runner`.** agent-runner is the substrate. agent-harness uses its CLI, session ingestion, trace, providers, and balancer. The harness extends, it does not duplicate.
- **`/compact` interoperability.** The harness replaces `/compact`; it does not engineer around it as an alternative.
- **AI-driven product strategy.** The harness uses AI to maintain its own working state, but product decisions (problem framing, roadmap, scope) are made by the user. The orchestrator presents options; it does not impose them.
- **Sidebar-of-many-chats UI.** Multiple initiatives are subgraphs walked by one orchestrator, not separate chats in tabs or panels. The single-tab orchestrator-as-UI shape is intentional, not provisional.
- **Infinite memory.** Hierarchical packing and unbounded depth do not mean infinite retention. Eviction, archival, and explicit user-initiated deletion are first-class.
- **Inventing context engineering for its own sake.** Every primitive (graph topology, summary contract, optimizer cadence) cites prior art or names its absence; nothing is novel because it sounds clever.

---

## Expansion Guidance

When the proposal encounters a problem surface not covered by these principles:

1. Check whether the surface is addressed by combining existing principles. Many specific design questions are resolved by applying two or three principles together.
2. If no combination of existing principles addresses the surface, identify the implicit principle and surface it through the philosophy alignment review process.
3. New principles are added through the expansion process, not by the proposer. The proposer flags the gap; the expansion agent evaluates whether the philosophy should grow.
