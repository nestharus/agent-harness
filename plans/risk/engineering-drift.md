# Engineering — Drift Risk Assessment

**Rating: LOW**

This assessment treats `philosophy.md` and `proposal.md` Constraints/Non-Goals as the locked-decision substitute for `DECISIONS.md`, per the operator override. The engineering roadmap (`product-strategy/engineering-roadmap.md`, 716 lines) preserves all 21 executive value slices, documents every deviation from executive ordering as a Pushback Summary entry framed as a cost/value tradeoff, expands Phase 0 into 0A/0B/0C as foundation-only substrate, and preserves each slice's anti-scope. No silent additions, drops, reinterpretations, or scope reductions were detected.

## Findings

### F-1. All 21 executive value slices are covered with operator-visible capability preserved

**Severity: NONE**

Coverage check (executive `executive-roadmap.md` §"Value Slice Inventory" → engineering `engineering-roadmap.md` §"Initiative Assessments"):

| Exec slice | Exec phase | Engineering treatment | Operator capability preserved? |
|---|---|---|---|
| VS-001 Inspect imposed working-set renders | Phase 1 | §"VS-001" L (XL if Phase 0 incomplete); "working-set render inspection, using the real evidence/budget/provider/config fields" | Yes — render of snapshot, working-set, summaries, evidence pointers, token estimate, cache prefix hash, provider/capability route |
| VS-002 Enforce summary contracts on visible nodes | Phase 1 | §"VS-002" M; "summary contract validation and invalid/stale render labels" | Yes — packed nodes render only through contract-valid summaries |
| VS-003 Capture tool-call provenance and audit events | Phase 1 | §"VS-003" L; "tool-call lifecycle capture, side-effect classification, approval state, evidence blobs, audit writer, drill-down UI" | Yes — first-class tool-call provenance |
| VS-004 Gate renders with budget ledgers and cache locality | Phase 1 | §"VS-004" M; "per-scope budget records, token/cache estimation, policy transitions, cost surface, blocked render handling" | Yes — token/latency/cost/cache prefix hash recorded across renders, turns, workers, optimizer, reviewers |
| VS-005 Inspect configuration as memory semantics | Phase 1 | §"VS-005" M; "Registry, default/inherited/source maps, empty-graph simulation, shape explanation, validation warnings, inspector pane" | Yes — full inspection lifecycle |
| VS-006 Preflight providers and expose capability fingerprints | Phase 1 | §"VS-006" M; "Redacted probes, entitlement model, capability matrix, route eligibility logic, denial reasons, provider panel" | Yes — eligible/degraded/blocked/unknown route labels with denial reasons |
| VS-007 Show initiative roots and current focus | Phase 1 | §"VS-007" M; "Initiative/focus UI, status selectors, subscriptions, notification classification, seeded fixtures" | Yes — initiative roots, focus path, action-needed vs passive-progress notifications |
| VS-008 Navigate with pack, unpack, focus, pin, and unpin | Phase 2 | §"VS-008" M; "Tool command schema, policy enforcement, state transitions, next-render invalidation" | Yes — bounded graph-walk tools whose effects appear on next render |
| VS-009 Run bounded orchestrator turns with advisory optimizer requests | Phase 2 | §"VS-009" L; "Durable turn orchestration, capture/commit transactions, advisory request queueing, /compact detection/avoidance" | Yes — durable turns, bounded foreground graph actions, advisory optimizer requests |
| VS-010 Refresh summaries and mark stale nodes | Phase 2 | §"VS-010" L; "Optimizer scoping, prompt/response schema, edit validation, merge attempts, conflict-on-stale-base, summary/stale UI" | Yes — budgeted summary/stale edits visible in optimizer log |
| VS-011 Simulate configuration effects and request shape repair | Phase 3 | §"VS-011" M; "Warning categories, anomaly detectors, request creation flow, UI linkage" | Yes — configuration warnings produce advisory optimizer requests |
| VS-012 Repack hierarchy and discover cross-references | Phase 3 | §"VS-012" L; "Topology edit operations, cross-reference discovery, repack planning, identity forwarding application, conflict generation, UI inspection" | Yes — cross-references and bounded hierarchy repack |
| VS-013 Resolve snapshot-merge conflicts and identity forwarding | Phase 3 | §"VS-013" L; "Merge precondition checks, stale-base conflict detection, forwarding-map resolver, conflict state machine, user/orchestrator conflict surfaces" | Yes — fail-closed conflicts with affected nodes, identity events, resolution state |
| VS-014 Quarantine poisoned or privilege-unsafe graph content | Phase 3 | §"VS-014" L; "Quarantine edit type behavior, privilege transforms, render blocks, validator fixtures, drill-down UI" | Yes — privilege labeling, render quarantine, evidence/policy traceability |
| VS-015 Dispatch provider-aware worker slices | Phase 4 | §"VS-015" L; "Slice validation, provider-aware route approval, prompt/render for worker slices, acceptance tracking, evidence ingestion, launch UI/control path" | Yes — bounded sub-agent slices via `agents` with graph context, write scope, provider preflight, budget, acceptance tracking |
| VS-016 Show the worker board and trace evidence | Phase 5 | §"VS-016" M; "Board filters, status selectors, trace/evidence mapping, capability/budget badges, ambiguity states" | Yes — worker slices, sessions, accepted/rejected/blocked/running/completed, fingerprints, budget, evidence |
| VS-017 Route NEEDS_INPUT answers as worker continuations | Phase 5 | §"VS-017" L; "NEEDS_INPUT envelope parsing, queue UI, answer artifact writer, acceptance watcher, failed-resume integration" | Yes — global action-needed queue, exact blocked-worker resume, child-acceptance tracking |
| VS-018 Stage worker output for graph reintegration | Phase 5 | §"VS-018" L; "Staged candidate model, parser/mappers, conflict-on-overlap logic, advisory optimizer request generation, staging UI" | Yes — graph candidates, evidence, blockers, conflicts, advisory optimizer requests |
| VS-019 Sample reviewers over high-consequence edits | Phase 6 | §"VS-019" M; "Sampling decisions, prompt packer, result schema, UI flags, budget-ledger integration" | Yes — reviewers as cost-aware evidence channel |
| VS-020 Surface recovery preflight and state accounting | Phase 6 | §"VS-020" L; "Preflight engine, preserved/replayed/discarded accounting, side-effect classifier, recovery surface, reconciliation transactions" | Yes — recovery actions with preconditions, affected state, side-effect class, preserved/replayed/discarded records |
| VS-021 Reroute or substitute failed provider/worker runs explicitly | Phase 7 | §"VS-021" M; "Route comparison, substitution planning, confirmation flow, contract-diff display, audit linkage" | Yes — reroute/substitute with capability/cost/side-effect/confirmation visibility |

