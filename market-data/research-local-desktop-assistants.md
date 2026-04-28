# Market Research: Local Desktop / IDE Agent Assistants

Evidence checked April 28, 2026. Scope is local desktop apps, IDE plugins, terminal CLIs, and hybrids that compete for developer desktop install, local workflow control, and agent attention. Cloud-only features are flagged as cross-lane rather than treated as direct local-agent competition.

## Per-Product Findings

### P-1 — Cline
**Vendor / docs / repo:** https://cline.bot/; https://docs.cline.bot/home; https://github.com/cline/cline

**Deployment model:** IDE plugin plus terminal CLI; vendor page also says JetBrains integration is available. Primary center of gravity is VS Code extension.

**Capability surface:** Cline positions itself as an open-source coding agent with Plan/Act modes, MCP, terminal-first workflows, file read/write, command execution, browser use, checkpoints, Memory Bank, Focus Chain, auto-approve, auto-compact, multi-root workspaces, subagents, and background edit. The Plan/Act docs define Plan mode as read/search/strategy without file modification or command execution, and Act mode as file modification, command execution, and plan execution while retaining planning context. Sources: https://cline.bot/; https://docs.cline.bot/core-workflows/plan-and-act; https://docs.cline.bot/features/memory-bank

**Local-vs-cloud split:** Cline runs locally in the developer environment but calls configured model providers unless using local models. The authorization docs support Cline account login, BYO keys for Anthropic, OpenAI, OpenRouter and other providers, and local models through Ollama or LM Studio “for complete privacy and zero per-request costs.” Source: https://docs.cline.bot/getting-started/authorizing-with-cline

**Multi-task / multi-session model:** Cline’s current vendor page claims a “new UI to orchestrate agents in Cline, Claude, and Codex”; docs also list subagents and background edit. This starts to overlap with multi-agent orchestration, but the documented local surface is still IDE/CLI-bound rather than a long-lived cross-CLI harness. Source: https://cline.bot/

**Memory / context behavior:** Memory Bank is a structured project-documentation pattern with `projectbrief.md`, `productContext.md`, `activeContext.md`, `systemPatterns.md`, `techContext.md`, and `progress.md`, intended to preserve context across sessions. It is file-system memory, not a graph with stable packed node identity. Source: https://docs.cline.bot/features/memory-bank

**Question / approval flow:** Cline’s home docs say it can “read files, write code, run commands, all with your approval.” Plan mode prevents writes/commands; Act mode enables them. Auto-approve can grant fuller autonomy, with CLI docs warning to use it on a clean branch. Sources: https://docs.cline.bot/home; https://docs.cline.bot/core-workflows/plan-and-act; https://docs.cline.bot/cline-cli/interactive-mode

**Pricing tiers:** Cline itself is open source; usage cost depends on provider. BYO key and local-model operation are explicit. The vendor site also offers account sign-up, but no primary pricing page with paid Cline seat tiers was found in this pass. Sources: https://github.com/cline/cline; https://docs.cline.bot/getting-started/authorizing-with-cline

**Customer segments:** General developers and enterprise developers; vendor page cites “5.0M installs across all platforms,” “61.0k stars,” and trust at Samsung, Salesforce, Oracle, Amazon, LG, Globant, Microsoft, eBay, Visa, and IBM. Source: https://cline.bot/

**Documented user complaints:** GitHub issue #8354 alleges task-completion claims without verification and unauthorized changes; short quote: “Production has come to a complete halt.” Source: https://github.com/cline/cline/issues/8354

**Public traction signals:** GitHub API on April 28, 2026: 61,063 stars, 6,297 forks, 734 open issues. Source: https://api.github.com/repos/cline/cline

**Positioning relative to agent-harness's space:** Close desktop/IDE competitor for agent attention, especially because Cline now uses orchestration language. It does not document a persistent cross-CLI graph, imposed working set, per-provider routing across multiple CLI stores, or a long-lived foreground/background optimizer.

**Notes:** Strongest adjacent signal for agent-harness: Cline users already understand Plan/Act, file-based memory, provider choice, and approval controls.

### P-2 — Continue.dev
**Vendor / docs / repo:** https://www.continue.dev/; https://docs.continue.dev/ide-extensions/quick-start; https://github.com/continuedev/continue

**Deployment model:** IDE extension for VS Code and JetBrains plus Continue CLI / cloud checks. Current homepage has shifted toward source-controlled AI checks in GitHub, which is partly cross-lane CI/cloud automation. The IDE extension remains in-scope. Sources: https://docs.continue.dev/ide-extensions/quick-start; https://www.continue.dev/

**Capability surface:** IDE extension docs list Autocomplete, Edit, Chat, and Agent mode. Context providers include file, code, Git diff, current file, terminal output, docs, open files, web, codebase snippets, folders, search, URL, clipboard, tree, problems, debugger locals, repository map, OS, and MCP. Sources: https://docs.continue.dev/ide-extensions/quick-start; https://docs.continue.dev/customize/custom-providers

**Local-vs-cloud split:** IDE extension runs locally in the IDE; models are connected from configured providers. Continue pricing now adds hosted platform/Hub capability, “frontier models,” integrations, private agents, and BYOK in team/company plans. Source: https://www.continue.dev/pricing

**Multi-task / multi-session model:** Local IDE mode is a pane/chat/editor interaction. Continue’s new platform runs agents as PR checks; that is cross-lane cloud/CI rather than local desktop concurrency. Source: https://www.continue.dev/

