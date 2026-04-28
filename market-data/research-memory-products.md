# Stage 0b Market Research: Memory-Layer / Context-Engineering Products

Verified primarily from vendor docs, pricing pages, GitHub repos, and named vendor posts on 2026-04-28 UTC. This report stays in the commercial memory/context-engineering lane. Where a product is mainly a vector/graph/database, I treat only its public "agent memory" positioning as in scope and flag the broader database/agent platform market as cross-lane.

## Product Walk

### P-1 - Letta

**Vendor / repo / docs / pricing:** Letta product/docs: https://docs.letta.com/guides/agents/memory, ADE docs: https://docs.letta.com/memory/, plans: https://docs.letta.com/guides/api/plans, Letta/MemGPT naming: https://www.letta.com/blog/memgpt-and-letta, repo: https://github.com/letta-ai/letta, funding announcement: https://www.prnewswire.com/news-releases/berkeley-ai-research-lab-spinout-letta-raises-10m-seed-financing-led-by-felicis-to-build-ai-with-memory-302257004.html

**Positioning:** Letta describes itself as the MemGPT-derived framework/cloud for "stateful agents" with transparent memory, state, prompts, and tools.

**Architecture (briefly):** Hybrid hierarchical agent memory. Docs say Letta is based on "self-editing memory, memory hierarchy, and intelligent context window management" (https://docs.letta.com/guides/agents/memory).

**Capability surface:** Persistent memory blocks; agent state visualization in ADE; direct read/write of agent memory; tool and data-source management without recreating an agent; agent import/export via `.af`; MemGPT-style context-window management; ADE simulator/debugger; Letta Code MemFS git-backed context repository for coding agents.

**Persistence model:** Letta Cloud is vendor-hosted with storage quotas; open source can be customer-hosted. Letta Code memory is a git-backed MemFS folder of markdown files (https://docs.letta.com/letta-code/memory/).

**SDK languages:** Python SDK, TypeScript/JavaScript SDK, REST API, and ADE/web UI are the visible developer surface in docs and repo.

**Pricing tiers:** Free: 5,000 monthly credits, API access, ADE editing, 2 agent templates, 1 GB storage. Pro: $20/month, 20,000 monthly credits, pay-as-you-go overage, unlimited agents, 20 templates, 10 GB storage. Enterprise: higher quotas, dedicated support, RBAC, SSO, and private model deployment options. Credits cover LLM inference and CPU cycles; Max Mode costs more for longer contexts (https://docs.letta.com/guides/api/plans).

**Customer segments:** Developers building stateful agents; coding-agent users via Letta Code; enterprise buyers needing SSO/private deployment. PR Newswire says Letta spun out of UC Berkeley BAIR and raised $10M seed to build AI with memory.

**Documented limitations:** Pricing docs make cost variable by model tier and Max Mode. Letta Code docs constrain where MemFS can be modified: the agent can chat in ADE/chat.letta.com but "only be able to modify its memory within Letta Code" (https://docs.letta.com/letta-code/memory/).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `letta-ai/letta` had 22,338 stars, 2,370 forks, 80 open issues, Apache-2.0 license. Funding: $10M seed led by Felicis, per PR Newswire.

**Positioning relative to agent-harness:** Letta is the closest architectural analogue: it exposes memory blocks, ADE visibility, and agent-owned editing. It still appears primarily model/agent-consulted: the agent owns memory tools and self-edits what is recalled or written. Agent-harness differs by imposing a graph-derived working set at every orchestrator turn and by requiring snapshot-walk-then-merge semantics for concurrent optimizer mutation.

**Notes:** Letta is a direct competitor/analogue for "AI with memory" but also an agent framework/cloud. Broader agent runtime/cloud hosting is cross-lane.

### P-2 - Mem0

**Vendor / repo / docs / pricing:** Product: https://mem0.ai/, platform overview: https://docs.mem0.ai/overview, open source docs: https://docs.mem0.ai/open-source, memory types: https://docs.mem0.ai/core-concepts/memory-types, API: https://docs.mem0.ai/api-reference, pricing: https://mem0.ai/pricing, repo: https://github.com/mem0ai/mem0, funding: https://mem0.ai/series-a

**Positioning:** "Universal memory layer for AI Agents" and "memory engine that keeps conversations contextual" (https://github.com/mem0ai/mem0, https://docs.mem0.ai/overview).

**Architecture (briefly):** Hybrid vector + graph memory. FAQ says Mem0 extracts facts/preferences and distributes them across a vector database and graph database (https://docs.mem0.ai/platform/faqs).

**Capability surface:** Add/search/get/update/delete memories; user/session/agent/org memory scopes; semantic search with filters; managed vector store, graph services and rerankers; CLI; API keys; integrations with LangChain, CrewAI, Vercel AI SDK and others; graph memory on Pro; analytics and projects.

**Persistence model:** Platform is vendor-hosted. Open source is self-hosted; docs also mention local deployment constraints such as AWS Lambda filesystem behavior (https://docs.mem0.ai/platform/faqs).

**SDK languages:** Python (`mem0ai`), JavaScript/TypeScript (`mem0ai` npm), REST API, CLI.

**Pricing tiers:** Hobby free: 10,000 memories, unlimited end users, 1,000 retrieval API calls/month, community support. Starter: $19/month, 50,000 memories, unlimited end users, 5,000 retrieval calls/month. Pro: $249/month, unlimited memories/end users, 50,000 retrieval calls/month, private Slack, graph memory, analytics, multiple projects. Usage-based custom pricing is available (https://mem0.ai/pricing).

**Customer segments:** Consumer personalization, healthcare, education, ecommerce, customer support, sales/CRM are vendor-cited verticals (https://mem0.ai/). Enterprise/security buyers: SOC 2 and HIPAA claims appear on the product page. Funding page says Mem0 raised $24M total; Series A led by Basis Set with Peak XV, GitHub Fund and YC participation (https://mem0.ai/series-a).

**Documented limitations:** Docs warn: "Avoid storing secrets or unredacted PII" because memory is retrievable by design (https://docs.mem0.ai/core-concepts/memory-types). Managed evaluation docs state that scores reflect proprietary platform optimizations not available in OSS (https://docs.mem0.ai/core-concepts/memory-evaluation). A GitHub issue titled "What we found after auditing 10,134 mem0 entries: 97.8% were junk" flags quality risks in memory write paths (https://github.com/mem0ai/mem0/issues/4573).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `mem0ai/mem0` had 54,230 stars, 6,105 forks, 238 open issues. The GitHub README search result showed 304 releases and latest Node SDK v3.0.0 on 2026-04-16. Funding: $24M total per Mem0.

**Positioning relative to agent-harness:** Mem0 overlaps on durable memory and personalization, but it is an API layer that returns relevant memories on request. Agent-harness candidates may still use Mem0 as a backing consulted memory store, but Mem0 does not impose a per-turn bounded working graph or manage foreground/optimizer graph snapshots.

**Notes:** The clearest buyer is an application team adding memory to an existing assistant/agent without building extraction, dedup, graph, and retrieval infrastructure.

### P-3 - Cognee

**Vendor / repo / docs / pricing:** Product: https://www.cognee.ai/, docs: https://docs.cognee.ai/examples/overview, cloud docs: https://docs.cognee.ai/how-to-guides/cognee-cloud, remember docs: https://docs.cognee.ai/core-concepts/main-operations/remember, pricing: https://www.cognee.ai/pricing, architecture post: https://www.cognee.ai/blog/fundamentals/how-cognee-builds-ai-memory, funding: https://www.cognee.ai/blog/cognee-news/cognee-raises-seven-million-five-hundred-thousand-dollars-seed, repo: https://github.com/topoteretes/cognee

**Positioning:** "Knowledge engine that learns" and "the fastest way to start building reliable AI agent memory" (https://www.cognee.ai/, https://www.cognee.ai/pricing).

**Architecture (briefly):** Knowledge-graph-centered hybrid memory. Docs say Cognee creates a unified memory layer combining knowledge graphs with vector search (https://docs.cognee.ai/examples/overview).

**Capability surface:** `remember()` for permanent/session memory; cognify pipelines; ECL extraction/cognify/load pipeline; self-improvement/background improve pass; ontologies/custom schemas; 14 retrieval modes; graph explorer/UI; 28-38+ data-source claims depending on page; Kuzu/LanceDB/PostgreSQL hosted backends; API endpoints for add/delete/list/cognify/search/get insights.

**Persistence model:** Open-source self-hosting with local/customer graph/vector/document stores; Cognee Cloud vendor-hosted with Kuzu, LanceDB and PostgreSQL; on-prem subscription for SMBs.

**SDK languages:** Python SDK is primary; Cognee Cloud exposes HTTP API. Docs reference experimental Rust SDK `cognee-RS` for edge/on-device memory.

**Pricing tiers:** Basic free: open-source license, tasks/pipelines, custom schema/ontology generation, evaluations, 28+ data sources. Cloud Subscription: $25/month beta, hosted platform, API endpoints, autoscaling/parallel processing, group memories per user/domain, 1 GB ingestion + 10,000 API calls. On-Prem Subscription for SMBs: $3,500/month, platform license, 1-day SLA, on-prem deployment, support, architecture review, roadmap prioritization, knowledge transfer (https://www.cognee.ai/pricing).

**Customer segments:** Vertical AI agents, data-silo unification, local agent memory. Funding post names Bayer, University of Wyoming, Dilbloom and dltHub, says production usage at 70+ companies and over one million pipelines/month (https://www.cognee.ai/blog/cognee-news/cognee-raises-seven-million-five-hundred-thousand-dollars-seed).

**Documented limitations:** FAQ says "cognee feels complicated" and answers that modular design can simplify adoption (https://docs.cognee.ai/faq). That is a vendor-acknowledged adoption/friction issue. Cloud docs say a valid credit/debit card is required on first sign-in for cloud (https://docs.cognee.ai/how-to-guides/cognee-cloud).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `topoteretes/cognee` had 16,853 stars, 1,748 forks, 77 open issues. Funding: $7.5M seed led by Pebblebed, per Cognee.

**Positioning relative to agent-harness:** Cognee is strongly adjacent because it uses graph memory, session/permanent tiers, and self-improvement. It still sells retrieval and reasoning over a knowledge graph behind an agent, not an imposed turn-by-turn working-set renderer with stable pack/unpack identities and optimizer merge semantics.

**Notes:** Cognee has the most explicit "RAG is not enough" pitch among OSS-first memory products.

### P-4 - Zep / Graphiti

**Vendor / repo / docs / pricing:** Zep product: https://www.getzep.com/, pricing: https://www.getzep.com/pricing, docs/FAQ: https://help.getzep.com/faq, Graphiti overview: https://help.getzep.com/graphiti/graphiti/overview, Zep vs Graphiti: https://help.getzep.com/zep-vs-graphiti, open source page: https://www.getzep.com/product/open-source, repos: https://github.com/getzep/graphiti and https://github.com/getzep/zep, paper PDF: https://blog.getzep.com/content/files/2025/01/ZEP__USING_KNOWLEDGE_GRAPHS_TO_POWER_LLM_AGENT_MEMORY_2025011700.pdf

**Positioning:** "Context Engineering & Agent Memory Platform for AI Agents"; Zep assembles context from chat history, business data, and user behavior (https://www.getzep.com/).

**Architecture (briefly):** Temporal knowledge graph. Graphiti builds dynamic, temporally aware graphs with semantic, keyword, full-text and graph algorithm retrieval (https://help.getzep.com/graphiti/graphiti/overview).

**Capability surface:** Message and graph-data ingestion; entity extraction; relationship/fact extraction; temporal validity and fact invalidation; context templates/blocks; relevant context assembly; JSON/text/transcript ingestion; direct graph access; SDKs for Python, TypeScript and Go; Graphiti self-hosting; MCP knowledge graph server; benchmarks claiming 80.32% LoCoMo accuracy at 189ms retrieval on homepage.

**Persistence model:** Zep Cloud fully managed; enterprise managed/BYOK/BYOM/BYOC options; Graphiti is self-hosted only (https://help.getzep.com/zep-vs-graphiti).

**SDK languages:** Python, TypeScript/JavaScript, Go SDKs are vendor-cited (https://app.getzep.com/ and homepage).

**Pricing tiers:** Metered: 2,500 messages free/month, then $1.25 per 1,000; 2.5 MB graph data free/month, then $2.50 per MB; up to 5 projects; in-app chat support. Enterprise: managed, BYOK, BYOM, BYOC; SOC 2 Type II, HIPAA BAA, custom limits/rate limits, Slack support, account manager, API/audit logs, SLAs. Pricing FAQ says ingestion/processing is charged, but storage is not (https://www.getzep.com/pricing).

**Customer segments:** Developers; engineering leaders; voice/video/live support; sales agents; customer support and personalized assistants. Homepage cites Sidekick personalized experience and named testimonials from Ken Collins and Lior Sinclair.

**Documented limitations:** Zep Cloud is paid managed service, while Graphiti is self-hosted only. Zep pricing FAQ says rate limits can be lowered depending on service usage (https://www.getzep.com/pricing). Graphiti issue #963 documents "Duplicate Entities in Neo4j" as an OSS graph-quality limitation (https://github.com/getzep/graphiti/issues/963).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `getzep/graphiti` had 25,456 stars, 2,528 forks, 376 open issues; `getzep/zep` had 4,494 stars, 608 forks, 23 open issues.

**Positioning relative to agent-harness:** Zep is the strongest "context engineering" commercial analogue: it explicitly retrieves and assembles formatted context. The difference is control locus. Zep assembles context when an agent/app calls the API; agent-harness imposes a graph-rendered working set as the primary session substrate and gives the foreground agent pack/unpack navigation rather than free-form memory recall.

**Notes:** Zep is likely the clearest category competitor for the phrase "context engineering platform."

### P-5 - OpenAI ChatGPT Memory

**Vendor / repo / docs / pricing:** Memory FAQ: https://help.openai.com/en/articles/8590148-memory-faq, product announcement: https://openai.com/index/memory-and-new-controls-for-chatgpt/, ChatGPT pricing: https://chatgpt.com/pricing/

**Positioning:** ChatGPT memory personalizes responses by using saved memories and, where enabled, past chat history.

**Architecture (briefly):** Vendor-hosted hybrid personalization memory. It has user-visible saved memories and a separate reference-chat-history mechanism (https://help.openai.com/en/articles/8590148-memory-faq).

**Capability surface:** Save/update/delete memories; toggle memory; reference saved memories; reference chat history; temporary chat; admin controls for business/enterprise; plan-differentiated memory capacity; memory management on web for Plus/Pro per FAQ; no public SDK for this ChatGPT feature.

**Persistence model:** Vendor-hosted within ChatGPT account/workspace. Not BYO storage. Workspace admins can control availability.

**SDK languages:** None for ChatGPT Memory as a product feature. OpenAI API has separate stateless/Responses tools, but ChatGPT Memory is bundled UI/product behavior.

**Pricing tiers:** Bundled in ChatGPT plans. Official pricing page lists Memory as Limited on Free, available on Go, Expanded on Plus/Pro/Business/Enterprise; Memory with past chats is Limited on Free, Yes on Go, Expanded on Plus/Pro, and Coming soon for Business/Enterprise (https://chatgpt.com/pricing/). The page lists Enterprise as custom pricing; exact Plus/Pro dollar figures were not visible in the crawled page content, so I avoid restating non-crawled dollar amounts here.

**Customer segments:** General ChatGPT users, paid consumers, teams/business users, enterprise/Edu workspaces. Product announcement examples are personal preferences, work style and recurring user context.

**Documented limitations:** FAQ says deleted memories may continue to be referenced for a few days, and saved memory storage has a limit. It also says OpenAI may use content including saved memories and past chats for training if "Improve the model for everyone" is on (https://help.openai.com/en/articles/8590148-memory-faq).

**Public traction signals:** No repo or install count. Traction is bundled ChatGPT distribution, not memory-specific public revenue. Public price-plan page signals packaging across all ChatGPT tiers (https://chatgpt.com/pricing/).

**Positioning relative to agent-harness:** OpenAI Memory is consulted personalization memory inside ChatGPT. It is incompatible with agent-harness's need for auditable local graph provenance, stable node identity, cross-CLI rendering, and explicit working-set budget controls.

**Notes:** This is a consumer/workspace feature, not a developer memory product.

### P-6 - Anthropic Claude Memory Tool / Claude App Memory

**Vendor / repo / docs / pricing:** API overview: https://docs.claude.com/en/api/overview, beta Messages API reference: https://platform.claude.com/docs/en/api/beta/messages/create, pricing: https://docs.anthropic.com/en/docs/about-claude/pricing, Claude app memory announcement: https://www.anthropic.com/news/memory, context engineering cookbook: https://platform.claude.com/cookbook/tool-use-context-engineering-context-engineering-tools

**Positioning:** Claude API Memory enables Claude to store/retrieve information across conversations; Claude app memory is "built for work" and remembers projects/preferences (https://docs.claude.com/en/api/overview, https://www.anthropic.com/news/memory).

**Architecture (briefly):** Vendor-hosted server-side tool memory in the Messages API plus app-level workspace memory. It is a tool the model can call, not a database product.

**Capability surface:** Server tool named `memory` with beta type `memory_20250818`; cross-conversation storage/retrieval; optional tool loading controls in API schema; app memory settings; Incognito chats; import/export memory details; Team/Enterprise admin control.

**Persistence model:** Vendor-hosted Anthropic memory. API memory is token-metered through Claude API; app memory is bundled with Claude Team/Enterprise rollout.

**SDK languages:** Any runtime can call Messages API; official SDKs are Python and TypeScript. Memory itself is an API tool.

**Pricing tiers:** API memory tool uses normal token-metered Claude pricing; pricing docs say tools use the same pricing structure as other tool use, with standard token accounting and prompt caching line items (https://docs.anthropic.com/en/docs/about-claude/pricing). Claude app memory rollout was for Team and Enterprise plan users, with Incognito available to all Claude users (https://www.anthropic.com/news/memory).

**Customer segments:** Work teams and enterprises: sales teams, product teams, executives are vendor-cited examples (https://www.anthropic.com/news/memory). API buyers are application developers building persistent Claude-backed apps.

**Documented limitations:** App announcement says memory is optional and admins can disable it. API overview marks Memory as Beta. Third-party platform docs for Bedrock note that Sonnet 4.5 includes memory, but customers must explicitly handle tool results in normal tool-use flow (https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters-anthropic-claude-messages-tool-use.html).

**Public traction signals:** No dedicated memory repo or install count. Signal is first-party platform availability across Anthropic API, Bedrock and Vertex AI Beta per docs.

**Positioning relative to agent-harness:** Claude memory is consulted by Claude through a tool and priced as API usage. It could become one provider-specific memory source, but it cannot be the harness graph because it does not expose stable topology, multi-CLI identity, provenance, or optimizer merge semantics.

**Notes:** Claude app memory and API memory are separate surfaces. Managed agents, Claude Code, and broader agent SDKs are cross-lane.

### P-7 - Cursor Memories

**Vendor / repo / docs / pricing:** Memories docs: https://docs.cursor.com/en/context/memories, context/pricing docs: https://docs.cursor.com/account/rate-limits, product pricing: https://www.cursor.com/en/pricing, model/context docs: https://docs.cursor.com/models, pricing policy: https://cursor.com/terms/pricing/

**Positioning:** Cursor Memories are automatically generated project-scoped rules from chat that maintain context across coding sessions.

**Architecture (briefly):** IDE sidecar memory. A sidecar model observes conversations and extracts memories; agent tool calls can also create memories (https://docs.cursor.com/en/context/memories).

**Capability surface:** Passive background extraction; user approval for background-generated memories; direct agent memory creation by tool calls; project-scoped memories; management via Cursor Settings -> Rules; context-window optimization/pruning; privacy mode; integration with Cursor agent/background agents.

**Persistence model:** Cursor-hosted/project-scoped account/editor state, exposed as rules in Cursor settings. Not BYO storage; local project files are separate.

**SDK languages:** None. This is bundled into Cursor IDE, not an external memory SDK.

**Pricing tiers:** Bundled with Cursor. Product page: Hobby free with Pro trial, limited agent requests/tab completions; Pro $20/month with unlimited agent requests/tab completions, background agents, Bug Bot and max context windows; Ultra $200/month with 20x usage; Teams $40/user/month with privacy mode org-wide, admin dashboard, billing and SSO; Enterprise custom (https://www.cursor.com/en/pricing). Cursor docs also describe usage pools: Pro includes $20 of API agent usage, Pro Plus $70, Ultra $400 plus bonus usage (https://docs.cursor.com/account/rate-limits).

**Customer segments:** Developers and engineering teams using Cursor as an AI IDE; team/enterprise buyers needing privacy mode, SSO and centralized billing.

**Documented limitations:** Background-generated memories require approval before saving. Cursor model docs recommend starting a new chat for each unique task because context windows grow and are pruned (https://docs.cursor.com/models). Pricing policy says beta/add-on fees may be set in settings or add-on service (https://cursor.com/terms/pricing/).

**Public traction signals:** No public memory repo. Cursor traction is product-level, not memory-specific. Pricing backlash/usage-metering is visible in public discussion but outside primary-source scope.

**Positioning relative to agent-harness:** Cursor Memories overlap only for coding-agent continuity. It is project-scoped, IDE-specific, and rule-like. Agent-harness is broader cross-CLI orchestration with explicit graph walking, not an IDE chat memory convenience.

**Notes:** Cursor Background Agents and broader IDE agent product are cross-lane.

### P-8 - MongoDB Atlas Vector Search + Agentic Memory Positioning

**Vendor / repo / docs / pricing:** AI agents docs: https://www.mongodb.com/docs/atlas/atlas-vector-search/ai-agents/, pricing: https://www.mongodb.com/pricing, self-managed search/vector announcement: https://investors.mongodb.com/news-releases/news-release-details/mongodb-extends-search-and-vector-search-capabilities-self, repo signal: https://github.com/mongodb/mongo

**Positioning:** MongoDB positions Atlas as both vector and document database support for agentic RAG and short/long-term agent memory.

**Architecture (briefly):** Retrieval/document store with vector search; no built-in memory semantics. Docs say memory can be implemented by storing interactions in collections and querying/updating them (https://www.mongodb.com/docs/atlas/atlas-vector-search/ai-agents/).

**Capability surface:** Vector search; document collections for interactions; metadata filters; hybrid/full-text search; Atlas UI inspection; semantic search; agentic RAG examples; self-managed preview of search/vector search for on-prem/local.

**Persistence model:** MongoDB Atlas hosted, self-managed Enterprise/Community with preview search/vector features, or customer cloud. Memory lives in customer MongoDB collections.

**SDK languages:** MongoDB drivers across major languages; docs/examples commonly use Python/JavaScript.

**Pricing tiers:** Atlas Free: $0/hour, 512 MB storage, shared RAM/vCPU. Flex: $0.011/hour, up to $30/month, 5 GB storage. Dedicated: from $0.08/hour, starts at $56.94/month, 10 GB-4 TB storage, 2-768 GB RAM. Vector Search dedicated search node pricing example S20: 106 GB storage, 4 GB RAM, 2 vCPUs, $0.12/hour; S80: 3420 GB storage, 128 GB RAM, 64 vCPUs, $3.26/hour (https://www.mongodb.com/pricing).

**Customer segments:** Existing MongoDB application teams building RAG/agents over operational data; regulated/self-managed customers after vector search moved beyond Atlas-only preview. Investor release cites IDC survey: more than 74% of organizations planned integrated vector DB use in agentic AI workflows (https://investors.mongodb.com/news-releases/news-release-details/mongodb-extends-search-and-vector-search-capabilities-self).

**Documented limitations:** MongoDB docs present memory as something developers implement, not a product feature. The AI agents page says the agent "can then query or update" the collection, placing policy burden on the app (https://www.mongodb.com/docs/atlas/atlas-vector-search/ai-agents/).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `mongodb/mongo` had 28,271 stars and 5,770 forks. MongoDB Inc. also reports Atlas revenue, but company financials are cross-lane.

**Positioning relative to agent-harness:** MongoDB could back a harness evidence/vector store, but it does not supply graph summary contracts, pack/unpack identities, or imposed context rendering.

**Notes:** This is database-infrastructure positioning, not a commercial memory layer.

### P-9 - Pinecone "For Agentic Memory"

**Vendor / repo / docs / pricing:** Pricing: https://www.pinecone.io/pricing/, assistant limits: https://docs.pinecone.io/guides/assistant/pricing-and-limits, cost docs: https://docs.pinecone.io/guides/manage-cost/understanding-cost, RU explainer: https://www.pinecone.io/learn/read-units/, agentic optimization post: https://www.pinecone.io/blog/optimizing-pinecone/, memory blog: https://www.pinecone.io/blog/memory-for-open-source-llms/, repo signal: https://github.com/pinecone-io/pinecone-python-client

**Positioning:** Pinecone sells managed vector infrastructure for knowledgeable/agentic AI; blogs position vector DBs as long-term conversational memory.

**Architecture (briefly):** Managed vector retrieval store with optional Assistant product; not a semantic memory lifecycle layer.

**Capability surface:** Serverless indexes; read/write units; metadata filtering; backups/restore; hosted embeddings/reranking via Pinecone Inference; Pinecone Assistant for chat/agent apps; BYOC; dedicated read nodes; MCP server release noted in docs.

**Persistence model:** Vendor-hosted Pinecone serverless/dedicated; BYOC in customer AWS/GCP/Azure; data persists as vectors/metadata/files depending on product.

**SDK languages:** Python client, JavaScript/TypeScript client, REST API; other clients exist.

**Pricing tiers:** Starter free for trying/small apps with included usage. Standard: $50/month minimum usage, 3-week trial with $300 credits, pay-as-you-go for database on-demand, inference and assistant usage. Enterprise: custom with advanced controls. Pricing page says on-demand database usage is separate from inference/assistant/import and usage-based; docs/RU page define read units for query/fetch/list and write units/storage as separate cost drivers (https://www.pinecone.io/pricing/, https://www.pinecone.io/learn/read-units/).

**Customer segments:** RAG/search/agent teams, legal/litigation and enterprise AI customers. Blog says Pinecone observed increased agentic workloads across its customer base (https://www.pinecone.io/blog/optimizing-pinecone/).

**Documented limitations:** RU costs vary with namespace size, dimensionality and metadata; Pinecone's own RU explainer says query RU cost is proportional to namespace size but sublinear (https://www.pinecone.io/learn/read-units/). That makes memory cost workload-dependent and not per-fact predictable.

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `pinecone-io/pinecone-python-client` had 434 stars, 123 forks, 46 open issues. Company/product traction is not memory-specific.

**Positioning relative to agent-harness:** Pinecone is a potential low-level vector backend. It is consulted retrieval infrastructure and lacks graph topology, summary contracts, context-render policy, foreground pack/unpack or optimizer coordination.

**Notes:** Pinecone Assistant begins to overlap with app-level agent products; keep that part cross-lane except where pricing affects memory.

### P-10 - Weaviate "Agent Memory"

**Vendor / repo / docs / pricing:** Agents docs: https://docs.weaviate.io/agents, Query Agent product: https://weaviate.io/product/query-agent, context-engineering blog: https://weaviate.io/blog/context-engineering, pricing: https://weaviate.io/pricing.html, repo: https://github.com/weaviate/weaviate

**Positioning:** Weaviate frames context engineering as memory and retrieval for AI agents; Weaviate Agents are prebuilt services for Weaviate Cloud users.

**Architecture (briefly):** Vector/hybrid retrieval database plus cloud-only query/transformation/personalization agents.

**Capability surface:** Vector DB; hybrid search; Query Agent for natural language questions over Weaviate data; Transformation Agent technical preview; Personalization Agent technical preview; Python/TypeScript clients; cloud console; embedding service; data import.

**Persistence model:** Weaviate Cloud hosted, customer self-hosted OSS/enterprise for database; Weaviate Agents are Cloud-only.

**SDK languages:** Python, TypeScript/JavaScript, Go, Java clients for database; Query Agent accessed through Python and TypeScript clients.

**Pricing tiers:** Pricing page uses Weaviate Cloud plans and AI Units (AIU) language; exact rates vary by cloud/region and the page points to full price list/contact. It includes serverless/cloud and enterprise options; Query Agent/Data Import/Embedding Service are incorporated into the commercial cloud experience (https://weaviate.io/pricing.html).

**Customer segments:** Developers building RAG/AI apps over Weaviate data; cloud users wanting prebuilt database-native agents.

**Documented limitations:** Docs explicitly state "Weaviate Agents is not an agent framework" and the agents are "Weaviate Cloud only" (https://docs.weaviate.io/agents). Transformation and Personalization Agents are marked Technical Preview.

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `weaviate/weaviate` had 16,089 stars, 1,268 forks, 564 open issues.

**Positioning relative to agent-harness:** Weaviate can store/retrieve memories and maybe personalize outputs, but it is still a consulted data service. It does not own the orchestrator's imposed working set or graph navigation lifecycle.

**Notes:** Weaviate Agents themselves are database-management/query agents, partly cross-lane.

### P-11 - Supabase pgvector "Agent Memory"

**Vendor / repo / docs / pricing:** pgvector docs: https://supabase.com/docs/guides/database/extensions/pgvector, pricing/storage docs: https://supabase.com/docs/guides/storage/management/pricing, product pricing: https://supabase.com/pricing, repo: https://github.com/supabase/supabase, pgvector repo signal: https://github.com/pgvector/pgvector

**Positioning:** Supabase positions Postgres + pgvector as an open backend for embeddings/RAG; "agent memory" positioning is mostly ecosystem/tutorial-level rather than a first-party memory product.

**Architecture (briefly):** Customer-controlled Postgres relational store plus vector extension. It is storage/search infrastructure, not a memory manager.

**Capability surface:** Store embeddings in Postgres; vector similarity search; metadata filters via SQL; HNSW/IVFFlat indexes; normal relational joins; auth/storage/realtime/edge functions around memory data; self-hostable Supabase stack.

**Persistence model:** Supabase Cloud managed Postgres or self-hosted Postgres/Supabase. Memory lives in customer tables.

**SDK languages:** Supabase JS, Python, Dart, Swift, Kotlin, C# plus any Postgres client. pgvector works through SQL.

**Pricing tiers:** Supabase commercial pricing is project/backend-bundled. Public secondary snippets and official docs show Free/Pro/Team/Enterprise structure; storage docs show Free includes 1 GB storage quota, Pro/Team include 100 GB storage and overage at $0.021/GB-month (https://supabase.com/docs/guides/storage/management/pricing). Product pricing page should be used for current base plan dollars at purchase time.

**Customer segments:** Indie developers/startups already using Postgres/Supabase; teams preferring SQL ownership and self-hosting.

**Documented limitations:** Supabase pgvector docs warn filtered ANN queries may return fewer rows than requested and recommend iterative search (https://supabase.com/docs/guides/database/extensions/pgvector). This is a concrete retrieval-quality edge case for agent memory.

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `supabase/supabase` had 101,501 stars; `pgvector/pgvector` had 21,020 stars.

**Positioning relative to agent-harness:** Supabase/pgvector could serve as local/cloud storage for evidence, vectors, and relational graph metadata, but all memory extraction, ranking, summary, provenance and working-set imposition would be agent-harness code.

**Notes:** Supabase is a backend platform; broader BaaS pricing is cross-lane.

### P-12 - Qdrant

**Vendor / repo / docs / pricing:** Product: https://qdrant.tech/, AI agents page: https://qdrant.tech/ai-agents/, agentic builder post: https://qdrant.tech/articles/agentic-builders-guide/, pricing: https://qdrant.tech/pricing/, docs: https://qdrant.tech/documentation/, repo: https://github.com/qdrant/qdrant

**Positioning:** Qdrant positions itself as AI-native vector search for agents, with real-time context and short/long-term memory use cases.

**Architecture (briefly):** Vector search engine/database with filters, hybrid search and quantization; not graph memory.

**Capability surface:** REST/gRPC APIs; real-time upserts; metadata filters; hybrid fusion/reranking; multivector retrieval; quantization; local/cloud/hybrid/private deployment; Cloud Inference; web UI.

**Persistence model:** Self-hosted OSS, Qdrant Cloud managed, Hybrid Cloud in customer infrastructure, Private Cloud/air-gapped.

**SDK languages:** Python, JavaScript/TypeScript, Rust, Go, Java/.NET clients are common; docs cite official clients Python and JavaScript.

**Pricing tiers:** Free Tier: single-node cluster, 0.5 vCPU, 1 GB RAM, 4 GB disk, free cloud inference with selected models. Standard: usage-based, dedicated resources, scaling, HA, backup/DR, 99.5% SLA. Premium: minimum spend, SSO/private links, 99.9% SLA and support. Hybrid/Private Cloud: contact sales. Billing is hourly for compute, memory, storage, backups and paid inference tokens (https://qdrant.tech/pricing/).

**Customer segments:** Developers building RAG/recommendation/search/agent apps; enterprise and regulated workloads needing hybrid/private cloud. Homepage claims it powers a multi-agent platform with 2M+ AI-driven conversations (https://qdrant.tech/).

**Documented limitations:** Pricing FAQ says the free tier is for testing/prototypes and limited to 1 GB RAM and 4 GB disk without HA (https://qdrant.tech/pricing/). Qdrant Skills post notes vector search choices involve "memory vs latency, recall vs throughput, precision vs cost" (https://qdrant.tech/blog/qdrant-skills-release/).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `qdrant/qdrant` had 30,789 stars, 2,214 forks, 538 open issues.

**Positioning relative to agent-harness:** Qdrant is a strong candidate storage backend for vector recall but not a harness competitor unless sold as a complete memory lifecycle. It is consulted retrieval.

**Notes:** Qdrant's deployment flexibility is attractive for local/control-sensitive users.

### P-13 - Redis Stack / Redis Agent Memory Server

**Vendor / repo / docs / pricing:** Agent memory blog: https://redis.io/blog/ai-agent-memory-stateful-systems/, build guide: https://redis.io/blog/build-smarter-ai-agents-manage-short-term-and-long-term-memory-with-redis/, Redis for AI docs: https://redis.io/docs/latest/develop/ai/, RedisVL docs: https://docs.redisvl.com/en/0.4.1/, Redis Agent Memory Server: https://redis.github.io/agent-memory-server/, pricing: https://redis.io/pricing, repos: https://github.com/redis/redis and https://github.com/redis/agent-memory-server

**Positioning:** Redis frames itself as fast memory infrastructure for stateful AI agents: session state, semantic cache, vector search, short-term and long-term memory.

**Architecture (briefly):** In-memory/Flash key-value + vector/full-text/hybrid search with optional agent memory server.

**Capability surface:** RedisVL index management; vector search; semantic cache; LLM session memory; semantic routing; Redis Agent Memory Server; short-term checkpoints; long-term vector memory; pub/sub/session state; Redis Cloud/Stack local.

**Persistence model:** Redis Cloud hosted, self-hosted Redis Stack, customer-managed/on-prem. Memory lives in Redis hashes/JSON/vector indexes.

**SDK languages:** Redis clients across languages; RedisVL is Python; node-redis, redis-py, Jedis, go-redis, NRedisStack are cited in docs.

**Pricing tiers:** Redis Cloud Free: $0, 30 MB single DB, shared cloud, best-effort SLA/community support. Essentials: from $0.007/hour, total $5/month, 250 MB-100 GB RAM & SSD, up to 99.99% uptime. Pro: from $0.014/hour with $200/month minimum, dedicated cloud, unlimited RAM/multiple DBs, active-active, auto-tiering, private connectivity, up to 99.999% uptime (https://redis.io/pricing).

**Customer segments:** Teams already using Redis for app state, high-throughput low-latency agents, semantic cache/cost reduction users. Redis blog cites Relevance AI reducing vector search latency from 2s to 10ms after migration (https://redis.io/blog/build-smarter-ai-agents-manage-short-term-and-long-term-memory-with-redis/).

**Documented limitations:** Redis itself says memory architecture requires deciding what persists, decay strategy, and retrieval design; "most teams underestimate this" (https://redis.io/blog/build-smarter-ai-agents-manage-short-term-and-long-term-memory-with-redis/). Agent Memory Server repo issues include inconsistent long-term-memory tag serialization (https://github.com/redis/agent-memory-server/pull/203).

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `redis/redis` had 74,039 stars; `redis/agent-memory-server` had 238 stars and 43 open issues.

**Positioning relative to agent-harness:** Redis could be a fast state/cache/vector substrate. It does not deliver imposed graph-derived working context or long-lived orchestration semantics out of the box.

**Notes:** Redis Agent Memory Server is closer to the memory lane than generic Redis Cloud, but still infrastructure.

### P-14a - HelixDB

**Vendor / repo / docs / pricing:** Product: https://www.helix-db.com/, docs: https://docs.helix-db.com/documentation/getting-started/intro, repo: https://github.com/HelixDB/helix-db, YC page: https://www.ycombinator.com/companies/helixdb

**Positioning:** Native graph-vector database for AI apps/agents/RAG, built in Rust.

**Architecture (briefly):** Unified graph-vector database with compiled graph/vector queries and built-in embeddings.

**Capability surface:** Property graph; vector search; keyword search; graph traversals; HelixQL; built-in MCP support so agents can discover data and walk graph; built-in embeddings; private-by-default compiled queries; local/cloud.

**Persistence model:** Local/self-hosted and Helix Cloud. Pricing page was referenced by the homepage as `pricing.md`, but I did not find a public pricing table in indexed docs.

**SDK languages:** Python SDK documented; HelixQL/REST-like database surface.

**Pricing tiers:** Evidence gap: homepage says "AI agents can view our pricing on the pricing.md page," but indexed source did not expose a plan table (https://www.helix-db.com/).

**Customer segments:** Builders of AI apps, agents and RAG needing one graph-vector backend. YC page says HelixDB brings structure to unstructured data for RAG and AI applications.

**Documented limitations:** No mature public limitation page found. Repo open issues and AGPL-3.0 licensing are practical adoption constraints.

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `HelixDB/helix-db` had 4,091 stars, 215 forks, 41 open issues, AGPL-3.0 license. Homepage says backed by Y Combinator, Nvidia and Vercel.

**Positioning relative to agent-harness:** HelixDB is a plausible graph-vector substrate for harness memory, especially with MCP and graph walking. It is not itself an orchestrator working-set policy or optimizer.

**Notes:** Early-stage product; pricing evidence is weak.

### P-14b - SurrealDB

**Vendor / repo / docs / pricing:** Product: https://surrealdb.com/, AI solutions: https://surrealdb.com/solutions/ai, features: https://surrealdb.com/features, pricing: https://surrealdb.com/pricing, whitepaper: https://surrealdb.com/static/surrealdb-context-layer-whitepaper.pdf, repo: https://github.com/surrealdb/surrealdb

**Positioning:** "The context layer for AI agents" and "multi-model database for AI agents" (https://surrealdb.com/).

**Architecture (briefly):** Multi-model database: document, graph, vector, temporal/time-series in one ACID system.

**Capability surface:** SurrealQL over documents/graphs/vectors/time series; graph traversal + vector search in one query; unified agent memory; file storage; functions/extensibility; Sidekick AI copilot; cloud/self-hosted.

**Persistence model:** SurrealDB Cloud managed or self-hosted.

**SDK languages:** SurrealDB has SDKs for JavaScript/TypeScript, Rust, Python, Go, Java and others; query layer is SurrealQL.

**Pricing tiers:** Cloud Free: 1 GB storage, 0.25 vCPU, 1 GB memory, limited compute node size, social auth, team collaboration, RBAC/ABAC, Sidekick AI copilot. Start: from $0.021/hour, single node, up to 512 GB storage, up to 16 vCPU, up to 64 GB memory, daily backups. Dedicated: contact sales, multiple nodes, up to 1 PB cluster storage (https://surrealdb.com/pricing).

**Customer segments:** AI agents, context-aware apps, multi-model RAG builders who want fewer databases.

**Documented limitations:** AI solutions page warns that agent swarms may generate "thundering herd traffic" that can sink a database (https://surrealdb.com/solutions/ai). That is a vendor-cited operational risk for agent workloads.

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `surrealdb/surrealdb` had 31,958 stars, 1,252 forks, 659 open issues.

**Positioning relative to agent-harness:** SurrealDB could host the harness graph/evidence/vector/temporal data in one store. It does not supply harness-level summary contracts or imposed render policy.

**Notes:** Strong context-layer language, but still database lane.

### P-14c - Neo4j AuraDB / Aura Agent / Agent Memory

**Vendor / repo / docs / pricing:** Pricing: https://neo4j.com/pricing/, Aura Agent docs: https://neo4j.com/docs/aura/aura-agent/, Aura Agent product: https://neo4j.com/product/aura-agent/, launch blog: https://neo4j.com/blog/agentic-ai/neo4j-launches-aura-agent/, GraphAcademy memory course: https://graphacademy.neo4j.com/courses/genai-context-graphs/1-introduction-to-agent-memory/3-why-graphs/, agent-memory repo: https://github.com/neo4j-labs/agent-memory

**Positioning:** Graph-grounded agents and graph-native memory for AI agents/context graphs.

**Architecture (briefly):** Native graph database with vector embeddings on message/entity nodes in `neo4j-agent-memory`; Aura Agent generates read-only graph-grounded agents over AuraDB.

**Capability surface:** AuraDB graph database; Aura Agent auto-ontology/schema-based construction; hosted secure MCP server; read-only graph queries; vector embeddings; entity resolution/dedup in agent-memory repo; similar task retrieval; FastAPI/Next.js starter.

**Persistence model:** AuraDB hosted or self-managed Neo4j; agent-memory stores in a running Neo4j instance. Aura Agent requires AuraDB.

**SDK languages:** Neo4j drivers for Python, JavaScript, Java, .NET, Go; agent-memory repo scaffolds Python/FastAPI and Next.js.

**Pricing tiers:** AuraDB Free: $0. AuraDB Professional: $65/GB/month, minimum 1 GB cluster; examples: 1 GB at $0.09/hour or $65.70/month; up to 64 GB at $5.76/hour or $4,204.80/month. Business Critical: $146/GB/month, minimum 2 GB cluster. Enterprise/VDC/self-managed options via sales. Aura Agent docs say internal agents are free; external agents incur charges per Neo4j pricing (https://neo4j.com/pricing/, https://neo4j.com/docs/aura/aura-agent/).

**Customer segments:** Enterprise graph/RAG users; teams with structured domain data and graph-grounded assistants; users of Claude, Cursor, ChatGPT via MCP per Aura Agent datasheet/product copy.

**Documented limitations:** Aura Agent docs say it currently supports read-only queries against the database (https://neo4j.com/docs/aura/aura-agent/). That limits use as writable agent memory unless paired with separate write paths.

**Public traction signals:** GitHub API on 2026-04-28T00:22Z: `neo4j-labs/agent-memory` had 157 stars, 46 forks, 46 open issues. Neo4j itself is a mature commercial graph company; memory product traction is newer.

**Positioning relative to agent-harness:** Neo4j is a credible graph substrate and `agent-memory` is directly relevant, but Aura Agent is query-grounding over a graph. Agent-harness still needs local orchestration state, summary contract, pack/unpack UX and optimizer transaction semantics.

**Notes:** Stronger for relational/graph facts than free-form episodic memory unless the app defines ingestion and lifecycle policy.

## Cross-Product Patterns

**Pricing patterns:** Dedicated memory platforms price on memory-domain units, not only storage. Mem0 charges by memory count and retrieval calls; Zep charges messages and graph-data MB processed; Cognee Cloud bundles ingestion GB and API calls; Letta uses credits plus storage quota. Database vendors price storage/compute/query/inference primitives: Qdrant bills compute, memory, storage, backups and inference tokens; Pinecone bills read/write units, storage, inference and assistant usage; MongoDB bills cluster/search-node resources; Redis bills RAM/SSD tier; Neo4j bills GB memory/month; SurrealDB bills compute/storage nodes. Free tiers are common, but their semantics differ: free memories/retrieval calls for memory vendors, free small databases or small clusters for DB vendors.

**Architectural-shape distribution:** Commercial memory specialists are mostly hybrid: vector + graph + reranking/context assembly (Mem0, Cognee, Zep) or hierarchical agent memory (Letta). Foundation-model vendors expose opaque hosted personalization/tool memory (OpenAI, Anthropic). IDE memory is rule/project scoped (Cursor). Infrastructure vendors mostly sell vector stores, graph-vector stores, or multi-model databases; they call these "agent memory" because agents can retrieve from them, not because the product manages memory lifecycle end to end.

**Customer-segment patterns:** Buyers fall into four groups: application developers adding personalization to assistants; GTM/support/health/education/ecommerce teams needing remembered user context; enterprise graph/RAG teams grounding agents in business data; developer-tool users wanting codebase/project continuity. Dedicated memory vendors cite vertical agents and personalization. Database vendors target existing data-platform buyers and regulated workloads needing data residency, self-hosting or BYOC.

**Imposed working context:** I found no commercial vendor clearly advertising agent-harness's pattern: an optimizer-maintained graph that imposes a bounded working set every orchestrator turn, with agent-owned pack/unpack navigation and snapshot-walk-then-merge. Zep comes closest by saying it retrieves and assembles context blocks, and Letta comes closest on visible agent state/memory hierarchy. But the commercial space remains overwhelmingly "consulted memory": the agent/app asks the memory layer or vector/graph store for relevant context, then injects it.

## Cross-Lane Signals

**Hosted cloud agent products:** Letta Cloud, Pinecone Assistant, Weaviate Agents, Neo4j Aura Agent, Claude app/managed surfaces and Cursor Background Agents overlap with cloud agent products. I included only their memory/context claims and pricing.

**Local desktop assistants:** Cursor Memories and Letta Code touch local/desktop coding assistants, but the broader IDE/desktop assistant market is cross-lane.

**Agent SDKs:** OpenAI Agents SDK, Claude Agent SDK, LangGraph, Google ADK and CrewAI appear as integration targets. They are cross-lane unless memory is the commercial product.

**KM-with-AI products:** Notion AI, Tana AI, Obsidian/Logseq memory plugins and generic knowledge-management assistants are cross-lane. They may compete for user attention but do not sell developer-facing "memory for agents" primitives in the same way.

## Open Evidence Gaps

HelixDB public pricing was not available in indexed primary pages beyond a homepage reference to `pricing.md`.

OpenAI's ChatGPT pricing page content exposed plan-relative memory availability but not all dollar amounts in the crawler output; use live checkout/pricing UI for exact consumer prices.

Weaviate's pricing page uses AI Units and region/provider-dependent list pricing; exact current rates require its downloadable price list or sales contact.

For several infrastructure vendors, "agent memory" is marketing/use-case language rather than a separately metered product. Public traction therefore reflects database/product adoption, not memory-specific adoption.

Most vendors document retrieval/storage costs and feature limits, but few publish rigorous failure modes: memory poisoning, stale fact invalidation accuracy, duplicate entity rates, deletion semantics, and cross-session recall precision remain weakly evidenced outside GitHub issues and benchmark papers.

## Citations

- Letta memory docs: https://docs.letta.com/guides/agents/memory
- Letta ADE docs: https://docs.letta.com/memory/
- Letta pricing: https://docs.letta.com/guides/api/plans
- Letta Code memory: https://docs.letta.com/letta-code/memory/
- Letta/MemGPT blog: https://www.letta.com/blog/memgpt-and-letta
- Letta repo: https://github.com/letta-ai/letta
- Letta funding: https://www.prnewswire.com/news-releases/berkeley-ai-research-lab-spinout-letta-raises-10m-seed-financing-led-by-felicis-to-build-ai-with-memory-302257004.html
- Mem0 product: https://mem0.ai/
- Mem0 platform docs: https://docs.mem0.ai/overview
- Mem0 open-source docs: https://docs.mem0.ai/open-source
- Mem0 memory types: https://docs.mem0.ai/core-concepts/memory-types
- Mem0 API docs: https://docs.mem0.ai/api-reference
- Mem0 pricing: https://mem0.ai/pricing
- Mem0 FAQ: https://docs.mem0.ai/platform/faqs
- Mem0 evaluation docs: https://docs.mem0.ai/core-concepts/memory-evaluation
- Mem0 repo: https://github.com/mem0ai/mem0
- Mem0 funding: https://mem0.ai/series-a
- Mem0 issue #4573: https://github.com/mem0ai/mem0/issues/4573
- Cognee product: https://www.cognee.ai/
- Cognee overview docs: https://docs.cognee.ai/examples/overview
- Cognee cloud docs: https://docs.cognee.ai/how-to-guides/cognee-cloud
- Cognee remember docs: https://docs.cognee.ai/core-concepts/main-operations/remember
- Cognee FAQ: https://docs.cognee.ai/faq
- Cognee pricing: https://www.cognee.ai/pricing
- Cognee architecture post: https://www.cognee.ai/blog/fundamentals/how-cognee-builds-ai-memory
- Cognee funding: https://www.cognee.ai/blog/cognee-news/cognee-raises-seven-million-five-hundred-thousand-dollars-seed
- Cognee repo: https://github.com/topoteretes/cognee
- Zep product: https://www.getzep.com/
- Zep pricing: https://www.getzep.com/pricing
- Zep FAQ: https://help.getzep.com/faq
- Graphiti overview: https://help.getzep.com/graphiti/graphiti/overview
- Zep vs Graphiti: https://help.getzep.com/zep-vs-graphiti
- Zep open source: https://www.getzep.com/product/open-source
- Graphiti repo: https://github.com/getzep/graphiti
- Zep repo: https://github.com/getzep/zep
- Graphiti issue #963: https://github.com/getzep/graphiti/issues/963
- Zep paper PDF: https://blog.getzep.com/content/files/2025/01/ZEP__USING_KNOWLEDGE_GRAPHS_TO_POWER_LLM_AGENT_MEMORY_2025011700.pdf
- OpenAI Memory FAQ: https://help.openai.com/en/articles/8590148-memory-faq
- OpenAI memory announcement: https://openai.com/index/memory-and-new-controls-for-chatgpt/
- ChatGPT pricing: https://chatgpt.com/pricing/
- Anthropic API overview: https://docs.claude.com/en/api/overview
- Anthropic beta Messages API: https://platform.claude.com/docs/en/api/beta/messages/create
- Anthropic pricing: https://docs.anthropic.com/en/docs/about-claude/pricing
- Anthropic app memory announcement: https://www.anthropic.com/news/memory
- Anthropic context engineering cookbook: https://platform.claude.com/cookbook/tool-use-context-engineering-context-engineering-tools
- AWS Bedrock Claude tool-use docs: https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters-anthropic-claude-messages-tool-use.html
- Cursor Memories: https://docs.cursor.com/en/context/memories
- Cursor account/model pricing docs: https://docs.cursor.com/account/rate-limits
- Cursor product pricing: https://www.cursor.com/en/pricing
- Cursor models/context docs: https://docs.cursor.com/models
- Cursor pricing policy: https://cursor.com/terms/pricing/
- MongoDB AI agents docs: https://www.mongodb.com/docs/atlas/atlas-vector-search/ai-agents/
- MongoDB pricing: https://www.mongodb.com/pricing
- MongoDB vector/search self-managed announcement: https://investors.mongodb.com/news-releases/news-release-details/mongodb-extends-search-and-vector-search-capabilities-self
- MongoDB repo: https://github.com/mongodb/mongo
- Pinecone pricing: https://www.pinecone.io/pricing/
- Pinecone Assistant limits: https://docs.pinecone.io/guides/assistant/pricing-and-limits
- Pinecone cost docs: https://docs.pinecone.io/guides/manage-cost/understanding-cost
- Pinecone read units: https://www.pinecone.io/learn/read-units/
- Pinecone agentic optimization: https://www.pinecone.io/blog/optimizing-pinecone/
- Pinecone LLM memory blog: https://www.pinecone.io/blog/memory-for-open-source-llms/
- Pinecone Python client repo: https://github.com/pinecone-io/pinecone-python-client
- Weaviate Agents docs: https://docs.weaviate.io/agents
- Weaviate Query Agent: https://weaviate.io/product/query-agent
- Weaviate context engineering blog: https://weaviate.io/blog/context-engineering
- Weaviate pricing: https://weaviate.io/pricing.html
- Weaviate repo: https://github.com/weaviate/weaviate
- Supabase pgvector docs: https://supabase.com/docs/guides/database/extensions/pgvector
- Supabase storage pricing docs: https://supabase.com/docs/guides/storage/management/pricing
- Supabase pricing: https://supabase.com/pricing
- Supabase repo: https://github.com/supabase/supabase
- pgvector repo: https://github.com/pgvector/pgvector
- Qdrant product: https://qdrant.tech/
- Qdrant AI agents: https://qdrant.tech/ai-agents/
- Qdrant agentic builder guide: https://qdrant.tech/articles/agentic-builders-guide/
- Qdrant pricing: https://qdrant.tech/pricing/
- Qdrant docs: https://qdrant.tech/documentation/
- Qdrant Skills post: https://qdrant.tech/blog/qdrant-skills-release/
- Qdrant repo: https://github.com/qdrant/qdrant
- Redis agent memory blog: https://redis.io/blog/ai-agent-memory-stateful-systems/
- Redis build guide: https://redis.io/blog/build-smarter-ai-agents-manage-short-term-and-long-term-memory-with-redis/
- Redis AI docs: https://redis.io/docs/latest/develop/ai/
- RedisVL docs: https://docs.redisvl.com/en/0.4.1/
- Redis Agent Memory Server: https://redis.github.io/agent-memory-server/
- Redis pricing: https://redis.io/pricing
- Redis repo: https://github.com/redis/redis
- Redis Agent Memory Server repo: https://github.com/redis/agent-memory-server
- Redis Agent Memory Server issue/PR #203: https://github.com/redis/agent-memory-server/pull/203
- HelixDB product: https://www.helix-db.com/
- HelixDB docs: https://docs.helix-db.com/documentation/getting-started/intro
- HelixDB repo: https://github.com/HelixDB/helix-db
- HelixDB YC page: https://www.ycombinator.com/companies/helixdb
- SurrealDB product: https://surrealdb.com/
- SurrealDB AI solutions: https://surrealdb.com/solutions/ai
- SurrealDB features: https://surrealdb.com/features
- SurrealDB pricing: https://surrealdb.com/pricing
- SurrealDB context-layer whitepaper: https://surrealdb.com/static/surrealdb-context-layer-whitepaper.pdf
- SurrealDB repo: https://github.com/surrealdb/surrealdb
- Neo4j pricing: https://neo4j.com/pricing/
- Neo4j Aura Agent docs: https://neo4j.com/docs/aura/aura-agent/
- Neo4j Aura Agent product: https://neo4j.com/product/aura-agent/
- Neo4j Aura Agent launch: https://neo4j.com/blog/agentic-ai/neo4j-launches-aura-agent/
- Neo4j GraphAcademy memory course: https://graphacademy.neo4j.com/courses/genai-context-graphs/1-introduction-to-agent-memory/3-why-graphs/
- Neo4j agent-memory repo: https://github.com/neo4j-labs/agent-memory
