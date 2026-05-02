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

---

# AI Roadmap (Phase 2) — Dependency Risk Assessment (Round 5, Option A brownfield edit-pass)

**Rating: LOW**

Round 5 is an externally-driven Option A edit-pass triggered by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 after the agent-runner team shipped the SessionOverrideContract feature requests. The audit at `plans/audit/ai-roadmap-phase-2.md` round-5 entry (lines 120–126) prescribes targeted annotation and dependency cleanup only: remove `Blocked-on` annotations from the eight r4-affected WUs (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47), retire the dropped Phase 0C-r4 schema-probe split (WU-0C-N4) so VS-010 schema-probe coverage now flows through WU-0C-N3, and stamp each touched WU with an r5 cascade note that confirms the r4 SessionOverrideContract criteria are retained. The proposer landed commit `ee68f91` with a 32-insertion / 31-deletion diff (63 lines changed) against `product-strategy/ai-roadmap-phase-2.md`. WU count remains 50 (`grep -c '^### WU-2-' product-strategy/ai-roadmap-phase-2.md` = 50). The 11-wave Parallelization Map at `product-strategy/ai-roadmap-phase-2.md:2399-2411` is byte-identical to r4 with member counts 13/8/5/5/5/4/4/2/2/1/1 summing to 50; the 35 explicit Phase 2 internal edge lines at `:2339-2375` are unchanged because round 5 added zero internal edges. Cross-phase incoming-from-Phase-0C is systematic across WU-2-43/45/46/47 with WU-0C-N4 cleanly retired (zero file occurrences via `grep "WU-0C-N4"`); WU-2-48/49/50 remain clean. Bidirectional consistency with Phase 0C-r5 outgoing-to-Phase-2 holds: Phase 0C-r5 line 3610 declares VS-010 consumes "SessionOverrideContract WU-0C-N1/N2/N3/N5 for turn-decomposition/detail-injection write-back" and Phase 2 r5 Stitch Notes "Incoming From Phase 0C" emits exactly the four pairs `(WU-0C-N1, VS-010)`, `(WU-0C-N2, VS-010)`, `(WU-0C-N3, VS-010)`, `(WU-0C-N5, VS-010)` at `:2763-2766` (the prior r4 `(WU-0C-N4, VS-010)` row is removed). Phase 0C-r5 foundation row 3586 (`SessionOverrideContract WU-0C-N1/N2/N3/N5 feed VS-010, VS-012, VS-018, VS-020, VS-021`) matches. `Blocked-on` and `Blocked on` annotations are fully absent across the whole file (`grep -E "Blocked-on|Blocked on"` returns zero hits), replaced by a single `**Revision rationale:**` cascade-note line on each of the eight touched WUs. D3 honestly enumerates the artifacts the proposer actually read (Phase 2 r4, Phase 2 r5 audit instructions, proposal-r6, engineering-roadmap-r5, Phase 0C-r5 WU-0C-N1..WU-0C-N3 plus WU-0C-N5) and explicitly disclaims dispatching the 3-gate risk loop. D4 classifies r5 as externally-driven `fix-created-family` generation 0 in the Phase 2-local loop and rates `dependency-encoding-family` LOW after the targeted WU-0C-N4 retirement, `parallelization-map-family` LOW with unchanged 11-wave topology, `bundling-family` LOW, `state-machine-criteria-family` LOW-MEDIUM (carry-forward on WU-2-14 depth), and `session-override-boundary-family` LOW. No cycle, no intra-wave edges, no missing cascade-relevant cross-phase edges, no Phase 0C-r5 outgoing declaration unmatched by a Phase 2 r5 incoming pair on the SessionOverrideContract surface. Dep gate is LOW; one INFO finding records pre-existing bidirectional over-declarations preserved by the r4 hard scope cap and re-preserved by the r5 hard scope cap, plus one minor stale-phrasing observation in the Outgoing-to-Phase-3+ narrative prose at `:2948`.

---

## Findings

### R5-DEP-F01. Parallelization Map preserved byte-identical from r4 — 11 waves summing to 50

**Severity: NONE (confirming)**

I verified the proposer's claim that the 11-wave Parallelization Map is unchanged from r4 (`product-strategy/ai-roadmap-phase-2.md:2417` D1 row, `:2419` D3 row, `:2497` D3 prose, `:2508` D4 row). The Parallelization Map at `:2399-2411` enumerates exactly 11 waves with the following counts:

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

