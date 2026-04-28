# Market Research: Agent Orchestration SDKs and Frameworks

Stage 0b research output for the developer-facing agent orchestration SDK / framework lane. Evidence is limited to primary sources where possible: vendor docs, official pricing pages, official blogs, GitHub repos/issues, GitHub REST API metadata, npm downloads API, and PyPI package metadata. Public metrics were retrieved on 2026-04-27 around 17:00 PDT unless otherwise noted. PyPI download counts are not included because PyPI does not expose package download counts as a primary package metric; where Python packages matter, this report uses PyPI package/version metadata plus GitHub traction.

## Framework Walk

### F-1 — LangGraph / LangChain

**Vendor / repo / docs:** LangChain Inc.; LangGraph docs: https://docs.langchain.com/oss/python/langgraph/overview; LangGraph repo: https://github.com/langchain-ai/langgraph; LangChain repo: https://github.com/langchain-ai/langchain; LangSmith pricing: https://www.langchain.com/pricing; funding/platform blog: https://www.blog.langchain.com/series-b/.

**License model:** OSS, MIT for LangGraph and LangChain per GitHub API/license metadata. Hosted LangSmith is proprietary SaaS.

**Hosted offering:** Yes: LangSmith and LangSmith Deployment. Pricing page lists Developer at $0/seat/month, Plus at $39/seat/month, and Enterprise custom. Developer includes 5k base traces/month and one Fleet agent with 50 Fleet runs/month. Plus includes 10k base traces/month, one dev-sized deployment, unlimited Fleet agents, and 500 Fleet runs/month. Additional Plus deployment runs are $0.005/run; dev deployment uptime is $0.0007/min and production deployment uptime is $0.0036/min. Base traces are $2.50/1k and extended traces $5/1k; Fleet overage is $0.05/run. Model usage is billed separately by the model provider. Source: https://www.langchain.com/pricing.

**Capability surface:** LangGraph positions itself as a low-level orchestration runtime for long-running, stateful agents. Docs call out durable execution, human-in-the-loop, short-term and long-term memory, streaming, subgraphs, time travel, checkpoint-backed persistence, LangSmith tracing/evaluation/observability, and production deployment. LangChain sits above it with higher-level agent abstractions and provider integrations. Source: https://docs.langchain.com/oss/python/langgraph/overview.

**Long-running / persistent-state support:** Strongest in this set. LangGraph explicitly supports durable execution, checkpointing/persistence, memory across sessions, and production deployments with a database whose uptime is billed on LangSmith Deployment. The pricing FAQ defines an agent run as a LangGraph deployment invocation and says a human-in-the-loop interrupt resume creates a separate agent run, which is a concrete billing and runtime signal that persistence/resume is first-class. Sources: https://docs.langchain.com/oss/python/langgraph/overview and https://www.langchain.com/pricing.

**Multi-agent orchestration support:** LangGraph is graph-native: nodes, edges, subgraphs, and explicit control flow. The docs describe it as focused entirely on agent orchestration and as the runtime powering LangChain’s agent layer. Subgraphs support composing multi-agent or nested workflows; LangSmith Deployment can expose agents over APIs and MCP. Source: https://docs.langchain.com/oss/python/langgraph/overview.

**Human-in-the-loop / question-routing support:** First-class. Docs list interrupts and human-in-the-loop as core capabilities; LangSmith billing and FAQ specifically define resumed interrupt executions as separate agent/Fleet runs. This gives LangGraph a durable pause/resume model, but one oriented around graph execution state, not cross-CLI user-question routing. Sources: https://docs.langchain.com/oss/python/langgraph/overview and https://www.langchain.com/pricing.

**Adoption signals:** GitHub API on 2026-04-27: langchain-ai/langgraph had 30,582 stars, 5,228 forks, 512 open issues; langchain-ai/langchain had 135,142 stars, 22,350 forks, 552 open issues. npm downloads API for 2026-03-28 to 2026-04-26: `@langchain/langgraph` had 8,385,583 downloads; `langchain` had 8,847,340. LangChain’s official Series B post claims $125M raised at a $1.25B valuation, combined 90M monthly downloads for `langchain` and `langgraph`, 35% of the Fortune 500 using its services, 12x YoY monthly trace volume, and customers including Replit, Clay, Harvey, Rippling, Cloudflare, Workday, and Cisco. Sources: https://api.github.com/repos/langchain-ai/langgraph, https://api.github.com/repos/langchain-ai/langchain, https://api.npmjs.org/downloads/point/last-month/%40langchain%2Flanggraph, https://api.npmjs.org/downloads/point/last-month/langchain, https://www.blog.langchain.com/series-b/.

**Customer segments:** Broadest enterprise penetration: AI-native startups, financial services, developer platforms, productivity SaaS, and Fortune 500 enterprise AI teams. The docs name Klarna, Uber, and J.P. Morgan as LangGraph users; the funding blog names Replit, Clay, Harvey, Rippling, Cloudflare, Workday, and Cisco. Sources: https://docs.langchain.com/oss/python/langgraph/overview and https://www.blog.langchain.com/series-b/.

**Documented developer pain:** High-engagement GitHub issues show durable-state and runtime complexity. `langgraph-checkpoint-postgres` SSL failure issue #3716 had 47 comments and points at operational friction in checkpoint storage: https://github.com/langchain-ai/langgraph/issues/3716. `InvalidUpdateError: Must write to at least one...` issue #740 had 35 comments and reflects state-update contract confusion: https://github.com/langchain-ai/langgraph/issues/740. `LLM is slow within langgraph agent` issue #2920 had 35 comments, a common agent-runtime cost/latency pain: https://github.com/langchain-ai/langgraph/issues/2920. The v1 roadmap issue #4973 had 79 comments, showing active API evolution and migration pressure: https://github.com/langchain-ai/langgraph/issues/4973.

**Positioning relative to agent-harness:** LangGraph solves a large part of the orchestration/state/HITL problem for applications built inside LangGraph. It does not solve the desktop cross-CLI orchestrator problem: it does not manage Claude/Codex/opencode sessions, impose a graph-derived working set onto independent CLIs, or supervise external agent transcripts as provenance. LangGraph users are candidate users if they are building agentic systems and need local developer orchestration around multiple coding CLIs, but LangGraph itself is more likely a conceptual analogue or hosted alternative than a direct dependency.

**Notes:** LangGraph is the strongest evidence that the market is paying for agent engineering infrastructure: OSS runtime plus commercial observability/deployment plus enterprise support.

### F-2 — AutoGen

**Vendor / repo / docs:** Microsoft; docs: https://microsoft.github.io/autogen/stable/index.html; repo: https://github.com/microsoft/autogen; code license file: https://github.com/microsoft/autogen/blob/main/LICENSE-CODE.

**License model:** OSS. GitHub API reports repo license as CC-BY-4.0 because the repo has multiple licenses, but `LICENSE-CODE` is MIT for code. Source: https://github.com/microsoft/autogen/blob/main/LICENSE-CODE.

**Hosted offering:** No direct AutoGen hosted tier. Microsoft positions AutoGen as OSS framework/components; Azure AI Foundry / Agent Service is cross-lane cloud-agent-product territory, not counted as AutoGen hosted pricing.

**Capability surface:** AutoGen has three layers: Studio for web-based no-code prototyping; AgentChat for conversational single- and multi-agent apps; Core for event-driven scalable multi-agent systems; Extensions for MCP, OpenAI Assistant API, Docker code execution, and distributed gRPC runtimes. Source: https://microsoft.github.io/autogen/stable/index.html.

