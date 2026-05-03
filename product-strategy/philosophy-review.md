# Stage 2 Philosophy Alignment Review — Round 7 (post-DECISIONS.md stack correction)

## Round scope

Round 7 is a brownfield revision triggered by canonical-stack recovery. The change to `proposal.md` since round 6 is bounded:

- `Fixed Substrate` (proposal.md:14–20) had a hallucinated frontend stack list (Bun, Turbo, React 19, TanStack Router, TanStack Query 5, Tailwind v4, xo-typescript, Prettier, Vitest, Playwright, Lefthook, Changesets, Commitlint) embedded inline in v0. That inline list has been removed.
- The corrected `Fixed Substrate` now references substrate **by category only** — Tauri v2 desktop shell, Rust/Tokio backend, local SQLite, frontend SPA — and forwards every concrete crate, package, version, lint/format tool, build tool, test runner, type-checker, ORM, migration runner, and model-role assignment to `/DECISIONS.md` (D2 application substrate from `~/projects/agent-runner`; D3–D17 frontend SPA substrate from `~/projects/server-manager` research, including D7–D7e and D8–D11 SPA cascade; D18 model assignments).
- `proposer.md` frontmatter explicitly authorizes this deferral: *"Stack/build-order content is roadmap-layer concern; defer to /DECISIONS.md and the roadmap layer rather than enumerating it here."*
- No design commitment, schema object, lifecycle state, governance gate, observability surface, AI/ML actor description, constraint, or non-goal was rewritten. Stage 1 confirmed (per `problem-review.md` round 7) that all 18 axes (§1–§18) remain aligned with the same mechanisms cited in rounds 3–6. Every axis proceeds to philosophy review.

This review re-checks (a) whether the corrected proposal still embodies `philosophy.md` for every aligned axis, and (b) whether the deferral-by-pointer pattern itself introduces any new violation, ungrounded decision, structural concern, or surfaceable philosophical concern.

The prior round-3 philosophy-review embodiment summary remains in force; what follows reaffirms each axis under the corrected proposal and addresses the stack-correction surface specifically.

---

## Stack-correction surface analysis

The categorical-substrate-by-pointer pattern is the only structurally new artifact in round 7. It deserves explicit philosophy checking.

### Does the deferral violate `P1` (Imposed Context, Visible to the User)?

No. `P1` is about runtime imposition of context onto the orchestrator and visibility of that imposition to the user. The proposal's deferral is at the *design-document* layer, not the runtime layer. `/DECISIONS.md` is a local, reviewable, append-only artifact under the user's filesystem; the proposal honestly says where stack content lives (`proposal.md:19` cites D2, D3–D17, D7–D7e, D8–D11) rather than presenting an undocumented or fabricated list. Visibility is preserved through the explicit pointer.

### Does the deferral violate `P15` (Local Control of Graph and Provenance)?

No. `/DECISIONS.md` is a local file in the same repository; references resolve without vendor dependency. The substrate categories the proposal still names — Tauri v2 desktop shell, Rust/Tokio backend, local SQLite, frontend SPA — are exactly the surfaces that `P15` cares about (local desktop, local-canonical state, no required vendor session store). Deferring concrete crate/package versions to a local decisions log does not relocate any state outside local control.

### Does the deferral violate `P3` / `P7` (Contract / Provenance)?

No. `P3` and `P7` apply to runtime graph contracts (summary contract conformance, evidence pointers, validation state). They are not document-format principles. The proposal's design contracts (`SummaryContract`, `ProvenancePointer`, `EvidenceArtifact`, `WorkingSetSnapshot`, `OptimizerEdit`, `IdentityEvent`) are textually unchanged in round 7, so contract and provenance embodiment is unchanged.

If anything, the round-7 correction *embodies* the spirit of `P3`/`P7` at the planning layer: the v0 inline stack list was a contract-invalid claim (no evidence, no provenance back to `~/projects/agent-runner` or `~/projects/server-manager` research). The correction replaces an unprovenanced fabrication with an explicit pointer to a provenanced decisions log whose own entries carry citations to source files (e.g., D2 cites agent-runner Cargo.toml/Cargo.lock; D4 cites server-manager research lines 14–17, 22–24; D7 cites agent-runner package.json and server-manager research). This mirrors at the planning layer what the proposal demands at runtime.

### Does the deferral violate `P16` (Single User, Single Tab, One Orchestrator)?

No. The substrate categories the proposal retains (Tauri v2 desktop, single SPA, local SQLite) are precisely what `P16` requires. Concrete framework choices (Solid 1.9.x vs. React 19, vanilla CSS vs. Tailwind, rusqlite vs. sqlx) live in DECISIONS.md without changing the single-user-single-tab posture.

### Does the deferral violate any anti-goal?