**Memory / context behavior:** Context is selected on demand through highlighted code, active file, files, codebase search, docs, terminal, Git diff, and custom providers. The docs describe context providers as plugins; persistence is not a long-running memory graph. Sources: https://docs.continue.dev/ide-extensions/chat/context-selection; https://docs.continue.dev/customize/custom-providers

**Question / approval flow:** Edit mode presents a diff and lets users accept or reject individual changes. Source: https://docs.continue.dev/ide-extensions/quick-start

**Pricing tiers:** Current pricing page: Starter at $3 per million tokens; Team at $20/seat/month; Company custom. Team features include private agents and controls; Company includes BYOK, SSO, commitment/invoicing/SLA. Source: https://www.continue.dev/pricing

**Customer segments:** Team and company engineering orgs adopting AI agents/checks. The current public positioning is more team/workflow than solo IDE plugin. Source: https://www.continue.dev/pricing

**Documented user complaints:** GitHub issue #3036 complains of slow context gathering; short quote: “Gathering context takes more than 15 minutes, and I have no idea why.” Source: https://github.com/continuedev/continue/issues/3036

**Public traction signals:** GitHub API on April 28, 2026: 32,841 stars, 4,425 forks, 641 open issues. Source: https://api.github.com/repos/continuedev/continue

**Positioning relative to agent-harness's space:** Continue competes for IDE context-provider muscle memory and BYO model routing, but current public product emphasis has moved toward cloud/CI AI checks. No evidence of a local long-running orchestrator with graph packing/unpacking.

**Notes:** Continue’s context-provider vocabulary is directly relevant prior art for local context surfaces.

### P-3 — Aider
**Vendor / docs / repo:** https://aider.chat/docs/; https://github.com/Aider-AI/aider

**Deployment model:** Terminal CLI.

**Capability surface:** Aider is “AI pair programming in your terminal.” It supports multi-file editing by adding files to chat, repository map context, automatic Git commits, undo, model switching, linting/testing, voice commands, watching files in an IDE/editor, over 100 languages, local and cloud LLMs. Sources: https://aider.chat/docs/; https://aider.chat/docs/usage.html; https://github.com/Aider-AI/aider

**Local-vs-cloud split:** CLI runs locally against the local Git repo. Model calls go to configured providers unless local LLMs are used. BYO provider/API key is the normal cost path. Sources: https://aider.chat/docs/; https://github.com/Aider-AI/aider

**Multi-task / multi-session model:** Aider is session-oriented in the terminal; separate terminal processes can run separately, but no vendor-documented orchestrator UI for many simultaneous initiatives was found.

**Memory / context behavior:** Repository map supplies codebase structure and context. State is primarily current chat/session plus Git repo and files added to chat. No persistent cross-session memory comparable to Cline Memory Bank or Claude `CLAUDE.md` was found in the reviewed docs. Source: https://aider.chat/docs/

**Question / approval flow:** Aider applies edits in chat, commits them, and lets users undo. It is more direct edit-and-commit than stepwise permission prompting. Source: https://aider.chat/docs/usage.html

**Pricing tiers:** Free and open source, Apache-2.0; users pay model-provider token costs or use local models. Source: https://github.com/Aider-AI/aider

**Customer segments:** Individual developers and CLI-heavy teams. No official customer-logo page found in this pass.

**Documented user complaints:** GitHub issue #752 reports repository-map token behavior; short quote: “I noticed very high input token usage when sending queries to claude-3.5-sonnet.” Source: https://github.com/Aider-AI/aider/issues/752

**Public traction signals:** GitHub API on April 28, 2026: 44,022 stars, 4,308 forks, 1,503 open issues. Source: https://api.github.com/repos/aider-ai/aider

**Positioning relative to agent-harness's space:** Strong terminal-local baseline and provider-flexibility precedent. It is not a desktop orchestrator and does not supervise many sub-agents.

**Notes:** Aider’s repo-map/token complaints are useful evidence that local codebase context selection remains hard even without a desktop UI.

### P-4 — JetBrains Junie
**Vendor / docs / repo:** https://www.jetbrains.com/junie/; https://junie.jetbrains.com/docs/; https://www.jetbrains.com/ai-ides/buy/

**Deployment model:** JetBrains IDE-native agent, plus Junie CLI and headless CI/CD scripts.

**Capability surface:** JetBrains describes Junie as an AI coding agent that can propose execution plans, adjust to a task, perform inspections, run tests, and collaborate intelligently. Docs state it runs in JetBrains IDEs, terminal, and headless CI/CD; integrations include GitHub Actions and GitLab CI/CD. Sources: https://www.jetbrains.com/junie/; https://junie.jetbrains.com/docs/

**Local-vs-cloud split:** IDE/CLI surfaces are local, but usage is mediated by JetBrains AI subscriptions and cloud credits. The public docs reviewed did not show local model support for Junie; a YouTrack request explicitly asks for it. Sources: https://www.jetbrains.com/junie/; https://youtrack.jetbrains.com/projects/JUNIE/issues/JUNIE-47/Add-ability-to-use-Junie-locally?backToIssues=false

**Multi-task / multi-session model:** Junie supports IDE, terminal, and CI/headless task execution. Public product docs imply task delegation but do not document a many-initiative local dashboard.

**Memory / context behavior:** No persistent memory feature comparable to Memory Bank or `CLAUDE.md` was found in the reviewed Junie docs.

**Question / approval flow:** Product docs emphasize proposed execution plans and review of results. A precise permission model for local commands/file edits was not extracted from primary docs in this pass.

