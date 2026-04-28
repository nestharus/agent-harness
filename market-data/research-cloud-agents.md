# Market Research: Long-Running Cloud Agent Products

Stage 0b input for the roadmap synthesis agent. Scope is cloud-hosted or cloud-adjacent long-running agent products that compete for the same user attention/value space as a long-lived agent orchestrator. Primary-source evidence only: vendor product/docs/pricing/blogs, official forums, official GitHub repositories/discussions/issues, and official help centers.

## P-1 — Devin / Cognition Labs

**Vendor / docs:** https://devin.ai/pricing/, https://docs.devin.ai/get-started/devin-intro, https://docs.devin.ai/get-started/first-run, https://docs.devin.ai/work-with-devin/devin-session-tools, https://docs.devin.ai/product-guides/scheduled-sessions, https://cognition.ai/blog/devin-can-now-schedule-devins, https://cognition.ai/blog/devin-can-now-manage-devins, https://devin.ai/customers, https://docs.devin.ai/admin/common-issues  
**Deployment model:** cloud-only. Devin runs in Cognition-hosted workspaces with Shell, Browser, IDE, session logs, repo integrations, API, Slack/Linear/MCP integrations, and persistent account/workspace state.

**Capability surface:** Autonomous software engineering agent; writes, runs, and tests code; browses the web; uses shell/IDE/browser; reviews PRs; answers codebase questions; fixes bugs; writes tests/docs; performs migrations/refactors; can run many tasks in parallel; can be invoked from chat or API; uses @-mentions for repos/files/macros/playbooks/skills/secrets/sessions; supports Ask Mode for planning/Q&A and Agent Mode for code changes.

**Long-running behavior:** Devin’s own docs say a practical task rule is roughly “if you can do it in three hours, Devin can most likely do it,” and recommend breaking larger projects into focused sessions. Scheduled Sessions support one-time or recurring runs, cron-like schedules, playbooks, repos, notifications, run-as user, and active/paused/error status. The March 20, 2026 scheduling announcement says scheduled Devins can carry state between runs by reading/writing notes. Common-issues docs set a hard session-continuation limit: sessions cannot be continued after 30 days.

**Multi-task / multi-initiative model:** Devin represents work as sessions. Managed Devins let a coordinator Devin split a large task into child Devin sessions, each with an isolated VM/terminal/browser/dev environment and a session link. The coordinator scopes tasks, assigns work, monitors progress, resolves conflicts, compiles results, and can message child sessions mid-task.

**Question routing:** The primary question path is conversational chat inside a session. Devin can ask for clarifications in the session, and browser takeover is supported for auth/CAPTCHA/testing. Scheduled sessions can notify by email or Slack.

**Recovery / cancellation surface:** Progress tab logs shell commands, code edits, and browser activity. Users can stop, take over, and resume. Session tools docs advise pausing before takeover and telling Devin what changed when resuming. Managed Devins expose child session links and controls to sleep/terminate child sessions. Past sessions remain visible even after a schedule is deleted.

**Pricing tiers:**

| Tier | Price | Evidence |
|---|---:|---|
| Free | $0 | Limited Devin usage, Devin Review, DeepWiki. |
| Pro | $20/month | Devin usage quota, Windsurf quota, pay-as-you-go past quota, Slack/Linear/MCP integrations. |
| Max | $200/month | Higher Devin and Windsurf quotas. |
| Teams | $80/month | Unlimited team members, shared/collab features, centralized billing, admin analytics. |
| Enterprise | Custom | SAML/OIDC SSO, enterprise admin controls, dedicated account team, custom terms. |

Concurrency: Pro/Max list up to 10 concurrent sessions; Teams/Enterprise list unlimited concurrent sessions.

**Customer segments cited by the vendor:** Enterprise software engineering and transformation work. Named customers/logos/case studies include Nubank, Itaú, RV Tech, FE fundinfo, The Citation Group, Litera, Bilt, Gumroad, Ramp, Linktree, Crossmint, and Hamming. Nubank case study claims 6M+ ETL lines and 100k data-class implementations migrated with 8-12x efficiency and 20x cost savings; task time improved from 40 minutes to 10 minutes after tuning. Other case-study claims include 10,000+ hours saved annually on Angular upgrades, 16x acceleration, and 70% .NET migration time reduction.

**Documented user complaints:** Vendor-documented limitations: GitHub/Slack orgs can only be connected to one Devin account, and “Devin sessions can’t be continued after 30 days.” The intro docs also set scope boundaries around extremely difficult tasks and stale docs.

**Public traction signals:** Public named enterprise customers and quantified case-study outcomes. No primary-source ARR, funding, or headcount numbers used here.

**Positioning relative to agent-harness's space:** Strong overlap with long-running, multi-session orchestration, especially Managed Devins and Scheduled Sessions. Difference: deployment, state, and execution are cloud-hosted and vendor-controlled; the multi-agent orchestration is presented as a product capability, not a local desktop orchestrator over user-owned CLIs.

**Notes:** Devin is the closest direct product evidence for a commercial “very long running multi-initiative” workflow: scheduled recurrence, coordinator/child sessions, session logs, stop/takeover/resume, and concurrent session pricing are all explicit.

## P-2 — Cursor Cloud Agents / Background Agents

**Vendor / docs:** https://cursor.com/docs/cloud-agent, https://cursor.com/docs/cloud-agent/capabilities, https://cursor.com/pricing, https://cursor.com/changelog/0-50, https://forum.cursor.com/t/background-agent-error-for-required-usage-limit-of-10-is-inaccurate-or-misleading/104018, https://forum.cursor.com/t/background-agents-spawned-via-api-cannot-post-pr-comments-or-reviews-despite-correct-github-app-permissions/153207  
**Deployment model:** cloud-only for Cloud Agents; hybrid product overall because Cursor also has local IDE agent mode.

**Capability surface:** Isolated cloud VM per agent; clones GitHub/GitLab repos; creates branches; pushes changes; can run build/test tools; controls a desktop/browser; uses MCP servers; supports hooks from `.cursor/hooks.json`; can be launched from Cursor Web, Desktop, Slack, GitHub comments, Linear, API, and mobile PWA; attaches artifacts such as screenshots/videos/log references to PRs; can auto-fix GitHub Actions failures.

**Long-running behavior:** Cursor says users can run as many cloud agents as they want in parallel and do not need to keep the local machine connected. Changelog 0.50 positioned Background Agents as parallel task execution for fixing nits, investigations, and medium PR drafts. Agents run in isolated remote environments and push branches for handoff.

