# Market Research Synthesis

## Methodology

This synthesis uses the six pre-gathered Stage 0b market files in `market-data/` plus `proposal.md`, `problem.md`, and `philosophy.md`. No new web research was performed. The market files cover roughly 14 cloud-agent products, 13 local/desktop/IDE assistants, 14 agent SDKs/frameworks, 14 memory/context products plus database-positioned memory infrastructure, 16 AI-augmented KM products, and a cross-product pricing/business-model analysis.

Evidence strength follows the operator contract: **strong** means 5+ independent sources or many competitors with consistent pain and value signals; **moderate** means 2-4 independent sources or fewer directly comparable products; **weak** means isolated evidence or sparse competitor presence. Source references cite the market-data file and product/section.

### Source Coverage Reference

| Market lane | Main competitors / products covered | Most relevant signal for agent-harness |
|---|---|---|
| Cloud and background agents | Devin, Cursor Cloud Agents, Codex Cloud, Replit Agent, GitHub Copilot cloud agent, Amp, Claude Code/Managed Agents, Goose schedules, ChatGPT Agent/Operator, Vercel Agent, Augment Remote Agent, Refact, Continue, Tabnine | Background work has become a visible product unit: sessions, PRs, tasks, scheduled automations, isolated VMs, branches, logs, checkpoints, and usage caps. Source: `research-cloud-agents.md, Cross-product patterns`. |
| Local desktop / IDE assistants | Cline, Continue, Aider, Junie, Claude Desktop, Cursor, Windsurf, Zed, Goose, OpenCode, Claude Code CLI, Codex CLI, Computer Use | Local users value provider choice, terminal/file access, approval controls, MCP/extensions, and memory/rules, but long sessions and context compression remain failure-prone. Source: `research-local-desktop-assistants.md, Cross-Product Patterns`. |
| Agent SDKs / frameworks | LangGraph, AutoGen, CrewAI, OpenAI Agents, Claude Agent SDK, Google ADK, Letta, Agno, Mastra, Inngest, DSPy, BeeAI, Swarm, Vercel AI SDK | The framework market converges on agents, tools, handoffs/subagents, state/session/memory, HITL, tracing, deployment, and provider abstraction, but not desktop CLI supervision. Source: `research-agent-sdks.md, Cross-Framework Patterns`. |
| Memory / context products | Letta, Mem0, Cognee, Zep/Graphiti, OpenAI Memory, Claude Memory, Cursor Memories, MongoDB, Pinecone, Weaviate, Supabase/pgvector, Qdrant, Redis, HelixDB, SurrealDB, Neo4j | Dedicated memory products validate persistent context and graph/vector retrieval, but the dominant model is consulted memory rather than an imposed graph-derived working set. Source: `research-memory-products.md, Cross-Product Patterns`. |
| AI knowledge management | Tana, Notion, Mem.ai, Reflect, Obsidian plugins, Roam, Logseq, Capacities, Heptabase, Anytype, Apple Notes, Microsoft Loop, Saga, NotePlan, Amplenote | Users already pay for AI over structured knowledge workspaces, but complaints cluster around pricing opacity, AI quality/editability, complexity, trust/local control, and product direction churn. Source: `research-km-with-ai.md, Cross-product Patterns`. |
| Pricing and business models | Cross-product pricing patterns and segments | Long-running/background agents rarely stay inside unlimited flat subscriptions; vendors use credits, ACUs, premium requests, API-like metering, task quotas, active-agent pricing, or deployment minutes. Source: `research-pricing-business-models.md, Executive Takeaways`, `Section 3`. |

### Repeated Market Patterns

| Pattern | Evidence across market data | Relevant problem axes |
|---|---|---|
| Session/task identity is the dominant work container | Devin sessions, Cursor Cloud Agent tasks, Codex cloud tasks, Replit task board, GitHub one-task-one-PR, Amp threads, Claude Code sessions, Goose sessions/recipes/schedules, OpenCode multi-session. Source: `research-cloud-agents.md, P-1 through P-6`; `research-local-desktop-assistants.md, P-9 through P-11`. | `problem.md` §4, §8, §14, §16 |
| Context memory is valuable but usually not canonical state | Letta memory blocks, Mem0 memories, Cognee knowledge graph, Zep context blocks, Cursor Memories, Claude/ChatGPT memory, Tana/Obsidian/Logseq knowledge graphs. Source: `research-memory-products.md, P-1 through P-7`; `research-km-with-ai.md, P-1`, `P-5`, `P-7`. | §2, §5, §12, §15 |
| HITL and approvals are common, but routing semantics are narrow | LangGraph interrupts, OpenAI Agents HITL, Claude `AskUserQuestion`, Agno human approval, Replit Plan Mode, GitHub PR comments, Devin clarifications. Source: `research-agent-sdks.md, F-1`, `F-4`, `F-5`, `F-8`; `research-cloud-agents.md, P-1`, `P-4`, `P-5`. | §9, §16 |
| Tool-call protocol integrity is a recurring source of bugs | OpenAI Agents missing tool outputs, Vercel AI SDK missing tool results/tool loops/stream hangs, CrewAI fabricated tool use, Agno async tools not awaited, Goose permission prompt failure. Source: `research-agent-sdks.md, F-4`, `F-14`, `F-3`, `F-8`; `research-cloud-agents.md, P-8`. | §10, §11, §12 |
| Recovery exists as product affordance but rarely as state accounting | Replit checkpoints, Claude resume, Goose resume/fork/export, Amp revert/handoff, LangGraph time travel, Inngest durable execution, Devin stop/takeover/resume. Source: `research-cloud-agents.md, P-1`, `P-4`, `P-6`, `P-7`; `research-agent-sdks.md, F-1`, `F-10`. | §16 |
| Usage cost becomes product behavior, not just billing | Cursor pricing pivots, Replit effort-based pricing, Codex cloud usage accounting, Claude rolling windows, LangSmith Fleet/deployment pricing, Letta active-agent/tool execution, Notion Custom Agent credits. Source: `research-pricing-business-models.md, Section 4`; `research-pricing-business-models.md, Section 3`. | §13 |

