# Phase 0C AI Roadmap — Dependency Risk Assessment (Round 5)

**Rating: LOW**

Phase 0C round 5 is a focused brownfield revision over round 4 (77 WUs → 76 WUs), driven externally by proposal-r6 (commit `20d4995`) and engineering-roadmap-r5 (commit `1a3f089`). Option A landed: agent-runner shipped the five SessionOverrideContract feature requests (`agents session locate / export / import-replace / pause-handshake / schema-probe`), so the v1 direct `AgentRunnerDbAdapter` is dropped, WU-0C-N3 is rescoped to the v2 `AgentRunnerCliAdapter`, and WU-0C-N4 (the standalone `AgentRunnerSchemaProbe`) is removed because schema compatibility is now a single CLI call inside N3's construction-time `safe_for_import_replace` gate. Every block-on annotation in the artifact is gone, and N3 has shed three Phase 0C dependencies (N4, N5, WU-0C-16) that only existed for the v1 storage-layout path.

Independent verification of the round-5 graph and Parallelization Map confirms the LOW criteria are met: the 76-WU dependency graph is acyclic; the 10-wave Parallelization Map contains no intra-wave dependency edges; the wave member count sums to 76 (23+18+13+6+5+4+2+2+2+1); the round-5 movement of WU-0C-N3 from W4 to W3 is correct under longest-path-from-leaf+1 reconstruction (max prereq is WU-0C-N1 at W2, so N3 = W3); WU-0C-N4 has been excised from every dependency edge, every Stitch Notes block, and every consumer-VS list; the only remaining mentions of N4 are explicit "Removed in r5" stubs and a revision-rationale crumb for traceability; bidirectional consistency holds against the Phase 1 r4 (`worktrees/phase-1-ai-roadmap-r4`) and Phase 2 r5 (`worktrees/phase-2-ai-roadmap-r5`) parallel-dispatch artifacts, which already drop N4 references and remove block-on annotations on their respective branches; D3/D4 honestly classify round 5 as `fix-created-family` gen 0 cascade-driven simplification with the carried-forward `session-override-boundary-family` watch updated for "do not recreate a harness-side DB adapter or schema-wrapper WU".

The single residual finding is the same pre-existing wave-pessimistic placement of WU-0C-16 (declared in Wave 3 but its Phase 0C deps are all Wave 1, so topological depth is Wave 2). This was inherited byte-for-byte from rounds 2/3/4; round 5 did not touch it. It does not violate the "no intra-wave dependency edges" rule and remains LOW. All round-5-specific work — the WU-0C-N3 rescope, the N4 removal, the dependency-edge cleanup, and the wave rebalance — is correctly placed.

A scope clarification, not a finding: the engineering-roadmap.md on this worktree branch is still at r4 (`product-strategy/engineering-roadmap.md` line 36 reads "v1 `AgentRunnerDbAdapter`" and line 669 reads "Phase 0C-r4"). The dispatch contract explicitly anticipated this — engineering-roadmap-r5 is a parallel dispatch on a sibling worktree (`worktrees/engineering-roadmap-r5/product-strategy/engineering-roadmap.md` line 36 reads "`AgentRunnerCliAdapter`" and line 670 reads "Phase 0C-r5"). Bidirectional consistency is verified against the parallel artifacts. No action required at this gate.

---

## Findings

### R5-DEP-F01. Internal 76-WU graph acyclic; round-5 wave map re-derives correctly with WU-0C-N3 at Wave 3

**Severity: NONE (positive finding)**

Independent topological reconstruction of the Detailed Phase 0C edges block (`product-strategy/ai-roadmap-phase-0c.md` lines 3232-3321) succeeds. Every Phase 0C WU's incoming edges resolve to either Phase 0A/0B WUs or Phase 0C WUs in strictly lower waves. No back-edges exist. Total nodes = 76 (77 from round 4 minus WU-0C-N4).

Round-5 SessionOverrideContract family verifies cleanly under longest-path-from-leaf+1 reconstruction:

- **WU-0C-N2** (DTOs) `<- WU-0B-09, WU-0B-15, WU-0B-21` (line 3275). Phase 0C deps: none. Wave = 1. Declared Wave 1 ✓.
- **WU-0C-N1** (trait) `<- WU-0C-N2, WU-0B-09, WU-0B-15` (line 3276). Phase 0C deps max wave = 1. Wave = 2. Declared Wave 2 ✓.
- **WU-0C-N3** (`AgentRunnerCliAdapter`) `<- WU-0C-N1, WU-0C-N2, WU-0A-15, WU-0B-09, WU-0B-15, WU-0B-31` (line 3277). Phase 0C deps max wave = 2 (N1). Wave = 3. Declared Wave 3 ✓. **Round-5 movement: W4 → W3.** This is the intended consequence of dropping the N4, N5, and WU-0C-16 prerequisites that only existed for v1 storage-layout and registry-coupled coordination. The CLI adapter no longer requires a Wave-3 prerequisite, so its topological depth drops to W3.
- **WU-0C-N5** (registry) `<- WU-0C-N1, WU-0C-N2, WU-0B-09, WU-0B-15, WU-0B-31` (line 3278). Phase 0C deps max wave = 2 (N1). Wave = 3. Declared Wave 3 ✓.

Wave-member sum check: 23+18+13+6+5+4+2+2+2+1 = 76 ✓ matches the inventory total at line 3388.

Per-wave delta from round 4 (77 WUs → 76 WUs):

| Wave | r4 count | r5 count | Δ | Cause |
|---:|---:|---:|---:|---|
| 1 | 23 | 23 | 0 | unchanged |
| 2 | 19 | 18 | −1 | WU-0C-N4 removed (was W2 in r4) |
| 3 | 12 | 13 | +1 | WU-0C-N3 moved in from W4 (W3 in r5) |
| 4 | 7 | 6 | −1 | WU-0C-N3 moved out to W3 |
| 5..10 | 5,4,2,2,2,1 | 5,4,2,2,2,1 | 0 | unchanged |

The two ±1 deltas in Waves 2/3/4 are exactly accounted for by the two structural changes (drop N4, move N3 earlier), and the W3/W4 swap is consistent with N3's reduced prerequisite set.

Intra-wave dependency check: zero violations across all 10 waves under independent reconstruction. Spot-checks in the affected waves:

- **Wave 2 (18 members):** every Phase 0C dep of every member resolves to Wave 1. WU-0C-02 ← {01,02a}; WU-0C-03 ← {01}; WU-0C-06 ← {05,06a}; WU-0C-09 ← {08,09a}; WU-0C-10 ← {08}; WU-0C-12a ← {11b}; WU-0C-13a ← {11a,11b}; WU-0C-13b ← {11b}; WU-0C-14a ← {11b}; WU-0C-17a ← {16b}; WU-0C-N1 ← {N2}; WU-0C-20a ← {19}; WU-0C-22a ← {21}; WU-0C-23b ← {01}; WU-0C-24 ← {24a}; WU-0C-29a ← {01,05}; WU-0C-30 ← {30a}; WU-0C-35 ← {35a}. All 18 prerequisites land in Wave 1.
- **Wave 3 (13 members):** every Phase 0C dep resolves to Wave 1 or Wave 2. WU-0C-04 ← {01,02,03,04a}; WU-0C-12 ← {11a,11b,12a}; WU-0C-13 ← {11a,11b,13a,13b}; WU-0C-14b ← {14a}; WU-0C-15b ← {13b}; WU-0C-16 ← {16a,16b}; WU-0C-N3 ← {N1,N2}; WU-0C-N5 ← {N1,N2}; WU-0C-22 ← {08,10,21,22a}; WU-0C-25a ← {21,24}; WU-0C-26a ← {21,22a,24}; WU-0C-31a ← {30}; WU-0C-37 ← {35,37a}. None of the 13 W3 members has an edge to another W3 member.
- **Wave 4 (6 members):** WU-0C-07 ← {04,05,06,07a}; WU-0C-14 ← {11b,14a,14b}; WU-0C-15c ← {15a,15b}; WU-0C-23 ← {04,23a,23b}; WU-0C-29c ← {04}; WU-0C-32 ← {11a,11b,12,13}. All resolve to Waves 1–3.

