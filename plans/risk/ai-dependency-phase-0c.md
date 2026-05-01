# Phase 0C AI Roadmap — Dependency Risk Assessment (Round 4)

**Rating: LOW**

Phase 0C round 4 is a brownfield revision of the converged round-3 artifact (72 WUs → 77 WUs), driven externally by the proposal-r5 / engineering-roadmap-r4 SessionOverrideContract cascade. Five new WUs were added — WU-0C-N1 (trait), WU-0C-N2 (DTOs), WU-0C-N3 (v1 `AgentRunnerDbAdapter`), WU-0C-N4 (`AgentRunnerSchemaProbe`), WU-0C-N5 (`SessionOverrideStore` registry) — and existing agent-runner / session-turn WUs were narrowed to read-only evidence with revision rationale captured in `product-strategy/ai-roadmap-phase-0c.md` lines 3268-3278.

Independent verification of the round-4 graph and Parallelization Map confirms the LOW criteria from the dispatch instructions are met: the 77-WU dependency graph is acyclic; the 10-wave Parallelization Map contains no intra-wave dependency edges; the wave member count sums to 77 (23+19+12+7+5+4+2+2+2+1); WU-0C-N1..WU-0C-N5 each sit at the correct topological depth (N2 W1, N1/N4 W2, N5 W3, N3 W4); each new WU's Dependencies block lists specific Phase 0A/0B upstream WU IDs; bidirectional consistency holds against engineering-roadmap-r4 (`product-strategy/engineering-roadmap.md` lines 36, 663-669, 826) for all five consumer VSes (VS-010, VS-012, VS-018, VS-020, VS-021); blocked-on annotations cite named `agents` feature requests rather than vague "agents updates"; and D4 honestly classifies round 4 as `fix-created-family` gen 0 cascade-driven plus newly-named `session-override-boundary-family`.

The single residual finding is one pre-existing wave-pessimistic placement (WU-0C-16 declared in Wave 3 but its Phase 0C deps are all Wave 1, so topological depth is Wave 2). This was inherited byte-for-byte from round 2/3 and does not violate the "no intra-wave dependency edges" rule. It is LOW. All round-4-specific work — the five new SessionOverrideContract WUs, the narrowed agent-runner/session-turn WUs, and the cross-phase outgoing declarations — is correctly placed.

---

## Findings

### R4-DEP-F01. Internal 77-WU graph acyclic; 10-wave topology re-derives correctly for the round-4 additions

**Severity: NONE (positive finding)**

Independent topological reconstruction of the Detailed Phase 0C edges block (`product-strategy/ai-roadmap-phase-0c.md` lines 3290-3378) succeeds. Every Phase 0C WU's incoming edges resolve to either Phase 0A/0B WUs or Phase 0C WUs in strictly lower waves. No back-edges exist. Total nodes = 77 (72 from round 3 + WU-0C-N1..N5).

Round-4 additions verify cleanly under longest-path-from-leaf+1 reconstruction:

- **WU-0C-N2** (`TranscriptTurn` / `SessionLocation` / `SessionMetadata` DTOs) `<- WU-0B-09, WU-0B-15, WU-0B-21`. Phase 0C deps: none. Wave = 1. Declared Wave 1 ✓.
- **WU-0C-N1** (`SessionOverrideContract` trait) `<- WU-0C-N2, WU-0B-09, WU-0B-15`. Phase 0C deps max wave = 1. Wave = 2. Declared Wave 2 ✓.
- **WU-0C-N4** (`AgentRunnerSchemaProbe`) `<- WU-0C-N2, WU-0A-03, WU-0A-15`. Phase 0C deps max wave = 1. Wave = 2. Declared Wave 2 ✓.
- **WU-0C-N5** (`SessionOverrideStore` registry) `<- WU-0C-N1, WU-0C-N2, WU-0B-09, WU-0B-15, WU-0B-31`. Phase 0C deps max wave = 2 (N1). Wave = 3. Declared Wave 3 ✓.
- **WU-0C-N3** (v1 `AgentRunnerDbAdapter`) `<- WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16, WU-0A-03, WU-0A-15, WU-0B-09, WU-0B-15`. Phase 0C deps max wave = max(N1=2, N2=1, N4=2, N5=3, C-16=3) = 3. Wave = 4. Declared Wave 4 ✓.

