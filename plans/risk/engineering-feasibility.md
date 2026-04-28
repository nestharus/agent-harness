# Engineering — Feasibility Risk Assessment

**Rating: LOW**

## Findings

### F-1. Phase magnitudes are strategically plausible

**Severity: NONE**

The roadmap correctly treats Phase 0A/0B/0C as substantial foundation work, which is the right magnitude for the current substrate. The research states that the worktree has no harness application code, migrations, packages, tests, or runtime services, and that the locked constraints live in `philosophy.md` plus `proposal.md` rather than `DECISIONS.md` (`product-strategy/engineering-research.md:5`, `product-strategy/engineering-research.md:13-15`, `product-strategy/engineering-research.md:21-31`, `product-strategy/engineering-research.md:33-35`). The roadmap then scopes Phase 0 across 22 foundation items: Tauri/Turbo/React/Rust/Tokio/SQLite scaffolding, GraphStore, evidence/provenance, audit, RenderEngine, PolicyEngine, BudgetLedger, provider fingerprints, CLI supervision, hook/MCP/plugin boundaries, IPC, AgentWalkState, optimizer/conflict/recovery shells, UI, fixtures, and tracing (`product-strategy/engineering-roadmap.md:23-48`).

The phase estimates are not off by an order of magnitude: Phase 0 is XL / 12-20 serial weeks; Phase 1 is seven slices / 10-16 serial weeks; Phase 3 grows for topology, identity, and quarantine depth; later phases shrink or grow according to slice count and dependency depth (`product-strategy/engineering-roadmap.md:552-616`, `product-strategy/engineering-roadmap.md:618-677`, `product-strategy/engineering-roadmap.md:709-723`). I used the agent-harness frame here: the 18 axes are the proposal's `D1` through `D18` (`product-strategy/proposal.md:1474-1495`), and the ten-subsystem frame is Context Graph & Provenance, Orchestrator, Continuous Optimizer, Sub-Agent Dispatch & Reintegration, NEEDS_INPUT Routing, Workflow Review & Governance, Cost & Budget, Recovery, User Surface, and Cross-CLI Adaptation. I did not use server-manager's 7/14 framing.

**Recommendation:** no action required.

---

### F-2. Sequencing is physically possible from the current substrate

**Severity: NONE**

The ordering is technically possible because each phase consumes foundations that are either already available from `agent-runner` or explicitly built before use. `agent-runner` provides invocation, file prompts, project working directories, resume, trace, session ingestion, quota state, provider diagnostics, and partial session capture; the research also states that reusable substrate stops there and that harness graph, render, policy, evidence, and UI work remains absent (`product-strategy/engineering-research.md:41-58`, `product-strategy/engineering-research.md:58-69`). Phase 0A creates the runtime shell, Phase 0B creates canonical state/contracts, and Phase 0C creates shared engines and integration shells before Phase 1 value slices consume them (`product-strategy/engineering-roadmap.md:552-604`).

The dependency order is acyclic at the acceptance-prerequisite level. Phase 1 requires VS-003 evidence/audit and VS-004 budget/cache primitives before VS-001 acceptance; Phase 3 places VS-013 identity/conflict mechanics before enabled VS-012 topology mutation; Phase 4 keeps VS-015 worker dispatch after provider, budget, turn, focus, render, evidence/audit, and conflict contracts; Phase 7 keeps VS-021 after VS-020 recovery accounting (`product-strategy/engineering-roadmap.md:620-629`, `product-strategy/engineering-roadmap.md:641-677`, `product-strategy/engineering-roadmap.md:701-707`, `product-strategy/engineering-roadmap.md:751-787`). The Phase 6 `VS-020 -> VS-019` event edge is explicitly handled as a later recovery-anomaly subscription after the core sampler ships, so it does not make Phase 6 physically cyclic (`product-strategy/engineering-roadmap.md:666-673`, `product-strategy/engineering-roadmap.md:779-787`).

**Recommendation:** no action required.

---

### F-3. Locked-decision consistency holds

**Severity: NONE**