The acyclicity-check bullets at lines 3323-3330 of the artifact remain factually accurate after the r5 edits, including the updated bullet at line 3328 that now lists "WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5" (no N4) as the SessionOverrideContract family.

**Recommendation:** No action required. The graph and wave map are valid for the round-5 simplification.

---

### R5-DEP-F02. WU-0C-N3 incoming edges correctly cleaned of dropped N4 / N5 / WU-0C-16 prerequisites

**Severity: NONE (positive finding)**

The round-4 WU-0C-N3 (v1 `AgentRunnerDbAdapter`) had dependencies `WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16; incoming WU-0A-03 LocalStorageLayout, WU-0A-15 FakeAgentsFixture, WU-0B-09 EvidenceArtifact, WU-0B-15 AuditEvent` because the v1 adapter needed (a) the schema probe (N4), (b) the override registry handoff at construction (N5), (c) per-CLI storage-layout knowledge from `AgentRunnerConfigSnapshotReader` (WU-0C-16), and (d) raw local storage layout (WU-0A-03).

Round-5 WU-0C-N3 (v2 `AgentRunnerCliAdapter`) declares (line 1782): `WU-0C-N1, WU-0C-N2; incoming WU-0A-15 FakeAgentsFixture, Phase 0B WU-0B-09 EvidenceArtifact for agents invocation logs, WU-0B-15 AuditEvent for override receipts, and WU-0B-31 RecoveryAction for write failures.` The Detailed Phase 0C edges block (line 3277) matches: `WU-0C-N3 <- WU-0C-N1, WU-0C-N2, WU-0A-15, WU-0B-09, WU-0B-15, WU-0B-31`.

The four removed edges all match the v1-only design surface:

- **N4 removed** because schema-probe is now `agents session schema-probe` invoked in N3's `schema_version_probe` method and as a construction-time `safe_for_import_replace` gate (line 1774). No separate harness-side schema-probe wrapper exists.
- **N5 removed** because the registry write is no longer a construction-time prerequisite; N5 still records adapter receipts but adapter construction does not require a registry handle (the registry call sites live in downstream consumers, and N5 itself depends on N1/N2 only).
- **WU-0C-16 removed** because per-CLI storage layout is no longer needed: the v2 adapter only reads/writes through `agents session` commands and never touches `state.db` or per-CLI JSONL paths directly (line 1778 anti-scope criterion).
- **WU-0A-03 removed** because raw local storage layout was a v1-only concern. The v2 adapter still consumes WU-0A-15 `FakeAgentsFixture` for the integration test substrate — that edge is preserved.

The added edge **WU-0B-31 RecoveryAction** is consistent with N3 acceptance criterion line 1777 ("write-failure recovery metadata through WU-0B-31 refs"). It does not introduce a cycle: WU-0B-31 is a Phase 0B leaf, so it sits at wave-0 from the Phase 0C perspective and cannot raise N3's wave.

Cross-check against the family-level Stitch Notes summary: line 3575 reads "SessionOverrideContract WU-0C-N1/N2/N3/N5 consume WU-0B-09 EvidenceArtifact and WU-0B-15 AuditEvent refs for receipts/refusals, and WU-0B-31 RecoveryAction refs for quarantine handoff. They do not define a new graph source of truth." Per-WU declarations match this family-level summary; no Phase 0B reference is dangling.

WU-0C-N4 references were grep-checked across the artifact: only six explicit "removed" mentions remain, all of them stubs or revision-rationale traces (line 1735 "### WU-0C-N4: Removed in r5" stub block, line 1737 explanatory body, line 1790 N3 revision-rationale crumb, line 3221 refactor-ledger update, line 3531 D3 regression note, line 3539 D4 row, line 3552 round-5 self-classification). No live edge in the dependency graph, the Wave map, the D1 ownership table, the Critical Path, or any Stitch Notes block points to a Wave-2 N4 node; the artifact treats N4 as deleted, not preserved.

