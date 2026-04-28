# Engineering — Drift Risk Assessment

**Rating: LOW**

This assessment treats `philosophy.md` and `proposal.md` Constraints/Non-Goals as the locked-decision substitute for `DECISIONS.md`, per the operator override. The engineering roadmap (`product-strategy/engineering-roadmap.md`, 716 lines) preserves all 21 executive value slices, documents every deviation from executive ordering as a Pushback Summary entry framed as a cost/value tradeoff, expands Phase 0 into 0A/0B/0C as foundation-only substrate, and preserves each slice's anti-scope. Round-2 brownfield revisions also reconciled the previously inconsistent Critical Path narrative with Pushback P-2. No silent additions, drops, reinterpretations, or scope reductions were detected.

## Findings

### F-1. All 21 executive value slices are covered with operator-visible capability preserved

**Severity: NONE**

Coverage check (executive `executive-roadmap.md` §"Value Slice Inventory" → engineering `engineering-roadmap.md` §"Initiative Assessments"):

| Exec slice | Exec phase | Engineering treatment | Operator capability preserved? |
|---|---|---|---|
| VS-001 Inspect imposed working-set renders | Phase 1 | §"VS-001" L (XL if Phase 0 incomplete); "renderer, snapshot store, render blob storage, token/cache estimator, IPC commands, inspector pane, audit integration" | Yes — render of snapshot, working-set, summaries, evidence pointers, token estimate, cache prefix hash, provider/capability route |
| VS-002 Enforce summary contracts on visible nodes | Phase 1 | §"VS-002" M; "Validators, template-source handling, evidence locator checks, UI invalid/stale labels, render blocking/labeling behavior" | Yes — packed nodes render only through contract-valid summaries |
| VS-003 Capture tool-call provenance and audit events | Phase 1 | §"VS-003" L; "Normalized tool-call lifecycle capture, side-effect classification, approval state, evidence blobs, audit writer, drill-down UI" | Yes — first-class tool-call provenance |
| VS-004 Gate renders with budget ledgers and cache locality | Phase 1 | §"VS-004" M; "Per-scope budget records, token/cache estimation, policy transitions, cost surface, blocked render handling" | Yes — token/latency/cost/cache prefix hash recorded across renders, turns, workers, optimizer, reviewers |
| VS-005 Inspect configuration as memory semantics | Phase 1 | §"VS-005" M; "Operator-visible configuration inspector, default/inherited/source maps, empty-graph simulation, shape explanation, configuration-warning emission, advisory `OptimizerRequest` creation, inspector-pane audit events" | Yes — full inspection lifecycle |
| VS-006 Preflight providers and expose capability fingerprints | Phase 1 | §"VS-006" M; "Redacted probes over the Phase 0 provider contracts, entitlement observations, capability matrix population, route eligibility logic, operator-visible denial reasons, provider panel" | Yes — eligible/degraded/blocked/unknown route labels with denial reasons |
| VS-007 Show initiative roots and current focus | Phase 1 | §"VS-007" M; "Initiative/focus UI, status selectors, subscriptions, notification classification, seeded fixtures" | Yes — initiative roots, focus path, action-needed vs passive-progress notifications |
| VS-008 Navigate with pack, unpack, focus, pin, and unpin | Phase 2 | §"VS-008" M; "Tool command schema, policy enforcement, state transitions, next-render invalidation, tool affordance tests" | Yes — bounded graph-walk tools whose effects appear on next render |
| VS-009 Run bounded orchestrator turns with advisory optimizer requests | Phase 2 | §"VS-009" L; "Durable turn orchestration, capture/commit transactions, advisory request queueing, /compact detection/avoidance, UI state" | Yes — durable turns, bounded foreground graph actions, advisory optimizer requests |
| VS-010 Refresh summaries and mark stale nodes | Phase 2 | §"VS-010" L; "Optimizer scoping, prompt/response schema, edit validation, merge attempts, conflict-on-stale-base behavior, summary/stale UI" | Yes — budgeted summary/stale edits visible in optimizer log |
| VS-011 Simulate configuration effects and request shape repair | Phase 3 | §"VS-011" M; "Expanded warning categories, anomaly detectors, shape-repair request flow over Phase 0 `OptimizerRequest` contract, UI linkage" | Yes — configuration warnings produce advisory optimizer requests |
| VS-012 Repack hierarchy and discover cross-references | Phase 3 | §"VS-012" L; "Topology edit operations, cross-reference discovery, repack planning, identity forwarding application, conflict generation, UI inspection" | Yes — cross-references and bounded hierarchy repack |
| VS-013 Resolve snapshot-merge conflicts and identity forwarding | Phase 3 | §"VS-013" L; "Merge precondition checks, stale-base conflict detection, forwarding-map resolver, conflict state machine, user/orchestrator conflict surfaces" | Yes — fail-closed conflicts with affected nodes, identity events, resolution state |
| VS-014 Quarantine poisoned or privilege-unsafe graph content | Phase 3 | §"VS-014" L; "Quarantine edit type behavior, privilege transforms, render blocks, validator fixtures, drill-down UI" | Yes — privilege labeling, render quarantine, evidence/policy traceability |
| VS-015 Dispatch provider-aware worker slices | Phase 4 | §"VS-015" L; "Slice validation, provider-aware route approval, prompt/render for worker slices, acceptance tracking, evidence ingestion, launch UI/control path" | Yes — bounded sub-agent slices via `agents` with graph context, write scope, provider preflight, budget, acceptance tracking |
| VS-016 Show the worker board and trace evidence | Phase 5 | §"VS-016" M; "Board filters, status selectors, trace/evidence mapping, capability/budget badges, ambiguity states" | Yes — worker slices, sessions, accepted/rejected/blocked/running/completed, fingerprints, budget, evidence |
| VS-017 Route NEEDS_INPUT answers as worker continuations | Phase 5 | §"VS-017" L; "NEEDS_INPUT envelope parsing, queue UI, answer artifact writer, acceptance watcher, failed-resume integration" | Yes — global action-needed queue, exact blocked-worker resume, child-acceptance tracking |
| VS-018 Stage worker output for graph reintegration | Phase 5 | §"VS-018" L; "Staged candidate model, parser/mappers, conflict-on-overlap logic, advisory optimizer request generation, staging UI" | Yes — graph candidates, evidence, blockers, conflicts, advisory optimizer requests |
| VS-019 Sample reviewers over high-consequence edits | Phase 6 | §"VS-019" M; "Sampling decisions, prompt packer, result schema, UI flags, budget-ledger integration, recovery-anomaly trigger hook subscribing to VS-020 events" | Yes — reviewers as cost-aware evidence channel |
| VS-020 Surface recovery preflight and state accounting | Phase 6 | §"VS-020" L; "Preflight engine, preserved/replayed/discarded accounting, side-effect classifier, recovery surface, reconciliation transactions" | Yes — recovery actions with preconditions, affected state, side-effect class, preserved/replayed/discarded records |
| VS-021 Reroute or substitute failed provider/worker runs explicitly | Phase 7 | §"VS-021" M; "Route comparison, substitution planning, confirmation flow, contract-diff display, audit linkage" | Yes — reroute/substitute with capability/cost/side-effect/confirmation visibility |

