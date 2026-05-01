# AI — Coverage Risk Assessment (Phase 2, round 4)

**Rating: LOW**

## Scope and inputs

- Artifact: `product-strategy/ai-roadmap-phase-2.md` (round 4 brownfield, 50 WUs unchanged from r3 — `### WU-2-` count = 50). Round 4 is an audit-driven minor edit-pass, not a full cascade.
- Cascade: `product-strategy/proposal.md` (proposal-r5, commit `c9ee3f9`) introduces the `SessionOverrideContract` axiom plus agent-runner boundary. `product-strategy/engineering-roadmap.md` (engineering-roadmap-r4, commit `b58d8c9`) consumes the trait at VS-010/012/018/020/021. `product-strategy/ai-roadmap-phase-0c.md` (Phase 0C r4, commit `aef7a56`) introduces WU-0C-N1..WU-0C-N5 (trait + DTOs + v1 adapter + schema probe + override store). Phase 0C r4 risk gates closed LOW/LOW/LOW (commit `8068dcb`).
- Per-phase audit history: `plans/audit/ai-roadmap-phase-2.md` (r1 = LOW/LOW/MEDIUM; r2 = LOW/LOW/LOW after Parallelization Map re-derive; r3 = LOW/LOW/LOW after VS-010 cascade; r4 round-4 entry at lines 110-118 prescribes the localized scope edits). Convergence rule: r1/r2/r3 = LOW for Coverage. r4 is brownfield-driven by an external cascade and is re-checked against the refactored WUs and the new WU-0C-N* edges.
- Round 4 mandate: minor edit-pass localized to WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, and WU-2-47. WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, and WU-2-48..50 remain clean (audit `ai-roadmap-phase-2.md` line 114). No new WU is added; no WU is removed; the 11-wave Parallelization Map is unchanged. Per-phase finding IDs use the `R4-COVERAGE-F<NN>` prefix.

## Round 4 diff scope (verified)

The r4 commit `378b4f3` ("ai-roadmap-phase-2: r4 SessionOverrideContract integration (audit-driven minor edit-pass)") modifies `product-strategy/ai-roadmap-phase-2.md` with 92 insertions / 51 deletions. The diff hunks fall into eight regions: Phase 2 Scope (line 31), WU-2-22 / WU-2-23 / WU-2-24 (lines 1036-1166), WU-2-26 (lines 1216-1257), WU-2-43 (lines 1968-2013), WU-2-45 (lines 2058-2099), WU-2-46 (lines 2103-2147), WU-2-47 (lines 2149-2195), Critical Path/Run Report/D1/D3/D4 prose (lines 2386-2515), and Stitch Notes (lines 2620-2950). No edits in WU bodies for WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48..50.

## Findings

### R4-COVERAGE-F01. SessionOverrideContract operations are consumed by WU-2-46, WU-2-47, and WU-2-43 with binary criteria

**Severity: NONE (positive)**