**Recommendation:** No action required.

---

### R5-DEP-F03. Bidirectional cross-phase consistency holds against parallel-dispatch downstream artifacts

**Severity: NONE (positive finding)**

The Phase 0C r5 outgoing-edge declarations and consumer-VS lists match what the parallel-dispatch downstream artifacts expect.

**Foundation-row outgoing block** (`product-strategy/ai-roadmap-phase-0c.md` line 3586): "SessionOverrideContract WU-0C-N1/N2/N3/N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata." The N4 reference is gone from the WU enumeration. ✓

**Per-VS Outgoing-to-Phase-2+ blocks** (lines 3610-3621): VS-010, VS-012, VS-018, VS-020, VS-021 each list `SessionOverrideContract WU-0C-N1/N2/N3/N5` (no N4) with concrete usage purposes:

- VS-010: "WU-0C-N1/N2/N3/N5 for turn-decomposition/detail-injection write-back" ✓.
- VS-012: "WU-0C-N1/N2/N3/N5 for repack transcript replacement" ✓.
- VS-018: "WU-0C-N1/N2/N3/N5 for accepted worker-output write-back" ✓.
- VS-020: "WU-0C-N1/N2/N3/N5 for refusal/quarantine metadata" ✓.
- VS-021: "WU-0C-N1/N2/N3/N5 for changed-session-contract evidence" ✓.

**Cross-phase outgoing edges** (lines 3623-3628): the Phase 1/2/3 destinations are still enumerated with the WU-0C-N1/N2/N3/N5 set (no N4) — worker-launcher/worker-output reintegration in Phase 1, turn-decomposition/detail-injection in Phase 2, repack-planner in Phase 3.

**Reciprocal incoming-from-Phase-0C edges** in the parallel-dispatch downstream artifacts:

- **Phase 1 r4** (`worktrees/phase-1-ai-roadmap-r4/product-strategy/ai-roadmap-phase-1.md`):
  - Round-4 audit table at lines 100-116 enumerates each affected Phase 1 WU's SessionOverrideContract dependency using only `WU-0C-N1`, `WU-0C-N2`, `WU-0C-N3`, `WU-0C-N5` (no N4); line 100 explicitly states "drops WU-0C-N4 references, and treats SessionOverrideContract as v2-only with schema-probe needs satisfied by WU-0C-N3 `schema_version_probe`."
  - Per-WU `Cross-phase incoming (SessionOverrideContract)` lines for WU-1-03/04/05/10/12/13/23/25/28/45/48/49/50 list only N1/N2/N3/N5.
  - Each affected WU's revision rationale carries "r4 cascade: agent-runner feature requests have landed; SessionOverrideContract v2-only; block-on annotations removed."

- **Phase 2 r5** (`worktrees/phase-2-ai-roadmap-r5/product-strategy/ai-roadmap-phase-2.md`):
  - Eight WU revision-rationale lines (1076, 1123, 1170, 1260, 2012, 2100, 2146, 2194) carry "r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed."
  - The D3 regression check at line 2419 records "Re-derived the Parallelization Map as unchanged because round 5 removes stale block-on annotations and the dropped schema-probe split edge."
  - The fix-created-family row at line 2509 names round 5 as "externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 SessionOverrideContract."

- **engineering-roadmap-r5** (`worktrees/engineering-roadmap-r5/product-strategy/engineering-roadmap.md`):
  - Line 36 needed-by row reads "`SessionOverrideContract` trait, `AgentRunnerCliAdapter`, schema-probe/safe-import gate, and fake-adapter fixtures | VS-010, VS-012, VS-018, VS-020, VS-021".
  - Line 670 binding row reads "Phase 0C-r5 | `SessionOverrideContract` trait, `AgentRunnerCliAdapter`, fake adapter, schema-probe feature gating, `safe_for_import_replace` verification...".
  - Line 826 cross-phase axiom reads "Phase 0C-r5 SessionOverrideContract -> VS-010, VS-012, VS-018, VS-020, VS-021."
  - Lines 832, 838, 848 each restate "Phase 0C-r5 SessionOverrideContract".