**Long-running / persistent-state support:** AutoGen Core is event-driven and scalable, with distributed agent runtime extensions, but the primary docs landing page emphasizes framework components rather than a durable-execution product. Persistence exists through team state / save-load APIs in deeper docs, but public positioning is less “durable workflow engine” than LangGraph/Inngest/ADK.

**Multi-agent orchestration support:** Strong. AgentChat is explicitly for conversational single and multi-agent applications; Core is for deterministic/dynamic workflows, research on collaboration, and distributed agents. Source: https://microsoft.github.io/autogen/stable/index.html.

**Human-in-the-loop / question-routing support:** Supported through AgentChat user proxy / user input patterns, but the landing docs do not present HITL as the headline primitive. It is adequate for chat-based involvement, weaker as an explicit question-artifact/routing system.

**Adoption signals:** GitHub API on 2026-04-27: 57,501 stars, 8,665 forks, 793 open issues. PyPI metadata on 2026-04-27: `autogen-agentchat` 0.7.5, `autogen-ext` 0.7.5, and `pyautogen` 0.10.0. Source: https://api.github.com/repos/microsoft/autogen and https://pypi.org/project/autogen-agentchat/.

**Customer segments:** Research groups, enterprise developers experimenting with multi-agent collaboration, Microsoft ecosystem users, and teams needing Python multi-agent patterns. The docs explicitly cite business processes, multi-agent research, and distributed multi-language applications. Source: https://microsoft.github.io/autogen/stable/index.html.

**Documented developer pain:** Top GitHub issues retrieved before rate limiting include mission/goal integrity feature requests and basic runtime failures. Issue #7487 asks for a “mission keeper” role for goal integrity in multi-agent systems, showing orchestration drift pain: https://github.com/microsoft/autogen/issues/7487. Issue #279 reports `AttributeError: 'str' object has no attribute 'get' after the task is finished`, showing fragility around terminal state/results: https://github.com/microsoft/autogen/issues/279. Issue #3345 covered OpenAI API key format confusion: https://github.com/microsoft/autogen/issues/3345.

**Positioning relative to agent-harness:** AutoGen is a framework for building multi-agent applications, not a desktop harness over existing provider CLIs. It overlaps conceptually in sub-agent supervision and user input, but does not solve cross-CLI session ingestion, imposed working sets, or long-lived local initiative graphs.

**Notes:** AutoGen’s strongest signal is OSS mindshare and Microsoft backing, not a direct monetization model.

### F-3 — CrewAI

**Vendor / repo / docs:** CrewAI Inc.; docs: https://docs.crewai.com/; repo: https://github.com/crewAIInc/crewAI; pricing: https://www.crewai.com/pricing.

**License model:** OSS, MIT per GitHub API. CrewAI AMP is proprietary/hybrid enterprise product.

**Hosted offering:** Yes. Pricing page lists Basic as free with visual editor, AI copilot, GitHub integration, and 50 workflow executions/month. Enterprise is custom and includes CrewAI-hosted or private infrastructure, on-site support/training, and 50 development hours/month. It also describes CrewAI AMP Cloud and AMP Factory for on-prem/private VPC deployment. Source: https://www.crewai.com/pricing.

**Capability surface:** Agents, crews, tasks, processes, flows, guardrails, memory, knowledge, structured Pydantic outputs, callbacks, human-in-the-loop triggers, observability, deployment automations, and enterprise triggers/integrations including Gmail, Slack, Salesforce, Teams, OneDrive, HubSpot. Source: https://docs.crewai.com/.

**Long-running / persistent-state support:** Docs describe Flows as orchestrating start/listen/router steps, managing state, persisting execution, and resuming long-running workflows. Source: https://docs.crewai.com/.

**Multi-agent orchestration support:** Native “crews” with sequential, hierarchical, or hybrid processes. CrewAI’s core abstraction is role-based collaborative agents assigned tasks. Source: https://docs.crewai.com/.

**Human-in-the-loop / question-routing support:** Docs list human-in-the-loop triggers in tasks/processes, but the public landing page does not expose a durable question-artifact model. It is more workflow approval/trigger oriented than graph-addressed user-question routing.

**Adoption signals:** GitHub API on 2026-04-27: 50,092 stars, 6,894 forks, 414 open issues. PyPI metadata: `crewai` 1.14.3. Pricing page displays customer logos including AB InBev, BDO, Docusign, Experian, EXL, Genpact, Globo, Havas, IBM, Johnson & Johnson, Konecta, KT, NICE Actimize, NTT Data, PepsiCo, PwC, and RBC. Sources: https://api.github.com/repos/crewAIInc/crewAI, https://pypi.org/project/crewai/, https://www.crewai.com/pricing.

**Customer segments:** Business automation, enterprise process automation, professional services, regulated industries, and operations teams. CrewAI’s AMP positioning is explicitly organizational adoption and “agentic workforce” management.

**Documented developer pain:** Issue #3154, 63 comments, reports an agent fabricating/simulating tool usage instead of invoking tools: https://github.com/crewAIInc/crewAI/issues/3154. Issue #2885, 58 comments, reports empty/invalid LLM responses: https://github.com/crewAIInc/crewAI/issues/2885. Issue #430, 63 comments, reports Serper tool argument mismatch: https://github.com/crewAIInc/crewAI/issues/430. These map to tool-call reliability, provider error handling, and integration fragility.

**Positioning relative to agent-harness:** CrewAI overlaps more with business-process agent teams than developer desktop orchestration. Users building CrewAI automations could still need a harness for coding/research work across CLIs, but CrewAI AMP itself is a cloud/enterprise management product rather than a local orchestrator.

**Notes:** CrewAI has unusually visible enterprise-logo marketing for this category.

### F-4 — OpenAI Agents SDK

**Vendor / repo / docs:** OpenAI; Python docs: https://openai.github.io/openai-agents-python/; TypeScript docs: https://openai.github.io/openai-agents-js/; Python repo: https://github.com/openai/openai-agents-python; TypeScript repo: https://github.com/openai/openai-agents-js; API pricing: https://openai.com/api/pricing/.

**License model:** OSS, MIT for Python/JS repos per GitHub API. Runs are paid through OpenAI API usage.

**Hosted offering:** No SDK-hosted enterprise tier. The runtime uses OpenAI APIs by default and can use provider-agnostic models. OpenAI API pricing is usage-based; official pricing page is the source of truth for model/token costs. Source: https://openai.com/api/pricing/.

**Capability surface:** Agents, agent loop, function tools, hosted tools, agents-as-tools, handoffs, guardrails, MCP tools, sessions, human-in-the-loop, tracing, Realtime Agents, voice pipelines, sandbox agents in Python, and model/provider abstraction. Sources: https://openai.github.io/openai-agents-python/ and https://openai.github.io/openai-agents-js/.

**Long-running / persistent-state support:** Sessions provide persistent memory within an agent loop; Python docs also describe sandbox agents with resumable sandbox sessions. The docs recommend the SDK over direct Responses API when the runtime should manage turns, tools, guardrails, handoffs, or sessions. Source: https://openai.github.io/openai-agents-python/.

**Multi-agent orchestration support:** Handoffs and agents-as-tools are core primitives. Docs explicitly direct users to agent orchestration guidance for choosing between handoffs and manager-style orchestration. Sources: https://openai.github.io/openai-agents-python/ and https://openai.github.io/openai-agents-js/.

**Human-in-the-loop / question-routing support:** Docs list built-in human-in-the-loop mechanisms and result/resume state. GitHub issues show users pressed for HITL early, suggesting this was a pain point as the SDK matured. Source: https://github.com/openai/openai-agents-python/issues/636.