### Surface Testing Summary

Most moderate-to-strong pains in the market data are already covered by the 16 problem axes. Context bloat maps to §1 and §5; stale or lossy summaries map to §2; concurrent/background state mutation maps to §3; identity drift and session/task routing map to §4, §8, and §9; tool protocol bugs map to §10; poisoning and privilege controls map to §12; cost surprises map to §13; user legibility maps to §14; local control maps to §15; recovery failures map to §16. Two recurring pains did not pass the binary coverage test and are written separately in `market-surfaces.md`: graph/memory configuration overhead, and provider/account entitlement friction in multi-provider local stacks.

## Findings by Subsystem

### 1. Context Graph & Provenance

**Competitive landscape:**
- Letta exposes persistent memory blocks, ADE state visualization, import/export, and Letta Code MemFS. It is the closest memory analogue, but it is primarily agent-consulted or agent-edited memory rather than a harness-imposed graph render. Source: `research-memory-products.md, P-1 Letta`; `research-agent-sdks.md, F-7 Letta`.
- Mem0, Cognee, and Zep/Graphiti validate durable memory and graph/vector context assembly. Mem0 sells a universal memory layer with user/session/agent/org scopes; Cognee combines knowledge graphs with vector search and background improve passes; Zep explicitly markets "context engineering" and temporal graph memory. None documents stable pack/unpack node identity or foreground/optimizer snapshot semantics. Source: `research-memory-products.md, P-2 Mem0`, `P-3 Cognee`, `P-4 Zep / Graphiti`.
- Cline Memory Bank, Claude Code memory, Cursor Memories, Windsurf Memories, Tana nodes/supertags, Obsidian/Logseq local files, and Capacities typed objects show many simpler context structures: flat memory files, rule-like memories, node graphs, object schemas, and semantic vault search. These are not provenance-preserving graph contracts. Source: `research-local-desktop-assistants.md, P-1 Cline`, `P-11 Claude Code CLI`, `P-6 Cursor`, `P-7 Windsurf`; `research-km-with-ai.md, P-1 Tana`, `P-5 Obsidian`, `P-7 Logseq`, `P-8 Capacities`.
- Infrastructure products position vector, graph, or multi-model stores as "agent memory": MongoDB, Pinecone, Weaviate, Supabase/pgvector, Qdrant, Redis, HelixDB, SurrealDB, and Neo4j. They provide storage/retrieval primitives, not memory lifecycle or graph working-set policy. Source: `research-memory-products.md, P-8 through P-14c`.

**User pain signals:**
- Memory quality and write policy are unstable. Mem0 has a cited GitHub issue claiming 97.8% junk in audited memory entries; Zep/Graphiti has duplicate-entity issues; Letta has archival memory/API reliability issues. Evidence strength: moderate. Source: `research-memory-products.md, P-2 Mem0`, `P-4 Zep / Graphiti`; `research-agent-sdks.md, F-7 Letta`.
- Context compression and memory loss are visible in coding agents. Claude Code issue #47145 calls auto-compaction a data-loss event; Claude Desktop issue #18866 says auto-compact was not triggering; Cline and Claude memory rely on files rather than durable graph provenance. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-11 Claude Code CLI`, `P-5 Claude Desktop`, `P-1 Cline`.
- Retrieval/context selection remains opaque. Continue issue #3036 reports context gathering taking more than 15 minutes with no explanation; Aider issue #752 reports high repo-map input token usage. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-2 Continue.dev`, `P-3 Aider`.

**Value indicators:**
- Dedicated memory products have direct monetization: Letta Pro/Max/API, Mem0 Starter/Pro, Cognee Cloud/on-prem, Zep metered messages/graph data. Evidence count: 4 memory vendors. Strength: moderate. Source: `research-memory-products.md, Cross-Product Patterns`.
- Developer infrastructure and KM products both compete on context continuity: LangGraph durable state, Letta stateful agents, Zep context blocks, Tana AI nodes, Obsidian semantic search, Capacities AI over object context. Evidence count: 6+ sources. Strength: strong. Source: `research-agent-sdks.md, Cross-Framework Patterns`; `research-memory-products.md, Cross-Product Patterns`; `research-km-with-ai.md, AI Feature Patterns`.
- Local control is a visible demand signal in Obsidian, Logseq, Anytype, Cline, Aider, Goose, and OpenCode. Evidence count: 6+ sources. Strength: strong. Source: `research-km-with-ai.md, P-5 Obsidian`, `P-7 Logseq`, `P-10 Anytype`; `research-local-desktop-assistants.md, P-1 Cline`, `P-3 Aider`, `P-9 Goose`, `P-10 OpenCode`.