The five-VS consumer set is identical across Phase 0C r5, engineering-roadmap-r5, Phase 1 r4, and Phase 2 r5; the WU enumeration is identical (N1/N2/N3/N5 only); the schema-probe ownership is identical (inside N3, not a separate N4 WU); the v1 adapter is dropped uniformly.

A note on this branch's snapshot of `engineering-roadmap.md`: the file in this worktree is at the `engineering-roadmap-r4` revision (last touched by commit `b58d8c9` per `git log --oneline -- product-strategy/engineering-roadmap.md`), so locally it still reads "v1 `AgentRunnerDbAdapter`" at line 36 and "Phase 0C-r4" at line 669. The dispatch contract anticipates this: engineering-roadmap-r5 is a parallel dispatch on the sibling `engineering-roadmap-r5` worktree, and the substantive r5 updates live there. The Phase 0C r5 artifact on this branch correctly references "proposal-r6 / engineering-roadmap-r5" (lines 1790, 3539, 3552), and bidirectional consistency is verified against the parallel artifacts. The only obligation at the Phase 0C gate is that this artifact's own outgoing declarations match the engineering-roadmap-r5 contract, which they do.

**Recommendation:** No action required at the Phase 0C gate. The Phase 1 / Phase 2 / engineering-roadmap-r5 parallel artifacts already reciprocate the r5 simplification on their own branches.

---

### R5-DEP-F04. Block-on annotations fully absent across the round-5 artifact

**Severity: NONE (positive finding)**

A grep over `product-strategy/ai-roadmap-phase-0c.md` for `[Bb]locked.?on` and `[Bb]lock.?on` returns zero matches. The round-4 block-on annotations on WU-0C-N1 (line 1738), WU-0C-N3 (line 1845), WU-0C-N4 (line 1786), WU-0C-N2 (line 1686), and the per-VS blocks for VS-010 / VS-012 / VS-018 / VS-020 / VS-021 (lines 3669, 3671, 3677, 3679, 3680) — every one of which named at least one of `agents session locate / export / import-replace / pause-handshake / schema-version probe` — has been removed entirely.

