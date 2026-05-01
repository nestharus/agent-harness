# Audit history — Layer 3 AI roadmap, **Phase 1 only**

## Purpose

Per-phase audit history for Phase 1 (Observable Imposed Context). Phase 1 is the first feature-phase after the foundation phases. Phases 0A (16 WUs, 2 rounds), 0B (32 WUs, 2 rounds), and 0C (72 WUs, 3 rounds) all converged at LOW/LOW/LOW.

## Scope

Phase 1 owns the operator-visible imposed-context surface that consumes the Phase 0 substrate (RenderEngine, BudgetLedger, PolicyEngine, ConfigurationRegistry runtime, ProviderStateMonitor, AgentRunnerClient facade, Tauri IPC layer, UI shell, audit emit pipeline). Per `engineering-roadmap.md` lines 618-629, Phase 1 contains:

1. **VS-001** Inspect Imposed Working-Set Renders.
2. **VS-002** Enforce Summary Contracts on Visible Nodes.
3. **VS-003** Capture Tool-Call Provenance and Audit Events.
4. **VS-004** Gate Renders with Budget Ledgers and Cache Locality.
5. **VS-005** Inspect Configuration as Memory Semantics.
6. **VS-006** Preflight Providers and Expose Capability Fingerprints.
7. **VS-007** Show Initiative Roots and Current Focus.

VS-003 backend evidence/audit spine and VS-004 budget/cache primitives are the engineering-order leaders; VS-001 acceptance waits on VS-003/VS-004 backbones; VS-002/VS-005/VS-006/VS-007 can lane in parallel after Phase 0 DTOs are frozen.

## Carryover watch signals (from Phase 0A/0B/0C learnings)

| Signal | Family | Phase 0 outcome | Phase 1 expectation |
|---|---|---|---|
| `bundling-family` | per-schema-object | Closed in 0A r2 (WU-0A-14 split). Re-fired in 0C r1 across agent-runner / render / optimizer / recovery / IPC families; closed in 0C r2 with systemic per-schema-object split (37 → 72 WUs). | Apply Rule D1 strictly. Phase 1 has many per-VS DTOs (RenderRequestDto, AuditEvent extensions, ProviderProbeRequest variants, ConfigurationInspectionRequest, FocusPathQuery) — each is its own WU. Watch for "input + output DTO" bundling pattern. |
| `state-machine-criteria-family` | functional contract criteria | Closed in 0A and 0B; preserved in 0C across 72 WUs. | Apply Rule D2 strictly. Phase 1 services have rich state machines: `WorkingSetSnapshot` lifecycle, `SummaryContract` validation states (valid / invalid_missing_evidence / invalid_conflict / invalid_stale / invalid_policy / needs_review), `ToolCallProvenance` lifecycle, `BudgetLedger` policy_action transitions, configuration warning emission paths. Every method/enum/transition gets binary criteria. |
| `dependency-encoding-family` | reviewer flagged in 0B/0C | Active across 0B (Parallelization Map fix in r2) and 0C (Stitch Notes systematic re-derive in r3). 0C local loop hit generation 2 before closure. | Phase 1 has the most cross-phase incoming dependencies (every Phase 1 VS consumes multiple Phase 0A/0B/0C WUs). **Topological-level wave partition required from the start.** **Cross-phase incoming-edge declarations to 0A/0B/0C must be systematic from the proposer's first draft, not patched later.** This is the highest-risk family for Phase 1. |
| `fix-created-family` | proposer's local-fix tendency | Active but quieting through 0A/0B/0C. | Low risk in Phase 1's first round; watch for r2 brownfields if they happen. |

## Phase 0 precedent (3-phase pilot of the per-phase pattern)

| Phase | WUs | Rounds | Verdict trajectory | Pattern |
|---|---|---|---|---|
| 0A | 16 | 2 | LOW/LOW/LOW (after WU-0A-14 split) | Single targeted brownfield. |
| 0B | 32 | 2 | LOW/LOW/LOW (after Parallelization Map re-derive) | Single targeted brownfield. |
| 0C | 72 | 3 | LOW/LOW/LOW (after r2 systemic split + r3 Stitch Notes systematic enumeration) | Two brownfields: r2 systemic family-level decomposition, r3 Stitch Notes enumeration. The systemic r2 decomposition grew the WU count from 37 → 72, then required a third round to enumerate Stitch Notes for the larger inventory. |

**Lesson for Phase 1:** The proposer should attempt the "systematic-from-start" pattern that closed 0C r3 in one round. Specifically:

- **Per-schema-object granularity from the start.** Do not bundle DTOs.
- **Topological-level Parallelization Map from the start.** Wave N contains only WUs whose dependencies all have wave numbers < N. Re-derive by computing each WU's longest-path-from-leaf depth.
- **Stitch Notes outgoing-edge enumeration from the start.** Walk the engineering-roadmap Phase 1 → Phase 2/3+ outgoing dependency rows once and emit one block per (Phase 1 foundation row, downstream VS) pair.
- **Stitch Notes INCOMING-edge enumeration from the start.** This is new for Phase 1 because Phase 1 is the first phase with three upstream phases. Walk each VS's "Foundation dependencies" row in `engineering-roadmap.md` lines 50-475 and emit one (Phase 0X WU, Phase 1 VS) edge per dependency.

## Round summaries

### Round 1 (greenfield, 55 WUs)

**Verdict: Decomp MEDIUM / Coverage LOW / Dep LOW.**

- **Decomposition MEDIUM** with 1 MEDIUM finding: R1-DECOMP-F01 — WU-1-45 RedactedProviderProbeService bundles service operation with FakeProviderProbeFixture (5-state fixture pack: ready/degraded/blocked/stale/probe_failed). Per Rule D1, fixture packs are first-class WUs (cf. WU-1-35 SummaryContractFixturePack). Engineering-roadmap VS-006 line 170 lists "fake provider probes" as separate substrate item. Single-WU blast radius, analogous to Phase 0A r2 WU-0A-14 split. Recommended split: 55 → 56 WUs. Plus R1-DECOMP-F02 LOW (Run Report has only 4-row Rule summary; per-WU D1 audit information lives in inventory table at lines 37-93 but not located inside Run Report section — locational only).
- **Coverage LOW.** 55 WUs cover every engineering-roadmap Phase 1 row (lines 618-629) and every per-VS "What is new" item (lines 52-192). D2 applied as two-layer pattern (systemic generic-2/generic-3 + named per-method/per-enum). All named state-machine watches preserved (SummaryContract 6-state, ToolCallProvenance lifecycle, BudgetLedger.policy_action, ImposedRenderLabel 10-state). `state-machine-criteria-family` does NOT re-fire. 2 INFO sub-findings non-blocking.
- **Dependency LOW.** Graph acyclic (55 nodes, 76 internal edges, 7 topological waves). Cross-phase incoming systematic — every Phase 1 WU lists specific WU-0A/0B/0C IDs; no vague "Phase 0 substrate" refs. Bidirectional consistency verified against Phase 0B/0C outgoing-to-Phase-1 blocks. Parallelization Map valid (no intra-wave deps; 17/12/9/5/7/3/2 = 55). Outgoing-to-Phase-2+ enumerates 24 (Phase 1 VS, Phase 2+ VS) pairs. D3/D4 honest. **`dependency-encoding-family` closes at generation 0 in Phase 1 local loop** — the proposer applied the systematic-from-start pattern (lesson from 0C r3) and got it right on the first pass.
- **Carryover watch outcomes:** `dependency-encoding-family` and `state-machine-criteria-family` did not re-fire. Only `bundling-family` re-fired at single-WU scope (R1-DECOMP-F01). Decision: dispatch r2 brownfield with single targeted split + Run Report D1 audit table relocation.

## Decision register

- **Phase 1 round 1 decision**: dispatch greenfield gpt-high proposer using engineering-roadmap.md Phase 1 section + sharpened operators + this audit history + the 3 converged Phase 0 artifacts (`ai-roadmap-phase-0a.md`, `ai-roadmap-phase-0b.md`, `ai-roadmap-phase-0c.md`) for cross-phase incoming-edge alignment. Result: 55 WUs, Decomp MEDIUM / Coverage LOW / Dep LOW. Single MEDIUM (WU-1-45 split).
- **Phase 1 round 2 decision**: dispatch tightly-scoped brownfield to (1) split WU-1-45 RedactedProviderProbeService + FakeProviderProbeFixture into separate WUs per Rule D1; (2) relocate Run Report's D1 ownership audit table to live inside the Run Report section (R1-DECOMP-F02 LOW). Hard scope-cap: WU-1-45 split + Run Report D1 table relocation only. Do not touch the 55-WU Dependency Graph or 7-wave Parallelization Map (only update for the +1 split-out fixture WU's incoming/wave assignment). Generation-2 watch active for `bundling-family` — if r2 emits a same-family bundling finding, hard-decompose triggers per audit-history convention.
- **Phase 1 convergence (post-r2)**: All three gates LOW. Phase 1 ai-roadmap-phase-1.md (2910 lines, 56 WUs, 7 waves: 18+12+9+5+7+3+2 = 56) is the converged Phase 1 artifact. WU-1-56 FakeProviderProbeFixture split out cleanly with 5 binary fixture-state criteria; WU-1-45 narrowed to RedactedProviderProbeService alone. Run Report D1 audit table now lives inside the Run Report section with 56 rows. `bundling-family` closes at generation 1 in Phase 1 local loop. Phase 1 converges in 2 rounds, matching the 0A/0B precedent. Outgoing handoff: stitching agent reads the Stitch Notes section's per-VS Outgoing-to-Phase-2+ blocks (24 enumerated pairs) to wire Phase 2+ slices to Phase 1 foundations. Advance to Phase 2.