No slice is missing. No slice is silently dropped. No slice is reinterpreted in a way that loses operator-visible capability. The engineering roadmap retains the same `WorkingSetSnapshot`, `BudgetLedger`, `SummaryContract`, `OptimizerEdit`, `ConflictRecord`, `WorkerSlice`, `QuestionArtifact`, and `RecoveryAction` operator surfaces named by the executive roadmap. Phase placements of VS-001 through VS-021 inside the same Phase 1–7 numbers as the executive roadmap are preserved.

**Recommendation:** No action required.

---

### F-2. All five engineering reorderings are documented as Pushback Summary entries with cost/value framing

**Severity: NONE**

The engineering roadmap deviates from executive ordering in five places, all collected under §"Pushback Summary" (`engineering-roadmap.md` lines 476–548):

| Pushback | Executive ordering | Engineering recommendation | Cost framing | Value-lost framing |
|---|---|---|---|---|
| P-1 (§"Pushback P-1: Expand Phase 0 before Phase 1 acceptance") | "Phase 1 slices begin after an engineering-defined Phase 0 placeholder with no detailed scope" | "Treat Phase 0 as an XL substrate phase containing GraphStore/migrations, evidence/audit backbone, RenderEngine core, PolicyEngine, BudgetLedger core, `ProviderState` / `EntitlementSnapshot` / `CapabilityFingerprint` records, CLI supervisor with subprocess management, ... Tauri IPC layer, frontend stack scaffolding and UI shell, and test fixtures" | "If VS-001 through VS-007 are built directly from the empty worktree, each slice will invent its own DTOs, schema migrations, render result states, audit calls, and UI event streams" | "Operators wait longer before seeing the first working-set inspector" |
| P-2 (§"Pushback P-2: Build provenance/audit and budget gate backends before declaring VS-001 complete") | "VS-001 inspect renders appears before VS-003 provenance/audit and VS-004 budget/cache" | "Keep VS-001 as the first operator-visible target, but implement the VS-003 evidence/audit backbone and VS-004 budget/cache ledger fields before VS-001 acceptance" | "If the inspector ships before those IDs and ledgers exist, it will show a temporary render shape and later require migration of snapshot rows, UI drill-down, test goldens, and policy decisions" | "The earliest inspector milestone may show less UI sooner, because backend evidence and budget plumbing must land first" |
| P-3 (§"Pushback P-3: Implement VS-013 conflict/identity mechanics before VS-012 topology edits") | "VS-012 repack/cross-reference appears before VS-013 conflict and identity forwarding in Phase 3" | "Build VS-013's identity resolver, optimistic merge engine, and `ConflictRecord` state machine before enabling VS-012 topology edit types" | "Building VS-012 first either produces throwaway merge logic or risks topology changes that cannot fail closed" | "Operators wait longer for cross-reference discovery and hierarchy repack" |
| P-4 (§"Pushback P-4: Pull a recovery skeleton into Phase 0 / Phase 5 prerequisites") | "Full recovery accounting is VS-020 in Phase 6, after NEEDS_INPUT routing and worker reintegration" | "Keep operator-visible recovery in VS-020, but implement `RecoveryAction` schema, side-effect taxonomy, and audit linkage in Phase 0, and require a failed-resume handoff path before VS-017 acceptance" | "Without recovery records, failed continuations become ad hoc error states that later need migration into VS-020's preserved/replayed/discarded accounting" | "VS-017 may take longer because it must write durable recovery handoff records even though full recovery UI arrives later" |
| P-5 (§"Pushback P-5: Treat provider fingerprints as an early contract, not just a VS-006 UI feature") | "VS-006 is one Phase 1 slice among seven" | "Define `ProviderState`, `EntitlementSnapshot`, and `CapabilityFingerprint` schemas and denial-reason taxonomy in Phase 0, then deliver the operator-visible preflight panel as VS-006" | "VS-001 render route display, VS-004 budget interpretation, VS-009 orchestrator launch, VS-015 worker dispatch, VS-017 continuation, VS-020 recovery, and VS-021 reroute all consume provider/capability records. If VS-006 owns the first real schema late in Phase 1, earlier slices either fake route state or need retrofits" | "The VS-006 panel itself is not visible earlier; only its schema contract is" |

