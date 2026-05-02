# AI — Decomposition Risk Assessment (Phase 2, Round 4, audit-driven minor edit-pass)

**Rating: LOW**

Phase 2 r4 is an externally-driven minor edit-pass triggered by proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 introducing the `SessionOverrideContract` boundary (WU-0C-N1..WU-0C-N5). The audit at `plans/audit/ai-roadmap-phase-2.md` lines 110–119 prescribed targeted scope changes to exactly 8 WUs (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47); the remaining 42 WUs were named CLEAN in `tmp/scratch/phase-2-audit-r4/REPORT.md` §1. The proposer landed a 92-insertion / 51-deletion diff (143 lines changed) against `product-strategy/ai-roadmap-phase-2.md` that touches exactly those 8 WUs and no others. WU count remains 50; the 11-wave Parallelization Map is preserved byte-for-byte; no Phase 2 internal edges were added; only cross-phase incoming edges to WU-0C-N1..N5 were added (and only to the four WUs the audit prescribed). Each of the touched WUs remains single-concern per Rule D1, including the AgentRunnerClient refactor of WU-2-22/23/24 which preserves three separate per-provider capability adapters rather than collapsing or bundling them. r1/r2/r3 closures hold by construction for the 42 CLEAN WUs.

## Findings

### R4-DECOMP-F01. WU count, D1 ownership table, and 11-wave Parallelization Map preserved

**Severity: NONE (regression-only confirmed)**

`grep -c '^### WU-2-' product-strategy/ai-roadmap-phase-2.md` returns 50, matching r3 exactly. The Run Report's Rule D1 ownership table at lines 2418–2466 still contains 50 rows. Three rows changed text only — WU-2-22, WU-2-23, WU-2-24 contract_kind moved from "Per-CLI adapter" to "Provider capability adapter over AgentRunnerClient" (lines 2419–2421 of the diff hunk at 322–325). All other 47 rows are byte-identical to r3. The Run Report self-attests "D1 audit count: 50 rows = 50 WUs" still holds.

The Parallelization Map at lines 2325–2335 and the wave table at lines 2401–2411 are byte-identical to r3 — 11 waves with WU-2-22/23/24 in Wave 2, WU-2-46 in Wave 5, WU-2-47 in Wave 6, WU-2-26 in Wave 8, WU-2-43 in Wave 11, WU-2-45 in Wave 1. r4 added zero Phase 2 internal edges (only cross-phase incoming edges to WU-0C-N1..N5 from VS-010 WUs and label changes elsewhere), so the topological-from-leaf+1 partition is unchanged. `parallelization-map-family` does not re-fire.

**Recommendation:** No action required.

---

### R4-DECOMP-F02. WU-2-22/23/24 AgentRunnerClient refactor preserves single-service per-WU coherence

**Severity: NONE**

The audit hard-required boundary refactor for WU-2-22/23/24 was executed cleanly without violating Rule D1. All three WUs still own exactly one method each, renamed from `launch_or_resume(prepared_turn) -> CliAcceptedSession` to `request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession`. The audit offered two paths ("rename/narrow to a Claude tool-surface/capability adapter only, or fold into a provider-neutral `AgentRunnerTurnAdapter`"); the proposer chose the rename/narrow path, preserving three separate WUs.

D1 per-object granularity holds because each WU owns a distinct, observable per-provider concern after AgentRunnerClient acceptance:

| WU | Post-r4 owned concern (from contract block) | Single-service check |
|---|---|---|
| WU-2-22 ClaudeTurnAdapter | "Claude-specific behavior is limited to observed capability, hook, and tool-surface differences after AgentRunnerClient acceptance." | One method (`request_turn`). Provider-specific capability/hook differences are real and Claude-only. Pass. |
| WU-2-23 CodexTurnAdapter | "Codex-specific behavior is limited to normalized post-acceptance rollout/tool-surface semantics." | One method (`request_turn`). Codex rollout semantics are distinct from Claude/Opencode. Pass. |
| WU-2-24 OpencodeTurnAdapter | "Opencode-specific behavior is limited to typed substrate-gap and tool-surface evidence observed after AgentRunnerClient acceptance." | One method (`request_turn`). Opencode-specific substrate-gap evidence is distinct from Claude/Codex. Pass. |

Each WU's contract block adds three new boundary statements forbidding direct CLI execution (`claude` / `codex` / `opencode`), session storage assumption (Claude session ID / Codex thread / Opencode session-row), provider/account route selection, resume composition, and session-id capture — those concerns now belong upstream to WU-0C-18 AgentRunnerClient. Each WU also adds one new acceptance criterion enforcing the boundary as a binary test (e.g., line 43 of the diff: "Launch, resume, provider routing, provider/account selection, session-id capture, and session evidence are accepted only from WU-0C-18 AgentRunnerClient outputs; any attempt to execute `claude` directly or infer Claude storage paths is rejected.").

`grep "Per-CLI adapter\|launch_or_resume\|per-CLI turn adapter"` returns zero hits in the file — no stale references survive. The Stitch Notes outgoing-to-Phase-3+ row at line 402–403 is updated to "WU-2-22..WU-2-24 AgentRunnerClient-mediated provider capability adapters" consistently. Run Report Rule D1 row text at 2419–2421 matches.

`bundling-family` does not re-fire: the three WUs are not collapsible because their per-provider capability evidence differs. The audit's alternate "fold into a provider-neutral `AgentRunnerTurnAdapter`" path would itself be a Rule D1 question (whether one WU can own three providers' divergent capability/rollout/substrate-gap surfaces); the chosen rename/narrow path keeps each provider's distinct evidence in its own contract — strictly more granular and clearly Rule D1-compliant.

**Recommendation:** No action required.

---

### R4-DECOMP-F03. WU-2-26 OrchestratorTurnFixturePack remains a single fixture pack

**Severity: NONE**

WU-2-26 received two added contract clauses (lines 144–145 of the diff) and two added/merged acceptance criteria (lines 154–155). The new content asserts `agents` / WU-0C-18 AgentRunnerClient invocation evidence in fixtures and forbids direct provider CLI command/resume/session-row/thread-storage/JSONL fixtures outside the AgentRunnerClient boundary. No new fixture pack was created; no WU split. The fixture pack continues to own exactly the same VS-009 fixture surface (happy path, tool path, blocked render, compact detected, failed capture, recovered turn, optimizer request) plus added boundary-routing assertions for AgentRunnerClient acceptance. Provider-specific fixture variants are explicitly limited to "observed capability, hook, tool-surface, rollout, and substrate-gap differences after AgentRunnerClient acceptance" — same boundary as WU-2-22/23/24, so coherence with F02 holds.

**Recommendation:** No action required.

---