No. The relevant anti-goals (general-purpose agent framework, multi-tenant SaaS, replacing `agent-runner`, `/compact` interoperability, sidebar-of-many-chats, infinite memory, novelty-for-its-own-sake) are about runtime behavior and product scope, not about where stack versions are documented. `agent-runner` is still the substrate (D2 says "Adopt agent-runner's actual Tauri+Rust substrate verbatim"), not replaced or duplicated.

### Categorical-substrate-by-pointer is consistent with existing philosophy

The pattern — proposal commits to substrate **categories** (what kind of thing is being used and why); decisions log commits to substrate **versions** (which crate at which pin and on whose evidence) — does not introduce a new principle. It is a routine application of the existing scope discipline that the proposer prompt makes explicit. No implicit principle needs to be articulated; no new philosophical axis is opened; no existing principles come into new tension.

---

## Per-axis reaffirmation under the corrected proposal

The mechanisms cited below are unchanged from round 3 and have been re-verified against the round-7 proposal.

### §1 Effective Working Set vs. Nominal Context Window

Embodied. **Imposed Working Set** (proposal.md:24–28), `WorkingSetSnapshot` (proposal.md:438–471), required pins, eviction order (proposal.md:934–942), `reasoning_budget_class`, and `overfull_required_context` continue to treat working-set quality as effective reasoning capacity rather than vendor context capacity. Implements `P1`, `P2`, `P12`. Stack deferral does not touch any of these mechanisms.

### §2 Summary Contract

Embodied. `SummaryContract` (proposal.md:362–391), `ProvenancePointer` (proposal.md:418–436), **Summary Regeneration** (proposal.md:1386–1412), summary-contract gates (proposal.md:1179), and `validation_state` enumerations preserve machine-checkable structure with explicit invalid states rather than fluent drift. Implements `P3`, `P7`. Untouched by deferral.

### §3 Concurrent Optimizer Mutation Under Foreground Walking

Embodied. **Snapshot-Walk-Then-Merge** (proposal.md:30–34), `GraphSnapshot` (proposal.md:290–316), bounded `GraphAction` (proposal.md:473–510), advisory `OptimizerRequest` (proposal.md:665–689), `OptimizerEdit` (proposal.md:691–717), `ConflictRecord` (proposal.md:719–738), and the **Orchestrator Turn** / **Optimizer Cycle** lifecycles (proposal.md:957–995, 1052–1081) keep per-turn views deterministic with optimistic merge and explicit conflict surfacing. Implements `P1`, `P5`, `P6`, `P14`. Untouched by deferral.

### §4 Stable Identity Across Summary Regeneration and Topology Reshape

Embodied. Opaque stable IDs (proposal.md:180–182), `GraphNode` (proposal.md:237–265), `IdentityEvent` (proposal.md:340–360), `GraphEdge.forwards_to` (proposal.md:326), snapshot-local identity resolution (proposal.md:312–316), and optimizer-owned topology edits keep identity durable across regeneration, split, merge, re-parenting, forwarding, and recovery. Implements `P4`, `P15`. Untouched by deferral.

### §5 Working-Set Policy

Embodied. **Working-Set Policy** (proposal.md:923–955), `AgentWalkState` (proposal.md:512–533), `WorkingSetSnapshot`, **Pack, Unpack, and Focus Tools** (proposal.md:1030–1050), eviction order, recursive bounds, and explicit `denied_*` tool states preserve the agent-walk-state-vs-graph-topology boundary. Implements `P2`, `P5`. Untouched by deferral.

### §6 Hierarchical Packing Without Bounded-Depth Precedent

Embodied. Recursive unpack bounds (proposal.md:944–953), `GraphEdge` containment plus `packs_into` / `unpacks_to` (proposal.md:326), `AgentWalkState.unpacked_stack` (proposal.md:522), and optimizer `repack` edit type (proposal.md:701) keep arbitrarily deep packed state navigable under live operation. Implements `P2`, `P4`, `P6`. Untouched by deferral.

### §7 Cross-CLI Rendering Asymmetry

Embodied. **Cross-CLI Adaptation** (proposal.md:62–73), `RenderEngine` (proposal.md:121–125), `CapabilityFingerprint` (proposal.md:795–823), `WorkerRun` (proposal.md:563–584), and the cross-CLI Constraints/Lost entry (proposal.md:1442) keep CLI capability differences explicit rather than hidden behind a uniform façade. Implements `P8`, `P13`. Untouched by deferral.

### §8 Sub-Agent Supervision via Subgraph Slices

Embodied. `WorkerSlice` (proposal.md:535–561), `WorkerRun`, **Sub-Agent Dispatch and Reintegration** (proposal.md:1083–1115), `WorkerDispatcher` (proposal.md:134–139), `overlap_policy` (proposal.md:548), and staged reintegration via advisory `OptimizerRequest` keep delegation a graph-slice operation with optimizer-owned curation. Implements `P5`, `P7`, `P8`, `P10`, `P12`. Untouched by deferral.

