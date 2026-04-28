# Executive — Completeness Risk Assessment

**Rating: LOW**

This is round-2 of the executive completeness gate. It re-assesses `executive-roadmap.md` after the brownfield cleanup commit (`dc56471`) that responded to round-1's three LOW advisories. The assessment uses the agent-harness-specific override stated in the prompt: ten subsystems (Context Graph & Provenance, Orchestrator, Continuous Optimizer, Sub-Agent Dispatch & Reintegration, NEEDS_INPUT Routing, Workflow Review & Governance, Cost & Budget, Recovery, User Surface, Cross-CLI Adaptation), six cross-cutting concerns (Privilege and Poisoning Controls, Audit and Reversibility, Local Control of State, No In-Product Compaction, Configuration as Memory Semantics, Provider State as Observable State), and the eighteen problem axes in `problem.md` §1..§18.

All ten subsystems are covered by at least one of VS-001..VS-021. All eighteen problem axes are addressed. All six cross-cutting concerns are now visibly handled: four through dedicated slices, plus `Local Control of State` and `No In-Product Compaction` as explicit Phase 0 pervasive constraints (lines 191-194). Every value slice passes the standalone-value test. The Phase 0 deferral retains specific operator-visibility rationale. No round-2 advisory rises above INFO.

## Findings

### F-1. All ten subsystems are covered

**Severity: INFO**

Each of the ten subsystems appears in the `Subsystem(s)` line of at least one value slice. The roadmap's "Deferred Subsystems" section enumerates the mapping; cross-checking each slice's `Subsystem(s)` line confirms it.

| Subsystem | Slices |
|---|---|
| Context Graph & Provenance | VS-001, VS-002, VS-003, VS-005, VS-007, VS-008, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-015, VS-018, VS-020 |
| Orchestrator | VS-001, VS-003, VS-004, VS-006, VS-007, VS-008, VS-009, VS-012, VS-015 |
| Continuous Optimizer | VS-004, VS-005, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-018, VS-019 |
| Sub-Agent Dispatch & Reintegration | VS-004, VS-006, VS-015, VS-016, VS-017, VS-018, VS-019, VS-020, VS-021 |
| NEEDS_INPUT Routing | VS-017, VS-020 |
| Workflow Review & Governance | VS-002, VS-003, VS-008, VS-009, VS-010, VS-011, VS-013, VS-014, VS-018, VS-019 |
| Cost & Budget | VS-001, VS-004, VS-006, VS-008, VS-009, VS-010, VS-015, VS-016, VS-019, VS-020, VS-021 |
| Recovery | VS-003, VS-006, VS-013, VS-017, VS-020, VS-021 |
| User Surface | VS-001, VS-002, VS-004, VS-005, VS-006, VS-007, VS-010, VS-011, VS-012, VS-013, VS-014, VS-016, VS-017, VS-019, VS-020, VS-021 |
| Cross-CLI Adaptation | VS-001, VS-005, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, VS-021 |

The roadmap's "Deferred Subsystems" line ("No proposal subsystem is strategically deferred") is consistent with the slice content. The single deferral is Phase 0 engineering foundations, with specific rationale: "those are not detailed here because they are not independently operator-visible value slices." That rationale is concrete (operator-visibility, not vague "later").

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
| §5 Working-set policy: pinning, eviction, recursive unpack | VS-001, VS-008 | Render shows pinned/unpacked/evicted nodes; `pack/unpack/focus/pin/unpin` tools. |
| §6 Hierarchical packing without bounded-depth precedent | VS-008, VS-012 | Unpacks/packs over hierarchy; optimizer repacks discover cross-references. |
| §7 Cross-CLI rendering asymmetry | VS-006, VS-015, VS-016, VS-017, VS-021 | `CapabilityFingerprint` exposed and gates worker dispatch, question continuations, and reroute. |
| §8 Sub-agent supervision via subgraph slices | VS-015, VS-016, VS-018 | Slice dispatch with provider preflight; worker board; staged graph reintegration. |
| §9 User-question routing as graph-state routing | VS-017 | `QuestionArtifact` resumes the exact blocked worker continuation with child-acceptance tracking. |
| §10 Tool-call protocol state as first-class graph provenance | VS-003 | `ToolCallProvenance` and `EvidenceArtifact` capture call IDs, approvals, and side effects. |
| §11 Workflow-reviewer reliability for graph mutations | VS-019 | Reviewer treated as evidence; cost-aware sampling policy; reviewer outputs cannot bypass deterministic gates. |
| §12 Instruction hierarchy and memory poisoning becoming graph poisoning | VS-014 | Trust state and `poison_quarantined` lifecycle block traversal into suspect content. |
| §13 Cost, latency, and resource tails | VS-004 | `BudgetLedger` records and gates renders, optimizer passes, reviewer passes, and worker runs. |
| §14 Multi-workstream legibility | VS-007, VS-016, VS-017, VS-020 | Initiative roots, focus path, action-needed vs. passive-progress notifications, worker board, recovery surface. |
| §15 Imposed working context without surveyed precedent | VS-001 | Imposed `WorkingSetSnapshot` is the central inspectable artifact (the entire roadmap leans on this axis). |
| §16 Recovery and resume surfaces are not neutral | VS-013, VS-020, VS-021 | `RecoveryAction` with preflight, side-effect classification, preserved/replayed/discarded accounting; conflict records. |
| §17 Graph and memory configuration overhead | VS-005, VS-011 | `GraphConfiguration` inspector; configuration-shape advisory optimizer requests. |
| §18 Provider, account, and entitlement friction | VS-006, VS-021 | `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`; provider-aware reroute with explicit contract change. |

