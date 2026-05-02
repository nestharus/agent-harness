# Audit history — Layer 3 AI roadmap, **Phase 2 only**

## Purpose

Per-phase audit history for Phase 2 (Lead Orchestrator Loop and Initial Curation). Phases 0A (16 WUs), 0B (32 WUs), 0C (72 WUs), 1 (56 WUs) all converged at LOW/LOW/LOW.

## Scope

Phase 2 owns the orchestrator turn lifecycle, navigation tools, and the first optimizer-edit cycle that touches graph state. Per `engineering-roadmap.md` lines 631-639:

1. **VS-008** Navigate with Pack, Unpack, Focus, Pin, and Unpin (engineering order: position 1).
2. **VS-009** Run Bounded Orchestrator Turns with Advisory Optimizer Requests (position 2).
3. **VS-010** Refresh Summaries and Mark Stale Nodes (position 3).

Engineering-roadmap notes: VS-008 and VS-009 share tool/turn handling — their state machine should be owned by one lane. VS-010 can start optimizer prompt/schema work in parallel, but merge behavior must consume the same `OptimizerRequest`, budget, evidence, and audit APIs.

## Carryover watch signals (from Phase 0A/0B/0C/1 learnings)

| Signal | Family | Phase 0/1 outcome | Phase 2 expectation |
|---|---|---|---|
| `bundling-family` | per-schema-object | Closed in 0A r2, 0C r2, Phase 1 r2 (single-WU split). Re-fired across multiple phases as fixture-pack bundling (WU-1-45 → WU-1-56 split). | Apply Rule D1 strictly. Phase 2 has many per-VS DTOs (turn state machine, tool command schemas, optimizer prompt/edit DTOs, summary refresh DTOs) — each is its own WU. Watch for: tool-command + tool-result DTO bundling; optimizer-prompt + optimizer-edit bundling; orchestrator-turn-init + orchestrator-turn-commit bundling; summary-refresh-request + stale-mark bundling; fixture pack bundling per VS. |
| `state-machine-criteria-family` | functional contract criteria | Closed in 0A and 0B, preserved in 0C, did not re-fire in Phase 1. | Apply Rule D2 strictly. Phase 2 has the most state-rich phase to date: `OrchestratorTurn` lifecycle (the most complex state machine in the system per audit history); `AgentWalkState` walk-state lifecycle (pack/unpack/focus/pin/unpin transitions); `OptimizerEdit` lifecycle (drafted → validated → merged or conflict_on_stale_base); `OptimizerRequest` capture/commit transactions; `GraphAction` foreground transactions; SummaryNode stale-state transitions; turn capture/commit transaction state machine. Every transition gets a binary criterion; every invalid transition gets a rejection test. |
| `dependency-encoding-family` | reviewer flagged in 0B/0C | Closed at generation 2 in 0C r3; closed at generation 0 in Phase 1 r1 with the systematic-from-start pattern. | Phase 2 consumes all four upstream phases (0A, 0B, 0C, 1). The systematic-from-start pattern is mandatory: walk every (Phase 0X/1 foundation row, Phase 2 VS) pair and emit one Dependencies entry per pair. Phase 1 demonstrated this works in r1; Phase 2 should achieve r1 LOW Dep gate. |
| `fix-created-family` | proposer's local-fix tendency | Active but quieting through 0A/0B/0C; did not fire in Phase 1. | Low risk in Phase 2's first round; watch for r2 brownfields if they happen. |

## Phase 0/1 precedent (4-phase pilot of the per-phase pattern)

| Phase | WUs | Rounds | Verdict trajectory | Pattern |
|---|---|---|---|---|
| 0A | 16 | 2 | LOW/LOW/LOW (after WU-0A-14 split) | Single targeted brownfield. |
| 0B | 32 | 2 | LOW/LOW/LOW (after Parallelization Map re-derive) | Single targeted brownfield. |
| 0C | 72 | 3 | LOW/LOW/LOW (after r2 systemic split + r3 Stitch Notes systematic enumeration) | Two brownfields. |
| 1 | 56 | 2 | LOW/LOW/LOW (after WU-1-45 fixture-pack split) | Single targeted brownfield. The "systematic-from-start" lesson from 0C r3 paid off — Dep LOW in r1, Coverage LOW in r1; only `bundling-family` re-fired at single-WU scope. |

**Lesson for Phase 2:** Apply Phase 1's systematic-from-start pattern. Specifically:

- **Per-schema-object granularity from the start.** Including fixture packs as first-class WUs (the recurring single-WU bundling pattern). VS-008/VS-009/VS-010 each likely need ≥1 fixture-pack WU.
- **Topological-level Parallelization Map from the start.**
- **Systematic Stitch Notes incoming-edge enumeration from the start.** Phase 2 has FOUR upstream phases (0A/0B/0C/1) — be exhaustive.
- **Systematic Stitch Notes outgoing-edge enumeration from the start.** Phase 2 → Phase 3+.

## Phase 2 expected WUs (rough)

Phase 2 has 3 VSes with rich state-machine content. Per Rule D1, expect **35-50 WUs**:

- **VS-008 navigation** (~8-12 WUs): tool command schema (5 distinct: pack, unpack, focus, pin, unpin — each is a separate command), AgentWalkState extensions, walk-state transition validators, render invalidation service, audit emission for tool calls, fixture pack.
- **VS-009 orchestrator turns** (~12-18 WUs): OrchestratorBridge service, OrchestratorTurn state machine (the deepest state machine in the system), turn capture/commit transaction service, advisory OptimizerRequest queue emitter, /compact detection/avoidance, turn UI surface, GraphAction transaction service, parent-invocation propagation, fixture pack.
- **VS-010 summary refresh** (~10-15 WUs): OptimizerScoping service, optimizer prompt schema, optimizer response schema, OptimizerEdit validator (deterministic), OptimizerEdit merge service, conflict-on-stale-base classifier, summary regeneration UI, stale-marker UI, glm invocation adapter, fixture pack.

Plus shared scaffolding WUs if any new shared concerns emerge across the 3 VSes (e.g., a shared `TurnLifecycleEvent` enum extension, a shared optimizer payload envelope).

## Round summaries

### Round 1 (greenfield, 44 WUs)

**Verdict: Decomp LOW / Coverage LOW / Dep MEDIUM.**

- **Decomposition LOW.** 44 WUs with clean per-schema-object granularity. All Phase 0/1 carryover bundling watch signals (tool-command + tool-result, optimizer-prompt + optimizer-response, orchestrator-turn-init + orchestrator-turn-commit, summary-refresh-request + stale-mark, per-VS fixture pack) did NOT re-fire. 5 navigation commands split (WU-2-01..05); 3 fixture packs first-class (WU-2-12, WU-2-26, WU-2-43); 3 per-CLI turn adapters split (WU-2-22/23/24); 4 optimizer DTOs split; OptimizerEdit lifecycle across 5 WUs. WU-2-14 OrchestratorTurnStateMachine carries 13 state + 11 valid-transition + 6 invalid-transition criteria. D1 audit table inside Run Report with 44 rows. Two INFO non-blocking findings.
- **Coverage LOW.** All 3 engineering-roadmap items owned by 44 WUs. Every method-bearing WU (26 of 44) carries D2 binary criterion. WU-2-14 enumerates 12 states + 11 valid + 6 invalid transitions; WU-2-08 WalkStateTransitionValidator and WU-2-38 SummaryNodeStaleStateTransitionHandler enumerate transitions. `state-machine-criteria-family` in remission for Phase 2. Two INFO non-blocking findings.
- **Dependency MEDIUM** with 1 MEDIUM finding (R1-DEP-F01). Graph acyclic; every per-VS Foundation row encodes as specific upstream WU-0A/0B/0C/1 IDs; bidirectional consistency holds against Phase 0C and Phase 1 Outgoing-to-Phase-2+ blocks; OptimizerRequest emission contract honored (WU-2-18, WU-2-39 → WU-0C-27). **`dependency-encoding-family` does NOT re-fire** — Phase 1's systematic-from-start pattern paid off. The MEDIUM is from a new family `parallelization-map-family` at generation 0 in Phase 2 local loop (parent-loop ancestor at Phase 0B r1). Two intra-wave dependency violations:
  - **Wave 1:** WU-2-22, WU-2-23, WU-2-24 (per-CLI turn adapters) alongside their dep WU-2-13.
  - **Wave 2:** WU-2-31 alongside its dep WU-2-30.
  - Mechanical fix: re-derive waves using longest-path-from-leaf+1 (proven approach from Phase 0B r2 closure).
- **Carryover watch outcomes:** `bundling-family`, `state-machine-criteria-family`, and `dependency-encoding-family` all closed at generation 0. Only the new `parallelization-map-family` fired at generation 0 with mechanical fix path. Decision: dispatch r2 brownfield with topological wave re-derive only.

## Decision register