This is consistent with the round-5 axiom that the upstream feature requests have landed (proposal-r6 commits #14-#23, surfaced in WU-0C-N3 revision rationale at line 1790). With the upstream surfaces shipping, there is no longer an unblocked-on dependency to track.

The five named CLI surfaces remain referenced as the *current* implementation contract of the v2 adapter, not as future deliverables: the WU-0C-N3 contract block (lines 1750-1761) lists each subcommand by name and the acceptance criteria (lines 1769-1778) bind each trait method to its CLI command, including the pause-handshake/resume-handshake lease lifecycle and the construction-time `agents session schema-probe` call.

**Recommendation:** No action required. Block-on annotation removal is complete and consistent with upstream landing.

---

### R5-DEP-F05. D3 Regression check and D4 watch-signal-compliance honestly describe round-5 simplification status

**Severity: NONE (positive finding)**

The dispatch criterion "D3/D4 honest" is satisfied:

- **D3 Regression check** (line 3531): "Round 5 SessionOverrideContract WUs now add four active nodes: DTOs in wave 1, trait in wave 2, CLI adapter plus registry in wave 3. WU-0C-N4 is intentionally removed because CLI schema-probe is a WU-0C-N3 construction gate. Existing session-turn WUs remain narrowed to normalized evidence." This accurately describes the r5 four-active-node state and explicitly justifies the N4 removal.
- **D4 fix-created-family row** (line 3539): "Round 5 classification is fix-created-family gen 0 from the proposal-r6/engineering-roadmap-r5 cascade. WU-0C-N3 is rescoped to the landed CLI surfaces and WU-0C-N4 is removed rather than preserved as a one-line wrapper." The "rather than preserved as a one-line wrapper" clause captures the simplification-vs-preservation decision honestly: the alternative would have been to keep N4 as a thin shim, and the proposer chose deletion.
- **D4 session-override-boundary-family row** (line 3541): "WU-0C-N3 shells out to `agents session` commands instead of writing storage directly." The watch is carried forward from r4 with its substantive content updated (per-CLI storage knowledge no longer mentioned because the boundary is now even tighter).
- **Self-classification round-5 line** (line 3552): "round-5 brownfield: proposal-r6 and engineering-roadmap-r5 drop the v1 DB adapter; WU-0C-N3 becomes the CLI adapter and WU-0C-N4 is removed. fix-created-family gen 0 remains externally driven."

The "Round 4 Refactor Ledger" header at line 3203 is preserved verbatim because the r4 narrowing decisions still hold; the r5 update is appended as a paragraph at line 3221 that explicitly states what changed in r5 ("WU-0C-N4 was removed... WU-0C-N3 was rescoped from direct DB/JSONL mutation to the CLI adapter") and what the post-r5 general rule is. This is an honest in-place edit, not a re-titled rewrite.

The D3 Regression check claim that "Parallelization Map is re-derived by topological depth from the declared graph and contains no intra-wave dependencies" (line 3530) holds for the round-5 wave rebalance under independent reconstruction (R5-DEP-F01). The pre-existing wave-pessimism in WU-0C-16 (R5-DEP-F06) is inherited from earlier rounds and does not invalidate the round-5-specific re-derivation claim.

The audit-history file `plans/audit/ai-roadmap-phase-0c.md` round-5 entry (lines 66-74) corroborates the D4 self-classification, names the same proposal-r6/engineering-roadmap-r5 driver, and lists the same wave sums (23+18+13+6+5+4+2+2+2+1 = 76). The proposer is consistent across the artifact and the audit history.

**Recommendation:** No action required.

---

### R5-DEP-F06. WU-0C-16 wave-pessimistic placement still inherited from rounds 2/3/4

**Severity: LOW**

Independent longest-path-from-leaf+1 reconstruction places WU-0C-16 (`AgentRunnerConfigSnapshotReader`) in Wave 2, but the artifact declares it in Wave 3 (`product-strategy/ai-roadmap-phase-0c.md` line 3394).

Verification:

- WU-0C-16 incoming edges (line 3269): `WU-0C-16 <- WU-0C-16a, WU-0C-16b, WU-0A-02, WU-0A-03, WU-0B-09`.
- WU-0C-16a is in Wave 1 (line 3392); WU-0C-16b is in Wave 1 (line 3392). Phase 0A/0B nodes are wave-0 leaves. Topological depth = max(1, 1) + 1 = 2.
- All other Wave 3 entries (12 of 13) have a verified Wave-2 prerequisite: WU-0C-04 (max-dep WU-0C-02/03 W2), WU-0C-12 (max-dep WU-0C-12a W2), WU-0C-13 (max-dep WU-0C-13a/b W2), WU-0C-14b (max-dep WU-0C-14a W2), WU-0C-15b (max-dep WU-0C-13b W2), WU-0C-N3 (max-dep WU-0C-N1 W2), WU-0C-N5 (max-dep WU-0C-N1 W2), WU-0C-22 (max-dep WU-0C-10 W2), WU-0C-25a (max-dep WU-0C-24 W2), WU-0C-26a (max-dep WU-0C-22a/24 W2), WU-0C-31a (max-dep WU-0C-30 W2), WU-0C-37 (max-dep WU-0C-35 W2). Only WU-0C-16's Wave-2 prerequisite is absent.

This is a wave-pessimistic placement, not an intra-wave dependency violation: WU-0C-16 sitting in Wave 3 instead of its true topological depth of Wave 2 does not break correctness. Downstream consumers (WU-0C-17 W6, WU-0C-18 W7, WU-0C-34 W9) are still strictly downstream. The only effect is reduced parallelism in Wave 2 (a 19-member wave under correct derivation, declared as 18-member).

The placement was inherited byte-for-byte from rounds 2/3 and r4. R4-DEP-F06 first flagged this finding at the round-4 dependency gate, also as LOW. Round 5 made no edits to WU-0C-16 or its declared wave; the round-5-specific changes (WU-0C-N3 rescope, WU-0C-N4 deletion, block-on annotation removal) are independent of WU-0C-16's placement.

`oscillation_classification: same-label (wave-pessimism on WU-0C-16). Pre-existing inheritance from rounds 2/3/4. Generation in Phase 0C local loop: 2 (R4-DEP-F06 was generation 1; R5-DEP-F06 is generation 2). Same-label-only, not same-family — still not a same-family ancestor of any prior dependency-encoding-family finding because R1-DEP-F01/F02 were intra-wave violations of different WUs (WU-0C-22, WU-0C-23) closed in round 2. The two-generation chain on WU-0C-16 wave-pessimism is a label-only oscillation of an inherited LOW finding and does not trigger any decompose threshold under the audit-history rules.`

**Recommendation:** Move WU-0C-16 from Wave 3 to Wave 2 in the Parallelization Map at line 3393-3394 (Wave 2: 18 → 19 members, Wave 3: 13 → 12 members; sums remain 76). Alternatively, justify the Wave-3 placement by adding an explicit dependency edge from WU-0C-16 to a Wave-2 WU (none currently exists). Optional and non-blocking for round 5 — the current placement is conservative-pessimistic, not unsafe.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R5-DEP-F01 | Internal 76-WU graph acyclic; round-5 Wave-3 placement of WU-0C-N3 verifies under longest-path-from-leaf+1 reconstruction; per-wave Δ from r4 (W2:−1, W3:+1, W4:−1) is consistent with the N4 drop and N3 movement | NONE |
| R5-DEP-F02 | WU-0C-N3 incoming edges correctly drop the v1-only N4/N5/WU-0C-16/WU-0A-03 prerequisites and add WU-0B-31; no dangling N4 reference in graph, wave map, D1 table, Critical Path, or Stitch Notes | NONE |
| R5-DEP-F03 | Bidirectional cross-phase consistency holds against engineering-roadmap-r5 (parallel worktree), Phase 1 r4, and Phase 2 r5; same-branch engineering-roadmap.md still on r4 is by parallel-dispatch design | NONE |
| R5-DEP-F04 | Block-on annotations fully absent across the artifact; the five `agents session` surfaces are referenced only as the current v2 contract, not as deferred deliverables | NONE |
| R5-DEP-F05 | D3 Regression check and D4 watch-signal-compliance honestly describe round-5 simplification with `fix-created-family` gen 0 cascade-driven and `session-override-boundary-family` carried forward | NONE |
| R5-DEP-F06 | WU-0C-16 wave-pessimistic placement (declared W3, topological depth W2) inherited from rounds 2/3/4; same-label generation 2 in Phase 0C local loop | LOW |

## Oscillation classification (round 5)

| Family | Findings | Same-family ancestor in Phase 0C local loop | Generation in Phase 0C |
|---|---|---|---|
| dependency-encoding-family | none in round 5 | R2-DEP-F01..F03 (closed in r3); R4-DEP-F01..F03 (closed at r4 LOW) | 0 (chain stops at gen 2; no recurrence) |
| wave-pessimism (label-only on WU-0C-16) | R5-DEP-F06 | R4-DEP-F06 (also LOW) | 2 (label-only; not same-family) |
| fix-created-family | none in round 5 | r1 watch carried; externally driven by proposal-r6/engineering-roadmap-r5 cascade | 0 (per D4 self-classification) |
| session-override-boundary-family | none in round 5 | newly declared r4 | 1 (carried forward, not re-fired) |
| simplification-drift (new watch in r5 audit) | none in round 5 | newly declared r5 | 0 |

The round-5 brownfield revision did not re-fire `dependency-encoding-family`. The four MEDIUMs from earlier rounds (R2-DEP-F01..F03 and the round-4 cross-phase consistency criteria) are all preserved closed: the foundation-row outgoing block continues to enumerate Phase 0C-scoped foundation rows including the SessionOverrideContract row at line 3586 (now reading "WU-0C-N1/N2/N3/N5", no N4); the per-VS Outgoing-to-Phase-1 / Outgoing-to-Phase-2+ blocks remain systematically derived (lines 3596-3621); the OptimizerRequest cross-slice contract still enumerates all six emitter→queue pairs (line 3594); WU-0C-N1/N2/N3/N5 entries integrate cleanly into the existing systematic structure with the N4 row excised everywhere it appeared.

R5-DEP-F06 is a same-label-only generation-2 chain on a single WU's wave-pessimistic placement and is not a same-family recurrence of R1-DEP-F01/F02 (which were intra-wave violations on different WUs). It does not trigger any decompose threshold. Decision posture is `continue → optional Wave-2 placement of WU-0C-16 in a future minor edit`; LOW gate is satisfied without that edit.

The new `simplification-drift` watch in the round-5 audit history (`plans/audit/ai-roadmap-phase-0c.md` line 74) is appropriately defensive: it asks the next reviewer to flag any reintroduction of a harness-side DB adapter, a harness-side schema wrapper WU, or a per-CLI storage parser. No such reintroduction exists in r5; the watch is forward-looking only.

## What LOW required (and was met)

LOW for the Phase 0C dependency gate at round 5 required:

1. **Topological sort acyclic across 76 WUs.** ✓ Verified by independent reconstruction. The N4 removal and N3 wave shift introduce no cycles.
2. **Parallelization Map re-derived correctly.** ✓ Wave sum = 76; ten waves; no intra-wave dependency edges; per-wave Δ from r4 (W2:−1, W3:+1, W4:−1) is fully accounted for by the two structural changes.
3. **Cross-phase incoming systematic.** ✓ WU-0C-N3 cleanly sheds the v1-only N4/N5/WU-0C-16/WU-0A-03 prerequisites; WU-0C-N1/N2/N5 incoming edges unchanged from r4 and remain consistent with their unchanged contract surfaces; no dangling reference to dropped N4 anywhere in the artifact.
4. **Bidirectional consistency.** ✓ Phase 0C outgoing-to-Phase-1/2/3 declarations match engineering-roadmap-r5 line 36 (needed-by table), lines 670/826 (Phase 0C-r5 binding/axiom), Phase 1 r4 audit table (lines 100-116), and Phase 2 r5 D3 row (line 2419). The five-VS consumer set is identical across all four artifacts.
5. **Block-on annotations fully absent.** ✓ Zero matches for `[Bb]lock(ed)?.?on` across the artifact; the five `agents session` surfaces are now current contract obligations, not future blockers.
6. **D3/D4 honest.** ✓ Round 5 is explicitly classified as `fix-created-family` gen 0 cascade-driven simplification, with `session-override-boundary-family` watch carried forward and the new `simplification-drift` watch added in the audit history; the audit-history round-5 entry corroborates.

The single LOW finding (R5-DEP-F06) is a pre-existing wave-pessimism inherited from rounds 2/3/4 and not introduced or modified by round 5; it does not violate any LOW criterion.

## What MEDIUM would have required (not present)

MEDIUM would have fired if any of the following had been observed:

- **Intra-wave violation introduced by the r5 wave rebalance.** None: the W2/W3/W4 deltas are arithmetic-clean and no same-wave WU-pair has a declared edge.
- **Dangling N4 reference.** None: every live edge previously pointing at N4 was rerouted (N4-as-trait-prereq → N3 construction-time gate) or dropped (N4 as a separately-listed dependency for N3).
- **Bidirectional inconsistency with parallel artifacts.** None: engineering-roadmap-r5 line 36, Phase 1 r4 audit table line 100, and Phase 2 r5 D3 row line 2419 all agree on the v1-drop / N4-removal / v2-only outcome.
- **Surviving block-on annotation.** None: zero matches in the artifact.
- **Dishonest D3/D4 — claiming greenfield convergence after the cascade.** None: D3/D4 explicitly mark r5 as cascade-driven brownfield with externally-driven fix-created-family gen 0.

HIGH would have required a cycle in the dependency graph or multiple orphan WUs created by the N4 drop; neither is present. The N4 removal is clean: no WU lost a needed prerequisite, no consumer-VS lost an outgoing edge it was relying on, and N3's reduced prerequisite set strictly tightens the topological wave depth rather than loosening it.