**Multi-task / multi-initiative model:** Tasks are Cloud Agent entries, visible in Cursor Web/Desktop task surfaces and tied to separate cloud VMs/branches. Concurrency is a core positioning point: “many parallel agents” in the 0.50 changelog and docs.

**Question routing:** Follow-up messages can be sent to an agent. Entry points include Slack/GitHub/Linear mentions, so user communication can be routed through those systems.

**Recovery / cancellation surface:** Docs expose starting/troubleshooting errors, secrets/workspace setup, branch handoff, and artifact links. The changelog says users can view status, send follow-up, and take over work. CI auto-fix can be disabled via settings or PR comments.

**Pricing tiers:** Background Agents are included in paid plans and billed at API pricing for the chosen model in Max Mode. Cursor pricing lists Hobby free with limited agent requests/completions; Pro $20/month with extended Agent, frontier models, MCP/skills/hooks, Cloud Agents; Pro+ $60/month with 3x usage for OpenAI/Claude/Gemini; Ultra $200/month with 20x usage and priority features; Teams $40/user/month with shared rules/chats/commands, analytics, org privacy, RBAC, SAML/OIDC; Enterprise custom with pooled usage, SCIM, audit logs, model/admin controls, support/account management. Pricing FAQ recommends Pro+ for daily agent users and Ultra for agent power users.

**Customer segments cited by the vendor:** Individual developers, teams, enterprises using Cursor IDE and cloud agents; no named customer case-study evidence used from the cited docs/pricing.

**Documented user complaints:** Official forum complaint on spend-limit messaging: a user said they were “risking an additional $210” after already paying $200/month to make Background Agent work; Cursor later said a bug incorrectly set UI spend to $0. Another official forum bug: API-spawned Background Agents could clone, branch, and push, but could not post PR comments/reviews or issue operations because GitHub API permissions were missing; status closed Apr 10, 2026.

**Public traction signals:** Changelog date May 15, 2025 for Background Agent preview. No primary-source ARR/funding/headcount used here.

**Positioning relative to agent-harness's space:** Overlaps on parallel cloud workstreams and per-agent state. Differs by being repo/branch/PR-centric and bundled into a cloud IDE subscription; no evidence of always-on cross-initiative context graph or local multi-CLI orchestration.

**Notes:** Cursor has one of the clearest “task panel plus many parallel agents” surfaces, but billing is tied to model/API usage rather than a clean per-agent seat only.

## P-3 — OpenAI Codex Cloud

**Vendor / docs:** https://openai.com/index/introducing-codex/, https://developers.openai.com/codex/cloud, https://developers.openai.com/codex/app/automations, https://developers.openai.com/codex/pricing, https://help.openai.com/en/articles/20001106-codex-rate-card, https://openai.com/index/codex-flexible-pricing-for-teams/, https://community.openai.com/t/pro-plan-hit-5-hour-limit-twice-in-2h-and-1-5h-and-nearly-exhausted-weekly-cap-in-1-day-after-today-s-update/1364782  
**Deployment model:** cloud-only for Codex Cloud; hybrid product overall because Codex also has CLI/IDE/local modes.

**Capability surface:** Cloud software engineering agent; reads/edits/runs code; runs tests/linters/typecheckers; answers codebase questions; writes features/fixes bugs; opens PRs; integrates with GitHub; can be delegated from IDE and GitHub; supports AGENTS.md guidance; provides terminal/test citations; app automations can run recurring tasks with cron syntax, standalone runs, thread automations, project-scoped automations, and dedicated background worktrees.

**Long-running behavior:** Launch blog says tasks typically complete in 1-30 minutes, run in isolated cloud sandboxes, and can run in parallel. Automations add scheduled recurring behavior and unattended background work. The app automation docs include local project or dedicated background worktree modes and Triage reporting for standalone runs.

**Multi-task / multi-initiative model:** Codex exposes task delegation, task list/progress monitoring, cloud task state, and parallel tasks. Automations add standalone and thread-linked recurring runs. Each cloud task has its own environment and resulting diff/PR pathway.

**Question routing:** Initial launch explicitly said early preview lacked mid-task course correction. Current cloud docs support GitHub mentions (`@codex`) and prompt-based clarification/initiation. Automations report results into Triage rather than a human-in-the-loop live prompt stream.

**Recovery / cancellation surface:** Cloud docs mention progress monitoring, applying diffs locally, and cancellation in the Codex environment. Launch blog says completion includes commits, terminal logs, test outputs, review/request revisions, GitHub PR, or local integration. Automations can archive/clean runs.

**Pricing tiers:** Codex pricing page lists Free $0 for quick tasks; Go $8/month for lightweight tasks; Plus $20/month with Codex web/CLI/IDE/iOS, cloud integrations, code review, Slack, and latest models; Pro from $100/month with 5x or 20x higher usage than Plus, including temporary 2026 promotional multipliers; API key option for CLI/SDK/IDE only, without cloud features; Business pay-as-you-go or seats, larger VMs, SAML SSO/MFA and no training by default; Enterprise/Edu custom/flexible with SCIM, EKM, analytics, domain verification, RBAC, audit logs, usage monitoring, retention/residency.

Usage limits: Plus GPT-5.3-Codex cloud tasks 10-60 per 5h; Pro 5x 50-300; Pro 20x 200-1200. Local messages and cloud tasks share the 5h window. Credits can be purchased after limits. Help rate card states average Codex cost is roughly $100-$200/developer/month and lists token/credit rates for GPT-5.5, GPT-5.4, GPT-5.4-mini, and GPT-5.3-Codex. Business flexible pricing charges tokens/credits without fixed rate limits. The Apr 2, 2026 business pricing post says Business/Enterprise can add Codex-only seats with no fixed monthly costs, and claims >2M builders use Codex weekly and Business/Enterprise Codex users grew 6x since January.

**Customer segments cited by the vendor:** Launch users include Cisco, Temporal, Superhuman, Kodiak. Business pricing post names Notion, Ramp, Braintrust, and Wasmer. Broader OpenAI business audience is professional software teams and enterprise builders.

**Documented user complaints:** Official OpenAI Community thread from Nov 2025: Pro user reported hitting the 5-hour limit twice in 2h/1.5h and nearly exhausting a weekly cap in one day; hung runs consumed limits and the user asked for exact limits and stuck-job handling.

**Public traction signals:** OpenAI claims >2M builders use Codex weekly; >9M paying business users rely on ChatGPT; Business/Enterprise Codex users grew 6x since January 2026.

