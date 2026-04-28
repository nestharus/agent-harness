# Executive — Completeness Risk Assessment

**Rating: LOW**

This report assesses whether `executive-roadmap.md` covers the agent-harness's ten subsystems, six cross-cutting concerns, and eighteen problem axes (overrides per the prompt; not server-manager's seven subsystems / fourteen axes). All ten subsystems are covered by at least one of VS-001..VS-021; all eighteen problem axes are addressed; four of six cross-cutting concerns are addressed by named slices; the remaining two (`Local Control of State`, `No In-Product Compaction`) are addressed implicitly through Phase 0 and the working-set / budget discipline that the named slices impose, but the roadmap does not name them as concerns. Every value slice passes the standalone-value test. The Phase 0 deferral has specific rationale (foundations are not independently operator-visible). Two LOW findings recommend making the implicit cross-cutting commitments visible.

## Findings

### F-1. All ten subsystems are covered

**Severity: INFO**

Each of the ten subsystems appears in the `Subsystem(s)` line of at least one value slice. The roadmap's "Deferred Subsystems" section enumerates the mapping; cross-checking each slice's `Subsystem(s)` line confirms it.

| Subsystem | Slices |
|---|---|
| Context Graph & Provenance | VS-001, VS-002, VS-003, VS-005, VS-007, VS-008, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-015, VS-016, VS-018, VS-020 |
| Orchestrator | VS-001, VS-003, VS-004, VS-006, VS-007, VS-008, VS-009, VS-012, VS-015 |
| Continuous Optimizer | VS-004, VS-005, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-018, VS-019 |
| Sub-Agent Dispatch & Reintegration | VS-004, VS-006, VS-015, VS-016, VS-017, VS-018, VS-019, VS-020, VS-021 |
| NEEDS_INPUT Routing | VS-017, VS-020 |
| Workflow Review & Governance | VS-002, VS-003, VS-008, VS-009, VS-010, VS-011, VS-013, VS-014, VS-018, VS-019 |
| Cost & Budget | VS-001, VS-004, VS-006, VS-008, VS-009, VS-010, VS-015, VS-016, VS-019, VS-020, VS-021 |
| Recovery | VS-003, VS-006, VS-013, VS-017, VS-020, VS-021 |
| User Surface | VS-001, VS-002, VS-004, VS-005, VS-006, VS-007, VS-010, VS-011, VS-012, VS-013, VS-014, VS-016, VS-017, VS-019, VS-020, VS-021 |
| Cross-CLI Adaptation | VS-001, VS-005, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, VS-021 |

The roadmap's "Deferred Subsystems" line ("No proposal subsystem is strategically deferred") matches the slice content. The single deferral is Phase 0 engineering foundations, which has specific rationale: "those are not detailed here because they are not independently operator-visible value slices." That rationale is concrete (operator-visibility, not vague "later").

**Recommendation:** No action required.

---

### F-2. All eighteen problem axes are addressed

**Severity: INFO**

Each axis in `problem.md` (§1..§18) is addressed by at least one slice. Many axes are addressed by multiple slices that operate on different parts of the failure mode (visibility, enforcement, recovery).

| Axis | Slice(s) | Engagement |
|---|---|---|
| §1 Effective working set vs. nominal context window | VS-001, VS-004, VS-008 | Render is `WorkingSetSnapshot`-bounded; pack/unpack tools deliberately manage `W`; budget caps the render. |
| §2 Summary contract: faithful unpacking | VS-002, VS-010 | Contract validation on visible nodes; optimizer regenerates contracts and stale markers. |
| §3 Concurrent optimizer mutation under foreground walking | VS-009, VS-010, VS-013 | Bounded turn lifecycle with snapshot reads + advisory optimizer requests; conflict records when merges fail. |
| §4 Stable identity across regeneration and reshape | VS-013, VS-012 | `IdentityEvent` forwarding; repack/reparent edits with stable IDs. |
| §5 Working-set policy: pinning, eviction, recursive unpack | VS-001, VS-008 | Render shows pinned/unpacked/evicted; `pack/unpack/focus/pin/unpin` tools. |
| §6 Hierarchical packing without bounded-depth precedent | VS-008, VS-012 | Unpacks/packs over hierarchy; optimizer repacks discover cross-references. |
| §7 Cross-CLI rendering asymmetry | VS-006, VS-015, VS-016, VS-017, VS-021 | `CapabilityFingerprint` is exposed and gates worker dispatch, question continuations, and reroute. |
| §8 Sub-agent supervision via subgraph slices | VS-015, VS-016, VS-018 | Slice dispatch with provider preflight; worker board; staged graph reintegration. |
| §9 User-question routing as graph-state routing | VS-017 | `QuestionArtifact` resumes the exact blocked worker continuation with child-acceptance tracking. |
| §10 Tool-call protocol state as first-class graph provenance | VS-003 | `ToolCallProvenance` and `EvidenceArtifact` capture call IDs, approvals, and side effects. |
| §11 Workflow-reviewer reliability for graph mutations | VS-019 | Reviewer is treated as evidence; cost-aware sampling policy; reviewer outputs cannot bypass deterministic gates. |
| §12 Instruction hierarchy and memory poisoning becoming graph poisoning | VS-014 | Trust state and `poison_quarantined` lifecycle block traversal into suspect content. |
| §13 Cost, latency, and resource tails | VS-004 | `BudgetLedger` records and gates renders, optimizer passes, reviewer passes, and worker runs. |
| §14 Multi-workstream legibility | VS-007, VS-016, VS-017, VS-020 | Initiative roots, focus path, action-needed vs. passive-progress notifications, worker board, recovery surface. |
| §15 Imposed working context without surveyed precedent | VS-001 | Imposed `WorkingSetSnapshot` is the central inspectable artifact (the entire roadmap leans on this axis). |
| §16 Recovery and resume surfaces are not neutral | VS-020, VS-021, VS-013 | `RecoveryAction` with preflight, side-effect classification, preserved/replayed/discarded accounting; conflict records. |
| §17 Graph and memory configuration overhead | VS-005, VS-011 | `GraphConfiguration` inspector; configuration-shape advisory optimizer requests. |
| §18 Provider, account, and entitlement friction | VS-006, VS-021 | `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`; provider-aware reroute with explicit contract change. |

