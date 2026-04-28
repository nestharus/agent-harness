# Product Problem Definition

This document defines the problem space for `agent-harness`: a desktop harness that gives a single user a long-lived, multi-CLI agent orchestrator. The orchestrator runs on `claude` and manages many concurrent initiatives; sub-agents are dispatched across `claude`, `codex`, and `opencode` through the existing `agents` CLI (agent-runner). The harness keeps the orchestrator on task across days or weeks by maintaining a persistent context graph of summary-pointing-to-detail nodes that the agent walks via pack/unpack tool calls, while a separate background optimizer continuously curates the graph.

The problems described here are not feature requests. Each section describes a core difficulty — why the problem is hard and resists solution — and the operational consequences of leaving it unaddressed. The harness is a desktop application built on the same Tauri / Bun / TanStack stack used elsewhere in this stack, but the difficulties below are not Tauri difficulties; they are difficulties of long-lived agentic operation under explicit graph-based context engineering.

---

## §1 Effective Working Set vs. Nominal Context Window

The underlying problem is reasoning quality under accumulated state, not transcript length. Long-context benchmarks show that nominal context windows overstate reliable working memory: retrieval, latent reasoning, multi-fact use, and middle-position recall degrade well before the advertised limit. Vendor docs corroborate this — long sessions can lose early instructions and degrade as context fills.

A context graph defers transcript saturation by externalizing detail, but the same failure mode reappears one layer up. Each turn renders a working set `W` from the graph; `W` must be small, focused, and faithful, or the agent suffers the same retrieval/reasoning degradation against a freshly assembled prompt. Rendering too much, rendering the wrong slice, or rendering many summaries that compete with each other reproduces the exact problem the graph was supposed to solve.

The core difficulty is that "fits in context" is not a useful success criterion for a long-lived orchestrator that juggles many initiatives. The orchestrator's effective working capacity is bounded by how much the model can actually reason over with fidelity, not by how many tokens fit. Without an explicit working-set discipline, every bounded `W` becomes a noisy `W`, and the value of the graph collapses.

---

## §2 Summary Contract: What Unpacking Must Faithfully Reveal

The deeper problem is that a summary node is only useful if its summary truthfully predicts what unpacking will reveal. Without a contract, summaries omit crucial detail (status, blockers, evidence pointers, uncertainty), become decoupled from the content they summarize as content evolves, and give the agent a misleading map. The agent then makes decisions on stale or speculative summary text and never bothers to unpack — the summary-only loop.

Surveyed prior art offers only partial summary contracts. GraphRAG community reports specify a strict template (title, executive summary, impact, grounded findings with data references). Generative Agents reflections require evidence pointers back to source observations. Cline Memory Bank uses role-specific files (current focus, system patterns, decisions, progress). None of these is a general-purpose contract for summary nodes in an agent's working-set graph; each was designed for its own retrieval shape.

The core difficulty is that a contract is not just a prose style. It defines what the renderer is allowed to show when detail is packed: focus, decision status, uncertainty, evidence pointers, open blockers, stale markers, and unpack affordances. Without those fields, the agent treats stale or speculative statements as settled work. With them, the optimizer at least has something to detect when summaries no longer match their content. Summary drift, broken cross-references, and loss-on-merge all arise from contract weakness, not from prose quality.

---

## §3 Concurrent Optimizer Mutation Under Foreground Walking

The actual problem is shared-state coordination between two distinct actors operating on the same graph: the foreground orchestrator that walks and packs/unpacks, and the background optimizer that updates summaries, cross-references, and topology. Both write. Their writes interleave with the agent's reasoning turns. The agent may unpack a node whose summary just changed, follow a cross-reference that was just rewritten, or rely on a topology that was just refactored.

Surveyed memory systems either avoid this problem (by being offline, like GraphRAG indexing) or acknowledge it as experimental (Letta sleep-time agents asynchronously modify shared blocks but the docs explicitly warn the area is unstable and provide no transaction semantics). No surveyed system documents coordination semantics for an optimizer mutating the same graph the agent is concurrently walking. The closest published patterns — locks, optimistic retry, append-only writes with delayed merge, snapshot-walk-then-merge — exist as primitives in databases and version control, not as agent-memory contracts.