**Positioning relative to agent-harness's space:** Strong overlap on cloud tasks, parallel execution, recurring automations, and worktrees. Differs by being OpenAI-hosted, GitHub/cloud-task oriented, and priced through ChatGPT/API credits rather than local long-lived orchestration.

**Notes:** Codex is rapidly moving from “1-30 minute task queue” toward recurring cloud automations. The documented weak point for the long-running pattern is clarity around limits and hung task billing.

## P-4 — Replit Agent

**Vendor / docs:** https://docs.replit.com/core-concepts/agent, https://docs.replit.com/core-concepts/agent/task-system, https://docs.replit.com/core-concepts/agent/checkpoints-and-rollbacks, https://docs.replit.com/billing/ai-billing, https://replit.com/pricing, https://replit.discourse.group/t/rollback-disabled-and-suggestion-to-create-a-new-chat/3555  
**Deployment model:** cloud-only inside Replit workspaces/projects.

**Capability surface:** Builds apps/sites/tools from natural language; plans and executes; writes code; tests; deploys; creates design prototypes on Design Canvas; produces multiple outputs in one project; works with docs/files; connects to services such as BigQuery, Linear, Slack, Notion; supports Lite/Economy/Power/Turbo modes; has task system and checkpoints.

**Long-running behavior:** Agent supports “autonomous long builds” on paid plans. Task system runs background tasks in independent threads/conversations. Core allows 1 active background task; Pro allows 10. Tasks can be queued, reviewed, applied, or dismissed.

**Multi-task / multi-initiative model:** Task board columns: Drafts, Active, Ready, Done. Each task runs in an isolated copy; the main version remains untouched until changes are applied. Replit detects dependencies and conflicts and resolves conflicts when applying changes.

**Question routing:** Plan Mode produces ordered task lists for user review/refinement before code. Task review includes work log, test results, and live preview. Follow-up suggestions appear after apply.

**Recovery / cancellation surface:** Checkpoints capture workspace contents, AI conversation context, environment config, Agent memory, and optionally dev database contents. Rollback restores selected state; production DB is not automatic. Rolling forward is possible if future checkpoints exist; new work after rollback creates an alternate branch.

**Pricing tiers:** Starter free: daily Agent credits/integration credits, 1 app, limited intelligence. Core $20/month or $18/month annually: $20 monthly credits, 5 collaborators, unlimited workspaces, autonomous long builds, remove badge. Pro $100/month or $90/month annually: $100 credits, 15 collaborators/50 viewers, strongest models, private deployments, database restore up to 28 days, premium support. Enterprise custom: Pro plus custom seats, SSO/SAML, privacy controls, design systems, warehouse connections, groups, dedicated support, single-tenant, region controls, static outbound IPs, VPC peering. AI billing is effort-based; all Agent interactions, including planning/text guidance, are billable; third-party APIs draw from Replit credits.

**Customer segments cited by the vendor:** App builders, non-specialist builders, teams deploying Replit apps, enterprises needing private deployment/compliance. No named customer case studies used from cited Replit pages.

**Documented user complaints:** Official Replit community thread, Feb 9, 2025: user said rollback was essential and complained that disabled rollback plus new-chat suggestions made the AI forget history. Response later said rollback was back, but new checkpoints still caused “too complex”/new-chat behavior.

**Public traction signals:** No primary-source traction numbers used here.

**Positioning relative to agent-harness's space:** Overlaps on long-running app-build sessions, task boards, isolated task copies, checkpoints, rollback, and background task concurrency. Differs by being a complete cloud IDE/app platform with credit-based AI billing and deployment embedded.

**Notes:** Replit has the clearest vendor-documented checkpoint/rollback model among this set.

## P-5 — GitHub Copilot Cloud Agent / Coding Agent

**Vendor / docs:** https://docs.github.com/copilot/concepts/agents/coding-agent/about-coding-agent, https://docs.github.com/en/copilot/how-tos/use-copilot-agents/cloud-agent/create-a-pr, https://docs.github.com/en/copilot/how-tos/use-copilot-agents/cloud-agent/troubleshoot-cloud-agent, https://github.com/features/copilot/plans, https://docs.github.com/en/copilot/concepts/billing/organizations-and-enterprises, https://github.blog/news-insights/product-news/github-copilot-meet-the-new-coding-agent/, https://github.com/orgs/community/discussions/188531, https://github.com/orgs/community/discussions/189951  
**Deployment model:** cloud-only for cloud agent; product overall is GitHub-hosted with IDE/mobile/CLI entry points.

**Capability surface:** Assign GitHub issues or prompts; research repo; create plan; make code changes on branch; run tests/linters in GitHub Actions-powered dev environment; create PR; iterate from PR comments; fix bugs, add features, write tests/docs, handle tech debt and merge conflicts; customize with instructions, Copilot Memory preview, MCP servers, hooks, skills, and custom agents.

**Long-running behavior:** Agent works in the background and creates a draft PR. Blog positions it for low-to-medium complexity tasks in well-tested codebases. Troubleshooting docs say stuck sessions may time out after an hour. The workflow is branch/PR based rather than open-ended session persistence.

**Multi-task / multi-initiative model:** Work is represented as issue/task to branch to PR. Docs state one branch at a time and exactly one PR per task. Agents panel shows ongoing and past sessions.

**Question routing:** Comments on the PR can trigger iteration; issue comments after initial assignment are not considered. Only open PR comments from write-access users are acted on. GitHub Mobile, CLI, GitHub.com, IDE chat, and integrations can initiate work.

**Recovery / cancellation surface:** If a session gets stuck, docs recommend retrying by unassign/reassign. Workflows are not run automatically; users approve and run them. Branch protection/rulesets may block completion. PRs require human approval before CI/CD workflows run.

**Pricing tiers:** Free $0: 50 agent/chat requests per month and 2,000 completions, but cloud agent not included. Pro $10/user/month: Copilot cloud agent, code review, 300 premium requests, unlimited GPT-5 mini agent/chat, unlimited inline suggestions. Pro+ $39/user/month: 1,500 premium requests, all models including Claude Opus 4.7, Spark. Business $19/user/month; Enterprise $39/user/month. Extra premium requests: $0.04/request. Cloud agent also consumes GitHub Actions minutes and Copilot premium requests.

**Customer segments cited by the vendor:** GitHub developer base and enterprise software teams. Launch blog cites EY and Carvana. It also states GitHub Actions runs >40M daily jobs on weekdays and has >25k marketplace actions.