**Recommendation:** No action required.

---

### F-3. Two cross-cutting concerns are addressed implicitly but not named in the roadmap

**Severity: LOW**

Of the six cross-cutting concerns, four are addressed by named slices or named opportunity-cost lines:

- **Privilege and Poisoning Controls** — VS-014 is dedicated to this; the opportunity-cost summary names "Privilege and poisoning controls."
- **Configuration as Memory Semantics** — VS-005 (inspect) and VS-011 (simulate / repair request) are dedicated; the opportunity-cost summary names it.
- **Provider State as Observable State** — VS-006 (preflight + capability fingerprints) and VS-021 (provider-aware reroute); the opportunity-cost summary names it.
- **Audit and Reversibility** — `AuditEvent` is in the `Schema objects` list of every slice from VS-001 through VS-021; the opportunity-cost summary repeatedly cites the user's ability to inspect and trace state. The proposal's `Audit and Reversibility` section is enforced through pervasive append-only revisions and `RecoveryAction`-as-revert-mechanism, which the roadmap inherits via shared schema.

Two concerns are addressed implicitly only:

- **Local Control of State** — The Phase 0 placeholder mentions "local process boundaries," and every slice's schema objects are local records (`GraphSnapshot`, `BudgetLedger`, `ProviderState`, etc., all stored locally per the proposal's `HarnessBackend` and `GraphStore`). The roadmap does not state local control as a roadmap-level commitment, even though the proposal does. A reader who only sees `executive-roadmap.md` would have to infer from Phase 0 plus the schema-object list. The concern is handled — but the only named commitment the operator gets in the executive roadmap is buried in one Phase 0 placeholder phrase.
- **No In-Product Compaction** — The proposal's `No In-Product Compaction` section is what makes the working-set discipline load-bearing: the harness replaces `/compact`, and an in-product compaction by the underlying CLI is treated as session corruption. The roadmap's VS-001 (inspect renders), VS-004 (budget ledgers + cache locality), and VS-008 (pack/unpack/focus) collectively impose the discipline that replaces in-product compaction, but the roadmap does not name "no in-product compaction" as a constraint, does not flag it in Phase 0, and does not record what happens when an underlying CLI compacts anyway. The constraint is handled in substance but invisible in the roadmap text.

Per the severity criteria, "A cross-cutting concern is ignored" is MEDIUM. Neither concern is *ignored* — both are operationally addressed — so this stays LOW. But the roadmap's visibility surface is weaker than the proposal's commitment, which is a small completeness gap.

**Recommendation:** When the executive roadmap is next revised (or when the engineering roadmap is produced), name `Local Control of State` and `No In-Product Compaction` as Phase 0 (or pervasive) constraints, with one-line rationale each. The substantive coverage is already present; making it visible removes a class of "did you forget about X?" questions in the human gate and downstream layers.

---

### F-4. All twenty-one value slices pass the standalone-value test

**Severity: INFO**

For each slice, an operator gains a new observable or interactive capability after the slice ships:

| Slice | New operator-visible capability after ship |
|---|---|
| VS-001 | Operator can inspect the exact `WorkingSetSnapshot` (graph, working-set, summaries, evidence, token estimate, cache prefix, route) imposed on the lead turn. |
| VS-002 | Visible summaries are either contract-valid or labeled invalid; operator can see why. |
| VS-003 | Operator can trace any tool result to its `ToolCallProvenance` and `EvidenceArtifact`; audit events are queryable. |
| VS-004 | Operator sees per-render, per-turn, per-worker, per-optimizer-pass, per-reviewer-pass cost and policy action. |
| VS-005 | Operator can ask "what would happen on an empty graph?" and "why is the graph in this shape?" before trusting it. |
| VS-006 | Operator and orchestrator see which provider routes are eligible/degraded/blocked/unknown for a workload, with denial reasons. |
| VS-007 | Single-tab UI shows initiative roots, focus, pins, blockers, action-needed vs. passive-progress notifications. |
| VS-008 | Orchestrator gains agency: `pack`, `unpack`, `focus`, `pin`, `unpin` change the next render in observable ways. |
| VS-009 | Lead orchestrator can complete a durable turn that records bounded foreground actions and queues optimizer requests. |
| VS-010 | Operator sees accepted, rejected, and conflicted optimizer edits in the optimizer log; stale markers appear. |
| VS-011 | Configuration warnings convert into advisory `OptimizerRequest` artifacts visible in the optimizer log. |
| VS-012 | Operator sees discovered cross-references and bounded hierarchy repacks; old detail remains findable without flooding `W`. |
| VS-013 | `ConflictRecord` and `IdentityEvent` artifacts are visible; merges and identity forwarding are auditable. |
| VS-014 | Lower-privilege content is labeled and quarantined in renders; promotion paths are blocked with audit. |
| VS-015 | Orchestrator can launch sub-agent slices with provider preflight, write scope, and acceptance tracking. |
| VS-016 | Worker board shows slices, sessions, acceptance, run state, capability fingerprint, and budget. |
| VS-017 | Operator can answer worker questions and resume the exact blocked continuation with child-acceptance tracking. |
| VS-018 | Worker outputs appear as graph candidates, evidence, blockers, conflicts, and advisory optimizer requests, not as final-prose paste. |
| VS-019 | Reviewer flags appear as evidence on high-consequence edits; sampling decisions are visible. |
| VS-020 | Recovery actions are visible with preconditions, affected state, side-effect class, and preserved/replayed/discarded records. |
| VS-021 | Failed routes can be rerouted or substituted only after capability/cost/side-effect/user-confirmation checks are visible. |

No slice is purely internal infrastructure; each is operator-visible (or, for VS-008 and VS-009, orchestrator-visible, which is what those slices are scoped to deliver).

**Recommendation:** No action required.

---

### F-5. Phase 0 deferral rationale is specific

**Severity: INFO**

Phase 0 ("Foundations — engineering-defined") is the only deferral in the roadmap. Its rationale ("those are not detailed here because they are not independently operator-visible value slices") is specific and concrete, matching the standalone-value test the roadmap applies to value slices. The rationale also enumerates plausible content (storage, eventing, migrations, local process boundaries, testing substrate, app shell wiring) so the engineering layer has a non-empty starting point.

**Recommendation:** No action required. (The unnamed cross-cutting concerns in F-3 would naturally land in Phase 0 if they were called out.)

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| F-1 | All ten subsystems covered by at least one slice; "Deferred Subsystems" section is consistent with slice content. | INFO |
| F-2 | All eighteen problem axes (§1..§18) are addressed by at least one slice. | INFO |
| F-3 | `Local Control of State` and `No In-Product Compaction` are addressed in substance but not named in the roadmap text. | LOW |
| F-4 | All twenty-one value slices pass the standalone-value test (operator- or orchestrator-visible after ship). | INFO |
| F-5 | Phase 0 deferral has specific operator-visibility rationale; not a vague "later." | INFO |

## What LOW requires

The LOW rating is valid as long as all of the following hold:

1. Every one of the ten subsystems remains present in at least one of VS-001..VS-021. Removing or renaming a subsystem in `proposal.md` without updating the slice mapping invalidates this condition.
2. Every one of the eighteen problem axes (§1..§18) remains addressed by at least one slice. New axes added to `problem.md` must either be mapped to an existing slice or trigger a slice addition.
3. The four named cross-cutting concerns (`Privilege and Poisoning Controls`, `Audit and Reversibility`, `Configuration as Memory Semantics`, `Provider State as Observable State`) remain handled by the slices currently mapped to them; no slice is removed without redistributing the concern.
4. `Local Control of State` and `No In-Product Compaction` remain operationally enforced through the Phase 0 placeholder and the VS-001 / VS-004 / VS-008 working-set + budget discipline. If those slices are reordered such that the discipline is not in place before worker dispatch (VS-015), or if the Phase 0 placeholder is rewritten to drop "local process boundaries," this condition fails and severity rises to MEDIUM.
5. Every value slice (including any added during revision) continues to pass the standalone-value test. A purely-internal infrastructure slice presented as a value slice rather than Phase 0 invalidates this condition.
6. Phase 0 retains a specific operator-visibility rationale for any items it defers. A roadmap revision that turns Phase 0 into a vague "foundations later" placeholder invalidates this condition.