**Differentiation opportunities:**
- The proposal differs from all surveyed memory layers by making graph state canonical and locally owned, with `SummaryContract`, `EvidenceArtifact`, `ProvenancePointer`, and `IdentityEvent` as operational records rather than auxiliary metadata.
- The proposal improves on consulted memory by imposing a bounded working set per turn, directly addressing `problem.md` §15 and `P1`; competitors mostly let the agent or app choose what to retrieve.
- Competitors provide richer low-level retrieval/database infrastructure than the proposal describes as product scope; those products could be adjacent substrates, but they do not replace the harness's graph contract.

**Overall evidence strength:** strong.

### 2. Orchestrator

**Competitive landscape:**
- LangGraph is the strongest framework analogue for durable, graph-shaped orchestration, with checkpointing, persistence, human-in-the-loop, subgraphs, time travel, and production deployment. It is an application runtime, not a desktop lead-agent loop over external CLIs. Source: `research-agent-sdks.md, F-1 LangGraph / LangChain`.
- Claude Code, Codex CLI, Cline, OpenCode, Goose, Aider, Cursor, Windsurf, and Zed all compete for the local agent loop. They expose sessions, plans, approvals, terminal tools, repo context, memories/rules, and in some cases subagents or background work. Source: `research-local-desktop-assistants.md, P-1 through P-12`.
- Cloud products externalize orchestration into task/session objects: Devin sessions and Managed Devins, Cursor Cloud Agent tasks, Codex cloud tasks/automations, Replit tasks, GitHub issue-to-PR tasks, and Augment Remote Agents. Source: `research-cloud-agents.md, P-1 Devin`, `P-2 Cursor`, `P-3 Codex`, `P-4 Replit`, `P-5 GitHub Copilot`, `P-11 Augment`.