I found no conflict with the locked-decision equivalents in `philosophy.md` and `proposal.md`. The roadmap preserves P5 agent-owned focus / optimizer-owned curation by keeping `OptimizerRequest` advisory-only in Phase 0C and by keeping pack/unpack/focus as `AgentWalkState` navigation, not graph topology mutation (`product-strategy/philosophy.md:45-51`, `product-strategy/proposal.md:473-510`, `product-strategy/proposal.md:1030-1050`, `product-strategy/engineering-roadmap.md:606-616`, `product-strategy/engineering-roadmap.md:631-650`). It preserves P6 coordinated concurrency by sequencing snapshot/merge and conflict semantics before topology mutation and by keeping optimizer edits visible at turn boundaries rather than mid-turn (`product-strategy/philosophy.md:55-61`, `product-strategy/proposal.md:1052-1081`, `product-strategy/engineering-roadmap.md:641-650`).

The roadmap also preserves P9, P15, and P16. It does not use `/compact` as a fallback; VS-009 calls out no-compaction discipline as a risk, while proposal/philosophy require compaction to be prevented or treated as corrupt (`product-strategy/philosophy.md:85-91`, `product-strategy/proposal.md:1438-1458`, `product-strategy/engineering-roadmap.md:737`). It keeps local GraphStore/evidence/audit state canonical while treating `agent-runner` state as evidence/substrate (`product-strategy/philosophy.md:145-151`, `product-strategy/proposal.md:114-119`, `product-strategy/engineering-roadmap.md:596-604`). It stays inside the single-user, single-tab, one-orchestrator scope through Phase 0A's single-tab shell and proposal's desktop scope (`product-strategy/philosophy.md:155-161`, `product-strategy/proposal.md:1438-1458`, `product-strategy/engineering-roadmap.md:552-554`).

**Recommendation:** no action required.

---

### F-4. Substrate-version assumptions are bounded and acknowledged

**Severity: NONE**

The roadmap does not rely on unshipped harness capabilities as if they already exist. It distinguishes the fixed external substrate from the missing harness implementation: `agent-runner` is installed and supplies invocation/resume/session-ingestion/quota boundaries, while the harness must still create process supervision, graph state, render and policy engines, provider fingerprints, evidence storage, IPC, and UI (`product-strategy/proposal.md:14-20`, `product-strategy/engineering-roadmap.md:5-11`, `product-strategy/engineering-research.md:127-148`). Substrate gaps are acknowledged rather than hidden: no harness MCP/hook/plugin code exists, opencode ingestion is weaker, transcript locators are optional, provider quota is not a complete harness budget ledger, and `sqlite3` CLI availability cannot be assumed (`product-strategy/engineering-research.md:85-87`, `product-strategy/engineering-research.md:216-230`).

The cross-cutting feasibility dependencies are therefore represented as work, not assumptions: local state/storage through GraphStore, evidence/provenance/audit, deterministic policy gates, budget/cache accounting, provider-state/capability fingerprints, and recovery/state accounting all appear in Phase 0A/0B/0C or as explicit later consumers (`product-strategy/engineering-roadmap.md:552-616`, `product-strategy/engineering-roadmap.md:725-749`). That matches the proposal's cross-cutting constraints: local graph state is canonical, provider state is observed rather than owned, `/compact` is corruption rather than fallback, reviewer coverage is sampled, configuration is explicit state, and single-user/single-tab scope remains fixed (`product-strategy/proposal.md:1436-1458`).

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

- Phase 0A/0B/0C remains substantial XL foundation work and is not compressed into a small bootstrap while still carrying GraphStore, render, policy, evidence/provenance/audit, provider, recovery, IPC, UI, and test foundations.
- Phase 1 acceptance for VS-001 continues to require real VS-003 evidence/provenance and VS-004 budget/cache primitives.
- VS-013 identity/conflict mechanics remain before enabled VS-012 topology mutation.
- VS-015 worker dispatch remains after provider fingerprints, budget gates, turn state, focus state, render snapshots, evidence/audit records, and conflict contracts exist.
- VS-017 continuations continue to require child acceptance and durable failed-resume handoff, rather than treating wrapper resume attempts as authoritative.
- VS-021 reroute/substitution remains after VS-020 recovery accounting, so fresh substitution is not represented as successful resume.
- The harness continues to use `/home/nes/.local/bin/agents` as the `agent-runner` boundary rather than replacing, forking, or duplicating it.
- Vendor sessions, CLI transcripts, and `agent-runner` SQLite remain evidence/substrate only; harness GraphStore/evidence/audit state remains canonical local state.
- `/compact` remains prevented or treated as corrupt state, not an interoperability fallback.
- Provider state remains observed and redacted, not owned as credential/account management.