Each pushback states implementation cost in concrete terms (duplicate substrate, migration cost across snapshot rows / UI / goldens, throwaway merge logic, ad hoc error states, retrofits across multiple slices) and a specific value-lost statement (delayed inspector, delayed cross-references, longer VS-017 build, no earlier panel). None is framed as "the executive ordering is wrong"; all are framed as "the executive ordering costs X because Y" with an explicit recommendation. This satisfies the proposer's Step 4 pushback contract.

The §"Engineering Phase Structure" Phase 1 ordering (`engineering-roadmap.md` lines 618–629) — VS-003/VS-004 → VS-001 → VS-002 → VS-005 → VS-006 → VS-007 — is the implementation of P-2 and is consistent with both the §"Critical Path" chain (line 705) and the §"Dependency Graph" listing (line 759). The Phase 3 engineering ordering (lines 641–650) — VS-011 → VS-013 → VS-012 → VS-014 — is the implementation of P-3.

**Recommendation:** No action required.

---

### F-3. Phase 0 expansion is foundation-only and does not add operator-visible capability outside executive scope

**Severity: NONE**

The executive roadmap (`executive-roadmap.md` §"Phase 0: Foundations") explicitly defers Phase 0 to engineering with the note "this phase may include storage, eventing, migrations, local process boundaries, testing substrate, and app shell wiring, but those are not detailed here because they are not independently operator-visible value slices." The two pervasive constraints are "Local Control of State" and "No In-Product Compaction."