**Pricing tiers:** Junie page/pricing page list JetBrains AI tiers: AI Pro $100/user/year, AI Ultimate $300/user/year, and higher enterprise-style tier shown at $720/user/year in page extraction. Source: https://www.jetbrains.com/junie/; https://www.jetbrains.com/ai-ides/buy/

**Customer segments:** Individuals and organizations already using JetBrains IDEs.

**Documented user complaints:** YouTrack JUNIE-47 asks for local model support and complains about credits; short quote: “The current credit system makes it feel like a gimmick.” Source: https://youtrack.jetbrains.com/projects/JUNIE/issues/JUNIE-47/Add-ability-to-use-Junie-locally?backToIssues=false

**Public traction signals:** No public GitHub star metric because Junie is proprietary. Public signal is JetBrains bundling and AI plan pricing.

**Positioning relative to agent-harness's space:** Competes inside JetBrains IDEs and terminal/headless workflows. Less direct for cross-CLI orchestration because provider routing and memory are vendor-controlled.

**Notes:** Key signal: proprietary IDE vendors are adding terminal/headless modes, narrowing the distinction between IDE assistant and local agent runner.

### P-5 — Anthropic Claude Desktop
**Vendor / docs / repo:** https://claude.ai/download; https://claude.com/pricing; https://www.anthropic.com/engineering/desktop-extensions

**Deployment model:** Desktop app for macOS, Windows, Windows arm64, ChromeOS, plus mobile and browser.

**Capability surface:** Claude Desktop provides local app access to Claude chat, local files, Desktop Extensions, Claude Code Desktop integration, and enterprise deployment. Desktop Extensions package local MCP servers as zip archives with a `manifest.json`, can interact with local applications and private data, and store sensitive config in the OS keychain. Sources: https://claude.ai/download; https://www.anthropic.com/engineering/desktop-extensions

**Local-vs-cloud split:** Claude app runs locally, but reasoning is cloud Claude. MCP servers/extensions can run locally and connect local/private data to the cloud model through Claude Desktop. Source: https://www.anthropic.com/engineering/desktop-extensions

**Multi-task / multi-session model:** Claude Desktop is chat/workspace oriented, not a documented multi-initiative orchestrator. Claude Code Desktop and computer-use features overlap with operational desktop assistance but are still Anthropic-client centric. Source: https://code.claude.com/docs/en/desktop

**Memory / context behavior:** Claude Desktop itself is outside the coding-specific memory docs; Claude Code adds `CLAUDE.md` and auto memory. Desktop Extensions are integration packaging, not memory. Sources: https://www.anthropic.com/engineering/desktop-extensions; https://code.claude.com/docs/en/memory

**Question / approval flow:** Desktop Extensions include enterprise controls: approved/private extension directories, block lists, Group Policy/MDM. The specific user prompt/approval flow for each extension depends on MCP/tool behavior. Source: https://www.anthropic.com/engineering/desktop-extensions

**Pricing tiers:** Claude pricing page lists Free, Pro, Max, Team, Enterprise. Scrape showed Pro from $17, Max from $100, Team $20/seat plus usage at API rates, and Enterprise contact/sales-style access. Source: https://claude.com/pricing

**Customer segments:** General knowledge workers, developers using Claude Code/Desktop, and enterprises deploying Desktop Extensions via MDM.

**Documented user complaints:** Anthropic’s public `claude-code` tracker has desktop-adjacent issues. Issue #18866 says auto-compact on Claude.ai web/desktop was not triggering; short quote: “Auto-compact doesn't work.” Source: https://github.com/anthropics/claude-code/issues/18866

**Public traction signals:** No public app install count found. Desktop Extensions engineering post indicates a strategic platform direction. Source: https://www.anthropic.com/engineering/desktop-extensions

**Positioning relative to agent-harness's space:** Competes for desktop install and MCP integration attention. It is not a cross-provider orchestrator; it routes to Anthropic’s cloud model and Anthropic client UX.

**Notes:** Cross-lane overlap: Claude Desktop is partly general assistant/KM, not purely coding. Keep it in scope only where it touches local desktop tools, MCP, Claude Code, and computer use.

### P-6 — Cursor
**Vendor / docs / repo:** https://cursor.com/product; https://cursor.com/pricing; https://cursor.com/customers

**Deployment model:** Desktop IDE.

**Capability surface:** Cursor ships an IDE with Agent, background agents, multi-file edits, terminal/context integration, memories/rules, MCPs, skills, hooks, cloud agents, tab completions, and model routing. Pricing page explicitly includes MCPs, skills, hooks, cloud agents, usage analytics, privacy controls, RBAC, SSO, SCIM, audit logs, and model controls in paid tiers. Sources: https://cursor.com/product; https://cursor.com/pricing

**Local-vs-cloud split:** IDE is local desktop; model inference and cloud/background agents are vendor-hosted. BYO key was not found on current pricing extraction.

**Multi-task / multi-session model:** Background agents and cloud agents are first-class in product/pricing; this is a direct concurrency signal but partially cloud-lane. Source: https://cursor.com/pricing

**Memory / context behavior:** Product extraction indicates custom embedding for codebase recall, memories/rules, MCP/hooks. Source: https://cursor.com/product

**Question / approval flow:** Agent interactions are IDE-integrated. Pricing and product pages do not expose a detailed permission protocol; background-agent cost complaints imply users want more guardrails.