Sum = 13+8+5+5+5+4+4+2+2+1+1 = **50** WUs across **11 waves**, byte-identical to r4 (`worktrees/phase-2-ai-roadmap-r4/product-strategy/ai-roadmap-phase-2.md:2399-2411` per the r4 R4-DEP-F01 reconstruction). The diff `git diff 335a161..ee68f91 -- product-strategy/ai-roadmap-phase-2.md` shows zero hunks against the wave block at lines 2399–2411 and zero hunks against the 35 explicit internal-edge lines at `:2339-2375`. The r4 longest-path-from-leaf+1 closure carries forward intact because round 5 added no Phase 2 internal edges — only the targeted WU-0C-N4 retirement on three cross-phase incoming lists (WU-2-43 at `:2008`, WU-2-46 at `:2142`, WU-2-47 at `:2190`) and the matching `(WU-0C-N4, VS-010)` Stitch-Notes row removal fall inside the r5 hard scope cap. `parallelization-map-family` does not re-fire (audit-history at `plans/audit/ai-roadmap-phase-2.md:126` predicted this: "`parallelization-map-family` remains closed because no Phase 2 internal edge changed.").

**Recommendation:** No action required.

---

### R5-DEP-F02. Cross-phase incoming-from-Phase-0C systematic; WU-0C-N4 retired cleanly; WU-2-48/49/50 remain clean

**Severity: NONE (confirming)**

The audit r5 entry prescribes retiring `WU-0C-N4` from the four r4-affected VS-010 cross-phase incoming lists and the corresponding Stitch-Notes pair (`plans/audit/ai-roadmap-phase-2.md:120-126`). `grep "WU-0C-N4" product-strategy/ai-roadmap-phase-2.md` returns zero hits across the entire 2956-line file (output_mode=count). Per-WU verification:

| WU | Phase 0C N* edges declared | Citation |
|---|---|---|
| WU-2-43 SummaryRefreshFixturePack | N1, N2, N3, N5 (N4 dropped) | `product-strategy/ai-roadmap-phase-2.md:2008` |
| WU-2-45 DetailRecordSchemaDto | N2 only | `product-strategy/ai-roadmap-phase-2.md:2096` |
| WU-2-46 TurnDecompositionService | N1, N2, N3, N5 (N4 dropped) | `product-strategy/ai-roadmap-phase-2.md:2142` |
| WU-2-47 DetailInjectionRouterService | N1, N2, N3, N5 (N4 dropped) | `product-strategy/ai-roadmap-phase-2.md:2190` |
| WU-2-48 IncrementalSummaryUpdateService | none | byte-identical to r4 |
| WU-2-49 FullSummaryRegenerationService | none | byte-identical to r4 |
| WU-2-50 StaleMarkDetectionService | none | byte-identical to r4 |

The N2-only narrowing on WU-2-45 is preserved from r4 and is still structurally correct: WU-2-45 is a pure DTO whose Source basis (`:2073`) is now `Phase 0C r5 WU-0C-N2`, and Phase 0C-r5 line 1679 declares "Canonical session-override DTOs consumed by WU-0C-N1, WU-0C-N3, WU-0C-N5, VS-010, VS-012, VS-018, VS-020, and VS-021" — N4 is correctly absent because it was deleted upstream.

The Stitch Notes "Incoming From Phase 0C" block emits exactly four `(WU-0C-N*, VS-010)` rows at `:2763-2766` for `N1, N2, N3, N5`; the previous `(WU-0C-N4, VS-010)` row at the r4 line 2766 is deleted. The narrative prose at `:2623` reads "Round 5 keeps WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5 only where SessionOverrideContract write-back, DTO evidence, schema-probe gating, fixtures, receipts, or refusals are consumed: WU-2-43, WU-2-45, WU-2-46, and WU-2-47. WU-2-48, WU-2-49, and WU-2-50 remain clean." The narrowing is still justified at the per-WU level because WU-2-48/49/50 consume DetailRecord / SummaryContract data after WU-2-46/47 have already mediated the canonical-turn / write-back boundary.

WU-2-43 also narrows its WU-0C-N3 reference from "v1 idle-only write-back" to "schema-probe/safe-import gating" in both the contract block (`:1980`) and the matching acceptance criterion (`:2000`), tracking the Phase 0C-r5 redefinition of WU-0C-N3 from "AgentRunnerDbAdapter v1" to "AgentRunnerCliAdapter v2 implementation" at `worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md:1739`. WU-2-46 and WU-2-47 widen their Source basis lines (`:2118` and `:2166`) to include WU-0C-N3 alongside N1/N2/N5, making N3 explicit (in r4 N3 was implicit through WU-2-43 fixture pass-through).

`dependency-encoding-family` does not re-fire — round 5's surgical retirement preserves the systematic-from-start encoding pattern from Phase 1 r1 and Phase 2 r1.