**User pain signals:**
- Long sessions degrade or fail. OpenCode issue #22883 reports crashes during extended runs; Claude Code users complain about auto-compaction and lack of recovery loops; GitHub cloud agent had an 18+ hour failed session with no commits. Evidence strength: strong. Source: `research-local-desktop-assistants.md, P-10 OpenCode`, `P-11 Claude Code CLI`; `research-cloud-agents.md, P-5 GitHub Copilot`, `P-7 Claude Code`.
- Goal drift is a recognized multi-agent/framework pain. AutoGen issue #7487 asks for a "mission keeper"; CrewAI has issues around fabricated tool use; Cline issue #8354 alleges task-completion claims without verification and unauthorized changes. Evidence strength: moderate. Source: `research-agent-sdks.md, F-2 AutoGen`, `F-3 CrewAI`; `research-local-desktop-assistants.md, P-1 Cline`.
- Context selection has cost and latency consequences. Aider repo-map token complaints and Continue slow context gathering show foreground orchestration depends on more than nominal context size. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-2 Continue.dev`, `P-3 Aider`.

**Value indicators:**
- High adoption around local agent loops: OpenCode 150k+ stars, Claude Code 118k+ stars, Codex 78k+ stars, Zed 79k+ stars, Cline 61k+ stars, Aider 44k+ stars, Goose 43k+ stars. Evidence count: 7 products. Strength: strong. Source: `research-local-desktop-assistants.md, Public traction signals`.
- Cloud orchestration has monetized work units: Devin sessions, Cursor Cloud Agents, Codex tasks, Replit tasks, GitHub Copilot cloud agent PRs, LangSmith Fleet runs. Evidence count: 6 sources. Strength: strong. Source: `research-cloud-agents.md, Cross-product patterns`; `research-pricing-business-models.md, Section 3`.

**Differentiation opportunities:**
- The proposal differs by keeping one lead orchestrator walking initiative-rooted graph state instead of many independent chats, PRs, or task threads. That maps directly to `problem.md` §1, §5, and §14.
- Competitors commonly offer task lists or session lists; they rarely expose exact rendered context and graph walk state as first-class artifacts.
- Frameworks offer programmable orchestration, but not a finished desktop harness that supervises pre-existing Claude/Codex/opencode CLIs.

**Overall evidence strength:** strong.

### 3. Continuous Optimizer

**Competitive landscape:**
- Letta has self-editing memory and intelligent context-window management; Cursor Memories uses a sidecar model to extract project-scoped memories; Cognee has self-improvement/background improve passes; ADK claims filtering, summarizing older turns, lazy-loading artifacts, and token tracking. Source: `research-memory-products.md, P-1 Letta`, `P-7 Cursor Memories`, `P-3 Cognee`; `research-agent-sdks.md, F-6 Google ADK`.
- DSPy validates optimizer language for LM programs, but its optimizers tune prompts/programs rather than continuously curating a live context graph. Source: `research-agent-sdks.md, F-11 DSPy`.
- Zep/Graphiti handles temporal validity and fact invalidation, but as a memory platform API, not a foreground-walk/background-merge topology optimizer. Source: `research-memory-products.md, P-4 Zep / Graphiti`.

**User pain signals:**
- Background memory generation needs human approval or quality controls. Cursor Memories requires user approval for background-generated memories; Mem0 has a junk-memory quality complaint; Zep has duplicate entity issues. Evidence strength: moderate. Source: `research-memory-products.md, P-7 Cursor Memories`, `P-2 Mem0`, `P-4 Zep / Graphiti`.
- State mutation contracts are hard. LangGraph issue #740 reflects state-update contract confusion, and Agno issue #5741 reports exponential session history growth due to recursive history storage. Evidence strength: moderate. Source: `research-agent-sdks.md, F-1 LangGraph`, `F-8 Agno`.
- Optimizer cost is visible even outside harness-like products: DSPy docs warn optimization can cost money and time; pricing analysis shows long-running agent work breaks flat-rate abstractions. Evidence strength: moderate. Source: `research-agent-sdks.md, F-11 DSPy`; `research-pricing-business-models.md, Executive Takeaways`.

**Value indicators:**
- Products with memory/optimization/control-plane layers monetize above raw tokens: LangSmith, Letta, Mem0, Mastra Memory Gateway, Agno AgentOS, Cognee, Zep. Evidence count: 7 sources. Strength: strong. Source: `research-agent-sdks.md, Cross-Framework Patterns`; `research-memory-products.md, Cross-Product Patterns`.
- Direct evidence for continuous topology refactor on a live graph is thin. Evidence count: 1-2 close analogues. Strength: weak for the exact mechanism. Source: `research-memory-products.md, Imposed working context`; `research-agent-sdks.md, Cross-Framework Patterns`.

**Differentiation opportunities:**
- The proposal's `OptimizerEdit`, snapshot-walk-then-merge, `ConflictRecord`, and review/deterministic gates differ from surveyed products, which generally expose memory writes or retrieval rather than an auditable graph-maintenance loop.
- Competitors validate the need for background memory and summarization, but also show why uncontrolled writes can create junk memory, duplicate entities, or state blowup.

**Overall evidence strength:** moderate.

### 4. Sub-Agent Dispatch & Reintegration

**Competitive landscape:**
- Devin Managed Devins are the closest commercial precedent: a coordinator Devin splits large tasks into child sessions, monitors progress, resolves conflicts, and compiles results. Source: `research-cloud-agents.md, P-1 Devin`.
- Cursor, Codex, Replit, GitHub Copilot, Augment, Claude Code, Goose, Cline, LangGraph, CrewAI, AutoGen, OpenAI Agents SDK, and Agno all offer some form of background agents, subagents, handoffs, teams, crews, tasks, or multi-agent workflows. Source: `research-cloud-agents.md, P-2 through P-5`, `P-11`; `research-local-desktop-assistants.md, P-1`, `P-9`, `P-11`, `P-12`; `research-agent-sdks.md, F-1 through F-8`.
- Existing units of work are heterogeneous: cloud VMs, PRs, branches, task cards, threads, framework runs, crews, and subgraphs. Few are graph-slice assignments with explicit reintegration semantics. Source: `research-cloud-agents.md, Cross-product patterns`; `research-agent-sdks.md, Cross-Framework Patterns`.

**User pain signals:**
- Worker isolation and reintegration are weakly surfaced. Amp documents that subagents work in isolation and cannot communicate; GitHub cloud agent maps one task to one PR; Replit detects dependencies/conflicts when applying task results, implying conflict is expected. Evidence strength: moderate. Source: `research-cloud-agents.md, P-6 Sourcegraph Amp`, `P-5 GitHub Copilot`, `P-4 Replit`.
- External tool permissions and branch/PR integration fail in real products. Cursor API-spawned Background Agents could clone/branch/push but not post PR comments because permissions were missing; GitHub cloud-agent discussions show internal errors and failed long sessions. Evidence strength: moderate. Source: `research-cloud-agents.md, P-2 Cursor`, `P-5 GitHub Copilot`.
- Tool-use and output truthfulness are live risks. CrewAI issue #3154 reports fabricated/simulated tool use; Cline issue #8354 alleges completion claims without verification. Evidence strength: moderate. Source: `research-agent-sdks.md, F-3 CrewAI`; `research-local-desktop-assistants.md, P-1 Cline`.

**Value indicators:**
- Parallel agents are a core sales feature for Devin, Cursor, Codex, Replit, GitHub Copilot, Augment, Goose, Claude Code, and Cline. Evidence count: 9 sources. Strength: strong. Source: `research-cloud-agents.md, Cross-product patterns`; `research-local-desktop-assistants.md, P-1`, `P-9`, `P-11`.
- Pricing often charges for parallel/background execution or caps it: Devin concurrent sessions, Replit active background tasks, Cursor usage pools, Codex cloud limits, LangSmith Fleet runs. Evidence count: 5+ sources. Strength: strong. Source: `research-pricing-business-models.md, Section 3`.

**Differentiation opportunities:**
- The proposal differs by making delegation a `WorkerSlice` over graph state, not just a prompt and final answer.
- Reintegration as staged graph candidates, evidence artifacts, conflicts, questions, and advisory optimizer requests directly addresses market pain around isolated subagents and unverifiable final outputs.
- Competitors have stronger hosted execution environments; the proposal's difference is local multi-CLI supervision rather than owning the sandbox.

**Overall evidence strength:** strong.

### 5. NEEDS_INPUT Routing

**Competitive landscape:**
- LangGraph interrupts, OpenAI Agents SDK HITL, Claude Agent SDK `AskUserQuestion`, Agno human approval, CrewAI human-in-the-loop triggers, Replit Plan Mode/review, GitHub PR comments, and Devin session clarifications all provide user-intervention surfaces. Source: `research-agent-sdks.md, F-1 LangGraph`, `F-4 OpenAI Agents`, `F-5 Claude Agent SDK`, `F-8 Agno`, `F-3 CrewAI`; `research-cloud-agents.md, P-1 Devin`, `P-4 Replit`, `P-5 GitHub Copilot`.
- Most products scope questions to one session, run, PR, or framework execution. The market data does not show a cross-CLI question artifact with render/slice/session correlation keys. Source: `research-agent-sdks.md, Cross-Framework Patterns`; `research-cloud-agents.md, Cross-product patterns`.

**User pain signals:**
- HITL surfaced as a missing or high-priority feature in SDKs: OpenAI issue #636 requested HITL as top priority; Claude SDK issue #327 asks for AskUserQuestion tooling; issue #137 asks how to pass tool approvals like the CLI. Evidence strength: moderate. Source: `research-agent-sdks.md, F-4 OpenAI Agents SDK`, `F-5 Claude Agent SDK`.
- Mid-task correction is uneven. Codex early preview lacked mid-task course correction; GitHub only acts on open PR comments from write-access users and ignores issue comments after assignment; Devin and Cursor use session/follow-up messaging but not graph-addressed continuations. Evidence strength: moderate. Source: `research-cloud-agents.md, P-3 Codex`, `P-5 GitHub Copilot`, `P-1 Devin`, `P-2 Cursor`.
- Resume/terminal state can be fragile. AutoGen issue #279 reports terminal-state/result fragility; OpenAI Agents issue #1061 reports `previous_response_id` failure from missing tool output, which can break continuation. Evidence strength: moderate. Source: `research-agent-sdks.md, F-2 AutoGen`, `F-4 OpenAI Agents SDK`.

**Value indicators:**
- HITL/approval appears across at least five frameworks and several cloud/local products. Evidence count: 8+ sources. Strength: strong for the general need.
- Evidence for cross-CLI graph-state question routing is weak; no surveyed product documents the exact `QuestionArtifact` model. Strength: weak for the exact mechanism.

**Differentiation opportunities:**
- The proposal's `QuestionArtifact` and child-acceptance tracking differ from chat-like clarification and PR-comment feedback loops.
- The proposal improves on generic HITL by routing answers as continuations of blocked work under `problem.md` §9 and `P11`, instead of appending answers to the lead transcript.

**Overall evidence strength:** moderate.

### 6. Workflow Review & Governance

**Competitive landscape:**
- Governance is common at the policy/enterprise layer: Cursor Teams/Enterprise has RBAC, SSO, SCIM, audit logs, model controls; GitHub Copilot Business/Enterprise has centralized policies; Tabnine/Refact emphasize privacy and deployment controls; LangSmith/Agno/CrewAI sell observability/control planes. Source: `research-cloud-agents.md, P-2 Cursor`, `P-5 GitHub Copilot`, `P-12 Refact/Continue/Tabnine`; `research-agent-sdks.md, F-1 LangGraph`, `F-3 CrewAI`, `F-8 Agno`.
- Agent-framework governance primitives exist: OpenAI Agents guardrails, CrewAI guardrails, BeeAI deterministic rule enforcement, Claude permissions, Cline approvals, Codex approval modes. Source: `research-agent-sdks.md, F-4 OpenAI Agents`, `F-3 CrewAI`, `F-12 BeeAI`; `research-local-desktop-assistants.md, P-1 Cline`, `P-11 Claude Code`, `P-12 Codex CLI`.

**User pain signals:**
- Permission and approval flows are fragile. Cline issue #8354 alleges unauthorized changes; Goose issue #5559 says Claude Code provider actions needing permissions did not show approval prompts; Claude SDK issue #137 asks about passing tool approvals. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-1 Cline`; `research-cloud-agents.md, P-8 Goose`; `research-agent-sdks.md, F-5 Claude Agent SDK`.
- Tool-call integrity is a recurrent failure mode. CrewAI fabricated tool use; Vercel AI SDK has missing tool results and tool loops; OpenAI Agents has missing tool output/tracing mismatch issues. Evidence strength: strong. Source: `research-agent-sdks.md, F-3 CrewAI`, `F-14 Vercel AI SDK`, `F-4 OpenAI Agents SDK`.
- Memory and prompt-injection risks are not cosmetic. Mem0 docs warn against storing secrets/PII; Goose documents prompt-injection detection; ChatGPT Agent/Operator requires confirmations for high-impact actions and monitors prompt injection. Evidence strength: moderate. Source: `research-memory-products.md, P-2 Mem0`; `research-local-desktop-assistants.md, P-9 Goose`; `research-cloud-agents.md, P-9 ChatGPT Agent / Operator`.