**Documented user complaints:** GitHub Community discussion Mar 3, 2026: “Repeated Copilot coding agent encountered an internal error” on an open PR; status unanswered. Discussion Mar 17, 2026: user reported a cloud agent session failed after 18+ hours, draft PR showed only the initial plan, with no commits or file changes; status closed.

**Public traction signals:** GitHub Actions scale metrics from launch blog; no Copilot cloud-agent user count used.

**Positioning relative to agent-harness's space:** Overlaps on background autonomous coding. Differs sharply by centering GitHub issues/branches/PRs, one PR per task, and GitHub Actions environments, rather than multi-initiative, persistent local orchestration.

**Notes:** GitHub’s workflow is highly legible to teams because the PR is the unit of review, but that same model limits general multi-initiative session continuity.

## P-6 — Sourcegraph Amp

**Vendor / docs:** https://ampcode.com/, https://ampcode.com/manual, https://sourcegraph.com/pricing  
**Deployment model:** hybrid/local. Amp is a terminal/editor agent with Sourcegraph-hosted billing, workspace/thread sharing, and enterprise controls.

**Capability surface:** Frontier coding agent for terminal/editor; CLI plus IDE integrations; multi-model modes (smart/rush/deep); code editing and shell tools; AGENTS.md instructions; file @-mentions; thread references by URL/ID; thread search by keyword/file/repo/author/date/task; handoff command to draft a new thread; queued messages; permissions; toolboxes; MCP; workspace sharing.

**Long-running behavior:** Amp manual pushes users toward focused threads and starting new threads when error clutter grows. It supports continuing/referenceable threads, thread handoff, and non-interactive `amp -x` execution, but primary docs do not position it as a days-long cloud task runner.

**Multi-task / multi-initiative model:** Threads are the core unit. Manual advises one subtask per thread, warns against mixing unrelated work, and supports searching/archive/reference of threads. Thread visibility can be public, unlisted, workspace-shared, group-shared, or private.

**Question routing:** Interactive CLI/editor chat; queued messages after the current turn; no evidence of cloud mid-task routing comparable to Devin/Cloud Agents.

**Recovery / cancellation surface:** Editing a prior message can revert changes after it; file-level reverts exist; thread handoff drafts a new context. Manual says there is no compaction; users should start clean threads when context gets noisy.

**Pricing tiers:** Usage-based, no subscription/commitment for individuals; minimum credit purchase $5; costs pass through actual LLM/tool usage with zero markup for individuals and non-enterprise workspaces. Enterprise usage is 50% more expensive and includes SSO/directory sync, zero data retention for text inputs, thread visibility controls, entitlements, MCP registry allowlists, managed settings, analytics/data, groups/retention/IP allowlist. Enterprise requires a $1,000 one-time purchase granting $1,000 usage and upgrading workspace. Sourcegraph’s separate Enterprise Search is $49/user/month and is not Amp pricing.

**Customer segments cited by the vendor:** Developers and teams using terminal/editor workflows; enterprise workspaces needing admin/security controls.

**Documented user complaints:** No primary official user complaint found. Vendor-documented limitations: keep threads small/focused; abandon cluttered failed threads; subagents work in isolation and cannot communicate with each other.

**Public traction signals:** Not used; no primary traction evidence found in cited sources.

**Positioning relative to agent-harness's space:** Cross-lane/hybrid. Amp overlaps on thread management, continuity, local CLI/editor agency, and search over past work. It differs by not being a cloud-hosted long-running task product and by charging direct model/tool usage.

**Notes:** Amp is a strong adjacent signal for thread/workspace economics and context hygiene, but not a direct cloud-agent competitor.

## P-7 — Claude Code / Claude Agent SDK / Managed Agents

**Vendor / docs:** https://www.anthropic.com/product/claude-code, https://code.claude.com/docs/en/agent-sdk/overview, https://code.claude.com/docs/en/headless, https://platform.claude.com/docs/en/about-claude/pricing, https://claude.com/pricing, https://github.com/anthropics/claude-code/issues/28489  
**Deployment model:** hybrid/local by default; cloud-runnable through headless CLI, SDK, CI/CD, API, and Managed Agents. Not primarily a hosted consumer cloud task queue like Devin/Codex/Cursor.

**Capability surface:** Reads codebase; changes files; runs tests; delivers committed code; uses shell/git/k8s/CLIs; manages CI failures; supports CLI, Python/TypeScript Agent SDK, built-in tools (Read/Write/Edit/Bash/Monitor/Glob/Grep/WebSearch/WebFetch/AskUserQuestion), hooks, subagents, MCP, permissions, sessions, headless non-interactive `claude -p`, JSON/streaming output, `--continue` and `--resume`.

**Long-running behavior:** Claude Code page says Anthropic engineers manage multiple agents in parallel. Headless docs support continuing the most recent conversation and resuming a specific session ID. Managed Agents pricing adds session runtime billing at $0.08/session-hour while running, indicating API-hosted agent sessions for sustained workloads.

**Multi-task / multi-initiative model:** Sessions and subagents. Agent SDK subagents have parent tool-use IDs for tracking. Sessions can be resumed with session IDs. Claude Team/Enterprise also has Projects and shared context surfaces, but that is broader Claude collaboration rather than coding-agent task orchestration.

**Question routing:** Agent SDK includes AskUserQuestion; CLI asks before modifying files/running commands by default, with permissions and allowed-tools controls for automation.

**Recovery / cancellation surface:** Headless docs expose `--continue`/`--resume`; issue #28489 complains there is no built-in retry/verification/resume loop for headless automation after crashes. Managed Agents charge runtime only while running, with idle/rescheduling/terminated not billed.

**Pricing tiers:** Claude.com Free $0; Pro $20 monthly or $17/month annually, includes Claude Code and Claude Cowork; Max from $100/month with 5x or 20x Pro usage; Team $25/seat/month or $20 annual standard, premium seat $125/month or $100 annual with 5x usage; Enterprise $20/seat plus API usage rates/custom terms. API model rates include Opus 4.7/4.6/4.5 at $5/MTok input and $25/MTok output; Sonnet 4.x at $3/$15; Haiku 4.5 at $1/$5; prompt caching read 0.1x, writes 1.25x/2x; batch 50% discount. Managed Agents add $0.08/session-hour while running.

**Customer segments cited by the vendor:** Stripe, Ramp, Wiz, Rakuten. Stripe deployed to 1,370 engineers; one team completed a 10k-line Scala-to-Java migration in four days. Ramp cut incident investigation time by 80%. Wiz migrated a 50k-line Python library to Go in ~20 hours versus 2-3 month estimate. Rakuten reduced feature delivery from 24 working days to 5 and runs multiple Claude Code sessions in parallel.