`product-strategy/proposal.md` Round 5 §1 and the `SessionOverrideContract` axiom (proposal-r5 commit `c9ee3f9`) define seven operations: `schema_version_probe`, `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, `append_turns`, `get_session_metadata`. Phase 0C r4 binds each operation to a Phase 0C WU (WU-0C-N1..WU-0C-N5; verified LOW in `plans/risk/ai-coverage-phase-0c.md` round 4 R4-COVERAGE-F01). Phase 2 r4 only consumes the write-back operations (`replace_transcript`, `truncate_after`, `append_turns`) and the canonical-turn evidence DTOs (WU-0C-N2), with WU-0C-N5 receipts/refusals propagated. Per-WU coverage:

| Phase 2 owner | Operations consumed | Binary criteria |
|---|---|---|
| WU-2-46 TurnDecompositionService (lines 2103-2147) | `append_turns`, `truncate_after`, `replace_transcript`; WU-0C-N2 canonical TranscriptTurn evidence; WU-0C-N5 receipts/refusals | Contract block lines 2111, 2114-2115 + AC lines 2127, 2132, 2133, 2134 (per-method binary AC; canonical-evidence rejection of raw provider JSONL; WU-0C-N1 write-back enumeration; WU-0C-N5 propagation; "never opens, locates, parses, truncates, rewrites, or appends provider-native JSONL directly") |
| WU-2-47 DetailInjectionRouterService (lines 2149-2195) | `append_turns`, `truncate_after`, `replace_transcript`; WU-0C-N5 receipts/refusals | Contract block lines 2160-2161 + AC lines 2173 (per-method binary AC), 2179 (graph-mutation proposal-only invariant), 2180 (WU-0C-N1 enumeration), 2181 (WU-0C-N5 propagation), 2182 ("never opens, locates, truncates, rewrites, or appends per-CLI JSONL directly") |
| WU-2-43 SummaryRefreshFixturePack (lines 1968-2013) | WU-0C-N1 fake-adapter success/refusal, WU-0C-N3 v1 idle-only write-back, preimage mismatch, session busy, unsupported storage, WU-0C-N5 override receipt propagation | Contract block lines 1980-1981 + AC line 2000 (fixture coverage assertion; explicit prohibition of direct provider JSONL fixtures outside WU-0C-N3) |

The four read-only operations `schema_version_probe`, `locate_session`, `read_transcript`, and `get_session_metadata` are not directly consumed by Phase 2 (they are the read entrypoints for VS-018 / Phase 5 worker output reintegration); their absence from Phase 2 owning WUs is correct, not a gap. WU-2-46 line 2127 ("Accepted turn input is WU-0C-N2 canonical TranscriptTurn / bundle evidence from WU-0C-N1 reads or normalized Phase 0C session-turn evidence; raw provider JSONL input is rejected") nonetheless binds Phase 2 to WU-0C-N1 read flow when canonical evidence is sourced. The Phase 0C r4 audit-driven engineering-roadmap-r4 boundary (engineering-roadmap line 661 "VS-010, VS-012, VS-018, VS-020, and VS-021 depend on this trait, not on either adapter") is honored at the WU-binding level.

**Recommendation:** No action required.

---

### R4-COVERAGE-F02. Refactored WUs (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-45) retain functional acceptance criteria

**Severity: NONE (positive)**

The audit prescribes a contract-narrowing pass: WU-2-22..24 are narrowed from per-CLI launch/resume ownership to provider capability adapters mediated by `AgentRunnerClient` (WU-0C-18); WU-2-26 fixture-pack scope is narrowed away from direct CLI/JSONL fixtures; WU-2-45 DetailRecord input is narrowed to canonical WU-0C-N2 evidence. For each, the round-3 functional AC body is preserved and a SessionOverrideContract / AgentRunnerClient boundary criterion is appended:

- **WU-2-22 ClaudeTurnAdapter** (lines 1032-1077). Contract `contract_kind` becomes "Provider capability adapter over AgentRunnerClient" (line 1039). Method signature `request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession` (line 1043) replaces `launch_or_resume(prepared_turn)` from r3, but the per-method binary AC is preserved (line 1054, "happy path, documented denial or blocked path, exact failure error, idempotency or duplicate-call behavior, and no-out-of-scope-mutation invariant"). Round-3 functional ACs at lines 1053, 1055-1063 carry forward byte-identically (round-trip fixture, exact-error rejection, single-concern PR-ability, audit/trace ref, no-upstream-redefinition, single OrchestratorTurn row, GraphAction action_type whitelist, OptimizerRequest source_type/advisory_state, /compact handling, error-mapping). Boundary AC at line 1064 ("Launch, resume, provider routing, provider/account selection, session-id capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `claude` directly or infer Claude storage paths is rejected") is the only addition. AC count: 12.
- **WU-2-23 CodexTurnAdapter** (lines 1079-1124). Same shape: `request_turn` (line 1090) + boundary AC at line 1111 forbidding direct `codex` execution / Codex thread storage inference. Round-3 ACs preserved at lines 1100-1110. AC count: 12.
- **WU-2-24 OpencodeTurnAdapter** (lines 1126-1171). Same shape: `request_turn` (line 1137) + boundary AC at line 1158 forbidding direct `opencode` execution / opencode session-row inference. Round-3 ACs preserved at lines 1147-1157. AC count: 12.
- **WU-2-26 OrchestratorTurnFixturePack** (lines 1216-1261). Contract block adds two narrowing lines (lines 1228-1229) that constrain provider-launch fixtures to `agents`/WU-0C-18 invocation evidence and forbid direct provider CLI command/resume/session-row/thread-storage/JSONL fixtures. ACs lines 1247-1248 add the WU-2-22/WU-2-23/WU-2-24 fixture-coverage assertion plus the post-acceptance capability-only constraint. The round-3 fixture-pack functional ACs at lines 1237-1246 (round-trip fixture, exact-error rejection, single-concern PR-ability, audit/trace ref, no-upstream-redefinition, single OrchestratorTurn row, GraphAction whitelist, OptimizerRequest source_type, /compact handling, fixture per-Contract enumeration) carry forward. AC count: 12.
- **WU-2-45 DetailRecordSchemaDto** (lines 2058-2101). Contract block adds the canonical-evidence sourcing line (line 2069) requiring `raw_span_refs` and turn refs to be derived from WU-0C-N2 DTOs and rejecting provider-native JSONL bodies / mutable transcript handles. Source-basis updated to proposal-r5 / engineering-roadmap-r4 / Phase 0C r4 WU-0C-N2 (line 2073). ACs lines 2085-2086 add canonical-evidence validation and provider-native input rejection. The round-3 r3-closure ACs (line 2081 round-trip; line 2082 exact-error rejection; line 2083 every-detail_type/scope/source_fidelity binary fixture; line 2084 raw_span_ref or evidence_id requirement; line 2087 detail_id / dedupe_key stability; line 2088 single-concern PR-ability) carry forward byte-identically. AC count: 8.

All five refactored WUs preserve their round-3 functional acceptance bodies; the round-4 additions are strictly boundary criteria for AgentRunnerClient or SessionOverrideContract. No method/enum/state-machine surface is removed; the `state-machine-criteria-family` WUs (WU-2-08 lines 428-434 with 6 valid + 4 invalid transitions; WU-2-14 lines 693-716 with 12 states + 11 valid + 6 invalid transitions; WU-2-38 lines 1764-1769 with 4 valid + 4 invalid transitions plus deterministic-first MiniMax-M2.7 ambiguity criterion) are not in the r4 edit set and remain byte-identical to r3.

**Recommendation:** No action required.

---

### R4-COVERAGE-F03. Cross-phase incoming-from-Phase-0C edges (audit §3) are present and correctly scoped

**Severity: NONE (positive)**

The audit `Round 4` entry (lines 114-117) prescribes that WU-0C-N1..WU-0C-N5 incoming edges are added only to the WUs that consume SessionOverrideContract operations, DTOs, fixtures, receipts, or refusals. WU-2-22..24 must NOT add WU-0C-N* edges because they route launch/resume through WU-0C-18 AgentRunnerClient, not the trait. Verified per-WU Dependencies blocks:

| WU | Phase 0C Dependencies line | WU-0C-N* edges added | Audit prescription |
|---|---|---|---|
| WU-2-22 ClaudeTurnAdapter | line 1072 | none | ✓ Correct (routes via WU-0C-18) |
| WU-2-23 CodexTurnAdapter | line 1119 | none | ✓ Correct (routes via WU-0C-18) |
| WU-2-24 OpencodeTurnAdapter | line 1166 | none | ✓ Correct (routes via WU-0C-18) |
| WU-2-26 OrchestratorTurnFixturePack | line 1256 | none | ✓ Correct (fixture pack covers AgentRunnerClient evidence only) |
| WU-2-43 SummaryRefreshFixturePack | line 2008 | WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5 | ✓ Required (fake-adapter, DTO, v1 adapter, schema probe, override store fixtures) |
| WU-2-45 DetailRecordSchemaDto | line 2096 | WU-0C-N2 only | ✓ Required (canonical TranscriptTurn DTO evidence) |
| WU-2-46 TurnDecompositionService | line 2142 | WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5 | ✓ Required (write-back trait, DTO evidence, v1 adapter, schema probe, override store) |
| WU-2-47 DetailInjectionRouterService | line 2190 | WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5 | ✓ Required (write-back trait, DTO evidence, v1 adapter, schema probe, override store) |
| WU-2-48 IncrementalSummaryUpdateService | line 2231 | none | ✓ Correct (CLEAN; consumes optimizer pipeline only) |
| WU-2-49 FullSummaryRegenerationService | line 2273 | none | ✓ Correct (CLEAN) |
| WU-2-50 StaleMarkDetectionService | line 2314 | none | ✓ Correct (CLEAN) |

Stitch Notes "Incoming From Phase 0C" prose at line 2623 confirms the prescription mechanically: "VS-009 provider capability adapters WU-2-22..24 preserve incoming WU-0C-11a..18 through WU-0C-18 AgentRunnerClient; they do not add WU-0C-N* edges. ... Round 4 adds WU-0C-N1..N5 only where SessionOverrideContract write-back, DTO evidence, fixtures, receipts, or refusals are consumed: WU-2-43, WU-2-45, WU-2-46, and WU-2-47. WU-2-48, WU-2-49, and WU-2-50 remain clean." The Stitch Notes per-pair list at lines 2763-2767 enumerates the (WU-0C-N1..N5, VS-010) pairs (5 entries, one per WU-0C-N* node). Bidirectional consistency with Phase 0C Stitch Notes outgoing-to-Phase-2+ holds: Phase 0C r4 line 3669 lists "SessionOverrideContract WU-0C-N1..WU-0C-N5 for turn-decomposition/detail-injection write-back" as the VS-010 outgoing block, matching the Phase 2 incoming side exactly.

**Recommendation:** No action required.

---

### R4-COVERAGE-F04. Stitch Notes outgoing edges (audit §4) are present

**Severity: NONE (positive)**

The audit prescribes outgoing Stitch Notes wording updates so that downstream phases (Phase 3+) can stitch correctly. Verified at the per-WU outgoing list and the engineering-roadmap dependency-row prose:

| Audit-prescribed edge | Stitch Notes location | Source line |
|---|---|---|
| WU-2-22..24 outgoing renamed to "AgentRunnerClient-mediated provider capability adapters" | lines 2896-2897 | "(WU-2-22..WU-2-24 AgentRunnerClient-mediated provider capability adapters, VS-015)" / "(..., VS-016)" |
| WU-2-43 SummaryRefreshFixturePack → VS-012 | line 2926 | "(WU-2-43 SummaryRefreshFixturePack, VS-012)" |
| WU-2-43 SummaryRefreshFixturePack → VS-018 | line 2927 | "(WU-2-43 SummaryRefreshFixturePack, VS-018)" |
| WU-2-46 TurnDecompositionService → VS-012 | line 2928 | "(WU-2-46 TurnDecompositionService, VS-012)" |
| WU-2-46 TurnDecompositionService → VS-018 | line 2929 | preserved from r3 |
| WU-2-46 TurnDecompositionService → VS-019 | line 2930 | preserved from r3 |
| WU-2-47 DetailInjectionRouterService → VS-012 | line 2931 | preserved from r3 |
| WU-2-47 DetailInjectionRouterService → VS-018 | line 2932 | preserved from r3 |

Engineering-roadmap dependency-row prose (lines 2944-2950) explicitly enumerates the new write-back boundary: "WU-2-46 TurnDecompositionService feeds VS-012 repack planning with canonical transcript turns and detail decomposition; any packed transcript replacement remains WU-0C-N1-mediated and carries WU-0C-N5 receipts/refusals" (line 2946); "WU-2-47 DetailInjectionRouterService feeds VS-012 with SessionOverrideContract receipts/refusals for packed transcript or canonical-turn write-back; graph/topology mutation remains proposal-only" (line 2947); "WU-2-46 and WU-2-47 feed VS-018 worker-output reintegration through the same canonical turns, detail routing, and WU-0C-N1 write-back boundary" (line 2948); "WU-2-43 SummaryRefreshFixturePack feeds VS-012 and VS-018 fixture flow-through for WU-0C-N1 fake adapter success/refusal and WU-0C-N3 idle-only write-back/refusal cases" (line 2949). The VS-009 row at line 2945 also updates "per-CLI adapters" → "AgentRunnerClient-mediated provider capability adapters" so that downstream phases that read this row (Phase 3+ reviewer / VS-015 worker dispatch / VS-018 worker output) inherit the boundary semantics correctly.

The outgoing edges retained byte-identically from r3 (e.g., WU-2-44 → VS-020, WU-2-45 → VS-011/VS-018, WU-2-48..50 → VS-014/VS-018/VS-019) are intentional: those WUs are CLEAN at the body level, and their outgoing semantics are unchanged.

**Recommendation:** No action required.

---

### R4-COVERAGE-F05. r1/r2/r3 closures hold under r4

**Severity: NONE (confirming-r3)**

The four carry-forward closures from prior rounds remain intact:

1. **`state-machine-criteria-family` in remission.** WU-2-14 OrchestratorTurnStateMachine (lines 693-716) still enumerates 12 state-reachability + 11 valid-transition + 6 invalid-transition criteria; WU-2-08 WalkStateTransitionValidator (lines 428-434) still carries 6 valid + 4 invalid transitions; WU-2-38 SummaryNodeStaleStateTransitionHandler (lines 1764-1769) still carries 4 valid + 4 invalid transitions plus the deterministic-first + bounded MiniMax-M2.7 ambiguity criterion. None of these WUs are in the r4 edit set; the r3→r4 diff has zero hits in their bodies.
2. **WU-2-34 OptimizerModelInvocationAdapter per-task dispatch criteria intact.** Lines 1565-1588 still bind MiniMax-M2.7 to `turn_decomposition`, `detail_injection_routing`, `incremental_summary_update`, `full_summary_regeneration`, `stale_mark_detection`, `cross_reference_discovery`, `repack_planning`, `optimizer_scoping`, `sub_agent_provider_routing`; Claude Opus 4.7 to `conflict_on_stale_base`; Claude Sonnet 4.6 to `lead_orchestrator_provider_routing`; GPT-5.5 to `reviewer_sampling`. The provider-exclusion criterion (line 1587) for GLM/Gemini/Qwen/Mistral/DeepSeek is unchanged. WU-2-34 is not in the r4 edit set.
3. **VS-010 pipeline binary criteria intact.** WU-2-48 (lines 2197-2236), WU-2-49 (lines 2238-2278), WU-2-50 (lines 2280-2319) carry the requires_full_regen trigger, 200K precondition, trigger enum (5 variants), action enum (7 variants in WU-2-47), 9-variant stale label enum (WU-2-50), and per-task MiniMax-M2.7 dispatch criterion in their original r3 form. None are in the r4 edit set.
4. **`bundling-family` and `dependency-encoding-family` closed.** WU count remains 50; D1 ownership table at lines 2444-2477 is byte-stable except for the contract_kind label change for WU-2-22/23/24 (lines 2484-2486). Cross-phase incoming-edge sets are exhaustive (Phase 0A / 0B / 0C / 1) for every WU and bidirectional with Phase 0C Stitch Notes outgoing.

The r3 LOW/LOW/LOW verdict (commits `2c2d222`, `6d5beaa`, `56d8a3c`) is not regressed by r4.

**Recommendation:** No action required.

---

### R4-COVERAGE-F06. CLEAN WUs (WU-2-48/49/50 + 39 others = 42 total) byte-stable in WU bodies

**Severity: NONE (positive)**

The audit `Round 4` entry (line 114) prescribes that WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, and WU-2-48..50 remain CLEAN — a 42-WU set (21 + 1 + 16 + 1 + 3 = 42; the prompt's "WU-2-48/49/50 + 41 others" totals 44, off by 2 relative to the audit's enumeration; the audit's 42 is the operative figure and is what the diff verifies). The r3→r4 diff against `product-strategy/ai-roadmap-phase-2.md` (commit `5ace819` → `378b4f3`, 92 insertions / 51 deletions) has hunk headers only at lines 31, 1036-1166 (WU-2-22/23/24), 1216-1257 (WU-2-26), 1965-2010 (WU-2-43), 2058-2099 (WU-2-45), 2103-2147 (WU-2-46), 2149-2195 (WU-2-47), and the structural sections (Critical Path, Run Report, D1 audit table, Stitch Notes). No diff hunk overlaps the body of any of the 42 CLEAN WUs:

- **VS-008 cluster** (WU-2-01..12): byte-stable. Five command-schema WUs (WU-2-01..05), result DTO (WU-2-06), mutation service (WU-2-07), validator (WU-2-08), invalidation service (WU-2-09), audit emitter (WU-2-10), affordance adapter (WU-2-11), fixture pack (WU-2-12).
- **VS-009 cluster** (WU-2-13..21, WU-2-25, WU-2-27, WU-2-28): byte-stable. Lifecycle enum (WU-2-13), state machine (WU-2-14), render-prep (WU-2-15), bridge (WU-2-16), capture/commit (WU-2-17), advisory emitter (WU-2-18), graph-action transaction (WU-2-19), compact guard (WU-2-20), parent invocation (WU-2-21), turn UI pane (WU-2-25), audit emitter (WU-2-27), preflight adapter (WU-2-28). The state-machine surface (WU-2-08, WU-2-14, WU-2-38) is byte-stable.
- **VS-010 cluster** (WU-2-29..42, WU-2-44, WU-2-48..50): byte-stable. Optimizer scoping DTO (WU-2-29), service (WU-2-30), prompt schema (WU-2-31), response schema (WU-2-32), refresh-request DTO (WU-2-33), model-invocation adapter (WU-2-34), deterministic validator (WU-2-35), merge service (WU-2-36), conflict classifier (WU-2-37), stale-state handler (WU-2-38), backend signal emitter (WU-2-39), regeneration UI (WU-2-40), stale-marker UI (WU-2-41), optimizer log UI (WU-2-42), edit audit emitter (WU-2-44), incremental summary update (WU-2-48), full regeneration (WU-2-49), stale-mark detection (WU-2-50).

The only structural changes outside the eight modified WUs (WU-2-22/23/24/26/43/45/46/47) are the Phase 2 Scope sentence (line 31), the D1 contract_kind labels for WU-2-22..24 (lines 2484-2486 in the audit table), the D2 Rule prose (lines 2486-2489), the D3 / D4 / Self-classification prose (lines 2492-2513), and the Stitch Notes prose plus the five (WU-0C-N*, VS-010) entries (lines 2763-2767) and the four outgoing edges (lines 2926-2928, 2944-2950). No CLEAN WU's Contract, Test boundary, Code boundary, Acceptance criteria, Dependencies, Produces, or Parallelizable fields are modified. r3-closure ACs in the CLEAN set carry forward by construction.

**Recommendation:** No action required.

---

### R4-COVERAGE-F07. No orphan WUs; engineering-roadmap-r4 surface fully owned

**Severity: NONE (positive)**

Walked the 50-row WU inventory at lines 51-102 of `ai-roadmap-phase-2.md`. Every WU has a Parent initiative tied to VS-008, VS-009, or VS-010 (engineering-roadmap-r4 lines 631-639). No orphan. Engineering-roadmap-r4 cross-references resolve to specific Phase 2 WUs:

| Engineering-roadmap-r4 surface | Source line(s) | Owning Phase 2 WU(s) |
|---|---|---|
| VS-008 navigation (pack/unpack/focus/pin/unpin) | 194-212 | WU-2-01..05 (commands), 06 (result), 07 (mutation), 08 (validator), 09 (invalidation), 10 (audit), 11 (affordance), 12 (fixtures) |
| VS-009 bounded orchestrator turns | 214-232 | WU-2-13..21 (lifecycle/state/services/guard/propagation), 22..24 (provider capability adapters via AgentRunnerClient), 25..28 (UI/fixtures/audit/preflight) |
| VS-009 advisory OptimizerRequest queueing | 619-629 | WU-2-18 (source_type orchestrator_turn) |
| VS-010 turn decomposition / detail injection / summary refresh / stale detection | 244-260 | WU-2-29..50 |
| VS-010 200K bounded-node precondition | 254 | WU-2-49 line 2262 |
| VS-010 backend_signal OptimizerRequest emission | 626 | WU-2-39 (source_type backend_signal) |
| Model Routing appendix items 1-12 | 738-809 | WU-2-34 line 1586 (assignment matrix); per-service dispatch ACs in WU-2-46..50 |
| Provider exclusions (Gemini/GLM/Qwen/Mistral/DeepSeek) | 805-809 | WU-2-34 line 1587 (`provider_excluded` fail-closed); WU-2-46 line 2131 |
| SessionOverrideContract Phase 0C foundation row consumers (VS-010 / VS-012 / VS-018 / VS-020 / VS-021) | 36, 661 | Phase 2 owns the VS-010 half: WU-2-46 (write-back invocation), WU-2-47 (router write-back), WU-2-43 (fixtures), WU-2-45 (canonical DTO consumption). VS-012/VS-018/VS-020/VS-021 are correctly outside Phase 2 scope and stitched via Stitch Notes outgoing edges to Phase 3+. |
| Engineering-roadmap-r4 omission of VS-015 from the SessionOverrideContract consumer row | 36, 343-361, 661 | Correctly NOT carried as a Phase 2 SessionOverrideContract dependency — VS-015 worker dispatch reads `agents -m <model> -p <project> -f <prompt>` only, with no per-CLI session-jsonl knowledge in the harness. WU-2-22..24 outgoing to VS-015 (line 2896) carries the AgentRunnerClient boundary, not the trait. |

Phase 2 scope (line 35) explicitly excludes topology mutation, splits/merges/reparenting, cross-reference creation, repack edits, worker dispatch, NEEDS_INPUT routing, reviewer sampling policy beyond deterministic hooks, recovery execution, and provider reroute/substitution; that boundary is enforced by WU-2-34 line 1581 (optimizer path accepts only summary_regeneration and stale_mark edit types in Phase 2), WU-2-35 line 1629 (deterministic-only), WU-2-47 line 2179 (graph-mutation proposal-only), and the Non-Ownership Notes block at lines 2954-2956. No engineering-roadmap-r4 Phase 2 surface item is silently dropped.

**Recommendation:** No action required.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-COVERAGE-F01 | SessionOverrideContract write-back operations (`replace_transcript` / `truncate_after` / `append_turns`) + WU-0C-N2 DTO evidence + WU-0C-N5 receipts/refusals consumed by WU-2-46/47/43 with binary criteria | NONE |
| R4-COVERAGE-F02 | Refactored WU-2-22/23/24/26/45 retain functional ACs (12/12/12/12/8); only added criteria are AgentRunnerClient / SessionOverrideContract boundary | NONE |
| R4-COVERAGE-F03 | Cross-phase incoming-from-Phase-0C edges (audit §3) present at WU-2-43/45/46/47; absent (correctly) at WU-2-22..24/26 and WU-2-48..50 | NONE |
| R4-COVERAGE-F04 | Stitch Notes outgoing (audit §4) present: WU-2-46/47 → VS-012, VS-018; WU-2-43 → VS-012, VS-018; WU-2-22..24 outgoing wording updated to "AgentRunnerClient-mediated provider capability adapters" | NONE |
| R4-COVERAGE-F05 | r1/r2/r3 closures hold (WU-2-08 / WU-2-14 / WU-2-38 transition criteria; WU-2-34 dispatch criteria; VS-010 pipeline binary criteria; bundling/dependency-encoding closures) | NONE |
| R4-COVERAGE-F06 | 42 CLEAN WUs (WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48..50) byte-stable in WU bodies — verified via r3→r4 diff hunk header analysis | NONE |
| R4-COVERAGE-F07 | No orphan WUs; engineering-roadmap-r4 VS-008/009/010 + Model Routing + SessionOverrideContract VS-010 surface fully owned | NONE |

---

## What LOW requires

The Coverage LOW rating is valid as long as the following remain true:

1. **WU-2-46 TurnDecompositionService** preserves the WU-0C-N1 `append_turns` / `truncate_after` / `replace_transcript` enumeration (line 2114 contract; line 2132 AC), the WU-0C-N2 canonical-evidence rejection of raw provider JSONL (line 2127 AC), the WU-0C-N5 propagation criterion (line 2133 AC), and the "never opens, locates, parses, truncates, rewrites, or appends provider-native JSONL directly" invariant (line 2134 AC).
2. **WU-2-47 DetailInjectionRouterService** preserves the WU-0C-N1 write-back enumeration (line 2160 contract; line 2180 AC), the WU-0C-N5 receipts/refusals propagation (line 2181 AC), the proposal-only graph-mutation invariant (line 2179 AC), and the per-CLI JSONL prohibition (line 2182 AC).
3. **WU-2-43 SummaryRefreshFixturePack** preserves the SessionOverrideContract fixture-coverage criterion (line 2000 AC) covering WU-0C-N1 fake-adapter / WU-0C-N3 v1 idle-only write-back / preimage mismatch / session busy / unsupported storage / WU-0C-N5 override receipts.
4. **WU-2-45 DetailRecordSchemaDto** preserves the WU-0C-N2-derived raw_span_refs criterion (line 2085 AC) and the provider-native JSONL / mutable transcript handle rejection (line 2086 AC).
5. **WU-2-22 / 23 / 24** preserve the AgentRunnerClient (WU-0C-18) launch/resume/provider-routing/session-id-capture binding (lines 1064 / 1111 / 1158 ACs) and do NOT add WU-0C-N* incoming edges.
6. **WU-2-26 OrchestratorTurnFixturePack** preserves the AgentRunnerClient fixture-coverage assertion (line 1247 AC) and the post-acceptance capability-only constraint (line 1248 AC).
7. **WU-2-48, WU-2-49, WU-2-50** remain CLEAN — body unchanged from r3, no WU-0C-N* incoming edges, no SessionOverrideContract-related ACs introduced. If a future round adds SessionOverrideContract scope to summary update / regeneration / stale detection, that would be a new edit-pass and re-trigger Coverage verification.
8. **WU-2-08 (6+4 transitions), WU-2-14 (12 states + 11 valid + 6 invalid), WU-2-38 (4+4 transitions + deterministic-first MiniMax-M2.7 ambiguity)** state-machine criteria stay byte-identical. If they regress, `state-machine-criteria-family` re-fires.
9. **WU-2-34 OptimizerModelInvocationAdapter** assignment-matrix criterion (line 1586) and provider-exclusion criterion (line 1587) remain in the AC list with the same task-class enum. If a new task class is introduced upstream without a Phase 2 owner, that is a Coverage MEDIUM.
10. **No new method, enum, state, or trigger** is introduced into proposal-r5, engineering-roadmap-r4, or research-16/17-v4 §7 without a corresponding binary acceptance criterion in the Phase 2 owning WU.
11. **Stitch Notes outgoing wording for WU-2-22..24** ("AgentRunnerClient-mediated provider capability adapters") and the WU-2-43/46/47 → VS-012/VS-018 outgoing edges remain present so Phase 3+ stitching works without back-fill.
12. **Phase 0C r4** stays LOW/LOW/LOW — Phase 2 r4 inherits the WU-0C-N1..N5 surface from Phase 0C r4. If Phase 0C re-fires a Coverage finding on the trait or v1 adapter, Phase 2 must re-verify the operations actually consumed.

If items 1-4 regress, that is a Coverage MEDIUM (missing binary criterion for a declared SessionOverrideContract operation or DTO consumption). If items 5-7 regress, that is a `session-override-boundary-family` event (would be generation 1 in this local loop). If item 8 regresses, `state-machine-criteria-family` re-fires (would be generation 1). If items 9-12 regress, the Coverage gate must re-derive the engineering-roadmap → WU traceability table from scratch.