**Value indicators:**
- Enterprise tiers repeatedly monetize control surfaces: SSO, SCIM, RBAC, audit logs, data controls, VPC/self-hosted, privacy, and support. Evidence count: 8+ sources. Strength: strong. Source: `research-pricing-business-models.md, Section 1`, `Section 2`.
- Observability/tracing is a paid control-plane value in LangSmith, Agno, Vercel AI SDK/AI Cloud, CrewAI AMP, and Inngest. Evidence count: 5 sources. Strength: strong. Source: `research-agent-sdks.md, Cross-Framework Patterns`.

**Differentiation opportunities:**
- The proposal's `WorkflowReviewer` as evidence rather than authority maps to `problem.md` §11 and `P10`; surveyed products expose guardrails and observability but do not document LLM reviewer fallibility as a governance contract.
- Deterministic gates for privilege, tool-call protocol, identity, budget, and summary contracts differ from generic "guardrails" claims.

**Overall evidence strength:** strong.

### 7. Cost & Budget

**Competitive landscape:**
- Pricing models across the market combine seats, credits, tokens, ACUs, premium requests, Fleet runs, deployment uptime, active agents, tool-execution seconds, and AI credits. Source: `research-pricing-business-models.md, Section 1`.
- Background work usually has usage rails: Devin ACUs/session concurrency, Cursor API-priced Background Agents, Codex cloud usage limits/credits, Replit effort-based pricing, GitHub premium requests, LangSmith Fleet/deployment charges, Letta active-agent/tool-execution pricing. Source: `research-pricing-business-models.md, Section 3`.
- KM and memory tools also meter AI: Tana AI credits, Heptabase AI credits, Notion Custom Agent credits, Mem0 memory/retrieval calls, Zep messages/graph data. Source: `research-km-with-ai.md, Pricing Patterns`; `research-memory-products.md, Cross-Product Patterns`.