**Recommendation:** No action required.

---

### R5-DEP-F03. WU-2-22/23/24 contracts byte-identical except for one cascade-note line; AgentRunnerClient routing preserved

**Severity: NONE (confirming)**

The audit r5 entry restricts r5 scope on WU-2-22/23/24 to a `Blocked-on` annotation removal plus a single `**Revision rationale:**` line. Verified per WU:

- **Contract preserved.** All three contracts continue to read `contract_kind: Provider capability adapter over AgentRunnerClient`; method signatures still take `agent_runner_session_evidence` and return `CliAcceptedSession using Phase 0C AgentRunnerClient invocation/resume evidence`; the contract clauses still state "No direct `<cli>` command execution, …storage assumption, provider/account route selection, resume composition, or session-id capture is owned here" (`product-strategy/ai-roadmap-phase-2.md:1039-1045` for WU-2-22, `:1086-1092` for WU-2-23, `:1133-1139` for WU-2-24).
- **Acceptance criterion preserved.** The final `WU-0C-18 AgentRunnerClient`-only routing acceptance criterion is byte-identical at `:1064` (WU-2-22 — Claude), `:1111` (WU-2-23 — Codex), `:1158` (WU-2-24 — Opencode). Each line still reads "Launch, resume, provider routing, provider/account selection, [thread/session-row mapping], session-id capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `<cli>` directly or infer `<cli>` storage … is rejected."
- **Phase 0C incoming preserved byte-identical.** The Cross-phase incoming from Phase 0C list for WU-2-22/23/24 keeps the full `WU-0C-04..0C-37` range *including* the entire agent-runner CLI subprocess supervisor cluster `WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0C-12, WU-0C-13a, WU-0C-13b, WU-0C-13, WU-0C-14a, WU-0C-14b, WU-0C-14, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-16a, WU-0C-16b, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0C-17, WU-0C-18` (`:1072` for WU-2-22, `:1119` for WU-2-23, `:1166` for WU-2-24). No `WU-0C-N*` IDs are added, matching the audit r5 entry's hard cap.
- **r5 cascade note inserted.** A single `**Revision rationale:**` line now sits between `Produces:` and `Parallelizable with:` on each WU at `:1076` (WU-2-22), `:1123` (WU-2-23), `:1170` (WU-2-24). The line text — "r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed." — is purely annotational and changes no dependency edge.
- **Bidirectional consistency.** Phase 0C-r5 outgoing-to-Phase-2 line 3609 declares VS-009 consumes "agent-runner WU-0C-11a..WU-0C-18", matching Phase 2 r5 incoming. Phase 0C-r5 line 3585 reserves the agent-runner cluster to "VS-001, VS-003, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021" (VS-009 in scope for VS-009-feeding adapters). Phase 0C-r5 line 3586 reserves WU-0C-N1/N2/N3/N5 to "VS-010, VS-012, VS-018, VS-020, VS-021" — VS-009 correctly omitted, matching WU-2-22/23/24 having no `WU-0C-N*` incoming. Phase 2 r5 honors this boundary: WU-2-22/23/24 cross-phase incoming sets contain no `WU-0C-N*` IDs.
- **Stitch Notes Outgoing-to-Phase-3+ preserved.** The relabeled rows from r4 — `(WU-2-22..WU-2-24 AgentRunnerClient-mediated provider capability adapters, VS-015 / VS-016)` — survive byte-identical at `:2895-2896`.

`session-override-boundary-family` (carry-forward watch from r4) does not fire — the AgentRunnerClient-only routing acceptance criterion still forbids direct CLI execution and JSONL mutation, and the cascade note only confirms that v2-only is in force.

**Recommendation:** No action required.

---

### R5-DEP-F04. Bidirectional consistency with Phase 0C-r5 outgoing-to-Phase-2 declarations on the SessionOverrideContract surface

**Severity: NONE (confirming)**

I cross-checked Phase 0C-r5 `Outgoing to Phase 2+` (`worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md:3606-3621`) against Phase 2 r5 `Stitch Notes Incoming From Phase 0C` (`product-strategy/ai-roadmap-phase-2.md:2621-2767`) plus the per-WU Dependencies blocks. The cascade-relevant matches are exact:

| Phase 0C-r5 outgoing edge | Phase 2 r5 incoming evidence | Match |
|---|---|---|
| VS-008 consumes policy WU-0C-04 / budget WU-0C-05..07 / render WU-0C-21..26 / hook WU-0C-32/33 / audit WU-0C-37 (line 3608) | VS-008 incoming pairs at `:2625-2668` enumerate WU-0C-04, 05, 06a, 06, 07a, 07, 21, 22a, 22, 23a, 23b, 23, 24a, 24, 25a, 25, 26a, 26, 32, 33a, 33, 37a, 37 | ✓ |
| VS-009 consumes policy / config / budget / render / agent-runner WU-0C-11a..18 / provider / optimizer WU-0C-27..29 / hook / IPC WU-0C-34/35 / audit (line 3609) | VS-009 incoming pairs at `:2669-2727` enumerate the full set | ✓ |
| VS-010 consumes policy / config / budget / render / optimizer WU-0C-27..29 / **SessionOverrideContract WU-0C-N1/N2/N3/N5** / IPC WU-0C-34/35 / UI WU-0C-36 / audit (line 3610) | VS-010 incoming pairs at `:2728-2766` enumerate the full set, including (WU-0C-N1, VS-010), (WU-0C-N2, VS-010), (WU-0C-N3, VS-010), (WU-0C-N5, VS-010) — **N4 correctly absent on both sides** | ✓ |
| `SessionOverrideContract WU-0C-N1/N2/N3/N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata` (line 3586) | WU-2-46 contract at `:2114` ("session write-back must call WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript` with WU-0C-N2 preconditions and propagate WU-0C-N5 receipts/refusals"); WU-2-47 contract at `:2160-2161` ("calls WU-0C-N1 `append_turns`, `truncate_after`, or `replace_transcript`, and returns the WU-0C-N5 receipt/refusal") | ✓ |
| WU-0C-N3 in r5 = AgentRunnerCliAdapter v2 implementation (line 1739: "AgentRunnerCliAdapter (SessionOverrideContract v2 implementation)"); construction-time `safe_for_import_replace` schema-probe is owned here (lines 1771–1774) | WU-2-43 acceptance criterion at `:2000` enumerates "WU-0C-N1 fake adapter success/refusal, WU-0C-N3 schema-probe/safe-import gating, preimage mismatch, session busy, unsupported storage, and WU-0C-N5 override receipt propagation" — phrasing tracks the Phase 0C-r5 N3 redefinition | ✓ |
| WU-0C-N4 removed in r5 (Phase 0C-r5 line 1735: "WU-0C-N4: Removed in r5"; line 1737: "Removed in round 5. The r4 `AgentRunnerSchemaProbe` existed only to protect the v1 direct `state.db` / JSONL adapter. With the v1 adapter dropped, schema compatibility is a single `agents session schema-probe` CLI call owned by WU-0C-N3") | Phase 2 r5 `grep "WU-0C-N4"` returns zero hits across the entire file; four locations cleanly cleaned (WU-2-43 incoming, WU-2-46 incoming, WU-2-47 incoming, Stitch-Notes pair) | ✓ |

The OptimizerRequest emission contract from Phase 0C-r5 line 3594 (VS-009 → orchestrator_turn, VS-010 → backend_signal) carries forward unchanged: WU-2-18 AdvisoryOptimizerRequestEmitter (acceptance criterion preserved) and WU-2-39 BackendStaleSignalEmitter both still depend on WU-0C-27/28/29 (Stitch-Notes pairs at `:2715-2719` for VS-009 and `:2750-2754` for VS-010).

**Recommendation:** No action required.

---

### R5-DEP-F05. Stitch Notes Outgoing-to-Phase-3+ preserved on the SessionOverrideContract surface

**Severity: NONE (confirming)**

Phase 2 r5 `Stitch Notes Outgoing To Phase 3+` (`product-strategy/ai-roadmap-phase-2.md:2866-2949`) is byte-identical to r4 in its edge-row enumeration — `git diff 335a161..ee68f91` shows no hunks in the lines `2866-2940` range. The cascade-relevant outgoing edges all survive:

| Phase 2 WU | Outgoing VS | Citation |
|---|---|---|
| WU-2-22..24 | VS-015, VS-016 | `:2895-2896` ("AgentRunnerClient-mediated provider capability adapters") |
| WU-2-43 SummaryRefreshFixturePack | VS-012, VS-018 | `:2925-2926` |
| WU-2-45 DetailRecordSchemaDto | VS-011, VS-018 | `:2923-2924` |
| WU-2-46 TurnDecompositionService | VS-012, VS-018, VS-019 | `:2927-2929` |
| WU-2-47 DetailInjectionRouterService | VS-012, VS-018 | `:2930-2931` |
| WU-2-48 IncrementalSummaryUpdateService | VS-014, VS-018, VS-019 | `:2932-2934` |
| WU-2-49 FullSummaryRegenerationService | VS-014, VS-018, VS-019 | `:2935-2937` |
| WU-2-50 StaleMarkDetectionService | VS-014, VS-018 | `:2938-2939` |