### R4-DECOMP-F04. WU-2-43 SummaryRefreshFixturePack remains a single fixture pack

**Severity: NONE**

WU-2-43 received two added contract clauses for SessionOverrideContract fixture coverage (lines 163–164 of the diff) and one new acceptance criterion (line 181) explicitly listing WU-0C-N1 fake adapter success/refusal, WU-0C-N3 v1 idle-only write-back, preimage mismatch, session busy, unsupported storage, and WU-0C-N5 override receipt propagation. The criterion also forbids direct provider JSONL open/truncate/rewrite/append/locate fixtures outside WU-0C-N3 adapter fixtures. Two pre-existing acceptance criteria were merged into one line each (lines 174 and 178 in the diff are line-consolidations of pre-r4 content, not behavior removals). No new fixture pack was created; no WU split. WU-2-43 still owns the VS-010 fixture surface end-to-end.

The audit prescribed exactly these additions in §2 ("WU-2-43 SummaryRefreshFixturePack ... add fixtures for WU-0C-N1 fake adapter success/refusal, WU-0C-N3 v1 idle-only write-back, preimage mismatch, session busy, unsupported storage, and override receipt propagation. Do not add direct JSONL mutation fixtures outside WU-0C-N3."). Match is exact.

**Recommendation:** No action required.

---

### R4-DECOMP-F05. WU-2-45 DetailRecordSchemaDto remains a single DTO

**Severity: NONE**

WU-2-45 stays a DTO-only WU. The contract gained one input-binding clause (line 202 of the diff: "raw_span_refs and turn refs are canonical TranscriptTurn/source-offset references derived from WU-0C-N2 DTOs; provider-native JSONL bodies and mutable transcript handles are rejected as DTO input.") and a Source basis update to add Phase 0C r4 WU-0C-N2 alongside the existing proposal/engineering-roadmap/research citations (lines 206–207). Two new rejection-fixture acceptance criteria were added (lines 215–216) for non-canonical transcript pointers and provider-native JSONL/file-descriptor/SQLite-handle/parser-object inputs.

No new DTO was created; the DetailRecord schema fields (detail_id, turn_id, detail_type, scope, ..., dedupe_key, base_graph_snapshot_id) and the three enums (detail_type, scope, source_fidelity) are byte-identical to r3. Cross-phase incoming gains exactly one ID (WU-0C-N2), matching the audit's §3 prescription ("WU-2-45: WU-0C-N2"). Single-DTO concern preserved.

**Recommendation:** No action required.

---

### R4-DECOMP-F06. WU-2-46 TurnDecompositionService remains a single-method service

**Severity: NONE**

WU-2-46 still owns one method, `TurnDecompositionService.decompose_turn_bundle(bundle_ref, max_adjacent_turns, base_graph_snapshot_id) -> DetailRecord[]`. The contract block now adds three boundary clauses (lines 234, 238–239 of the diff) and the acceptance criteria add four new binary criteria (lines 248, 253, 254, 255):

- Input must be canonical WU-0C-N2 `TranscriptTurn` evidence; raw provider JSONL is rejected.
- When the service emits canonical synthetic turns or packed transcript deltas for session visibility, write-back calls only WU-0C-N1 `append_turns` / `truncate_after` / `replace_transcript`.
- WU-0C-N5 receipts/refusals are propagated, not swallowed.
- The service never opens/locates/parses/truncates/rewrites/appends provider-native JSONL directly.

The conditional session write-back is a downstream delegation to WU-0C-N1 (a method on a different service in a different phase), not a second method on `TurnDecompositionService`. The contract describes it as a routing/precondition rule for the existing single method's outputs. The "Produces" line is unchanged: "TurnDecompositionService contract and tests; no unlisted downstream behavior." Single-service per Rule D1 holds.

Cross-phase incoming gains WU-0C-N1, N2, N3, N4, N5 (line 264), exactly matching the audit's §3 prescription ("WU-2-46: WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5"). Source basis updated to include proposal-r5, engineering-roadmap-r4, and Phase 0C r4 WU-0C-N1/N2/N5 (line 240). No bundling.

**Recommendation:** No action required.

---

### R4-DECOMP-F07. WU-2-47 DetailInjectionRouterService remains a single-method service

**Severity: NONE**

WU-2-47 still owns one method, `DetailInjectionRouterService.route_details(detail_records, candidate_node_scope, base_graph_snapshot_id) -> InjectionPlan[]`. The contract adds the same shape of clauses as WU-2-46 (lines 272–273): an optional session-visible output path that emits canonical turns or a packed transcript delta and then calls WU-0C-N1 `append_turns` / `truncate_after` / `replace_transcript`, returning the WU-0C-N5 receipt/refusal. Acceptance criteria add four new binary clauses (lines 285–288): proposal-only graph mutation (re-asserting an existing rule), session-visible writes go only through WU-0C-N1, WU-0C-N5 receipts/refusals are returned to callers, and direct per-CLI JSONL operations are forbidden.

As with WU-2-46, the optional session-visible write-back is a downstream delegation to WU-0C-N1, not a second method on `DetailInjectionRouterService`. The InjectionPlan output schema (action enum: append_to_node, create_node, create_edge_candidate, park, defer, quarantine, drop) is byte-identical to r3. The "Produces" line is unchanged. Single-service per Rule D1 holds.

Cross-phase incoming gains WU-0C-N1..N5 (line 297), matching the audit's §3 prescription. Source basis updated to include proposal-r5, engineering-roadmap-r4, and Phase 0C r4 WU-0C-N1/N2/N5 (line 277).

The two-WU pattern — WU-2-46 emits canonical turns; WU-2-47 emits InjectionPlans and may also emit canonical turns/packed deltas for session visibility — is consistent with the audit verdict that WU-2-46 and WU-2-47 are separate concerns (decomposition vs. routing). No service merge or split is required; no second method introduced.

**Recommendation:** No action required.

---

### R4-DECOMP-F08. Stitch Notes additions match audit §4; outgoing-edge labels updated coherently

**Severity: NONE**

Per the audit §4, four Stitch Notes outgoing additions/clarifications were prescribed. All four are present in the r4 Stitch Notes at the bottom of `ai-roadmap-phase-2.md`:

| Audit-prescribed edge | Diff location | Present? |
|---|---|---|
| `WU-2-46 TurnDecompositionService -> VS-012` | Line 413 (Outgoing-to-Phase-3+ block) | Yes |
| `WU-2-47 -> VS-012` carries SessionOverrideContract receipts/refusals | Existing edge at line 416 preserved; prose at line 425 makes the SessionOverrideContract receipt/refusal expectation explicit | Yes |
| `WU-2-46, WU-2-47 -> VS-018` carry canonical write-back boundary | Existing edges preserved; prose at line 426 makes the worker-output reintegration boundary explicit | Yes |
| `WU-2-43 SummaryRefreshFixturePack -> VS-012, VS-018` fixture flow-through | Lines 411–412 (two new outgoing rows) | Yes |