**User pain signals:**
- Pricing units fail when agent effort varies. Cursor moved away from fixed request counts and per-tool-call Max Mode pricing; Replit replaced $0.25/checkpoint with effort-based pricing; Codex cloud tasks moved into usage accounting. Evidence strength: strong. Source: `research-pricing-business-models.md, Section 4`.
- Users complain when background agents unexpectedly burn budgets. Cursor forum users described one prompt draining plan usage and spend-limit confusion; OpenAI community user reported hitting Codex limits quickly and hung runs consuming limits. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-6 Cursor`; `research-cloud-agents.md, P-3 OpenAI Codex Cloud`; `research-cloud-agents.md, P-2 Cursor`.
- Local tools still expose token-cost anxiety. Aider repo-map token usage and Claude Code average cost documentation show local workflows are not "free" just because execution is local. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-3 Aider`, `P-11 Claude Code CLI`.

**Value indicators:**
- A $20/month personal AI anchor and $100-$200/month power-user compute anchor are repeated across ChatGPT, Claude, Cursor, Devin, Letta, Augment, and Codex. Evidence count: 6+ sources. Strength: strong. Source: `research-pricing-business-models.md, Cross-Lane Signals`.
- BYO-key is credible for local orchestrators because users value provider choice and cost exposure, but it limits vendor revenue capture. Evidence count: 4 monetization paths. Strength: strong. Source: `research-pricing-business-models.md, Section 5`.

**Differentiation opportunities:**
- The proposal's `BudgetLedger`, cache prefix hashes, render token ceilings, and budget-gated optimizer/reviewer/worker behavior align closely with market pain around opaque usage.
- Competitors expose budgets mostly as billing/plan dashboards; the proposal treats cost as correctness, mapping to `problem.md` §13 and `P12`.

**Overall evidence strength:** strong.

### 8. Recovery

**Competitive landscape:**
- Replit has the clearest documented checkpoint/rollback model: checkpoints capture workspace contents, AI conversation context, environment config, Agent memory, and optionally dev DB contents. Source: `research-cloud-agents.md, P-4 Replit`.
- Claude Code/Claude Agent SDK, Goose, OpenCode, Amp, Codex, GitHub Copilot, Devin, LangGraph, Inngest, and AutoGen all expose pieces of resume, fork, export, retry, time travel, or durable execution. Source: `research-local-desktop-assistants.md, P-9 Goose`, `P-10 OpenCode`, `P-11 Claude Code`; `research-cloud-agents.md, P-1 Devin`, `P-3 Codex`, `P-5 GitHub Copilot`; `research-agent-sdks.md, F-1 LangGraph`, `F-10 Inngest`, `F-2 AutoGen`.

**User pain signals:**
- Recovery can fail or lose state. Replit community thread says disabled rollback plus new-chat suggestions made the AI forget history; OpenCode crashes during extended runs; Continue has "Session file does not exist" issues. Evidence strength: moderate. Source: `research-cloud-agents.md, P-4 Replit`; `research-local-desktop-assistants.md, P-10 OpenCode`; `research-cloud-agents.md, P-12 Refact/Continue/Tabnine`.
- Retry/resume loops are missing in headless automation. Claude Code issue #28489 complains `claude -p` lacked a built-in loop for iteration and automatic recovery after crashes. Evidence strength: weak alone, moderate with other recovery complaints. Source: `research-cloud-agents.md, P-7 Claude Code`.
- Long-running cloud work can fail expensively. GitHub cloud-agent discussion reports an 18+ hour failed session with only the initial plan; Codex user reports hung runs consuming limits. Evidence strength: moderate. Source: `research-cloud-agents.md, P-5 GitHub Copilot`, `P-3 OpenAI Codex Cloud`.