The narrative summary block at `:2941-2949` ties the WU-2-46/47/43 outgoing edges to specific SessionOverrideContract methods (`WU-0C-N1 append_turns`/`truncate_after`/`replace_transcript`, `WU-0C-N5 receipts/refusals`) and explicitly preserves the "graph/topology mutation remains proposal-only" boundary at `:2946`. Phase 0C-r5's Phase 3 expectation at line 3586 (SessionOverrideContract feeding VS-012, VS-018, VS-020, VS-021 transcript write-back receipts) is honored by WU-2-47's contract at `:2161` ("The router never opens, truncates, rewrites, appends, or locates per-CLI JSONL directly") and WU-2-46's analogous line at `:2115`.

One minor INFO observation: the Outgoing-to-Phase-3+ narrative bullet at `:2948` reads "WU-2-43 SummaryRefreshFixturePack feeds VS-012 and VS-018 fixture flow-through for WU-0C-N1 fake adapter success/refusal and WU-0C-N3 idle-only write-back/refusal cases." The phrase "WU-0C-N3 idle-only write-back/refusal" preserves r4 v1-adapter terminology even though Phase 0C-r5 has redefined WU-0C-N3 as the v2 CLI adapter and the WU-2-43 contract block at `:1980` and acceptance criterion at `:2000` correctly use "schema-probe/safe-import gating". This is a stale-phrasing carry-forward in narrative prose only; the per-pair Stitch-Notes edge `(WU-2-43 SummaryRefreshFixturePack, VS-018)` at `:2926` is untouched and the contract / acceptance-criteria phrasing is correct. Folded into R5-DEP-F08 INFO below; no Dep-gate action.

**Recommendation:** No action required for the round-5 Dep gate. See R5-DEP-F08 for the INFO observation.

---

### R5-DEP-F06. `Blocked-on` annotations fully absent across the file; per-WU cascade notes are pure annotation

**Severity: NONE (confirming)**

`grep -E "Blocked-on|Blocked on" product-strategy/ai-roadmap-phase-2.md` (output_mode=count) returns zero hits across the entire 2956-line file. Each of the eight r4-affected WUs (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47) carries one cascade note in place of the prior r4 block-on syntax:

> **Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.

Cascade-note locations: `:1076` (WU-2-22), `:1123` (WU-2-23), `:1170` (WU-2-24), `:1260` (WU-2-26), `:2011` (WU-2-43), `:2101` (WU-2-45), `:2147` (WU-2-46), `:2195` (WU-2-47). The 42 r5-unaffected WUs receive zero cascade notes and zero diff hits in their Dependencies / Produces / Parallelizable-with blocks; the audit r5 entry's hard scope cap is honored exactly.

The annotation removal is purely textual — no dependency edge is added, removed, or relabeled. The Stitch Notes Outgoing-to-Phase-3+ block at `:2866-2940` carries no block-on syntax on any of its (WU-2-43 → VS-012), (WU-2-43 → VS-018), (WU-2-46 → VS-012), (WU-2-46 → VS-018), (WU-2-46 → VS-019), (WU-2-47 → VS-012), (WU-2-47 → VS-018) edges. The Parallelization-Map preface at `:2395` (Optimizer branch critical path) carries no block-on annotation.

**Recommendation:** No action required.

---

### R5-DEP-F07. D3 and D4 honest about the r5 cascade-driven edit-pass

**Severity: NONE (confirming)**

D3 (`product-strategy/ai-roadmap-phase-2.md:2419` Run Report row, `:2492-2499` prose) explicitly enumerates what the proposer actually read:

- Phase 0A, Phase 0B, Phase 0C, Phase 1, proposal, engineering-roadmap, and audit histories (line counts and inventory).
- Phase 2 r4 roadmap and Phase 2 r5 audit instructions for the required minor edit-pass scope.
- Proposal r6, engineering-roadmap r5, and Phase 0C r5 WU-0C-N1..WU-0C-N3 plus WU-0C-N5 for SessionOverrideContract dependency and write-back boundaries.
- Phase 0C r5 Stitch Notes outgoing-to-Phase-2+ for WU-0C-N1..WU-0C-N3 plus WU-0C-N5 → Phase 2 turn-decomposition / detail-injection expectations.
- The 11-wave Parallelization Map was re-derived as unchanged because no Phase 2 internal edges changed.