**Adoption signals:** GitHub API on 2026-04-27: Python repo 25,420 stars, 3,880 forks, 64 open issues; JS repo 2,865 stars, 713 forks, 41 open issues. npm downloads API for 2026-03-28 to 2026-04-26: `@openai/agents` 2,195,942 downloads and `@openai/agents-core` 2,326,093. PyPI metadata: `openai-agents` 0.14.6. Sources: https://api.github.com/repos/openai/openai-agents-python, https://api.github.com/repos/openai/openai-agents-js, https://api.npmjs.org/downloads/point/last-month/%40openai%2Fagents, https://pypi.org/project/openai-agents/.

**Customer segments:** Developers building OpenAI-backed production agents, voice agents, customer-facing apps, sandboxed coding/review/document agents, and teams that want few abstractions over Responses API.

**Documented developer pain:** Issue #636 requested HITL as top priority: https://github.com/openai/openai-agents-python/issues/636. Issue #1061 reports `previous_response_id` failure due to missing tool output, a protocol-state/correlation pain: https://github.com/openai/openai-agents-python/issues/1061. Issue #1156 reports tool call results missing from Realtime API/tracing: https://github.com/openai/openai-agents-python/issues/1156.

**Positioning relative to agent-harness:** OpenAI Agents SDK is a programmable agent runtime. It does not orchestrate desktop CLIs or preserve cross-provider CLI transcripts as graph provenance. It partially solves handoffs, sessions, HITL, and tracing inside OpenAI-centered apps.

**Notes:** OpenAI explicitly calls Agents SDK a production-ready upgrade of Swarm, making Swarm mostly historical context.

### F-5 — Anthropic Claude Agent SDK

**Vendor / repo / docs:** Anthropic; docs: https://code.claude.com/docs/en/agent-sdk/overview; Python repo: https://github.com/anthropics/claude-agent-sdk-python; TypeScript repo: https://github.com/anthropics/claude-agent-sdk-typescript; pricing: https://platform.claude.com/docs/en/docs/about-claude/pricing.

**License model:** Hybrid in practice. Python repo is MIT: https://raw.githubusercontent.com/anthropics/claude-agent-sdk-python/main/LICENSE. TypeScript repo license file states Anthropic PBC all rights reserved and use subject to Anthropic Commercial Terms: https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/LICENSE.md. Docs also say use is governed by Anthropic Commercial Terms, except components with separate licenses. Source: https://code.claude.com/docs/en/agent-sdk/overview.

**Hosted offering:** No separate hosted SDK tier; uses Claude API, Bedrock, Vertex AI, or Azure AI Foundry authentication. Anthropic direct API pricing is usage-based; current docs list model/token pricing and prompt caching/batch discounts. Source: https://platform.claude.com/docs/en/docs/about-claude/pricing.

**Capability surface:** Built-in code/file/web tools; Read, Write, Edit, Bash, Monitor, Glob, Grep, WebSearch, WebFetch, AskUserQuestion; hooks; subagents; MCP; permissions; sessions; skills; slash commands; memory via `CLAUDE.md`; plugins. Source: https://code.claude.com/docs/en/agent-sdk/overview.

**Long-running / persistent-state support:** Sessions can be resumed by session ID and preserve files read, analysis, and conversation history. The SDK can fork sessions. Source: https://code.claude.com/docs/en/agent-sdk/overview.

**Multi-agent orchestration support:** Supports subagents through an Agent tool. Main agent delegates to custom agent definitions; subagent messages include `parent_tool_use_id` for trace correlation. Source: https://code.claude.com/docs/en/agent-sdk/overview.

**Human-in-the-loop / question-routing support:** Stronger than many SDKs for local coding workflows because `AskUserQuestion` is a built-in tool and permissions can require approval. It is still scoped to Claude Agent SDK sessions, not a multi-CLI graph question router. Source: https://code.claude.com/docs/en/agent-sdk/overview.

**Adoption signals:** GitHub API on 2026-04-27: Python repo 6,577 stars, 928 forks, 249 open issues; TypeScript repo 1,340 stars, 152 forks, 120 open issues; demos repo 2,258 stars. npm downloads API for 2026-03-28 to 2026-04-26: `@anthropic-ai/claude-agent-sdk` 16,718,097 downloads. PyPI metadata: `claude-agent-sdk` 0.1.68. Sources: https://api.github.com/repos/anthropics/claude-agent-sdk-python, https://api.github.com/repos/anthropics/claude-agent-sdk-typescript, https://api.npmjs.org/downloads/point/last-month/%40anthropic-ai%2Fclaude-agent-sdk, https://pypi.org/project/claude-agent-sdk/.

**Customer segments:** Coding agents, CI/CD automations, custom developer tools, research/email assistants, and teams that want Claude Code capabilities in applications. Docs explicitly compare CLI for interactive development and SDK for CI/CD/custom applications/production automation. Source: https://code.claude.com/docs/en/agent-sdk/overview.

**Documented developer pain:** Issue #23 asks for non-interactive `/compact`, reflecting context-management pressure: https://github.com/anthropics/claude-agent-sdk-python/issues/23. Issue #137 asks how to pass tool approvals like the CLI: https://github.com/anthropics/claude-agent-sdk-python/issues/137. Issue #327 asks for AskUserQuestion tooling: https://github.com/anthropics/claude-agent-sdk-python/issues/327. Issue #180 requests structured JSON outputs: https://github.com/anthropics/claude-agent-sdk-python/issues/180.

**Positioning relative to agent-harness:** This is the closest to agent-harness in coding-agent shape, but it is Claude-specific. Agent-harness is cross-CLI and wants imposed graph state across Claude/Codex/opencode; Claude Agent SDK gives a programmable Claude Code loop with sessions, hooks, tools, subagents, and AskUserQuestion.

**Notes:** The API/authentication note says third parties generally cannot offer claude.ai login/rate limits for their products, which matters for product packaging.

### F-6 — Google Agent Development Kit (ADK)

**Vendor / repo / docs:** Google; docs: https://adk.dev/; repo: https://github.com/google/adk-python; docs repo: https://github.com/google/adk-docs; Google Cloud Agent Platform pricing: https://cloud.google.com/gemini-enterprise-agent-platform/generative-ai/pricing.

**License model:** OSS, Apache-2.0 per GitHub API.

**Hosted offering:** ADK itself is OSS. Google Cloud / Gemini Enterprise Agent Platform provides managed deployment and model execution. Pricing is model/token and service based. Google pricing page lists, for example, Gemini 2.5 Pro at $1.25/1M input tokens under 200k context and $10/1M text output; Gemini 2.5 Flash at $0.30/1M input and $2.50/1M output; Gemini 3 Pro Preview at $2/1M input and $12/1M output; plus grounding, cache, batch/flex, priority, provisioned throughput, tuning, and partner-model pricing. Source: https://cloud.google.com/gemini-enterprise-agent-platform/generative-ai/pricing.

**Capability surface:** Agents in Python/TypeScript/Go/Java; tools such as Google Search; multi-agent orchestration; graph-based workflows; visual debugging; evaluation framework; partner integrations; AI-assisted coding resources; deployment to Agent Runtime / Cloud Run / GKE; context management with session/memory/tool-output/artifact assembly. Source: https://adk.dev/.

**Long-running / persistent-state support:** Docs say ADK can handle failures and resume stopped tasks. FAQ says ADK manages context by filtering irrelevant events, summarizing older turns, lazy-loading artifacts, and tracking token usage. Source: https://adk.dev/.

**Multi-agent orchestration support:** Explicit: ADK supports multi-agent orchestration and graph-based workflows. Source: https://adk.dev/.

