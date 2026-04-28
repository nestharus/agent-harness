# Executive Roadmap

## Scoring Methodology

**Pain severity (1-5):** 5 means strong market signal plus active operational harm; 4 means strong signal or moderate signal plus active harm; 3 means moderate market signal with a stable gap; 2 means weak signal or future risk; 1 means speculative value.
**Competitive position:** gap = no analyzed competitor offers this capability; parity = one or two competitors offer a related capability with known limitations; table-stakes = three or more competitors offer related capability and absence would make the harness non-viable.
**Dependency depth:** upstream value-slice count required before the slice can function. Phase 0 foundations do not count.
**Operational risk of delay:** active-harm = operators are harmed while the gap remains; static-gap = stable workaround exists; future-risk = harm appears as usage grows.
**Composite formula:** `pain * competitive_weight * delay_risk_weight / (depth + 1)`, where gap = 1.5, parity = 1.0, table-stakes = 2.0, active-harm = 1.5, static-gap = 1.0, future-risk = 0.7.

The roadmap floats cost observability and provider/configuration legibility earlier than their dependency depth might otherwise suggest. `P12` says cost is correctness, and `D17`/`D18` are load-bearing market surfaces: if render budgets, cache-prefix effects, provider readiness, and configuration semantics are invisible, later optimizer and worker slices cannot be evaluated.

## Value Slice Inventory

### VS-001: Inspect imposed working-set renders

**Subsystem(s):** Context Graph & Provenance; Orchestrator; Cost & Budget; User Surface; Cross-CLI Adaptation.
**Capability delivered:** The user and orchestrator can see the exact graph snapshot, working-set nodes, summaries, evidence pointers, token estimate, cache prefix hash, and provider/capability route imposed on a lead turn.
**Schema objects:** `GraphSnapshot`, `WorkingSetSnapshot`, `GraphNode`, `GraphEdge`, `SummaryContract`, `ProvenancePointer`, `OrchestratorTurn`, `BudgetLedger`, `CapabilityFingerprint`, `ProviderState`, `AuditEvent`.
**State transitions:** Orchestrator `idle -> snapshotting -> rendering`; render `within budget -> launching_or_resuming` or `blocked`; UI inspection over last/current `WorkingSetSnapshot`.
**Anti-scope:** Does not include pack/unpack/focus tools, optimizer edits, worker dispatch, routed questions, or recovery actions.

### VS-002: Enforce summary contracts on visible nodes

**Subsystem(s):** Context Graph & Provenance; Workflow Review & Governance; User Surface; Configuration as Memory Semantics.
**Capability delivered:** Packed nodes render only through contract-valid summaries that expose status, uncertainty, blockers, stale markers, evidence pointers, and unpack affordances.
**Schema objects:** `SummaryContract`, `GraphNode`, `NodeRevision`, `EvidenceArtifact`, `ProvenancePointer`, `GraphConfiguration`, `PolicySet`, `AuditEvent`.
**State transitions:** Summary validation `valid`, `invalid_missing_evidence`, `invalid_conflict`, `invalid_stale`, `invalid_policy`, `needs_review`; render blocks or labels invalid summaries.
**Anti-scope:** Does not regenerate summaries, discover new cross-references, repair provenance, or change graph topology.

### VS-003: Capture tool-call provenance and audit events

**Subsystem(s):** Context Graph & Provenance; Orchestrator; Workflow Review & Governance; Recovery.
**Capability delivered:** Tool calls, approvals, tool results, side-effect class, correlation keys, and evidence artifacts are preserved as first-class provenance that summaries and recovery can cite.
**Schema objects:** `ToolCallProvenance`, `EvidenceArtifact`, `ProvenancePointer`, `GraphAction`, `OrchestratorTurn`, `PolicySet`, `AuditEvent`, `RecoveryAction`.
**State transitions:** Tool call `requested -> approved -> executed -> result_recorded`, or `failed`, `orphaned`, `reconciled`; `GraphAction` `draft -> committed` or `rejected`.
**Anti-scope:** Does not authorize topology mutation, worker reintegration, reviewer sampling, or recovery replay.

### VS-004: Gate renders with budget ledgers and cache locality

**Subsystem(s):** Cost & Budget; Orchestrator; Continuous Optimizer; Sub-Agent Dispatch; User Surface.
**Capability delivered:** The harness records token, latency, cost, budget state, policy action, and cache prefix hash for renders, turns, workers, optimizer passes, and reviewer passes.
**Schema objects:** `BudgetLedger`, `WorkingSetSnapshot`, `OrchestratorTurn`, `WorkerRun`, `OptimizerEdit`, `PolicySet`, `ProviderState`, `AuditEvent`.
**State transitions:** Budget `within -> near_limit -> exceeded -> blocked`; policy action `none`, `warn`, `narrow_scope`, `require_user_approval`, `block`.
**Anti-scope:** Does not build provider billing import, pricing prediction, optimizer scheduling, or worker launch by itself.

### VS-005: Inspect configuration as memory semantics

**Subsystem(s):** Context Graph & Provenance; Continuous Optimizer; Workflow Review & Governance; User Surface; Cross-CLI Adaptation.
**Capability delivered:** The user can inspect effective graph/memory/render/provider configuration, value sources, empty-graph simulation, shape explanations, and validation warnings before those settings affect graph truth.
**Schema objects:** `GraphConfiguration`, `GraphWorkspace`, `GraphSnapshot`, `WorkingSetSnapshot`, `PolicySet`, `OptimizerRequest`, `AuditEvent`.
**State transitions:** Configuration `resolved -> empty_graph_simulated -> shape_explained -> validated`, `warning`, `invalid`, or `superseded`.
**Anti-scope:** Does not auto-rewrite graph shape, regenerate summaries, repair indexes, or choose provider routes.

### VS-006: Preflight providers and expose capability fingerprints