D3 also explicitly disclaims the 3-gate risk loop ("Did not dispatch the 3-gate risk loop; that is explicitly separate after proposer lands" at `:2499`). The disclaimer is appropriate — the risk gates are this Dep gate plus parallel Decomposition / Coverage gates, run after the proposer's edit lands.

D4 (`:2420` Run Report row, `:2503-2509` watch-signal table, `:2511-2513` self-classification) classifies r5 as "fix-created-family gen 0, externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 SessionOverrideContract" and rates the watch-signal families with explicit residual-risk language:
- `bundling-family`: LOW (no DTO/service pair re-bundled; provider capability adapters remain three WUs; fixture packs remain WU-2-12, WU-2-26, WU-2-43).
- `state-machine-criteria-family`: LOW-MEDIUM (carry-forward — WU-2-14 remains the deepest state machine and "should receive reviewer attention"; honest carry-forward).
- `dependency-encoding-family`: LOW after the targeted WU-0C-N4 retirement; WU-0C-N1/N2/N3/N5 only on WU-2-43/45/46/47; WU-2-48/49/50 remain clean.
- `parallelization-map-family`: LOW with unchanged 11-wave topology.
- `fix-created-family`: LOW (round 5 is gen 0, externally driven; r4's LOW-MEDIUM has dropped to LOW because the agent-runner cascade has shipped and the dropped schema-probe split is now confirmed retired upstream).
- `session-override-boundary-family` (the carry-forward watch added in r4): LOW — every touched WU's r4 boundary acceptance criterion is preserved verbatim in r5, and no new direct CLI launch/resume or JSONL mutation language is introduced.

The self-classification at `:2513` honestly summarizes: "Round 5 externally-driven minor edit-pass: preserved 50 WUs and the 11-wave map; retained the r4 SessionOverrideContract criteria for WU-2-22/23/24/26/43/45/46/47; removed stale block-on annotations and the dropped schema-probe split edge after agent-runner feature requests landed." That matches what the diff shows.

**Recommendation:** No action required.

---

### R5-DEP-F08. Pre-existing bidirectional over-declarations and one stale-phrasing carry-forward preserved by hard scope cap

**Severity: INFO (carry-forward, not r5-introduced)**

The Phase 2 r5 hard scope cap preserves WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, and WU-2-48..50 byte-for-byte from r4 (`plans/audit/ai-roadmap-phase-2.md:120-126`). That cap also preserves the same pre-existing bidirectional over-declarations flagged INFO in r4 R4-DEP-F07 (and originally r3 R3-DEP-F03):

- **VS-008 over-declares agent-runner.** Phase 2 r5 VS-008 incoming-from-Phase-0C pairs at `product-strategy/ai-roadmap-phase-2.md:2631-2651` still include `WU-0C-11a..18` (the AgentRunnerClient subprocess supervisor cluster). Phase 0C-r5 line 3608 ("VS-008 consumes policy WU-0C-04; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; hook/plugin WU-0C-32/WU-0C-33; and audit WU-0C-37") and line 3585 ("CLI subprocess supervisor around `agents` WU-0C-11a..WU-0C-18 feed VS-001, VS-003, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021") both omit VS-008 from the agent-runner consumer set. Navigation tools (VS-008) consume `AgentWalkState` and don't structurally need agent-runner trace context.
- **VS-010 over-declares identity/conflict shell.** Phase 2 r5 VS-010 incoming still includes `(WU-0C-29d, VS-010)` at `:2755`. Phase 0C-r5 line 3610 only lists "optimizer WU-0C-27..WU-0C-29" for VS-010, and line 3588 reserves identity/conflict shell `WU-0C-29c/WU-0C-29d` to "VS-012, VS-013, VS-017, VS-018, VS-020, and VS-021" — not VS-010.
- **Stale-phrasing carry-forward at `:2948`.** The Outgoing-to-Phase-3+ narrative bullet still reads "WU-2-43 SummaryRefreshFixturePack feeds VS-012 and VS-018 fixture flow-through for WU-0C-N1 fake adapter success/refusal and WU-0C-N3 idle-only write-back/refusal cases." The phrase "WU-0C-N3 idle-only write-back/refusal" preserves r4 v1-adapter terminology even though Phase 0C-r5 redefines WU-0C-N3 as the v2 CLI adapter (Phase 0C-r5 line 1739) and the WU-2-43 contract block / acceptance criterion correctly use "schema-probe/safe-import gating" (`:1980`, `:2000`). The Stitch-Notes per-pair edges `(WU-2-43, VS-012)` and `(WU-2-43, VS-018)` at `:2925-2926` are untouched and structurally correct.

The first two over-declarations were INFO-classified in r3 (R3-DEP-F03) and r4 (R4-DEP-F07) as "set-inclusion interpretation" carry-forwards from r1 and are byte-for-byte preserved by the r5 hard scope cap. The third is r5-introduced only in the sense that round 5 redefined N3 upstream and the proposer correctly updated the contract / acceptance criterion but missed the narrative-prose bullet. None of the three are r5-introduced dependency-encoding regressions; over-declarations and stale narrative phrasing rather than missing edges, so they do not break Phase 2 r5's structural correctness on the SessionOverrideContract surface (the surface where r5 actually moved). Set-inclusion under-conservatism is safer than under-inclusion: it cannot cause a silent omitted dependency.

This is informational, not a Dep-gate finding. A future "outgoing/incoming exact-set-equality" reconciliation pass — still out of scope for round 5 per the audit's hard scope cap — would be the right place to either tighten the Phase 2 incoming sets, expand the Phase 0C-r5 outgoing declarations, or refresh the narrative prose bullet to match the contract phrasing. The narrative-prose item is a one-token edit ("idle-only write-back" → "schema-probe/safe-import gating") whenever the next non-cascade reconciliation round opens.

**Recommendation:** No action required for the round-5 Dep gate. Flag all three as candidate cleanup items for the next non-cascade reconciliation round; do not block r5 LOW.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R5-DEP-F01 | 11-wave Parallelization Map preserved byte-identical from r4; sum = 50; no Phase 2 internal edges added or removed | NONE (confirming) |
| R5-DEP-F02 | Cross-phase incoming-from-Phase-0C systematic; `WU-0C-N4` retired cleanly (zero file occurrences); WU-2-43/46/47 keep N1/N2/N3/N5; WU-2-45 keeps N2 only; WU-2-48/49/50 remain clean; WU-2-43 N3 phrase narrowed to "schema-probe/safe-import gating" tracking Phase 0C-r5 | NONE (confirming) |
| R5-DEP-F03 | WU-2-22/23/24 contracts byte-identical except for one cascade-note line; WU-0C-11a..18 incoming preserved; AgentRunnerClient-only routing acceptance criterion preserved | NONE (confirming) |
| R5-DEP-F04 | Bidirectional consistency with Phase 0C-r5 outgoing-to-Phase-2 declarations holds on the SessionOverrideContract surface; WU-0C-N1/N2/N3/N5 → VS-010 matches; WU-0C-11a..18 → VS-009 matches; OptimizerRequest contract preserved | NONE (confirming) |
| R5-DEP-F05 | Stitch Notes outgoing-to-Phase-3 preserved byte-identical from r4 with explicit `WU-0C-N1 append_turns/replace_transcript` and `WU-0C-N5 receipts/refusals` language; topology mutation remains proposal-only | NONE (confirming) |
| R5-DEP-F06 | `Blocked-on` and `Blocked on` annotations fully absent across the file (zero grep hits); each of the eight touched WUs carries one cascade-note line | NONE (confirming) |
| R5-DEP-F07 | D3 enumerates the artifacts actually read and explicitly disclaims the 3-gate risk loop; D4 honestly classifies r5 as `fix-created-family` gen 0 externally-driven and rates `dependency-encoding-family`, `parallelization-map-family`, `bundling-family`, `session-override-boundary-family` LOW with `state-machine-criteria-family` LOW-MEDIUM (carry-forward on WU-2-14 depth) | NONE (confirming) |
| R5-DEP-F08 | Pre-existing bidirectional over-declarations (VS-008 includes agent-runner; VS-010 includes identity/conflict shell) preserved by r5 hard scope cap; one stale-phrasing carry-forward at `:2948` ("WU-0C-N3 idle-only write-back" should now read "schema-probe/safe-import gating") in narrative prose only | INFO |

---

## What LOW requires

The following conditions must all hold for the round-5 LOW rating to remain valid:

1. **Parallelization Map unchanged from r4.** 11 waves with member counts 13/8/5/5/5/4/4/2/2/1/1 summing to 50 (`product-strategy/ai-roadmap-phase-2.md:2399-2411`). If a future reviewer adds a Phase 2 internal edge, the Parallelization Map must be re-derived from longest-path-from-leaf+1 at that point.
2. **No new Phase 2 internal edges in the Dependency Graph.** The 35 explicit edge lines at `product-strategy/ai-roadmap-phase-2.md:2339-2375` remain byte-identical to r4. Round 5 explicitly limits its scope to cross-phase incoming subtractions (WU-0C-N4 retirement) and annotation cleanup.
3. **WU-2-43 declares WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5 (no N4) in its Phase 0C incoming list.** All four N* IDs at `:2008`. The fixture pack must cite `WU-0C-N1 fake adapter success/refusal`, `WU-0C-N3 schema-probe/safe-import gating`, `preimage mismatch`, `session busy`, `unsupported storage`, and `WU-0C-N5 override receipt propagation` (acceptance criterion at `:2000`).
4. **WU-2-45 declares WU-0C-N2 only.** Sole N* ID at `:2096`. The DTO must reject `provider-native JSONL bodies`, `mutable transcript handles`, and `non-canonical provider transcript pointers` (acceptance criteria preserved from r4).
5. **WU-2-46 and WU-2-47 declare WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5 (no N4).** All four N* IDs at `:2142` and `:2190`. Each contract must call session write-back exclusively through WU-0C-N1 `append_turns`/`truncate_after`/`replace_transcript` and propagate WU-0C-N5 receipts/refusals (`:2114` and `:2160-2161`); each acceptance set must explicitly forbid direct provider-native JSONL operations.
6. **WU-2-48/49/50 declare no WU-0C-N* edges.** Phase 0C incoming lists end at `WU-0C-37`. These services consume DetailRecord/SummaryContract data after WU-2-46/47 mediate the canonical-turn / write-back boundary.
7. **WU-2-22/23/24 keep WU-0C-11a..18 incoming and route launch/resume only through WU-0C-18 AgentRunnerClient.** Full 11a..18 cluster at `:1072`, `:1119`, `:1166`. Last acceptance criterion at `:1064`, `:1111`, `:1158` pins routing to WU-0C-18 outputs and rejects direct CLI execution. None of WU-2-22/23/24 acquires any `WU-0C-N*` edge.
8. **WU-0C-N4 cleanly retired across the entire file.** `grep "WU-0C-N4" product-strategy/ai-roadmap-phase-2.md` returns zero hits. The Stitch-Notes Incoming-From-Phase-0C list ends at `(WU-0C-N5, VS-010)` with no N4 row.
9. **Bidirectional consistency on the SessionOverrideContract surface.** Phase 0C-r5 line 3610 declares VS-010 consumes WU-0C-N1/N2/N3/N5 for turn-decomposition / detail-injection write-back; Phase 2 r5 Stitch Notes "Incoming From Phase 0C" emits the four `(WU-0C-N1..N3, N5, VS-010)` pairs at `:2763-2766`. Phase 0C-r5 line 3586 declares the WU-0C-N1/N2/N3/N5 → VS-010/012/018/020/021 foundation-row outgoing edge; Phase 2 r5 Stitch Notes Outgoing-to-Phase-3+ for WU-2-46/47 cite `WU-0C-N1`-mediated write-back and `WU-0C-N5` receipts at `:2945-2947`.
10. **`Blocked-on` annotations absent.** `grep -E "Blocked-on|Blocked on" product-strategy/ai-roadmap-phase-2.md` returns zero hits. Each of the eight r4-affected WUs carries one `**Revision rationale:**` cascade note at `:1076`, `:1123`, `:1170`, `:1260`, `:2011`, `:2101`, `:2147`, `:2195`.
11. **OptimizerRequest emission contract preserved.** WU-2-18 (`source_type=orchestrator_turn`) and WU-2-39 (`source_type=backend_signal`) both depend on WU-0C-27 OptimizerQueueService (Stitch-Notes pairs at `:2715-2719` and `:2750-2754`); per-task model dispatch routes through WU-2-34 OptimizerModelInvocationAdapter for VS-010 pipeline cycles.
12. **D3 and D4 honest.** D3 enumerates the actual artifacts read (Phase 2 r4, Phase 2 r5 audit, proposal-r6, engineering-roadmap-r5, Phase 0C-r5 WU-0C-N1..N3+N5) and disclaims the 3-gate risk loop; D4 classifies r5 as `fix-created-family` gen 0 externally-driven by the agent-runner-shipped cascade and keeps `session-override-boundary-family` LOW.

If any of these conditions break in a future round (e.g., a per-CLI turn adapter quietly re-acquires direct CLI launch language, or a future r6 introduces a Phase 2 internal edge that pushes a WU into a different wave, or a new VS-010 service is added without WU-0C-N1/N2/N3/N5 in its Phase 0C incoming block, or a Phase 0C-r6 reintroduces a schema-probe split that Phase 2 has not consumed, or a `Blocked-on` annotation is reintroduced after the agent-runner cascade has shipped), the Dep gate should re-fire MEDIUM under the appropriate watch-signal family (`session-override-boundary-family`, `parallelization-map-family`, or `dependency-encoding-family`).