**Human-in-the-loop / question-routing support:** ADK supports managed task structures, tool calls, failure handling, and resume, but public landing evidence does not show a question-routing artifact comparable to LangGraph interrupts or Claude AskUserQuestion.

**Adoption signals:** GitHub API on 2026-04-27: google/adk-python had 19,313 stars, 3,295 forks, 809 open issues; google/adk-docs had 1,321 stars; google/adk-web had 927 stars. npm downloads API for 2026-03-28 to 2026-04-26: `@google/adk` 158,246 downloads. PyPI metadata: `google-adk` 1.31.1. Sources: https://api.github.com/repos/google/adk-python, https://api.github.com/repos/google/adk-docs, https://api.npmjs.org/downloads/point/last-month/%40google%2Fadk, https://pypi.org/project/google-adk/.

**Customer segments:** Google Cloud/Gemini developers, enterprise teams needing cloud deployment/security/governance, and polyglot developers. ADK’s docs explicitly emphasize enterprise scale, Google Cloud deployment, and model/provider flexibility. Source: https://adk.dev/.

**Documented developer pain:** Issue #211 requested context caching support: https://github.com/google/adk-python/issues/211. Issue #2133 was the Q3 2025 roadmap with 47 comments: https://github.com/google/adk-python/issues/2133. Issue #3611 asks for support for Claude skill features, showing cross-framework feature pressure: https://github.com/google/adk-python/issues/3611.

**Positioning relative to agent-harness:** ADK is a cloud-friendly, framework-level agent runtime with context-management claims. It does not address a local desktop orchestrator over multiple existing CLIs. Its context-management language is directly relevant market validation for imposed/managed context, but ADK’s control plane is Google Cloud-oriented.

**Notes:** ADK has the broadest language support claim among the surveyed frameworks.

### F-7 — Letta

**Vendor / repo / docs:** Letta Inc.; docs: https://docs.letta.com/; repo: https://github.com/letta-ai/letta; pricing: https://www.letta.com/pricing; about: https://www.letta.com/about-us.

**License model:** OSS, Apache-2.0 per GitHub API and PyPI metadata. Hosted Letta Cloud/API is proprietary.

**Hosted offering:** Yes. Pricing lists Pro $20/month, Max Lite $100/month, Max $200/month, and API Plan $20/month plus usage. API Plan includes unlimited agents at $0.10/active agent/month, $0.00015/sec tool execution, API key auth, and pay-as-you-go LLM usage. Source: https://www.letta.com/pricing.

**Capability surface:** Stateful agents, memory-first coding harness, Letta API, persistent agents portable across models, desktop app/CLI, skills, memory customization, client-side local filesystem actions, server-side API tools, remote MCP tools. Sources: https://docs.letta.com/ and https://www.letta.com/pricing.

**Long-running / persistent-state support:** Core value proposition. Letta describes itself as building stateful agents that remember, learn, and improve; Letta Code works with persisted agents rather than independent sessions. Source: https://www.letta.com/pricing and https://www.letta.com/about-us.

**Multi-agent orchestration support:** Letta is more memory/state oriented than orchestration-framework oriented. It supports multiple stateful agents and API plan unlimited agents, but the public pages emphasize memory and persistent identity more than multi-agent supervision/routing primitives.

**Human-in-the-loop / question-routing support:** Not a headline capability in the retrieved docs. User interaction exists through desktop/mobile/Telegram and developer APIs, but not a documented blocking question/resume protocol in the pages retrieved.

**Adoption signals:** GitHub API on 2026-04-27: 22,338 stars, 2,370 forks, 80 open issues. PyPI metadata: `letta` 0.16.7. Pricing page and site show 22.3k GitHub stars; startup page says companies like 11x and BILT Rewards use Letta to serve millions of agents in production. Sources: https://api.github.com/repos/letta-ai/letta, https://pypi.org/project/letta/, https://www.letta.com/pricing, https://forms.letta.com/startups.

**Customer segments:** Memory-heavy agents, coding agents, digital employees, companions, startups building perpetual/model-agnostic agents, and app developers needing persistent agent APIs.

**Documented developer pain:** Issue #381 reports archival memory/memory issues: https://github.com/letta-ai/letta/issues/381. Issue #480 asks for an easy-to-use MemGPT API: https://github.com/letta-ai/letta/issues/480. Issue #2255 reports repeated “API call didn't return a message” errors: https://github.com/letta-ai/letta/issues/2255. These are memory reliability/API ergonomics/provider reliability pains.

**Positioning relative to agent-harness:** Letta overlaps strongly with persistent memory/state, which is adjacent but cross-lane as a dedicated memory product. It does not solve cross-CLI orchestration, worker supervision, or imposed graph working sets for existing CLIs. It validates that persistent agents and memory portability are monetizable.

**Notes:** Treat Letta as both framework evidence and cross-lane dedicated memory product.

### F-8 — Agno

**Vendor / repo / docs:** Agno; docs: https://docs.agno.com/; repo: https://github.com/agno-agi/agno; pricing: https://www.agno.com/pricing.

**License model:** OSS, Apache-2.0 per GitHub API and PyPI metadata.

**Hosted offering:** Yes. Pricing lists Free for OSS/local AgentOS control plane; Pro at $150/month for production systems with one live connection, four seats, unlimited usage/monitoring/retention/knowledge/memories/chats, plus $30/month per added seat and $95/month per live connection; Enterprise custom with support SLA, SSO/RBAC, custom solutions, and self-hosted control plane. Source: https://www.agno.com/pricing.

**Capability surface:** Agent SDK for agents/teams/workflows with memory, knowledge, guardrails, and 100+ integrations; AgentOS runtime as FastAPI backend; 50+ endpoints; persisted sessions; tracing; scheduling; RBAC; human approval; OpenTelemetry; Slack/Telegram/WhatsApp/Discord/AG-UI interfaces; Docker/Railway/AWS/GCP deployment. Source: https://docs.agno.com/.

**Long-running / persistent-state support:** AgentOS stores sessions, memory, knowledge, and traces in the user’s database; scheduling/background jobs are built in. Source: https://docs.agno.com/.

**Multi-agent orchestration support:** Agents, teams, and workflows are first-class. Source: https://docs.agno.com/.

**Human-in-the-loop / question-routing support:** Docs list human approval that can pause runs for user confirmation, admin approval, or external execution. Source: https://docs.agno.com/.

**Adoption signals:** GitHub API on 2026-04-27: 39,718 stars, 5,302 forks, 806 open issues. PyPI metadata: `agno` 2.6.2. Source: https://api.github.com/repos/agno-agi/agno and https://pypi.org/project/agno/.

**Customer segments:** Developers shipping agents as production services, teams needing APIs/RBAC/observability around agent systems, and teams that want to wrap agents built in Agno, Claude Agent SDK, LangGraph, or DSPy behind a runtime/control plane. Source: https://docs.agno.com/.

**Documented developer pain:** Issue #2296 reports async tools in teams not awaited properly: https://github.com/agno-agi/agno/issues/2296. Issue #5741 reports exponential session history growth due to recursive history storage: https://github.com/agno-agi/agno/issues/5741. Issue #3951 reports token counts not emitted to external monitoring: https://github.com/agno-agi/agno/issues/3951. These pains map to multi-agent async correctness, state blowup, and observability gaps.

**Positioning relative to agent-harness:** Agno’s AgentOS is close to a production runtime/control plane for agents, but it is server/API oriented. It could compete for developers who want a local-ish backend and persisted sessions, but it does not manage existing coding CLIs or impose working sets across them.

**Notes:** Agno is increasingly positioned as “runtime for agentic software,” not just a Python agent helper.

### F-9 — Mastra