The engineering Phase 0 expands into 0A/0B/0C (`engineering-roadmap.md` §"Engineering Phase Structure", lines 552–616):

- **0A "Repository and Runtime Skeleton"** — explicitly stated: "This phase has no operator-visible value beyond a running shell." The 22 enumerated foundation items in §"Foundation Phase" cover the Tauri/Turbo/React monorepo scaffold, Rust/Tokio backend skeleton, settings/storage roots, IPC commands, UI shell, and logging/tracing — all infrastructure.
- **0B "Canonical State and Contracts"** — migrations and base GraphStore schema. Explicitly stated: "Implement repository boundaries and fixture builders, but keep features inert unless a value slice turns them on."
- **0C "Shared Engines and Integration Shells"** — RenderEngine core interfaces, PolicyEngine gate framework, BudgetLedger core service, `ConfigurationRegistry` skeleton/read API, CLI subprocess supervisor, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, identity/conflict shell, recovery-action writer, Tauri IPC commands/Channel event streams, and seeded UI panes. Explicitly stated: "no value-slice-specific claims such as 'workers can launch' or 'optimizer refreshes summaries.'"

Schemas that look like operator-facing capability are deliberately ring-fenced as foundation only:
- "WorkerSlice / WorkerRun schema and state contract, **without launch UI**" — launch is delivered in VS-015.
- "QuestionArtifact schema and continuation-envelope contract, **without full queue UI**" — queue is delivered in VS-017.
- "RecoveryAction schema, side-effect classification taxonomy, and audit linkage" — operator-visible recovery is delivered in VS-020 (P-4 makes this explicit: "Keep operator-visible recovery in VS-020").
- "ProviderStateMonitor, `ProviderState`, `EntitlementSnapshot`, `CapabilityFingerprint`, and denial-reason taxonomy" — operator preflight panel is delivered in VS-006 (P-5: "deliver the operator-visible preflight panel as VS-006").
- "Optimizer queue, `OptimizerRequest` / `OptimizerEdit` store, and merge-validation shell" — actual optimizer cycles are delivered in VS-010, VS-011, VS-012, VS-013.
- "`ConfigurationRegistry` skeleton and read API over `GraphConfiguration`" — operator-visible inspector is delivered in VS-005 (engineering VS-005 explicitly says: "It does not create the registry itself; Phase 0 owns the registry skeleton and read path").
- "`AgentWalkState` schema and navigation state service shell" — pack/unpack/focus tools are delivered in VS-008.

Two cross-cutting Phase 0C contracts (§"Substrate Integration Contract", §"Cross-slice contract: `OptimizerRequest` emission") freeze the agent-runner subprocess wrapper API and the `OptimizerRequest` taxonomy with stable enum values and advisory-only semantics. These are shared contracts for downstream slices to consume; neither adds new operator-visible capability and both explicitly forbid emitter-specific queue variants or direct graph mutation by emitters.

The two executive Phase 0 pervasive constraints are preserved:
- **Local Control of State** — `engineering-roadmap.md` §"Technical Landscape Summary" names the proposal's "local SQLite data model" as the canonical store; Phase 0B places GraphStore migrations locally; no value-slice claims a vendor session store as critical state. The Substrate Integration Contract explicitly states "no harness ownership of vendor credentials" and treats agent-runner trace as "evidence/substrate rather than canonical graph truth."
- **No In-Product Compaction** — explicitly preserved at VS-009 ("`/compact` detection/avoidance"); the engineering roadmap does not add `/compact` semantics or compaction-equivalent behavior at any slice.

**Recommendation:** No action required.

---

### F-4. Anti-scope is preserved across all 21 slices

**Severity: NONE**

