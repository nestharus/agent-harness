# agent-harness — Project Routing & Conventions

This project follows the shared `~/ai/` workflow library. Start with [`~/ai/AGENTS.md`](/home/nes/ai/AGENTS.md) for the generic operator-routing layer; this file documents only the project-local overrides and conventions.

## Project facts

- **Repo root:** `/home/nes/projects/agent-harness/`
- **Stack:** see canonical [`/DECISIONS.md`](DECISIONS.md) — Tauri 2.10.2 + Rust 2024 + Tokio 1.49 + rusqlite 0.38 + hand-rolled migrations from `~/projects/agent-runner` (D2); SolidJS 1.9.x + Vite 7 + `@solidjs/router` + TanStack Solid Query/Form + Ark UI Solid + vanilla CSS + Oxlint/Oxfmt/tsgo + Vitest 4 + Playwright 1.58 + Bun 1.2+ from `~/projects/server-manager` research (D3–D17). Tailwind explicitly REJECTED.
- **Worktrees root:** `worktrees/` (per-WU implementation worktrees: `worktrees/impl-<wu_id-lower>`).
- **Per-WU scratch root:** `tmp/scratch/<wu_id-lower>/` (prompts, logs, question artifacts, `phase6/` outputs).
- **Logs:** `tmp/logs/<wu_id-lower>-<phase>.log` (paired 1:1 with prompts in `tmp/scratch/<wu_id-lower>/prompts/`).
- **DECISIONS.md** lives at repo root and is appended by the orchestrator on every Tier-1/2/3 escalation, every supported-surface termination, every accepted-residual decision.

## Project-local strategic artifacts (in `product-strategy/`)

The project keeps its own `problem.md`, `philosophy.md`, `proposal.md`, and the Stage 1/2 outputs (`problem-review.md`, `philosophy-review.md`, future `*-surfaces.md` / `*-classification.md` / `philosophy-decisions.md`). The project also keeps its own axis-reference table at `product-strategy/problem-axis-table.md` (referenced as runtime input by the shared `problem-alignment` operator). Roadmap-layer outputs (`executive-roadmap.md`, `engineering-roadmap.md`, per-phase `ai-roadmap-phase-N.md`, `market-research.md`, `engineering-research.md`, `run-report.md`) also live here.

**The operators themselves live in `~/ai/agents/`**, not in `product-strategy/`. See `~/ai/AGENTS.md` for routing.

## Implementation pipeline

**All Work Unit implementation goes through the [`implementation-pipeline-orchestrator`](/home/nes/ai/agents/implementation-pipeline-orchestrator.md) (`claude-opus`)**, dispatched via:

```bash
agents -m claude-opus \
  -a implementation-pipeline-orchestrator \
  -i wu_id=<WU-ID> \
  -i ticket_branch=<tickets-phase-NN-rN> \
  -i repo_root=/home/nes/projects/agent-harness \
  -i worktree_path=/home/nes/projects/agent-harness/worktrees/impl-<wu-lower> \
  -i scratch_dir=/home/nes/projects/agent-harness/tmp/scratch/<wu-lower>/ \
  2>&1 | tee tmp/logs/<wu-lower>-orchestrator.log
```

The orchestrator owns every dispatch in the WU lifecycle. The root conversation does NOT inline any phase work — doing so removes the orchestration from `agents trace --json` and prevents `process-tree-auditor` from auditing the run end-to-end.

## Surviving human gates (only two)

1. **Phase 2.5 problem-map review** — orchestrator emits a NEEDS_INPUT to the root carrying the proposed `research/<wu>-problem-map.md`.
2. **NEEDS_INPUT new-value question** — sub-agents that surface a previously-unevaluated value, scope, or trade-off question are forwarded to the root. Procedural NEEDS_INPUT (missing path, missing input the orchestrator can supply) is resolved by the orchestrator and does not reach the user.

All other human gates from `~/ai/workflows/implementation-pipeline.md` (Phase 0/1/2/5, draft-PR open, Phase 10 promotion) are removed for this project's runs and handled autonomously by the orchestrator.

## Workflow violation policy

Strict adherence is mandatory. Per the shared workflow + project policy, any deviation from `~/ai/workflows/implementation-pipeline.md` triggers the **3-tier escalation** (autonomous, no user confirmation required for any tier):

1. **Tier 1 — Rewind.** Identify the last commit produced under full pipeline compliance. `git reset --hard <commit>` + `git push --force-with-lease origin main`. Delete affected branches + worktrees. Re-attempt from the failed phase. Record in `DECISIONS.md`.
2. **Tier 2 — Split.** If a Tier-1 retry produces another violation on the same WU, split the WU on a fresh per-phase ticket-regen branch into smaller WUs. Re-enter Phase 2.5 for each split.
3. **Tier 3 — Shrink.** If Tier-2 still violates, shrink to single-AC or single-function WUs.

Three consecutive Tier-3 shrinks failing on the same WU emits a NEEDS_INPUT new-value-question to the root and halts.

## Project-specific conventions

- **Use `agents` CLI for every dispatch.** Never the Claude Code local Agent (Task subagent) tool for any pipeline work. The local Explore subagent is OK for read-only context lookups that protect this conversation's window — never for impl/review/audit work.
- **`bun run` invocations** in worktrees may need `bun install` first. The orchestrator handles this via worktree bootstrap.
- **Determinism:** `BTreeMap` (not `HashMap`) anywhere maps appear in serialized JSON; otherwise fixture round-trips become flaky.
- **`impl_sqlite_text_enum!` macro convention** for SQLx Type/Encode/Decode on string-enum columns.
- **`JsonField<T>` codec wrapper** for serde+sqlx; deterministic key ordering required.
- **`phase_0a_scaffold_commands()` invariant:** count of registered Tauri commands stays at 1 (`subscribe_workspace_events` from WU-0A-08) until Phase 0C explicitly grows it.

## Pre-product planning state

Main currently sits at `cc16433` (post-roadmap-layer-2, all risk gates LOW). All product implementation (Phase 0A onwards) is being redone through the proper orchestrator after the prior bundled-dispatch attempts were wiped.

## Authoritative references

- [`~/ai/AGENTS.md`](/home/nes/ai/AGENTS.md) — shared operator routing
- [`~/ai/workflows/implementation-pipeline.md`](/home/nes/ai/workflows/implementation-pipeline.md) — 10-phase pipeline spec
- [`~/ai/agents/implementation-pipeline-orchestrator.md`](/home/nes/ai/agents/implementation-pipeline-orchestrator.md) — the orchestrator that walks the pipeline
- [`~/ai/agents/process-tree-auditor.md`](/home/nes/ai/agents/process-tree-auditor.md) — required at the Phase 4, Phase 6, and Phase 8 join points
- [`~/ai/conventions/gate-ownership.md`](/home/nes/ai/conventions/gate-ownership.md) — gate-owner table (with the human-gate restrictions applied)
- [`~/ai/conventions/agent-questions-and-session-graph.md`](/home/nes/ai/conventions/agent-questions-and-session-graph.md) — NEEDS_INPUT envelope and session graph
- [`~/ai/conventions/audit-history.md`](/home/nes/ai/conventions/audit-history.md) — revise/review loop conventions
- [`~/ai/conventions/workflow-execution-violations.md`](/home/nes/ai/conventions/workflow-execution-violations.md) — violation taxonomy
- [`~/ai/workflows/agents-cli.md`](/home/nes/ai/workflows/agents-cli.md) — `agents` CLI conventions
- [`~/ai/models/roles.md`](/home/nes/ai/models/roles.md) — model-by-phase assignment matrix
