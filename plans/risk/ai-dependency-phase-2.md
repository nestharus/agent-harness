# AI Roadmap (Phase 2) — Dependency Risk Assessment (Round 4)

**Rating: LOW**

Round 4 is an audit-driven minor edit-pass triggered by the upstream `SessionOverrideContract` cascade (proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 WU-0C-N1..WU-0C-N5). The audit prescribed localized scope edits to WU-2-22/23/24 (narrow per-CLI provider capability adapters to `AgentRunnerClient`-mediated invocation/resume evidence) and to WU-2-43/45/46/47 (consume canonical `TranscriptTurn` evidence and route session write-back through `SessionOverrideContract`); WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, and WU-2-48..50 are preserved byte-for-byte from r3. WU count remains 50 and the 11-wave Parallelization Map carries forward unchanged because no Phase 2 internal dependency edges were added (`product-strategy/ai-roadmap-phase-2.md:2397-2412`). The four cascade-affected WUs add explicit Phase 0C incoming edges to WU-0C-N1..N5 (with WU-2-45 narrowed to N2 only, since it is a DTO that only consumes the canonical `TranscriptTurn` schema). WU-2-22/23/24 keep their full WU-0C-11a..18 Phase 0C incoming set and add a `WU-0C-18 AgentRunnerClient`-only routing acceptance criterion; their contracts now read "Provider capability adapter over AgentRunnerClient." Bidirectional consistency with Phase 0C-r4 outgoing-to-Phase-2 holds for the new SessionOverrideContract cluster: Phase 0C-r4 line 3669 declares VS-010 consumes WU-0C-N1..N5 for turn-decomposition/detail-injection write-back; Phase 2 r4 Stitch Notes "Incoming From Phase 0C" enumerates `(WU-0C-N1..N5, VS-010)` (`product-strategy/ai-roadmap-phase-2.md:2763-2767`). VS-009 still consumes the full agent-runner / provider / optimizer / hook / IPC / audit cluster declared in Phase 0C-r4 line 3668; VS-008 still consumes policy/budget/render/hook/audit declared in Phase 0C-r4 line 3667. Outgoing-to-Phase-3+ Stitch Notes for VS-012 (repack) and VS-018 (worker reintegration) are precise: WU-2-43 → VS-012/VS-018 fixture flow-through cites WU-0C-N1 fake-adapter and WU-0C-N3 v1 idle-only write-back cases; WU-2-46 → VS-012/VS-018 cites canonical turns and `WU-0C-N1 append_turns/truncate_after/replace_transcript`; WU-2-47 → VS-012/VS-018 cites `SessionOverrideContract receipts/refusals` for packed-transcript or canonical-turn write-back; topology mutation remains proposal-only (`product-strategy/ai-roadmap-phase-2.md:2926-2950`). D3 honestly enumerates the artifacts the proposer actually read (Phase 2 r3, Phase 2 r4 audit report, proposal r5, engineering-roadmap r4, Phase 0C r4 WU-0C-N1..WU-0C-N5) and explicitly disclaims dispatching the 3-gate risk loop. D4 classifies the round as externally-driven `fix-created-family` at generation 0 in the Phase 2-local loop, rates `parallelization-map-family` LOW with unchanged 11-wave topology, rates `dependency-encoding-family` LOW after the targeted WU-0C-N* edge pass, and adds a watch entry for `session-override-boundary-family` (no direct provider CLI launch/resume or JSONL mutation language). No cycle, no intra-wave edges, no missing cascade-relevant cross-phase edges, no Phase 0C-r4 outgoing declaration unmatched by a Phase 2 r4 incoming pair on the SessionOverrideContract surface. Dep gate is LOW; one INFO finding records pre-existing bidirectional over-declarations preserved by the hard scope cap.

---

## Findings

### R4-DEP-F01. Parallelization Map preserved byte-identical from r3 — 11 waves summing to 50

**Severity: NONE (confirming)**

I verified the proposer's claim that the 11-wave Parallelization Map is unchanged from r3 (`product-strategy/ai-roadmap-phase-2.md:2419` D3 row, `:2497-2498` D3 prose, `:2508` D4 row). The Parallelization Map at `product-strategy/ai-roadmap-phase-2.md:2399-2411` enumerates exactly 11 waves with the following counts:

| Wave | Members | Count |
|---:|---|---:|
| 1 | WU-2-01, 02, 03, 04, 05, 06, 13, 20, 21, 28, 29, 33, 45 | 13 |
| 2 | WU-2-08, 14, 22, 23, 24, 27, 30, 39 | 8 |
| 3 | WU-2-07, 11, 15, 19, 31 | 5 |
| 4 | WU-2-09, 10, 17, 32, 34 | 5 |
| 5 | WU-2-12, 18, 35, 46, 50 | 5 |
| 6 | WU-2-16, 37, 38, 47 | 4 |
| 7 | WU-2-25, 36, 41, 48 | 4 |
| 8 | WU-2-26, 49 | 2 |
| 9 | WU-2-40, 44 | 2 |
| 10 | WU-2-42 | 1 |
| 11 | WU-2-43 | 1 |

Sum = 13+8+5+5+5+4+4+2+2+1+1 = **50** WUs across **11 waves**, byte-identical to r3 (`worktrees/phase-2-ai-roadmap-r3/product-strategy/ai-roadmap-phase-2.md:2369-2383`). The r3 R3-DEP-F01 reconstruction (longest-path-from-leaf+1) closure carries forward intact because round 4 added no Phase 2 internal edges — the 35 explicit internal edge lines at `product-strategy/ai-roadmap-phase-2.md:2339-2375` are unchanged, and only the cascade-driven cross-phase incoming-edge additions (Phase 0C → Phase 2 for WU-2-43/45/46/47) and the WU-2-22/23/24 acceptance-criteria narrowing fall inside the r4 hard scope cap. `parallelization-map-family` does not re-fire (audit history at `plans/audit/ai-roadmap-phase-2.md:118` predicted this: "should remain closed unless a reviewer finds a hidden Phase 2 internal edge").

**Recommendation:** No action required.

---

### R4-DEP-F02. Cross-phase incoming-from-Phase-0C systematic for WU-2-43/45/46/47 with WU-0C-N* IDs; WU-2-48/49/50 remain clean

**Severity: NONE (confirming)**

The audit prescribed adding WU-0C-N1..N5 edges only to WU-2-43, WU-2-45, WU-2-46, WU-2-47 and leaving WU-2-48/49/50 clean (`plans/audit/ai-roadmap-phase-2.md:115-118`). The Phase 2 r4 per-WU Dependencies blocks honor this exactly:

| WU | Phase 0C N* edges declared | Citation |
|---|---|---|
| WU-2-43 SummaryRefreshFixturePack | N1, N2, N3, N4, N5 | `product-strategy/ai-roadmap-phase-2.md:2008` |
| WU-2-45 DetailRecordSchemaDto | N2 only | `product-strategy/ai-roadmap-phase-2.md:2096` |
| WU-2-46 TurnDecompositionService | N1, N2, N3, N4, N5 | `product-strategy/ai-roadmap-phase-2.md:2142` |
| WU-2-47 DetailInjectionRouterService | N1, N2, N3, N4, N5 | `product-strategy/ai-roadmap-phase-2.md:2190` |
| WU-2-48 IncrementalSummaryUpdateService | none | `product-strategy/ai-roadmap-phase-2.md:2231` |
| WU-2-49 FullSummaryRegenerationService | none | `product-strategy/ai-roadmap-phase-2.md:2273` |
| WU-2-50 StaleMarkDetectionService | none | `product-strategy/ai-roadmap-phase-2.md:2314` |

The N2-only narrowing on WU-2-45 is structurally correct: WU-2-45 is a pure DTO whose Contract input is "WU-0C-N2-derived canonical `TranscriptTurn` evidence" (`product-strategy/ai-roadmap-phase-2.md:2066-2069`). It does not perform write-back, so the `WU-0C-N1` trait, `WU-0C-N3` v1 adapter, `WU-0C-N4` schema probe, and `WU-0C-N5` registry edges are not load-bearing. The narrowing matches Phase 0C-r4 line 1680: `WU-0C-N2` "produces canonical session-override DTOs consumed by WU-0C-N1, WU-0C-N3, WU-0C-N4, WU-0C-N5, VS-010, VS-012, VS-018, VS-020, and VS-021."

