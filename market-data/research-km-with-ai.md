# Stage 0b Market Research: AI-augmented Knowledge Management / Outliner Products

Research date: 2026-04-28. Scope is commercial / market evidence for hierarchical, graph, outliner, object, or note-workspace products that add AI features. This stays in the KM lane. Cloud agents, desktop assistants, agent SDKs, and dedicated memory APIs are flagged as cross-lane rather than analyzed as direct competitors.

## Per-product Findings

### P-1 - Tana

**Vendor / docs / pricing:** https://outliner.tana.inc/docs/tana-ai, https://outliner.tana.inc/pricing, https://outliner.tana.inc/docs/ai-command-nodes, https://outliner.tana.inc/docs/ai-for-builders, https://outliner.tana.inc/articles/whats-new-in-tana-2025-product-updates

**Hierarchy primitive:** outliner blocks / nodes, with supertags, fields, views, search nodes, daily notes, and a knowledge graph. Tana is one of the closest commercial analogues to graph-organized working context because every item is a node and supertags act like object schemas.

**AI features:** Tana documents AI as deeply integrated into notes and nodes: AI chat on notes, chat with selected note context, @-mention context injection, AI commands, command events triggered when children are added/removed, botless meeting notetaker, meeting agent bot, live transcription, mobile voice memos, Google Calendar classification, automatic field filling, AI image generation, web search across models, model selection across OpenAI / Anthropic / Google, and prompt caching for AI chat. Tana says paid plans can select multiple providers and reasoning levels; free users can try AI with credits. Tana's 2026 release-note feed also lists Local API/MCP for programmatic access to local Tana data, including search/read nodes, create content with Tana Paste, and update fields programmatically. That is adjacent to agent-harness, but still productized as a user's note workspace, not an orchestrator harness.

**Pricing tiers:** The text-rendered pricing page exposes plan names and AI credit allotments but not the main list prices in the static HTML. It does show education/academic/NGO discounts at 50%: Tana Plus $5/month or $48/year, Tana Pro $9/month or $84/year. That implies regular Plus is $10/month or $96/year and Pro is $18/month or $168/year, but this is an inference from the official discount language, not directly printed in the extracted price table. Tana Free includes 500 monthly AI credits and cannot buy top-ups. Tana Plus includes 2,000 AI credits. Tana Pro includes 5,000 AI credits. Tana says 5,000 credits equate to one of: 22 x 30-minute meetings, 125 images, 570 transcript-generated blog posts, or 25 hours of live voice transcription. BYO-key is limited: Tana says full Tana AI cannot be replaced by the user's AI-provider subscription, but paid users can use their own OpenAI key for two command nodes, `Ask AI` and `Ask AI (non-streaming)`.

**Customer segments:** Tana's pricing page says it is used by "forward thinking professionals in leading tech teams" and displays logos for Airbnb, Amazon, Figma, Google, GitHub, Meta, Nike, Tesla, Twitch, Apple, Stripe, Unity, and DoorDash. It also lists "Tana for individuals" segments: personal use, PKM, solo entrepreneurs, students; and "Tana for teams": content creation, UX/design, engineering, management, product, startups.

**Active user / install signals:** No DAU, MAU, paying-user count, or revenue number found in official static pages. Vendor-published traction is logo-based, not numeric.