**Subsystem(s):** Cross-CLI Adaptation; Sub-Agent Dispatch; Orchestrator; Cost & Budget; Recovery; User Surface.
**Capability delivered:** The harness can tell the user and orchestrator which CLI/provider/model routes are eligible, degraded, blocked, or unknown for a workload, with explicit denial reasons.
**Schema objects:** `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, `WorkingSetSnapshot`, `WorkerSlice`, `WorkerRun`, `RecoveryAction`, `AuditEvent`.
**State transitions:** Provider route `unprobed -> probing -> ready`, `degraded`, `blocked`, `running`, `failed_mid_run`, `revalidating`; capability route `eligible`, `eligible_with_warnings`, `degraded`, `blocked`, `unknown`.
**Anti-scope:** Does not store credentials, repair vendor accounts, dispatch workers, or perform recovery reroutes.

### VS-007: Show initiative roots and current focus

**Subsystem(s):** User Surface; Orchestrator; Context Graph & Provenance.
**Capability delivered:** The single-tab UI shows initiative roots, active focus path, pins, unpacked nodes, evictions, blockers, stale/recovering states, and action-needed versus passive-progress notifications.
**Schema objects:** `GraphNode`, `GraphEdge`, `AgentWalkState`, `WorkingSetSnapshot`, `QuestionArtifact`, `WorkerSlice`, `OptimizerEdit`, `RecoveryAction`, `AuditEvent`.
**State transitions:** Node lifecycle `active`, `packed`, `unpacked`, `blocked`, `stale`, `recovering`, `archived`, `quarantined`; notification class `action_needed` or `passive_progress`.
**Anti-scope:** Does not include worker dispatch controls, question answering, recovery execution, cost drill-down, or optimizer edit approval.

### VS-008: Navigate with pack, unpack, focus, pin, and unpin

**Subsystem(s):** Orchestrator; Context Graph & Provenance; Cost & Budget; Workflow Review & Governance.
**Capability delivered:** The orchestrator can change its focus and working-set shape through bounded graph-walk tools whose effects are visible on the next render.
**Schema objects:** `AgentWalkState`, `WorkingSetSnapshot`, `GraphSnapshot`, `GraphAction`, `GraphNode`, `GraphEdge`, `BudgetLedger`, `AuditEvent`.
**State transitions:** Walk tool `requested -> validated -> applied_to_walk_state -> rendered_next_turn`, or `denied_policy`, `denied_budget`, `denied_conflict`; `AgentWalkState` `idle`, `rendering`, `thinking`, `blocked`.
**Anti-scope:** Does not mutate graph topology, summaries, cross-references, evidence, identity, or worker assignments.

### VS-009: Run bounded orchestrator turns with advisory optimizer requests

**Subsystem(s):** Orchestrator; Context Graph & Provenance; Continuous Optimizer; Workflow Review & Governance; Cost & Budget; Cross-CLI Adaptation.
**Capability delivered:** The lead orchestrator can complete durable turns, record bounded foreground graph actions, and request optimizer consideration without directly curating graph truth.
**Schema objects:** `OrchestratorTurn`, `GraphAction`, `OptimizerRequest`, `WorkingSetSnapshot`, `GraphSnapshot`, `ToolCallProvenance`, `BudgetLedger`, `AuditEvent`.
**State transitions:** Orchestrator `rendering -> launching_or_resuming -> thinking -> tool_pending/capturing -> committing -> optimizer_enqueue -> complete`, or `blocked`/`recovering`; `OptimizerRequest` `queued -> accepted_for_scoping`, `ignored`, `superseded`, or `converted_to_optimizer_edit`.
**Anti-scope:** Does not include actual optimizer edits, worker dispatch, question routing, or recovery execution.

### VS-010: Refresh summaries and mark stale nodes

**Subsystem(s):** Continuous Optimizer; Context Graph & Provenance; Workflow Review & Governance; Cost & Budget; User Surface.
**Capability delivered:** The optimizer can draft budgeted summary regenerations and stale markers against a snapshot, and the user can see accepted, rejected, or conflicted edits in the optimizer log.
**Schema objects:** `OptimizerRequest`, `OptimizerEdit`, `SummaryContract`, `NodeRevision`, `EvidenceArtifact`, `ProvenancePointer`, `BudgetLedger`, `AuditEvent`.
**State transitions:** Optimizer `queued -> scoped -> snapshot_read -> drafting -> schema_validation -> contract_validation -> policy_validation -> merge_attempt -> merged`, `rejected`, or `conflicted`.
**Anti-scope:** Does not split, merge, re-parent, repack, discover cross-references, quarantine poisoned nodes, or reintegrate workers.

### VS-011: Simulate configuration effects and request shape repair

**Subsystem(s):** Continuous Optimizer; Context Graph & Provenance; Workflow Review & Governance; User Surface.
**Capability delivered:** Configuration warnings can create advisory optimizer requests when current graph shape, summary templates, index scopes, or render policy appear inconsistent with effective memory semantics.
**Schema objects:** `GraphConfiguration`, `OptimizerRequest`, `OptimizerEdit`, `GraphSnapshot`, `SummaryContract`, `AuditEvent`.
**State transitions:** Configuration `warning -> advisory OptimizerRequest queued`; optimizer request `accepted_for_scoping`, `ignored`, or `converted_to_optimizer_edit`.
**Anti-scope:** Does not silently rewrite graph truth, add new schema concepts, or replace explicit user configuration.

### VS-012: Repack hierarchy and discover cross-references

**Subsystem(s):** Continuous Optimizer; Context Graph & Provenance; Orchestrator; User Surface.
**Capability delivered:** The optimizer can propose cross-references and bounded hierarchy repacks so old or adjacent detail remains findable without flooding the current working set.
**Schema objects:** `OptimizerEdit`, `GraphEdge`, `GraphNode`, `NodeRevision`, `IdentityEvent`, `SummaryContract`, `ProvenancePointer`, `ConflictRecord`, `AuditEvent`.
**State transitions:** Optimizer edit types `cross_reference`, `repack`, `split`, `merge`, `reparent`; edge states `candidate -> active`, `rejected`, `stale`, `archived`.
**Anti-scope:** Does not run worker reintegration, mutate the current orchestrator turn, or bypass conflict handling for identity/topology changes.

### VS-013: Resolve snapshot-merge conflicts and identity forwarding

**Subsystem(s):** Continuous Optimizer; Context Graph & Provenance; Recovery; Workflow Review & Governance; User Surface.
**Capability delivered:** Optimizer edits, foreground actions, and later reintegration can fail closed into explicit conflicts with affected nodes, identity events, and resolution state.
**Schema objects:** `ConflictRecord`, `IdentityEvent`, `OptimizerEdit`, `GraphSnapshot`, `NodeRevision`, `GraphEdge`, `RecoveryAction`, `AuditEvent`.
**State transitions:** Merge `merge_attempt -> merged` or `conflicted`; conflict `open -> auto_resolved`, `needs_orchestrator`, `needs_user`, `rejected`, or `superseded`; identity `split`, `merged`, `forwarded`, `restored`.
**Anti-scope:** Does not decide arbitrary conflict policy for every future worker type, execute recovery, or expose multi-user collaboration controls.

### VS-014: Quarantine poisoned or privilege-unsafe graph content

**Subsystem(s):** Workflow Review & Governance; Context Graph & Provenance; Continuous Optimizer; User Surface.
**Capability delivered:** Lower-privilege or suspect content can be labeled, blocked from instruction promotion, quarantined in renders, and made traceable to evidence and policy decisions.
**Schema objects:** `GraphNode`, `NodeRevision`, `EvidenceArtifact`, `ProvenancePointer`, `SummaryContract`, `OptimizerEdit`, `PolicySet`, `AuditEvent`.
**State transitions:** Trust `trusted -> derived -> unverified -> suspect -> poison_quarantined`; optimizer edit `poison_quarantine`; render blocks traversal into quarantined/deleted/conflicted nodes without explicit tool action.
**Anti-scope:** Does not make reviewers authoritative, delete evidence, or provide enterprise team access-control features.

### VS-015: Dispatch provider-aware worker slices

**Subsystem(s):** Sub-Agent Dispatch & Reintegration; Cross-CLI Adaptation; Cost & Budget; Orchestrator; Context Graph & Provenance.
**Capability delivered:** The orchestrator can launch bounded sub-agent slices through `agents` with graph context, write scope, provider preflight, budget state, and acceptance tracking.
**Schema objects:** `WorkerSlice`, `WorkerRun`, `GraphSnapshot`, `WorkingSetSnapshot`, `CapabilityFingerprint`, `ProviderState`, `EntitlementSnapshot`, `BudgetLedger`, `AuditEvent`.
**State transitions:** Worker `slice_draft -> slice_validated -> launched -> accepted -> running`, or `failed`, `cancelled`, `blocked`; provider route `ready/degraded/blocked`.
**Anti-scope:** Does not reintegrate output as graph truth, route user questions, sample reviewers, or perform recovery substitution.

### VS-016: Show the worker board and trace evidence

**Subsystem(s):** User Surface; Sub-Agent Dispatch & Reintegration; Context Graph & Provenance; Cross-CLI Adaptation; Cost & Budget.
**Capability delivered:** The user can see worker slices, sessions, accepted/rejected status, blocked/running/completed state, capability fingerprints, budget state, and trace evidence.
**Schema objects:** `WorkerSlice`, `WorkerRun`, `EvidenceArtifact`, `ToolCallProvenance`, `BudgetLedger`, `CapabilityFingerprint`, `AuditEvent`.
**State transitions:** Worker run `starting -> running -> blocked/completed/failed/lost/cancelled`; acceptance `unknown -> accepted`, `rejected`, `timed_out`, `ambiguous`.
**Anti-scope:** Does not answer questions, merge worker output, cancel/recover workers, or resolve provider failures.

### VS-017: Route NEEDS_INPUT answers as worker continuations

**Subsystem(s):** NEEDS_INPUT Routing; Sub-Agent Dispatch & Reintegration; Recovery; User Surface; Cross-CLI Adaptation.
**Capability delivered:** Worker questions appear in a global action-needed queue and user answers resume the exact blocked worker continuation with child-acceptance tracking.
**Schema objects:** `QuestionArtifact`, `WorkerSlice`, `WorkerRun`, `WorkingSetSnapshot`, `GraphNode`, `GraphEdge`, `RecoveryAction`, `AuditEvent`.
**State transitions:** Question `created -> validated -> presented -> answered -> resume_requested -> child_accepted -> resumed`, or `failed_resume`, `superseded`, `cancelled`.
**Anti-scope:** Does not append answers to the lead transcript as a substitute, resolve graph conflicts, or fresh-substitute workers after failed resume.

### VS-018: Stage worker output for graph reintegration

**Subsystem(s):** Sub-Agent Dispatch & Reintegration; Continuous Optimizer; Context Graph & Provenance; Workflow Review & Governance.
**Capability delivered:** Worker outputs become staged graph candidates, evidence, blockers, questions, conflicts, and advisory optimizer requests rather than unverified final-response paste.
**Schema objects:** `WorkerSlice`, `WorkerRun`, `EvidenceArtifact`, `NodeRevision`, `OptimizerRequest`, `ConflictRecord`, `ToolCallProvenance`, `AuditEvent`.
**State transitions:** Worker `completed -> reintegrating -> integrated` or `conflicted`; optimizer request `queued -> converted_to_optimizer_edit` or `ignored`; conflict `open -> needs_user/needs_orchestrator/auto_resolved`.
**Anti-scope:** Does not let workers directly mutate topology, summaries, identity, cross-references, quarantine state, or current orchestrator focus.

### VS-019: Sample reviewers over high-consequence edits

**Subsystem(s):** Workflow Review & Governance; Continuous Optimizer; Sub-Agent Dispatch & Reintegration; Cost & Budget; User Surface.
**Capability delivered:** High-consequence optimizer edits, worker reintegration plans, provider anomalies, and configuration-shape anomalies can be reviewed as an evidence channel under a cost-aware sampling policy.
**Schema objects:** `PolicySet`, `OptimizerEdit`, `WorkerSlice`, `ConflictRecord`, `BudgetLedger`, `ProvenancePointer`, `AuditEvent`.
**State transitions:** Reviewer `not_required -> sampled_pending -> passed`, `flagged`, or `inconclusive`; edit `validated -> review_sampling -> merge_attempt` or `conflicted`.
**Anti-scope:** Does not make reviewer approval sufficient for merge, review every mutation, or replace deterministic gates.

### VS-020: Surface recovery preflight and state accounting

**Subsystem(s):** Recovery; Context Graph & Provenance; Sub-Agent Dispatch & Reintegration; NEEDS_INPUT Routing; Cost & Budget; User Surface; Cross-CLI Adaptation.
**Capability delivered:** The user can see recovery actions with preconditions, affected graph/session/provider state, side-effect class, and preserved/replayed/discarded records.
**Schema objects:** `RecoveryAction`, `GraphSnapshot`, `ConflictRecord`, `QuestionArtifact`, `WorkerRun`, `ProviderState`, `EntitlementSnapshot`, `ToolCallProvenance`, `AuditEvent`.
**State transitions:** Recovery `detected -> classified -> preflight -> requires_user/applying -> reconciled`, `partial`, or `failed`; recovery result `planned`, `applied`, `failed`, `partially_applied`, `reverted`.
**Anti-scope:** Does not silently resume, substitute workers, or reroute providers without exposing changed execution contracts.

### VS-021: Reroute or substitute failed provider/worker runs explicitly

**Subsystem(s):** Recovery; Cross-CLI Adaptation; Sub-Agent Dispatch & Reintegration; Cost & Budget; User Surface.
**Capability delivered:** Failed provider routes, entitlement changes, sandbox failures, and lost workers can be rerouted or substituted only after capability, cost, side-effect, and user-confirmation checks are visible.
**Schema objects:** `RecoveryAction`, `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, `WorkerSlice`, `WorkerRun`, `BudgetLedger`, `AuditEvent`.
**State transitions:** Provider `failed_mid_run -> revalidating -> ready/degraded/blocked`; recovery actions `resume_session`, `fresh_worker_substitution`, `cancel_worker`, `quarantine_session`; confirmation `not_required`, `required`, `granted`, `denied`.
**Anti-scope:** Does not repair vendor accounts, hide degraded execution contracts, or claim a fresh worker is a successful resume.