### §9 User-Question Routing as Graph-State Routing

Embodied. **Questions as Continuations** (proposal.md:87–89), `QuestionArtifact` (proposal.md:613–637), **NEEDS_INPUT Routing** (proposal.md:1117–1138), `child_accepted` requirement, and the resume state machine attach questions to slice/worker/render/blocked output and demand acceptance before treating answers as resumed work. Implements `P11`, `P14`. Untouched by deferral.

### §10 Tool-Call Protocol State as First-Class Graph Provenance

Embodied. **Provenance Beside Summaries** (proposal.md:56–60), `EvidenceArtifact` (proposal.md:393–416), `ToolCallProvenance` (proposal.md:639–663), `GraphAction.record_tool_provenance` (proposal.md:483, 492), deterministic tool-protocol gates (proposal.md:1190), and **Fact Extraction** (proposal.md:1414–1434) preserve protocol IDs, results, approval state, retry semantics, side-effect class, and source protocol. Implements `P7`, `P10`, `P14`. Untouched by deferral.

### §11 Workflow-Reviewer Reliability for Graph Mutations

Embodied. `WorkflowReviewer` (proposal.md:149–153), `PolicyEngine` (proposal.md:155–160), **Deterministic Gates** (proposal.md:1175–1196), **Reviewer Sampling** (proposal.md:1198–1223), and the `Workflow Reviewer` AI-use section (proposal.md:1359–1384) keep reviewer output as fallible evidence with deterministic gates carrying hard guarantees. Implements `P10`, `P12`. Untouched by deferral.

### §12 Instruction Hierarchy and Memory Poisoning Becoming Graph Poisoning

Embodied. **Privilege and Poisoning Controls** (proposal.md:1225–1229), `GraphNode.privilege_origin` and `trust_state` (proposal.md:249–250), `SummaryContract.poison_risk` (proposal.md:381), `ProvenancePointer.privilege_transform` (proposal.md:430), `quarantined` lifecycle states, and optimizer `poison_quarantine` edit type (proposal.md:701) engage poisoning as a graph-promotion and imposed-context risk. Implements `P1`, `P7`, `P10`, `P15`. Untouched by deferral.

### §13 Cost, Latency, and Resource Tails of Graph-Walking Workloads

Embodied. **Cost as Correctness** (proposal.md:91–95), `BudgetLedger` (proposal.md:825–846), render/optimizer/reviewer budget gates, `WorkingSetSnapshot.prefix_hash` cache locality (proposal.md:459), and `policy_action` thresholds treat cost as operational correctness. Implements `P12`. Untouched by deferral.

### §14 Multi-Workstream Legibility for the Agent and the User

Embodied. `UserSurface` (proposal.md:174–178, 1155–1171), **Observability** (proposal.md:1245–1262), initiative-root states, global question queue, worker board, optimizer log, recovery surface, provider panel, configuration inspector, and action-needed/passive notification separation keep the shared graph as the multi-workstream model. Implements `P1`, `P13`, `P14`, `P16`. Untouched by deferral.

### §15 Imposed Working Context Without Surveyed Precedent

Embodied. **Imposed Working Set**, `GraphStore` as canonical local SQLite + filesystem (proposal.md:114–119), `RenderEngine`, bounded `GraphAction`, advisory `OptimizerRequest`, optimizer-owned curation, **No In-Product Compaction** (proposal.md:83–85), and the "local graph state is canonical" Constraints/Lost entry (proposal.md:1439) show the stronger render/provenance/audit/recovery guarantees imposition demands. Implements `P1`, `P5`, `P9`, `P15`. Untouched by deferral.

### §16 Recovery and Resume Surfaces Are Not Neutral

Embodied. `RecoveryAction` (proposal.md:848–872), **Recovery** lifecycle (proposal.md:1140–1153), `QuestionArtifact`, `ToolCallProvenance`, `WorkerRun.acceptance_state` (proposal.md:575), **Provider Preflight and Routing** (proposal.md:1013–1028), recovery gates, and explicit `preserved_ref`/`replayed_ref`/`discarded_ref` distinguish neutral vs. side-effecting state changes. Implements `P14`, `P15`. Untouched by deferral.

### §17 Graph and Memory Configuration Overhead

Embodied. **Configuration as Memory Semantics** (proposal.md:42–48), `GraphConfiguration` (proposal.md:205–235), `ConfigurationRegistry` (proposal.md:162–166), `WorkingSetSnapshot.configuration_explanation_ref` (proposal.md:462), `OptimizerEdit.configuration_refs` (proposal.md:705), **Configuration Inspection and Validation** (proposal.md:997–1011), configuration gates, anomaly-triggered reviewer sampling for configuration shape (proposal.md:1205), and **Configuration and Provider Accountability** audit (proposal.md:1237–1243) make configuration a versioned, cited cause of graph shape and failure attribution. Implements `P1`, `P3`, `P7`, `P13`, `P15`. Untouched by deferral.