Each engineering slice's "What is new" was checked against the executive slice's "Anti-scope." No silent inclusion of anti-scope items was detected. Selected confirmations:

- VS-001 anti-scope ("Does not include pack/unpack/focus tools, optimizer edits, worker dispatch, routed questions, or recovery actions"): engineering VS-001 lists "renderer, snapshot store, render blob storage, token/cache estimator, IPC commands, inspector pane, audit integration" — none cross into anti-scope. Pack/unpack is VS-008; optimizer edits are VS-010+; dispatch is VS-015; questions are VS-017; recovery is VS-020/VS-021.
- VS-002 anti-scope ("Does not regenerate summaries, discover new cross-references, repair provenance, or change graph topology"): engineering VS-002 limits itself to validators, template-source handling, evidence locator checks, UI labels, render blocking — no regeneration. Regeneration is VS-010; cross-references are VS-012; topology is VS-012.
- VS-003 anti-scope ("Does not authorize topology mutation, worker reintegration, reviewer sampling, or recovery replay"): engineering VS-003 limits itself to capture/classification/audit; topology is VS-012; reintegration is VS-018; reviewers are VS-019; recovery replay is VS-020.
- VS-004 anti-scope ("Does not build provider billing import, pricing prediction, optimizer scheduling, or worker launch by itself"): engineering VS-004 limits to ledger records / estimation / policy transitions / cost surface; optimizer scheduling is VS-010; launch is VS-015.
- VS-005 anti-scope ("Does not auto-rewrite graph shape, regenerate summaries, repair indexes, or choose provider routes"): engineering VS-005 explicitly lists shape *explanation* and validation *warnings*, not rewrites; configuration warnings only become advisory requests in VS-011; provider routing remains VS-006/VS-021.
- VS-006 anti-scope ("Does not store credentials, repair vendor accounts, dispatch workers, or perform recovery reroutes"): engineering VS-006 names "redaction" and "not storing secrets" as risk factors; dispatch is VS-015; reroute is VS-021.
- VS-007 anti-scope ("Does not include worker dispatch controls, question answering, recovery execution, cost drill-down, or optimizer edit approval"): engineering VS-007 limits to initiative/focus UI, status selectors, subscriptions, notification classification.
- VS-008 anti-scope ("Does not mutate graph topology, summaries, cross-references, evidence, identity, or worker assignments"): engineering VS-008 limits itself to `AgentWalkState` mutation; topology mutation is VS-012.
- VS-009 anti-scope ("Does not include actual optimizer edits, worker dispatch, question routing, or recovery execution"): engineering VS-009 explicitly limits to "advisory request queueing"; optimizer edits are VS-010; dispatch is VS-015; routing is VS-017; recovery execution is VS-020.
- VS-010 anti-scope ("Does not split, merge, re-parent, repack, discover cross-references, quarantine poisoned nodes, or reintegrate workers"): engineering VS-010 explicitly bounds the slice to "summary/stale edit types"; topology is VS-012; quarantine is VS-014; reintegration is VS-018.
- VS-011 anti-scope ("Does not silently rewrite graph truth, add new schema concepts, or replace explicit user configuration"): engineering VS-011 limits itself to anomaly detection and shape-repair *request* flow over the Phase 0 `OptimizerRequest` contract.
- VS-012 anti-scope ("Does not run worker reintegration, mutate the current orchestrator turn, or bypass conflict handling for identity/topology changes"): engineering VS-012 explicitly depends on VS-013 conflict/identity mechanics (P-3 ordering); reintegration is VS-018.
- VS-013 anti-scope ("Does not decide arbitrary conflict policy for every future worker type, execute recovery, or expose multi-user collaboration controls"): engineering VS-013 limits itself to merge precondition checks, forwarding-map resolver, conflict state machine, conflict surfaces.
- VS-014 anti-scope ("Does not make reviewers authoritative, delete evidence, or provide enterprise team access-control features"): engineering VS-014 limits itself to "Quarantine edit type behavior, privilege transforms, render blocks, validator fixtures, drill-down UI"; no evidence deletion or team ACL is named.
- VS-015 anti-scope ("Does not reintegrate output as graph truth, route user questions, sample reviewers, or perform recovery substitution"): engineering VS-015 explicitly lists "evidence ingestion, launch UI/control path"; reintegration is VS-018; routing is VS-017; reviewers are VS-019; substitution is VS-021.
- VS-016 anti-scope ("Does not answer questions, merge worker output, cancel/recover workers, or resolve provider failures"): engineering VS-016 limits itself to board filters, status selectors, trace/evidence mapping, capability/budget badges, ambiguity states.
- VS-017 anti-scope ("Does not append answers to the lead transcript as a substitute, resolve graph conflicts, or fresh-substitute workers after failed resume"): engineering VS-017's "failed-resume integration" is the recovery *handoff record* per Pushback P-4 ("durable recovery handoff records"), not fresh substitution; fresh substitution remains VS-021.
- VS-018 anti-scope ("Does not let workers directly mutate topology, summaries, identity, cross-references, quarantine state, or current orchestrator focus"): engineering VS-018 limits to "Staged candidate model, parser/mappers, conflict-on-overlap logic, advisory optimizer request generation, staging UI" — staged candidates do not directly mutate.
- VS-019 anti-scope ("Does not make reviewer approval sufficient for merge, review every mutation, or replace deterministic gates"): engineering VS-019 lists deterministic-gate-precedence as a risk factor and limits itself to sampling. The added VS-020 → VS-019 event-subscription edge is a trigger source for sampling, not a path to authoritative reviewer judgment, and the executive VS-019 capability already names "provider anomalies" as a reviewable category.
- VS-020 anti-scope ("Does not silently resume, substitute workers, or reroute providers without exposing changed execution contracts"): engineering VS-020 limits itself to preflight/accounting/reconciliation; substitution and reroute are VS-021.
- VS-021 anti-scope ("Does not repair vendor accounts, hide degraded execution contracts, or claim a fresh worker is a successful resume"): engineering VS-021 lists "fresh substitution honesty" as a risk factor and limits itself to comparator/planner/confirmation/contract-diff.