The Outgoing-to-Phase-3+ block also relabels `(WU-2-22..WU-2-24 per-CLI turn adapters, VS-015 / VS-016)` to `(WU-2-22..WU-2-24 AgentRunnerClient-mediated provider capability adapters, VS-015 / VS-016)` (lines 402–403), preserving the two existing edges with updated descriptive text consistent with the F02 contract_kind change.

The Incoming-From-Phase-0C block adds five new rows for `(WU-0C-N1..N5, VS-010)` (lines 388–392), matching the four touched VS-010 WUs (WU-2-43/45/46/47) at the per-pair level expected by the systematic-from-start pattern from Phase 1 r1.

The Engineering-roadmap dependency-rows prose (lines 423–428) gains four bullet points capturing the new SessionOverrideContract write-back semantics for WU-2-46, WU-2-47, and WU-2-43 outgoing edges, plus a relabeled VS-009 bullet to "AgentRunnerClient-mediated provider capability adapters." No outgoing edge is removed; no Phase 0A/0B/1 incoming-edge change.

**Recommendation:** No action required.

---

### R4-DECOMP-F09. r1/r2/r3 closures preserved on the 42 CLEAN WUs

**Severity: NONE**

The audit §1 named 42 CLEAN WUs (WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48..50). Inspecting the r3-to-r4 diff line by line, these 42 WUs receive zero hits in their `Contract:` / `Test boundary:` / `Code boundary:` / `Acceptance criteria:` / `Dependencies:` / `Produces:` / `Parallelizable with:` blocks. The only non-touched-WU diff hits are: (a) the file-scope boundary line at line 9 ("Phase 2 scope boundaries — Included") which gets updated to reference proposal-r5 / engineering-roadmap-r4 / Phase 0C r4 WU-0C-N1..N5 instead of proposal-r4 / engineering-roadmap-r3; (b) the Run Report self-classification rows for D1/D2/D3/D4 (lines 309–312, 333–337, 348–352, 364–366, 371) which are accurate updates rather than WU-body changes; and (c) the Parallelization Map preface line 2386 ("Optimizer branch critical path: ...") is unchanged.

This means r1's 44 LOW-converged WUs (WU-2-01..44 minus VS-010 expansions), r2's wave-rederive preservation, and r3's six new VS-010 WUs (WU-2-45..50) all survive the r4 pass with the touched-WU set strictly bounded to the audit's 8-WU prescription. WU-2-13's r3 cross-CLI routing acceptance criteria (the r3 INFO placement deviation) is byte-identical in r4 — no change to the deviation, but no escalation either.

`bundling-family`, `state-machine-criteria-family`, and `dependency-encoding-family` (the three carryover families from Phase 0A/0B/0C/1) do not re-fire on the 42 CLEAN WUs because their bodies are unchanged. WU-2-14 OrchestratorTurnStateMachine (12 states + 11 valid + 6 invalid transitions) and WU-2-08 / WU-2-38 transition criteria are intact.

**Recommendation:** No action required.

---

### R4-DECOMP-F10. fix-created-family classification is generation 0; no spurious WU additions

**Severity: NONE**

Round 4 is `fix-created-family` generation 0 in the Phase 2 local loop, externally driven by the proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 SessionOverrideContract cascade — the same shape as r3's cascade-driven brownfield (which was also fix-created-family gen 0 from a different upstream cascade). The Run Report self-classifies r4 as "externally-driven minor edit-pass" at line 371 of the diff. The Phase 2 audit history at lines 110–119 explicitly frames r4 this way: "fix-created-family is generation 0 in the Phase 2-local loop, externally driven from the upstream SessionOverrideContract cascade."

Generation-counter check: r3 closed clean (R3-DECOMP-F03 was the only INFO; r3 self-attested no same-family reviewer-finding chain). r4 is not continuing an r3 finding-chain — it is responding to a *different* upstream cascade (Phase 0C-r4 SessionOverrideContract, not Phase 0C-r3 / research-17-v4). Per the watch posture, generation-counter advancement requires a same-family reviewer-flagged r2 → r3 → r4 chain; that chain doesn't exist for any Phase 2-internal family.