The core difficulty is that the agent's view becomes non-deterministic between turns. The agent can rely on what it just wrote, but it cannot rely on what was true last turn after background mutation. Every optimizer edit is a potential invalidation of the agent's plan, its in-progress unpack/pack actions, the IDs it just remembered, or the assignments it gave to workers. Without a coordination semantic, the harness oscillates between two failure modes: too-eager optimization that destabilizes the agent mid-thought, or too-conservative optimization that lets the graph rot.

---

## §4 Stable Identity Across Summary Regeneration and Topology Reshape

The real problem is that pack/unpack, cross-references, question nodes, and worker subgraph slices all need durable identifiers that survive operations the optimizer routinely performs: regenerating a summary from updated content, splitting one node into two, merging two into one, re-parenting a node into a different subtree. If a node ID changes when its summary is regenerated, every reference to it breaks silently. If a merge does not preserve forwarding identity, every worker waiting to deliver back into the merged node gets routed to a node that no longer exists.

Prior art has identity primitives but not for this exact use case. Letta blocks use labels and unique block IDs. LangGraph's store keys identity by `(namespace, key)`. Org-mode's `ID` property survives heading moves. Zep/Graphiti performs entity resolution with temporal validity but routinely duplicates or mis-links entities under noise. Cline Memory Bank avoids the problem by using a flat, hand-curated file hierarchy. None covers continuous topology reshape on a graph the agent walks live.

The core difficulty is that identity is not a database key chosen once. It is a contract: the harness commits that the same logical bucket of work is reachable by the same identifier across summary rewrites, splits, merges, and re-parents, even when the prose text and the parent location change. Without this commitment, every optimizer pass introduces silent identity drift that corrupts question routing, worker reintegration, and the agent's own walk history.

---

## §5 Working-Set Policy: Pinning, Eviction, and Recursive Unpack

The problem is selecting what stays in `W` from one turn to the next, not retrieving relevant content once. The agent unpacks to gain detail and packs to relinquish it, but every turn the harness must decide which already-unpacked nodes survive, which get evicted, which become summary references, and which child summaries are revealed when a node is unpacked recursively. The policy can be implicit (whatever the agent last touched) or explicit (pinned roots plus a budget) but it cannot be absent.

Existing systems cover pieces. Pinned context appears in Letta memory blocks, Claude `CLAUDE.md`, and Cline core files. Retrieval-scored context appears in Generative Agents (recency × importance × relevance), HippoRAG (PageRank seeded by query entities), and LangGraph stores. User-selected context appears in Continue context providers. Budgeted map/reduce appears in GraphRAG. But no surveyed system publishes a general eviction or recursion policy for live graph nodes unpacked into an agent working set.

The core difficulty is that working-set policy interacts with every other graph problem. Aggressive eviction risks premature pack — packing detail the agent was about to need. Conservative pinning risks unpack flood — the working set bloats until reasoning quality collapses (§1). Recursive unpack without depth control reveals more than the agent asked for. None of these is a tuning concern; they shape whether the architecture works at all.

---

## §6 Hierarchical Packing Without Bounded-Depth Precedent

The deeper problem is unbounded-depth packing — nodes inside nodes — under live operation. The user's pattern says nodes can be packed within nodes when they are not currently relevant; the agent's surface area stays bounded even as the underlying graph grows. This is a load-bearing affordance for very-long-running agents because it lets old, deep, mostly-irrelevant context exist without taxing each turn's render.

Surveyed prior art offers only shallow versions of this. GraphRAG has hierarchical community summaries but the hierarchy is tree-shaped and bounded by cluster levels, not by agent navigation. Claude Code memory imports cap at depth 5 and load only the first 200 lines or 25 KB of `MEMORY.md`. Anthropic Skills enforce a 5 KB cap per skill and 25 KB total. Cline Memory Bank is shallow and hand-maintained. No system documents arbitrary nodes-inside-nodes with stable pack/unpack identity, bounded surface area, and lifecycle semantics for "deeply packed becomes barely visible."

The core difficulty is that hierarchy is itself state that the optimizer maintains and the agent walks. Every pack-into-deeper-node is a topological move with consequences for traversal, identity, and the agent's ability to find what it once knew. Without a published precedent, every choice (what depth to traverse on unpack, how summaries propagate to parents, how identity follows nested moves) is a design invention that has to be validated under live operation.

---