## Phase Structure

### Phase 0: Foundations (engineering-defined)

Placeholder: the engineering layer determines what technical foundations are needed before value slices can begin. This phase may include storage, eventing, migrations, local process boundaries, testing substrate, and app shell wiring, but those are not detailed here because they are not independently operator-visible value slices.

### Phase 1: Observable Imposed Context

**Capability gained:** The harness can render graph-backed working context, validate visible summaries, preserve tool evidence, expose budget/cache behavior, and show configuration/provider readiness before the lead agent or user trusts long-running state.
**Workflows enabled:** Inspect the current imposed working set; see whether a summary is valid; trace a tool result to evidence; see whether a render is over budget; diagnose empty/thin context as configuration-driven; know which provider routes are ready or blocked.
**Still missing:** Agent-owned navigation, durable orchestrator turn loop, optimizer mutation, worker dispatch, question continuations, conflict resolution, reviewer sampling, and recovery execution.
**Prerequisites:** none.

#### VS-001: Inspect imposed working-set renders

- **Pain severity:** 5 — long sessions degrade or fail with strong evidence in OpenCode, Claude Code, GitHub cloud agent, and Claude Code recovery-loop complaints; context selection also has cost/latency complaints in Aider and Continue (`market-research.md`, Orchestrator user pain signals).
- **Competitive position:** gap — many products expose sessions or tasks, but the synthesis says competitors rarely expose exact rendered context and graph walk state as first-class artifacts (`market-research.md`, Orchestrator differentiation opportunities; Evidence Gaps, exact imposed working-set precedent).
- **Dependency depth:** 0 — depends only on Phase 0 foundations.
- **Operational risk of delay:** active-harm — without explicit `W`, bounded context becomes noisy and the graph collapses into the same failure mode as long transcripts (`problem.md` §1 and §15).
- **Composite priority:** 11.25.
- **Opportunity cost:** By not building this yet, the user cannot tell what context was imposed on the orchestrator, so failures from noisy or wrong renders remain indistinguishable from model failure (`problem.md` §1, §15).