**Documented user complaints:** Public complaints center on complexity and AI execution quality. A Tana subreddit thread says: "AI search ability in Tana is really poor" (https://www.reddit.com/r/TanaInc/comments/1n1myvk). Another says AI settings on supertags and fields are "way too unclear" and worries Tana is "way too fiddly" for normal users (https://www.reddit.com/r/TanaInc/comments/1ln0uid). A recent Tana-versus-Notion complaint says the AI "can't even edit a node in place" and misses a basic table feature (https://www.reddit.com/r/TanaInc/comments/1suv5ru/tables_and_tanas_ai_vs_notion/).

**Positioning relative to agent-harness:** Tana overlaps strongly with the "knowledge graph plus AI action" surface, especially because nodes, supertags, fields, command nodes, events, and Local API/MCP can let AI manipulate a structured graph. It does not appear to impose a bounded working set on a long-lived orchestrator or manage cross-CLI sub-agents. Tana users who already build supertag-driven workflows and AI command nodes are plausible candidates for agent-harness if they want agents to execute across tools rather than enrich notes.

**Notes:** Tana is the strongest commercial signal that users pay for graph/outliner structure plus AI automation. Its pricing also shows that AI credits are sold as a plan-bundled compute quota, not purely BYO-key.

### P-2 - Notion + Notion AI

**Vendor / docs / pricing:** https://www.notion.com/pricing, https://www.notion.com/help/2025-pricing-changes, https://www.notion.com/help/notion-ai-security-practices, https://www.notion.com/help/guides/notion-ai-for-databases

**Hierarchy primitive:** pages and blocks, with databases, relations, rollups, synced databases, automations, teamspaces, and now AI agents / AI meeting notes. Notion is pages-dominant, but databases add typed properties and relation graphs.

**AI features:** Notion pricing describes Notion AI Core as chat, document generation/editing, database autofill, translation, and workspace Q&A. Business adds Notion Agent, AI Meeting Notes, Enterprise Search beta, Research Mode beta, and Notion Mail AI. Custom Agents are a separate beta product handling repetitive tasks. Notion's pricing page says Custom Agents are "free to try, then $10 per 1,000 monthly Notion credits." Security docs say Notion AI data is shared with AI subprocessors only to provide features and is not used to train models; Enterprise gets zero data retention with LLM providers.

**Pricing tiers:** Official current pricing: Free $0/member/month; Plus $10/member/month; Business $20/member/month; Enterprise custom. Free includes trial AI capabilities. Plus includes trial AI capabilities. Business includes full AI workspace features. Enterprise includes Business plus advanced security and zero-data-retention LLM provider handling. The 2025 pricing-change help page is important: as of May 13, 2025, Notion AI stopped being an add-on for new Free/Plus users and became available by default only with Business or Enterprise. Existing users who had the old add-on keep access while subscribed. BYO-key support was not found in Notion's official product docs.

**Customer segments:** The pricing page says Notion is trusted by teams at OpenAI, Figma, Volvo, Ramp, and Cursor. Navigation and product pages segment by engineering/product, design, marketing, IT, startups, small businesses, enterprise, education, personal, professional, wiki/knowledge-base, docs, and projects.

**Active user / install signals:** No current official DAU/MAU/paying-user count found in the pages opened. Public traction is logo- and category-based. Notion is clearly broader than KM: it is a company workspace platform.

**Documented user complaints:** Current complaints focus on AI packaging and per-credit agent monetization. One Reddit thread says Notion AI database work "arbitrarily stops at 20 rows" and pushes users toward Autofill / Agents (https://www.reddit.com/r/Notion/comments/1stur3s/notion_ai_feels_intentionally_limited_just_to/). Another says Custom Agents pricing and no-rollover credits make "even a simple daily agent cost about $30/month" (https://www.reddit.com/r/Notion/comments/1rdd3av/petition_the_new_pricing_of_notion_custom_agents/). A separate thread says AI Autofill moved from a lightweight included feature into a monetized Custom Agent path (https://www.reddit.com/r/Notion/comments/1rz57qp/ai_autofill_on_properties_is_now_a_custom_agent/).

**Positioning relative to agent-harness:** Notion overlaps on workspace knowledge, databases, relations, and increasingly "agent does work in a workspace." The overlap becomes cross-lane when Notion Agent and Custom Agents are considered because those are cloud-agent/workflow-agent features. Users of Notion AI are candidates for agent-harness if they are frustrated by Business-only bundling, credit pricing, or lack of local/cross-CLI orchestration. However, Notion's primary market is broader collaboration and company operating system, not local agent context graphs.

**Notes:** Notion is a pricing anchor: AI is now bundled upward into Business rather than sold as a simple add-on to every plan. That raises the minimum durable AI price for many small teams from Plus to Business.

### P-3 - Mem.ai

**Vendor / docs / pricing:** https://get.mem.ai/pricing

**Hierarchy primitive:** hybrid note objects / collections with an AI-first "self-organizing" workspace posture. Mem is less hierarchical than Tana, Roam, Logseq, or Obsidian; its differentiation is retrieval and automatic organization rather than user-authored topology.

**AI features:** Mem pricing emphasizes AI model selection, unlimited chat messages on Pro, unlimited deep searches, templates, connected emails, API keys, and beta features such as meeting briefs. Vendor messaging in pricing testimony says users can "toss it in Mem" and not worry how to organize notes.

**Pricing tiers:** Static pricing page lists Free with 25 notes/month and 25 chat messages/month. Mem Pro is $12/month and includes dark mode, AI model selection, unlimited notes, unlimited chat messages, unlimited deep searches, unlimited collections, unlimited templates, unlimited connected emails, unlimited API keys, and beta meeting briefs. The page also contains a repeated "$14.99" Mem label in the extracted text; the clean plan line says Pro $12/month. BYO-key support is implied by "unlimited API keys," but the pricing page does not spell out provider scope.

**Customer segments:** Mem positions for individuals and teams, especially knowledge workers who do not want to manually organize notes. The pricing page includes a named testimonial from Zach Price, Founder at Impulse to Impact. No specific vertical pages were found in the extracted pricing page.

**Active user / install signals:** No official active-user or revenue number found. No app-store or extension install count collected.

**Documented user complaints:** Complaints focus on the promise of self-organization not matching expectations and pricing. A Reddit user said they tried Mem, which claimed to be the first self-organizing workspace, but "all of the organizing boiled down to manual tagging" (https://www.reddit.com/r/NoteTaking/comments/14ocf1k). Another Mem.ai 2.0 thread says Mem shines at AI retrieval but "the price is right though it should be a little bit cheaper" (https://www.reddit.com/r/PKMS/comments/1nvps6d/memai_20/). Older pricing complaints called smart writing expensive and asked whether AI token usage is limited (https://www.reddit.com/r/PKMS/comments/zovvkv).

**Positioning relative to agent-harness:** Mem overlaps less on explicit graph walking and more on "AI can retrieve my notes without manual structure." That is a different value proposition from agent-harness, which assumes explicit graph/state control. Mem users are candidates if they outgrow passive retrieval and need durable workstream state, but Mem itself is not evidence that users want agent-owned pack/unpack mechanics.

**Notes:** Mem is a cautionary market signal: the "self-organizing" claim attracts demand, but users complain when the product still requires manual organization.

### P-4 - Reflect

**Vendor / docs / pricing:** https://reflect.app/

**Hierarchy primitive:** networked notes with backlinks and graph visualization. Reflect is graph-adjacent but deliberately simpler than Roam/Tana.

**AI features:** Reflect says it uses GPT-4 and Whisper from OpenAI to improve writing, organize thoughts, and act as an intellectual thought partner. The landing page emphasizes AI-assisted notes, networked note-taking, calendar integration, publishing, instant capture, web clipping, Kindle sync, Readwise, and Google/Outlook integration.

**Pricing tiers:** Reflect says "one plan one price": $10/month billed annually. The extracted page does not list a free tier. BYO-key support was not found on the official landing page.

**Customer segments:** Reflect targets "thinkers" and professionals who want fast personal notes. Public proof is a "wall of love" from named people / social handles rather than vertical case studies.

**Active user / install signals:** No official active-user count found.

**Documented user complaints:** Public complaints focus on missing advanced capabilities and subscription packaging. In a PKMS Reddit thread, a user comparing Reflect with other tools says "Reflect notes doesn't have that" while discussing expected features, though they liked integrated AI (https://www.reddit.com/r/PKMS/comments/z5fntj). Another thread says Reflect discontinued monthly pricing but may extend trials (https://www.reddit.com/r/PKMS/comments/1azkqh4).

**Positioning relative to agent-harness:** Reflect users are plausible candidates if they value networked notes plus built-in AI but want more operational execution. The product does not seem to overlap with agent-harness's live graph optimizer or multi-agent work management.

**Notes:** Reflect is a clean signal for a $10/month personal AI-note subscription, but its market proof is weaker than Notion/Tana because it publishes fewer numbers.

### P-5 - Obsidian + AI Plugins

**Vendor / docs / pricing:** https://obsidian.md/pricing, https://smartconnections.app/smart-connections/, https://www.obsidiancopilot.com/en/pricing

**Hierarchy primitive:** local Markdown files, folders, wikilinks, backlinks, graph view, and plugins. Hierarchy is files/folders plus links; AI mostly comes from third-party plugins, not Obsidian core.

**AI features:** Obsidian itself does not sell a first-party AI add-on in the pricing page. AI comes from ecosystem plugins. Smart Connections provides local-first semantic search, local embeddings, related-note suggestions, smart lookup, Smart Context bundles, Smart Graph, Smart Chat, and optional cloud providers. It says core indexing/retrieval run locally by default and no API key is required for Smart Connections Core. Copilot for Obsidian offers BYO-key chat, image support, local data store for Vault Q&A, embedding-powered vault search, and paid Plus features including built-in model, smarter vault search, AI agent with web search / document reading / long-term memory, project context, and chat with PDFs/EPUBs/50+ file types.

**Pricing tiers:** Obsidian core is free without limits, no sign-up. Obsidian Sync is $4/user/month annual or $5 monthly; Publish is $8/site/month annual or $10 monthly; Catalyst is $25 one-time; Commercial license is $50/user/year but not required for commercial use. Smart Connections Core is free; Pro is optional, but the extracted Smart Connections page does not expose a price. Copilot Free is $0 with BYO-key and open-source features. Copilot Plus is $14.99/month monthly or $139.99/year ($11.67/month) annual. Copilot Self-Host Supporter is $349.99 lifetime self-host access and includes two years of Plus.

**Customer segments:** Obsidian targets privacy/local-first personal knowledge management, writers, researchers, builders, and technical users. Smart Connections explicitly says it is built for "writers, researchers, and builders." Copilot targets users who want a thought partner that grows with their second brain.

**Active user / install signals:** Smart Connections subreddit discussion claims the plugin had over 800,000 downloads, but that is a Reddit claim rather than official plugin-store text (https://www.reddit.com/r/ObsidianMD/comments/1qpo8yj/smart_connections_plugin_quietly_switches_to_a/). No official Obsidian DAU/MAU count was found in the pricing page.

**Documented user complaints:** The biggest recent complaint is Smart Connections' license shift. A Reddit thread says the plugin "quietly switches to a proprietary license" and describes a noncompete clause that blocks forks from removing the paywall (https://www.reddit.com/r/ObsidianMD/comments/1qpo8yj/smart_connections_plugin_quietly_switches_to_a/). Other complaints are setup and reliability: one user says they did not have the option to use anything other than local AI models (https://www.reddit.com/r/ObsidianMD/comments/1ftx9sv), and another says Smart Connections disappeared from the plugin list/chat (https://www.reddit.com/r/ObsidianMD/comments/1fcyevp). Smart Connections' own FAQ warns initial embedding can be resource-heavy for large vaults.

**Positioning relative to agent-harness:** Obsidian is close on local files, graph-ish notes, plugin extensibility, and local-first semantics. The ecosystem shows real user demand for semantic retrieval and chat over a vault. It does not impose a curated working set on a long-lived agent; most plugins are consulted by the user or chat assistant. Users of Obsidian AI plugins are strong agent-harness candidates if they already pay or configure AI around a knowledge base but want execution and graph-state discipline.

**Notes:** Obsidian's market structure differs from SaaS competitors: core product monetizes sync/publish/licenses, while AI monetization is mostly third-party plugin subscription or BYO-key.

### P-6 - Roam Research

**Vendor / docs / pricing:** https://roamresearch.com/, public pricing references at https://www.producthunt.com/products/roam-research/questions/is-roam-research-free and https://www.reddit.com/r/RoamResearch/comments/gywjc3

**Hierarchy primitive:** outliner blocks, pages, backlinks, block references, graph database. Roam is a foundational graph/outliner product.

**AI features:** Official text-rendered pages did not expose current AI product docs. Third-party pages describe AI enhancements, but primary-source evidence found here does not show a current first-party Roam AI feature comparable to Notion AI, Tana AI, or Capacities AI. Treat Roam AI status as an evidence gap unless a logged-in Roam help page or official announcement is supplied.

**Pricing tiers:** Product Hunt's Roam Research page says no free plan; monthly payments are $15/month, annual upfront is $13.75/month, and Believer is $500 for five years with first access to new features, community calls, and priority support. Reddit launch/pricing discussions cite the same $500/5-year Believer plan. Because Roam's official site did not return static pricing text, these should be treated as public pricing evidence but weaker than a live vendor pricing table.

**Customer segments:** Researchers, writers, academics, and power PKM users. Roam's own slogan, visible on the website title, is "A note taking tool for networked thought." No current official logo/customer page was extracted.

**Active user / install signals:** No official active-user or revenue number found.

**Documented user complaints:** Price and perceived stagnation dominate. One thread asks whether "$15/month" is justifiable and says the price is not realistic for many students or users in weaker-currency countries (https://www.reddit.com/r/RoamResearch/comments/y314za). Another says the broader tools-for-thought market is growing while Roam appears to be moving in "the exact opposite" direction, citing quiet release notes and community decline (https://www.reddit.com/r/RoamResearch/comments/1dybuul).

**Positioning relative to agent-harness:** Roam's block graph is conceptually adjacent to agent-harness, but the current commercial evidence does not show active first-party AI packaging. Roam users are candidates because they already value block references and graph navigation, but Roam itself is not strong evidence for current AI adoption.

**Notes:** Roam remains a category-defining reference point, but not a current AI-pricing benchmark unless better official evidence is supplied.

### P-7 - Logseq

**Vendor / docs / pricing:** https://github.com/logseq/logseq, https://blog.logseq.com/how-to-setup-and-use-logseq-sync/, https://github.com/briansunter/logseq-plugin-gpt3-openai, https://discuss.logseq.com/t/how-is-logseq-s-official-development-aligning-with-the-emerging-agentic-ai-trend/34823

**Hierarchy primitive:** outliner blocks over local Markdown or Org-mode files, pages, backlinks, graph, and a new DB architecture. Local-first / open-source is central.

**AI features:** Logseq core does not present a first-party AI subscription in the sources found. AI exists via plugins, such as `logseq-plugin-gpt3-openai`, which adds GPT commands, page/block generation, summarization/explanation, translation, classification, keyword tagging, and image generation through OpenAI APIs. A 2026 official forum response from a Logseq team member says Logseq DB has an MCP server and CLI, so users can use AI chatbots with LLMs through those interfaces. That is an interoperability/agent-access signal, not a packaged Notion-style AI product.

**Pricing tiers:** Logseq is open source. Logseq Sync beta access is available to active Open Collective contributors with monthly recurring donation of $5/month or $15/month; the official Sync setup blog says a normal subscription system will come later. Sync is still described as a public beta and not collaboration. Storage limits during beta: max 10 remote graphs, max graph size 10 GB each, max asset 100 MB. Plugin AI costs are BYO OpenAI API cost.

**Customer segments:** Privacy-first local note-takers, academics, developers, students, researchers, PKM users. No current official customer logo list was found in first-party sources.

**Active user / install signals:** GitHub page is primary for OSS traction. Extracted GitHub text from a repository leaderboard reported 41,011 stars and 2,455 forks for `logseq/logseq`, but that leaderboard is not GitHub's official page. The direct GitHub repository was opened but the static extraction did not surface star counts. Use GitHub itself as the primary repo reference and treat exact counts as volatile.

**Documented user complaints:** The official Sync blog itself warns that Sync is "still very much a beta test," does not support collaboration, and "issues like data loss can still occur." It also warns users may overwrite cloud notes because current sync does not compare pages. A forum discussion about agentic AI contains user concern that moving from Markdown files to database storage "seems like a move away from easy integration with AI tools"; a Logseq team member replies that DB has MCP, CLI, and HTTP API and can export Markdown/EDN/JSON. On Reddit, a user complains of "very sparse communication" and plugin authors moving away (https://www.reddit.com/r/logseq/comments/1pigcs1/so_long_and_thanks_for_all_the_fish/).

**Positioning relative to agent-harness:** Logseq is highly adjacent in data model and agent-access pattern: outliner, graph, local-first, CLI/MCP, and OSS. The key distinction is that Logseq is a note graph; agent-harness is an imposed working-set orchestrator over multi-CLI agent work. Logseq users and plugin builders are likely high-fit candidates.

**Notes:** Logseq is evidence for demand for local graph notes and agent-access APIs, but not for paid AI feature adoption.

### P-8 - Capacities

**Vendor / docs / pricing:** https://capacities.io/pricing/, https://docs.capacities.io/reference/ai-assistant, https://capacities.io/product/ai

**Hierarchy primitive:** typed objects, collections, tags, properties, backlinks, daily notes. Capacities is object-dominant rather than outliner-dominant.

**AI features:** Capacities AI Assistant supports AI Chats with note/block context, AI Panel for translation/rewriting/summarization, AI auto-tagging, AI collection selection, AI property autofill, AI image analysis with OCR/handwriting extraction/color palette/category, and `/Ask AI`. AI can search notes and read notes from backlinks/search results. AI chats can be saved as objects, making conversations part of the knowledge base. Capacities uses current OpenAI GPT models for Capacities AI and supports BYO keys for OpenAI, Gemini, Anthropic, Mistral, and xAI. Users can restrict keys by space and set monthly dollar budgets.

**Pricing tiers:** The official pricing page's static extraction confirms free core product but did not expose numeric plan prices. Third-party search snippets consistently report Free, Pro $11.99/month or $9.99/month annual, Believer $14.99/month or $12.49/month annual; because the user asked for primary sources, those numbers should be treated as unverified unless a rendered price table is captured separately. Official AI docs say AI is tied to a subscription, has a daily budget, and BYO keys can exceed limits. Local-only AI is not supported yet; docs say it is "on the radar."

**Customer segments:** Capacities targets people building knowledge, with examples around students, researchers, writers, project notes, books, people, and abstract thinking. The product page includes named testimonials/social handles but no quantified enterprise customer list.

**Active user / install signals:** No official user count found in extracted pages.

**Documented user complaints:** Complaints are mostly about maturity and pricing relative to beta status. A Tana/Capacities comparison thread says Capacities pricing seemed "crazy" given it was still in beta and missing core functionality (https://www.reddit.com/r/TanaInc/comments/18pk36j). Capacities' own AI docs also imply limit friction: daily budget is dynamically adjusted, and exceeding it requires BYO API keys.

**Positioning relative to agent-harness:** Capacities overlaps with typed objects, properties, auto-tagging, and AI over linked notes. The saved-AI-chat-as-object pattern is particularly relevant because it persists AI interactions into the graph. It does not manage multi-agent execution or impose working sets. Capacities users are plausible candidates if they need more deterministic graph operations and execution.

**Notes:** Capacities is a strong object-schema counterpart to Tana's supertag schema.

### P-9 - Heptabase

**Vendor / docs / pricing:** https://support.heptabase.com/en/articles/12990121-pro-premium-and-premium-plans-pricing-faq

**Hierarchy primitive:** visual whiteboards, cards, sections, mind-map style spatial organization, plus PDFs and knowledge artifacts. It is visual-board dominant rather than outliner or page dominant.

**AI features:** Heptabase AI credits can be spent on AI chat, video transcript, insight generator, and voice note. Premium users get model choices from Gemini, OpenAI, and Anthropic; Pro users are limited to Gemini-series models in AI chat. Premium/Premium+ support unlimited PDF uploads and parsing. BYO OpenAI or Anthropic API key is supported for users who want higher-tier models or run out of credits.

**Pricing tiers:** Pro early bird $9.99 monthly or $83.88 yearly. Pro $11.99 monthly or $107.88 yearly. Premium $23.99 monthly or $215.88 yearly. Premium+ $71.99 monthly or $647.88 yearly. Pro includes 100 AI credits/month. Premium includes 1,800 AI credits/month. Premium+ includes 8,100 AI credits/month. Free trial lasts up to seven days or until trial AI credits are used. Heptabase does not yet support purchasing extra AI credits as an add-on; it says this is on the roadmap.

**Customer segments:** Heptabase targets visual learners, researchers, knowledge workers, students, creators, and people managing complex learning/research. The pricing FAQ does not list logos.

**Active user / install signals:** No official active-user or revenue number found in the pricing FAQ.

**Documented user complaints:** A feature request asks for OpenAI-compatible provider support, indicating users want broader provider flexibility than first-party supported keys (https://www.reddit.com/r/heptabase/comments/1on229p/feature_request_add_support_for_openaicompatible/). The large price jump to Premium+ is also visible in official pricing, but I did not find a primary complaint thread specifically about the new Premium+ price.

**Positioning relative to agent-harness:** Heptabase overlaps with visual knowledge-base AI and, notably, a recent Reddit release note says Heptabase introduced a CLI for coding agents to interact with the local knowledge base (https://www.reddit.com/r/heptabase/comments/1ssi2mb/heptabase_v1910_update_cli_for_coding_agents/). That release-note source is not a vendor page in the extracted evidence, but if accurate it is highly adjacent. Heptabase users are candidates when visual thinking meets agent workflows.

**Notes:** Heptabase is one of the clearest AI-credit pricing examples: narrow Pro trial allocation, mid-tier full model choice, high-tier large quota.

### P-10 - Anytype

**Vendor / docs / pricing:** https://doc.anytype.io/anytype-docs/advanced/monetization, https://community.anytype.io/tag/roadmap/84, https://business.anytype.io/

**Hierarchy primitive:** local-first object graph: objects, types, relations/properties, sets/collections, spaces. Anytype is typed-object / graph dominant.

**AI features:** Anytype's consumer docs opened here do not show a packaged AI assistant. Roadmap topic listings include "Roadmap for local AI/LLM Integration?" and "AI" tags. Anytype Business page says Model Context Protocol support enables AI agent collaboration, but this is business/collaboration positioning rather than a consumer AI note assistant. Treat Anytype consumer AI as roadmap/interoperability status, not shipped Notion-style AI.

**Pricing tiers:** Free: 100 MB remote storage, 10 shared spaces, unlimited private spaces. Plus: 1 GB remote storage, unlimited shared/private spaces, ANY ID 9+ symbols. Pro: 10 GB remote storage, unlimited spaces, ANY ID 7+ symbols. Ultra: 100 GB remote storage, unlimited spaces, ANY ID 5+ symbols. The docs page did not expose numeric USD prices in static extraction. Contributors and students/faculty are eligible for 50% discounts. Self-hosters can manage viewer/editor limits themselves and may buy names/support at the same membership prices.

**Customer segments:** Local-first privacy-conscious individuals and, via Anytype Business, sovereign collaboration customers. No named customer logos found in extracted pages.

**Active user / install signals:** No official active-user number found in extracted pages.

**Documented user complaints:** The community top page shows multiple active complaint/feedback topics: "DO NOT Waste Time on AI," "Editor (and overall software) quality - some priority here, please?", and roadmap update threads (https://community.anytype.io/top). The roadmap tag page shows the local AI/LLM integration topic had 22 replies and 1,970 views, which is a public signal of demand but not a product commitment.

**Positioning relative to agent-harness:** Anytype is architecturally adjacent because it is local-first, object graph oriented, and moving toward MCP/agent collaboration in business positioning. It is not currently a direct AI-KM pricing benchmark because AI features are not clearly packaged in consumer plans. Anytype users are good candidates if they value local ownership and structured graph objects.

**Notes:** Anytype belongs in the taxonomy, but current AI adoption evidence is weaker than Tana, Notion, Capacities, or Heptabase.

### P-11 - Apple Notes / Apple Intelligence

**Vendor / docs / pricing:** https://support.apple.com/guide/iphone/use-writing-tools-iph6f08da1d2/ios, https://podcasters.apple.com/support/5588-writing-tools-shape-podcast-copy

**Hierarchy primitive:** folders, notes, tags, links, attachments, shared notes. Apple Notes is not graph-native but is an incumbent personal note store.

**AI features:** Apple Intelligence Writing Tools work in Notes and other apps: proofread, rewrite, change tone, describe a change, summarize, key points, list, table, and compose text from scratch. Apple creator docs say Writing Tools are accessible in Notes, Pages, Messages, and Mail. Apple Intelligence also supports audio recording summaries in Notes in current OS features, but the opened official support page here focused on writing tools.

**Pricing tiers:** Apple Notes is included with Apple devices / Apple accounts; AI availability depends on Apple Intelligence-supported hardware, OS, language, and region. No per-note AI subscription or BYO-key support found.

**Customer segments:** Consumer, students, professionals, educators, and anyone in the Apple ecosystem. The Apple Education community includes AI-powered observation notes for teachers (https://education.apple.com/resource/250014925), suggesting education workflows.

**Active user / install signals:** No Notes-specific active-user count found.

**Documented user complaints:** Complaints focus on closed data and limited batch/graph AI. One Reddit thread says Apple Notes is "too closed" for handing notes to AI for batch analysis/summarization/tagging (https://www.reddit.com/r/AppleNotesGang/comments/1nxpvy0/apple_notes_is_too_closed_is_there_a_good_way_to/). Another complains Writing Tools make "all or nothing" blind changes outside Notes, making review hard (https://www.reddit.com/r/ios/comments/1hdygoe).

**Positioning relative to agent-harness:** Apple Notes is an incumbent threat only at the lightweight end: built-in writing and summarization reduce willingness to pay for simple AI note polish. It does not overlap with agent graph walking.

**Notes:** Apple sets a "free with OS" baseline for simple AI writing/summarization.

### P-12 - Microsoft Loop / Copilot Pages / Copilot Notebooks

**Vendor / docs / pricing:** https://support.microsoft.com/en-us/office/frequently-asked-questions-about-copilot-in-loop-cdba1c99-3a3f-4f0d-bb1b-8ca62d0bb23d, https://support.microsoft.com/en-us/topic/compare-microsoft-loop-copilot-pages-and-copilot-notebooks-d6bf4dc0-7b09-4f54-9a92-ff898b8397f7

**Hierarchy primitive:** Loop workspaces, pages, portable components, lists, tables, tasks; Copilot Pages are flat editable canvases; Copilot Notebooks are hierarchical project contexts grouping files, chats, pages, links, meeting notes, and instructions.

**AI features:** Copilot in Loop can ask general or enterprise-specific questions, insert Copilot responses onto pages, recap changes, and produce collaborative meeting notes / suggested tasks. Microsoft says some Copilot integrations were removed from Loop in late 2025 to align Loop with Microsoft 365 Copilot Pages. The comparison doc says Loop has medium AI integration; Copilot Pages has high integration; Copilot Notebooks has very high AI integration grounded in notebook content.

**Pricing tiers:** Microsoft says a Microsoft 365 Copilot license is required to use Copilot in Loop, and a Microsoft 365 E3 or E5 license is required to purchase a Microsoft 365 Copilot license. This places AI Loop in enterprise Microsoft 365 licensing, not consumer PKM pricing. BYO-key not supported.

**Customer segments:** Enterprise Microsoft 365 organizations, teams, managers, meeting-heavy knowledge workers. Microsoft explicitly frames Loop for collaboration, Copilot Pages for refining generated content, and Copilot Notebooks for research, onboarding, or content development.

**Active user / install signals:** No Loop active-user number found in official support pages.

**Documented user complaints:** Microsoft itself documents capability churn: in late 2025 it "removed some Copilot integrations" from Loop due to user confusion. A Reddit thread complains about retirement of Copilot-generated recaps (https://www.reddit.com/r/Office365/comments/1sa8g76/wtf_microsoft_loop_retirement_of_copilotgenerated/), matching the official reduction/realignment. Another user speculates that Copilot Pages/Notebooks may be replacing Loop focus (https://www.reddit.com/r/microsoft/comments/1ri1pu1/question_regarding_loop/).

**Positioning relative to agent-harness:** Loop/Copilot is cross-lane at the enterprise suite layer. It overlaps with shared pages and AI summaries, not local graph orchestration. Its pricing and licensing are useful as an incumbent benchmark: enterprise AI features are bundled into M365 Copilot rather than priced as a note-app add-on.

**Notes:** Microsoft evidence suggests incumbents can reshuffle AI features between adjacent canvases, creating user confusion.

### P-13 - Mem0 / OpenMemory

**Vendor / docs / pricing:** https://mem0.ai/

**Hierarchy primitive:** dedicated memory layer for AI apps, not a consumer hierarchical note/outliner product.

**AI features:** Mem0's website positions it as "The Memory Layer for your AI Apps." This is architecturally related to persistent memory, but it is a developer/API memory product rather than a note graph users organize.

**Pricing tiers:** Not analyzed in this lane. Dedicated memory products are cross-lane by the user's rule.

**Customer segments:** Developers building AI apps, agent frameworks, and memory-enabled products.

**Active user / install signals:** Not collected because it is cross-lane.

**Documented user complaints:** Not collected because it is cross-lane.

**Positioning relative to agent-harness:** Cross-lane. Mem0 may be architecturally informative for memory APIs, but it is not commercial evidence for users paying for graph-organized KM with AI.

**Notes:** Keep separate from Mem.ai, which is a direct-to-consumer AI notes product.

### P-14 - Saga

**Vendor / docs / pricing:** https://www.saga.so/pricing, https://www.saga.so/ai

**Hierarchy primitive:** pages / linked notes / workspace. Saga is adjacent to the note workspace category.

**AI features:** Evidence gap. The official URLs were searched/opened, but usable static text was not returned in the extracted source set. Do not rely on unsourced claims.

**Pricing tiers:** Evidence gap from primary static source. Needs a rendered-page capture or vendor docs export.

**Customer segments:** Evidence gap.

**Active user / install signals:** Not found.

**Documented user complaints:** Not found in the collected source set.

**Positioning relative to agent-harness:** Likely adjacent lightweight KM, but not enough primary evidence in this pass to compare responsibly.

**Notes:** This is one of the clearest open evidence gaps.

### P-15 - NotePlan

**Vendor / docs / pricing:** https://noteplan.co/pricing, https://help.noteplan.co/article/230-ai-in-noteplan

**Hierarchy primitive:** files/notes plus calendar/task hierarchy. NotePlan is day/project-note centered rather than graph-native.

**AI features:** The pricing page does not describe AI. The named AI help article URL was attempted, but usable text was not returned in the extracted source set. Public Reddit notes a separate "Memo AI" app in the NotePlan ecosystem, with AI running on recordings and costing per use (https://www.reddit.com/r/noteplanapp/comments/1sr52ns/memo_ai_is_live_on_the_app_store/), but that appears separate from core NotePlan.

**Pricing tiers:** Annual $8.33/month, $99 paid annually, 7-day trial. Monthly $12/month, 3-day trial. All plans include full access across devices and there are "no additional payments" according to the pricing page.

**Customer segments:** Professionals who combine tasks, projects, calendar, and knowledge management. Stronger productivity/planning than graph-KM.

**Active user / install signals:** No official active-user number found.

**Documented user complaints:** A NotePlan subreddit thread complained the price did not appear clearly on the website and called pricing policy "not really transparent" (https://www.reddit.com/r/noteplanapp/comments/wqvh54). Current pricing page is now explicit.

**Positioning relative to agent-harness:** NotePlan is adjacent through daily notes and task/project planning, but not an agent graph product. Users may be candidates if they want long-running task orchestration rather than personal planning.

**Notes:** Need better primary evidence for NotePlan AI status before treating it as AI-augmented KM.

### P-16 - Amplenote

**Vendor / docs / pricing:** https://www.amplenote.com/subscriptions/new, https://www.amplenote.com/help/plans_and_prices_billing, https://www.amplenote.com/plugins/ample_agent_pro

**Hierarchy primitive:** notes, tasks, calendars, tags, backlinks; productivity-note hybrid.

**AI features:** Ample Agent Pro is an Amplenote plugin subscription. It unlocks AI writing tools, note and task agents, model-powered research, and a dashboard connecting daily agenda to quarterly goals. The page says there are 16 AI features and 0 AI keys required for the core suite. Included features: AmpleAI writing/editing suite, Task Agent, Note Agent, dashboard task ideas from Quarterly Goals, and monthly quota of free AI use. Deep Research uses provider API keys the user connects; other core AI features do not require separate keys.

**Pricing tiers:** Base Amplenote has Free plus Basic/Pro/Founder subscription levels. Help page says Basic costs $6.99/month or $71.88/year. The plugin page says Ample Agent Pro costs $8/month. The subscription page says all accounts offer advanced note taking and task scheduling across platforms for free, but detailed Pro/Founder pricing was not exposed in the extracted text. BYO-key appears only for Deep Research in Ample Agent Pro.

**Customer segments:** Productivity users who blend notes, tasks, schedules, and execution. Ample Agent Pro explicitly targets users who want AI "woven into capture, planning, writing, and execution."

**Active user / install signals:** No official user count found.

**Documented user complaints:** No strong current AI complaint found in collected primary/community sources. General gaps: the pricing help page is richer than the subscription page static extraction; detailed plan comparison needs rendered capture.

**Positioning relative to agent-harness:** Amplenote is closer than NotePlan because "note and task agents" plus quarterly/daily planning starts to point toward execution. It remains a note/task app plugin, not an external multi-CLI orchestrator.

**Notes:** Amplenote shows a different AI monetization pattern: plugin subscription on top of a freemium/productivity app.

## Cross-product Patterns

### Pricing Patterns

AI pricing clusters into four models.

1. **Bundled upward into higher SaaS tiers:** Notion is the strongest example. Since May 13, 2025, new users need Business or Enterprise for full Notion AI. Microsoft Loop requires Microsoft 365 Copilot plus E3/E5 eligibility. This moves AI from "personal add-on" into business-suite packaging.

2. **AI credits included per plan:** Tana, Heptabase, and likely Capacities use budget/credit patterns. Tana Free gets 500 credits, Plus 2,000, Pro 5,000. Heptabase Pro gets 100, Premium 1,800, Premium+ 8,100. Credits are explained in user-output equivalents: meetings, images, transcriptions, blog posts.

3. **BYO-key local/plugin ecosystems:** Obsidian Copilot, Smart Connections, Logseq GPT plugins, Capacities BYO keys, and Heptabase BYO keys all use user-supplied provider accounts for power users. BYO-key is strongest in local-first ecosystems and weaker in polished SaaS.

4. **Separate AI plugin subscription:** Amplenote's Ample Agent Pro is $8/month on top of the main app. Obsidian Copilot Plus is also effectively an AI plugin subscription.

Personal AI-KM pricing baselines: $8-$15/month is common for personal tools or plugins (Reflect $10/year-billed monthly equivalent; Mem Pro $12; Obsidian Copilot Plus $11.67 annual / $14.99 monthly; NotePlan $8.33 annual / $12 monthly; Ample Agent Pro $8). Higher AI quotas push into $20-$24/month (Notion Business $20; Heptabase Premium $23.99; Tana Pro inferred $18/month). High-consumption AI can be much higher (Heptabase Premium+ $71.99/month).

### Hierarchy-primitive Patterns

Outliner-dominant products: Tana, Roam, Logseq. They expose block/node structure and backlinks naturally.

Page/database-dominant products: Notion, Microsoft Loop/Copilot Pages, Saga. They organize through pages and databases/components rather than visible block graphs.

Typed-object dominant products: Capacities and Anytype. These are important because they map naturally to structured AI autofill, object properties, and relations.

File/local graph products: Obsidian and Logseq file mode. These are the strongest fit for local AI agents because notes are accessible as files or through CLI/MCP.

Visual-spatial products: Heptabase. It monetizes research/learning through boards and AI insight generation rather than deep outliner operations.

### AI Feature Patterns

Common features now table stakes: summarize, rewrite, translate, chat with selected note/page context, ask questions over notes, meeting/audio transcription, and property/database autofill where structured data exists.

Emerging features: AI agents inside the workspace (Notion Agent, Amplenote Note/Task Agent, Tana AI agents/commands, Obsidian Copilot agents), AI-triggered automations/events, graph-aware semantic search, saved AI chats as durable objects, and MCP/CLI surfaces for external agents.

Features still rare or missing: explicit bounded working-set policy; stable identity across summary regeneration/split/merge; live foreground agent plus background optimizer concurrency; graph mutation provenance as a first-class user-visible contract; cross-CLI agent session orchestration. Tana/Logseq/Heptabase/Anytype are moving toward agent-access surfaces, but not toward the full agent-harness pattern.

### Customer-segment Patterns

The market repeatedly targets: researchers, writers, students, product managers, designers, engineers, founders, solo entrepreneurs, creators, managers, and meeting-heavy professionals. Notion/Microsoft skew team/enterprise; Tana and Capacities span individual PKM and teams; Obsidian/Logseq skew technical/local-first PKM; Reflect/Mem target personal professional notes; Heptabase targets visual learning/research.

### Complaints Pattern

User complaints cluster around five issues:

- AI pricing opacity or credit anxiety: Notion Custom Agents, Tana credits/top-ups, Heptabase tier jumps, Mem pricing.
- AI quality and editability: Tana AI search, Notion Autofill reliability, Apple Writing Tools review UX.
- Complexity: Tana settings, Obsidian plugin configuration, Capacities object model.
- Trust / local control: Smart Connections license shift, Apple Notes closed export, Logseq DB versus Markdown concerns.
- Product direction churn: Microsoft Loop Copilot feature removals, Logseq sparse communication, Roam stagnation perception.

## Cross-lane Signals

Cloud agent products are cross-lane. Notion Custom Agents and Notion Agent partially overlap but should be treated as workspace cloud-agent products, not KM-only products.

Desktop assistants are cross-lane. Apple Intelligence and Microsoft Copilot are incumbents that lower willingness to pay for simple AI writing, but they are not graph-oriented agent harnesses.

Agent SDKs are cross-lane. Logseq MCP/CLI, Tana Local API/MCP, Heptabase CLI, and Anytype Business MCP are interoperability signals. They are relevant to distribution and integration but not the same as a commercial KM app with bundled AI.

Dedicated memory products are cross-lane. Mem0 is architecturally adjacent but positioned as an AI-app memory layer, not a consumer note graph.

## Open Evidence Gaps

- Tana pricing main table did not expose list prices in extracted HTML; Plus/Pro prices are inferred from official 50% discount text.
- Roam's official current pricing/AI pages did not return static text. Pricing evidence relies on Product Hunt and Reddit unless a logged-in or rendered source is supplied.
- Capacities official pricing page did not expose numeric plan prices in static extraction. AI docs are strong; price numbers need rendered confirmation.
- Saga AI/pricing pages did not return usable static text. Needs rendered-page capture or vendor docs.
- NotePlan AI status needs better primary evidence. Pricing is clear; AI is not.
- Anytype consumer AI status is roadmap/interoperability rather than a shipped AI assistant in the collected evidence.
- Active-user, paying-user, DAU/MAU, and revenue numbers are mostly absent across vendors. Public traction usually appears as customer logos, GitHub stars, plugin downloads, or testimonials.

## Citations

- Tana AI docs: https://outliner.tana.inc/docs/tana-ai
- Tana pricing: https://outliner.tana.inc/pricing
- Tana AI command nodes: https://outliner.tana.inc/docs/ai-command-nodes
- Tana AI for builders: https://outliner.tana.inc/docs/ai-for-builders
- Tana 2025 updates: https://outliner.tana.inc/articles/whats-new-in-tana-2025-product-updates
- Tana AI search complaint: https://www.reddit.com/r/TanaInc/comments/1n1myvk
- Tana AI settings complaint: https://www.reddit.com/r/TanaInc/comments/1ln0uid
- Tana tables/AI complaint: https://www.reddit.com/r/TanaInc/comments/1suv5ru/tables_and_tanas_ai_vs_notion/
- Notion pricing: https://www.notion.com/pricing
- Notion 2025 pricing changes: https://www.notion.com/help/2025-pricing-changes
- Notion AI security: https://www.notion.com/help/notion-ai-security-practices
- Notion AI databases guide: https://www.notion.com/help/guides/notion-ai-for-databases
- Notion AI 20-row complaint: https://www.reddit.com/r/Notion/comments/1stur3s/notion_ai_feels_intentionally_limited_just_to/
- Notion Custom Agents pricing complaint: https://www.reddit.com/r/Notion/comments/1rdd3av/petition_the_new_pricing_of_notion_custom_agents/
- Notion Autofill complaint: https://www.reddit.com/r/Notion/comments/1rz57qp/ai_autofill_on_properties_is_now_a_custom_agent/
- Mem pricing: https://get.mem.ai/pricing
- Mem self-organizing complaint: https://www.reddit.com/r/NoteTaking/comments/14ocf1k
- Mem 2.0 pricing/retrieval thread: https://www.reddit.com/r/PKMS/comments/1nvps6d/memai_20/
- Mem smart writing pricing complaint: https://www.reddit.com/r/PKMS/comments/zovvkv
- Reflect official site/pricing: https://reflect.app/
- Reflect PKMS thread: https://www.reddit.com/r/PKMS/comments/z5fntj
- Reflect monthly plan thread: https://www.reddit.com/r/PKMS/comments/1azkqh4
- Obsidian pricing: https://obsidian.md/pricing
- Smart Connections: https://smartconnections.app/smart-connections/
- Copilot for Obsidian pricing: https://www.obsidiancopilot.com/en/pricing
- Smart Connections license complaint: https://www.reddit.com/r/ObsidianMD/comments/1qpo8yj/smart_connections_plugin_quietly_switches_to_a/
- Smart Connections API/setup complaint: https://www.reddit.com/r/ObsidianMD/comments/1ftx9sv
- Smart Connections disappeared complaint: https://www.reddit.com/r/ObsidianMD/comments/1fcyevp
- Roam Research site: https://roamresearch.com/
- Product Hunt Roam pricing Q&A: https://www.producthunt.com/products/roam-research/questions/is-roam-research-free
- Roam pricing Reddit: https://www.reddit.com/r/RoamResearch/comments/gywjc3
- Roam price complaint: https://www.reddit.com/r/RoamResearch/comments/y314za
- Roam stagnation complaint: https://www.reddit.com/r/RoamResearch/comments/1dybuul
- Logseq GitHub repo: https://github.com/logseq/logseq
- Logseq Sync setup: https://blog.logseq.com/how-to-setup-and-use-logseq-sync/
- Logseq GPT plugin: https://github.com/briansunter/logseq-plugin-gpt3-openai
- Logseq agentic AI forum: https://discuss.logseq.com/t/how-is-logseq-s-official-development-aligning-with-the-emerging-agentic-ai-trend/34823
- Logseq communication complaint: https://www.reddit.com/r/logseq/comments/1pigcs1/so_long_and_thanks_for_all_the_fish/
- Capacities pricing: https://capacities.io/pricing/
- Capacities AI assistant docs: https://docs.capacities.io/reference/ai-assistant
- Capacities AI product page: https://capacities.io/product/ai
- Tana/Capacities comparison complaint: https://www.reddit.com/r/TanaInc/comments/18pk36j
- Heptabase pricing/AI credits FAQ: https://support.heptabase.com/en/articles/12990121-pro-premium-and-premium-plans-pricing-faq
- Heptabase provider request: https://www.reddit.com/r/heptabase/comments/1on229p/feature_request_add_support_for_openaicompatible/
- Heptabase CLI release thread: https://www.reddit.com/r/heptabase/comments/1ssi2mb/heptabase_v1910_update_cli_for_coding_agents/
- Anytype monetization docs: https://doc.anytype.io/anytype-docs/advanced/monetization
- Anytype roadmap tag: https://community.anytype.io/tag/roadmap/84
- Anytype Business: https://business.anytype.io/
- Anytype community top page: https://community.anytype.io/top
- Apple Writing Tools support: https://support.apple.com/guide/iphone/use-writing-tools-iph6f08da1d2/ios
- Apple Podcasts Writing Tools support: https://podcasters.apple.com/support/5588-writing-tools-shape-podcast-copy
- Apple Education note workflow: https://education.apple.com/resource/250014925
- Apple Notes closed complaint: https://www.reddit.com/r/AppleNotesGang/comments/1nxpvy0/apple_notes_is_too_closed_is_there_a_good_way_to/
- Apple Writing Tools complaint: https://www.reddit.com/r/ios/comments/1hdygoe
- Microsoft Copilot in Loop FAQ: https://support.microsoft.com/en-us/office/frequently-asked-questions-about-copilot-in-loop-cdba1c99-3a3f-4f0d-bb1b-8ca62d0bb23d
- Microsoft Loop / Copilot Pages / Copilot Notebooks comparison: https://support.microsoft.com/en-us/topic/compare-microsoft-loop-copilot-pages-and-copilot-notebooks-d6bf4dc0-7b09-4f54-9a92-ff898b8397f7
- Microsoft Loop recap complaint: https://www.reddit.com/r/Office365/comments/1sa8g76/wtf_microsoft_loop_retirement_of_copilotgenerated/
- Microsoft Loop/Copilot direction complaint: https://www.reddit.com/r/microsoft/comments/1ri1pu1/question_regarding_loop/
- Mem0: https://mem0.ai/
- Saga pricing: https://www.saga.so/pricing
- Saga AI: https://www.saga.so/ai
- NotePlan pricing: https://noteplan.co/pricing
- NotePlan AI help URL attempted: https://help.noteplan.co/article/230-ai-in-noteplan
- NotePlan Memo AI thread: https://www.reddit.com/r/noteplanapp/comments/1sr52ns/memo_ai_is_live_on_the_app_store/
- NotePlan pricing complaint: https://www.reddit.com/r/noteplanapp/comments/wqvh54
- Amplenote subscriptions: https://www.amplenote.com/subscriptions/new
- Amplenote pricing help: https://www.amplenote.com/help/plans_and_prices_billing
- Ample Agent Pro: https://www.amplenote.com/plugins/ample_agent_pro