## §7 Cross-CLI Rendering Asymmetry

The real problem is that the orchestrator and its workers run on three different CLIs (claude, codex, opencode) with three different storage formats, three different injection surfaces, and three different lifecycle semantics. Claude stores append-only JSONL under `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` and exposes mature hooks (`UserPromptSubmit`, `SessionStart`, `PreToolUse`). Codex stores date-sharded rollout JSONL under `~/.codex/sessions/...` and offers similar hooks but partial in some tool-hook fields. Opencode persists sessions in SQLite tables (session, message, part, todo, permission) and offers a plugin system whose message/system transforms are powerful but marked experimental.

The harness renders the same graph node into a Claude lead turn, a Codex worker rollout, and an opencode worker session. The same `W` may need different prompt shape, different provenance compression, different tool affordances, and different focus-control instructions per CLI. MCP servers are portable across all three but only as resource/tool/prompt providers; MCP does not schedule per-turn insertion without a host-side hook or wrapper. Wrappers at the `agents` layer can control launch and resume but cannot mutate an already assembled provider request.

The core difficulty is that no single mechanism is uniformly strong across the three CLIs. A Claude-only mechanism (rich `UserPromptSubmit` injection) leaves worker context weak. A portable mechanism (MCP) is uniformly weak — it cannot impose context, only offer it. A mixed strategy works but introduces asymmetry: the lead orchestrator may have the strongest possible context discipline while a sub-agent on opencode has a weaker plugin-transformed render. The asymmetry must be visible and accounted for; otherwise the harness silently treats different CLIs as equivalent and the optimizer's intent decays unpredictably as work moves between them.

---

## §8 Sub-Agent Supervision via Subgraph Slices

