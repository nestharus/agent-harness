# Engineering — Feasibility Risk Assessment

**Rating: LOW**

## Findings

### F-1. Phase magnitudes are strategically plausible

**Severity: NONE**

The roadmap correctly treats Phase 0A/0B/0C as substantial foundation work rather than a small bootstrap. This matches the substrate evidence: the worktree has no harness app code, migrations, packages, or tests (`product-strategy/engineering-research.md:5`, `product-strategy/engineering-research.md:13-15`, `product-strategy/engineering-research.md:21-31`), while the roadmap identifies 21 foundation items across the fixed desktop stack, GraphStore, evidence/provenance, RenderEngine, PolicyEngine, BudgetLedger, provider fingerprints, CLI supervision, IPC, UI shell, and tests (`product-strategy/engineering-roadmap.md:23-47`). The phase-level estimate of XL / 12-20 serial weeks for Phase 0 and 45-82 serial weeks overall is magnitude-consistent with an empty codebase and 21 value slices (`product-strategy/engineering-roadmap.md:640-654`).

Later phase magnitudes also scale with slice count and dependency depth: Phase 1 carries seven observable-context slices; Phase 2 introduces orchestrator turns and optimizer summaries; Phase 3 introduces topology/concurrency/identity risk; Phase 4-7 add worker dispatch, reintegration, reviewer sampling, recovery, and provider reroute closure (`product-strategy/engineering-roadmap.md:549-608`, `product-strategy/engineering-roadmap.md:656-680`). This covers the product's 18 difficulty axes, especially D1-D18 in `product-strategy/problem.md:9-185`, through the proposal's runtime subsystem set rather than server-manager's 7/14 framing.

**Recommendation:** no action required.

---

### F-2. Sequencing is physically possible from the current substrate

**Severity: NONE**

The phase order is feasible given that only `agent-runner` and reference Tauri patterns exist today. Phase 0A creates the repository/runtime skeleton; Phase 0B creates canonical state and inert contracts; Phase 0C creates shared engines and integration shells (`product-strategy/engineering-roadmap.md:537-547`). Phase 1 then uses those foundations for observable imposed context, with VS-003/VS-004 backend primitives before VS-001 acceptance so the first inspector does not depend on missing evidence and budget fields (`product-strategy/engineering-roadmap.md:549-560`).

The later dependency graph is acyclic and each high-risk capability is sequenced after its required substrate: VS-009 follows render/provenance/budget/provider foundations; VS-013 precedes VS-012 topology mutation; VS-015 waits for provider, budget, turn, focus, and conflict contracts; VS-021 waits for provider fingerprints, worker dispatch, and recovery accounting (`product-strategy/engineering-roadmap.md:632-638`, `product-strategy/engineering-roadmap.md:682-716`). This is feasible across the 10 proposal runtime subsystems: GraphStore, RenderEngine, OrchestratorBridge, WorkerDispatcher, Optimizer, WorkflowReviewer, PolicyEngine, ConfigurationRegistry, ProviderStateMonitor, and UserSurface (`product-strategy/proposal.md:114-178`).

**Recommendation:** no action required.

---

### F-3. Locked-decision consistency holds

**Severity: NONE**

I found no conflict with the locked-decision equivalents in `philosophy.md` and `proposal.md`. The roadmap preserves P5 by keeping VS-008 pack/unpack/focus in `AgentWalkState` and VS-012 topology edits under optimizer/merge mechanics rather than agent foreground writes (`product-strategy/philosophy.md:45-51`, `product-strategy/engineering-roadmap.md:562-581`). It preserves P6 by using snapshot/merge foundations before topology mutation and conflict handling (`product-strategy/philosophy.md:55-61`, `product-strategy/proposal.md:30-34`, `product-strategy/engineering-roadmap.md:572-581`). It preserves P9 by routing orchestrator turns through a no-compaction discipline rather than treating `/compact` as a fallback (`product-strategy/philosophy.md:85-91`, `product-strategy/proposal.md:83-85`, `product-strategy/engineering-roadmap.md:668`).

