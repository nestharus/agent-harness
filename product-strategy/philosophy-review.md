# Philosophy Alignment Review

## Review basis

Stage 2 was run across all 18 axes. `problem-review.md` is the round-2 Stage 1 handoff and lists §1-§16 as aligned; per the round-3 dispatch, §17 and §18 are treated as aligned for this philosophy review.

## Round-3 focus

The round-3 additions are philosophy-grounded.

### Configuration as Memory Semantics

The **Configuration as Memory Semantics** commitment treats schema choices, summary contract templates, optimizer cadence and scope, render budget caps, capability fingerprints, provider routing, memory-policy defaults, index scopes, and default unpack policies as versioned configuration state that changes what the agent can see and what failures mean. That decision embodies:

- `P1 -- Imposed Context, Visible to the User`: configuration is part of what is imposed. The proposal requires renders, optimizer edits, and audit events to cite configuration state, and **Working-Set Policy** requires the renderer to record which caps, depth defaults, allowlists, templates, index scopes, routing defaults, and promotion rules shaped `W`.
- `P3 -- Summary Contract as a Hard Invariant`: summary templates and field-meaning descriptions are treated as contract inputs, not prose preferences. `SummaryContract.template_source`, `GraphConfiguration.summary_contract_template_ids`, `GraphConfiguration.schema_profile`, and **Summary Regeneration** all make configuration part of contract validation and failure attribution.
- `P7 -- Provenance Preserved Beside Summaries`: configuration values are cited beside rendered and derived graph state, so a summary or render can be traced to the policy/template that shaped it.
- `P13 -- Multi-Workstream Legibility for the User`: **Configuration Inspection and Validation** and the `UserSurface` configuration inspector make empty-graph behavior, graph-shape explanations, index state, render caps, templates, and effective value sources inspectable.
- `P15 -- Local Control of Graph and Provenance`: `GraphConfiguration` is a local graph-owned object, changes are append-only revisions, and prior snapshots keep their original configuration.

The proposal also preserves `P5 -- Agent-Owned Focus, Optimizer-Owned Curation`. Configuration is not exposed as an agent-owned focus tool or an optimizer-owned topology shortcut. `GraphConfiguration.effective_value_sources` distinguishes `system_required`, `default`, `inherited`, `user_configured`, and `recovered`; **Configuration Inspection and Validation** says warnings create `AuditEvent` records and may create advisory `OptimizerRequest` records, but do not silently rewrite graph truth; `OptimizerRequest` can only request `consider_configuration_warning`. This keeps configuration warnings from becoming a hidden optimizer side channel for graph mutation.

### Provider State as Observable State

The **Provider State as Observable State** commitment makes auth presence, account identity, billing/quota health, model entitlement, feature support, local runtime availability, network reachability, sandbox boundaries, and provider-specific restrictions inspectable before work is routed. That decision embodies:

- `P1 -- Imposed Context, Visible to the User`: provider state participates in the imposed working context through `WorkingSetSnapshot.provider_state_id`, `CapabilityFingerprint`, route eligibility, route denial reasons, provider panels, audit events, and recovery surfaces.
- `P8 -- Cross-CLI Adaptation, Not Lowest Common Denominator`: provider/account/runtime differences are represented as actual capability fingerprints and entitlement snapshots rather than hidden behind interchangeable model names.
- `P12 -- Cost as a Correctness Control`: quota, billing, route cost risk, and provider-specific limits feed `BudgetLedger`, provider routing, and denial/degradation behavior.
- `P13 -- Multi-Workstream Legibility for the User`: the `UserSurface` includes a provider state panel, worker board capability fingerprints, and explicit route denial reasons.
- `P14 -- Recovery Boundaries Are Explicit`: mid-run provider failure creates `RecoveryAction` with provider cause, side-effect classification, preserved trace, replayability classification, and reroute candidates.
- `P15 -- Local Control of Graph and Provenance`: the proposal reconciles local control with vendor auth reality by storing only redacted derived `ProviderState` and `EntitlementSnapshot` records, never secret material. Vendor auth stores remain external evidence sources; the harness-owned graph, provenance, audits, route decisions, and recovery records remain local and exportable.