**Vendor / repo / docs:** Mastra; docs: https://mastra.ai/docs; repo: https://github.com/mastra-ai/mastra; pricing: https://mastra.ai/pricing; funding blog: https://mastra.ai/blog/series-a.

**License model:** Hybrid. License file says Apache-2.0 outside enterprise (`ee/`) directories, with `ee/` under separate license. Source: https://raw.githubusercontent.com/mastra-ai/mastra/main/LICENSE.md.

**Hosted offering:** Yes. Pricing lists Platform Starter free with unlimited users/deployments, Studio, 100k observability events, Server, 24h CPU uptime, and 10GB data egress. Teams is $250/team/month with multiple teams, custom SSO, SOC 2 docs, 250h CPU time, and 100GB egress. Enterprise custom. Memory Gateway is a distinct memory product: Starter free with 100k memory tokens; Teams $250/team/month with 1M memory tokens and BYOK; add-on tokens $10/1M. Source: https://mastra.ai/pricing.

**Capability surface:** TypeScript agents, workflows, tools, model router, Studio UI, framework integrations (Next.js, React, Astro, Express, SvelteKit, Hono), observability/evals, memory/context, templates, Slack/browser/GitHub PR review/database/PDF agents. Sources: https://mastra.ai/docs and https://mastra.ai/.

**Long-running / persistent-state support:** Mastra provides memory/context and server/studio platform features, but retrieved docs emphasize productizing agents and observability more than durable execution semantics. Memory Gateway is cross-lane dedicated memory.

**Multi-agent orchestration support:** Mastra supports agents and workflows; the repo/issues show “network agent” and agent.network patterns. It is a JS/TS agent framework rather than a workflow engine like Inngest.

**Human-in-the-loop / question-routing support:** Not a headline feature in retrieved primary docs. It can be built in workflows but no explicit question-routing primitive was found in the scraped landing/pricing pages.

**Adoption signals:** GitHub API on 2026-04-27: 23,370 stars, 1,968 forks, 398 open issues. npm downloads API for 2026-03-28 to 2026-04-26: `@mastra/core` 2,695,662 downloads. Official Series A post says Mastra raised $22M led by Spark Capital, bringing total funding to $35M. It names teams/users including Brex, Sanity, Factorial, Indeed, Marsh McLennan, MongoDB, Workday, Salesforce, and Replit. Sources: https://api.github.com/repos/mastra-ai/mastra, https://api.npmjs.org/downloads/point/last-month/%40mastra%2Fcore, https://mastra.ai/blog/series-a.

**Customer segments:** TypeScript-first application developers, product teams embedding agents, internal copilots, data analysis agents, content automation, DevOps/SRE automation, sales/GTM workflows. Source: https://mastra.ai/docs.

**Documented developer pain:** Issue #10092 reports unpredictable behavior from the network agent using `@mastra/ai-sdk` beta: https://github.com/mastra-ai/mastra/issues/10092. Issue #9024 reports duplicate assistant messages with `agent.network()` plus memory: https://github.com/mastra-ai/mastra/issues/9024. Issue #5470 tracks AI SDK v5 support: https://github.com/mastra-ai/mastra/issues/5470. These show framework/provider dependency churn and multi-agent/memory message duplication.

**Positioning relative to agent-harness:** Mastra competes for “build an agent app in TS,” not “orchestrate existing desktop CLIs.” Its Memory Gateway is cross-lane. It validates demand for TS-native agent infrastructure and platform packaging.

**Notes:** Mastra is one of the few with clear recent funding and JS-first traction.

### F-10 — Inngest AgentKit

**Vendor / repo / docs:** Inngest; docs: https://agentkit.inngest.com/; repo: https://github.com/inngest/agent-kit; pricing: https://www.inngest.com/pricing.

**License model:** OSS, Apache-2.0 for AgentKit per GitHub API. Inngest hosted platform is proprietary; core `inngest-js` is GPL-3.0 per GitHub API, which matters if adopting deeper runtime code.

**Hosted offering:** AgentKit is a library; Inngest platform pricing applies when using hosted durable execution. Hobby is $0/month with 50k executions/month, 5 concurrent steps, 50 realtime connections, 3 users. Pro starts at $75/month with 1M included executions, 100+ concurrent steps, 1000+ realtime connections, 15+ users. Overage pricing includes $50 per 1M executions on Pro; events are 5M/month included then $0.5 per 1M. Enterprise custom. Source: https://www.inngest.com/pricing.

**Capability surface:** TypeScript agents, networks, routers, state, tools, MCP tools, OpenAI/Anthropic/Gemini/OpenAI-compatible models, UI streaming, local live traces/logs with Inngest Dev Server, integrations with E2B, Browserbase, Smithery, and Daytona. Source: https://agentkit.inngest.com/.

**Long-running / persistent-state support:** AgentKit itself records network state. Inngest as the runtime is durable execution infrastructure, making this one of the strongest long-running candidates when paired with Inngest functions. Source: https://agentkit.inngest.com/ and https://www.inngest.com/pricing.

**Multi-agent orchestration support:** First-class. Agents are combined into Networks; Routers choose which agent should be called; State is used by routers, agents, and tools. Source: https://agentkit.inngest.com/.

**Human-in-the-loop / question-routing support:** Not emphasized in AgentKit landing docs. Inngest’s durable functions can wait/retry/event-resume, but no explicit AgentKit user-question primitive was found in the retrieved docs.

**Adoption signals:** GitHub API on 2026-04-27: AgentKit 847 stars, 122 forks, 35 open issues; `inngest-js` 936 stars. npm downloads API for 2026-03-28 to 2026-04-26: `@inngest/agent-kit` 44,939. Inngest pricing page names SoundCloud, Fey, and GitBook customer stories. Sources: https://api.github.com/repos/inngest/agent-kit, https://api.npmjs.org/downloads/point/last-month/%40inngest%2Fagent-kit, https://www.inngest.com/pricing.

**Customer segments:** Developers already using durable execution, TypeScript backend teams, production workflows with agents embedded into event-driven systems.

**Documented developer pain:** Issue #81 reports ESM/CommonJS packaging trouble: https://github.com/inngest/agent-kit/issues/81. Issue #173 says MCP support is not compatible with Gemini models: https://github.com/inngest/agent-kit/issues/173. Issue #244 reports a deep research example failing due to undefined `step`: https://github.com/inngest/agent-kit/issues/244.

**Positioning relative to agent-harness:** Inngest AgentKit is a backend framework and workflow-engine-backed runtime. It overlaps on durable orchestration, but not desktop CLI supervision or local graph working-set imposition.

**Notes:** Smaller framework traction, but strong durability story through Inngest’s existing product.

### F-11 — DSPy

**Vendor / repo / docs:** Stanford NLP; docs: https://dspy.ai/; repo: https://github.com/stanfordnlp/dspy; PyPI: https://pypi.org/project/dspy/.

**License model:** OSS, MIT per GitHub API and PyPI metadata.

**Hosted offering:** None. DSPy is OSS; users pay their model providers. Docs include examples with OpenAI, Anthropic, Databricks, Gemini, local Ollama/SGLang, and any LiteLLM provider. Source: https://dspy.ai/.

**Capability surface:** Declarative signatures, modules, predictors, ChainOfThought, ReAct agents, RAG modules, structured extraction, multi-stage pipelines, optimizers (BootstrapRS, GEPA, MIPROv2, BootstrapFinetune), automatic caching, provider abstraction through LiteLLM, and evaluation/metrics-driven compilation. Source: https://dspy.ai/.

**Long-running / persistent-state support:** Weak as an orchestration runtime. DSPy is about compiling/optimizing LM programs, not durable sessions. Caching exists, but not a long-running agent state system.