#### VS-002: Enforce summary contracts on visible nodes

- **Pain severity:** 4 — context compression and memory loss are visible in coding agents, and memory quality/write policy issues appear in Mem0, Zep/Graphiti, and Letta with moderate evidence (`market-research.md`, Context Graph & Provenance user pain signals).
- **Competitive position:** parity — GraphRAG, Generative Agents, Cline Memory Bank, and memory products offer partial summary or memory structure, but none provides the proposal's general-purpose contract for packed graph nodes (`market-research.md`, Context Graph & Provenance competitive landscape and differentiation opportunities).
- **Dependency depth:** 0 — contract validation can operate on graph records before optimizer regeneration exists.
- **Operational risk of delay:** active-harm — stale or speculative summary text causes the agent to make decisions without unpacking evidence (`problem.md` §2).
- **Composite priority:** 6.00.
- **Opportunity cost:** By not building this yet, the user continues to face summary-only loops where packed nodes look authoritative even when they omit blockers, uncertainty, or evidence (`problem.md` §2).

#### VS-003: Capture tool-call provenance and audit events

- **Pain severity:** 5 — tool-call integrity is a recurrent strong failure mode across CrewAI, Vercel AI SDK, and OpenAI Agents SDK; permissions/approval failures also recur in Cline, Goose, and Claude SDK (`market-research.md`, Workflow Review & Governance user pain signals).
- **Competitive position:** table-stakes — guardrails, approvals, tracing, audit logs, and observability are common across SDKs, local CLIs, and enterprise tiers (`market-research.md`, Workflow Review & Governance competitive landscape and value indicators).
- **Dependency depth:** 0 — provenance capture can begin with orchestrator/tool evidence before optimizer or workers depend on it.
- **Operational risk of delay:** active-harm — orphaned or lossy tool state corrupts later resume, summaries, and protocol-specific continuations (`problem.md` §10).
- **Composite priority:** 15.00.
- **Opportunity cost:** By not building this yet, the harness cannot verify tool claims or preserve exact tool-call IDs, approvals, failures, and side effects for later unpack or recovery (`problem.md` §10).

#### VS-004: Gate renders with budget ledgers and cache locality

- **Pain severity:** 5 — pricing units fail when agent effort varies with strong evidence from Cursor pricing changes, Replit effort pricing, and Codex cloud usage accounting; users also complain about background agents burning budgets (`market-research.md`, Cost & Budget user pain signals).
- **Competitive position:** table-stakes — many competitors expose credits, ACUs, premium requests, active-agent pricing, token usage, or usage rails (`market-research.md`, Cost & Budget competitive landscape).
- **Dependency depth:** 0 — budget instrumentation must exist before later optimizer, reviewer, and worker behavior can be measured.
- **Operational risk of delay:** active-harm — unmeasured graph maintenance can exceed the savings from bounded context and make the architecture unusable (`problem.md` §13).
- **Composite priority:** 15.00.
- **Opportunity cost:** By not building this yet, the user cannot know whether renders, optimizer passes, reviewers, or workers are consuming more cost than the graph saves (`problem.md` §13).

