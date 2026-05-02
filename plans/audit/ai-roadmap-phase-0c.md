# Audit history — Layer 3 AI roadmap, **Phase 0C only**

## Purpose

Per-phase audit history for Phase 0C (Shared Engines and Integration Shells). Phases 0A and 0B both converged in 2 rounds.

## Scope

Phase 0C owns the runtime engines and integration shells that consume Phase 0B's canonical state: RenderEngine core, PolicyEngine, BudgetLedger core, OptimizerScheduler/Cycle, ConfigurationRegistry runtime, ProviderStateMonitor, RecoveryAction processor skeleton, agent-runner subprocess supervisor (the various WU-011x families), Tauri IPC layer, UI shell wiring, audit emit pipeline.

## Carryover watch signals

| Signal | Family | Status | Phase 0C expectation |
|---|---|---|---|
| bundling-family | per-schema-object | Closed in 0A and 0B | Apply Rule D1 strictly. Phase 0C has fewer schema objects than 0B but more functional service objects (engines, schedulers, processors); each is its own WU. |
| state-machine-criteria-family | functional contract criteria | Closed in 0A and 0B | Apply Rule D2 strictly. Phase 0C engines have rich method signatures + lifecycle states; every method gets binary criteria. |
| fix-created-family | proposer's local-fix tendency | Active but quieting | Phase 0C is greenfield → low fix-created risk. |
| dependency-encoding-family | reviewer flagged in 0B r1 | Active (Phase 0B Parallelization Map fix in r2) | Phase 0C has the most cross-WU dependencies of any phase (engines depend on Phase 0B schemas + each other). Topological-level wave partition required from the start. |

## Phase 0A + 0B precedent

Both phases converged in 2 rounds with one targeted brownfield each. Phase 0A: WU-0A-14 split (Rust + TS halves). Phase 0B: Parallelization Map re-derive.

## Round summaries

### Round 1 (greenfield, 37 WUs)

**Verdict: Decomp MEDIUM / Coverage MEDIUM / Dep MEDIUM.**

- **Decomposition** flagged 4 MEDIUM: F-1 WU-0C-15 family-level input/output DTO bundling (verbatim repeat of parent-loop R6-DECOMP-F01); F-2 systemic family-level bundling across agent-runner / render / optimizer / recovery / IPC families; F-3 WU-0C-11 bundles two distinct schema_objects + two functions under unlisted exception; F-4 D1 audit table did not reflect splits.
- **Coverage** flagged 1 MEDIUM: F-1 missing identity/conflict shell WU (engineering-roadmap line 592 explicitly calls it out as Phase 0C deliverable; proposer omitted runtime owner of `IdentityEvent` / `ConflictRecord`).
- **Dependency** flagged 4 MEDIUM: F-1 WU-0C-22 wave assignment with WU-0C-10 in same wave; F-2 WU-0C-23 wave assignment with WU-0C-04 in same wave; F-3 missing identity/conflict shell incoming + outgoing edges; F-4 Stitch Notes outgoing-edge declarations systematically incomplete relative to engineering-roadmap Phase 0 foundation needed-by table; F-5 WU-0C-04 contradiction in "Parallelizable with" line; F-6 D3 Regression-check overstated.
- **Carryover watch:** all 3 named families (bundling-family, dependency-encoding-family, fix-created-family) re-fired in round 1. Decision: dispatch r2 brownfield with systemic family-level fixes, identity/conflict shell addition, and topological-level wave re-derive.

### Round 2 (brownfield, 37 → 72 WUs)

**Verdict: Decomp LOW / Coverage LOW / Dep MEDIUM.**