The round-4 additions add no cycles. Specifically the N3→N5→N1→N2 and N3→N1→N2 paths are strictly forward; WU-0C-N3's wave-4 placement keeps it strictly downstream of every prerequisite. The "Acyclicity check" bullets at lines 3381-3388 of the artifact are factually accurate for the new family.

Wave-member sum check: 23+19+12+7+5+4+2+2+2+1 = 77 ✓ matches the inventory total.

Intra-wave dependency check: zero violations across all 10 waves under independent reconstruction. No pair of WUs in the same wave has a declared edge between them.

**Recommendation:** No action required for round-4 additions. The graph and wave map are valid for the new SessionOverrideContract family.

---

### R4-DEP-F02. Cross-phase incoming edges for WU-0C-N1..N5 are systematic and complete

**Severity: NONE (positive finding)**

Each new WU's `Dependencies` block lists specific named Phase 0A/0B upstream WU IDs, not vague "Phase 0B base":

- **WU-0C-N2** `Dependencies` (line 1678): `Incoming WU-0B-09 EvidenceArtifact, WU-0B-15 AuditEvent, WU-0B-21 WorkingSetSnapshot.` Matches the DTO contract's `evidence_ref`, audit-event linkage, and SessionRef-style references.
- **WU-0C-N1** `Dependencies` (line 1730): `WU-0C-N2; incoming WU-0B-09 EvidenceArtifact, WU-0B-15 AuditEvent.` Matches `OverrideReceipt`/audit-event-id fields in the trait.
- **WU-0C-N4** `Dependencies` (line 1778): `WU-0C-N2; incoming WU-0A-03 LocalStorageLayout, WU-0A-15 FakeAgentsFixture.` Matches the probe's local SQLite/binary fixture surface.
- **WU-0C-N3** `Dependencies` (line 1837): `WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16; incoming WU-0A-03 LocalStorageLayout, WU-0A-15 FakeAgentsFixture, WU-0B-09 EvidenceArtifact, WU-0B-15 AuditEvent.` Matches the adapter's state.db/JSONL/lock surface and audit/evidence emission.
- **WU-0C-N5** `Dependencies` (line 1895): `WU-0C-N1, WU-0C-N2; incoming WU-0B-09 EvidenceArtifact, WU-0B-15 AuditEvent, WU-0B-31 RecoveryAction.` Matches the `quarantined_storage_conflict` -> RecoveryAction quarantine handoff path declared at line 3634.

The Stitch Notes "Incoming edges from Phase 0B to preserve" block (line 3634) summarizes the family-level constraint: "SessionOverrideContract WU-0C-N1..WU-0C-N5 consume WU-0B-09 EvidenceArtifact and WU-0B-15 AuditEvent refs for receipts/refusals, and WU-0B-31 RecoveryAction refs for quarantine handoff. They do not define a new graph source of truth." Per-WU declarations are consistent with this family-level summary.

The dispatch hypothesis that "WU-0C-N3 needs WU-0B-17 ProviderState for provider name lookup" does not match the proposer's design. WU-0C-N3 routes provider/storage knowledge through WU-0C-16 `AgentRunnerConfigSnapshotReader` and WU-0C-N4 `AgentRunnerSchemaProbe`, not through WU-0B-17 (runtime provider availability). Under the proposer's contract this is correct: provider routing remains an `agent-runner` responsibility, and the v1 adapter only needs storage layout + schema fingerprints, both of which are config/probe concerns. WU-0B-17 is properly absent from N3's incoming.

**Recommendation:** No action required.

---

### R4-DEP-F03. Bidirectional cross-phase consistency holds for SessionOverrideContract outgoing edges

**Severity: NONE (positive finding)**

Phase 0C round-4 outgoing-edge declarations match what the upstream and downstream artifacts expect:

- **engineering-roadmap-r4 needed-by row** (`product-strategy/engineering-roadmap.md` line 36): `"SessionOverrideContract trait, v1 AgentRunnerDbAdapter, agent-runner-binding/schema-version probe, and fake-adapter fixtures | VS-010, VS-012, VS-018, VS-020, VS-021"`.
- **engineering-roadmap-r4 cross-phase axiom** (line 826): `"Phase 0C-r4 SessionOverrideContract -> VS-010, VS-012, VS-018, VS-020, VS-021."`
- **Phase 0C Stitch Notes foundation-row block** (line 3645): `"SessionOverrideContract WU-0C-N1..WU-0C-N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata."`

All three sources name the same five direct VS consumers. ✓

Per-VS blocks each enumerate the SessionOverrideContract dependency with a concrete usage purpose, not a vague "consumes contract":

- VS-010 (line 3669): `"SessionOverrideContract WU-0C-N1..WU-0C-N5 for turn-decomposition/detail-injection write-back"`.
- VS-012 (line 3671): `"SessionOverrideContract WU-0C-N1..WU-0C-N5 for repack transcript replacement"`.
- VS-018 (line 3677): `"SessionOverrideContract WU-0C-N1..WU-0C-N5 for accepted worker-output write-back"`.
- VS-020 (line 3679): `"SessionOverrideContract WU-0C-N1..WU-0C-N5 for refusal/quarantine metadata"`.
- VS-021 (line 3680): `"SessionOverrideContract WU-0C-N1..WU-0C-N5 for changed-session-contract evidence"`.

These purposes line up with each VS's `Session override impact` paragraph in engineering-roadmap-r4 (lines 251 VS-010, 315 VS-012, 423 VS-018; VS-020/021 cross-phase declarations at lines 826, 838, 848).

The cross-phase outgoing edges added in round 4 (lines 3684-3687) explicitly call out the Phase 1, Phase 2, and Phase 3 destinations:

- Phase 1 worker-launcher and worker-output-reintegration WUs (override receipt display, refusal surfacing, accepted-output write-back prerequisites);
- Phase 2 turn-decomposition and detail-injection-router WUs (must call `append_turns`/`truncate_after`/`replace_transcript` through WU-0C-N1);
- Phase 3 repack-planner WUs (may plan packed transcript replacement but must not open CLI JSONL files directly).

The note "Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1..WU-0C-N5" (line 3687) is honest: it acknowledges that the reciprocal incoming-from-Phase-0C edges in the Phase 1/2/3 artifacts are a deferred follow-up, not yet authored. This does not violate bidirectional consistency at the Phase 0C gate, because Phase 1/2/3 artifacts are not in scope for this gate; the Phase 0C side of the cross-phase contract is fully declared.

VS-009 is not listed as a SessionOverrideContract direct consumer, which is consistent with engineering-roadmap-r4 line 760 ("VS-009 + VS-010 partial | Shared OptimizerRequest lifecycle, audit events, and session refs consumed by SessionOverrideContract") — VS-009 shares only the OptimizerRequest substrate with VS-010; only VS-010 owns the contract write-back. The Stitch Notes correctly route VS-009 to optimizer queue WU-0C-27 and not to WU-0C-N1..N5.

**Recommendation:** No action required at the Phase 0C gate. When Phase 1/2/3 enter their respective dependency gates, those gates should verify the reciprocal incoming-from-Phase-0C edges back to WU-0C-N1..N5 are declared in the Phase 1/2/3 artifacts.

---

### R4-DEP-F04. Block-on annotations cite specific named agent-runner feature requests

**Severity: NONE (positive finding)**

Every blocked-on annotation in round-4 WUs and per-VS blocks names a specific upstream `agent-runner` feature request rather than vague "agents updates", satisfying the dispatch criterion:

- **WU-0C-N3** (line 1845): `"v1 ships now under schema-version pinning and idle-only writes. v2 swap-later is blocked on agents session locate, agents session export, agents session import-replace, pause-handshake, and schema-version probe. Atomic mid-session override remains blocked on agents pause-handshake."`
- **WU-0C-N1** (line 1738): `"None for v1 trait acceptance. v2 adapter migration is blocked on agents session locate, agents session export, agents session import-replace, pause-handshake, and schema-version probe."`
- **WU-0C-N4** (line 1786): `"v1 ships with local table/binary probing. Replacement by an upstream supported-surface probe is blocked on agents schema-version probe or equivalent."`
- **WU-0C-N2** (line 1686): `"None for the DTO contract. v2 population fields are placeholders until agents session locate/export/import-replace, pause-handshake, and schema-version probe land."`
- **VS-010 / VS-012 / VS-018 per-VS blocks** (lines 3669, 3671, 3677): each block carries `"Blocked-on: v2 adapter migration needs agents session locate/export/import-replace; atomic mid-session override needs agents pause-handshake. Until then VS-010|VS-012|VS-018 uses WU-0C-N3 v1 idle-only write-back / replacement / accepted-output staging."`
- **VS-020 per-VS block** (line 3679): `"Blocked-on: recovery actions that need live mid-session replacement require agents pause-handshake; v2 replacement requires agents session import-replace."`
- **VS-021 per-VS block** (line 3680): `"Blocked-on: v2 reroute/write-back migration requires agents session locate/export/import-replace and schema-version probe."`

The five named feature requests (`agents session locate`, `agents session export`, `agents session import-replace`, `agents pause-handshake`, `agents schema-version probe`) are consistent with engineering-roadmap-r4's "agent-runner-binding/schema-version probe" substrate-gap row (line 36) and its Phase 0C-r4 binding clause (line 669). Each blocked-on clause states which v1 fallback path applies until the named upstream feature lands, so the stitching agent can sequence v1-only work without waiting on substrate gaps.

**Recommendation:** No action required.

---

### R4-DEP-F05. D3 Regression check and D4 watch-signal-compliance honestly describe round-4 brownfield-cascade-driven status

**Severity: NONE (positive finding)**

The dispatch criterion "D3 should explicitly note round 4 is brownfield-cascade-driven" is satisfied:

- **D3 Regression check** (lines 3586-3591) documents: "SessionOverrideContract WUs add five new nodes: DTOs in wave 1, trait/probe in wave 2, registry in wave 3, and v1 adapter in wave 4. Existing session-turn WUs were narrowed to normalized evidence; no WU became empty or required removal." This accurately describes the round-4 cascade-driven additions.
- **D4 watch-signal-compliance** (line 3599): "`fix-created-family` — Round 4 classification is fix-created-family gen 0 from the proposal-r5/engineering-roadmap-r4 cascade. New concerns are assigned to WU-0C-N1..WU-0C-N5; existing agent-runner/session-turn WUs are narrowed rather than deleted." This is the explicit cascade-driven statement.
- **D4 watch-signal-compliance** (line 3601): "`session-override-boundary-family` — New in round 4. Provider routing, account/quota/auth, resume composition, session porting, and per-CLI storage knowledge are excluded from general harness WUs; direct DB/JSONL writes are isolated to WU-0C-N3 behind WU-0C-N1." This adds a new, named family watch tied to round 4's specific boundary risk.
- **Self-classification** (line 3611): "round-4 brownfield: SessionOverrideContract cascade integrated from proposal-r5 and engineering-roadmap-r4; fix-created-family gen 0 watch active for new adapter/trait dependencies."

The D3 Regression check claim that "Parallelization Map is re-derived by topological depth from the declared graph and contains no intra-wave dependencies" (line 3590) holds for the round-4 additions under independent reconstruction. The pre-existing wave-pessimism in WU-0C-16 (see R4-DEP-F06) is inherited from earlier rounds and does not invalidate the round-4-specific re-derivation claim, because no round-4 WU is mis-placed.

The audit-history file `plans/audit/ai-roadmap-phase-0c.md` lines 56-65 corroborates the D4 self-classification; the proposer is consistent across the artifact and the audit history.

**Recommendation:** No action required.

---

### R4-DEP-F06. WU-0C-16 wave-pessimistic placement carried over from round 3

**Severity: LOW**