**Multi-agent orchestration support:** Agent capabilities exist through `dspy.ReAct` and composable modules; multi-stage pipelines are natural. It is not primarily a multi-agent supervision framework.

**Human-in-the-loop / question-routing support:** Not a core capability in retrieved docs.

**Adoption signals:** GitHub API on 2026-04-27: 34,029 stars, 2,845 forks, 510 open issues. PyPI metadata: `dspy` 3.2.0. Docs say 250 contributors and that DSPy introduced hundreds of thousands of people to modular LM programs. Source: https://api.github.com/repos/stanfordnlp/dspy, https://pypi.org/project/dspy/, https://dspy.ai/.

**Customer segments:** Researchers, ML engineers, teams optimizing prompts/programs/evals, RAG builders, and model-portability users.

**Documented developer pain:** Issue #1344 reports optimized COPRO prompt not being used when calling the LLM, showing optimizer/runtime mental-model confusion: https://github.com/stanfordnlp/dspy/issues/1344. Docs themselves warn optimizer runs can range from cents to tens of dollars and that a typical simple optimization costs about $2 and takes about 20 minutes, showing cost/latency as a documented limitation. Source: https://dspy.ai/.

**Positioning relative to agent-harness:** DSPy is not a competitor for desktop orchestration. It is relevant because it demonstrates developer appetite for programmable, optimizable LM systems rather than prompt strings.

**Notes:** DSPy is an important “programming model” signal, not a deployed agent runtime signal.

### F-12 — BeeAI Framework

**Vendor / repo / docs:** IBM-origin / BeeAI, hosted under Linux Foundation open governance per docs; docs: https://framework.beeai.dev/; repo: https://github.com/i-am-bee/beeai-framework; PyPI: https://pypi.org/project/beeai-framework/.

**License model:** OSS, Apache-2.0 per GitHub API and PyPI metadata.

**Hosted offering:** No framework hosted tier found in primary docs. IBM watsonx / watsonx Orchestrate is cross-lane cloud agent/product territory.

**Capability surface:** Production-grade multi-agent systems; Python and TypeScript parity; agents with constraints; deterministic rule enforcement; dynamic workflows with parallelism, retries, replanning; declarative YAML orchestration; caching/memory/resource management; OpenTelemetry observability; MCP and A2A; 10+ LLM providers including Ollama, Groq, OpenAI, and Watsonx.ai. Source: https://framework.beeai.dev/.

**Long-running / persistent-state support:** Docs mention memory optimization and resource management, but the retrieved landing page does not describe durable checkpoint/resume semantics.

**Multi-agent orchestration support:** Strong positioning: production-grade multi-agent systems, dynamic workflows, YAML orchestration, parallelism/retries/replanning. Source: https://framework.beeai.dev/.

**Human-in-the-loop / question-routing support:** Not visible in retrieved landing docs.

**Adoption signals:** GitHub API on 2026-04-27: 3,226 stars, 425 forks, 7 open issues. npm downloads API for 2026-03-28 to 2026-04-26: `beeai-framework` 12,647. PyPI metadata: `beeai-framework` 0.1.79. Sources: https://api.github.com/repos/i-am-bee/beeai-framework, https://api.npmjs.org/downloads/point/last-month/beeai-framework, https://pypi.org/project/beeai-framework/.

**Customer segments:** Enterprise developers, IBM/watsonx ecosystem users, and teams requiring governance/constraint enforcement in multi-agent systems.

**Documented developer pain:** GitHub issue search results show compatibility and docs gaps: issue #1324 reports LangChain compatibility breakage with v0.3 needing `langchain-classic`: https://github.com/i-am-bee/beeai-framework/issues/1324. Issue #1193 is an epic to expand lifecycle events and middleware: https://github.com/i-am-bee/beeai-framework/issues/1193. These indicate dependency churn and lifecycle extensibility needs.

**Positioning relative to agent-harness:** BeeAI overlaps in multi-agent governance and constraints, but not local CLI session orchestration. Its A2A/MCP-native positioning is relevant to cross-agent interoperability.

**Notes:** The Linux Foundation governance claim is a differentiator for enterprise trust.

### F-13 — OpenAI Swarm

**Vendor / repo / docs:** OpenAI; repo/docs: https://github.com/openai/swarm.

**License model:** OSS, MIT.

**Hosted offering:** None. It is explicitly experimental/educational and replaced by OpenAI Agents SDK for production. Source: https://github.com/openai/swarm.

**Capability surface:** Agents, handoffs, Python functions as tools, context variables, streaming, debug mode, max turns, tool execution loop, examples for triage, airline, support bot, personal shopper, weather, and evals. Source: https://github.com/openai/swarm.

**Long-running / persistent-state support:** Weak by design. README states Swarm is powered by Chat Completions and is stateless between calls; response fields can be passed into the next call to continue, but no persistence layer exists. Source: https://github.com/openai/swarm.

**Multi-agent orchestration support:** Handoffs are the core primitive. If multiple handoff functions are called, only the last handoff is used, which is a documented limitation. Source: https://github.com/openai/swarm.

**Human-in-the-loop / question-routing support:** None beyond client-controlled loops.

**Adoption signals:** GitHub API on 2026-04-27: 21,399 stars, 2,278 forks, 27 open issues. No npm package. Source: https://api.github.com/repos/openai/swarm.

**Customer segments:** Educational users, prototype builders, developers learning multi-agent handoff patterns.

**Documented developer pain:** The README itself documents the central limitations: stateless between calls, educational resource, not production, and replaced by Agents SDK. Source: https://github.com/openai/swarm.

**Positioning relative to agent-harness:** Not a competitor. It is useful evidence that handoffs became a canonical primitive, but it lacks durable state, HITL, and production support.

**Notes:** Treat Swarm as historical pattern evidence.

### F-14 — Vercel AI SDK

**Vendor / repo / docs:** Vercel; docs: https://ai-sdk.dev/docs/introduction; repo: https://github.com/vercel/ai; pricing: https://vercel.com/pricing; license: https://raw.githubusercontent.com/vercel/ai/main/LICENSE.

**License model:** OSS, Apache-2.0.

**Hosted offering:** AI SDK is OSS. Vercel platform pricing applies for hosting and AI Cloud. Vercel Pro is $20/month plus usage with $20 included credit. Vercel pricing page lists Vercel Agent beta at $0.30/action plus pass-through token cost, and AI Gateway features such as observability, BYOK, fallback, load balancing, spend monitoring, and retries. Source: https://vercel.com/pricing.

**Capability surface:** TypeScript toolkit for AI apps and agents; AI SDK Core for text, structured objects, tool calls, agents, MCP tools, middleware, provider management, telemetry, testing; AI SDK UI for chat hooks, message persistence, resume streams, tool usage, generative UI; support for many providers including Vercel AI Gateway, OpenAI, Anthropic, Google, xAI, Azure, Bedrock, Groq, Mistral, Fireworks, DeepSeek, Cerebras, and others. Source: https://ai-sdk.dev/docs/introduction.

**Long-running / persistent-state support:** UI layer supports chatbot message persistence and resume streams. Agent docs include memory and loop control. It is not a durable workflow engine by itself; Vercel Workflow/Queues/Sandbox are platform products and cross-lane if treated as hosted agent infrastructure. Source: https://ai-sdk.dev/docs/introduction.

**Multi-agent orchestration support:** AI SDK v6 docs have an Agents section covering building agents, workflows, loop control, memory, and subagents. Source: https://ai-sdk.dev/docs/introduction.

**Human-in-the-loop / question-routing support:** Tool usage and UI streams support interactive apps, but no retrieved primary evidence of a durable question artifact/resume protocol equivalent to LangGraph interrupts or Claude AskUserQuestion.