The roadmap also preserves P15 local graph control by building a harness-owned GraphStore and treating agent-runner state as substrate/evidence, not canonical graph truth (`product-strategy/philosophy.md:145-151`, `product-strategy/proposal.md:114-119`, `product-strategy/engineering-roadmap.md:27-35`). It preserves P16 single-user/single-tab scope through a Tauri desktop shell and single-tab UI shell rather than multi-tenant or many-chat surfaces (`product-strategy/philosophy.md:155-161`, `product-strategy/proposal.md:1438-1458`, `product-strategy/engineering-roadmap.md:43`, `product-strategy/engineering-roadmap.md:539`).

**Recommendation:** no action required.

---

### F-4. Substrate-version assumptions are bounded and acknowledged

**Severity: NONE**

The roadmap does not assume nonexistent harness capabilities already exist. It distinguishes reusable `agent-runner` substrate from harness-owned work: `agent-runner` supplies multi-provider invocation, resume, trace, session ingestion, provider diagnostics, quota windows, and partial session capture, while the harness still needs process supervision, graph state, evidence storage, render and policy engines, provider fingerprints, and UI (`product-strategy/engineering-roadmap.md:5-11`, `product-strategy/engineering-research.md:41-58`). It also acknowledges substrate gaps such as no opencode turn adapter, optional transcript locators, no harness MCP/hook/plugin code, and no SQLite CLI assumption (`product-strategy/engineering-research.md:50-58`, `product-strategy/engineering-research.md:85-97`).

The fixed substrate in the proposal is current architectural input, not treated as already implemented: Tauri v2/Bun/Turbo/React/Rust/Tokio/SQLite are target stack constraints, and `agent-runner` remains the invocation/resume/session-ingestion boundary (`product-strategy/proposal.md:14-20`). The roadmap's Phase 0A/0B/0C covers the six high-fanout cross-cutting concerns needed before value slices can safely depend on the substrate: local state/storage, evidence/provenance/audit, deterministic policy gates, budget/cache accounting, provider capability fingerprints, and recovery/state accounting (`product-strategy/engineering-roadmap.md:537-547`).

**Recommendation:** no action required.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| F-1 | Phase magnitudes are strategically plausible | NONE |
| F-2 | Sequencing is physically possible from the current substrate | NONE |
| F-3 | Locked-decision consistency holds | NONE |
| F-4 | Substrate-version assumptions are bounded and acknowledged | NONE |

## What LOW requires

- Phase 0A/0B/0C remains explicitly XL/substantial foundation work and is not compressed into a small bootstrap while still carrying GraphStore, render, policy, evidence, provider, recovery, IPC, UI, and test foundations.
- Phase 1 acceptance for VS-001 continues to require real evidence/provenance and budget/cache fields from VS-003/VS-004 primitives, as stated in the roadmap.
- VS-013 conflict/identity mechanics remain before enabled VS-012 topology mutation.
- VS-015 worker dispatch remains after provider fingerprints, budget gates, turn state, focus state, render snapshots, evidence/audit, and conflict contracts exist.
- VS-017 continuations continue to require child acceptance and durable failed-resume handoff, rather than treating wrapper resume attempts as authoritative.
- VS-021 reroute/substitution remains after VS-020 recovery accounting, so fresh substitution is not represented as successful resume.
- The harness continues to use `/home/nes/.local/bin/agents` as the agent-runner boundary rather than replacing, forking, or duplicating it.
- Vendor sessions, CLI transcripts, and agent-runner SQLite remain evidence/substrate only; harness GraphStore/evidence/audit state remains canonical local state.
- `/compact` remains prevented or treated as corrupt state, not an interoperability fallback.
- Provider state remains observed and redacted, not owned as credential/account management.