#### VS-005: Inspect configuration as memory semantics

- **Pain severity:** 4 — configuration overhead appears in Tana, Cognee, Obsidian AI plugins, Mem.ai, and Continue, with the synthesis identifying it as a load-bearing market surface (`problem.md` §17; `market-research.md`, Surface Testing Summary).
- **Competitive position:** table-stakes — three or more memory/KM/local context products expose related setup and configuration burden, even if none gives the proposal's full semantic inspector (`problem.md` §17; `market-research.md`, AI knowledge management and memory/context lanes).
- **Dependency depth:** 0 — configuration inspection can run before graph truth is rewritten.
- **Operational risk of delay:** active-harm — bad schema, stale index scope, or mis-scoped memory rules change what the agent can see and make failures ambiguous (`problem.md` §17).
- **Composite priority:** 12.00.
- **Opportunity cost:** By not building this yet, the user cannot distinguish an empty graph from a badly shaped graph, incomplete index, bad default, or misconfigured provider route (`problem.md` §17).

#### VS-006: Preflight providers and expose capability fingerprints

- **Pain severity:** 4 — provider/auth/setup friction appears in Goose Copilot auth, Junie local-model requests, Codex network/sandbox failures, Heptabase provider requests, ADK Claude-skill requests, and Gemini MCP incompatibility (`market-research.md`, Cross-CLI Adaptation user pain signals).
- **Competitive position:** table-stakes — provider choice and local control are repeated purchase/adoption signals across Cline, Aider, Goose, OpenCode, Zed, Obsidian, Logseq, Capacities, and Heptabase (`market-research.md`, Cross-CLI Adaptation value indicators).
- **Dependency depth:** 0 — preflight can run before launch or reroute workflows exist.
- **Operational risk of delay:** active-harm — provider failures look arbitrary when auth, entitlement, network, runtime, sandbox, quota, and feature boundaries are invisible (`problem.md` §18).
- **Composite priority:** 12.00.
- **Opportunity cost:** By not building this yet, the harness treats provider interchangeability as real when it is only nominal, causing routing failures that the user and orchestrator cannot interpret (`problem.md` §18).

#### VS-007: Show initiative roots and current focus

- **Pain severity:** 4 — users need legibility into progress, blockers, and cost; the synthesis cites Cursor spend-limit UI bugs, GitHub failed long sessions, Replit rollback/memory loss, and Augment notification requests (`market-research.md`, User Surface user pain signals).
- **Competitive position:** table-stakes — task boards, session lists, status badges, logs, branches, PRs, checkpoints, and follow-up surfaces are table stakes for cloud agents (`market-research.md`, User Surface value indicators).
- **Dependency depth:** 0 — initiative/focus status can render from graph and working-set state once Phase 0 foundations exist.
- **Operational risk of delay:** active-harm — the user cannot distinguish active, blocked, stale, recovering, waiting-for-worker, and waiting-for-user initiatives in one long-lived orchestrator (`problem.md` §14).
- **Composite priority:** 12.00.
- **Opportunity cost:** By not building this yet, multiple initiatives collapse into one undifferentiated transcript and blockers become easy to miss (`problem.md` §14).

### Phase 2: Lead Orchestrator Loop and Initial Curation

**Capability gained:** The lead orchestrator can walk the graph with bounded focus tools, complete durable turns, and hand curation requests to an optimizer that refreshes summaries and stale state under budget and configuration visibility.
**Workflows enabled:** Focus an initiative; unpack detail; pack detail back to contract summary; complete an orchestrator turn; record bounded foreground actions; queue optimizer requests; see summary refreshes and stale markers in the optimizer log.
**Still missing:** Configuration-shape repair requests, deep topology refactors, identity conflict resolution, poisoning quarantine, worker dispatch, question routing, reviewer sampling, and recovery execution.
**Prerequisites:** Phase 1.

#### VS-008: Navigate with pack, unpack, focus, pin, and unpin

- **Pain severity:** 5 — long sessions degrade or fail with strong evidence, and foreground context selection has cost/latency complaints in Continue and Aider (`market-research.md`, Orchestrator user pain signals).
- **Competitive position:** gap — memory products mostly expose consulted memory, while the proposal uses imposed graph navigation with pack/unpack tools and no surveyed exact precedent (`market-research.md`, Evidence Gaps, exact imposed working-set precedent).
- **Dependency depth:** 3 — upstream slices: VS-001 inspect renders, VS-002 summary contracts, VS-004 budget gates.
- **Operational risk of delay:** active-harm — without explicit pinning, eviction, and recursive unpack discipline, working set bloat or premature packing reproduces context-window failure (`problem.md` §5 and §6).
- **Composite priority:** 2.81.
- **Opportunity cost:** By not building this yet, the orchestrator cannot deliberately trade summary view for detail view and remains dependent on whatever context was initially rendered (`problem.md` §5).

#### VS-009: Run bounded orchestrator turns with advisory optimizer requests

- **Pain severity:** 5 — local agent loops have strong adoption and long-running failures; goal drift is a recognized framework pain in AutoGen, CrewAI, and Cline (`market-research.md`, Orchestrator user pain and value indicators).
- **Competitive position:** table-stakes — sessions, tasks, plans, local agent loops, and cloud work units are central competitive surfaces across local and cloud agents (`market-research.md`, Orchestrator competitive landscape and value indicators).
- **Dependency depth:** 4 — upstream slices: VS-001 inspect renders, VS-003 tool provenance, VS-004 budget gates, VS-006 provider preflight.
- **Operational risk of delay:** active-harm — foreground turns that can directly mutate topology would race the optimizer; turns without durability lose the trace needed for graph continuity (`problem.md` §3, §4, §10).
- **Composite priority:** 3.00.
- **Opportunity cost:** By not building this yet, the user has an inspectable render but no durable lead-agent loop that can advance initiatives while preserving graph boundaries (`problem.md` §3 and §10).

#### VS-010: Refresh summaries and mark stale nodes