The Stitch Notes "Incoming From Phase 0C" block also emits the five `(WU-0C-N1..N5, VS-010)` pairs at the slice level (`product-strategy/ai-roadmap-phase-2.md:2763-2767`), and the prose at `:2623` explicitly states "Round 4 adds WU-0C-N1..N5 only where SessionOverrideContract write-back, DTO evidence, fixtures, receipts, or refusals are consumed: WU-2-43, WU-2-45, WU-2-46, and WU-2-47. WU-2-48, WU-2-49, and WU-2-50 remain clean." The narrowing is justified at the per-WU level because WU-2-48/49/50 consume DetailRecord / SummaryContract data after WU-2-46/47 have already mediated the canonical-turn / write-back boundary; they never call SessionOverrideContract directly.

`dependency-encoding-family` does not re-fire — round 4's surgical additions preserve the systematic-from-start encoding pattern from Phase 1 r1 and Phase 2 r1.

**Recommendation:** No action required.

---

### R4-DEP-F03. WU-2-22/23/24 keep WU-0C-11a..18 incoming and add explicit `AgentRunnerClient` scope routing

**Severity: NONE (confirming)**

The audit prescribed narrowing the three per-CLI turn adapters to `AgentRunnerClient`-mediated invocation/resume evidence rather than per-CLI launch/resume ownership (`plans/audit/ai-roadmap-phase-2.md:115`). Verified per WU:

- **Contract change.** All three contracts now read `contract_kind: Provider capability adapter over AgentRunnerClient` and the method signatures take `agent_runner_session_evidence` and return `CliAcceptedSession using Phase 0C AgentRunnerClient invocation/resume evidence` (`product-strategy/ai-roadmap-phase-2.md:1039-1045` for WU-2-22, `:1086-1092` for WU-2-23, `:1133-1139` for WU-2-24). Each contract additionally states "No direct `<cli>` command execution, …storage assumption, provider/account route selection, resume composition, or session-id capture is owned here."
- **Acceptance criterion.** The final acceptance-criterion line on each WU pins the `WU-0C-18 AgentRunnerClient`-only routing: `:1064` (WU-2-22 — Claude), `:1111` (WU-2-23 — Codex), `:1158` (WU-2-24 — Opencode). Each line reads "Launch, resume, provider routing, provider/account selection, [thread/session-row mapping], session-id capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `<cli>` directly or infer `<cli>` storage … is rejected."
- **Phase 0C incoming preserved.** The Cross-phase incoming from Phase 0C list for WU-2-22/23/24 keeps the full WU-0C-04..0C-37 range *including* the entire agent-runner CLI subprocess supervisor cluster `WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18` (`product-strategy/ai-roadmap-phase-2.md:1072` for WU-2-22, `:1119` for WU-2-23, `:1166` for WU-2-24).
- **Bidirectional consistency.** Phase 0C-r4 outgoing-to-Phase-2 line 3668 declares VS-009 consumes "agent-runner WU-0C-11a..WU-0C-18" (`worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md:3668`). The agent-runner cluster lives at `WU-0C-N1..N5` is excluded from VS-009's outgoing set (correct — VS-009 turn capability adapters use only the existing AgentRunnerClient facade, not SessionOverrideContract), and Phase 0C-r4 line 3645 reserves `WU-0C-N1..N5` to "VS-010, VS-012, VS-018, VS-020, VS-021." Phase 2 r4 honors this boundary: WU-2-22/23/24 incoming sets contain no `WU-0C-N*` IDs.
- **Stitch Notes Outgoing-to-Phase-3+.** Round 4 also rewords the Outgoing block from "per-CLI turn adapters" to "AgentRunnerClient-mediated provider capability adapters" (`product-strategy/ai-roadmap-phase-2.md:2896-2897`). VS-015 / VS-016 outgoing edges preserved.

`session-override-boundary-family` (D4 watch entry at `:2509`) does not fire — the rewording explicitly forbids direct CLI execution and JSONL mutation.

**Recommendation:** No action required.

---

### R4-DEP-F04. Bidirectional consistency with Phase 0C-r4 outgoing-to-Phase-2 declarations on the SessionOverrideContract surface

**Severity: NONE (confirming)**

I cross-checked Phase 0C-r4 `Outgoing to Phase 2+` (`worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md:3665-3680`) against Phase 2 r4 `Stitch Notes Incoming From Phase 0C` (`product-strategy/ai-roadmap-phase-2.md:2621-2767`) plus the per-WU Dependencies blocks. The cascade-relevant matches are exact:

| Phase 0C-r4 outgoing edge | Phase 2 r4 incoming evidence | Match |
|---|---|---|
| VS-008 consumes policy WU-0C-04 / budget WU-0C-05..07 / render WU-0C-21..26 / hook WU-0C-32/33 / audit WU-0C-37 (line 3667) | VS-008 incoming pairs at `:2625-2668` enumerate WU-0C-04, 05, 06a, 06, 07a, 07, 21, 22a, 22, 23a, 23b, 23, 24a, 24, 25a, 25, 26a, 26, 32, 33a, 33, 37a, 37 | ✓ |
| VS-009 consumes policy / config / budget / render / agent-runner WU-0C-11a..18 / provider / optimizer WU-0C-27..29 / hook / IPC WU-0C-34/35 / audit (line 3668) | VS-009 incoming pairs at `:2669-2727` enumerate the full set | ✓ |
| VS-010 consumes policy / config / budget / render / optimizer WU-0C-27..29 / **SessionOverrideContract WU-0C-N1..N5** / IPC WU-0C-34/35 / UI WU-0C-36 / audit (line 3669) | VS-010 incoming pairs at `:2728-2767` enumerate the full set, including (WU-0C-N1..N5, VS-010) | ✓ |
| WU-0C-N1..N5 → "Phase 2 turn-decomposition and detail-injection-router WUs … must call `append_turns`, `truncate_after`, or `replace_transcript` through WU-0C-N1" (line 3685) | WU-2-46 contract at `:2114` ("session write-back must call WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript` with WU-0C-N2 preconditions and propagate WU-0C-N5 receipts/refusals"); WU-2-47 contract at `:2160-2161` ("calls WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript`, and returns the WU-0C-N5 receipt/refusal") | ✓ |
| WU-0C-N3 v1 adapter "consumed by VS-010, VS-012, VS-018, VS-020, and VS-021 until the v2 CLI adapter lands" (line 1839) | WU-2-43 fixture coverage criterion at `:2000` ("WU-0C-N1 fake adapter success/refusal, WU-0C-N3 v1 idle-only write-back, preimage mismatch, session busy, unsupported storage, and WU-0C-N5 override receipt propagation") | ✓ |

The OptimizerRequest emission contract from Phase 0C-r4 line 3653 (VS-009 → orchestrator_turn, VS-010 → backend_signal) carries forward unchanged: WU-2-18 AdvisoryOptimizerRequestEmitter (acceptance criterion at `product-strategy/ai-roadmap-phase-2.md:2418` "OptimizerRequest emitted by this WU family has source_type orchestrator_turn") and WU-2-39 BackendStaleSignalEmitter both still depend on WU-0C-27/28/29.

**Recommendation:** No action required.

---

### R4-DEP-F05. Stitch Notes Outgoing-to-Phase-3 (VS-012 / VS-018) precise on the SessionOverrideContract surface

**Severity: NONE (confirming)**

Phase 2 r4 `Stitch Notes Outgoing To Phase 3+` (`product-strategy/ai-roadmap-phase-2.md:2867-2950`) declares one per-WU outgoing line per (Phase 2 WU, Phase 3+ VS) pair. The cascade-relevant outgoing edges are:

| Phase 2 WU | Outgoing VS | Cited mechanism |
|---|---|---|
| WU-2-43 SummaryRefreshFixturePack | VS-012, VS-018 | "fixture flow-through for WU-0C-N1 fake adapter success/refusal and WU-0C-N3 idle-only write-back/refusal cases" (`:2949`) |
| WU-2-45 DetailRecordSchemaDto | VS-011, VS-018 | DTO consumption (`:2924-2925`) |
| WU-2-46 TurnDecompositionService | VS-012, VS-018, VS-019 | "feeds VS-012 repack planning with canonical transcript turns and detail decomposition; any packed transcript replacement remains WU-0C-N1-mediated and carries WU-0C-N5 receipts/refusals" + "feed VS-018 worker-output reintegration through the same canonical turns, detail routing, and WU-0C-N1 write-back boundary" (`:2946`, `:2948`) |
| WU-2-47 DetailInjectionRouterService | VS-012, VS-018 | "feeds VS-012 with SessionOverrideContract receipts/refusals for packed transcript or canonical-turn write-back; graph/topology mutation remains proposal-only" (`:2947`) + same VS-018 boundary (`:2948`) |
| WU-2-48 IncrementalSummaryUpdateService | VS-014, VS-018, VS-019 | unchanged from r3 (`:2933-2935`) |
| WU-2-49 FullSummaryRegenerationService | VS-014, VS-018, VS-019 | unchanged from r3 (`:2936-2938`) |
| WU-2-50 StaleMarkDetectionService | VS-014, VS-018 | unchanged from r3 (`:2939-2940`) |