**Recommendation:** No action required.

---

### F-3. All six cross-cutting concerns are now visibly handled (round-1 LOW advisory resolved)

**Severity: INFO**

Round-1 of this gate flagged that two cross-cutting concerns (`Local Control of State` and `No In-Product Compaction`) were operationally addressed but not named in the roadmap text. The brownfield cleanup commit (`dc56471`) added both as explicit Phase 0 pervasive constraints (lines 191-194), with one-line rationale each:

- "Local Control of State: Graph and provenance state live under the user filesystem or local SQLite owned by the harness; no critical state lives only in vendor session stores. This constraint is pervasive across every slice."
- "No In-Product Compaction: The harness replaces `/compact` and equivalents with explicit graph discipline: imposed working set, summary contracts, provenance, and optimizer maintenance. This constraint is not interoperable with vendor-side compaction and is pervasive across every slice that touches CLI sessions."

This resolves the round-1 LOW advisory. All six cross-cutting concerns are now addressed visibly:

| Cross-cutting concern | How addressed in roadmap |
|---|---|
| Privilege and Poisoning Controls | VS-014 dedicated slice; opportunity-cost summary names "Privilege and poisoning controls"; trust-state lifecycle in slice schema. |
| Audit and Reversibility | `AuditEvent` is in the `Schema objects` list of every slice from VS-001..VS-021; conflict records (VS-013) and recovery actions (VS-020) provide the reversibility surface; the proposal's "Audit and Reversibility" governance section is enforced through pervasive append-only revisions inherited via shared schema. |
| Local Control of State | Phase 0 pervasive constraint (lines 191-194); enforced by every slice's local-record schema (`GraphSnapshot`, `BudgetLedger`, `ProviderState`, etc.). |
| No In-Product Compaction | Phase 0 pervasive constraint (lines 191-194); enforced operationally by VS-001 (inspect renders), VS-004 (budget + cache locality), VS-008 (pack/unpack/focus). |
| Configuration as Memory Semantics | VS-005 (inspect) and VS-011 (simulate / repair request); opportunity-cost summary names it. |
| Provider State as Observable State | VS-006 (preflight + capability fingerprints) and VS-021 (provider-aware reroute); opportunity-cost summary names it. |

**Recommendation:** No action required.

---

### F-4. All twenty-one value slices pass the standalone-value test

**Severity: INFO**

For each slice, an operator (or, for tooling slices, the orchestrator) gains a new observable or interactive capability after the slice ships:

| Slice | New visible capability after ship |
|---|---|
| VS-001 | Operator can inspect the exact `WorkingSetSnapshot` (graph snapshot, working set, summaries, evidence pointers, token estimate, cache prefix, route) imposed on the lead turn. |
| VS-002 | Visible summaries are either contract-valid or labeled invalid; render blocks treating invalid summaries as settled state. |
| VS-003 | Operator can trace any tool result to its `ToolCallProvenance` and `EvidenceArtifact`; audit events are queryable. |
| VS-004 | Operator sees per-render, per-turn, per-worker, per-optimizer-pass, per-reviewer-pass cost, cache prefix hash, and policy action; budget gates fire on overruns. |
| VS-005 | Operator can ask "what would happen on an empty graph?" and "why is the graph in this shape?" before trusting it. |
| VS-006 | Operator and orchestrator see which provider routes are eligible/degraded/blocked/unknown for a workload, with explicit denial reasons. |
| VS-007 | Single-tab UI shows initiative roots, focus, pins, blockers, action-needed vs. passive-progress notifications. |
| VS-008 | Orchestrator gains agency: `pack`, `unpack`, `focus`, `pin`, `unpin` tools change the next render in observable ways. |
| VS-009 | Lead orchestrator can complete a durable turn that records bounded foreground actions and queues advisory `OptimizerRequest` artifacts. |
| VS-010 | Operator sees accepted, rejected, and conflicted optimizer summary edits in the optimizer log; stale markers appear. |
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