**Documented user complaints:** GitHub issue #28489, Feb 25, 2026: `claude -p` lacked a built-in loop for iteration and automatic recovery after crashes; issue closed.

**Public traction signals:** Anthropic says a majority of its own code is now written by Claude Code; named enterprise customer metrics above.

**Positioning relative to agent-harness's space:** Cross-lane/hybrid. It overlaps heavily as an agent CLI and SDK that can be orchestrated and run headlessly, but Anthropic’s product is not primarily a cloud-hosted multi-initiative orchestrator UI.

**Notes:** Claude Managed Agents session-hour pricing is a notable signal: vendors are beginning to charge not just tokens, but live agent runtime.

## P-8 — Goose (Block)

**Vendor / docs:** https://goose-docs.ai/, https://goose-docs.ai/docs/guides/goose-cli-commands/, https://goose-docs.ai/docs/guides/recipes/session-recipes/, https://github.com/block/goose, https://github.com/block/goose/issues/5559  
**Deployment model:** local/hybrid. Goose is an open-source agent that runs on the user’s machine; it can connect to cloud LLM providers and MCP extensions.

**Capability surface:** Desktop app for macOS/Linux/Windows; full CLI; embeddable API; works with 15+ providers; 70+ MCP extensions; install/execute/edit/test with any LLM; sessions; recipes; schedules; containers; export; fork; run recipes interactively or non-interactively; max-turns; cron schedules.

**Long-running behavior:** CLI docs include `goose schedule add --cron`, `schedule list`, `schedule sessions`, `run-now`, and `remove`. Sessions can be resumed by name/session-id/path, forked, exported to JSON/Markdown/YAML, or run from recipes. Ralph Loop tutorial shows iterative handoff via files for repeated sessions.

**Multi-task / multi-initiative model:** Sessions, recipes, schedules. A recipe packages a session setup with tools/goals/instructions for one-click reuse. Schedules run recipes on cron and retain schedule sessions.

**Question routing:** Interactive sessions; recipes can run with `--interactive`; tool permissions depend on provider/extensions and local CLI.

**Recovery / cancellation surface:** Resume by name, session ID, or exported JSON/JSONL; fork sessions; export session backups; remove sessions by ID/name/regex. No cloud task dashboard evidence.

**Pricing tiers:** Open-source product; no vendor SaaS pricing found in primary sources. Users pay underlying LLM/provider costs. No BYO-key restriction found; provider model is explicit.

**Customer segments cited by the vendor:** General-purpose AI agent for coding, research, writing, automation, data analysis. Block positions it beyond code suggestions.

**Documented user complaints:** GitHub issue #5559, Nov 4, 2025: Goose with Claude Code provider could not invoke actions requiring permissions, such as creating a file, and did not present approval prompt; issue closed.

**Public traction signals:** GitHub repo shows 43.4k stars in scrape; repository description positions Goose as an open-source extensible AI agent.

**Positioning relative to agent-harness's space:** Cross-lane/local. Goose overlaps strongly with local sessions/recipes/schedules and multi-provider extensibility, but not as cloud-hosted long-running agent SaaS.

**Notes:** Goose is a key adjacent signal for recipes and scheduled local sessions. It should be kept out of the cloud-only competitor bucket.

## P-9 — OpenAI ChatGPT Agent / Operator

**Vendor / docs:** https://openai.com/index/introducing-chatgpt-agent/, https://help.openai.com/en/articles/11752874-chatgpt-agent, https://chatgpt.com/features/agent/, https://openai.com/business/chatgpt-pricing/, https://openai.com/index/introducing-operator/  
**Deployment model:** cloud-only consumer/business ChatGPT agent mode.

**Capability surface:** Navigates websites; researches data; completes online tasks; fills forms; edits spreadsheets; connects to third-party data sources; visual browser; code interpreter; terminal; app connectors such as Gmail/GitHub; creates editable slides/spreadsheets. Operator predecessor used its own browser for repetitive browser tasks and GUI interaction.

**Long-running behavior:** ChatGPT Agent announcement says tasks can be paused/resumed and managed collaboratively over time. Help docs define monthly message limits per unique agent invocation; intermediate clarification/authentication steps do not count against usage.

**Multi-task / multi-initiative model:** ChatGPT conversation/agent invocation model. No primary evidence of a multi-agent task board comparable to Devin/Cursor/Replit.

**Question routing:** Agent can ask clarifications and request authentication; high-impact actions require user confirmation. “Watch mode” requires supervision on certain sites.

**Recovery / cancellation surface:** User supervision and confirmations; restricted websites; safety refusals; prompt-injection monitoring. No detailed cloud task retry/rollback surface found.

**Pricing tiers:** Agent mode monthly message limits: Plus 40/month, Pro 400/month, Business & Enterprise 40/month; Business/Enterprise flexible pricing: 30 credits/message. ChatGPT Business pricing page lists Business Codex pay-as-you-go; Business ChatGPT & Codex $20/user/month; Enterprise custom. Consumer Plus/Pro prices are covered on OpenAI plan pages; cited business page does not show Plus/Pro in extracted pricing.

**Customer segments cited by the vendor:** Everyday and professional tasks; Business/Enterprise teams with app connectors, documents/tools/codebases, admin controls and no training on business data.

**Documented user complaints:** No official issue/forum complaint used. Vendor-documented limitations: subject to rate limits, cannot visit certain restricted sites, requires user confirmations/supervision for high-impact or sensitive tasks.

**Public traction signals:** None specific to ChatGPT Agent in cited primary sources.

**Positioning relative to agent-harness's space:** Adjacent more than direct. It competes for user attention around “agent that acts for me,” but is not developer multi-initiative orchestration. It is a broad cloud task agent with consumer/business packaging.

**Notes:** Operator was announced Jan 23, 2025 as a U.S. Pro research preview; ChatGPT Agent announcement on Jul 17, 2025 effectively folds browser, research, and action into ChatGPT.

## P-10 — Vercel v0 / Vercel Agent / Anthropic Projects

**Vendor / docs:** https://vercel.com/docs/agent/pricing, https://vercel.com/blog/updated-v0-pricing, https://vercel.com/pricing, https://support.anthropic.com/en/articles/9529781-examples-of-projects-you-can-create, https://support.anthropic.com/en/articles/9519189-project-visibility-and-sharing  
**Deployment model:** cloud-only SaaS for v0/Vercel Agent; Anthropic Projects are cloud-hosted Claude workspace artifacts.