**Adoption signals:** GitHub API on 2026-04-27: 23,836 stars, 4,282 forks, 1,569 open issues. npm downloads API for 2026-03-28 to 2026-04-26: `ai` 45,635,454 downloads. Docs landing page says Vercel AI SDK is trusted by OpenAI, Photoroom, Leonardo.ai, and Zapier. Sources: https://api.github.com/repos/vercel/ai, https://api.npmjs.org/downloads/point/last-month/ai, https://ai-sdk.dev/docs/introduction.

**Customer segments:** Frontend/full-stack JS developers building AI apps, Next.js/Vercel teams, chatbot/generative UI builders, provider-agnostic model consumers.

**Documented developer pain:** Issue #1512 reports tool call results not appearing in `messages`: https://github.com/vercel/ai/issues/1512. Issue #3944 reports toolChoice causing an endless loop with `streamText`: https://github.com/vercel/ai/issues/3944. Issue #7919 reports v5 streaming hanging after `streamText`: https://github.com/vercel/ai/issues/7919. These are directly relevant to tool-call protocol state, streaming complexity, and frontend/backend integration pain.

**Positioning relative to agent-harness:** Vercel AI SDK is a web app SDK, not a desktop orchestrator. It is very relevant for UI/protocol patterns and adoption gradient: massive npm traction shows JS developers are building AI apps, but it does not manage coding CLI sessions or local cross-provider workers.

**Notes:** Vercel’s hosted AI Cloud and Vercel Agent are cross-lane hosted cloud agent/product signals, not SDK-only signals.

## Cross-Framework Patterns

**License/business models:** The dominant model is OSS core plus paid hosted control plane or observability/deployment tier. LangGraph/LangChain, CrewAI, Letta, Agno, Mastra, Inngest, Vercel, and Google ADK all show some version of open framework plus proprietary SaaS/cloud/control plane. AutoGen, DSPy, BeeAI, and Swarm are closer to OSS-only frameworks, though BeeAI has IBM ecosystem adjacency and AutoGen has Azure ecosystem adjacency. Anthropic’s SDK is more constrained: Python is MIT, TypeScript and docs point to Anthropic Commercial Terms.

**Hosted-tier pricing patterns:** Hosted offerings split into four charging shapes. First, per-seat plus usage: LangSmith Plus at $39/seat/month, Vercel Pro at $20/month, Mastra Teams at $250/team/month, Agno Pro at $150/month. Second, execution/run metering: LangSmith deployment/Fleet run pricing, Inngest executions, CrewAI workflow executions. Third, token/model pass-through: OpenAI, Anthropic, Google Vertex/Gemini, Vercel AI Gateway, CrewAI/Letta BYOK/pay-as-you-go. Fourth, state/memory/control-plane pricing: Letta active agents/tool execution seconds, Mastra Memory Gateway tokens/storage, Agno live connections.

**Adoption gradient:** Vercel AI SDK has the largest npm signal (`ai` 45.6M last-month downloads), followed by Anthropic Claude Agent SDK TS package (16.7M), LangChain/LangGraph JS packages (~8.8M/~8.4M), Mastra core (2.7M), and OpenAI Agents JS (2.2M). On GitHub, LangChain (135k), AutoGen (57.5k), CrewAI (50.1k), Agno (39.7k), DSPy (34.0k), LangGraph (30.6k), OpenAI Agents Python (25.4k), Vercel AI SDK (23.8k), Mastra (23.4k), Letta (22.3k), and Swarm (21.4k) show broad public interest. Enterprise commercial traction is clearest for LangChain/LangSmith, CrewAI AMP, Vercel, Inngest, and Mastra.

**Customer segment split:** Python research/prototyping still clusters around AutoGen, DSPy, CrewAI, LangGraph, Letta, and Agno. TypeScript app developers cluster around Vercel AI SDK, Mastra, Inngest AgentKit, OpenAI Agents JS, and Google ADK JS. Enterprise cloud buyers cluster around LangSmith, CrewAI AMP, Google ADK/Agent Platform, Vercel AI Cloud, Agno AgentOS, and Inngest. Coding-agent developers cluster around Claude Agent SDK, OpenAI Agents sandbox agents, Letta Code, and agent-harness’s target space.

**Common documented limitations and pain:** Tool-call protocol state is a recurring failure mode: Vercel AI SDK issues show missing tool results and tool loops; OpenAI Agents issues show missing tool output / tracing mismatch; CrewAI has fabricated tool-use reports; Agno has async tools not awaited. Long-running state creates storage and replay issues: LangGraph checkpoint/database errors, Agno recursive session history growth, Letta memory issues, ADK context caching requests. Streaming and UI integration remain brittle: Vercel streaming hangs, OpenAI Realtime tracing/tool issues, LangGraph streaming issues. API churn and dependency churn are constant: Mastra AI SDK v5 support, BeeAI LangChain compatibility, AutoGen migration from 0.2 to newer packages, LangGraph v1 roadmap.

**Frameworks do not converge on agent-harness’s exact problem:** They converge on primitives: agents, tools, handoffs/subagents, state/session/memory, HITL/approval, tracing, deployment, and provider abstraction. They do not generally manage independent desktop CLI agents, cross-CLI transcript/provenance ingestion, imposed working sets, summary-contract graph navigation, or multi-workstream desktop UX. The closest overlaps are Claude Agent SDK for coding-agent sessions, LangGraph for graph-state/HITL, Letta for persistent agents, and Agno/Inngest for production runtime/control-plane.

## Cross-Lane Signals

**Hosted cloud agent products:** LangSmith Deployment/Fleet, CrewAI AMP, Google Gemini Enterprise Agent Platform / Vertex AI Agent Builder, Vercel AI Cloud / Vercel Agent, Inngest hosted durable execution, Agno Control Plane, and Mastra Platform are cross-lane if the research question shifts from SDK/framework adoption to cloud agent products.

**Local desktop assistants:** Letta Code desktop app, Claude Code CLI/SDK, and any Claude/Codex/opencode desktop coding assistant behavior are cross-lane unless used only as framework evidence. Agent-harness should treat these as product/UX competition, not framework competition.

**Dedicated memory products:** Letta’s core platform and Mastra Memory Gateway are cross-lane dedicated memory products. They are still useful evidence that persistent memory is monetizable, but they do not answer the SDK orchestration question alone.

**KM tools:** RAG templates, internal knowledge-base assistants, and memory/retrieval storage products inside Vercel/Mastra/Letta/Agno are KM-adjacent. Stop at framework signals unless the next stage explicitly covers KM.

## Open Evidence Gaps

- PyPI download counts are absent from primary PyPI package pages; any PyPI download number would require third-party sources such as pepy.tech or pypistats and is therefore excluded from primary-source metrics.
- Some issue rankings were limited by unauthenticated GitHub API rate limiting. The report still includes primary GitHub issue URLs, but not a complete comment-sorted issue census for DSPy, BeeAI, Swarm, and Vercel.
- Funding data is included only where official/vendor primary sources were found quickly: LangChain and Mastra. CrewAI, Agno, Letta, Inngest, and others may have funding announcements in primary or SEC sources, but they were not needed to establish framework traction and are left as follow-up evidence gaps.
- Human-in-the-loop details often require deeper docs than landing pages. LangGraph, OpenAI Agents, Claude Agent SDK, CrewAI, and Agno show explicit HITL/approval/question primitives in retrieved sources; others may support equivalent patterns without prominent primary evidence in the pages scraped.
- Customer claims are vendor-stated and not independently verified. This is acceptable for primary-source market positioning, but not for proof of production depth.