- **Pain severity:** 4 — background memory generation needs approval or quality controls, and state mutation contracts are hard in LangGraph and Agno; optimizer cost is also visible in DSPy and pricing analysis (`market-research.md`, Continuous Optimizer user pain signals).
- **Competitive position:** parity — Letta, Cursor Memories, Cognee, ADK, and Zep/Graphiti provide related memory or summarization passes, but not an auditable live graph-maintenance loop (`market-research.md`, Continuous Optimizer competitive landscape and differentiation opportunities).
- **Dependency depth:** 4 — upstream slices: VS-002 summary contracts, VS-003 provenance capture, VS-004 budget gates, VS-005 configuration inspector.
- **Operational risk of delay:** active-harm — summary drift, stale markers, and missing evidence accumulate while the foreground agent keeps walking old graph state (`problem.md` §2 and §3).
- **Composite priority:** 1.20.
- **Opportunity cost:** By not building this yet, packed nodes will decay and the user cannot tell whether a summary still predicts what unpacking will reveal (`problem.md` §2).

### Phase 3: Topology Safety and Graph Integrity

**Capability gained:** The optimizer can turn configuration warnings into bounded curation requests and change graph shape without silently breaking identity, references, privilege boundaries, or user trust.
**Workflows enabled:** Create configuration-shape repair requests; create cross-references; repack deep context; split/merge/reparent with forwarding records; surface merge conflicts; quarantine suspect content; block traversal into poisoned or unresolved nodes.
**Still missing:** Worker dispatch, question continuations, worker reintegration, reviewer sampling across reintegration, and recovery actions.
**Prerequisites:** Phase 2.

#### VS-011: Simulate configuration effects and request shape repair

- **Pain severity:** 4 — graph/memory configuration overhead is a load-bearing market surface, with related pain in Tana, Cognee, Obsidian AI plugins, Mem.ai, and Continue (`problem.md` §17; `market-research.md`, Surface Testing Summary).
- **Competitive position:** gap — competitors show the pain and related configuration surfaces, but the synthesis does not identify a product that turns configuration semantics into graph-addressed optimizer requests and audit state (`market-research.md`, Evidence Gaps; User Surface differentiation opportunities).
- **Dependency depth:** 2 — upstream slices: VS-005 configuration inspector, VS-010 summary/stale optimizer.
- **Operational risk of delay:** static-gap — the graph can operate with defaults, but bad configuration quietly changes memory semantics and makes later failures expensive to attribute (`problem.md` §17).
- **Composite priority:** 2.00.
- **Opportunity cost:** By not building this yet, configuration warnings remain informational and cannot trigger bounded graph-shape review when current state contradicts declared memory semantics (`problem.md` §17).

#### VS-012: Repack hierarchy and discover cross-references

- **Pain severity:** 3 — continuous topology refactor has weak direct evidence, but memory/context products validate persistent context and graph/vector retrieval while the exact live mechanism is thin (`market-research.md`, Continuous Optimizer value indicators and Evidence Gaps).
- **Competitive position:** gap — no surveyed product documents arbitrary nodes-inside-nodes with stable pack/unpack identity and live topology reshape (`problem.md` §6; `market-research.md`, Evidence Gaps, continuous optimizer merge semantics).
- **Dependency depth:** 2 — upstream slices: VS-008 navigation, VS-010 summary/stale optimizer.
- **Operational risk of delay:** static-gap — old and adjacent context can remain accessible through manual unpack, but topology rot and missing cross-references increase as the graph grows (`problem.md` §6).
- **Composite priority:** 1.50.
- **Opportunity cost:** By not building this yet, deep context either stays too visible and bloats `W` or becomes too hidden for the orchestrator to rediscover reliably (`problem.md` §6).

#### VS-013: Resolve snapshot-merge conflicts and identity forwarding

- **Pain severity:** 4 — state mutation contracts are hard in LangGraph and Agno, while worker conflicts are expected in Replit and subagent products; evidence is moderate but tied to active harm (`market-research.md`, Continuous Optimizer and Sub-Agent Dispatch user pain signals).
- **Competitive position:** gap — the synthesis identifies limited direct market evidence for snapshot-walk-then-merge or continuous topology refactor on a graph a foreground agent is actively walking (`market-research.md`, Evidence Gaps).
- **Dependency depth:** 2 — upstream slices: VS-009 bounded orchestrator turns, VS-010 optimizer summary/stale edits.
- **Operational risk of delay:** active-harm — without identity forwarding and conflict records, optimizer edits can invalidate plans, pack/unpack references, question routes, and worker assignments (`problem.md` §3 and §4).
- **Composite priority:** 3.00.
- **Opportunity cost:** By not building this yet, every optimizer or later worker graph mutation risks silent identity drift or inconsistent foreground state (`problem.md` §3 and §4).

#### VS-014: Quarantine poisoned or privilege-unsafe graph content

- **Pain severity:** 4 — prompt/tool misuse, unauthorized changes, permission prompt failures, and durable memory risk appear across CrewAI, Vercel/OpenAI, Cline, Goose, ChatGPT Agent/Operator, Mem0, and Cursor Memories (`market-research.md`, Cross-cutting Findings, Privilege and Poisoning Controls).
- **Competitive position:** table-stakes — guardrails, permissions, approvals, high-impact confirmations, and prompt-injection controls are common across analyzed agent products, though graph poisoning is a proposal-specific improvement (`market-research.md`, Workflow Review & Governance competitive landscape).
- **Dependency depth:** 3 — upstream slices: VS-002 summary contracts, VS-003 provenance capture, VS-010 optimizer edits.
- **Operational risk of delay:** active-harm — optimizer-promoted poisoned content becomes imposed context the agent sees next turn (`problem.md` §12).
- **Composite priority:** 3.00.
- **Opportunity cost:** By not building this yet, lower-privilege tool or worker text can be summarized into high-traffic graph context without quarantine or visible trust state (`problem.md` §12).

### Phase 4: Worker Slice Dispatch

**Capability gained:** The orchestrator can delegate bounded graph slices to workers across CLIs with provider, budget, write-scope, and acceptance constraints.
**Workflows enabled:** Launch a worker with a graph slice; deny routes that lack required provider features; record child session acceptance; keep parent context bounded while delegated work starts.
**Still missing:** Worker board detail, routed questions, staged reintegration, reviewer sampling, and recovery/reroute execution.
**Prerequisites:** Phase 3.

#### VS-015: Dispatch provider-aware worker slices