**Capability surface:** Vercel Agent is evidence for AI code review/investigations priced per action plus tokens. v0 is an app/UI generation workspace with token/credit metering. Anthropic Projects organize context for research, product development, content, education, personal finance, home renovation, and other workflows.

**Long-running behavior:** Vercel Agent pricing docs describe investigations and code reviews, not multi-day autonomous sessions. v0 pricing update says usage is metered on input/output tokens and credits. Claude Projects persist project context; free users can create up to five projects.

**Multi-task / multi-initiative model:** v0/Projects are project/workspace constructs; Vercel Agent actions are investigations/reviews. No primary evidence of a multi-agent task board or scheduled long-running sessions.

**Question routing:** Standard SaaS chat/project interaction; no specific mid-task routing evidence found.

**Recovery / cancellation surface:** No agent-specific recovery surface found from primary sources in this lane.

**Pricing tiers:** Vercel Agent: $0.30 per Code Review or additional investigation, plus pass-through token costs based on AI provider rates; Observability Plus teams get 10 investigations per billing cycle; one $100 promotional credit for Pro teams, valid two weeks. v0 pricing update (May 13, 2025): Free includes $5 credits/month, Premium $20 credits/month, Team $30 credits/month; purchased credits expire after one year. Anthropic Projects: free users can create five projects; paid plan pricing is in Claude pricing (see P-7).

**Customer segments cited by the vendor:** Vercel developers/teams, frontend/app builders, product teams, research/content/education users for Claude Projects.

**Documented user complaints:** No primary official user complaint found for this section.

**Public traction signals:** None used.

**Positioning relative to agent-harness's space:** Adjacent. v0 and Projects compete for persistent AI workspace attention, but they are not evidence of a very long-running multi-initiative orchestrator.

**Notes:** Keep this in cross-lane/adjoining market unless investigating app builders, design-to-code, or persistent project workspaces.

## P-11 — Augment Code

**Vendor / docs:** https://www.augmentcode.com/, https://www.augmentcode.com/pricing, https://www.augmentcode.com/blog/introducing-remote-agent, https://www.augmentcode.com/blog/augment-codes-pricing-is-changing, https://github.com/augmentcode/augment-agent/issues  
**Deployment model:** hybrid. Augment has IDE/CLI/code-review products and Remote Agent cloud execution.

**Capability surface:** Context Engine; Coding Agent; Intent; chat and agents; MCP/native tools; usage analytics; Slack integration; code review; PR summaries/inline comments; Auto/Manual mode; PR guidelines; GitHub multi-org; security/compliance features. Remote Agent handles flaky tests, docs debt, refactors, bug-backlog tickets, coverage, config/lint/format migrations.

**Long-running behavior:** Remote Agent blog says it can run in the cloud at any time and encourages parallelizing small tasks. Augment frames best use as small, well-scoped, specific, validated tasks with skeptical PR review.

**Multi-task / multi-initiative model:** Remote Agents are parallelized task units; no primary evidence of a full cross-initiative task board from cited sources.

**Question routing:** Product docs indicate Auto/Manual mode and code review feedback surfaces; no detailed mid-task question routing found.

**Recovery / cancellation surface:** No detailed recovery/rollback surface found. Vendor blog emphasizes self-validation and PR review.

**Pricing tiers:** Indie $20/month with 40,000 credits. Standard $60/month/developer with 130,000 credits. Max $200/month/developer with 450,000 credits. Enterprise custom with unlimited users, bespoke credit limit, volume annual discounts, SSO/OIDC/SCIM, SOC 2/security reports, CMEK/ISO 42001, dedicated support, no AI training. Auto top-up: $15/24k credits. Pricing-change blog states power users relying on Remote Agents/CLI automation should expect $200+/month.

**Customer segments cited by the vendor:** Completions/Next Edit users, daily Agent users, power users, enterprise engineering teams. No named customers used from cited sources.

**Documented user complaints:** Official GitHub issues include Mar 28, 2026 bug: `remember` tool fails with “upper bound size missing”; Nov 26, 2025 feature request for audible task-completion notification.

**Public traction signals:** No primary traction numbers used here.

**Positioning relative to agent-harness's space:** Overlap on cloud remote agents and parallel task delegation. Differs by focusing on codebase context, PR/code review, and credit-priced team seats rather than a persistent multi-CLI orchestrator.

**Notes:** Augment’s pricing explicitly segments “power users” as $200+/month when Remote Agents write most code.

## P-12 — Refact.ai / Continue.dev Cloud / Tabnine Agentic Platform

**Vendor / docs:** https://refact.ai/, https://refact.ai/pricing/, https://github.com/smallcloudai/refact, https://www.continue.dev/pricing, https://github.com/continuedev/continue, https://github.com/continuedev/continue/issues, https://www.tabnine.com/pricing/, https://docs.tabnine.com/main/getting-started/tabnine-agent  
**Deployment model:** mixed. Refact is SaaS/self-hosted/on-prem/AWS. Continue is open-source CLI plus hosted team/cloud agents/checks. Tabnine supports SaaS, VPC, on-prem, and fully air-gapped deployments.

**Capability surface:** Refact: autonomous AI agent for end-to-end engineering tasks, reasoning, repo search/analysis, GitHub/databases/CI/CD, autocomplete, IDE chat, vector DB/RAG, BYOK, self-hosting. Continue: create/run AI agents, integrations with Slack/Sentry/Snyk, source-controlled AI checks as GitHub status checks, agents as Markdown files in `.continue/checks/`, CLI/VS Code extension. Tabnine: autonomous task-oriented assistant, codebase-wide refactoring, automated tests, docs synthesis, policy validation, tight feedback loop, determines when to check in, terminal-native coding agent, organizational standards awareness.

**Long-running behavior:** Continue emphasizes agents/checks running on every PR as CI status checks rather than open-ended sessions. Tabnine docs say the agent responds to state changes, accounts for dependencies, and breaks down complex tasks. Refact positions end-to-end multi-step tasks but cited pages did not document a cloud long-duration task board.

**Multi-task / multi-initiative model:** Continue uses source-controlled checks/agents and PR status checks. Refact uses IDE/agent task execution. Tabnine uses task-oriented agent flows. No primary evidence of a broad multi-initiative scheduler/board across these three.

**Question routing:** Tabnine states it keeps a tight feedback loop and determines when to check in for input/approval. Continue and Refact question-routing details were not found in primary sources used here.

**Recovery / cancellation surface:** Not documented in the primary sources used beyond CI/status-check pass/fail and local IDE/agent controls.