**No spurious additions.** The touched-WU set (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47) matches the audit §2 REFACTOR list exactly. No new WUs were created in r4 (count stays at 50 vs. r3's 50). Each touched WU's edits trace to a specific audit-report line (audit §2 REFACTOR scope edits) and to specific upstream artifacts (proposal-r5 §SessionOverrideContract Boundary, engineering-roadmap-r4 VS-010 §"Session override impact", Phase 0C-r4 WU-0C-N1..N5). The boundary upstream is real: `grep "^### WU-0C-N" /home/nes/projects/agent-harness/worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` confirms WU-0C-N1 (SessionOverrideContract Trait), N2 (TranscriptTurn DTO), N3 (AgentRunnerDbAdapter v1), N4 (Schema Probe), N5 (Override Store Registry) are defined.

`session-override-boundary-family` (a new watch signal flagged in the audit-history) is observable on the surface only — no touched WU language re-introduces direct CLI launch/resume, JSONL mutation, or session-id capture; every such concern is explicitly forbidden by new acceptance criteria (F02, F06, F07) or by fixture-pack rules (F03, F04). This watch signal stays LOW.

**Recommendation:** No action required.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-DECOMP-F01 | WU count = 50 unchanged; D1 ownership table 50 rows (3 contract_kind text changes for WU-2-22/23/24); 11-wave Parallelization Map byte-identical to r3. | NONE |
| R4-DECOMP-F02 | WU-2-22/23/24 AgentRunnerClient refactor preserves three single-method per-provider capability adapters; method renamed `launch_or_resume` → `request_turn`; direct CLI/storage forbidden by new acceptance criteria; no bundling. | NONE |
| R4-DECOMP-F03 | WU-2-26 OrchestratorTurnFixturePack remains a single fixture pack with added AgentRunnerClient routing assertions; no split. | NONE |
| R4-DECOMP-F04 | WU-2-43 SummaryRefreshFixturePack remains a single fixture pack with added SessionOverrideContract fixtures (WU-0C-N1..N5) per audit §2; no split. | NONE |
| R4-DECOMP-F05 | WU-2-45 DetailRecordSchemaDto remains a single DTO; canonical TranscriptTurn binding via WU-0C-N2 added; non-canonical inputs rejected. | NONE |
| R4-DECOMP-F06 | WU-2-46 TurnDecompositionService remains a single-method service; conditional WU-0C-N1 write-back is a downstream delegation, not a second method. | NONE |
| R4-DECOMP-F07 | WU-2-47 DetailInjectionRouterService remains a single-method service; optional session-visible output via WU-0C-N1 is a downstream delegation. | NONE |
| R4-DECOMP-F08 | Stitch Notes outgoing additions (WU-2-43→VS-012, WU-2-43→VS-018, WU-2-46→VS-012; WU-2-47/46→VS-018 prose) match audit §4 exactly; per-CLI labels relabeled coherently. | NONE |
| R4-DECOMP-F09 | r1/r2/r3 closures preserved byte-for-byte on 42 CLEAN WUs (WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48..50). | NONE |
| R4-DECOMP-F10 | r4 is fix-created-family gen 0 in the Phase 2 local loop, externally driven; touched-WU set matches audit §2 prescription; no spurious additions. | NONE |

Oscillation classification: r4 is a `fix-created-family` round at generation 0, externally driven by the Phase 0C-r4 / proposal-r5 / engineering-roadmap-r4 SessionOverrideContract cascade. No same-family reviewer chain advances. `bundling-family`, `state-machine-criteria-family`, `dependency-encoding-family`, and `parallelization-map-family` all remain LOW. The new `session-override-boundary-family` watch signal flagged in the audit history is observable as LOW: every touched WU's contract or acceptance criteria explicitly forbid direct provider CLI command, resume composition, session-id capture, session-row mapping, thread storage, or JSONL mutation; all such operations are routed through WU-0C-18 (AgentRunnerClient) for VS-009 launch/resume and through WU-0C-N1..N5 (SessionOverrideContract) for VS-010 transcript write-back.

## What LOW requires

- WU count is 50, unchanged from r3: VS-008 = 12, VS-009 = 16, VS-010 = 22.
- D1 ownership table inside the Run Report contains 50 rows; only the contract_kind cell for WU-2-22, WU-2-23, WU-2-24 changes from "Per-CLI adapter" to "Provider capability adapter over AgentRunnerClient".
- WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48, WU-2-49, WU-2-50 (42 WUs) remain byte-for-byte identical to the r3 baseline in their `Contract:`, `Test boundary:`, `Code boundary:`, `Acceptance criteria:`, `Dependencies:`, `Produces:`, and `Parallelizable with:` blocks.
- WU-2-22 ClaudeTurnAdapter, WU-2-23 CodexTurnAdapter, WU-2-24 OpencodeTurnAdapter each own one method `request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession`; each adds one new acceptance criterion forbidding direct CLI execution and out-of-boundary storage assumptions; cross-phase incoming dependency lists are byte-identical to r3 (no WU-0C-N* edge added, per audit §3 explicit "no WU-0C-N* edge" prescription).
- WU-2-26 OrchestratorTurnFixturePack contract block adds two clauses asserting AgentRunnerClient invocation evidence in fixtures and forbidding direct provider CLI/resume/session-row/thread/JSONL fixtures outside that boundary; no new fixture pack is created; the fixture surface (happy path, tool path, blocked render, compact detected, failed capture, recovered turn, optimizer request) is preserved.
- WU-2-43 SummaryRefreshFixturePack contract block adds two clauses for SessionOverrideContract coverage and forbids direct JSONL mutation fixtures outside WU-0C-N3 adapter fixtures; cross-phase incoming gains WU-0C-N1, N2, N3, N4, N5; one new acceptance criterion enumerates the WU-0C-N* fixtures.
- WU-2-45 DetailRecordSchemaDto remains a DTO with a single contract block; raw_span_refs and turn refs are bound to canonical WU-0C-N2 evidence; provider-native JSONL/file-descriptor/SQLite-handle/parser-object inputs are rejected by two new acceptance criteria; cross-phase incoming gains exactly WU-0C-N2.
- WU-2-46 TurnDecompositionService owns one method `decompose_turn_bundle(...)`; conditional canonical-turn or packed-delta write-back is a downstream call to WU-0C-N1 with WU-0C-N2 preconditions; WU-0C-N5 receipts/refusals are propagated; direct provider JSONL operations are forbidden by acceptance criteria; cross-phase incoming gains WU-0C-N1..N5.
- WU-2-47 DetailInjectionRouterService owns one method `route_details(...)`; optional session-visible output is a downstream call to WU-0C-N1; WU-0C-N5 receipts/refusals are returned to callers; graph/topology mutation remains proposal-only; cross-phase incoming gains WU-0C-N1..N5.
- The 11-wave Parallelization Map (Wave 1: 13 WUs through Wave 11: 1 WU) is byte-identical to r3; no Phase 2 internal edges were added in r4.
- Stitch Notes Incoming-From-Phase-0C block gains five new `(WU-0C-N1..N5, VS-010)` rows matching the four touched VS-010 WUs with new dependencies. Outgoing-to-Phase-3+ block gains three rows: `(WU-2-43 SummaryRefreshFixturePack, VS-012)`, `(WU-2-43 SummaryRefreshFixturePack, VS-018)`, `(WU-2-46 TurnDecompositionService, VS-012)`. The two existing per-CLI adapter rows are relabeled to "AgentRunnerClient-mediated provider capability adapters". Engineering-roadmap dependency-rows prose adds four bullets capturing the SessionOverrideContract write-back semantics. No outgoing edge is removed.
- The Run Report's Rule D4 watch-signal table self-classifies r4 as fix-created-family generation 0, externally driven by proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4; `dependency-encoding-family` LOW after targeted WU-0C-N* edge pass; `parallelization-map-family` LOW with unchanged 11-wave topology; `bundling-family` LOW; `state-machine-criteria-family` LOW (WU-2-14 unchanged); `session-override-boundary-family` LOW given explicit boundary acceptance criteria on every touched WU.
- `grep "Per-CLI adapter\|launch_or_resume\|per-CLI turn adapter" product-strategy/ai-roadmap-phase-2.md` returns zero hits; the AgentRunnerClient refactor leaves no stale terminology.

---

# AI — Decomposition Risk Assessment (Phase 2, Round 5, Option A brownfield edit-pass)

**Rating: LOW**

Phase 2 r5 is an externally-driven Option A edit-pass triggered by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 after the agent-runner team shipped all five SessionOverrideContract feature requests. The audit at `plans/audit/ai-roadmap-phase-2.md` round-5 entry (lines 120–126) prescribes targeted annotation and dependency cleanup only: remove `Blocked-on` annotations from the 8 r4-affected WUs (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47), retire the dropped Phase 0C-r4 schema-probe split (WU-0C-N4) so VS-010 schema-probe coverage now flows through WU-0C-N3, and stamp each touched WU with an r5 cascade note that confirms the r4 SessionOverrideContract criteria are retained. The proposer landed commit `ee68f91` with a 32-insertion / 31-deletion diff (63 lines changed) against `product-strategy/ai-roadmap-phase-2.md`. WU count remains 50; the 11-wave Parallelization Map (13+8+5+5+5+4+4+2+2+1+1 = 50) is byte-identical to r4; the D1 ownership table at lines 2426–2477 still contains 50 single-object rows; no Phase 2 internal edges were added or removed; the only Stitch-Notes edge change is the deletion of `(WU-0C-N4, VS-010)` plus the prose update describing the new WU-0C-N1/N2/N3/N5 set. r4 closures hold by construction for the 8 r4-affected WUs (their r4 contract / test boundary / acceptance criteria / cross-phase incoming bodies survive byte-for-byte except for the targeted N4→N3 swap and the appended cascade note); r1/r2/r3 closures hold by construction for the 42 r5-unaffected WUs (zero diff hits inside their bodies).

## Findings

### R5-DECOMP-F01. WU count = 50 preserved; D1 ownership table unchanged; 11-wave Parallelization Map byte-identical to r4

**Severity: NONE (regression-only confirmed)**

`grep -c '^### WU-2-' product-strategy/ai-roadmap-phase-2.md` returns 50, matching r4 exactly (Wave 1 leaf DTOs/enums/guards/adapters: WU-2-01..06, WU-2-13, WU-2-20, WU-2-21, WU-2-28, WU-2-29, WU-2-33, WU-2-45 = 13; Wave 2 first validators/adapters: WU-2-08, WU-2-14, WU-2-22, WU-2-23, WU-2-24, WU-2-27, WU-2-30, WU-2-39 = 8; through Wave 11 WU-2-43 = 1; sum = 50). The D1 ownership table at lines 2426–2477 still has 50 rows, each naming a single object owner (DTOs, services, validators, state machines, fixture packs, UI components, audit emitters, capability adapters); zero rows were added, removed, or reclassified in r5. No bundling, no splits.

The wave-table at lines 2400–2411 and the Parallelization Map preface are byte-identical to r4 — `git diff 335a161..ee68f91 -- product-strategy/ai-roadmap-phase-2.md` shows no diff hunks against the wave block. r5 added zero Phase 2 internal edges and removed only a single cross-phase incoming edge (`WU-0C-N4`) on WU-2-43, WU-2-46, and WU-2-47, plus the matching `(WU-0C-N4, VS-010)` row from the Stitch-Notes Incoming-From-Phase-0C list. The topological-from-leaf+1 partition is unchanged. `parallelization-map-family` does not re-fire.

**Recommendation:** No action required.

---

### R5-DECOMP-F02. Per-schema-object ownership preserved; no blob WUs introduced

**Severity: NONE**

Every one of the 50 D1 rows still owns exactly one schema object or service method:

- DTOs (12): WU-2-01..06, WU-2-13 (enum), WU-2-29, WU-2-31, WU-2-32, WU-2-33, WU-2-45 — each owns exactly one schema with field/enum positive and negative fixtures.
- Method-bearing services (24): WU-2-07, WU-2-09, WU-2-10, WU-2-11, WU-2-15, WU-2-16, WU-2-17, WU-2-18, WU-2-19, WU-2-20, WU-2-21, WU-2-25, WU-2-27, WU-2-28, WU-2-30, WU-2-34, WU-2-35, WU-2-36, WU-2-37, WU-2-39, WU-2-44, WU-2-46, WU-2-47, WU-2-48, WU-2-49, WU-2-50 — each owns one method.
- State-machine validators (3): WU-2-08, WU-2-14, WU-2-38 — each owns one transition table with explicit invalid-transition rejections.
- UI components / surfaces (4): WU-2-25, WU-2-40, WU-2-41, WU-2-42 — each owns one surface with loading/empty/happy/blocked/failed/stale states.
- Provider capability adapters (3): WU-2-22 ClaudeTurnAdapter, WU-2-23 CodexTurnAdapter, WU-2-24 OpencodeTurnAdapter — each owns one method `request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession` over WU-0C-18 AgentRunnerClient.
- Fixture packs (3): WU-2-12 NavigationFixturePack, WU-2-26 OrchestratorTurnFixturePack, WU-2-43 SummaryRefreshFixturePack — each owns one VS-scoped fixture surface.

No WU was merged, collapsed, or otherwise reshaped to bundle multiple objects in r5. No new WU was added, so no new bundling vector exists. The D1 ownership classification cells (column 3 of lines 2428–2477) are byte-identical to r4 — including the r4 "Provider capability adapter over AgentRunnerClient" classification on WU-2-22/23/24 and the "Fixture pack" classification on WU-2-26/43. `bundling-family` does not re-fire.

**Recommendation:** No action required.

---

### R5-DECOMP-F03. r4 closures preserved on the 8 r4-affected WUs (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47)

**Severity: NONE**

For each of the 8 r4-affected WUs, the r5 edit-pass retains the r4 contract block, test boundary, and acceptance criteria byte-for-byte except for the targeted dropped-N4→retained-N3 dependency adjustment, the WU-0C-N3 phrase narrowing, and an appended single-line `Revision rationale:` stamp.

| WU | r4 closure that must hold | r5 evidence |
|---|---|---|
| WU-2-22 ClaudeTurnAdapter | R4-DECOMP-F02 single-method `request_turn` per-Claude capability surface; AgentRunnerClient acceptance; direct `claude` execution / Claude session storage forbidden by acceptance criterion. | r5 diff touches only the inserted `**Revision rationale:**` line at 1076; contract block / test boundary / acceptance criteria / cross-phase incoming are byte-identical to r4. |
| WU-2-23 CodexTurnAdapter | R4-DECOMP-F02 single-method per-Codex capability surface; direct `codex` / thread-storage forbidden. | r5 diff touches only the inserted `**Revision rationale:**` line at 1123; rest byte-identical. |
| WU-2-24 OpencodeTurnAdapter | R4-DECOMP-F02 single-method per-Opencode capability surface; direct `opencode` / session-row forbidden. | r5 diff touches only the inserted `**Revision rationale:**` line at 1170; rest byte-identical. |
| WU-2-26 OrchestratorTurnFixturePack | R4-DECOMP-F03 single fixture pack with AgentRunnerClient routing assertions and forbidden direct provider CLI/JSONL fixtures. | r5 diff touches only the inserted `**Revision rationale:**` line at 1260; contract clauses, fixture surface, and acceptance criteria are byte-identical to r4. |
| WU-2-43 SummaryRefreshFixturePack | R4-DECOMP-F04 single fixture pack with SessionOverrideContract coverage of WU-0C-N1 fake adapter success/refusal, WU-0C-N3, WU-0C-N5 receipt propagation, and forbidden direct JSONL mutation fixtures outside WU-0C-N3. | r5 narrows the WU-0C-N3 phrase from "v1 idle-only write-back" to "schema-probe/safe-import gating" (lines 1980 contract, 2000 acceptance criterion); drops `WU-0C-N4` from cross-phase incoming (line 2008); appends `**Revision rationale:**` at line 2011. The N1/N3/N5 coverage closure, the forbidden-direct-JSONL closure, and the single-fixture-pack closure all hold. |
| WU-2-45 DetailRecordSchemaDto | R4-DECOMP-F05 single DTO; canonical TranscriptTurn binding via WU-0C-N2; provider-native JSONL/file-descriptor/SQLite-handle/parser-object inputs rejected. | r5 updates Source basis to "Phase 0C r5 WU-0C-N2" and Round 6 / r5 attribution (line 2073); cross-phase incoming gains no new ID and removes none (WU-0C-N2 stays; no WU-0C-N4 was ever on this WU); appends `**Revision rationale:**` at line 2101. The DTO field set, the three enums, and the rejection-fixture acceptance criteria are byte-identical to r4. |
| WU-2-46 TurnDecompositionService | R4-DECOMP-F06 single-method `decompose_turn_bundle`; canonical WU-0C-N1 write-back; WU-0C-N5 receipts propagated; direct provider JSONL operations forbidden. | r5 widens Source basis to include WU-0C-N3 alongside N1/N2/N5 (line 2118); drops `WU-0C-N4` from cross-phase incoming (line 2142); appends `**Revision rationale:**` at line 2147. The single-method contract, the conditional WU-0C-N1 delegation, and the WU-0C-N5 propagation closure all hold. No second method was introduced. |
| WU-2-47 DetailInjectionRouterService | R4-DECOMP-F07 single-method `route_details`; optional session-visible output via WU-0C-N1; WU-0C-N5 receipts/refusals returned; graph mutation remains proposal-only. | r5 widens Source basis to include WU-0C-N3 alongside N1/N2/N5 (line 2166); drops `WU-0C-N4` from cross-phase incoming (line 2190); appends `**Revision rationale:**` at line 2195. The single-method contract, the optional-write-back delegation, the WU-0C-N5 propagation closure, and the InjectionPlan action enum are byte-identical to r4. |

The D1 row for each touched WU keeps its r4 classification cell verbatim (Provider capability adapter, Fixture pack, DTO, Method-bearing service). No closure regression is possible without changing those cells, and the r5 diff does not change them.

**Recommendation:** No action required.

---

### R5-DECOMP-F04. r1/r2/r3/r4 closures preserved on the 42 r5-unaffected WUs

**Severity: NONE**

The audit r5 entry restricts r5 scope to the 8 r4-affected WUs plus Stitch Notes / Run Report references (`plans/audit/ai-roadmap-phase-2.md` lines 120–126). The 42 r5-unaffected WUs are: WU-2-01, WU-2-02, WU-2-03, WU-2-04, WU-2-05, WU-2-06, WU-2-07, WU-2-08, WU-2-09, WU-2-10, WU-2-11, WU-2-12, WU-2-13, WU-2-14, WU-2-15, WU-2-16, WU-2-17, WU-2-18, WU-2-19, WU-2-20, WU-2-21, WU-2-25, WU-2-27, WU-2-28, WU-2-29, WU-2-30, WU-2-31, WU-2-32, WU-2-33, WU-2-34, WU-2-35, WU-2-36, WU-2-37, WU-2-38, WU-2-39, WU-2-40, WU-2-41, WU-2-42, WU-2-44, WU-2-48, WU-2-49, WU-2-50.

Inspecting `git diff 335a161..ee68f91 -- product-strategy/ai-roadmap-phase-2.md` line by line, these 42 WUs receive zero hits in their `Contract:` / `Test boundary:` / `Code boundary:` / `Acceptance criteria:` / `Dependencies:` / `Produces:` / `Parallelizable with:` blocks. The only non-touched-WU diff hits are: (a) the file-scope boundary line at line 34 ("Included:" scope text) which was updated to reference proposal-r6 / engineering-roadmap-r5 / Phase 0C r5 WU-0C-N1..WU-0C-N3 plus WU-0C-N5 instead of r4 attribution; (b) the Run Report rows for D1/D3/D4 (lines 2417–2420), the D2-evidence bullet at line 2488, the round-suffix bullet at line 2489, and the Rule D3 / D4 prose at lines 2495–2498, 2507–2509, 2513 — each updated only to swap "Round 4" → "Round 5" and "WU-0C-N1..WU-0C-N5" → "WU-0C-N1..WU-0C-N3 plus WU-0C-N5"; (c) one Stitch-Notes prose line at 2623 (Incoming-From-Phase-0C narrative) re-stating the dependency set; (d) the deletion of `(WU-0C-N4, VS-010)` from the Stitch-Notes incoming list at line 2766 of the r4 file. None of these touch a WU body.

This means r1's leaf-DTO/enum closures (WU-2-01..06, WU-2-13), r2's wave-rederive preservation (state-machine WU-2-08, WU-2-14, WU-2-38), r3's six VS-010 WUs (WU-2-45..50 — five of which are r5-unaffected: WU-2-48/49/50 plus WU-2-45 stays single-DTO, only with a non-body Source-basis-text refresh), and r4's 42-CLEAN closure (R4-DECOMP-F09) all survive r5 verbatim. WU-2-13's r3 cross-CLI routing INFO placement deviation is byte-identical in r5. WU-2-14's 12-state lifecycle / 11-valid / 6-invalid transition closure (state-machine-criteria-family LOW-MEDIUM) is byte-identical in r5. WU-2-08 and WU-2-38 transition criteria are intact.

`bundling-family`, `state-machine-criteria-family`, and `dependency-encoding-family` do not re-fire on the 42 unaffected WUs because their bodies are unchanged.

**Recommendation:** No action required.

---

### R5-DECOMP-F05. Blocked-on annotations fully removed; no residual blocking syntax

**Severity: NONE**

`grep -E "Blocked-on|Blocked on" product-strategy/ai-roadmap-phase-2.md` returns zero hits across the entire file (`output_mode: count` = 0). The r5 commit message and audit r5 entry both attest the removal, and the diff confirms it: every line that previously carried a "Blocked-on:" annotation has been replaced with a blank line or a `**Revision rationale:**` stamp recording the removal context. The 8 r4-affected WUs each carry one such stamp:

> **Revision rationale:** r4 SessionOverrideContract refactor criteria retained; r5 cascade: agent-runner feature requests have landed; v2-only; block-on annotations removed.

No WU body, Stitch-Notes block, Parallelization-Map note, or Run-Report row retains a "Blocked-on" or "Blocked on" string. The Outgoing-to-Phase-3+ Stitch-Notes block at the bottom of the file (lines preceding 2956) carries no block-on annotations on its (WU-2-43 → VS-012), (WU-2-43 → VS-018), (WU-2-46 → VS-012), (WU-2-47 → VS-012), or (WU-2-46/47 → VS-018) edges. The annotation removal is observable as a clean, non-destructive cleanup: it removes only the temporary block markers that were valid while the upstream agent-runner cascade was open and that the audit r5 entry explicitly authorizes removing now that the cascade has shipped.

`session-override-boundary-family` (the watch signal flagged in r4) stays LOW: every touched WU's contract or acceptance criteria still explicitly forbid direct provider CLI command, resume composition, session-id capture, session-row mapping, thread storage, or JSONL mutation; all such operations are still routed through WU-0C-18 (AgentRunnerClient) for VS-009 launch/resume and through WU-0C-N1 / N2 / N3 / N5 (SessionOverrideContract) for VS-010 transcript write-back. The removal of block-on syntax is purely annotational and changes no boundary.

**Recommendation:** No action required.

---

### R5-DECOMP-F06. WU-0C-N4 references absent from Phase 2 r5; schema-probe coverage routes through WU-0C-N3

**Severity: NONE**

`grep "WU-0C-N4" product-strategy/ai-roadmap-phase-2.md` returns zero hits across the entire file (`output_mode: count` = 0). The r5 commit removes WU-0C-N4 from four locations: (1) WU-2-43 cross-phase incoming list at line 2008; (2) WU-2-46 cross-phase incoming list at line 2142; (3) WU-2-47 cross-phase incoming list at line 2190; (4) Stitch-Notes Incoming-From-Phase-0C row `(WU-0C-N4, VS-010)`. No other references remained. The Phase 0C r5 cascade dropped WU-0C-N4 (the original schema-probe split WU) and rolled its scope into WU-0C-N3; the Phase 2 r5 edit-pass faithfully follows that retirement.

Schema-probe coverage for VS-010 is now satisfied through WU-0C-N3 only:

- WU-2-43 contract block (line 1980): "WU-0C-N3 schema-probe/safe-import gating" replaces the prior "WU-0C-N3 v1 idle-only write-back" phrasing — a narrowing that aligns the fixture pack to the consolidated WU-0C-N3 surface.
- WU-2-43 acceptance criterion (line 2000): mirrors the contract phrasing exactly; fixture coverage explicitly enumerates WU-0C-N1 fake adapter success/refusal, WU-0C-N3 schema-probe/safe-import gating, preimage mismatch, session busy, unsupported storage, and WU-0C-N5 override receipt propagation.
- WU-2-46 / WU-2-47 Source basis: "Phase 0C r5 WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5" — N3 is now an explicit upstream alongside N1/N2/N5 (whereas r4 had only N1/N2/N5 with N3 implicit through WU-2-43).

D1 single-object granularity is unaffected: the dropped N4 was a Phase 0C-internal split, never a Phase 2-internal split. No Phase 2 D1 row referenced WU-0C-N4, so its retirement causes no Phase 2 ownership-table change. No Phase 2 internal edge depended on WU-0C-N4. `dependency-encoding-family` stays LOW — the targeted four-location removal plus the N3-narrowing prose update are exactly the prescribed scope and nothing more.

**Recommendation:** No action required.

---

### R5-DECOMP-F07. fix-created-family classification is generation 0; no spurious WU additions or removals

**Severity: NONE**

Round 5 is `fix-created-family` generation 0 in the Phase 2 local loop, externally driven by the proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 cascade — the same shape as r4's cascade-driven brownfield (which was also fix-created-family gen 0 from the Phase 0C-r4 SessionOverrideContract introduction cascade). The Run Report self-classifies r5 as "externally-driven minor edit-pass" at line 2513. The Phase 2 audit history at lines 120–126 explicitly frames r5 this way: "Phase 2 r5 is externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 after the agent-runner session feature requests landed."

Generation-counter check: r4 closed clean (R4-DECOMP-F01..F10 were all NONE; r4 self-attested no same-family reviewer-finding chain). r5 is not continuing an r4 finding-chain — it is responding to a *different* downstream event (the agent-runner team shipping the five feature requests, which cleared the block-on annotations and consolidated WU-0C-N4 into WU-0C-N3). Per the watch posture, generation-counter advancement requires a same-family reviewer-flagged r2 → r3 → r4 → r5 chain; that chain doesn't exist for any Phase 2-internal family.

**No spurious additions or removals.** The touched-WU set (WU-2-22, WU-2-23, WU-2-24, WU-2-26, WU-2-43, WU-2-45, WU-2-46, WU-2-47) matches the audit r5 entry scope exactly. Zero WUs were created or deleted in r5 (count stays at 50 vs. r4's 50). Each touched WU's r5 edit traces to a specific audit-r5-entry instruction — block-on removal, WU-0C-N4 retirement, cascade note — and to specific upstream artifacts (proposal-r6, engineering-roadmap-r5, Phase 0C-r5). The N3-narrowing on WU-2-43 contract / acceptance criterion traces to the Phase 0C r5 consolidation of N4 into N3.

`session-override-boundary-family` is observable on the surface as LOW: no touched WU language re-introduces direct CLI launch/resume, JSONL mutation, or session-id capture; every such concern is still explicitly forbidden by the r4 acceptance criteria preserved verbatim in r5 (F03, F05).

**Recommendation:** No action required.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R5-DECOMP-F01 | WU count = 50 unchanged; D1 ownership table 50 rows byte-identical; 11-wave Parallelization Map (13+8+5+5+5+4+4+2+2+1+1) byte-identical to r4. | NONE |
| R5-DECOMP-F02 | Per-schema-object ownership preserved on every D1 row; no blob WUs; bundling-family LOW. | NONE |
| R5-DECOMP-F03 | r4 closures (R4-DECOMP-F02..F08) preserved on the 8 r4-affected WUs (WU-2-22/23/24/26/43/45/46/47); contract / test boundary / acceptance criteria byte-identical except for targeted N4→N3 swap and appended cascade note. | NONE |
| R5-DECOMP-F04 | r1/r2/r3/r4 closures preserved on the 42 r5-unaffected WUs (WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48..50); bodies receive zero diff hits. | NONE |
| R5-DECOMP-F05 | `Blocked-on` / `Blocked on` annotations fully removed; `grep` returns zero hits; SessionOverrideContract boundary acceptance criteria preserved. | NONE |
| R5-DECOMP-F06 | `WU-0C-N4` references absent across the entire file; schema-probe coverage routes through WU-0C-N3 (narrowed phrase "schema-probe/safe-import gating" on WU-2-43; N3 added to Source basis on WU-2-46/47). | NONE |
| R5-DECOMP-F07 | r5 is fix-created-family gen 0 in the Phase 2 local loop, externally driven; touched-WU set matches audit r5 scope exactly; no spurious WU additions or removals. | NONE |

Oscillation classification: r5 is a `fix-created-family` round at generation 0, externally driven by the Phase 0C-r5 / proposal-r6 / engineering-roadmap-r5 cascade (agent-runner shipped). No same-family reviewer chain advances. `bundling-family`, `state-machine-criteria-family`, `dependency-encoding-family`, and `parallelization-map-family` all remain LOW. `session-override-boundary-family` remains LOW: every touched WU's r4 boundary acceptance criterion (no direct provider CLI command, resume composition, session-id capture, session-row mapping, thread storage, or JSONL mutation; all such operations routed through WU-0C-18 for VS-009 and through WU-0C-N1/N2/N3/N5 for VS-010) is preserved verbatim in r5.

## What LOW requires

- WU count is 50, unchanged from r4 (`### WU-2-` count = 50). VS-008 = 12, VS-009 = 16, VS-010 = 22. No WU added; no WU removed.
- D1 ownership table at lines 2426–2477 contains 50 single-object rows, byte-identical to r4 — Pack/Unpack/Focus/Pin/Unpin command DTOs, NavigationToolResultDto, AgentWalkStateMutationService, WalkStateTransitionValidator, RenderInvalidationService, NavigationToolCallAuditEmitter, NavigationToolAffordanceAdapter, NavigationFixturePack, TurnLifecycleEventEnum, OrchestratorTurnStateMachine, TurnRenderPreparationService, OrchestratorBridgeService, TurnCaptureCommitTransactionService, AdvisoryOptimizerRequestEmitter, GraphActionTransactionService, CompactDetectionAvoidanceGuard, ParentInvocationPropagator, ClaudeTurnAdapter, CodexTurnAdapter, OpencodeTurnAdapter, TurnUIPaneSurface, OrchestratorTurnFixturePack, TurnLifecycleAuditEmitter, TurnProviderPreflightAdapter, OptimizerScopingRequestDto, OptimizerScopingService, OptimizerPromptSchemaDto, OptimizerResponseSchemaDto, SummaryRefreshRequestDto, OptimizerModelInvocationAdapter, DeterministicOptimizerEditValidator, OptimizerEditMergeService, ConflictOnStaleBaseClassifier, SummaryNodeStaleStateTransitionHandler, BackendStaleSignalEmitter, SummaryRegenerationUIComponent, StaleMarkerUIComponent, OptimizerLogUI, SummaryRefreshFixturePack, OptimizerEditAuditEmitter, DetailRecordSchemaDto, TurnDecompositionService, DetailInjectionRouterService, IncrementalSummaryUpdateService, FullSummaryRegenerationService, StaleMarkDetectionService.
- WU-2-22 ClaudeTurnAdapter, WU-2-23 CodexTurnAdapter, WU-2-24 OpencodeTurnAdapter retain the r4 single-method `request_turn(prepared_turn, agent_runner_session_evidence) -> CliAcceptedSession` contract over WU-0C-18 AgentRunnerClient; bodies are byte-identical to r4 except for one inserted `**Revision rationale:**` line per WU.
- WU-2-26 OrchestratorTurnFixturePack retains the r4 fixture surface (happy path, tool path, blocked render, compact detected, failed capture, recovered turn, optimizer request) plus AgentRunnerClient routing assertions; body byte-identical to r4 except one inserted `**Revision rationale:**` line.
- WU-2-43 SummaryRefreshFixturePack retains the r4 SessionOverrideContract coverage with the WU-0C-N3 phrase narrowed from "v1 idle-only write-back" to "schema-probe/safe-import gating" (contract block and matching acceptance criterion); cross-phase incoming drops `WU-0C-N4` while keeping `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5`; one inserted `**Revision rationale:**` line.
- WU-2-45 DetailRecordSchemaDto retains the single-DTO surface; Source basis updated to "proposal.md Round 6 §1; engineering-roadmap.md r5 VS-010; Phase 0C r5 WU-0C-N2"; cross-phase incoming unchanged (WU-0C-N4 was never present); one inserted `**Revision rationale:**` line.
- WU-2-46 TurnDecompositionService retains the single-method `decompose_turn_bundle(...)` contract; Source basis adds WU-0C-N3 alongside N1/N2/N5; cross-phase incoming drops `WU-0C-N4`; one inserted `**Revision rationale:**` line.
- WU-2-47 DetailInjectionRouterService retains the single-method `route_details(...)` contract; Source basis adds WU-0C-N3 alongside N1/N2/N5; cross-phase incoming drops `WU-0C-N4`; one inserted `**Revision rationale:**` line.
- WU-2-01..21, WU-2-25, WU-2-27..42, WU-2-44, WU-2-48, WU-2-49, WU-2-50 (42 WUs) remain byte-for-byte identical to the r4 baseline in their `Contract:`, `Test boundary:`, `Code boundary:`, `Acceptance criteria:`, `Dependencies:`, `Produces:`, and `Parallelizable with:` blocks.
- The 11-wave Parallelization Map (Wave 1: 13 WUs through Wave 11: 1 WU; sum = 50) is byte-identical to r4; no Phase 2 internal edges added or removed.
- Stitch Notes Incoming-From-Phase-0C block drops one row (`(WU-0C-N4, VS-010)`); the prose line at 2623 is updated to "Round 5 keeps WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5 only where SessionOverrideContract write-back, DTO evidence, schema-probe gating, fixtures, receipts, or refusals are consumed: WU-2-43, WU-2-45, WU-2-46, and WU-2-47. WU-2-48, WU-2-49, and WU-2-50 remain clean." Outgoing-to-Phase-3+ block is byte-identical to r4 (no edge added or removed).
- The Run Report's Rule D4 watch-signal table self-classifies r5 as fix-created-family generation 0, externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5; `dependency-encoding-family` LOW after the targeted WU-0C-N4 retirement; `parallelization-map-family` LOW with unchanged 11-wave topology; `bundling-family` LOW; `state-machine-criteria-family` LOW (WU-2-14 unchanged); `session-override-boundary-family` LOW given preserved boundary acceptance criteria on every touched WU.
- `grep -E "Blocked-on|Blocked on" product-strategy/ai-roadmap-phase-2.md` returns zero hits.
- `grep "WU-0C-N4" product-strategy/ai-roadmap-phase-2.md` returns zero hits.
- `grep "Per-CLI adapter\|launch_or_resume\|per-CLI turn adapter" product-strategy/ai-roadmap-phase-2.md` continues to return zero hits (r4 closure preserved).