**Pricing tiers:** Hobby free; Pro $20/month; Pro+ $60/month; Ultra $200/month; Teams $40/user/month; Enterprise custom. Bugbot has separate Pro/Teams/Enterprise pricing at $40/user/month or custom. Source: https://cursor.com/pricing

**Customer segments:** “Modern engineering organizations.” Customer page names Stripe, Brex, OpenAI, Sentry, Coinbase, Datadog, eBay, Rippling, Mercado Libre, and others. Claims include “over 800 engineers,” “50% more code,” and multiple adoption rates. Source: https://cursor.com/customers

**Documented user complaints:** Official Cursor forum thread complains of background-agent usage burn; short quote: “One prompt = 50 background model calls = total plan drained instantly.” Source: https://forum.cursor.com/t/pro-plan-burned-in-10-minutes-by-background-agent-calls-completely-unacceptable/118368?page=2

**Public traction signals:** Proprietary; no GitHub stars. Public traction comes from customer logos/case studies and explicit adoption claims. Source: https://cursor.com/customers

**Positioning relative to agent-harness's space:** Strongest commercial desktop IDE competitor for agent attention. It offers background/cloud agents but does not expose cross-CLI provider orchestration or user-owned graph memory.

**Notes:** Cost guardrails are a visible pain point in official forum evidence.

### P-7 — Windsurf
**Vendor / docs / repo:** https://windsurf.com/; https://windsurf.com/pricing; https://docs.windsurf.com/windsurf/cascade/cascade

**Deployment model:** Desktop IDE-style editor with Cascade agent; also inherited Codeium plugin/enterprise footprint.

**Capability surface:** Cascade supports Code/Chat modes, web search, memories and rules, MCP, terminal, workflows, app deploys, tool calling, voice input, checkpoints, linter integration, real-time awareness, and multiple simultaneous Cascades. Sources: https://docs.windsurf.com/windsurf/cascade/cascade; https://docs.windsurf.com/windsurf/cascade/memories

**Local-vs-cloud split:** Editor runs locally; model usage and quotas are through Windsurf accounts/plans. BYO key was not extracted from current pricing page. Source: https://windsurf.com/pricing

**Multi-task / multi-session model:** Docs state users can run multiple Cascades simultaneously and navigate between them. Source: https://docs.windsurf.com/windsurf/cascade/cascade

**Memory / context behavior:** Cascade includes Memories & Rules and a planning/todo surface. Docs call Memories a system for sharing and persisting context across conversations. Sources: https://docs.windsurf.com/windsurf/cascade/cascade; https://docs.windsurf.com/windsurf/cascade/memories

**Question / approval flow:** Cascade has checkpoints and queued messages. Detailed permission protocol was not extracted.

**Pricing tiers:** Free $0; Pro $20; Max $200; Teams $40/user; Enterprise “Let’s talk.” Usage buckets extracted as Light, Standard, Heavy, Standard, Unlimited. Source: https://windsurf.com/pricing

**Customer segments:** Individual developers plus enterprises. Official blog case-study list names athenahealth, Mercado Libre, DRW, JPMorgan Chase, Zillow, World Wide Technology, Clearwater Analytics, and Anduril. Source: https://windsurf.com/blog?tag=case+studies

**Documented user complaints:** Official troubleshooting docs list common issues: rate limiting, macOS damaged-app warnings, Windows update issues, Linux launch crashes, blank Cascade panel, stuck terminal session. Short quote: “We are actively working on getting these limits increased.” Source: https://docs.windsurf.com/troubleshooting/windsurf-common-issues

**Public traction signals:** Proprietary; no main repo stars. Official blog claims enterprise customer adoption via case studies. Source: https://windsurf.com/blog?tag=case+studies

**Positioning relative to agent-harness's space:** Direct desktop/IDE competitor with multi-Cascade local UI. Less direct on cross-provider CLI orchestration and user-owned graph context.

**Notes:** Real-time awareness and multiple Cascades are important competitive patterns.

### P-8 — Zed
**Vendor / docs / repo:** https://zed.dev/ai; https://zed.dev/pricing; https://github.com/zed-industries/zed

**Deployment model:** Desktop code editor, open source, with hosted AI and BYO/local provider support.

**Capability surface:** Zed AI supports agentic editing, reviewing editable unified diffs, collaboration/following agents in code navigation, hosted models, BYO keys, and “any agent, any tool” through Agent Client Protocol (ACP). Sources: https://zed.dev/ai; https://zed.dev/blog/bring-your-own-agent-to-zed

**Local-vs-cloud split:** Editor runs locally. Hosted Zed models are billed through Zed; BYO keys/local provider options are supported for personal use. Sources: https://zed.dev/ai; https://zed.dev/pricing

**Multi-task / multi-session model:** Zed has agent threads/panels and collaboration primitives; public product copy emphasizes multiple agents/ACP more than a long-running workstream dashboard. Source: https://zed.dev/ai

**Memory / context behavior:** Agents build context as they work; Zed docs/issues show active work around context compression and summaries, but no durable graph memory. Source: https://github.com/zed-industries/zed/issues/35019

**Question / approval flow:** Editable unified diff is the primary review/refine surface. Source: https://zed.dev/ai

**Pricing tiers:** Personal $0 forever; Pro $10/month with $5 token credits and unlimited accepted edit predictions; Enterprise contact. Pricing page also says Personal allows BYO keys. Source: https://zed.dev/pricing

**Customer segments:** Developers who want a fast editor, collaboration, and AI agent workflows. No official customer-logo page found.