The problem is delegation under bounded context with reintegration. Sub-agents must run in their own context window to keep parent context bounded; that isolation is what makes delegation worth doing. But final-response-only return (the dominant pattern in Anthropic's documented sub-agents) hides the trajectory the parent needs to supervise. With a context graph, delegation is no longer "send a prompt and receive a string." A worker receives a slice of subgraph as its working context, may request to unpack adjacent nodes, and produces output that has to be reintegrated as new nodes, summary updates, cross-references, or question nodes — not as one undifferentiated block of prose.

Cross-CLI makes this harder. Each worker's session lives in a vendor-specific store; a single harness-level "delegation" may resolve to a Claude UUID JSONL, a Codex rollout, or an opencode session row set. The harness must externalize sub-agent identity, slice-assignment provenance, and reintegration plan independently of any one CLI's session model.

The core difficulty is that supervision becomes a graph operation: deciding what slice a worker gets, watching how the worker interacts with its slice, and deciding how the worker's output should land in the graph. Done poorly, worker outputs bloat the graph with low-fidelity summaries, lose evidence on reintegration, or drift the parent's belief about what was actually accomplished. Done with insufficient discipline, two parallel workers operating on overlapping slices write conflicting updates into the same nodes.

---

## §9 User-Question Routing as Graph-State Routing

The deeper problem is that a sub-agent question is not chat text; it is a blocker attached to a specific subgraph, a specific blocked output, a specific worker session, and a specific render that produced the question. When the user answers, the answer must resume the exact blocked work — not append to the lead transcript, not get re-parsed by a fresh agent that has lost the worker's local context.

Surveyed multi-agent frameworks all converge on the need for durable correlation: LangGraph's `Interrupt(value, id)` plus checkpointers, OpenAI Assistants' thread/run/tool-call IDs, OpenAI Agents SDK serialized `RunState`, AutoGen's `UserInputRequestedEvent.request_id` plus team state, ADK's function-call IDs and event replay, Claude Code's pending-tool-call deferral (limited to single-tool-call non-interactive turns), and MCP Elicitation as a protocol-native ask-user artifact. They diverge on what state survives the pause: parallel interrupts may resume only partially, active runs may lock the thread, checkpoints may replay node code and produce duplicate side effects, terminal markers preserved in logs may instantly re-terminate a resumed group chat.

The core difficulty is that questions become graph nodes (or pointers from graph nodes to question artifacts), and answers are continuations of specific blocked work, not chat replies. If questions are not graph-addressable, the orchestrator cannot manage many simultaneous initiatives or know which packed node to reopen when the user responds. If continuations are not protocol-aware, the user's answer applies to the wrong paused task, replays side effects, or terminates a resume that was supposed to continue.

---

## §10 Tool-Call Protocol State as First-Class Graph Provenance

The problem is that tool calls and tool results are not ordinary prose; they are protocol artifacts with exact-correlation requirements. Wrong API loops, hallucinated APIs, repeated calls, orphan function calls, rejected calls represented as completed, and missing tool-result blocks all corrupt later resume protocols (Assistants requires exact `tool_call_id` match; OpenAI Agents SDK approvals preserve pending tool approvals by tool-call ID; Claude Code deferral joins on session plus deferred tool ID/name/input). When tool state corrupts, summaries that say "the test failed because X" without preserving the tool call, output, timestamp, and correlation key become unverifiable on later unpack.

Cross-CLI multiplies this. Claude/Codex/opencode each expose different hook and tool-interception semantics. The harness cannot assume one common tool-transcript shape across the three.

The core difficulty is that pack/unpack cannot collapse tool protocol state into ordinary prose. Pending tool calls, approvals, failures, generated artifacts, and exact correlation IDs need graph provenance that survives focus changes, summary regeneration, sub-agent handoff, and cross-CLI worker boundaries. Provenance is not an audit log laid alongside the graph; it is part of the graph contract — raw evidence preserved separately from derived summaries, addressable from any node that derives from it.

---

## §11 Workflow-Reviewer Reliability for Graph Mutations

The actual problem is that a workflow-reviewer agent — used in this architecture to verify that workers followed their declared workflow and to audit optimizer edits, summary contracts, provenance links, topology changes, and worker reintegration — is itself a fallible LLM. LLM-as-judge studies document position, verbosity, and self-preference biases; step-error localization is weak (GPT-4 reaches only 52.87% on BIG-Bench Mistake with direct step-level prompting); long traces hide relevant evidence behind the same long-context effects the agent suffers; misleading reviewer feedback can destabilize correct answers (large drops are documented under deceptive judges). Frameworks themselves report bug clusters around unexpected intermediate state and unexpected execution sequence in trace/state systems the reviewer would inspect.

No primary source measures false-positive or false-negative rates for an LLM reviewer auditing process adherence over real long, tool-heavy traces. The closest published analogue (Agent-as-a-Judge over DevAI) reports 90% human agreement but evaluates development-task requirements over artifacts, not declared runbook adherence.

The core difficulty is that the reviewer cannot be the only protection against bad optimizer edits, summary drift, malicious or stale nodes, or out-of-policy worker reintegration. A reviewer can sample mutations and flag candidate violations; it cannot be ground truth. The harness must combine reviewer signals with non-LLM evidence (provenance diffs, schema validation, tool-call protocol checks, contract-format checks, policy violation rules) and must treat reviewer agreement as evidence, not proof.

---

## §12 Instruction Hierarchy and Memory Poisoning Becoming Graph Poisoning

The deeper problem is that the orchestrator runs on a model that does not strictly obey privilege boundaries between system, user, and tool text. Documented instruction-hierarchy work shows models treating these layers at similar priority, enabling prompt injection from lower-privilege tool output. Memory layers compound this: stored hallucinated content becomes durable fact, scope drift applies memory to the wrong project/session/entity, pre-prompt drift lets injected rules accumulate and lose relevance.

The graph raises the severity. The optimizer has write authority over what lands in `W` next turn. If the optimizer promotes poisoned content into a pinned summary, an initiative root, or a high-traffic cross-reference, that content becomes imposed context — not optional memory the agent can ignore. Summary-only loops amplify the risk: the agent acts on visible summaries without unpacking the suspect evidence behind them.

The core difficulty is that the optimizer is a privileged actor whose edits reach the agent's reasoning surface unilaterally. Trust in the optimizer is therefore trust in (a) its scope (which subgraphs it may touch), (b) its provenance (every edit links to the evidence that justifies it), (c) its privilege boundaries (it must distinguish system/user/tool/skill content as it integrates), and (d) its reversibility (an edit must be inspectable and undoable). Without these, the optimizer is a vector for poisoning that the agent cannot defend against in-stream.

---

## §13 Cost, Latency, and Resource Tails of Graph-Walking Workloads

The problem is that the architecture combines several cost actors that compound. Parent context creates a recurring per-turn cost slope; prompt caching reduces input cost roughly ten-fold but only when prefix stability and TTL hold; human waits cross cache-retention windows; sub-agents multiply total tokens (Anthropic's published research-system finding: multi-agent ≈ 15× chat tokens, agent teams ≈ 7× standard-session tokens); reviewer passes multiply by trace length and candidate count; hook/MCP/plugin choices each add per-turn latency.

Graph maintenance adds another cost actor: summary regeneration, cross-reference discovery, stale detection, repacking, topology refactor, provenance repair, and conflict checks. Letta sleep-time agents are token-consuming and explicitly experimental. GraphRAG community refresh is offline-expensive. Continuous optimizer cadence is not free.

The core difficulty is that a context graph can reduce render size only if its maintenance cost stays below the context savings it provides. A graph that triggers expensive summarization, reviewer checks, or topology edits on every turn can dominate cost. Cache locality may also suffer if the optimizer rewrites prefix content — every render becomes a cache miss. Cost is therefore a correctness control, not a billing concern: an architecture that cannot stay within tolerable cost and latency tails on real workloads is unusable regardless of its other merits.

---

## §14 Multi-Workstream Legibility for the Agent and the User

The real problem is two simultaneous demands: the agent must walk many initiative-rooted subgraphs without losing track of what is active, blocked, recovering, or stale; the user must see enough of the orchestrator's state to know which initiatives are progressing, which are waiting, which need an answer, and which are consuming resources. Both demands operate on the same underlying graph but require different views of it.

Prior agent-product UX evidence (Cursor background agents, Devin sessions, Sourcegraph Amp threads, GitHub Copilot Coding Agent, Replit Agent, Goose, Codex Cloud, JetBrains Junie, Cline) overwhelmingly externalizes concurrency into separate workstream identities — task lists, status badges, blocked filters, drill-down logs, notification routing, checkpoint timelines. Coarse states like `RUNNING` flatten "making progress," "stuck," "waiting on tools," and "needs user input." Single-tab orchestrator-as-UI with multiple initiatives in one window has thin direct evidence in the surveyed products.

The core difficulty is that the graph is the multi-initiative model — initiatives become roots or named subgraphs — but the harness still needs a scannable user surface that does not collapse blockers, optimizer edits, or stale nodes into one undifferentiated transcript. Notification volume must distinguish action-needed (a worker question routed to the user) from passive progress (a summary regenerated, an initiative auto-paused), or alert fatigue buries blockers and the user stops trusting the surface.

---

## §15 Imposed Working Context Without Surveyed Precedent

The deeper problem is that the user's intended pattern is structurally different from existing memory-layer products. Letta, MemGPT, Anthropic's memory tool, Goose, Cline, Mem0, and OpenAI Assistants `file_search` all put memory *outside* the active conversation; the model retrieves, searches, or tool-calls into it. The model is in charge of what enters its working context. The surveyed memory layers are *consulted* by the model.

The harness's pattern *imposes* the working set: the optimizer decides what the agent sees each turn. The agent has pack/unpack tools, but those tools navigate a graph the optimizer maintains; the agent does not own the working set the way it owns its tool selection. Combined with the four pillars (concurrent optimizer mutating the walked graph, agent-owned pack/unpack tools, unbounded hierarchical packing, continuous topology reshape), this composite has no surveyed prior art. The closest fragments — Letta sleep-time agents, MemGPT paging, GraphRAG hierarchy, Generative Agents reflection citations, Cline role contracts, Org-mode stable IDs, Anthropic Skills progressive disclosure — solve different problems individually.

The core difficulty is that imposed context demands stronger guarantees than consulted memory. With consulted memory, a bad recall is the agent's problem to recover from; the agent can re-query, re-rank, or ignore. With imposed context, a bad render is the harness's responsibility — the agent has no recourse mid-turn. That increases fidelity requirements, audit requirements, scope requirements, recovery requirements, and provenance requirements simultaneously. It also means the failure modes that show up in production will be ones no surveyed system has encountered, because no surveyed system runs this architecture in production.

---

## §16 Recovery and Resume Surfaces Are Not Neutral

The actual problem is that every recovery action — resume, rollback, optimizer-edit revert, worker cancellation, session restoration — changes state and can erase, replay, misattribute, or fail. Frameworks document this directly: LangGraph resumes by re-entering nodes and may replay side effects; AutoGen save/load can produce inconsistent or immediate-stop behavior at the wrong time; ADK resume is at-least-once and loses in-memory state; Claude resume has documented loader strictness fixes around large/error sessions, dead-end branches, and self-references; opencode session import preserves IDs but raw DB hot edits are unsupported and schema-validated. Wrapper-layer resume can target a session but child acceptance is not guaranteed by the wrapper's own record of the attempt.

In a context-graph architecture, recovery crosses additional boundaries: the optimizer's edit history, the working set's pack/unpack stack, sub-agent slice assignments, blocked-question routes, and tool-call protocol state. A recovery that looks neutral at the CLI layer may corrupt a graph invariant — a question node points to a worker session that was reset, a subgraph slice references nodes that were merged during the outage, a pinned summary references evidence that was repacked deeper.

The core difficulty is that recovery boundaries must be explicit and auditable. The harness must distinguish "the operation already happened, only the record is delayed" (safe to retry) from "the operation has not happened, preconditions may have changed" (re-validation required). It must distinguish optimizer edits that survive recovery from those that should be discarded. It must show the user what was preserved, what was replayed, and what was discarded — silently restoring "what looked like a session" without surfacing those distinctions trains the user not to trust recovery and not to trust the underlying state.

---

## §17 Graph and Memory Configuration Overhead

The underlying problem is configuration-to-usable-context translation, not the discoverability of setup controls. Schema-rich graph, memory, and context systems ask the user to define how work should be represented before the agent has demonstrated that the representation is useful: which objects exist, which fields matter, how memory is promoted, how embeddings or indexes are built, how context providers are scoped, and which relationships should affect retrieval or rendering. The burden appears before multi-workstream legibility exists; the user is still trying to make the substrate predictable enough to trust.

Surveyed market evidence shows this shape outside this proposal. Tana users report unclear AI behavior around supertags and fields. Cognee acknowledges that its graph-memory model can feel complicated. Obsidian AI plugin users encounter setup, reliability, and large-vault embedding costs. Mem.ai's promised self-organization can collapse back into manual tagging. Continue.dev context gathering can run long without explaining what is being collected. These are not complaints about one interface polish layer; they are symptoms of systems whose power depends on configuration the user cannot easily validate.

The core difficulty is that configuration is part of the memory semantics. A bad schema, stale embedding, over-broad context provider, or mis-scoped memory rule does not merely make setup unpleasant; it changes what the agent can see and what the user believes the agent should know. The resulting failures are ambiguous: the agent may be wrong because the graph is empty, because the graph is shaped badly, because indexing is incomplete, because the wrong provider was consulted, or because the user and system disagree about what a configured field means. Without a way to make that pre-operational burden legible, the graph can become technically powerful but operationally unusable.

---

## §18 Provider, Account, and Entitlement Friction

The real problem is provider-stack state, not cross-CLI prompt rendering. A local or hybrid agent stack that spans providers inherits authentication stores, billing accounts, model entitlements, local runtime availability, provider-specific feature matrices, network restrictions, and sandbox boundaries. "Provider choice" is therefore not just a model-selection affordance; it expands the state space of permissions, capabilities, failure modes, and user expectations that the harness must operate within.

Surveyed market evidence shows the same friction across products that do not use this proposal's design. Goose users hit Copilot authentication failures in WSL. JetBrains Junie users ask for local model support and object to restrictive credit systems. Codex CLI users encounter network and sandbox failures through extension surfaces. Heptabase users request OpenAI-compatible provider support. Agent SDK users run into MCP incompatibility with Gemini models and ask for Claude skill support in Google ADK. The recurring pattern is that multi-provider operation exposes account, entitlement, and feature-boundary problems that are not reducible to rendering the same context differently for different CLIs.

The core difficulty is that provider failures are often indistinguishable at the orchestration layer until the user has already lost trust. A request can fail because a token was not stored, a model is unavailable on the account, a feature is provider-specific, a local runtime is not installed, a sandbox blocks the network, or a billing limit was reached. Each provider adds not only another backend but another policy surface, support matrix, and recovery path. If those boundaries are invisible, the harness treats provider interchangeability as real when it is only nominal, and work fails in ways that look arbitrary to both the agent and the user.