## Citations

- LangGraph overview: https://docs.langchain.com/oss/python/langgraph/overview
- LangGraph GitHub API: https://api.github.com/repos/langchain-ai/langgraph
- LangChain GitHub API: https://api.github.com/repos/langchain-ai/langchain
- LangGraph npm downloads: https://api.npmjs.org/downloads/point/last-month/%40langchain%2Flanggraph
- LangChain npm downloads: https://api.npmjs.org/downloads/point/last-month/langchain
- LangSmith pricing: https://www.langchain.com/pricing
- LangChain Series B: https://www.blog.langchain.com/series-b/
- LangGraph issues: https://github.com/langchain-ai/langgraph/issues/4973, https://github.com/langchain-ai/langgraph/issues/3716, https://github.com/langchain-ai/langgraph/issues/740, https://github.com/langchain-ai/langgraph/issues/2920
- AutoGen docs: https://microsoft.github.io/autogen/stable/index.html
- AutoGen repo/API: https://github.com/microsoft/autogen, https://api.github.com/repos/microsoft/autogen
- AutoGen license: https://github.com/microsoft/autogen/blob/main/LICENSE-CODE
- AutoGen PyPI: https://pypi.org/project/autogen-agentchat/
- AutoGen issues: https://github.com/microsoft/autogen/issues/7487, https://github.com/microsoft/autogen/issues/279, https://github.com/microsoft/autogen/issues/3345
- CrewAI docs/repo/pricing/PyPI: https://docs.crewai.com/, https://github.com/crewAIInc/crewAI, https://api.github.com/repos/crewAIInc/crewAI, https://www.crewai.com/pricing, https://pypi.org/project/crewai/
- CrewAI issues: https://github.com/crewAIInc/crewAI/issues/3154, https://github.com/crewAIInc/crewAI/issues/2885, https://github.com/crewAIInc/crewAI/issues/430
- OpenAI Agents docs/repos/pricing: https://openai.github.io/openai-agents-python/, https://openai.github.io/openai-agents-js/, https://github.com/openai/openai-agents-python, https://github.com/openai/openai-agents-js, https://openai.com/api/pricing/
- OpenAI Agents metrics: https://api.github.com/repos/openai/openai-agents-python, https://api.github.com/repos/openai/openai-agents-js, https://api.npmjs.org/downloads/point/last-month/%40openai%2Fagents, https://api.npmjs.org/downloads/point/last-month/%40openai%2Fagents-core, https://pypi.org/project/openai-agents/
- OpenAI Agents issues: https://github.com/openai/openai-agents-python/issues/636, https://github.com/openai/openai-agents-python/issues/1061, https://github.com/openai/openai-agents-python/issues/1156
- Claude Agent SDK docs/pricing/repos: https://code.claude.com/docs/en/agent-sdk/overview, https://platform.claude.com/docs/en/docs/about-claude/pricing, https://github.com/anthropics/claude-agent-sdk-python, https://github.com/anthropics/claude-agent-sdk-typescript
- Claude Agent SDK metrics/licenses: https://api.github.com/repos/anthropics/claude-agent-sdk-python, https://api.github.com/repos/anthropics/claude-agent-sdk-typescript, https://api.npmjs.org/downloads/point/last-month/%40anthropic-ai%2Fclaude-agent-sdk, https://pypi.org/project/claude-agent-sdk/, https://raw.githubusercontent.com/anthropics/claude-agent-sdk-python/main/LICENSE, https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/LICENSE.md
- Claude Agent SDK issues: https://github.com/anthropics/claude-agent-sdk-python/issues/23, https://github.com/anthropics/claude-agent-sdk-python/issues/137, https://github.com/anthropics/claude-agent-sdk-python/issues/327, https://github.com/anthropics/claude-agent-sdk-python/issues/180
- Google ADK docs/repos/pricing: https://adk.dev/, https://github.com/google/adk-python, https://github.com/google/adk-docs, https://cloud.google.com/gemini-enterprise-agent-platform/generative-ai/pricing
- Google ADK metrics: https://api.github.com/repos/google/adk-python, https://api.github.com/repos/google/adk-docs, https://api.npmjs.org/downloads/point/last-month/%40google%2Fadk, https://pypi.org/project/google-adk/
- Google ADK issues: https://github.com/google/adk-python/issues/211, https://github.com/google/adk-python/issues/2133, https://github.com/google/adk-python/issues/3611
- Letta docs/repo/pricing/about/startups: https://docs.letta.com/, https://github.com/letta-ai/letta, https://www.letta.com/pricing, https://www.letta.com/about-us, https://forms.letta.com/startups
- Letta metrics/issues: https://api.github.com/repos/letta-ai/letta, https://pypi.org/project/letta/, https://github.com/letta-ai/letta/issues/381, https://github.com/letta-ai/letta/issues/480, https://github.com/letta-ai/letta/issues/2255
- Agno docs/repo/pricing/PyPI/issues: https://docs.agno.com/, https://github.com/agno-agi/agno, https://www.agno.com/pricing, https://api.github.com/repos/agno-agi/agno, https://pypi.org/project/agno/, https://github.com/agno-agi/agno/issues/2296, https://github.com/agno-agi/agno/issues/5741, https://github.com/agno-agi/agno/issues/3951
- Mastra docs/repo/pricing/funding/license/metrics/issues: https://mastra.ai/docs, https://github.com/mastra-ai/mastra, https://mastra.ai/pricing, https://mastra.ai/blog/series-a, https://raw.githubusercontent.com/mastra-ai/mastra/main/LICENSE.md, https://api.github.com/repos/mastra-ai/mastra, https://api.npmjs.org/downloads/point/last-month/%40mastra%2Fcore, https://github.com/mastra-ai/mastra/issues/10092, https://github.com/mastra-ai/mastra/issues/9024, https://github.com/mastra-ai/mastra/issues/5470
- Inngest AgentKit docs/repo/pricing/metrics/issues: https://agentkit.inngest.com/, https://github.com/inngest/agent-kit, https://www.inngest.com/pricing, https://api.github.com/repos/inngest/agent-kit, https://api.npmjs.org/downloads/point/last-month/%40inngest%2Fagent-kit, https://github.com/inngest/agent-kit/issues/81, https://github.com/inngest/agent-kit/issues/173, https://github.com/inngest/agent-kit/issues/244
- DSPy docs/repo/PyPI/issue: https://dspy.ai/, https://github.com/stanfordnlp/dspy, https://api.github.com/repos/stanfordnlp/dspy, https://pypi.org/project/dspy/, https://github.com/stanfordnlp/dspy/issues/1344
- BeeAI docs/repo/PyPI/metrics/issues: https://framework.beeai.dev/, https://github.com/i-am-bee/beeai-framework, https://api.github.com/repos/i-am-bee/beeai-framework, https://api.npmjs.org/downloads/point/last-month/beeai-framework, https://pypi.org/project/beeai-framework/, https://github.com/i-am-bee/beeai-framework/issues/1324, https://github.com/i-am-bee/beeai-framework/issues/1193
- OpenAI Swarm repo/API: https://github.com/openai/swarm, https://api.github.com/repos/openai/swarm
- Vercel AI SDK docs/repo/pricing/license/metrics/issues: https://ai-sdk.dev/docs/introduction, https://github.com/vercel/ai, https://vercel.com/pricing, https://raw.githubusercontent.com/vercel/ai/main/LICENSE, https://api.github.com/repos/vercel/ai, https://api.npmjs.org/downloads/point/last-month/ai, https://github.com/vercel/ai/issues/1512, https://github.com/vercel/ai/issues/3944, https://github.com/vercel/ai/issues/7919