- **Phase 2 round 1 decision**: dispatch greenfield gpt-high proposer using engineering-roadmap.md Phase 2 section + sharpened operators + this audit history + the 4 converged upstream artifacts (`ai-roadmap-phase-0a.md`, `ai-roadmap-phase-0b.md`, `ai-roadmap-phase-0c.md`, `ai-roadmap-phase-1.md`) for cross-phase incoming-edge alignment. Apply Phase 1's systematic-from-start pattern. Result: 44 WUs, Decomp LOW / Coverage LOW / Dep MEDIUM. Single MEDIUM (Parallelization Map re-derive).
- **Phase 2 round 2 decision**: dispatch tightly-scoped brownfield to re-derive Parallelization Map using topological-level partition (longest-path-from-leaf+1). Apply the proven approach from Phase 0B r2 closure. Hard scope-cap: Parallelization Map section only — do not modify any of the 44 WUs themselves (Contract, Test boundary, Code boundary, Acceptance criteria, Dependencies). Do not modify the Dependency Graph (it is correct; the Parallelization Map is what's wrong). Generation-2 watch active for `parallelization-map-family` — if r2 emits a same-family wave-violation finding, it would be generation 1 (still no hard-decompose; that fires at generation 3).
- **Phase 2 convergence (post-r2)**: All three gates LOW. Phase 2 ai-roadmap-phase-2.md (44 WUs, 10 waves, OptimizerRequest contract honored) is the converged Phase 2 artifact. `parallelization-map-family` closes at generation 0 in Phase 2 local loop without same-family re-fire. Phase 2 converges in 2 rounds, matching the 0A/0B/Phase 1 precedent. Outgoing handoff: stitching agent reads the Stitch Notes section's per-VS Outgoing-to-Phase-3+ blocks. Advance to Phase 3.

### Round 2 (brownfield, 44 WUs unchanged — Parallelization Map re-derive)

**Verdict: Decomp LOW / Coverage LOW / Dep LOW. Phase 2 converged.**

- **Decomposition LOW (regression-only confirmed).** WU count 44 unchanged; D1 ownership table 44 rows unchanged; r2 diff has 0 hits in `Contract:` / `Test boundary` / `Code boundary` / `Dependencies` blocks. All round-1 closures preserved by construction. 8 NONE confirming-r1 findings + 1 INFO carry-forward.
- **Coverage LOW (regression-only confirmed).** Inventory and ownership unchanged; WU-2-14 OrchestratorTurnStateMachine still carries 12 state-reachability + 11 valid + 6 invalid transition criteria; WU-2-08 (6 valid + 4 invalid) and WU-2-38 (4 valid + 4 invalid) transition criteria intact. No new method/enum/state-machine introduced.
- **Dependency LOW.** R1-DEP-F01 closed: independent topological reconstruction confirms new 10-wave layout matches longest-path-from-leaf+1 for all 44 WUs. WU-2-22/23/24 now Wave 2 (children of WU-2-13 in Wave 1); WU-2-31 now Wave 3 (child of WU-2-30 in Wave 2). No intra-wave edges anywhere. WU bodies, Dependency Graph (32 edge lines), and Stitch Notes byte-identical to round 1; cross-phase systematic encoding, bidirectional Phase 0C/1 consistency, OptimizerRequest contract (WU-2-18, WU-2-39 → WU-0C-27), and per-WU parallelization claims all carry forward.
- **Oscillation closure:** `parallelization-map-family` closes at generation 0 in Phase 2 local loop without same-family re-fire — no generation 1 produced.

### Round 3 watch posture (cascade-driven brownfield, in flight)

This round is triggered by upstream Layer 0 context-management research (research-15-v2, research-16, research-17-v4) cascading through proposal-r4 + engineering-roadmap-r3 — NOT by a reviewer-flagged finding from Phase 2 r2.

**Cascade scope:**

- VS-010 expansion to absorb the full turn-decomposition / detail-injection / incremental-summary-update pipeline (5-7 new WUs).
- WU-2-34 rename from `GlmInvocationAdapter` to `OptimizerModelInvocationAdapter` (per operator's GLM exclusion). New service dispatches by task class to MiniMax-M2.7 (workhorse), Claude Opus 4.7 (conflict only), Claude Sonnet 4.6 (lead substrate), GPT-5.5 (reviewer substrate).
- All optimizer-cycle WUs gain a model-dispatch acceptance criterion referencing the per-task assignment matrix in proposal-r4 §1 / research-17-v4 §7.
- WU-2-16 OrchestratorBridgeService updated for cross-CLI provider routing per proposal-r4.
- Stitch Notes outgoing-to-Phase-3+ may grow with new WUs; cross-phase incoming-edge bidirectional consistency must be preserved.
- Parallelization Map re-derived for new WUs.

**Watch signals carried into r3:**

- `bundling-family` — apply Rule D1 strictly to new WUs (DetailRecordSchemaDto separate from TurnDecompositionService; IncrementalSummaryUpdateService separate from FullSummaryRegenerationService; etc.).
- `dependency-encoding-family` — bidirectional consistency with Phase 0A/0B/0C/1 incoming + outgoing-to-Phase-3+ must hold.
- `parallelization-map-family` — re-derive Parallelization Map topologically; verify no intra-wave deps after new WU placement.
- `state-machine-criteria-family` — every new method/enum/state-machine gets binary criteria.
- `fix-created-family` — round 3 IS a fix-created round (cascade-driven). Watch for spurious additions; new WUs must trace to engineering-roadmap-r3 / proposal-r4 sections.

**Hard scope-cap:**

- VS-008 + VS-009 WUs (WU-2-01..WU-2-28) preserved byte-for-byte except for WU-2-16 acceptance criteria addition.
- VS-010 WUs (WU-2-29..WU-2-44) updated for model-dispatch acceptance criteria; WU-2-34 renamed.
- New VS-010 WUs added per Rule D1.

If r3 ships clean (Decomp LOW / Coverage LOW / Dep LOW), Phase 2 is re-converged with the cascade integrated. If any gate fires MEDIUM, that's a fix-created-family event because cascade-driven changes shouldn't introduce findings.

### Round 4 (audit-driven minor edit-pass, 50 WUs preserved)

**Verdict entering proposer: minor edit-pass, not full r4 cascade.**

- **Round summary.** Phase 2 r4 is externally driven by proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 introducing `SessionOverrideContract`. The audit prescribed localized scope edits for WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, and WU-2-47, while WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, and WU-2-48..50 remain clean. WU count stays 50; 11-wave Parallelization Map remains unchanged because no Phase 2 internal edges are added.
- **D1 assessment: LOW.** No new object family is created. WU-2-22/23/24 remain separate provider capability adapters, narrowed to `AgentRunnerClient`-mediated invocation/resume evidence instead of per-CLI launch/resume ownership. WU-2-45 remains DTO-only; WU-2-46 and WU-2-47 remain separate services; WU-2-43 remains a fixture pack.
- **D2 assessment: LOW.** Affected WUs carry 8-12 binary criteria after edit. New criteria cover `agents` / `AgentRunnerClient` fixture assertions, canonical WU-0C-N2 `TranscriptTurn` evidence, WU-0C-N1 write-back methods, WU-0C-N5 receipts/refusals, and direct JSONL mutation prohibitions.
- **D3 assessment: LOW-MEDIUM until risk gates.** The proposer pass reads the audit report and upstream SessionOverrideContract WUs and applies the requested scope changes, but the 3-gate risk loop is intentionally separate after proposer lands. The main residual D3 risk is a missed cross-phase edge or outgoing Stitch Note wording mismatch, not decomposition.
- **Watch signals.** `fix-created-family` is generation 0 in the Phase 2-local loop, externally driven from the upstream SessionOverrideContract cascade. `dependency-encoding-family` is active only for targeted WU-0C-N* edge additions. `parallelization-map-family` should remain closed unless a reviewer finds a hidden Phase 2 internal edge. `session-override-boundary-family` should be watched for accidental direct provider CLI launch/resume or JSONL mutation language.

### Round 5 (brownfield Option A, 50 WUs preserved)

**Verdict entering proposer: targeted annotation and dependency cleanup only.**

- **Round summary.** Phase 2 r5 is externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 after the agent-runner session feature requests landed. Scope is limited to WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, and WU-2-47 plus Stitch Notes / Run Report references. WU count stays 50; the 11-wave Parallelization Map remains unchanged.
- **D1/D2 assessment: LOW.** No new object family or criterion family is created. The eight affected WUs keep the r4 SessionOverrideContract criteria and gain only the r5 cascade note that agent-runner feature requests have landed, v2-only is in force, and block-on annotations are removed.
- **D3/D4 assessment: LOW.** The obsolete schema-probe split references are removed from Phase 2 dependencies and incoming Stitch Notes; schema-probe coverage is satisfied through WU-0C-N3. `fix-created-family` remains generation 0 and externally driven; `parallelization-map-family` remains closed because no Phase 2 internal edge changed.