**Pricing tiers:** Refact Free $0/month: all Autonomous AI Agent capabilities, 2,000 coins, chat/agent, models, unlimited fast autocomplete, RAG, self-hosting option, Discord support. Refact Pro $10/month: 10,000 coins/month, additional coins from $5 minimum at $1=1,000 coins. Refact Enterprise: private server, fine-tuning, multi-GPU load sharing, access control/statistics, on-prem/private cloud, zero telemetry, priority support. Continue Starter: $3/million tokens pay-as-you-go; Team $20/seat/month with private shared agents and control over team agents; Company custom with SAML/OIDC, BYOK, commitments/invoicing/SLA. Tabnine Code Assistant $39/user/month; Tabnine Agentic Platform $59/user/month with autonomous agents, terminal-native agent, unlimited codebase connections, MCP, flexible deployment, zero code retention/privacy.

**Customer segments cited by the vendor:** Refact: thousands of developers, open-source/BYOK/privacy-sensitive teams. Continue: teams wanting source-controlled AI checks, integrations, frontier models. Tabnine: enterprises requiring code privacy, compliance, flexible deployment, organizational standards.

**Documented user complaints:** Continue GitHub issues show Apr 2026 errors for models/providers, CLI install failure with no error message, and “Session file does not exist.” Refact GitHub repo directs users to issues/feature requests but no specific complaint was extracted. Tabnine official docs/support page points users to support tickets; no primary complaint thread found.

**Public traction signals:** Refact GitHub repo: 3.5k stars. Continue GitHub repo: 32.8k stars. Tabnine pricing page modified Mar 24, 2026; no traction number used.

**Positioning relative to agent-harness's space:** Adjacent/hybrid. These products compete for coding-agent attention and enterprise agent budgets, but the primary evidence shows IDE/CI/checks/context-engine positioning more than long-running cloud orchestration.

**Notes:** Continue’s source-controlled AI checks are a distinct pattern: agents become versioned repo artifacts and CI gates, not free-floating cloud workers.

## Cross-product patterns

**Pricing patterns:** The market has not settled on one model. Devin, Cursor, Augment, Tabnine, GitHub Copilot, and ChatGPT bundle agents into seat tiers. Codex, Continue, Refact, Replit, Amp, and Vercel expose usage/credit/token/action pricing. Runtime pricing appears in Anthropic Managed Agents at $0.08/session-hour. Vercel Agent prices a fixed action fee plus pass-through tokens. GitHub adds premium requests and GitHub Actions minutes. Cursor charges API pricing for selected model in Max Mode. Several products have “power user” price anchors near $200/month: Cursor Ultra $200, Devin Max $200, ChatGPT Pro $200, Augment Max $200, Codex Pro 20x $200, Claude Max starts at $100 and extends to 20x.

**Capability patterns:** The core common bundle is: repo access, isolated execution, code edits, shell/test execution, branch/PR handoff, logs/artifacts, and some human review step. The strongest cloud products also add task lists, schedules, concurrent workers, and integrations with GitHub/Slack/Linear. Recovery surfaces usually stop at logs, PRs, retry, cancel, rollback, or resume; only Replit strongly documents checkpoint rollback. Very few products expose a vendor-documented model for persistent cross-initiative memory beyond sessions/projects/threads.

**Customer-segment patterns:** Buyers/users are software engineering teams, enterprise modernization programs, app builders, and power developers. Named enterprise evidence clusters around migration/refactor/testing/customer engineering: Devin/Nubank, Anthropic/Stripe/Ramp/Wiz/Rakuten, OpenAI/Notion/Ramp/Braintrust/Wasmer, GitHub/EY/Carvana. Adjacent products emphasize enterprise privacy/compliance: Tabnine, Refact, Augment, Sourcegraph/Amp, Claude Enterprise, ChatGPT Business/Enterprise.

**Very long running positioning:** Devin is the clearest for multi-day/always-on behavior: scheduled sessions, Managed Devins, state notes, and 30-day session continuation limit. Codex now supports scheduled recurring automations and background worktrees, but launch-task expectations were 1-30 minutes. Replit supports autonomous long builds, background tasks, and checkpoint rollback, with Pro allowing 10 active background tasks. Goose supports local cron schedules and resumable sessions. GitHub Copilot cloud agent is background PR automation, but docs say stuck sessions may time out after an hour and one task maps to one PR. Cursor supports many parallel cloud agents and disconnected local machines but documentation does not establish multi-day persistence. Anthropic Managed Agents introduce session-hour runtime pricing, but Claude Code remains primarily CLI/API/hybrid.

**What none of them clearly offer in primary evidence:** A single-user desktop-first, multi-CLI, multi-initiative orchestrator with a durable context graph across agents. Products have sessions, threads, tasks, projects, schedules, or PRs, but primary docs rarely describe long-lived cross-task context normalization, global context optimization, or recovery across heterogeneous CLIs.

## Cross-lane signals

Amp, Claude Code, Goose, Refact, Continue, and Tabnine are hybrid/local/IDE/CLI/CI products as much as cloud-agent products. They belong partly to local desktop assistants, agent SDKs, enterprise coding assistants, and CI automation lanes. Vercel v0 and Anthropic Projects belong partly to app builders, design-to-code, and persistent workspace/knowledge lanes. ChatGPT Agent/Operator belongs partly to consumer/generalist web agents. I included them because the prompt named them, but the direct cloud-hosted long-running competitor set is narrower: Devin, Cursor Cloud Agents, Codex Cloud, Replit Agent, GitHub Copilot cloud agent, and Augment Remote Agent.

## Open evidence gaps

- No primary-source ARR, headcount, or funding figures were used for most vendors.
- Amp official user complaint evidence was not found; only vendor-documented limitations were available.
- Tabnine primary complaint evidence was not found in public official issue/forum surfaces.
- Refact specific official complaint threads were not extracted; only repo/issues entry points and product/pricing evidence.
- Vercel v0/Agent and Anthropic Projects do not provide primary evidence of long-running multi-agent orchestration; they are adjacent workspace/action products.
- Cursor customer logos/case-study metrics were not used because the cited product/docs/pricing evidence did not provide them.

## Citations