**Documented user complaints:** GitHub issue #29401 reports agent/tool protocol errors; short quote: “Error interacting with language model an error occurred while interacting with the Anthropic API.” Source: https://github.com/zed-industries/zed/issues/29401

**Public traction signals:** GitHub API on April 28, 2026: 79,902 stars, 8,023 forks, 2,830 open issues. Source: https://api.github.com/repos/zed-industries/zed

**Positioning relative to agent-harness's space:** Strong local editor competitor and interesting ACP precedent. It exposes agent interoperability, but not a desktop orchestrator over Claude/Codex/opencode CLIs.

**Notes:** Zed is the most explicit editor-level evidence that “bring your own agent” is becoming a product surface.

### P-9 — Goose Desktop / CLI
**Vendor / docs / repo:** https://goose-docs.ai/; https://github.com/aaif-goose/goose

**Deployment model:** Desktop app, CLI, and API.

**Capability surface:** Goose is a native open-source AI agent for code, workflows, research, writing, automation, and data analysis. It supports desktop app, full CLI, embeddable API, 70+ extensions via MCP, 15+ providers, portable YAML workflows, interactive extension UIs, independent subagents, prompt-injection detection, and sandbox mode. Sources: https://goose-docs.ai/; https://goose-docs.ai/docs/getting-started/installation

**Local-vs-cloud split:** Goose runs on the user’s machine and connects to providers. Provider docs list Anthropic, OpenAI, Gemini, GitHub Copilot, Bedrock, Vertex, OpenRouter, Ollama, LM Studio, Docker Model Runner, Ramalama, LiteLLM, and others. Local models are supported through Ollama/LM Studio/Docker Model Runner/Ramalama. Source: https://goose-docs.ai/docs/getting-started/providers

**Multi-task / multi-session model:** Vendor page explicitly says Goose can spawn independent subagents for parallel task handling. Source: https://goose-docs.ai/

**Memory / context behavior:** Goose’s main persistence pattern in reviewed docs is workflows/recipes/extensions and session state, not a stable graph memory.

**Question / approval flow:** Sandbox mode and extension/tool boundaries are documented; detailed user approval UX was not extracted.

**Pricing tiers:** Open-source product; pricing depends on provider keys/subscriptions. Docs mention provider-specific auth and pricing variation, with some free tiers and subscription fees. Source: https://goose-docs.ai/docs/getting-started/providers

**Customer segments:** General-purpose local agent users, developers, and automation/data-analysis users; now under Agentic AI Foundation positioning. Source: https://goose-docs.ai/

**Documented user complaints:** GitHub issue #6607 reports Copilot auth trouble in WSL; short quote: “Failed to authenticate: Execution error: Failed to save token.” Source: https://github.com/aaif-goose/goose/issues/6607

**Public traction signals:** GitHub API on April 28, 2026: 43,403 stars, 4,424 forks, 380 open issues. Source: https://api.github.com/repos/aaif-goose/goose

**Positioning relative to agent-harness's space:** Very close on local desktop/CLI agent plus subagents and MCP. It lacks documented persistent graph working-set orchestration across multiple external CLIs.

**Notes:** Goose is one of the strongest proof points that local users value broad provider and extension choice.

### P-10 — OpenCode
**Vendor / docs / repo:** https://opencode.ai/; https://opencode.ai/docs/; https://github.com/sst/opencode

**Deployment model:** Terminal TUI, desktop app beta, IDE extension.

**Capability surface:** OpenCode positions itself as an open-source AI coding agent for terminal, IDE, or desktop. Product page lists LSP-enabled context, multi-session, share links, GitHub Copilot and ChatGPT Plus/Pro integration, and 75+ LLM providers through Models.dev. Sources: https://opencode.ai/; https://opencode.ai/docs/

**Local-vs-cloud split:** Client runs locally; model calls route to connected accounts/providers. Product page says free models are included and users can connect Claude, GPT, Gemini, and others. Source: https://opencode.ai/

**Multi-task / multi-session model:** Multi-session is a first-class claim. Source: https://opencode.ai/

**Memory / context behavior:** Docs extraction found shareable conversations and session context; no durable memory graph.

**Question / approval flow:** Docs describe Plan mode and Build mode toggled with Tab; approval specifics beyond mode separation were not extracted. Source: https://opencode.ai/docs/

**Pricing tiers:** Open-source plus free models included; otherwise connect any provider/account. Source: https://opencode.ai/

**Customer segments:** Developers who want provider-agnostic terminal/desktop/IDE agent workflows. No official customer cases found.

**Documented user complaints:** GitHub issue #22883 reports long-session crashes; short quote: “OpenCode crashes when running for extended periods.” Source: https://github.com/anomalyco/opencode/issues/22883

**Public traction signals:** GitHub redirect/API for `sst/opencode` resolves to `anomalyco/opencode`; API on April 28, 2026: 150,652 stars, 17,314 forks, 6,169 open issues. Source: https://api.github.com/repos/sst/opencode

**Positioning relative to agent-harness's space:** Very close in terminal/local agent attention, with multi-session and provider-agnostic positioning. It does not document a single desktop orchestrator supervising Claude/Codex/opencode as interchangeable CLI workers.

**Notes:** High public traction plus long-session crash complaints are directly relevant to long-running local-agent reliability.

### P-11 — Claude Code CLI
**Vendor / docs / repo:** https://code.claude.com/docs/en/overview; https://code.claude.com/docs/en/costs; https://github.com/anthropics/claude-code

**Deployment model:** Terminal CLI, IDE integration, desktop/browser continuity.