### §18 Provider, Account, and Entitlement Friction

Embodied. **Provider State as Observable State** (proposal.md:75–81), `ProviderStateMonitor` (proposal.md:168–172), `ProviderState` (proposal.md:740–768), `EntitlementSnapshot` (proposal.md:770–793), extended `CapabilityFingerprint`, `WorkerSlice.required_provider_features` (proposal.md:552), **Provider Preflight and Routing**, `route_denial_reasons` (proposal.md:815), provider-aware `BudgetLedger`, provider-caused `RecoveryAction` (proposal.md:861–862), and provider accountability audit (proposal.md:1241) make auth, billing, quota, entitlement, runtime, network, sandbox, feature support, freshness, and confidence pre-routing state without taking ownership of vendor credential stores. Implements `P8`, `P12`, `P13`, `P14`, `P15`. Untouched by deferral.

---

## Violations

None.

The corrected `Fixed Substrate` references substrate categories that align with `P15` (Tauri desktop, local SQLite, Rust backend) and `P16` (single user, single tab) and forwards concrete versions to a local decisions log that itself satisfies `P15` (local file, append-only, citation-bearing). No design decision in the proposal was rewritten in a way that would introduce a new violation, and the deferral pattern itself is consistent with every relevant principle as analyzed above.

---

## Ungrounded decisions

None.

The deferral pattern is itself grounded — explicitly in `proposer.md`'s scope rule (frontmatter line 2: stack content belongs in `/DECISIONS.md` rather than the proposal). It is not an ungrounded design decision because the proposal is not designing a stack here; it is honoring a documented scope discipline by pointing at the artifact that does carry the stack commitments.

Substrate categories named in `Fixed Substrate` (Tauri v2 desktop shell, Rust/Tokio backend, local SQLite, frontend SPA) trace cleanly to `P15`, `P16`, and the anti-goal against replacing `agent-runner`. They are not ungrounded.

---

## Structural concerns

None at the system-design level introduced by round 7.

The proposal's internal mechanisms (graph schema, working-set policy, snapshot-walk-then-merge, identity events, summary contracts, provenance pointers, evidence artifacts, optimizer cycle, worker slices, NEEDS_INPUT routing, recovery, configuration, provider state) interact the same way they did in round 3–6, because round 7 did not modify any of them. The deferral-by-pointer change is at the planning-document layer and creates no new mechanism interactions inside the harness.

One minor planning-layer note (recorded for completeness, not as a system-design concern): `/DECISIONS.md` is currently marked "DRAFT for agent-harness — pending user approval" (DECISIONS.md line 1). The proposal's pointer is therefore to a draft. This is a process-status fact, not a philosophy violation — the proposal does not claim the draft is ratified, and `proposer.md` already authorizes the deferral pattern. Resolving the draft status is a roadmap/orchestrator concern, not a proposal redesign concern.

---

## Embodiment summary (round 7)

All 18 axes remain embodied under the corrected proposal. The principle citations from round 3 hold without modification:

- §1: `P1`, `P2`, `P12`
- §2: `P3`, `P7`
- §3: `P1`, `P5`, `P6`, `P14`
- §4: `P1`, `P4`, `P5`, `P15`
- §5: `P2`, `P5`
- §6: `P2`, `P4`, `P6`
- §7: `P8`, `P13`
- §8: `P5`, `P7`, `P8`, `P10`, `P12`
- §9: `P11`, `P14`
- §10: `P7`, `P10`, `P14`
- §11: `P10`, `P12`
- §12: `P1`, `P7`, `P10`, `P15`
- §13: `P12`
- §14: `P1`, `P13`, `P14`, `P16`
- §15: `P1`, `P5`, `P9`, `P15`
- §16: `P14`, `P15`
- §17: `P1`, `P3`, `P7`, `P13`, `P15`
- §18: `P1`, `P8`, `P12`, `P13`, `P14`, `P15`

No new philosophical surfaces (implicit principles, principle tensions, or new philosophical axes) were discovered. The stack-correction event is itself analyzable through the existing principles (`P1` visibility through honest pointer; `P3`/`P7` discipline against unprovenanced inline claims at the planning layer; `P15` local control of the decisions log; `proposer.md` scope rule already in force) without requiring philosophy expansion.

`philosophy-surfaces.md` was intentionally not created.

---

## Files written

- `/home/nes/projects/agent-harness/product-strategy/philosophy-review.md` (this file).

`/home/nes/projects/agent-harness/product-strategy/philosophy-surfaces.md` was intentionally not written; round 7's stack correction surfaced no new philosophical concerns.