No anti-scope item is silently included.

**Recommendation:** No action required.

---

### F-5. No undocumented scope additions and no silent scope reductions

**Severity: NONE**

Two areas were checked specifically for hidden scope changes:

1. **VS-001 acceptance prerequisite** (engineering-roadmap.md line 70–72): "VS-001 remains the first operator-visible Phase 1 target, but it is not accepted until VS-003's evidence/audit backbone and VS-004's budget/cache primitives are real enough for `WorkingSetSnapshot` rows to include evidence pointers, token estimates, cache-prefix hashes, provider-state references, configuration-explanation references, and audit links without later migration." This is the implementation of Pushback P-2; it does not narrow VS-001's operator capability (the inspector still shows the same `WorkingSetSnapshot` fields the executive defines for it) — it only sequences the backend prerequisites needed before acceptance. The capability statement in `executive-roadmap.md` VS-001 is satisfied by the engineering "What is new" list.

2. **VS-017 acceptance prerequisite** (engineering-roadmap.md line 392): "VS-017 is not accepted until a failed-resume handoff path writes durable `RecoveryAction` records with side-effect class, affected worker/session/question refs, audit linkage, and enough preserved/replayed/discarded placeholders for VS-020 to surface later without migration. Full operator-visible recovery remains VS-020." This is the implementation of Pushback P-4. The added record-writing is internal substrate (per executive Phase 0 deferral to engineering). VS-017's operator-visible capability in the executive roadmap — global action-needed queue, exact blocked-worker resume, child-acceptance tracking — is fully delivered, and the executive VS-017 anti-scope (no fresh substitution after failed resume) is preserved.

3. **VS-019 + VS-020 event subscription** (engineering-roadmap.md lines 432–434, 673, 781): Engineering adds an event-subscription edge `VS-020 -> VS-019` so reviewer sampling can be triggered by recovery-anomaly events. Engineering Phase 6 sequence is `1. VS-019, 2. VS-020` (matching executive Phase 6 ordering). The narrative explicitly states: "VS-019 can ship its core sampler first, then subscribe to VS-020 recovery-anomaly events once the recovery surface emits them; this adds the explicit `VS-020 -> VS-019` event edge without changing the Phase 6 sequence." The executive VS-019 capability already names "provider anomalies and configuration-shape anomalies" as reviewable categories, so recovery-anomaly events fit within the executive scope.