- **Pain severity:** 5 — parallel agents are a core sales feature across Devin, Cursor, Codex, Replit, GitHub Copilot, Augment, Goose, Claude Code, and Cline, and worker isolation/reintegration is weakly surfaced (`market-research.md`, Sub-Agent Dispatch value indicators and user pain signals).
- **Competitive position:** table-stakes — background agents, subagents, tasks, handoffs, teams, crews, and multi-agent workflows appear across many cloud, local, and SDK products (`market-research.md`, Sub-Agent Dispatch competitive landscape).
- **Dependency depth:** 4 — upstream slices: VS-006 provider preflight, VS-004 budget gates, VS-009 bounded orchestrator turns, VS-007 initiative/focus surface.
- **Operational risk of delay:** active-harm — without delegation, the single orchestrator must carry all work in one bounded context, undermining the multi-initiative value proposition (`problem.md` §8 and §14).
- **Composite priority:** 3.00.
- **Opportunity cost:** By not building this yet, the orchestrator cannot keep parent context bounded by sending isolated slices to workers (`problem.md` §8).

### Phase 5: Worker Legibility and Reintegration

**Capability gained:** Delegated work becomes legible and graph-addressed: the user can inspect worker state, route worker questions, and stage completed output for curation instead of pasting final prose into graph truth.
**Workflows enabled:** See worker acceptance and run state; inspect worker evidence and budget; answer worker questions as continuations; stage worker outputs as evidence, blockers, conflicts, and optimizer requests.
**Still missing:** Reviewer sampling over high-consequence reintegration and full recovery/reroute execution.
**Prerequisites:** Phase 4.

#### VS-016: Show the worker board and trace evidence

- **Pain severity:** 4 — users need legibility into progress, blockers, and cost, and task boards/session lists/status badges are table stakes in cloud agents (`market-research.md`, User Surface user pain and value indicators).
- **Competitive position:** table-stakes — task boards, progress tabs, multi-session panes, agent threads, logs, and status surfaces are common across cloud and local products (`market-research.md`, User Surface competitive landscape).
- **Dependency depth:** 1 — upstream slice: VS-015 worker dispatch.
- **Operational risk of delay:** active-harm — hidden worker state turns parallel work into opaque background activity and buries blocked or failed slices (`problem.md` §8 and §14).
- **Composite priority:** 6.00.
- **Opportunity cost:** By not building this yet, the user cannot tell which workers are accepted, running, blocked, failed, or consuming budget (`problem.md` §14).

#### VS-017: Route NEEDS_INPUT answers as worker continuations

- **Pain severity:** 4 — HITL is missing or high-priority in OpenAI and Claude SDK issues, mid-task correction is uneven in Codex/GitHub/Devin/Cursor, and resume state is fragile in AutoGen/OpenAI Agents (`market-research.md`, NEEDS_INPUT Routing user pain signals).
- **Competitive position:** table-stakes — HITL/approval appears across at least five frameworks and several cloud/local products, even though cross-CLI graph-state question routing is unique (`market-research.md`, NEEDS_INPUT Routing value indicators).
- **Dependency depth:** 2 — upstream slices: VS-015 worker dispatch, VS-006 provider preflight.
- **Operational risk of delay:** active-harm — unanswered or misrouted worker questions block work or resume the wrong task (`problem.md` §9 and §16).
- **Composite priority:** 4.00.
- **Opportunity cost:** By not building this yet, user answers remain ordinary chat text and cannot reliably resume the exact blocked worker state (`problem.md` §9).

#### VS-018: Stage worker output for graph reintegration

- **Pain severity:** 4 — worker isolation and reintegration are weakly surfaced, external permissions fail, and fabricated/simulated tool use appears in CrewAI and Cline complaints (`market-research.md`, Sub-Agent Dispatch user pain signals).
- **Competitive position:** parity — Devin and Replit have coordinator/conflict/application concepts, but the proposal's graph-slice reintegration with evidence and optimizer requests is more explicit (`market-research.md`, Sub-Agent Dispatch competitive landscape and differentiation opportunities).
- **Dependency depth:** 4 — upstream slices: VS-015 worker dispatch, VS-010 optimizer summary/stale edits, VS-013 conflict records, VS-003 tool provenance.
- **Operational risk of delay:** active-harm — final-response-only return hides the worker trajectory and can bloat or corrupt the graph on reintegration (`problem.md` §8 and §10).
- **Composite priority:** 1.20.
- **Opportunity cost:** By not building this yet, worker outputs remain unverified prose rather than graph-addressed evidence, updates, blockers, conflicts, and optimizer requests (`problem.md` §8).

### Phase 6: Governance Sampling and Recovery Accounting

**Capability gained:** High-consequence changes are reviewed as evidence under budget discipline, and failures become explicit recovery actions with preserved/replayed/discarded accounting.
**Workflows enabled:** Sample reviewers on risky edits; flag reintegration anomalies; preflight recovery; classify deferred recording versus deferred execution; restore questions; resume sessions; cancel workers; show partial or failed recovery.
**Still missing:** Provider-aware reroute and fresh worker substitution after recovery accounting.
**Prerequisites:** Phase 5.

#### VS-019: Sample reviewers over high-consequence edits

- **Pain severity:** 4 — tool-call integrity failures are strong, while governance, audit logs, guardrails, and observability are monetized control-plane values across many products (`market-research.md`, Workflow Review & Governance user pain and value indicators).
- **Competitive position:** parity — competitors expose guardrails and observability, but the proposal differs by treating LLM reviewer output as evidence rather than ground truth (`market-research.md`, Workflow Review & Governance differentiation opportunities; Evidence Gaps, Workflow reviewer reliability).
- **Dependency depth:** 4 — upstream slices: VS-010 optimizer edits, VS-018 worker reintegration staging, VS-003 provenance capture, VS-004 budget gates.
- **Operational risk of delay:** active-harm — deterministic gates catch structure, but high-consequence graph changes can still be wrong or misleading without sampled process review (`problem.md` §11).
- **Composite priority:** 1.20.
- **Opportunity cost:** By not building this yet, risky optimizer and worker-reintegration changes lack a reviewer evidence channel, leaving only deterministic checks where process adherence matters (`problem.md` §11).

#### VS-020: Surface recovery preflight and state accounting