**Capability surface:** Claude Code reads codebases, edits files, runs commands, automates reviews, creates commits/PRs, runs multiple sessions side by side, schedules recurring tasks, integrates with GitHub Actions/GitLab CI/CD, supports MCP, and can operate through terminal, IDE, desktop app, and browser. Sources: https://code.claude.com/docs/en/overview; https://code.claude.com/docs/en/desktop

**Local-vs-cloud split:** CLI runs locally but calls Anthropic Claude unless mediated through subscription/cloud experiences. Requires Claude subscription or Anthropic Console/API account. Source: https://code.claude.com/docs/en/overview

**Multi-task / multi-session model:** Docs explicitly mention running multiple sessions side by side and continuity across platforms. Source: https://code.claude.com/docs/en/overview

**Memory / context behavior:** Claude Code uses `CLAUDE.md`, `AGENTS.md`, `MEMORY.md`, hierarchy of managed policy/project/user/local instructions, `/init`, `/memory`, `/compact`, auto memory, first 200 lines of `MEMORY.md`, and import depth limit of 5. Source: https://code.claude.com/docs/en/memory

**Question / approval flow:** Claude Code has permissions and administrative controls; cost docs mention spend limits and usage tracking. Permission bug issues show this is a live surface. Sources: https://code.claude.com/docs/en/overview; https://github.com/anthropics/claude-code/issues/36959

**Pricing tiers:** Costs page says Claude Code charges by API token consumption for API users, while Pro/Max subscribers have usage included. It gives average costs of about $13/developer/active day and $150-250/developer/month, with enterprise average below $30/active day for 90% of users. Source: https://code.claude.com/docs/en/costs

**Customer segments:** Individual developers, teams, enterprise engineering orgs using Console/admin controls.

**Documented user complaints:** GitHub issue #47145 complains about auto-compaction; short quote: “this is a data loss event with no recovery path except manual transcript search.” Source: https://github.com/anthropics/claude-code/issues/47145

**Public traction signals:** GitHub API on April 28, 2026: 118,483 stars, 19,696 forks, 10,660 open issues. Source: https://api.github.com/repos/anthropics/claude-code

**Positioning relative to agent-harness's space:** Direct local CLI competitor and also a likely substrate for agent-harness. It has strong local memory primitives but no user-owned cross-provider graph optimizer.

**Notes:** Claude Code’s memory limits and auto-compact complaints are the sharpest primary-source evidence for agent-harness’s context-discipline problem space.

### P-12 — OpenAI Codex CLI
**Vendor / docs / repo:** https://developers.openai.com/codex/cli; https://github.com/openai/codex

**Deployment model:** Terminal CLI with IDE extension support and cloud task integration.

**Capability surface:** OpenAI docs list interactive code running, image inputs/generation, web search, local code review, subagents, Codex Cloud tasks, scripting with `exec`, MCP, approval modes, GitHub/Slack/Linear integrations, and install via `npm install -g @openai/codex`. Sources: https://developers.openai.com/codex/cli; https://github.com/openai/codex

**Local-vs-cloud split:** CLI runs locally; Codex model inference is OpenAI-hosted. Codex Cloud tasks are explicitly cloud. Source: https://developers.openai.com/codex/cli

**Multi-task / multi-session model:** Supports subagents and cloud tasks; multi-session details are less explicit than Claude Code/OpenCode. Source: https://developers.openai.com/codex/cli

**Memory / context behavior:** Repo extraction indicated session context; no durable memory-file hierarchy comparable to Claude Code was extracted in this pass.

**Question / approval flow:** Docs and repo mention sandbox/approval modes. Source: https://developers.openai.com/codex/cli

**Pricing tiers:** Codex CLI access is included in ChatGPT Plus, Pro, Business, Edu, and Enterprise; ChatGPT pricing page also shows limited Codex on Free, more on Plus $20/month, Pro from $100/month, Business $20/user/month, Enterprise contact. API pricing page lists `gpt-5.3-codex` at $1.75/input 1M tokens, $0.175 cached input, $14 output; priority doubles those prices. Sources: https://developers.openai.com/codex/cli; https://chatgpt.com/pricing/; https://developers.openai.com/api/docs/pricing

**Customer segments:** ChatGPT subscribers, developers, business/enterprise orgs, and teams integrating with GitHub/Slack/Linear.

**Documented user complaints:** GitHub issue #5041 reports network/sandbox failure in the VS Code extension; short quote: “No network task succeeds under these conditions.” Source: https://github.com/openai/codex/issues/5041

**Public traction signals:** GitHub API on April 28, 2026: 78,298 stars, 11,182 forks, 3,273 open issues. Source: https://api.github.com/repos/openai/codex

**Positioning relative to agent-harness's space:** Direct CLI competitor and likely worker substrate. Its subagents/cloud tasks overlap with orchestration, but within OpenAI’s model/client environment.

**Notes:** Pricing now blends subscription access and API metering; this mirrors Claude Code and complicates per-agent cost predictability.

### P-13 — Anthropic Computer Use Desktop Scaffolding
**Vendor / docs / repo:** https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool; https://github.com/anthropics/claude-quickstarts/tree/main/computer-use-demo

**Deployment model:** Reference implementation / Docker scaffold, not a polished desktop assistant.

**Capability surface:** Computer use is a beta feature enabling screenshot capture, mouse control, keyboard input, and desktop automation. The reference implementation includes web interface, Docker container, example tools, Streamlit app, and an agent loop using Claude API, Bedrock, or Vertex. Sources: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool; https://github.com/anthropics/claude-quickstarts/tree/main/computer-use-demo