### Round 2 (brownfield, 55 → 56 WUs)

**Verdict: Decomp LOW / Coverage LOW / Dep LOW. Phase 1 converged.**

- **Decomposition LOW.** R1-DECOMP-F01 closed: WU-1-45 narrowed to `run_redacted_provider_probe(...)` service only; new WU-1-56 `FakeProviderProbeFixture` owns the five-state fixture pack with 5 binary fixture-state criteria. R1-DECOMP-F02 closed: new `### Rule D1` subsection inside Run Report contains a 56-row per-WU audit table with explicit `D1 audit count: 56 rows = 56 WUs` assertion. WU count 56 (header inventory, per-WU table, Run Report Metric table all agree). No new bundled WU; per-CLI normalizer split, reader/writer split, service/DTO split, VS-005 8-WU split, WU-1-35 fixture-pack split all preserved.
- **Coverage LOW.** WU-1-56 declares 5 binary fixture-state criteria (ready/degraded/blocked/stale/probe_failed). WU-1-45 retains method-level service criteria for `run_redacted_provider_probe` and now carries explicit redaction-invariant criterion (was implicit in r1). 55 unchanged WUs retain R1 acceptance criteria byte-identical; only Wave-1 parallelizable lists extended to add WU-1-56. 2 INFO sub-findings (stylistic inconsistencies) non-blocking.
- **Dependency LOW.** Internal graph acyclic; 56 nodes, +1 leaf (WU-1-56) only. WU-1-56 correctly placed in Wave 1 with `Phase 1 internal: none`, no outgoing internal edges, mirrored cross-phase envelope of WU-1-45. WU-1-45's incoming edges byte-for-byte unchanged; split is concern-clean. Parallelization Map 18+12+9+5+7+3+2 = 56 (only Wave 1 grew by one). Outgoing-to-Phase-2+ preserves all 24 (Phase 1 VS, Phase 2+ VS) pairs; WU-1-56 correctly excluded. Bidirectional consistency with Phase 0B/0C unchanged. D3/D4 honest.
- **Oscillation closure:** `bundling-family` closes at generation 1 in Phase 1 local loop. No same-label oscillation, no fix-created, no two-generation in-gate. No other family fires.

### Round 3 (brownfield cascade, 56 WUs unchanged)

**Proposer dispatch summary.** proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 introduced `SessionOverrideContract` plus WU-0C-N1..WU-0C-N5. Phase 1 r3 seeds from the converged r2 artifact and integrates those upstream boundaries without adding, deleting, or merging Phase 1 WUs.

- **D1 assessment: LOW with watch.** No WU collapsed. The artifact contains no worker-launcher, worker-output-reintegration, or orchestrator-bridge mutation WU, so the r3 change does not create a blob WU. Existing affected WUs are narrowed in place: WU-1-03/04/05 normalize already captured provider-shaped evidence, WU-1-10 owns read-only transcript ingestion through SessionOverrideContract, WU-1-12/13/23/25/28 display override-derived audit/evidence metadata, and WU-1-45/48/49/50 remain read-only provider preflight surfaces.
- **D2 assessment: LOW with targeted additions.** Affected WUs gained binary criteria forbidding direct JSONL/session-file access, provider routing, resume composition, session mutation, or hidden adapter behavior. WU-1-10 has explicit read-only criteria and a precise `Blocked-on` note for v2 read migration (`agents session locate` / `agents session export`). Downstream VS-015 and VS-018 stitch rows carry precise blockers for thin worker launch, v2 import/export migration, and `agents pause-handshake`.
- **D3 assessment: MEDIUM until gates run.** The proposer re-derived local topology and found no Phase 1-local wave reshuffle: 18+12+9+5+7+3+2 remains valid because WU-0C-N* are cross-phase incoming gates. Cross-phase overlay now names WU-0C-N1..WU-0C-N5 edges to WU-1-10 and read-only evidence/provider surfaces, plus downstream blockers for worker launcher/reintegration. Residual risk is dependency completeness: the separate decomposition / coverage / dependency gates should verify every Phase 0C-r4 outgoing SessionOverrideContract edge has the intended Phase 1 or downstream landing.
- **Watch signals.** `fix-created-family` reopens at generation 0 in the Phase 1-local loop, externally driven by the proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 cascade. This is not a local reviewer-discovered defect in r2, but it can create new dependency-encoding drift if downstream worker/reintegration WUs later duplicate direct JSONL or provider-routing responsibilities.
- **Decision register addendum.** Dispatch the three risk gates next on this branch. Do not dispatch them from the proposer. Gate focus should be: (1) whether affected WUs are truly thin/read-only, (2) whether SessionOverrideContract dependencies are neither missing nor over-broad, and (3) whether blocked-on annotations are precise enough for future VS-015/VS-018 owners.