**Value indicators:**
- Checkpoints, rollback, resume, fork/export, and durable execution are visible product features across at least six products/frameworks. Evidence count: 6+ sources. Strength: strong.
- Exact graph-state recovery across CLI sessions is not documented in competitors. Strength: weak for the exact mechanism.

**Differentiation opportunities:**
- The proposal's `RecoveryAction` with preserved/replayed/discarded records maps directly to `problem.md` §16 and `P14`.
- Competitors often expose retry, resume, rollback, or PR handoff; they rarely explain cross-boundary recovery effects on memory, questions, provenance, and worker slices.

**Overall evidence strength:** strong.

### 9. User Surface

**Competitive landscape:**
- Cloud agents externalize work into sessions, task lists, task boards, PRs, branches, and status surfaces: Devin sessions/progress tab, Cursor task panels, Codex task list/progress, Replit task board, GitHub Agents panel, Augment remote tasks. Source: `research-cloud-agents.md, P-1 through P-5`, `P-11 Augment`.
- Local/desktop products expose multiple sessions or agent panes: Windsurf multiple Cascades, OpenCode multi-session, Claude Code multiple side-by-side sessions, Zed agent threads, Cline orchestration UI claims, Goose desktop/CLI sessions. Source: `research-local-desktop-assistants.md, P-7 Windsurf`, `P-10 OpenCode`, `P-11 Claude Code`, `P-8 Zed`, `P-1 Cline`, `P-9 Goose`.
- KM products provide graph/object/page surfaces: Tana nodes, Notion databases, Obsidian graph/files, Logseq outliner graph, Capacities typed objects, Heptabase boards. Source: `research-km-with-ai.md, Hierarchy-primitive Patterns`.

**User pain signals:**
- Users need legibility into progress, blockers, and cost. Cursor spend-limit UI bugs, GitHub failed long session with no progress, Replit rollback/new-chat memory loss, and Augment audible completion request all show status/notification expectations. Evidence strength: moderate. Source: `research-cloud-agents.md, P-2 Cursor`, `P-5 GitHub Copilot`, `P-4 Replit`, `P-11 Augment`.
- Product direction and feature moves create trust issues in KM/workspace tools: Microsoft Loop removed Copilot integrations due to confusion; Smart Connections license shift upset Obsidian users; Notion AI/Custom Agent credit moves drew complaints. Evidence strength: moderate. Source: `research-km-with-ai.md, P-12 Microsoft Loop`, `P-5 Obsidian`, `P-2 Notion`.
- Single-tab multi-initiative orchestration has thin direct precedent. Market products tend to use many sessions/tasks/threads rather than one orchestrator walking many initiative roots. Evidence strength: weak for the exact UI. Source: `research-cloud-agents.md, Cross-product patterns`; `research-local-desktop-assistants.md, Cross-Product Patterns`.

**Value indicators:**
- Task boards, session lists, status badges, logs, branches, PRs, checkpoints, and follow-up surfaces are table stakes for cloud agents. Evidence count: 5+ sources. Strength: strong.
- AI-KM products show users pay for graph/object/page workspaces with AI, but mostly for notes and knowledge work rather than live agent operations. Evidence count: 6+ sources. Strength: moderate. Source: `research-km-with-ai.md, Pricing Patterns`, `AI Feature Patterns`.

**Differentiation opportunities:**
- The proposal differs by combining initiative roots, focus pane, question queue, worker board, optimizer log, cost surface, recovery surface, and evidence drill-down into one graph-backed surface.
- The separation of action-needed from passive-progress notifications maps directly to `problem.md` §14 and `P13`; competitors often expose progress but do not distinguish graph-state continuations from informational updates.

**Overall evidence strength:** strong.

### 10. Cross-CLI Adaptation

**Competitive landscape:**
- Goose, OpenCode, Cline, Zed, Continue, Aider, Claude Code, and Codex CLI all support provider routing, local execution, MCP/extensions, or agent protocol surfaces. Goose lists 15+ providers; OpenCode claims 75+ providers through Models.dev; Zed supports "bring your own agent" via ACP. Source: `research-local-desktop-assistants.md, P-9 Goose`, `P-10 OpenCode`, `P-1 Cline`, `P-8 Zed`, `P-2 Continue`, `P-3 Aider`, `P-11 Claude Code`, `P-12 Codex CLI`.
- Agent SDKs also converge on provider abstraction, MCP, handoffs, tools, sessions, and tracing, but inside app runtimes rather than across desktop CLIs. Source: `research-agent-sdks.md, Cross-Framework Patterns`.
- Cloud products are less cross-CLI and more platform-specific: Devin, Cursor, Codex, GitHub, Replit, and Augment provide their own task environments. Source: `research-cloud-agents.md, Cross-product patterns`.