**Local-vs-cloud split:** Client-side tool and Docker desktop environment run locally; model calls go to Claude API/Bedrock/Vertex. Source: https://github.com/anthropics/claude-quickstarts/tree/main/computer-use-demo

**Multi-task / multi-session model:** Demo loop is single-environment focused; no multi-initiative orchestration.

**Memory / context behavior:** No durable memory layer in reviewed docs; it is a tool-use scaffold.

**Question / approval flow:** Anthropic docs emphasize safety precautions: use dedicated VMs, avoid sensitive data, and get user consent before enabling computer-use features. Source: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool

**Pricing tiers:** Standard tool-use/token pricing applies; API key required for Claude Console unless using Bedrock/Vertex provider paths. Sources: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool; https://github.com/anthropics/claude-quickstarts/tree/main/computer-use-demo

**Customer segments:** Developers building desktop automation agents and researchers testing computer-use capabilities.

**Documented user complaints:** GitHub issue #147 reports a macOS port conflict in the demo; short quote: “I have to disable screen-sharing on macOS to free this port.” Source: https://github.com/anthropics/claude-quickstarts/issues/147

**Public traction signals:** `anthropics/claude-quickstarts` GitHub API on April 28, 2026: 16,342 stars, 2,778 forks, 167 open issues. Source: https://api.github.com/repos/anthropics/claude-quickstarts

**Positioning relative to agent-harness's space:** Adjacent, not direct. It competes for “agent controls desktop” imagination, but is a beta scaffold, not a developer workstream orchestrator.

**Notes:** Cross-lane risk: computer-use scaffolding is a capability primitive, not a product segment by itself.

## Cross-Product Patterns

**Pricing patterns for local/desktop/IDE agent products:** Four patterns dominate. First, OSS/BYO-key tools (Aider, Cline, Goose, OpenCode) charge no product seat fee and shift spend to provider tokens or local-model hardware. Second, proprietary IDEs (Cursor, Windsurf, Zed, JetBrains) charge seat/month plans plus usage credits, quotas, or hosted model pools. Third, vendor CLIs (Claude Code, Codex CLI) blend subscription entitlements with API-token billing. Fourth, enterprise pricing adds SSO, RBAC, audit logs, privacy controls, centralized billing, admin model controls, and support.

**Local-vs-cloud split patterns:** Nearly every product is “local shell, cloud brain” by default. Local execution covers IDE/editor/terminal UI, file access, command execution, diff review, sandboxing, and extension/MCP adapters. Cloud covers frontier-model inference, hosted indexing/embeddings in some products, background agents, and billing/quota enforcement. Fully local operation exists mainly through Ollama/LM Studio/other local providers in OSS products or BYO-key editors.

**Memory/context patterns specific to local install:** The common local memory primitives are flat files (`CLAUDE.md`, `MEMORY.md`, Cline Memory Bank), rules files, memories/rules, codebase indexes, repository maps, context providers, MCP servers, and session summaries. The products generally let models consult or accumulate context; none documents imposed graph-derived working sets with stable node identity, bounded-depth packing, and concurrent background optimizer mutation.

**What every product in this segment offers:** Local code/file awareness, chat/agent interaction, multi-file edits or diff proposals, command/tool execution or terminal integration, model-provider routing or hosted model choice, and some approval/review concept. Nearly all now mention MCP, agent modes, memories/rules, or background/subagent execution.

**What no product offers:** No surveyed primary source documents a user-owned desktop Tauri orchestrator that supervises multiple external CLIs as interchangeable workers, maintains a persistent context graph with pack/unpack tools, imposes a bounded working set each turn, continuously optimizes that graph in the background, and preserves cross-CLI tool-call provenance as first-class graph state.

**Customer-segment patterns:** Cursor, Windsurf, JetBrains, and Zed target professional developers and engineering orgs. Cursor and Windsurf publish enterprise/logo case studies. OSS CLI tools target individual power users and developer teams that value provider control. Claude Desktop broadens to knowledge workers but matters here through MCP/Desktop Extensions and Claude Code integration.

## Cross-Lane Signals

Cloud-only or primarily cloud features should be routed to other lanes: Cursor Cloud Agents, Continue PR checks, Codex Cloud tasks, Claude Code web/browser tasks, JetBrains headless CI, and Windsurf app deploys. Agent SDKs/frameworks, dedicated memory products, knowledge-management tools, and pricing-broadly research are cross-lane and not analyzed here beyond how they affect local desktop/IDE products.

## Open Evidence Gaps

- Continue.dev has visibly shifted public homepage/pricing toward CI AI checks; older IDE-extension positioning remains in docs, but current product segmentation needs a separate Continue-specific validation pass if this lane depends heavily on it.
- Claude Desktop complaints are sparse in primary sources; the best available public official tracker evidence is desktop-adjacent through `anthropics/claude-code`.
- Windsurf user complaints from official community/forum sources were not found; official troubleshooting docs were used instead.
- Proprietary products do not publish install counts or open issue trackers consistently, making traction less comparable than GitHub-starred OSS tools.
- Several pricing pages use annual/monthly toggles and usage pools; exact monthly effective prices should be rechecked immediately before roadmap synthesis if pricing is used quantitatively.

## Citations