The Effort Summary table (lines 713–723) lists 22 foundation items (Phase 0) plus 21 value slices, exactly matching the executive's "21 value slices + Phase 0 (engineering-defined)" structure. No slice is consolidated or absorbed into another. No new slice IDs (e.g., VS-022) are introduced.

**Recommendation:** No action required.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| F-1 | All 21 executive value slices covered with operator-visible capability preserved | NONE |
| F-2 | All five engineering reorderings documented as cost/value-framed pushback (P-1 through P-5); narratives consistent with phase ordering | NONE |
| F-3 | Phase 0 expansion (0A/0B/0C) is foundation-only; pervasive constraints (Local Control of State, No In-Product Compaction) preserved | NONE |
| F-4 | Anti-scope preserved across all 21 slices | NONE |
| F-5 | No undocumented scope additions or silent scope reductions; acceptance prerequisites and event subscription are internal sequencing within executive scope | NONE |

## What LOW requires

For this LOW rating to remain valid, the following conditions must hold:

1. **Coverage complete.** Every one of VS-001 through VS-021 continues to appear by ID in `engineering-roadmap.md` §"Initiative Assessments," each with a "What is new" list that delivers the operator-visible capability named in `executive-roadmap.md` §"Value Slice Inventory" for the same VS-NNN. Phase placements (Phase 1–7) match the executive Phase numbers.
2. **Reorderings remain documented.** The five deviations from executive ordering (Phase 0 expansion, VS-003/VS-004 backend before VS-001 acceptance, VS-013 before VS-012 within Phase 3, RecoveryAction schema in Phase 0 with VS-017 handoff prerequisite, ProviderState/EntitlementSnapshot/CapabilityFingerprint schema in Phase 0) remain enumerated in `engineering-roadmap.md` §"Pushback Summary" with the implementation-cost statement, the value-lost statement, and the explicit recommendation. No new deviation may be introduced without a new Pushback entry in the same format.
3. **Phase 0 stays foundation-only.** No item in `engineering-roadmap.md` §"Foundation Phase" promotes itself from substrate (schema, interface, registry, shell, fixtures) to operator-visible capability that the executive roadmap reserved for a value slice. In particular: WorkerSlice schema in Phase 0 retains "without launch UI"; QuestionArtifact schema retains "without full queue UI"; RecoveryAction schema retains "without recovery UI"; OptimizerRequest/Edit store retains "merge-validation shell" rather than active optimizer cycles; ProviderStateMonitor retains "record model" rather than the operator-visible preflight panel; `ConfigurationRegistry` retains "skeleton and read API" rather than the operator-visible inspector; `AgentWalkState` retains "schema and navigation state service shell" rather than the pack/unpack tools.
4. **Anti-scope continues to be honored.** No engineering slice's "What is new" list expands to include items the corresponding executive slice declared anti-scope (e.g., VS-009 must not begin authoring optimizer edits; VS-014 must not delete evidence; VS-017 must not fresh-substitute workers; VS-020 must not silently resume without exposing changed contracts; VS-019 must not become reviewer-as-ground-truth).
5. **Pervasive constraints from executive Phase 0 remain intact.** "Local Control of State" must continue to be reflected in local SQLite ownership across all slices and in the Substrate Integration Contract's no-credential-ownership clause. "No In-Product Compaction" must remain enforced — VS-009 must continue to detect/avoid `/compact`, and no later slice may reintroduce vendor-side compaction or in-product compaction equivalents.
6. **Pushback framing stays cost/value, not "executive is wrong."** Each Pushback Summary entry must continue to state implementation cost in concrete terms and the value lost by the engineering ordering. None may shift to a framing that argues the executive roadmap's strategic ordering is incorrect rather than carrying a higher implementation cost.
7. **No new VS-IDs.** The roadmap continues to deliver exactly 21 value slices VS-001 through VS-021. Engineering may split or merge work units inside a slice (and may add internal acceptance prerequisites), but it must not introduce a new value slice ID outside the executive inventory or absorb an executive slice's operator-visible capability into Phase 0.