- **Pain severity:** 4 — recovery can fail or lose state in Replit, OpenCode, and Continue; long-running cloud work can fail expensively in GitHub Copilot and Codex (`market-research.md`, Recovery user pain signals).
- **Competitive position:** table-stakes — checkpoints, rollback, resume, fork/export, retry, time travel, and durable execution are visible product features across at least six products/frameworks (`market-research.md`, Recovery value indicators).
- **Dependency depth:** 5 — upstream slices: VS-003 provenance capture, VS-006 provider preflight, VS-013 conflicts/identity, VS-017 routed questions, VS-018 reintegration staging.
- **Operational risk of delay:** active-harm — recovery can erase, replay, misattribute, or fail across sessions, graph state, questions, workers, and tool calls (`problem.md` §16).
- **Composite priority:** 2.00.
- **Opportunity cost:** By not building this yet, resume, rollback, cancellation, and failed continuation remain trust-damaging events with no preserved/replayed/discarded accounting (`problem.md` §16).

### Phase 7: Provider-Aware Recovery Closure

**Capability gained:** Recovery can complete by changing provider or worker routes only when the changed execution contract is explicit and accepted.
**Workflows enabled:** Revalidate provider state after failure; compare alternate capability fingerprints; reroute or fresh-substitute a worker; show changed model, feature, runtime, sandbox, cost, or quota contract.
**Still missing:** Nothing from the proposal is strategically deferred; later engineering roadmaps may split these slices further.
**Prerequisites:** Phase 6.

#### VS-021: Reroute or substitute failed provider/worker runs explicitly

- **Pain severity:** 4 — provider/auth/setup friction and feature parity pressure recur across Goose, Junie, Codex, Heptabase, ADK, Inngest, and Continue (`market-research.md`, Cross-CLI Adaptation user pain signals).
- **Competitive position:** parity — many products support provider choice, but few expose provider-aware recovery that names changed execution contracts; the proposal improves on common provider routing surfaces (`market-research.md`, Cross-CLI Adaptation differentiation opportunities).
- **Dependency depth:** 3 — upstream slices: VS-006 provider preflight, VS-015 worker dispatch, VS-020 recovery accounting.
- **Operational risk of delay:** active-harm — provider failures mid-run look arbitrary and fresh worker substitution can be mistaken for successful resume unless contract changes are explicit (`problem.md` §18 and §16).
- **Composite priority:** 1.50.
- **Opportunity cost:** By not building this yet, auth, billing, quota, entitlement, runtime, network, and sandbox failures cannot be recovered through an auditable route change (`problem.md` §18).

## Opportunity Cost Summary

| Initiative | Opportunity Cost |
|---|---|
| Imposed working-set visibility | Without inspectable renders, the user cannot know whether failure came from model reasoning, wrong context, overfull context, or optimizer-imposed context (`problem.md` §1, §15). |
| Summary contract and provenance | Without contract-valid summaries and evidence pointers, packed nodes can drift, omit blockers, or become trusted without unpackable support (`problem.md` §2, §10). |
| Cost as correctness | Without budget ledgers and cache-prefix visibility, optimizer, reviewer, worker, and render cost can exceed the value of the context graph (`problem.md` §13). |
| Configuration as memory semantics | Without configuration inspection and shape warnings, schema, index, render, and memory-policy choices silently change what the agent can see (`problem.md` §17). |
| Provider state as observable state | Without provider preflight and capability fingerprints, account, entitlement, feature, runtime, network, quota, and sandbox failures look arbitrary (`problem.md` §18). |
| Multi-workstream user surface | Without initiative roots, focus state, worker board, question queue, optimizer log, cost, and recovery surfaces, active/blocking/recovering/stale states collapse into one transcript (`problem.md` §14). |
| Agent-owned graph navigation | Without pack/unpack/focus tools, the orchestrator cannot deliberately manage pinning, eviction, recursive unpack, or bounded detail (`problem.md` §5, §6). |
| Bounded orchestrator turn lifecycle | Without durable turns and advisory-only foreground graph actions, foreground reasoning can race curation or lose the trace needed for graph continuity (`problem.md` §3, §10). |
| Continuous optimizer | Without summary refresh, stale markers, cross-references, repacking, and topology edits, the graph rots or bloats as initiatives accumulate (`problem.md` §2, §3, §6). |
| Identity and conflict governance | Without identity forwarding and conflict records, splits, merges, re-parents, worker assignments, and question routes break silently (`problem.md` §3, §4). |
| Privilege and poisoning controls | Without privilege-aware quarantine, lower-privilege tool or worker content can become imposed context and steer the orchestrator (`problem.md` §12). |
| Worker slice dispatch | Without bounded sub-agent slices, the parent orchestrator must carry all work in one context window and cannot supervise parallel work as graph state (`problem.md` §8). |
| NEEDS_INPUT routing | Without graph-state continuations, user answers can resume the wrong worker, append to the wrong transcript, or fail without visible child acceptance (`problem.md` §9, §16). |
| Worker reintegration | Without staged reintegration, worker outputs arrive as unverified prose and lose evidence, conflicts, blockers, and cross-reference intent (`problem.md` §8, §10). |
| Reviewer sampling | Without sampled reviewer evidence, high-consequence optimizer and reintegration changes depend only on deterministic structure checks despite known LLM-reviewer and trace-inspection limits (`problem.md` §11). |
| Recovery and reroute | Without explicit recovery actions, preserved/replayed/discarded records, and provider-aware reroute, failures can erase, replay, misattribute, or silently substitute work (`problem.md` §16, §18). |

## Deferred Subsystems

No proposal subsystem is strategically deferred out of the roadmap. Every one of the ten subsystems appears in at least one value slice:

- Context Graph & Provenance: VS-001, VS-002, VS-003, VS-012, VS-013.
- Orchestrator: VS-001, VS-008, VS-009.
- Continuous Optimizer: VS-010, VS-011, VS-012, VS-013.
- Sub-Agent Dispatch & Reintegration: VS-015, VS-016, VS-018.
- NEEDS_INPUT Routing: VS-017.
- Workflow Review & Governance: VS-002, VS-003, VS-014, VS-019.
- Cost & Budget: VS-004 and budget gates across VS-015, VS-019, VS-021.
- Recovery: VS-020, VS-021.
- User Surface: VS-001, VS-005, VS-006, VS-007, VS-016, VS-017, VS-020.
- Cross-CLI Adaptation: VS-006, VS-015, VS-017, VS-021.

The only deferral is within Phase 0: engineering foundations are acknowledged but intentionally left to the engineering roadmap because they do not themselves deliver operator-visible Tauri UI behavior or an orchestrator-visible capability.