This is a compatible application of `P15`, not a violation. The proposal does not claim to own `~/.claude/`, `~/.codex/`, `~/.config/opencode/`, billing accounts, or local runtime registries. It records what was observed, where it was observed, when it was observed, and with what confidence. That preserves the harness's source of truth for graph/provenance while refusing to pretend external credentials are locally controllable.

### GraphConfiguration / ProviderState / EntitlementSnapshot trio

The three new schema objects are mutually coherent:

- `GraphConfiguration` defines the memory and routing policies that shape renders, optimizer scope, indexing, summary contracts, provider routing, and capability probing. It is versioned, local, cited by snapshots and edits, and validated before use. This embodies `P1`, `P3`, `P7`, `P13`, and `P15`.
- `ProviderState` records redacted observable provider/CLI/account/runtime state with auth, billing, quota, network, runtime, sandbox, freshness, confidence, and checked-source fields. It is produced by `ProviderStateMonitor`, not the orchestrator, and is referenced by routing, render, budget, recovery, and audit records. This embodies `P8`, `P13`, `P14`, and `P15`.
- `EntitlementSnapshot` records point-in-time model and feature availability, probe method, evidence, and validity interval without storing credentials. It feeds capability fingerprints, routing policy, launch preflight, and recovery reroute decisions. This embodies `P8`, `P12`, `P13`, and `P15`.

No new contradiction appears among the trio. `GraphConfiguration.provider_routing_policy_ref` can require provider properties, while `ProviderState` and `EntitlementSnapshot` provide observed evidence about whether those requirements are satisfied. Invalid or stale observations block, degrade, or revalidate routing rather than being treated as truth.

### New operational sections

**Configuration Inspection and Validation** preserves the existing `P5` boundary. It can produce audit events and advisory optimizer requests when graph shape appears inconsistent with configuration, but it does not silently rewrite graph truth. It also reinforces `P1`, `P3`, `P7`, `P13`, and `P15` by making empty-graph behavior and graph-shape explanations operational outputs, not detached help text.

**Provider Preflight and Routing** preserves existing recovery and question-routing semantics. It blocks or degrades workload-specific routes before launch, exposes route denial reasons, and on mid-run failure opens `RecoveryAction` rather than pretending a reroute is neutral. It does not alter `P11` question-as-continuation behavior: provider failure during a worker run is handled through worker/session/recovery state, not by converting blocked work into lead-chat replies.

## Violations

None.

## Ungrounded decisions

None.

## Structural concerns

None.

## Embodiment summary