- Aider docs: https://aider.chat/docs/
- Aider usage: https://aider.chat/docs/usage.html
- Aider repo: https://github.com/Aider-AI/aider
- Aider issue #752: https://github.com/Aider-AI/aider/issues/752
- Aider GitHub API: https://api.github.com/repos/aider-ai/aider
- Anthropic Claude pricing: https://claude.com/pricing
- Anthropic Computer Use docs: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool
- Anthropic Desktop Extensions: https://www.anthropic.com/engineering/desktop-extensions
- Anthropic quickstarts computer-use demo: https://github.com/anthropics/claude-quickstarts/tree/main/computer-use-demo
- Anthropic quickstarts issue #147: https://github.com/anthropics/claude-quickstarts/issues/147
- Anthropic quickstarts GitHub API: https://api.github.com/repos/anthropics/claude-quickstarts
- Claude Code overview: https://code.claude.com/docs/en/overview
- Claude Code desktop: https://code.claude.com/docs/en/desktop
- Claude Code costs: https://code.claude.com/docs/en/costs
- Claude Code memory: https://code.claude.com/docs/en/memory
- Claude Code repo: https://github.com/anthropics/claude-code
- Claude Code issue #18866: https://github.com/anthropics/claude-code/issues/18866
- Claude Code issue #36959: https://github.com/anthropics/claude-code/issues/36959
- Claude Code issue #47145: https://github.com/anthropics/claude-code/issues/47145
- Claude Code GitHub API: https://api.github.com/repos/anthropics/claude-code
- Claude download: https://claude.ai/download
- Cline site: https://cline.bot/
- Cline docs home: https://docs.cline.bot/home
- Cline Plan & Act: https://docs.cline.bot/core-workflows/plan-and-act
- Cline Memory Bank: https://docs.cline.bot/features/memory-bank
- Cline authorization: https://docs.cline.bot/getting-started/authorizing-with-cline
- Cline CLI interactive mode: https://docs.cline.bot/cline-cli/interactive-mode
- Cline repo: https://github.com/cline/cline
- Cline issue #8354: https://github.com/cline/cline/issues/8354
- Cline GitHub API: https://api.github.com/repos/cline/cline
- Codex CLI docs: https://developers.openai.com/codex/cli
- Codex repo: https://github.com/openai/codex
- Codex issue #5041: https://github.com/openai/codex/issues/5041
- Codex GitHub API: https://api.github.com/repos/openai/codex
- Continue site: https://www.continue.dev/
- Continue pricing: https://www.continue.dev/pricing
- Continue IDE quick start: https://docs.continue.dev/ide-extensions/quick-start
- Continue context selection: https://docs.continue.dev/ide-extensions/chat/context-selection
- Continue context providers: https://docs.continue.dev/customize/custom-providers
- Continue repo: https://github.com/continuedev/continue
- Continue issue #3036: https://github.com/continuedev/continue/issues/3036
- Continue GitHub API: https://api.github.com/repos/continuedev/continue
- Cursor product: https://cursor.com/product
- Cursor pricing: https://cursor.com/pricing
- Cursor customers: https://cursor.com/customers
- Cursor forum complaint: https://forum.cursor.com/t/pro-plan-burned-in-10-minutes-by-background-agent-calls-completely-unacceptable/118368?page=2
- Goose docs: https://goose-docs.ai/
- Goose install: https://goose-docs.ai/docs/getting-started/installation
- Goose providers: https://goose-docs.ai/docs/getting-started/providers
- Goose repo: https://github.com/aaif-goose/goose
- Goose issue #6607: https://github.com/aaif-goose/goose/issues/6607
- Goose GitHub API: https://api.github.com/repos/aaif-goose/goose
- JetBrains AI pricing: https://www.jetbrains.com/ai-ides/buy/
- JetBrains Junie: https://www.jetbrains.com/junie/
- Junie docs: https://junie.jetbrains.com/docs/
- Junie YouTrack JUNIE-47: https://youtrack.jetbrains.com/projects/JUNIE/issues/JUNIE-47/Add-ability-to-use-Junie-locally?backToIssues=false
- OpenAI API pricing: https://developers.openai.com/api/docs/pricing
- OpenAI ChatGPT pricing: https://chatgpt.com/pricing/
- OpenCode site: https://opencode.ai/
- OpenCode docs: https://opencode.ai/docs/
- OpenCode repo redirect: https://github.com/sst/opencode
- OpenCode issue #22883: https://github.com/anomalyco/opencode/issues/22883
- OpenCode GitHub API: https://api.github.com/repos/sst/opencode
- Windsurf pricing: https://windsurf.com/pricing
- Windsurf Cascade docs: https://docs.windsurf.com/windsurf/cascade/cascade
- Windsurf Memories docs: https://docs.windsurf.com/windsurf/cascade/memories
- Windsurf common issues: https://docs.windsurf.com/troubleshooting/windsurf-common-issues
- Windsurf case-study blog index: https://windsurf.com/blog?tag=case+studies
- Zed AI: https://zed.dev/ai
- Zed pricing: https://zed.dev/pricing
- Zed AI plans/usage: https://zed.dev/docs/ai/plans-and-usage
- Zed models/pricing: https://zed.dev/docs/ai/models
- Zed BYO agent blog: https://zed.dev/blog/bring-your-own-agent-to-zed
- Zed repo: https://github.com/zed-industries/zed
- Zed issue #29401: https://github.com/zed-industries/zed/issues/29401
- Zed issue #35019: https://github.com/zed-industries/zed/issues/35019
- Zed GitHub API: https://api.github.com/repos/zed-industries/zed