Round 4 also reworded the Outgoing block for WU-2-22..24 from "per-CLI turn adapters" to "AgentRunnerClient-mediated provider capability adapters" feeding VS-015 / VS-016 (`:2896-2897`), matching the WU-2-22/23/24 contract narrowing in R4-DEP-F03.

The narrative summary block at `:2944-2950` ties each outgoing edge to its specific SessionOverrideContract method (`append_turns`, `replace_transcript`) and explicitly preserves the "topology mutation remains proposal-only" boundary. VS-012's repack-planner expectation from Phase 0C-r4 line 3686 ("Phase 3 repack-planner WUs … must not open CLI JSONL files directly") is honored by WU-2-47's contract at `:2161` ("The router never opens, truncates, rewrites, appends, or locates per-CLI JSONL directly") and by WU-2-46's analogous line at `:2115`.

**Recommendation:** No action required.

---

### R4-DEP-F06. D3 and D4 honest about the cascade-driven minor edit-pass

**Severity: NONE (confirming)**

D3 (`product-strategy/ai-roadmap-phase-2.md:2419` Run Report row, `:2492-2499` prose) explicitly enumerates what the proposer actually read:
- Phase 0A, Phase 0B, Phase 0C, Phase 1, proposal, engineering-roadmap, and audit histories (line counts and inventory).
- Phase 2 r3 roadmap and Phase 2 r4 audit report for the required minor edit-pass scope.
- Proposal r5, engineering-roadmap r4, and Phase 0C r4 WU-0C-N1..WU-0C-N5 for SessionOverrideContract dependency and write-back boundaries.
- Phase 0C Stitch Notes outgoing-to-Phase-2+ for WU-0C-N1..N5 → Phase 2 turn-decomposition / detail-injection expectations.
- The 11-wave Parallelization Map was re-derived as unchanged because no Phase 2 internal edges changed.

D3 also explicitly disclaims the 3-gate risk loop ("Did not dispatch the 3-gate risk loop; that is explicitly separate after proposer lands" at `:2499`). The disclaimer is appropriate — the risk gates are this Dep gate plus parallel Decomposition / Coverage gates, run after the proposer's edit lands.

D4 (`:2420` Run Report row, `:2501-2509` watch-signal table, `:2511-2513` self-classification) classifies r4 as "fix-created-family gen 0, externally driven by proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 SessionOverrideContract" and rates the four pre-existing watch-signal families with explicit residual-risk language:
- `bundling-family`: LOW (no DTO/service pair re-bundled).
- `state-machine-criteria-family`: LOW-MEDIUM (because WU-2-14 remains the deepest state machine and "should receive reviewer attention" — honest acknowledgement).
- `dependency-encoding-family`: LOW after targeted SessionOverrideContract edge pass.
- `parallelization-map-family`: LOW with unchanged 11-wave topology.
- `fix-created-family`: LOW-MEDIUM until risk gates confirm no WU-0C-N* edge omissions — honest acknowledgement that the gate-loop is the verification, not the proposer's own claim.
- `session-override-boundary-family` is listed as a new watch entry in the audit history (`plans/audit/ai-roadmap-phase-2.md:118`).

The self-classification at `:2513` honestly summarizes: "preserved 50 WUs and the 11-wave map; narrowed WU-2-22..24 through AgentRunnerClient; added SessionOverrideContract scope/dependencies to WU-2-43, WU-2-45, WU-2-46, and WU-2-47." That matches what the diff shows.

**Recommendation:** No action required.

---

### R4-DEP-F07. Pre-existing bidirectional over-declarations preserved by hard scope cap

**Severity: INFO (carry-forward, not r4-introduced)**

The Phase 2 r4 hard scope cap preserves WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, and WU-2-48..50 byte-for-byte from r3 (`plans/audit/ai-roadmap-phase-2.md:114`). That cap also preserves a few pre-existing bidirectional over-declarations in the per-WU Dependencies blocks — Phase 2 incoming sets that are slightly broader than Phase 0C-r4's outgoing-to-Phase-2 enumerations. Examples spotted while auditing:

- **VS-008 over-declares agent-runner.** Phase 2 r4 VS-008 incoming-from-Phase-0C pairs at `product-strategy/ai-roadmap-phase-2.md:2631-2651` include `WU-0C-11a..18` (the AgentRunnerClient subprocess supervisor cluster). Phase 0C-r4 line 3667 ("VS-008 consumes policy WU-0C-04; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; hook/plugin WU-0C-32/WU-0C-33; and audit WU-0C-37") and line 3644 ("CLI subprocess supervisor around `agents` WU-0C-11a..WU-0C-18 feed VS-001, VS-003, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021") both omit VS-008 from the agent-runner consumer set. Navigation tools (VS-008) consume `AgentWalkState` and don't structurally need agent-runner trace context.
- **VS-010 over-declares identity/conflict shell.** Phase 2 r4 VS-010 incoming includes `(WU-0C-29d, VS-010)` at `:2755`. Phase 0C-r4 line 3669 only lists "optimizer WU-0C-27..WU-0C-29" for VS-010, and line 3647 reserves identity/conflict shell `WU-0C-29c/WU-0C-29d` to VS-012/013/017/018/020/021 — not VS-010.

These over-declarations were present in r3 (the r3 R3-DEP-F03 finding INFO-classified them as "set-inclusion interpretation" carry-forwards from r1) and are byte-for-byte preserved by the r4 hard scope cap. They are not r4-introduced and they are over-declarations rather than missing edges, so they do not break Phase 2 r4's structural correctness on the SessionOverrideContract surface (the surface where r4 actually moved). Set-inclusion under-conservatism (VS-008 also reading agent-runner; VS-010 also reading identity-conflict shell) is safer than under-inclusion: it cannot cause a silent omitted dependency.

This is informational, not a Dep-gate finding. A future "outgoing/incoming exact-set-equality" reconciliation pass — still out of scope for round 4 per the audit's hard scope cap — would be the right place to either tighten the Phase 2 incoming sets or expand the Phase 0C outgoing declarations, whichever is structurally correct.

**Recommendation:** No action required for the round-4 Dep gate. Flag as a candidate cleanup item for the next non-cascade reconciliation round; do not block r4 LOW.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-DEP-F01 | 11-wave Parallelization Map preserved byte-identical from r3; sum = 50 | NONE (confirming) |
| R4-DEP-F02 | Cross-phase incoming-from-Phase-0C systematic for WU-2-43/45/46/47 with WU-0C-N* IDs; WU-2-48/49/50 remain clean; WU-2-45 narrowed to N2 only is structurally justified | NONE (confirming) |
| R4-DEP-F03 | WU-2-22/23/24 keep WU-0C-11a..18 incoming and add explicit `WU-0C-18 AgentRunnerClient`-only routing acceptance criterion plus contract narrowing | NONE (confirming) |
| R4-DEP-F04 | Bidirectional consistency with Phase 0C-r4 outgoing-to-Phase-2 declarations holds on the SessionOverrideContract surface; WU-0C-N1..N5 → VS-010 matches; WU-0C-11a..18 → VS-009 matches; OptimizerRequest contract preserved | NONE (confirming) |
| R4-DEP-F05 | Stitch Notes outgoing-to-Phase-3 precise: WU-2-43/46/47 → VS-012/VS-018 with explicit `WU-0C-N1 append_turns/replace_transcript` and `WU-0C-N5 receipts/refusals` language; topology mutation remains proposal-only | NONE (confirming) |
| R4-DEP-F06 | D3 enumerates the artifacts actually read and explicitly disclaims the 3-gate risk loop; D4 honestly classifies r4 as `fix-created-family` gen 0 externally-driven and rates pre-existing families LOW with one LOW-MEDIUM watch on WU-2-14 state-machine depth | NONE (confirming) |
| R4-DEP-F07 | Pre-existing bidirectional over-declarations (VS-008 includes agent-runner; VS-010 includes identity/conflict shell) preserved by r4 hard scope cap; carried forward from r1/r2/r3 | INFO |

---

## What LOW requires

The following conditions must all hold for the round-4 LOW rating to remain valid:

1. **Parallelization Map unchanged from r3.** 11 waves with member counts 13/8/5/5/5/4/4/2/2/1/1 summing to 50 (`product-strategy/ai-roadmap-phase-2.md:2399-2411`). If a future r5 reviewer adds a Phase 2 internal edge, the Parallelization Map must be re-derived from longest-path-from-leaf+1 at that point.
2. **No new Phase 2 internal edges in the Dependency Graph.** The 35 explicit edge lines at `product-strategy/ai-roadmap-phase-2.md:2339-2375` remain byte-identical to r3. Round 4 explicitly limits its scope to cross-phase incoming additions and acceptance-criteria narrowing.
3. **WU-2-43 declares WU-0C-N1..N5 in its Phase 0C incoming list.** All five N* IDs at `:2008`. The fixture pack must cite `WU-0C-N1 fake adapter success/refusal`, `WU-0C-N3 v1 idle-only write-back`, `preimage mismatch`, `session busy`, `unsupported storage`, and `WU-0C-N5 override receipt propagation` (acceptance criterion at `:2000`).
4. **WU-2-45 declares WU-0C-N2 only.** Sole N* ID at `:2096`. The DTO must reject `provider-native JSONL bodies`, `mutable transcript handles`, and `non-canonical provider transcript pointers` (acceptance criteria at `:2085-2086`).
5. **WU-2-46 and WU-2-47 declare WU-0C-N1..N5.** All five N* IDs at `:2142` and `:2190`. Each contract must call session write-back exclusively through WU-0C-N1 `append_turns`/`truncate_after`/`replace_transcript` and propagate WU-0C-N5 receipts/refusals (`:2114` and `:2160-2161`); each acceptance set must explicitly forbid direct provider-native JSONL operations (`:2134`, `:2161`, `:2182`).
6. **WU-2-48/49/50 declare no WU-0C-N* edges.** Phase 0C incoming lists at `:2231`, `:2273`, `:2314` end at `WU-0C-37`. These services consume DetailRecord/SummaryContract data after WU-2-46/47 mediate the canonical-turn / write-back boundary.
7. **WU-2-22/23/24 keep WU-0C-11a..18 incoming and route launch/resume only through WU-0C-18 AgentRunnerClient.** Full 11a..18 cluster at `:1072`, `:1119`, `:1166`. Last acceptance criterion at `:1064`, `:1111`, `:1158` pins routing to WU-0C-18 outputs and rejects direct CLI execution.
8. **Bidirectional consistency on the SessionOverrideContract surface.** Phase 0C-r4 line 3669 declares VS-010 consumes WU-0C-N1..N5 for turn-decomposition / detail-injection write-back; Phase 2 r4 Stitch Notes "Incoming From Phase 0C" emits the five `(WU-0C-N1..N5, VS-010)` pairs at `:2763-2767`. Phase 0C-r4 line 3686 declares the WU-0C-N1..N5 → Phase 3 repack-planner edge; Phase 2 r4 Stitch Notes Outgoing-to-Phase-3+ for WU-2-46/47 cite `WU-0C-N1`-mediated write-back and `WU-0C-N5` receipts at `:2946-2949`.
9. **OptimizerRequest emission contract preserved.** WU-2-18 (`source_type=orchestrator_turn`) and WU-2-39 (`source_type=backend_signal`) both depend on WU-0C-27 OptimizerQueueService; per-task model dispatch routes through WU-2-34 OptimizerModelInvocationAdapter for VS-010 pipeline cycles.
10. **D3 and D4 honest.** D3 enumerates the actual artifacts read and disclaims the 3-gate risk loop; D4 classifies r4 as `fix-created-family` gen 0 externally-driven by the SessionOverrideContract cascade and adds a `session-override-boundary-family` watch.

If any of these conditions break in a future round (e.g., a per-CLI turn adapter quietly re-acquires direct CLI launch language, or a future r5 introduces a Phase 2 internal edge that pushes a WU into a different wave, or a new VS-010 service is added without WU-0C-N1..N5 in its Phase 0C incoming block, or a Phase 0C-r5 narrows the WU-0C-N5 receipt-propagation contract that WU-2-46/47 currently rely on), the Dep gate should re-fire MEDIUM under the appropriate watch-signal family (`session-override-boundary-family`, `parallelization-map-family`, or `dependency-encoding-family`).