No slice is missing. No slice is silently dropped. No slice is reinterpreted in a way that loses operator-visible capability. The engineering roadmap retains the same `WorkingSetSnapshot`, `BudgetLedger`, `SummaryContract`, `OptimizerEdit`, `ConflictRecord`, `WorkerSlice`, `QuestionArtifact`, and `RecoveryAction` operator surfaces named by the executive roadmap.

**Recommendation:** No action required.

---

### F-2. All five engineering reorderings are documented as Pushback Summary entries with cost/value framing

**Severity: NONE**

The engineering roadmap deviates from executive ordering in five places, all collected under §"Pushback Summary":

| Pushback | Executive ordering | Engineering recommendation | Cost framing | Value-lost framing |
|---|---|---|---|---|
| P-1 (engineering-roadmap.md §"Pushback P-1") | "Phase 1 slices begin after an engineering-defined Phase 0 placeholder with no detailed scope" | "Treat Phase 0 as an XL substrate phase containing GraphStore/migrations, evidence/audit, RenderEngine core, PolicyEngine, BudgetLedger core, ProviderState/CapabilityFingerprint records, CLI supervisor, IPC, UI shell, and test fixtures" | "If VS-001 through VS-007 are built directly from the empty worktree, each slice will invent its own DTOs, schema migrations, render result states, audit calls, and UI event streams" | "Operators wait longer before seeing the first working-set inspector" |
| P-2 (§"Pushback P-2") | "VS-001 inspect renders appears before VS-003 provenance/audit and VS-004 budget/cache" | "Implement the VS-003 evidence/audit backbone and VS-004 budget/cache ledger fields before VS-001 acceptance" | "If the inspector ships before those IDs and ledgers exist, it will show a temporary render shape and later require migration of snapshot rows, UI drill-down, test goldens, and policy decisions" | "The earliest inspector milestone may show less UI sooner, because backend evidence and budget plumbing must land first" |
| P-3 (§"Pushback P-3") | "VS-012 repack/cross-reference appears before VS-013 conflict and identity forwarding in Phase 3" | "Build VS-013's identity resolver, optimistic merge engine, and `ConflictRecord` state machine before enabling VS-012 topology edit types" | "Building VS-012 first either produces throwaway merge logic or risks topology changes that cannot fail closed" | "Operators wait longer for cross-reference discovery and hierarchy repack" |
| P-4 (§"Pushback P-4") | "Full recovery accounting is VS-020 in Phase 6, after NEEDS_INPUT routing and worker reintegration" | "Keep operator-visible recovery in VS-020, but implement `RecoveryAction` schema, side-effect taxonomy, and audit linkage in Phase 0, and require a failed-resume handoff path before VS-017 acceptance" | "Without recovery records, failed continuations become ad hoc error states that later need migration into VS-020's preserved/replayed/discarded accounting" | "VS-017 may take longer because it must write durable recovery handoff records even though full recovery UI arrives later" |
| P-5 (§"Pushback P-5") | "VS-006 is one Phase 1 slice among seven" | "Define `ProviderState`, `EntitlementSnapshot`, and `CapabilityFingerprint` schemas and denial-reason taxonomy in Phase 0, then deliver the operator-visible preflight panel as VS-006" | "VS-001 render route display, VS-004 budget interpretation, VS-009 orchestrator launch, VS-015 worker dispatch, VS-017 continuation, VS-020 recovery, and VS-021 reroute all consume provider/capability records. If VS-006 owns the first real schema late in Phase 1, earlier slices either fake route state or need retrofits" | "The VS-006 panel itself is not visible earlier; only its schema contract is" |