Independent longest-path-from-leaf+1 reconstruction places WU-0C-16 (`AgentRunnerConfigSnapshotReader`) in Wave 2, but the artifact declares it in Wave 3 (`product-strategy/ai-roadmap-phase-0c.md` line 3453).

Verification:

- WU-0C-16 incoming edges (line 3326): `WU-0C-16 <- WU-0C-16a, WU-0C-16b, WU-0A-02, WU-0A-03, WU-0B-09`.
- WU-0C-16a is in Wave 1 (line 3451); WU-0C-16b is in Wave 1 (line 3451). Phase 0A/0B nodes are wave-0 leaves. Topological depth = max(1, 1) + 1 = 2.
- Declared Wave 3 contains 12 entries (line 3453). All other Wave 3 entries verify: WU-0C-04 (max-dep WU-0C-02 W2), WU-0C-12 (max-dep WU-0C-12a W2), WU-0C-13 (max-dep WU-0C-13a/b W2), WU-0C-14b (max-dep WU-0C-14a W2), WU-0C-15b (max-dep WU-0C-13b W2), WU-0C-22 (max-dep WU-0C-10 W2), WU-0C-25a (max-dep WU-0C-24 W2), WU-0C-26a (max-dep WU-0C-22a/24 W2), WU-0C-31a (max-dep WU-0C-30 W2), WU-0C-37 (max-dep WU-0C-35 W2), WU-0C-N5 (max-dep WU-0C-N1 W2). Each of those eleven has a Wave-2 prerequisite. Only WU-0C-16's Wave-2 prerequisite is absent.

This is a wave-pessimistic placement, not an intra-wave dependency violation: WU-0C-16 sitting in Wave 3 instead of its true topological depth of Wave 2 does not break correctness. Downstream consumers (WU-0C-N3 W4, WU-0C-17 W6, WU-0C-18 W7, WU-0C-34 W9) are still strictly downstream. The only effect is reduced parallelism in Wave 2 (a 20-member wave under correct derivation, declared as 19-member).

The placement was inherited byte-for-byte from round 3 (`worktrees/phase-0c-ai-roadmap-r3/product-strategy/ai-roadmap-phase-0c.md` line 3043 lists WU-0C-16 in Wave 3 of the 72-WU graph). Round 3 was rated LOW with the "preserved byte-for-byte" claim; round 4 inherits that placement without change. Round-4-specific work (WU-0C-N1..N5 plus narrowed agent-runner/session-turn WUs) is correctly placed.

`oscillation_classification: same-label (wave-pessimism on WU-0C-16). Pre-existing inheritance from round 3 — round 3 did not flag this issue. Generation in Phase 0C local loop: 1 (first time flagged at this gate). Not a same-family ancestor of any prior dependency-encoding-family finding because R1-DEP-F01/F02 were intra-wave violations of different WUs (WU-0C-22, WU-0C-23) that were closed in round 2.`

**Recommendation:** Move WU-0C-16 from Wave 3 to Wave 2 in the Parallelization Map at line 3452-3453 (Wave 2: 19 -> 20 members, Wave 3: 12 -> 11 members; sums remain 77). Alternatively, justify the Wave-3 placement by adding an explicit dependency edge from WU-0C-16 to a Wave-2 WU (none currently exists). Optional and non-blocking for round 4 — the current placement is conservative-pessimistic, not unsafe.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-DEP-F01 | Internal 77-WU graph acyclic; round-4 N1..N5 placement matches longest-path-from-leaf+1 reconstruction | NONE |
| R4-DEP-F02 | Cross-phase incoming edges for WU-0C-N1..N5 are systematic with named Phase 0A/0B WU IDs | NONE |
| R4-DEP-F03 | Bidirectional cross-phase consistency holds; engineering-roadmap-r4 and Phase 0C agree on five consumer VSes; cross-phase outgoing-only caveat for Phase 1/2/3 is honest | NONE |
| R4-DEP-F04 | Block-on annotations cite specific named `agents` feature requests (locate/export/import-replace/pause-handshake/schema-version probe) | NONE |
| R4-DEP-F05 | D3 Regression check and D4 watch-signal-compliance honestly describe round-4 brownfield-cascade-driven status with named family `session-override-boundary-family` | NONE |
| R4-DEP-F06 | WU-0C-16 wave-pessimistic placement (declared W3, topological depth W2) inherited byte-for-byte from round 3 | LOW |