- **Decomposition LOW.** F-1/F-2/F-3 splits applied per Rule D1: agent-runner / render / optimizer / recovery / IPC families decomposed into per-schema-object WUs (37 → 72). F-4 D1 audit table now lists 72 entries matching inventory.
- **Coverage LOW.** Identity/conflict shell added as WU-0C-29c (IdentityResolverRuntime) and WU-0C-29d (ConflictRecordWriterShell), with Phase 0B incoming edges and per-VS outgoing edges declared.
- **Dependency MEDIUM** with 3 residual MEDIUM findings (R2-DEP-F01, F02, F03), all `dependency-encoding-family` at generation 2 in Phase 0C local loop. R1-DEP-F01/F02/F03/F05/F06 verified closed; internal 72-WU graph is acyclic with valid 10-wave topological partition (verified by reviewer's independent topo-sort). Residuals are entirely about Stitch Notes outgoing-edge declarations:
  - **R2-DEP-F01:** Foundation-row outgoing block enumerates 6 of 14+ Phase 0C foundation rows. Missing: PolicyEngine, CLI subprocess supervisor, ProviderStateMonitor, Optimizer queue, Recovery shell, Audit emit pipeline.
  - **R2-DEP-F02:** Per-VS blocks miss systematic engineering-roadmap-encoded edges — PolicyEngine WU-0C-04 missing from 9 of 17 VS, AuditEvent WU-0C-37 missing from 11 of 21 VS, CLI supervisor missing from VS-001 and VS-021.
  - **R2-DEP-F03:** Cross-slice OptimizerRequest emission contract — VS-005 and VS-020 outgoing edges do not include WU-0C-27 despite engineering-roadmap lines 606-616 declaring them as emitters.
- **Oscillation classification:** same-family at generation 2 (R1-DEP-F04 → R2-DEP-F01..F03). No three-generation chain in Phase 0C local loop yet. Reviewer explicitly authorizes `continue → proposer revises Stitch Notes`. No hard decompose trigger.

### Round 3 (brownfield, Stitch Notes only — 72 WUs unchanged)

**Verdict: Decomp LOW / Coverage LOW / Dep LOW. Phase 0C converged.**

- **Decomposition LOW (regression-only confirmed).** WU count 72 unchanged; D1 ownership table 72 rows unchanged; r3 diff has 0 hits in `Contract:` / `Test boundary` / `Code boundary` blocks. All round-2 closures preserved by construction (no Contract modified).
- **Coverage LOW (regression-only confirmed).** Identity/conflict shell (WU-0C-29c/29d) intact; D2 binary criteria intact across spot-checked engines (PolicyEngine, BudgetGateService, AgentRunnerClient, RenderEngine Core, OptimizerScheduler); all 13 Phase 0C-scoped foundation rows from engineering-roadmap lines 23-46 still owned by ≥1 WU.
- **Dependency LOW.** All 3 round-2 MEDIUMs closed cleanly under independent verification. Foundation-row block: 13 of 13 Phase 0C-scoped rows declared with consumer-VS lists matching engineering-roadmap Needed-by columns. Per-VS systematic edges: PolicyEngine×9, AuditEvent×11, CLI supervisor×2 all encoded; per-slice cross-check additions trace to lines 50-475. OptimizerRequest cross-slice contract: all 6 emitter→queue pairs explicitly enumerated. Internal graph and 10-wave Parallelization Map preserved byte-for-byte from round 2 (wave sums 22+17+11+6+5+4+2+2+2+1 = 72).
- **Oscillation closure:** `dependency-encoding-family` chain stops at generation 2 in the Phase 0C local loop. **Hard-decompose trigger does NOT fire.** Phase 0C converges in 3 rounds (one more round than 0A/0B due to the systemic family-level F-2 fix in r2 that grew the WU count from 37 → 72; Stitch Notes for the larger inventory then needed a third round to enumerate systematically).

### Round 4 (brownfield, SessionOverrideContract integration — 72 → 77 WUs)

**Verdict before gates: proposer self-assessment D1 LOW / D2 LOW / D3 LOW. Formal 3-gate risk loop is pending separate dispatch.**

- **Round summary.** Integrated the proposal-r5 and engineering-roadmap-r4 SessionOverrideContract cascade into the converged Phase 0C roadmap. Added WU-0C-N1..WU-0C-N5 for override DTOs, trait, schema probe, v1 `AgentRunnerDbAdapter`, and override registry. Refactored agent-runner/session-turn WUs so general harness slices do not encode provider routing, account/quota/auth, resume composition, cross-provider porting, or raw per-CLI transcript mutation. Re-derived the dependency graph and Parallelization Map from 72 to 77 WUs and updated Stitch Notes with outgoing SessionOverrideContract edges to VS-010, VS-012, VS-018, VS-020, VS-021, plus Phase 1/2/3 downstream handoff notes.
- **D1 decomposition risk: LOW.** The new family is split by concern: DTOs, trait surface, schema probe, v1 adapter, and registry are separate WUs with single-concern PR constraints. Existing WUs were narrowed rather than merged; no WU became empty or needed removal.
- **D2 coverage risk: LOW.** Each new WU has full Contract, test boundary, code boundary, dependencies, 6-12 binary acceptance criteria, blocked-on annotations, and explicit anti-scope. Existing affected WUs now include revision rationale and no-direct-storage/provider-routing constraints.
- **D3 dependency risk: LOW.** New WUs sit early: N2 wave 1, N1/N4 wave 2, N5 wave 3, N3 wave 4. Downstream write-back consumers depend on the trait/store rather than raw JSONL. v2 migration and atomic mid-session override are explicitly blocked on named agent-runner feature requests: `agents session locate`, `agents session export`, `agents session import-replace`, `agents pause-handshake`, and schema-version probe.
- **Watch signals.** `fix-created-family` gen 0, externally driven by proposal-r5/engineering-roadmap-r4 cascade. New `session-override-boundary-family` watch added so future revisions keep provider routing, session porting, and per-CLI storage ownership in `agent-runner` except for the pinned v1 adapter.

### Round 5 (brownfield, Option A SessionOverrideContract simplification — 77 → 76 WUs)

**Verdict before gates: proposer self-assessment D1 LOW / D2 LOW / D3 LOW. Formal 3-gate risk loop is pending separate dispatch.**

- **Round summary.** Applied proposal-r6 and engineering-roadmap-r5 Option A: drop the v1 direct `AgentRunnerDbAdapter`, make `AgentRunnerCliAdapter` the only SessionOverrideContract implementation, and remove WU-0C-N4 as a separate schema-probe WU. WU-0C-N3 now calls landed `agents session locate/export/import-replace/pause-handshake/resume-handshake/schema-probe` surfaces, with schema-probe enforced at adapter construction.
- **D1 decomposition risk: LOW.** WU count drops to 76 because N4's only remaining responsibility is a one-line CLI call inside N3. N1, N2, and N5 remain scoped to trait, DTO, and registry concerns; N3 owns only CLI adapter command mapping and fake-binary tests.
- **D2 coverage risk: LOW.** N3 acceptance criteria now cover command mapping, `unsupported-storage`, exit 13 `session-busy`, preimage mismatch, pause lease TTL/drop cleanup, construction-time `safe_for_import_replace`, and fake `agents` fixture cases. N4 removal is documented with redirect to N3's `schema_version_probe` criteria.
- **D3 dependency risk: LOW.** Block-on annotations were removed because the upstream CLI surfaces have landed. The graph removes N4, removes N3's N4/N5/WU-0C-16 dependencies, moves N3 to wave 3, and rebalances wave sums to 23+18+13+6+5+4+2+2+2+1 = 76 with no same-wave dependency edges introduced.
- **Watch signals.** `fix-created-family` gen 0, externally driven by upstream proposal-r6/engineering-roadmap-r5 and agent-runner feature landings. The watch item is simplification drift: do not recreate a harness-side DB adapter, harness-side schema wrapper WU, or per-CLI storage parser unless upstream removes the `agents session` contract.

## Decision register

- **Phase 0C round 1 decision**: dispatch greenfield proposer using engineering-roadmap.md Phase 0C section + sharpened operators + this audit history.
- **Phase 0C round 2 decision**: dispatch brownfield with systemic F-2 family-level decomposition fix, identity/conflict shell addition, topological-level wave re-derive. Result: Decomp LOW / Coverage LOW / Dep MEDIUM (Stitch Notes outgoing-edge gaps remain).
- **Phase 0C round 3 decision**: dispatch tightly-scoped brownfield to systematically enumerate Stitch Notes outgoing edges from the engineering-roadmap Phase 0 foundation table (lines 23-46) cross-checked against per-slice "Foundation dependencies" (lines 50-475). Do not modify the 72-WU dependency graph or wave map — the reviewer verified those LOW. Hard scope-cap: Stitch Notes section + D4 watch-signal-compliance row only. Generation-3 watch active for `dependency-encoding-family` — if r3 emits a same-family finding, hard-decompose triggers.
- **Phase 0C convergence (post-r3)**: All three gates LOW. Phase 0C ai-roadmap-phase-0c.md (3265 lines, 72 WUs, 10 waves, 13 foundation-row outgoing blocks, 21 per-VS outgoing blocks, OptimizerRequest cross-slice contract enumerated) is the converged Phase 0C artifact. Outgoing handoff: stitching agent reads the Stitch Notes section's foundation-row blocks + per-VS blocks to wire Phase 1+ slices to Phase 0C foundations. Advance to Phase 1.
- **Phase 0C round 4 decision**: dispatch brownfield from proposal-r5/engineering-roadmap-r4 to add SessionOverrideContract foundations before downstream VS-010/012/018/020/021 consume transcript write-back. Result: proposer revision at 77 WUs, 10 waves, formal Decomposition/Coverage/Dependency gates pending separate dispatch.
- **Phase 0C round 5 decision**: dispatch focused brownfield from proposal-r6/engineering-roadmap-r5 Option A after agent-runner SessionOverrideContract features landed. Result: proposer revision at 76 WUs, 10 waves, WU-0C-N3 rescoped to CLI adapter, WU-0C-N4 removed, formal gates pending separate dispatch.