Each pushback states implementation cost in concrete terms (duplicate substrate, migration cost, throwaway merge logic, ad hoc error states, retrofits) and a specific value-lost statement (delayed inspector, delayed cross-references, longer VS-017 build, no earlier panel). None is framed as "the executive ordering is wrong"; all are framed as "the executive ordering costs X because Y." This satisfies the proposer's Step 4 pushback contract.

**Recommendation:** No action required.

---

### F-3. Phase 0 expansion is foundation-only and does not add operator-visible capability outside executive scope

**Severity: NONE**

The executive roadmap (`executive-roadmap.md` §"Phase 0: Foundations") explicitly defers Phase 0 to engineering with the note "this phase may include storage, eventing, migrations, local process boundaries, testing substrate, and app shell wiring, but those are not detailed here because they are not independently operator-visible value slices." The two pervasive constraints are "Local Control of State" and "No In-Product Compaction."

The engineering Phase 0 expands into 0A/0B/0C (`engineering-roadmap.md` §"Engineering Phase Structure"):

- **0A "Repository and Runtime Skeleton"** — explicitly stated: "This phase has no operator-visible value beyond a running shell." The 21 enumerated foundation items (§"Foundation Phase" table) cover the Tauri/Turbo/React monorepo scaffold, Rust/Tokio backend skeleton, settings/storage roots, IPC commands, UI shell, and logging/tracing — all infrastructure.
- **0B "Canonical State and Contracts"** — migrations and base GraphStore tables. Explicitly stated: "Implement repository boundaries and fixture builders, but keep features inert unless a value slice turns them on."
- **0C "Shared Engines and Integration Shells"** — RenderEngine core interfaces, PolicyEngine gate framework, CLI subprocess supervisor, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, identity/conflict shell, seeded UI panes. Explicitly stated: "no value-slice-specific claims such as 'workers can launch' or 'optimizer refreshes summaries.'"

Schemas that look like operator-facing capability are deliberately ring-fenced as foundation only:
- "WorkerSlice/WorkerRun schema and state contract, **without launch UI**" — launch is delivered in VS-015.
- "QuestionArtifact schema and continuation-envelope contract, **without full queue UI**" — queue is delivered in VS-017.
- "RecoveryAction schema, side-effect classification taxonomy, and audit linkage" — operator-visible recovery is delivered in VS-020 (P-4 makes this explicit: "Keep operator-visible recovery in VS-020").
- "ProviderStateMonitor and CapabilityFingerprint record model" — operator preflight panel is delivered in VS-006 (P-5: "deliver the operator-visible preflight panel as VS-006").
- "Optimizer queue, OptimizerRequest / OptimizerEdit store, and merge-validation shell" — actual optimizer cycles are delivered in VS-010, VS-011, VS-012, VS-013.

The two executive Phase 0 pervasive constraints are preserved:
- **Local Control of State** — `engineering-roadmap.md` §"Technical Landscape Summary" names "local SQLite data model" as the canonical store; Phase 0B places GraphStore migrations locally; no value-slice claims a vendor session store as critical state.
- **No In-Product Compaction** — explicitly preserved at VS-009 ("`/compact` detection/avoidance"); the engineering roadmap does not add `/compact` semantics or compaction-equivalent behavior at any slice.