## Oscillation classification (round 4)

| Family | Findings | Same-family ancestor in Phase 0C local loop | Generation in Phase 0C |
|---|---|---|---|
| dependency-encoding-family | none in round 4 | R2-DEP-F01..F03 (closed in r3) | 0 (chain stops at gen 2; no recurrence) |
| wave-pessimism (label-only) | R4-DEP-F06 | none (first observation at this gate) | 1 |
| fix-created-family | none in round 4 | r1 watch carried, externally driven by proposal-r5/engineering-roadmap-r4 cascade | 0 (per D4 self-classification) |
| session-override-boundary-family | newly declared | none (new in round 4) | 0 |

The round-4 brownfield revision did not re-fire `dependency-encoding-family`. Round 3's three MEDIUMs (R2-DEP-F01..F03) are all preserved closed: the foundation-row outgoing block continues to enumerate all 13 Phase 0C-scoped foundation rows (line 3636-3651), now extended to 14 with the SessionOverrideContract row (line 3645); the per-VS Outgoing-to-Phase-1 / Outgoing-to-Phase-2+ blocks remain systematically derived (lines 3657-3680); the OptimizerRequest cross-slice contract still enumerates all six emitter→queue pairs (line 3653); WU-0C-N1..N5 entries integrate cleanly into the existing systematic structure.

R4-DEP-F06 is a single-WU wave-pessimism observation, not a same-family recurrence of R1-DEP-F01/F02 (which were intra-wave violations on different WUs). It does not trigger any decompose threshold. Decision posture is `continue → optional wave-2 placement of WU-0C-16 in a future minor edit`; LOW gate is satisfied without that edit.

## What LOW required (and was met)

LOW for the Phase 0C dependency gate at round 4 required:

1. **Topological sort acyclic across 77 WUs.** ✓ Verified by independent reconstruction.
2. **Parallelization Map valid.** ✓ Wave sum = 77; ten waves; no intra-wave dependency edges; WU-0C-N1..N5 placed correctly.
3. **Cross-phase incoming systematic.** ✓ Each new WU's Dependencies block lists specific Phase 0A/0B upstream WU IDs (WU-0B-09, WU-0B-15, WU-0B-21, WU-0B-31, WU-0A-03, WU-0A-15) tied to concrete contract fields.
4. **Bidirectional consistency.** ✓ Phase 0C outgoing-to-Phase-1/2/3 declarations match engineering-roadmap-r4 line 36 (needed-by table), line 826 (cross-phase axiom), and per-VS `Session override impact` paragraphs at lines 251, 315, 423.
5. **Block-on annotations precise.** ✓ Every blocked-on clause cites at least one of `agents session locate/export/import-replace`, `agents pause-handshake`, or `agents schema-version probe`.
6. **D3/D4 honest.** ✓ Round 4 is explicitly classified as fix-created-family gen 0 cascade-driven, with `session-override-boundary-family` newly named to track the boundary; the audit history (`plans/audit/ai-roadmap-phase-0c.md` lines 56-65) corroborates.

The single LOW finding (R4-DEP-F06) is a pre-existing wave-pessimism inherited from round 3 and not introduced by round 4; it does not violate any LOW criterion.

## What MEDIUM would have required (not present)

MEDIUM would have fired if any of the following had been observed:

- **Intra-wave violation from a new WU.** None: WU-0C-N1..N5 placements respect their dependency chains.
- **Missing cross-phase edge.** None: each of N1..N5 declares specific 0A/0B upstream WU IDs; each consumer VS (010, 012, 018, 020, 021) lists SessionOverrideContract WU-0C-N1..N5 with concrete usage purpose.
- **Bidirectional inconsistency.** None: engineering-roadmap-r4 line 36, line 826, and Phase 0C Stitch Notes line 3645 agree on the five-VS consumer set.

HIGH would have required a cycle in the dependency graph or multiple orphan WUs; neither is present.