**User pain signals:**
- Provider/auth/setup differences create friction. Goose issue #6607 reports Copilot auth trouble in WSL; Junie users ask for local model support because credits feel restrictive; Codex issue #5041 reports network/sandbox failure; Heptabase users request OpenAI-compatible providers. Evidence strength: moderate. Source: `research-local-desktop-assistants.md, P-9 Goose`, `P-4 Junie`, `P-12 Codex CLI`; `research-km-with-ai.md, P-9 Heptabase`.
- Cross-framework feature parity pressure exists. Google ADK issue #3611 asks for Claude skill support; Inngest AgentKit issue #173 reports MCP incompatibility with Gemini; Continue issues include model/provider errors. Evidence strength: moderate. Source: `research-agent-sdks.md, F-6 Google ADK`, `F-10 Inngest`; `research-cloud-agents.md, P-12 Refact/Continue/Tabnine`.
- Permission semantics differ. Claude, Codex, Cline, Goose, and Cursor each expose different approval, hooks, or permission behavior; failures appear in user complaints. Evidence strength: moderate. Source: `research-agent-sdks.md, F-5 Claude Agent SDK`; `research-local-desktop-assistants.md, P-1 Cline`, `P-9 Goose`, `P-12 Codex CLI`; `research-cloud-agents.md, P-2 Cursor`.

**Value indicators:**
- Provider choice and local control are repeated purchase/adoption signals in Cline, Aider, Goose, OpenCode, Zed, Obsidian plugins, Logseq, Capacities, and Heptabase. Evidence count: 8+ sources. Strength: strong.
- Exact adaptation across Claude/Codex/opencode CLI stores and injection mechanisms is not directly present in surveyed products. Strength: weak for the exact mechanism.

**Differentiation opportunities:**
- The proposal's `CapabilityFingerprint` makes mechanism asymmetry visible, aligning with `problem.md` §7 and `P8`.
- Competitors often market provider breadth as uniform flexibility; the proposal's distinction is to treat each CLI's render, tool interception, resume, and question-routing surface as operationally different.

**Overall evidence strength:** moderate.

## Cross-cutting Findings

**Privilege and Poisoning Controls:** The market supports this concern strongly. Prompt/tool misuse appears in CrewAI fabricated tool-use issues, Vercel/OpenAI tool-result protocol issues, Cline unauthorized-change complaints, Goose permission prompt failures, and ChatGPT Agent/Operator high-impact confirmation rules. Memory products add risk because stored content is durable: Mem0 warns against secrets/PII and has memory-quality complaints; Cursor requires approval for background-generated memories. These findings map to `problem.md` §10 and §12 and to `P7`, `P10`, and `P15`.

**Audit and Reversibility:** Strong evidence exists for auditability, but weaker evidence exists for full graph reversibility. Enterprise pricing repeatedly sells audit logs and admin controls. Replit checkpoints, Amp reverts/thread handoff, Goose export/fork/resume, LangGraph time travel, and GitHub PR review all validate user demand for recoverable history. None documents the proposal's exact audit chain for optimizer edits, recovery actions, and worker reintegration. This maps to `problem.md` §16, `P14`, and `P15`.

**Local Control of State:** Strong evidence exists in local/desktop tools and KM products. Cline, Aider, Goose, OpenCode, Zed, Obsidian, Logseq, Anytype, Supabase/pgvector, Qdrant, Redis, and SurrealDB all show demand for local, BYO-key, self-hosted, or provider-flexible operation. Pricing analysis says BYO-key is credible for a local personal orchestrator because users value provider choice and cost exposure. This maps to `problem.md` §15 and `P15`.

**No In-Product Compaction:** The market supports the underlying pain but not the exact policy. Claude Code auto-compact complaints, Claude SDK requests for non-interactive `/compact`, Aider repo-map token complaints, Continue context-gathering delays, and memory products' summarization/context assembly all show context discipline pressure. The proposal's anti-compaction stance is more specific than competitors, and maps to `problem.md` §1, §2, §15, and `P9`.

## Evidence Gaps

- **Exact imposed working-set precedent:** The memory and KM lanes found no product clearly advertising an optimizer-maintained graph that imposes a bounded working set every orchestrator turn with pack/unpack navigation. Zep and Letta are closest, but still mostly consulted memory.
- **Continuous optimizer merge semantics:** There is limited direct market evidence for snapshot-walk-then-merge or continuous topology refactor on a graph a foreground agent is actively walking.
- **Cross-CLI provenance:** Many products support MCP, provider choice, or agent protocols, but none documents preserving Claude/Codex/opencode tool-call provenance under one stable local graph contract.
- **Single-tab orchestrator UI:** Market products validate task/session boards and graph/KM surfaces, but not one user surface where one lead orchestrator manages many initiative roots without separate chats.
- **Workflow reviewer reliability:** Frameworks sell guardrails and observability, but the market files contain little direct product evidence for LLM reviewers auditing graph mutations as a sampled evidence channel.
- **Commercial pricing precedent for this exact scope:** Pricing data strongly supports BYO-key local tools and heavy-user AI budgets, but `research-pricing-business-models.md` explicitly notes no direct public precedent for a "very long running personal orchestrator with local graph provenance."