**Recommendation:** No action required.

---

### F-4. Anti-scope is preserved across all 21 slices

**Severity: NONE**

Each engineering slice's "What is new" was checked against the executive slice's "Anti-scope." No silent inclusion of anti-scope items was detected. Selected confirmations:

- VS-001 anti-scope ("Does not include pack/unpack/focus tools, optimizer edits, worker dispatch, routed questions, or recovery actions"): engineering VS-001 lists "renderer, snapshot store, render blob storage, token/cache estimator, IPC commands, inspector pane, audit integration" — none cross into anti-scope.
- VS-002 anti-scope ("Does not regenerate summaries, discover new cross-references, repair provenance, or change graph topology"): engineering VS-002 limits itself to validators, template-source handling, evidence locator checks, UI labels, render blocking — no regeneration.
- VS-003 anti-scope ("Does not authorize topology mutation, worker reintegration, reviewer sampling, or recovery replay"): engineering VS-003 limits itself to capture/classification/audit; recovery replay is VS-020.
- VS-005 anti-scope ("Does not auto-rewrite graph shape, regenerate summaries, repair indexes, or choose provider routes"): engineering VS-005 explicitly lists shape *explanation* and validation *warnings*, not rewrites; configuration warnings only become advisory requests in VS-011.
- VS-006 anti-scope ("Does not store credentials, repair vendor accounts, dispatch workers, or perform recovery reroutes"): engineering VS-006 names "redaction" and "not storing secrets" as risk factors, dispatch is VS-015, reroute is VS-021.
- VS-008 anti-scope ("Does not mutate graph topology, summaries, cross-references, evidence, identity, or worker assignments"): engineering VS-008 limits itself to `AgentWalkState` mutation; topology mutation is VS-012.
- VS-009 anti-scope ("Does not include actual optimizer edits, worker dispatch, question routing, or recovery execution"): engineering VS-009 explicitly limits to "advisory request queueing"; optimizer edits are VS-010, dispatch is VS-015.
- VS-010 anti-scope ("Does not split, merge, re-parent, repack, discover cross-references, quarantine poisoned nodes, or reintegrate workers"): engineering VS-010 explicitly bounds the slice to "summary/stale edit types"; topology is VS-012; quarantine is VS-014; reintegration is VS-018.
- VS-014 anti-scope ("Does not make reviewers authoritative, delete evidence, or provide enterprise team access-control features"): engineering VS-014 limits itself to "Quarantine edit type behavior, privilege transforms, render blocks, validator fixtures, drill-down UI"; no evidence deletion or team ACL is named.
- VS-015 anti-scope ("Does not reintegrate output as graph truth, route user questions, sample reviewers, or perform recovery substitution"): engineering VS-015 explicitly lists "evidence ingestion, launch UI/control path"; reintegration is VS-018; routing is VS-017; reviewers are VS-019; substitution is VS-021.
- VS-017 anti-scope ("Does not append answers to the lead transcript as a substitute, resolve graph conflicts, or fresh-substitute workers after failed resume"): engineering VS-017's "failed-resume integration" is the recovery *handoff record* per Pushback P-4 ("durable recovery handoff records"), not fresh substitution; fresh substitution remains VS-021.
- VS-018 anti-scope ("Does not let workers directly mutate topology, summaries, identity, cross-references, quarantine state, or current orchestrator focus"): engineering VS-018 limits to "Staged candidate model, parser/mappers, conflict-on-overlap logic, advisory optimizer request generation, staging UI" — staged candidates do not directly mutate.
- VS-019 anti-scope ("Does not make reviewer approval sufficient for merge, review every mutation, or replace deterministic gates"): engineering VS-019 lists deterministic-gate-precedence as a risk factor and limits itself to sampling.
- VS-020 anti-scope ("Does not silently resume, substitute workers, or reroute providers without exposing changed execution contracts"): engineering VS-020 limits itself to preflight/accounting/reconciliation; substitution is VS-021.
- VS-021 anti-scope ("Does not repair vendor accounts, hide degraded execution contracts, or claim a fresh worker is a successful resume"): engineering VS-021 lists "fresh substitution honesty" as a risk factor and limits itself to comparator/planner/confirmation/contract-diff.

No anti-scope item is silently included.

**Recommendation:** No action required.

---

### F-5. Internal inconsistency between Critical Path narrative and Phase 1 ordering — internal only, not drift