- Amp Owner's Manual. https://ampcode.com/manual
- Anthropic Claude Code product page. https://www.anthropic.com/product/claude-code
- Anthropic Claude pricing. https://claude.com/pricing
- Anthropic Claude Projects examples. https://support.anthropic.com/en/articles/9529781-examples-of-projects-you-can-create
- Anthropic Claude Projects visibility/sharing. https://support.anthropic.com/en/articles/9519189-project-visibility-and-sharing
- Anthropic Claude API pricing. https://platform.claude.com/docs/en/about-claude/pricing
- Anthropic Claude Agent SDK overview. https://code.claude.com/docs/en/agent-sdk/overview
- Anthropic Claude Code headless/programmatic docs. https://code.claude.com/docs/en/headless
- Anthropic Claude Code issue #28489, Feb 25, 2026. https://github.com/anthropics/claude-code/issues/28489
- Augment Agent GitHub issues. https://github.com/augmentcode/augment-agent/issues
- Augment Code pricing. https://www.augmentcode.com/pricing
- Augment Code product page. https://www.augmentcode.com/
- Augment Code Remote Agent announcement, May 7, 2025. https://www.augmentcode.com/blog/introducing-remote-agent
- Augment Code pricing-change blog, Oct 6, 2025. https://www.augmentcode.com/blog/augment-codes-pricing-is-changing
- Block Goose GitHub repo. https://github.com/block/goose
- ChatGPT Agent help. https://help.openai.com/en/articles/11752874-chatgpt-agent
- ChatGPT Agent product page. https://chatgpt.com/features/agent/
- ChatGPT business pricing. https://openai.com/business/chatgpt-pricing/
- Cognition Devin Managed Devins blog, Mar 19, 2026. https://cognition.ai/blog/devin-can-now-manage-devins
- Cognition Devin Scheduled Devins blog, Mar 20, 2026. https://cognition.ai/blog/devin-can-now-schedule-devins
- Continue pricing. https://www.continue.dev/pricing
- Continue GitHub issues. https://github.com/continuedev/continue/issues
- Continue GitHub repo. https://github.com/continuedev/continue
- Cursor Background Agent changelog 0.50, May 15, 2025. https://cursor.com/changelog/0-50
- Cursor Cloud Agent capabilities docs. https://cursor.com/docs/cloud-agent/capabilities
- Cursor Cloud Agent docs. https://cursor.com/docs/cloud-agent
- Cursor forum API-spawned background-agent bug, Feb 27, 2026. https://forum.cursor.com/t/background-agents-spawned-via-api-cannot-post-pr-comments-or-reviews-despite-correct-github-app-permissions/153207
- Cursor forum spend-limit bug, 2025. https://forum.cursor.com/t/background-agent-error-for-required-usage-limit-of-10-is-inaccurate-or-misleading/104018
- Cursor pricing. https://cursor.com/pricing
- Devin admin common issues. https://docs.devin.ai/admin/common-issues
- Devin customers. https://devin.ai/customers
- Devin first-run docs. https://docs.devin.ai/get-started/first-run
- Devin intro docs. https://docs.devin.ai/get-started/devin-intro
- Devin pricing. https://devin.ai/pricing/
- Devin scheduled sessions docs. https://docs.devin.ai/product-guides/scheduled-sessions
- Devin session tools docs. https://docs.devin.ai/work-with-devin/devin-session-tools
- GitHub Copilot billing for organizations and enterprises. https://docs.github.com/en/copilot/concepts/billing/organizations-and-enterprises
- GitHub Copilot cloud agent about docs. https://docs.github.com/copilot/concepts/agents/coding-agent/about-coding-agent
- GitHub Copilot cloud agent create-PR docs. https://docs.github.com/en/copilot/how-tos/use-copilot-agents/cloud-agent/create-a-pr
- GitHub Copilot cloud agent troubleshooting docs. https://docs.github.com/en/copilot/how-tos/use-copilot-agents/cloud-agent/troubleshoot-cloud-agent
- GitHub Copilot coding agent launch blog, May 19, 2025. https://github.blog/news-insights/product-news/github-copilot-meet-the-new-coding-agent/
- GitHub Copilot Community discussion #188531, Mar 3, 2026. https://github.com/orgs/community/discussions/188531
- GitHub Copilot Community discussion #189951, Mar 17, 2026. https://github.com/orgs/community/discussions/189951
- GitHub Copilot plans. https://github.com/features/copilot/plans
- Goose CLI commands docs. https://goose-docs.ai/docs/guides/goose-cli-commands/
- Goose docs homepage. https://goose-docs.ai/
- Goose issue #5559, Nov 4, 2025. https://github.com/block/goose/issues/5559
- Goose reusable recipes docs. https://goose-docs.ai/docs/guides/recipes/session-recipes/
- OpenAI ChatGPT Agent announcement, Jul 17, 2025. https://openai.com/index/introducing-chatgpt-agent/
- OpenAI Codex automations docs. https://developers.openai.com/codex/app/automations
- OpenAI Codex cloud docs. https://developers.openai.com/codex/cloud
- OpenAI Codex flexible pricing for teams, Apr 2, 2026. https://openai.com/index/codex-flexible-pricing-for-teams/
- OpenAI Codex pricing. https://developers.openai.com/codex/pricing
- OpenAI Codex rate card help. https://help.openai.com/en/articles/20001106-codex-rate-card
- OpenAI Codex launch blog, May 16, 2025. https://openai.com/index/introducing-codex/
- OpenAI Community Codex limits complaint, Nov 2025. https://community.openai.com/t/pro-plan-hit-5-hour-limit-twice-in-2h-and-1-5h-and-nearly-exhausted-weekly-cap-in-1-day-after-today-s-update/1364782
- OpenAI Operator announcement, Jan 23, 2025. https://openai.com/index/introducing-operator/
- Refact GitHub repo. https://github.com/smallcloudai/refact
- Refact pricing. https://refact.ai/pricing/
- Refact product page. https://refact.ai/
- Replit Agent billing. https://docs.replit.com/billing/ai-billing
- Replit Agent checkpoints and rollbacks. https://docs.replit.com/core-concepts/agent/checkpoints-and-rollbacks
- Replit Agent docs. https://docs.replit.com/core-concepts/agent
- Replit Agent task system. https://docs.replit.com/core-concepts/agent/task-system
- Replit community rollback complaint, Feb 9, 2025. https://replit.discourse.group/t/rollback-disabled-and-suggestion-to-create-a-new-chat/3555
- Replit pricing. https://replit.com/pricing
- Sourcegraph pricing. https://sourcegraph.com/pricing
- Tabnine Agent docs. https://docs.tabnine.com/main/getting-started/tabnine-agent
- Tabnine pricing. https://www.tabnine.com/pricing/
- Vercel Agent pricing. https://vercel.com/docs/agent/pricing
- Vercel pricing. https://vercel.com/pricing
- Vercel v0 pricing update, May 13, 2025. https://vercel.com/blog/updated-v0-pricing