- **§1 Effective Working Set vs. Nominal Context Window:** Embodied through **Imposed Working Set**, `WorkingSetSnapshot`, required pins, eviction order, effective-reasoning budgets, and render invalidation for noisy or overfull context. This follows `P1`, `P2`, and `P12`.
- **§2 Summary Contract:** Embodied through **Summary Contract**, `SummaryContract`, `ProvenancePointer`, invalid summary states, and **Summary Regeneration**. This follows `P3` and `P7`.
- **§3 Concurrent Optimizer Mutation:** Embodied through **Snapshot-Walk-Then-Merge**, immutable `GraphSnapshot`, append-only `OptimizerEdit`, bounded foreground `GraphAction`, advisory `OptimizerRequest`, `ConflictRecord`, and turn-boundary visibility. This follows `P1`, `P5`, `P6`, and `P14`.
- **§4 Stable Identity:** Embodied through opaque stable IDs, `IdentityEvent`, forwarding maps, snapshot-scoped identity resolution, and optimizer-owned topology edits. This follows `P1`, `P4`, `P5`, and `P15`.
- **§5 Working-Set Policy:** Embodied through `AgentWalkState`, pack/unpack/focus/pin tools that modify walk state rather than graph topology, required pins, eviction rules, and recursive unpack bounds. This follows `P2` and `P5`.
- **§6 Hierarchical Packing:** Embodied through containment and packing edges, recursive unpack bounds, stable identity across topology changes, and optimizer-owned repack edits. This follows `P2`, `P4`, and `P6`.
- **§7 Cross-CLI Rendering Asymmetry:** Embodied through **Cross-CLI Adaptation**, CLI-specific render paths, and `CapabilityFingerprint` surfaces for weaker injection, resume, tooling, entitlement, runtime, and sandbox capabilities. This follows `P8` and `P13`.
- **§8 Sub-Agent Supervision:** Embodied through `WorkerSlice`, `WorkerRun`, staged worker output, overlap policy, acceptance tracking, provider preflight, and advisory `OptimizerRequest` for curation-affecting candidates. This follows `P5`, `P7`, `P8`, `P10`, and `P12`.
- **§9 User-Question Routing:** Embodied through **Questions as Continuations**, `QuestionArtifact`, **NEEDS_INPUT Routing**, child acceptance tracking, and refusal to substitute lead-chat append for worker resume. This follows `P11` and `P14`.
- **§10 Tool-Call Protocol Provenance:** Embodied through `EvidenceArtifact`, `ToolCallProvenance`, deterministic protocol gates, exact correlation IDs, approval and retry semantics, and evidence-linked summaries. This follows `P7`, `P10`, and `P14`.
- **§11 Workflow-Reviewer Reliability:** Embodied through `WorkflowReviewer` as evidence only, deterministic gates, reviewer sampling, and explicit reviewer limitations. This follows `P10` and `P12`.
- **§12 Graph Poisoning:** Embodied through `privilege_origin`, `trust_state`, quarantine, `ProvenancePointer.privilege_transform`, privilege/poisoning controls, and deterministic validation. This follows `P1`, `P7`, `P10`, and `P15`.
- **§13 Cost and Resource Tails:** Embodied through **Cost as Correctness**, `BudgetLedger`, budget-gated optimizer/reviewer/worker behavior, cache-prefix observability, quota interpretation, and cadence narrowing. This follows `P12`.
- **§14 Multi-Workstream Legibility:** Embodied through the single-tab `UserSurface`, initiative/status panes, working-set inspector, configuration inspector, provider panel, global question queue, worker board, optimizer log, evidence drill-down, cost surface, recovery surface, and notification separation. This follows `P1`, `P13`, `P14`, and `P16`.
- **§15 Imposed Context Precedent Gap:** Embodied through graph-derived renders, visible imposed working sets, local graph canonicality, no in-product compaction, bounded foreground actions, advisory requests, and optimizer-owned curation. This follows `P1`, `P5`, `P9`, and `P15`.
- **§16 Recovery and Resume Surfaces:** Embodied through `RecoveryAction`, recovery preflight, explicit preserved/replayed/discarded records, failed-resume handling, provider-aware recovery, audit events, and user-visible recovery surfaces. This follows `P14` and `P15`.
- **§17 Graph and Memory Configuration Overhead:** Embodied through **Configuration as Memory Semantics**, `GraphConfiguration`, configuration provenance, empty-graph simulation, shape explanations, configuration validation gates, configuration-aware summary regeneration, and configuration/accountability audits. This follows `P1`, `P3`, `P7`, `P13`, and `P15`.
- **§18 Provider, Account, and Entitlement Friction:** Embodied through **Provider State as Observable State**, `ProviderState`, `EntitlementSnapshot`, extended `CapabilityFingerprint`, provider preflight, workload-specific route denial reasons, redacted observation rather than credential ownership, budget/quota awareness, and provider-aware recovery. This follows `P1`, `P8`, `P12`, `P13`, `P14`, and `P15`.

No new philosophical surfaces were found.