**Severity: INFO**

The §"Critical Path" narrative reads:

> Phase 0 GraphStore / RenderEngine / PolicyEngine / Evidence / Budget / Provider / CLI supervisor / tests (XL) -> VS-001 inspect renders (L) -> VS-003 provenance and VS-004 budget gates (L/M, needed before turns are trustworthy) -> VS-009 bounded orchestrator turns (L) ...

The §"Engineering Phase Structure" Phase 1 ordering reads:

> 1. VS-003 backend evidence/audit spine and VS-004 budget/cache ledger primitives.
> 2. VS-001 working-set render inspection, using the real evidence/budget/provider/config fields.

Pushback P-2 explicitly says VS-003/VS-004 backends land before VS-001 acceptance. The narrative chain places VS-001 before VS-003/VS-004, which contradicts P-2. The §"Dependency Graph" listing ("VS-001, VS-003, VS-004, VS-006 -> VS-009") is consistent with P-2 because it shows VS-009 consuming all four, not because it places VS-001 ahead of VS-003/VS-004.

This is an internal narrative inconsistency, not drift from the executive roadmap. It does not change which executive slice is delivered or its operator-visible capability — VS-001 is still the first operator-visible inspector slice, and VS-003/VS-004 backends are still required before VS-001 ships. The Strategic Integration risk gate is the appropriate place to surface internal-narrative consistency concerns; this is logged here as INFO so the Drift gate is not silently using it as evidence of executive-roadmap deviation.

**Recommendation:** No action required from a drift perspective. Surfacing here only because the Drift gate read both sections.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| F-1 | All 21 executive value slices covered with operator-visible capability preserved | NONE |
| F-2 | All five engineering reorderings documented as cost/value-framed pushback (P-1 through P-5) | NONE |
| F-3 | Phase 0 expansion (0A/0B/0C) is foundation-only; pervasive constraints (Local Control of State, No In-Product Compaction) preserved | NONE |
| F-4 | Anti-scope preserved across all 21 slices | NONE |
| F-5 | Internal narrative inconsistency between Critical Path and Phase 1 ordering | INFO |

## What LOW requires

For this LOW rating to remain valid, the following conditions must hold:

1. **Coverage complete.** Every one of VS-001 through VS-021 continues to appear by ID in `engineering-roadmap.md` §"Initiative Assessments," each with a "What is new" list that delivers the operator-visible capability named in `executive-roadmap.md` §"Value Slice Inventory" for the same VS-NNN.
2. **Reorderings remain documented.** The five deviations from executive ordering (Phase 0 expansion, VS-003/VS-004 backend before VS-001 acceptance, VS-013 before VS-012, RecoveryAction schema in Phase 0, ProviderState/CapabilityFingerprint schema in Phase 0) remain enumerated in `engineering-roadmap.md` §"Pushback Summary" with the implementation-cost statement, the value-lost statement, and the explicit recommendation. No new deviation may be introduced without a new Pushback entry.
3. **Phase 0 stays foundation-only.** No item in `engineering-roadmap.md` §"Foundation Phase" promotes itself from substrate (schema, interface, registry, shell, fixtures) to operator-visible capability that the executive roadmap reserved for a value slice. In particular: WorkerSlice schema in Phase 0 retains "without launch UI"; QuestionArtifact schema retains "without full queue UI"; RecoveryAction schema retains "without recovery UI"; OptimizerRequest/Edit store retains "merge-validation shell" rather than active optimizer cycles; ProviderStateMonitor retains "record model" rather than the operator-visible preflight panel.
4. **Anti-scope continues to be honored.** No engineering slice's "What is new" list expands to include items the corresponding executive slice declared anti-scope (e.g., VS-009 must not begin authoring optimizer edits; VS-014 must not delete evidence; VS-017 must not fresh-substitute workers; VS-020 must not silently resume without exposing changed contracts).
5. **Pervasive constraints from executive Phase 0 remain intact.** "Local Control of State" must continue to be reflected in local SQLite ownership across all slices. "No In-Product Compaction" must remain enforced — VS-009 must continue to detect/avoid `/compact`, and no later slice may reintroduce vendor-side compaction or in-product compaction equivalents.
6. **Pushback framing stays cost/value, not "executive is wrong."** Each Pushback Summary entry must continue to state implementation cost in concrete terms and the value lost by the engineering ordering. None may shift to a framing that argues the executive roadmap's strategic ordering is incorrect rather than carrying a higher implementation cost.