No slice is purely internal infrastructure; each is operator-visible (or, for VS-008 and VS-009, orchestrator-visible, which is what those slices are scoped to deliver). VS-008 and VS-009 are not pure-infrastructure slices because their effects propagate to the next render, which the operator inspects via VS-001 and VS-007.

**Recommendation:** No action required.

---

### F-5. Phase 0 deferral rationale is specific and distinct from value-slice content

**Severity: INFO**

Phase 0 ("Foundations — engineering-defined") is the only deferral in the roadmap. Its rationale ("those are not detailed here because they are not independently operator-visible value slices") is specific and concrete, matching the standalone-value test the roadmap applies to value slices. The rationale enumerates plausible content (storage, eventing, migrations, local process boundaries, testing substrate, app shell wiring) so the engineering layer has a non-empty starting point.

Phase 0 also now carries the two pervasive constraints from F-3 (`Local Control of State`, `No In-Product Compaction`), which are not engineering items per se — they are roadmap-level commitments that every slice inherits. Naming them under Phase 0 is appropriate because they are pre-conditions every later slice depends on without owning.

**Recommendation:** No action required.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| F-1 | All ten subsystems covered by at least one slice; "Deferred Subsystems" section is consistent with slice content. | INFO |
| F-2 | All eighteen problem axes (§1..§18) are addressed by at least one slice. | INFO |
| F-3 | All six cross-cutting concerns are now visibly handled; round-1 LOW advisory (`Local Control of State`, `No In-Product Compaction` not named) is resolved. | INFO |
| F-4 | All twenty-one value slices pass the standalone-value test (operator- or orchestrator-visible after ship). | INFO |
| F-5 | Phase 0 deferral has specific operator-visibility rationale and now also carries the two pervasive cross-cutting constraints. | INFO |

## What LOW requires

The LOW rating is valid as long as all of the following hold:

1. Every one of the ten subsystems remains present in at least one of VS-001..VS-021. Removing or renaming a subsystem in `proposal.md` without updating the slice mapping invalidates this condition.
2. Every one of the eighteen problem axes (§1..§18) remains addressed by at least one slice. New axes added to `problem.md` must either be mapped to an existing slice or trigger a slice addition.
3. The six cross-cutting concerns remain handled by their currently-mapped surfaces:
   - `Privilege and Poisoning Controls` by VS-014 (and the trust-state lifecycle in dependent slice schemas).
   - `Audit and Reversibility` by `AuditEvent` in every slice's schema list, plus `ConflictRecord` (VS-013) and `RecoveryAction` (VS-020).
   - `Configuration as Memory Semantics` by VS-005 and VS-011.
   - `Provider State as Observable State` by VS-006 and VS-021.
   - `Local Control of State` by the Phase 0 pervasive constraint and the local-record schema in every slice.
   - `No In-Product Compaction` by the Phase 0 pervasive constraint and the working-set + budget discipline in VS-001 / VS-004 / VS-008.
   Removing the Phase 0 pervasive-constraint paragraph, or removing `AuditEvent` from slice schemas, or dropping VS-014 / VS-005 / VS-011 / VS-006 / VS-021 without redistributing the concern, invalidates this condition.
4. Every value slice (including any added during revision) continues to pass the standalone-value test. A purely-internal infrastructure slice presented as a value slice rather than Phase 0 content invalidates this condition.
5. Phase 0 retains specific operator-visibility rationale for the engineering content it defers. A roadmap revision that turns Phase 0 into a vague "foundations later" placeholder invalidates this condition.
6. Slice ordering keeps the working-set + budget + cache-locality discipline (VS-001, VS-004, VS-008) in place before worker dispatch (VS-015), so that the `No In-Product Compaction` constraint is operationally enforceable when sub-agents start running. Reordering that lets workers dispatch before that discipline ships invalidates this condition.
