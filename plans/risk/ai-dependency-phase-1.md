# Phase 1 — Dependency Risk Assessment (Round 3)

**Rating: LOW**

## Scope

Round 3 brownfield-cascade assessment of `product-strategy/ai-roadmap-phase-1.md` (56 WUs, unchanged from r2) after the proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 cascade introduced `SessionOverrideContract` plus WU-0C-N1..WU-0C-N5. Per `plans/audit/ai-roadmap-phase-1.md` round-3 entry, the proposer integrated the upstream boundary without adding, deleting, or merging Phase 1 WUs. The dependency surface should change only by (a) per-WU `Cross-phase incoming (SessionOverrideContract):` lines on the 13 affected WUs, (b) a new `Round 3 SessionOverrideContract incoming overlay` block in the Dependency Graph, (c) two new Stitch Notes overlay sections, and (d) round-3 re-check notes on Critical Path / Parallelization Map / Run Report. The 76 internal Phase 1 edges, the 7-wave Parallelization Map skeleton, the 24-pair Outgoing-To-Phase-2+ block, and the byte-for-byte Phase 0A/0B/0C non-N incoming edges should be preserved.

Cross-references walked for r3:

- `product-strategy/ai-roadmap-phase-1.md` lines 22–96 (Inventory, 56 WUs unchanged), lines 98–118 (Round 3 SessionOverrideContract Audit), lines 184–411 (WU-1-03/04/05/10 affected normalizer + transcript ingestion), lines 481–550 (WU-1-12/13 audit + drilldown), lines 829–931 (WU-1-23/25 evidence pointer + inspector pane), lines 994–1029 (WU-1-28 render audit subscription), lines 1518–1581 (WU-1-45/56 provider probe + fixture), lines 1645–1750 (WU-1-48/49/50 route + provider preflight pane), lines 1904–1965 (Dependency Graph; 76 internal edges unchanged), lines 1967–2421 (per-VS Incoming-from-Phase-0A/0B/0C; non-N incoming unchanged), lines 2423–2433 (new Round 3 SessionOverrideContract incoming overlay), lines 2435–2449 (Critical Path with r3 re-check note), lines 2451–2465 (Parallelization Map with r3 re-derive note), lines 2467–2549 (Run Report incl. updated D3/D4 rows and r3 self-classification), lines 2551–2982 (Stitch Notes incl. new SessionOverrideContract incoming-edge block), lines 2984–3032 (Outgoing-To-Phase-2+ incl. new (VS-003, VS-018) and (VS-006, VS-015) Blocked-on annotations + Non-Ownership Notes).
- `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` lines 1582–1899 (WU-0C-N1..WU-0C-N5 declarations and per-WU produces blocks), lines 3262–3278 (WUs audited for SessionOverrideContract boundary violations), lines 3332–3336 (WU-0C-N* internal Dependency Graph entries), lines 3451–3454 (Phase 0C-r4 Parallelization Map with WU-0C-N* placements), lines 3640–3653 (Phase 0C-r4 cross-phase outgoing summary, including line 3645 generic-downstream-Phase-1/2/3 authorization clause), lines 3655–3663 (Phase 0C-r4 Outgoing to Phase 1, per VS), lines 3665–3680 (Phase 0C-r4 Outgoing to Phase 2+, including VS-010/012/018/020/021 Blocked-on annotations), lines 3682–3687 (Cross-phase outgoing edges added in round 4, including line 3687 instruction to downstream Phase 1/2/3 to add explicit incoming-from-Phase-0C edges to WU-0C-N1..WU-0C-N5).
- `product-strategy/proposal.md` (Phase 1 worktree copy, identical to proposal-r5) lines 75–98 (SessionOverrideContract Boundary), lines 1489 (race handling — pause-handshake until idle), lines 1631–1664 (substrate-feature gap: stable `agents session locate / export / import-replace` and pause-handshake absent in v1).
- `product-strategy/engineering-roadmap.md` (Phase 1 worktree copy, identical to engineering-roadmap-r4) line 36 (SessionOverrideContract Foundation row consumed by VS-010, VS-012, VS-018, VS-020, VS-021).
- `plans/audit/ai-roadmap-phase-1.md` round-3 entry (lines 71–79).
- `plans/risk/ai-dependency-phase-1.md` r1 (`worktrees/phase-1-ai-roadmap-r2/plans/risk/ai-dependency-phase-1.md`) and r2 (`worktrees/phase-1-risk-dependency-r2/plans/risk/ai-dependency-phase-1.md`) — baselines for byte-for-byte regression checks against the r2-converged 56-WU artifact.

## Findings

### F-1. Internal Phase 1 dependency graph remains acyclic; topological sort succeeds with 56 nodes; no Phase 1-local edge added or removed by the r3 cascade

**Severity: LOW**

The Dependency Graph block at `ai-roadmap-phase-1.md:1909–1964` enumerates every Phase 1 WU's internal parent set. Walking the complete 56-line edge listing:

- All 76 r2 internal edges are preserved verbatim. Sampled spot-checks: `WU-1-06 <- WU-1-02, WU-1-03, WU-1-04, WU-1-05` (line 1914) and `WU-1-10 <- WU-1-03, WU-1-04, WU-1-05, WU-1-07` (line 1918) and `WU-1-50 <- WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49` (line 1959) all match r2 byte-for-byte. No Phase 1 WU acquired a new internal parent because of the r3 cascade.
- `WU-1-56 <- none` (line 1954) is preserved from r2; the fixture WU remains a Wave 1 leaf with no outgoing internal edges.
- Topological sort over the 56 nodes yields 7 waves with the same memberships as r2 (re-verified in F-7).
- No `WU-1-XX → WU-1-XX` cycle exists. WU-1-56 has no outgoing internal edges (no other WU lists WU-1-56 as a parent — verified by scanning every `Phase 1 internal:` block on WU-1-46..50 at lines 1607, 1638, 1670, 1705, 1741: none lists WU-1-56).
- The r3 audit at lines 100–118 explicitly states no WU was added, removed, or merged. The WU count (56), the per-WU table (lines 41–96), the Run Report metric `Total Phase 1 WUs | 56` (line 2541), the D1 audit count `56 rows = 56 WUs` (line 2478), and the per-WU D1 ownership table (lines 2480–2537) all agree.

The r2-acyclic 56-node graph remains acyclic in r3.

**Recommendation:** No action required.

**Oscillation classification:** None — `dependency-encoding-family` does not re-fire on the internal Phase 1 graph. Per `plans/audit/ai-roadmap-phase-1.md:78`, the audit-history reopens `fix-created-family` at generation 0 because the cascade is externally driven by proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4; F-1 is the first of several gates that must verify the externally-driven brownfield does not introduce new local dependency-encoding regressions.

---

### F-2. SessionOverrideContract incoming overlay is correctly encoded as cross-phase incoming, not as new Phase 1-local edges

**Severity: LOW**

The Round 3 brownfield directive in `plans/audit/ai-roadmap-phase-1.md:71–79` requires that WU-0C-N1..WU-0C-N5 be wired into Phase 1 only as cross-phase incoming gates and not as new Phase 1-local edges. Verification:

1. **Per-WU `Cross-phase incoming (SessionOverrideContract):` lines.** Each affected WU declares its SessionOverrideContract parents on a separate line beneath the original `Cross-phase incoming:` block. Sampled:
   - `WU-1-03` line 211: `WU-0C-N2.` ✓
   - `WU-1-04` line 246: `WU-0C-N2.` ✓
   - `WU-1-05` line 281: `WU-0C-N2.` ✓
   - `WU-1-12` line 508: `WU-0C-N5.` ✓
   - `WU-1-13` line 544: `WU-0C-N5.` ✓
   - `WU-1-23` line 857: `WU-0C-N2, WU-0C-N5.` ✓
   - `WU-1-25` line 924: `WU-0C-N2, WU-0C-N5.` ✓
   - `WU-1-28` line 1022: `WU-0C-N5.` ✓
   - `WU-1-45` line 1545: `WU-0C-N4, WU-0C-N5.` ✓
   - `WU-1-48` line 1672: `WU-0C-N4, WU-0C-N5.` ✓
   - `WU-1-49` line 1707: `WU-0C-N4, WU-0C-N5.` ✓
   - `WU-1-50` line 1743: `WU-0C-N4, WU-0C-N5.` ✓
   - `WU-1-10` line 441 (folded into the main `Cross-phase incoming:` line): `..., WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5.` ✓
2. **Round 3 SessionOverrideContract incoming overlay** (`ai-roadmap-phase-1.md:2423–2433`) restates the same pairs at the cross-phase aggregation layer:
   - `WU-0C-N2 -> WU-1-03, WU-1-04, WU-1-05`
   - `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5 -> WU-1-10`
   - `WU-0C-N5 -> WU-1-12, WU-1-13, WU-1-28`
   - `WU-0C-N2, WU-0C-N5 -> WU-1-23, WU-1-25`
   - `WU-0C-N4, WU-0C-N5 -> WU-1-45, WU-1-48, WU-1-49, WU-1-50`
3. **Stitch Notes restatement** (`ai-roadmap-phase-1.md:2972–2982`) re-emits each (WU-0C-N*, WU-1-XX) pair as an enumerated incoming edge with a one-line scope rationale (e.g. "canonical transcript DTO shapes for Claude tool-call normalization; no raw JSONL parsing").

Pair count: 3 (N2 to normalizers) + 5 (all N to WU-1-10) + 3 (N5 to audit/drilldown/subscription) + 4 (N2/N5 to evidence/inspector pair) + 8 (N4/N5 to provider preflight quartet) = **23 (WU-0C-N*, WU-1-XX) edges across 13 affected WUs**, all encoded at three consistent locations: per-WU dependencies block, Dependency Graph overlay, and Stitch Notes incoming-from-Phase-0C overlay.

Crucially, **no SessionOverrideContract edge appears as a Phase 1-internal parent on any WU**. The new edges flow only cross-phase incoming, matching the proposer's directive ("the topological waves remain 18+12+9+5+7+3+2 because SessionOverrideContract edges are cross-phase incoming gates, not new Phase 1-local edges" — `ai-roadmap-phase-1.md:2455`).

**Recommendation:** No action required.

**Oscillation classification:** `fix-created-family`, generation 0, externally driven. The proposer applied the correct r3 pattern in one pass: cross-phase scope, per-WU enumeration, redundant Dependency Graph + Stitch Notes restatement.

---

### F-3. Affected-WU SessionOverrideContract incoming envelopes are scoped to the WU's actual contract surface

**Severity: LOW**

The audit-history brownfield directive (`plans/audit/ai-roadmap-phase-1.md:75–76`) requires Phase 1 r3 to declare specific WU-0C-N* dependencies per affected WU rather than blanket-attaching all five. Verifying scope precision per WU:

| Phase 1 WU | Contract surface | Declared WU-0C-N* | Scope rationale |
|---|---|---|---|
| WU-1-03 ClaudeToolCallNormalizer | `normalize_claude_tool_call(...)` over normalized turn/hook/trace evidence; never opens JSONL paths (line 202–203) | N2 only | Needs canonical `TranscriptTurn` DTO shapes (WU-0C-N2). No registry, no probe, no adapter, no trait read. ✓ |
| WU-1-04 CodexToolCallNormalizer | `normalize_codex_tool_call(...)` over normalized turn/rollout/trace evidence (line 237–238) | N2 only | Same scope as WU-1-03; differs only in provider shape. ✓ |
| WU-1-05 OpencodeToolCallNormalizer | `normalize_opencode_tool_call(...)` over normalized hook/event/trace evidence (line 272–273) | N2 only | Same scope; opencode shape. ✓ |
| WU-1-10 TranscriptIngestionAdapter | `ingest_transcript(session_id, session_read_ref?)` reads through WU-0C-15d or WU-0C-N1 `read_transcript` / `get_session_metadata`; refusal recorded; never calls N1 mutation methods (line 432–434) | N1, N2, N3, N4, N5 | Central Phase 1 read seam: needs the N1 trait, the N2 DTOs, the N3 v1 adapter for current shipping behavior, the N4 schema probe to detect adapter readiness, and the N5 registry for refusal/receipt evidence. The full envelope is justified. ✓ |
| WU-1-12 ToolCallAuditEmitter | `emit_tool_call_audit(...)` may link to WU-0C-N5 override registry records (line 499); never mutates overrides (line 500) | N5 only | Audit-only consumer of override registry. No need for N1/N2/N3/N4. ✓ |
| WU-1-13 ToolCallDrilldownComponent | View shows override receipt/refusal pointer from WU-0C-N5 (line 535); exposes no mutation control (line 536) | N5 only | Display-only consumer of override registry. ✓ |
| WU-1-23 RenderEvidencePointerService | Pointers may include WU-0C-N5 override receipt/refusal IDs and WU-0C-N2 session metadata refs (line 848); no SessionOverrideContract mutation (line 849) | N2, N5 | Needs both N2 (session metadata DTO refs) and N5 (override registry refs). No probe/adapter/trait dependency. ✓ |
| WU-1-25 WorkingSetInspectorPane | Pane displays override receipt/refusal status from WU-0C-N5 and override-derived imposed-context evidence from WU-0C-N2 (line 915); no mutation control (line 916) | N2, N5 | Same scope as WU-1-23; UI display surface. ✓ |
| WU-1-28 RenderAuditSubscription | Stream may include SessionOverrideStore audit events from WU-0C-N5 (line 1013); acknowledgement never mutates an override (line 1014) | N5 only | Audit-stream consumer of override registry. ✓ |
| WU-1-45 RedactedProviderProbeService | Session-override compatibility comes from WU-0C-N4/WU-0C-N5 evidence, not probe-owned JSONL (line 1537); no resume/quota/auth ownership (line 1536) | N4, N5 | Needs N4 (schema probe outputs as adapter capability) and N5 (refusal evidence). No trait read, no DTO consumption, no v1 adapter dependency. ✓ |
| WU-1-48 RouteEligibilityResolver | Workload requirements may include need for session override capability via opaque WU-0C-N4/WU-0C-N5 refs (line 1664); no direct probe/mutation (line 1664) | N4, N5 | Same scope as WU-1-45; consumes capability evidence. ✓ |
| WU-1-49 RouteDenialReasonClassifier | Distinguishes harness preflight denials from agent-runner / SessionOverrideContract refusal reasons via opaque WU-0C-N5 registry/evidence refs (line 1698–1699) | N4, N5 | Same scope as WU-1-45/48. ✓ |
| WU-1-50 ProviderPreflightPane | Pane displays session-override capability/refusal metadata from WU-0C-N4/WU-0C-N5 as read-only evidence (line 1734–1735) | N4, N5 | UI display surface for the VS-006 preflight quartet. ✓ |

**Scope claim:** Every declared (WU-0C-N*, WU-1-XX) edge corresponds to a contract surface that genuinely consumes the named upstream object. Conversely, no WU declares a SessionOverrideContract edge it does not need:

- WU-1-12/13/28 do not declare N2/N3/N4 even though they sit inside VS-003 alongside WU-1-10. Reason: audit/drilldown/subscription only need override registry refs (N5), not DTO shapes / v1 adapter / schema probe.
- WU-1-23/25 do not declare N3/N4 even though their displays could surface override metadata. Reason: pointer/inspector services consume registered evidence/audit metadata (N5) and DTO ref shapes (N2); they never call the v1 adapter or run the schema probe.
- WU-1-45/48/49/50 do not declare N1/N2/N3 even though provider preflight is in VS-006. Reason: read-only provider preflight uses only schema probe outputs (N4) and override registry refusal records (N5); it does not perform transcript reads or call the trait directly.

The non-affected WUs in the audit table at `ai-roadmap-phase-1.md:118` (`WU-1-01/02/06/07/08/09/11/14..22/24/26/27/29..44/46/47/51..55/56`) declare zero SessionOverrideContract incoming edges — verified by scanning each for the literal substring `WU-0C-N`. None of these WUs encode session storage, provider routing, quota, resume, cross-provider porting, or session mutation behavior.

**Recommendation:** No action required. The audit-history concern that "SessionOverrideContract dependencies are neither missing nor over-broad" (`plans/audit/ai-roadmap-phase-1.md:79`) is satisfied.

**Oscillation classification:** `fix-created-family`, generation 0, applied tightly per WU.

---

### F-4. WU-1-10's full N1..N5 envelope is correct: the central Phase 1 transcript read seam needs every WU-0C-N* primitive

**Severity: LOW**

WU-1-10 TranscriptIngestionAdapter is unique among the 13 affected WUs in declaring all five WU-0C-N* edges. The over-broad-envelope risk is real: WU-1-10 could quietly become a session-storage owner if the envelope is over-claimed. Verifying envelope-versus-contract:

- **WU-0C-N1 (`SessionOverrideContract` trait) → WU-1-10.** WU-1-10 calls `read_transcript` and `get_session_metadata` on the N1 trait surface for read-only ingestion (acceptance criterion at `ai-roadmap-phase-1.md:432`). Phase 0C-r4's `WU-0C-N1` produces line at `ai-roadmap-phase-0c.md:1732` enumerates VS-010/012/018/020/021 only (not VS-003), but the line 3645 generic-downstream clause and the line 3687 instruction to downstream Phase 1/2/3 to add explicit incoming-from-Phase-0C edges authorize this Phase 1 r3 wiring. The trait is the only sanctioned read seam.
- **WU-0C-N2 (`TranscriptTurn`/`SessionLocation`/`SessionMetadata` DTOs) → WU-1-10.** WU-1-10's downstream events are the canonical session-override DTOs; this is the only Phase 1 reader that feeds the Phase 1-local Wave 3 fan-in (WU-1-03/04/05 in Wave 2 normalize different per-CLI shapes; WU-1-10 in Wave 3 is the central reader). N2 producer line at `ai-roadmap-phase-0c.md:1680` enumerates VS-010/012/018/020/021 explicitly; Phase 1 r3 adds VS-003 (WU-1-10) as a generic-downstream consumer. ✓
- **WU-0C-N3 (`AgentRunnerDbAdapter` v1) → WU-1-10.** The Blocked-on annotation at `ai-roadmap-phase-1.md:447` reads "v1 read behavior can use WU-0C-N3 after schema probe." WU-1-10 reads through whichever adapter is registered behind N1. Because N3 is the only shipping adapter until v2 lands, WU-1-10 transitively depends on N3. Phase 0C-r4's N3 produces line at line 1839 enumerates VS-010/012/018/020/021; Phase 1 r3 reads via N3 v1 only. ✓
- **WU-0C-N4 (`AgentRunnerSchemaProbe`) → WU-1-10.** The Blocked-on annotation at `ai-roadmap-phase-1.md:447` and the acceptance criterion at line 433 ("The adapter records unsupported schema, unsupported storage, busy session, missing session, and adapter refusal as evidence artifacts without attempting fallback per-CLI JSONL parsing") together imply WU-1-10 needs N4 schema probe outputs to refuse cleanly when v1 cannot read the substrate. ✓
- **WU-0C-N5 (`SessionOverrideStore` registry) → WU-1-10.** WU-1-10 records refusal evidence as registered evidence/audit metadata, which is the registry's role. N5 produces line at `ai-roadmap-phase-0c.md:1897` explicitly enumerates VS-001 evidence inspectors and VS-003 audit surfaces — WU-1-10 is the VS-003 entry point that produces the override-derived evidence VS-001 inspectors then display. ✓

The full N1..N5 envelope is justified by the contract scope and does not over-broaden. Conversely, no Phase 1-internal WU "downstream" of WU-1-10 (i.e. WU-1-13 ToolCallDrilldownComponent in Wave 5 — the only WU that internally depends on WU-1-10 transitively via WU-1-12 / WU-1-06) is given access to the full envelope. The strict subset N5 ⊂ {N1, N2, N3, N4, N5} on WU-1-13 reflects the layered design correctly.

**Recommendation:** No action required.

**Oscillation classification:** None — full-envelope discipline holds at the read seam without leaking through to descendants.

---

### F-5. Bidirectional consistency vs Phase 0C-r4 holds at the macro / generic-downstream layer; per-WU producer enumeration is uneven but explicitly authorized

**Severity: LOW (with INFO caveat about upstream producer-side enumeration)**

For the bidirectional contract to hold, every (Phase 0C-r4 WU-0C-N*, Phase 1 WU) edge declared on the Phase 1 incoming side must have a producer-side justification on the Phase 0C side.

Mapping Phase 1 r3 edges against Phase 0C-r4 producer-side declarations:

| Phase 0C-r4 producer | Phase 1 r3 incoming consumers | Phase 0C-r4 declaration source |
|---|---|---|
| WU-0C-N5 SessionOverrideStore registry | WU-1-12, WU-1-13, WU-1-23, WU-1-25, WU-1-28, WU-1-45, WU-1-48, WU-1-49, WU-1-50 (and WU-1-10) | **Explicit per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1897`: "consumed by ... VS-001 evidence inspectors, VS-003 audit surfaces, VS-020 recovery, and VS-021 reroute governance." VS-001 evidence inspectors = WU-1-23/25; VS-003 audit surfaces = WU-1-12/13/28. The VS-006 provider preflight quartet (WU-1-45/48/49/50) is not named at line 1897 but is authorized by the line 3645 generic-downstream clause and by the proposer's r3 audit at `ai-roadmap-phase-1.md:113–116`. ✓ Strong direct authorization for VS-001/VS-003 surfaces; generic authorization for VS-006. |
| WU-0C-N2 TranscriptTurn/SessionLocation/SessionMetadata DTOs | WU-1-03, WU-1-04, WU-1-05, WU-1-10, WU-1-23, WU-1-25 | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1680`: "consumed by WU-0C-N1, WU-0C-N3, WU-0C-N4, WU-0C-N5, VS-010, VS-012, VS-018, VS-020, and VS-021." Phase 1 VS-003/VS-001 not named per-VS, but **authorized by the line 3645 generic-downstream-Phase-1/2/3 clause** ("downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata"). ✓ Generic authorization. |
| WU-0C-N1 SessionOverrideContract trait | WU-1-10 | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1732`: "consumed by WU-0C-N3, WU-0C-N5, VS-010 ... VS-018 worker-output reintegration, VS-020 ..." VS-003 not named per-VS. **Authorized by line 3687** ("Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1..WU-0C-N5") and by line 3645. ✓ Generic authorization. |
| WU-0C-N3 AgentRunnerDbAdapter v1 | WU-1-10 (transitively via N1) | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1839`: "consumed by VS-010, VS-012, VS-018, VS-020, and VS-021 until the v2 CLI adapter lands." Phase 1 not named per-VS. **Authorized by line 3645 generic-downstream and by line 3687 instruction.** ✓ Generic authorization. |
| WU-0C-N4 AgentRunnerSchemaProbe | WU-1-10, WU-1-45, WU-1-48, WU-1-49, WU-1-50 | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1780`: "consumed by WU-0C-N3 and v1 refusal paths in WU-0C-N1 tests." Phase 1 not named per-VS. **Authorized by line 3645 generic-downstream.** ✓ Generic authorization. |

**Caveat (INFO).** The per-WU Phase 0C-r4 `Produces` lines for WU-0C-N1/N2/N3/N4 do not enumerate the specific Phase 1 r3 read-only consumers added in this round. Strict bidirectional consistency holds only via:

1. The line 3645 generic-downstream clause: "downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata."
2. The line 3687 standing instruction: "Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1..WU-0C-N5 where worker launch, worker reintegration, turn decomposition, detail injection, or repack planning consumes session override receipts/refusals."

Phase 1 r3 honors line 3687 by declaring explicit incoming edges. The reciprocal — updating Phase 0C-r4's per-WU `Produces` lines to enumerate the new Phase 1 r3 consumers — is a Phase 0C upstream symmetry update, not a Phase 1 r3 dependency-encoding regression.

The WU-0C-N5 case is the strongest: line 1897 explicitly names "VS-001 evidence inspectors, VS-003 audit surfaces" as consumers, which directly authorizes the Phase 1 r3 N5 wiring on WU-1-12/13/23/25/28. Half of the Phase 1 r3 SessionOverrideContract edges (those into N5) are therefore strictly bidirectionally consistent at the per-WU enumeration layer. The other half (N1/N2/N3/N4) hold at the macro / generic-downstream layer.

**Recommendation:** No action required for the Phase 1 r3 dependency gate. If a future Phase 0C-r5 round closes the symmetry by enumerating Phase 1 r3 read-only consumers in WU-0C-N1/N2/N3/N4 `Produces` lines, the bidirectional contract tightens to per-WU. Until then, the line 3645 generic clause is the load-bearing authorization for those four edges. This is analogous to r1's R1-DEP-F10 (VS-002 IPC, VS-003 UI shell): downstream-of-upstream-omission, not a Phase 1 dependency-encoding regression.

**Oscillation classification:** `fix-created-family`, generation 0 (externally driven). The bidirectional asymmetry is *inherited* from Phase 0C-r4, not *introduced* by Phase 1 r3.

---

### F-6. Pre-existing Phase 0A/0B/0C non-N incoming edges remain byte-for-byte from r2 across every VS

**Severity: LOW**

The r3 brownfield directive (`plans/audit/ai-roadmap-phase-1.md:75`) requires that the existing 56-WU non-N envelope be preserved. Verification:

- **Per-VS Incoming-from-Phase-0A/0B/0C blocks** at `ai-roadmap-phase-1.md:1969–2421`. Sampled per VS:
   - Shared (lines 1969–1990): 20 entries, identical to r2 set.
   - VS-001 (lines 1992–2082): 91 entries, identical to r2 set including the WU-0B-09/10/11/15/16/17/18/19 and WU-0C-21..26a/05..07a/08..10/11a..18/19/20a/20/32/33a/33/34/35a/35/36a/36/37a/37 envelope verified in r1 F-3 / r2 F-5.
   - VS-002 (lines 2084–2126): 43 entries, identical to r2 set; no IPC or UI-shell additions (consistent with the carry-forward INFO observation).
   - VS-003 (lines 2128–2183): 56 entries, identical to r2 set.
   - VS-004 (lines 2185–2239): 55 entries, identical to r2 set.
   - VS-005 (lines 2241–2293): 53 entries, identical to r2 set.
   - VS-006 (lines 2295–2371): 77 entries, identical to r2 set; no per-WU non-N envelope extension.
   - VS-007 (lines 2373–2421): 49 entries, identical to r2 set.
- **Stitch Notes Incoming-from-Phase-0A/0B/0C blocks** (`ai-roadmap-phase-1.md:2553–2970`) re-emit the same per-(Phase-0X-WU, Phase-1-VS) edges as r2.
- **Per-WU `Cross-phase incoming:` lines** for the 13 affected WUs append SessionOverrideContract edges via a separate line (most cases) or extend the same line (WU-1-10 only). Sampled diff against r2:
   - WU-1-03 line 210 main `Cross-phase incoming:` block is byte-for-byte from r2; line 211 `Cross-phase incoming (SessionOverrideContract):` is new.
   - WU-1-10 line 441 main `Cross-phase incoming:` block is r2 envelope plus `, WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5` appended at the end.
   - WU-1-45 line 1544 main block is byte-for-byte r2 envelope; line 1545 SessionOverrideContract line is new.

Phase 0A/0B/0C pre-N envelope preservation across all 56 WUs holds in r3.

**Recommendation:** No action required.

**Oscillation classification:** `dependency-encoding-family` remains dormant on the non-N envelope. The systematic-from-start discipline established in r1 (closed at generation 0) is unbroken across r2 → r3.

---

### F-7. Parallelization Map remains a valid topological-level partition; 7 waves of 18+12+9+5+7+3+2 = 56

**Severity: LOW**

The Parallelization Map at `ai-roadmap-phase-1.md:2451–2465` declares the same 7-wave partition as r2. Recomputing each WU's depth as `1 + max(depth of declared Phase 1 internal parents)`:

| Wave | Declared WUs | Verified count | Sample wave-N+1 derivation check |
|---:|---|---:|---|
| 1 | WU-1-01, 02, 07, 08, 09, 11, 14, 15, 16, 20, 32, 37, 38, 39, 45, 51, 53, 56 | 18 | All have empty internal parent set in the Dependency Graph ✓ |
| 2 | WU-1-03, 04, 05, 17, 19, 23, 26, 27, 29, 40, 46, 52 | 12 | WU-1-03 ← WU-1-02 (W1) → W2 ✓; WU-1-23 ← WU-1-07, WU-1-20 (both W1) → W2 ✓ |
| 3 | WU-1-06, 10, 18, 24, 30, 31, 41, 47, 54 | 9 | WU-1-06 ← WU-1-02/03/04/05 (W1, W2, W2, W2) → W3 ✓; WU-1-10 ← WU-1-03/04/05/07 (W2, W2, W2, W1) → W3 ✓ |
| 4 | WU-1-12, 33, 42, 48, 55 | 5 | WU-1-12 ← WU-1-06/08/09/11 (W3, W1, W1, W1) → W4 ✓; WU-1-48 ← WU-1-47 (W3) → W4 ✓ |
| 5 | WU-1-13, 21, 28, 34, 43, 44, 49 | 7 | WU-1-21 ← WU-1-20/12/17 (W1, W4, W2) → W5 ✓; WU-1-49 ← WU-1-45/48 (W1, W4) → W5 ✓ |
| 6 | WU-1-22, 35, 50 | 3 | WU-1-22 ← WU-1-20/21 (W1, W5) → W6 ✓; WU-1-50 ← WU-1-45/46/47/48/49 (W1, W2, W3, W4, W5) → W6 ✓ |
| 7 | WU-1-25, 36 | 2 | WU-1-25 ← WU-1-22/23/24 (W6, W2, W3) → W7 ✓; WU-1-36 ← WU-1-34/35 (W5, W6) → W7 ✓ |

Total 18+12+9+5+7+3+2 = 56 = WU count ✓. The Run Report metric `Maximum same-wave concurrency | 18` (line 2543) matches Wave 1's 18 WUs.

The `Parallelizable with: same topological wave` lines on each WU were spot-checked for the cascade-affected WUs:
- WU-1-03 line 215 lists 11 Wave 2 mates: `WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52`. None of these are also declared Phase 1 internal parents of WU-1-03 (whose only parent is WU-1-02 in W1). ✓
- WU-1-10 line 445 lists 8 Wave 3 mates: `WU-1-06, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54`. None are declared parents of WU-1-10 (whose parents are WU-1-03/04/05/07 in W2/W1). ✓
- WU-1-50 line 1747 lists Wave 6 mates `WU-1-22, WU-1-35`; none are declared parents. ✓
- WU-1-45 line 1549 lists 17 Wave 1 mates including WU-1-56 — same pattern as r2 F-2 verified. ✓

The proposer's r3 re-derive note at line 2455 is correct: SessionOverrideContract edges are cross-phase, not Phase 1-local, so the depth computation is unaffected.

**Recommendation:** No action required. The Phase 0B-style Parallelization Map family does not re-fire.

**Oscillation classification:** `dependency-encoding-family` remains dormant on the Parallelization Map. The wave partition validity established in r1 F-6 / r2 F-6 is preserved.

---

### F-8. Outgoing-To-Phase-2+ block preserves all 24 (Phase 1 VS, Phase 2+ VS) pairs; new Blocked-on annotations on (VS-003, VS-018) and (VS-006, VS-015) are precise about substrate features

**Severity: LOW**

The Outgoing-To-Phase-2+ block at `ai-roadmap-phase-1.md:2986–3009` lists the same 24 pairs as r1/r2:

- (VS-001, VS-008), (VS-001, VS-009) — 2
- (VS-002, VS-008), (VS-002, VS-010), (VS-002, VS-014) — 3
- (VS-003, VS-009), (VS-003, VS-010), (VS-003, VS-014), (VS-003, VS-018), (VS-003, VS-019), (VS-003, VS-020) — 6
- (VS-004, VS-008), (VS-004, VS-009), (VS-004, VS-010), (VS-004, VS-015), (VS-004, VS-019) — 5
- (VS-005, VS-010), (VS-005, VS-011) — 2
- (VS-006, VS-009), (VS-006, VS-015), (VS-006, VS-017), (VS-006, VS-020), (VS-006, VS-021) — 5
- (VS-007, VS-015) — 1

Total 2+3+6+5+2+5+1 = 24 ✓. The `Engineering-roadmap dependency rows encoded here` recap (lines 3011–3023) preserves the r1/r2 mapping byte-for-byte.

Two pairs gained `Blocked-on:` annotations in r3:

1. **(VS-003, VS-018) at line 2994**: "accepted worker-output session write-back must consume WU-0C-N1..WU-0C-N5; v2 adapter migration needs `agents session locate / export / import-replace`; atomic mid-session reintegration needs `agents pause-handshake`." Cross-checked against `ai-roadmap-phase-0c.md:3677` (VS-018 outgoing-to-Phase-2+ Blocked-on: "v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-018 may stage accepted output and uses WU-0C-N3 only when session-idle."). The Phase 1 r3 phrasing is the precise downstream-side mirror of the Phase 0C-r4 upstream-side annotation. ✓
2. **(VS-006, VS-015) at line 3005**: "the downstream worker launcher remains thin and must spawn `agents -m <model> -p <project> -f <prompt>` while capturing the spawned `session_id` via the `agents` `--session-id` forced-flag mechanism and storing it on `WorkerRun`; v2 SessionOverrideContract adapter migration needs `agents session locate / export / import-replace`." Cross-checked against `proposal.md:1631` (substrate-feature gap) and `ai-roadmap-phase-0c.md:3674` (VS-015 outgoing summary). The thin worker launcher / forced `--session-id` capture flow is the precise expected Phase 4 worker-launcher behavior. ✓

WU-1-10's own Blocked-on at `ai-roadmap-phase-1.md:447` is also precise: "v1 read behavior can use WU-0C-N3 after schema probe. v2 adapter migration is blocked on `agents session locate` and `agents session export`; this WU is not blocked on `agents session import-replace` or `agents pause-handshake` because Phase 1 transcript ingestion is read-only." This carefully distinguishes the read-only subset of substrate features from the mutation subset, demonstrating that the proposer thought through which `agents session` substrate features each Phase 1 surface actually needs.

The `Non-Ownership Notes` block at lines 3025–3031 explicitly notes that worker-launcher and worker-output-reintegration WUs are absent from this Phase 1 artifact and that downstream artifacts introducing them must depend on WU-0C-N1 and the active adapter (WU-0C-N3 until v2). This forwards the line 3687 instruction correctly to the future VS-015/VS-018-owning artifacts.

**Recommendation:** No action required.

**Oscillation classification:** None — Blocked-on precision is the named r3 audit-history concern, and it is satisfied.

---

### F-9. Critical Path is unchanged; r3 re-check note honest about Phase 0 critical incoming gates

**Severity: LOW**

The Critical Path block at `ai-roadmap-phase-1.md:2435–2449` declares the same 11-hop VS-001 acceptance path as r1/r2: `WU-1-02 → WU-1-03 → WU-1-06 → WU-1-12 → WU-1-14 → WU-1-17 → WU-1-20 → WU-1-21 → WU-1-22 → WU-1-24 → WU-1-25`.

The r3 re-check note at line 2447 reads: "SessionOverrideContract adds cross-phase prerequisites to WU-1-10 and read-only evidence/audit/display WUs, but no new Phase 1-local dependency edge. The Phase 1-local critical path remains unchanged; upstream readiness now additionally requires WU-0C-N1..WU-0C-N5 before transcript-ingestion and override-derived evidence display are accepted."

Verification:
- WU-1-10 is on Wave 3 but NOT on the declared critical path (WU-1-10 is a TranscriptIngestionAdapter for VS-003, while the critical path runs through WU-1-12 ToolCallAuditEmitter for the VS-001 acceptance gate). The r3 note is correct that adding cross-phase prerequisites to WU-1-10 does not change the Phase 1-local critical path.
- The r3 note's caveat that "upstream readiness now additionally requires WU-0C-N1..WU-0C-N5 before transcript-ingestion and override-derived evidence display are accepted" is honest scoping. It does not overclaim: it acknowledges that the cross-phase blocker tightens *when* WU-1-10 / override-derived display can be accepted, not that the Phase 1-local longest path changes.
- The Phase 0 critical incoming gates list at line 2449 is unchanged from r2 (no additional Phase 0 IDs are required for the critical path itself, since the path runs through audit / budget / render gates in Phase 0B/0C, not through SessionOverrideContract).

**Recommendation:** No action required.

**Oscillation classification:** None.

---

### F-10. D3 Run Report row honest; D4 row honestly classifies r3 fix-created-family generation-0 reopen due to external cascade

**Severity: LOW**

The D3 row at `ai-roadmap-phase-1.md:2473` reads: "I checked WU count, internal dependency IDs, topological wave acyclicity, SessionOverrideContract cross-phase overlay, and explicit blocked-on annotations by text scan. I did not run external reviewer tools or implementation tests because this is a roadmap artifact." The five named checks correspond to verifiable artifact properties (WU count = 56 verified; internal dependency IDs verified in F-1; topological wave acyclicity verified in F-7; SessionOverrideContract cross-phase overlay verified in F-2; blocked-on annotations verified in F-8). The disclaimer about implementation tests is appropriate. No overstatement — in particular, no claim of running external reviewer scripts or schema validators.

The D4 row at `ai-roadmap-phase-1.md:2474` reads: "bundling-family remains closed by the WU-1-45/WU-1-56 split; state-machine-criteria-family remains addressed by per-method criteria; dependency-encoding-family is updated for WU-0C-N1..WU-0C-N5 incoming edges; fix-created-family reopens at generation 0 for this externally-driven Round 3 cascade from proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4."

Per-family verification:

- **bundling-family:** F-1 confirmed WU-1-45/WU-1-56 split is preserved byte-for-byte from r2; no new bundle introduced by r3 (no WU was merged or expanded). ✓ "closed" claim correct.
- **state-machine-criteria-family:** Spot-checked acceptance criteria on affected WUs:
   - WU-1-12 line 494–501: per-method binary criteria for `emit_tool_call_audit` plus override-link/no-mutation criteria preserved. ✓
   - WU-1-25 line 909–917: per-state binary criteria for view rendering plus no-mutation-control criteria. ✓
   - WU-1-45 line 1531–1538: per-method binary criteria for `run_redacted_provider_probe` plus six new no-routing/no-quota/no-resume/no-mutation criteria. ✓
   - WU-1-56 line 1566–1571: five per-state fixture criteria preserved from r2. ✓
   "addressed by per-method criteria" claim correct.
- **dependency-encoding-family:** F-1, F-2, F-3, F-4, F-6, F-7 above confirm the explicit-IDs + topological-waves discipline holds and now extends to WU-0C-N* IDs. The D4 row does not overstate — it claims "updated for WU-0C-N1..WU-0C-N5 incoming edges," which is exactly what is delivered. ✓
- **fix-created-family:** This is the second brownfield in the Phase 1 local loop (after r2's WU-1-45/56 split). The "reopens at generation 0" framing matches `plans/audit/ai-roadmap-phase-1.md:78` ("`fix-created-family` reopens at generation 0 in the Phase 1-local loop, externally driven by the proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 cascade. This is not a local reviewer-discovered defect in r2..."). The r3 fix is tightly scoped (cross-phase edge addition + per-WU narrowing language + new Stitch Notes overlay) and does not create new local fixes elsewhere in the artifact (verified by F-1 internal-graph preservation, F-6 non-N envelope preservation, F-8 outgoing block preservation, F-9 critical path preservation). ✓

The D4 row is honest about the externally-driven r3 reopen and does not inflate progress or overstate closure on any other family.

**Recommendation:** No action required.

**Oscillation classification:** `fix-created-family`, generation 0 in the Phase 1 local loop, externally driven. No same-family oscillation, no two-generation in-gate.

---

### F-11. Two presentation observations carry forward from r1/r2 unchanged (INFO)

**Severity: INFO**

The two r1/r2 INFO observations remain factually accurate in r3 and require no r3 action:

1. **VS-002 WUs do not list IPC envelope primitives (WU-0A-05..08, WU-0C-34/35).** Re-checked at `ai-roadmap-phase-1.md:2084–2126` (VS-002 incoming). VS-002 incoming includes WU-0C-36/36a/37/37a but not WU-0C-34/35 — bidirectionally consistent with `ai-roadmap-phase-0c.md:3658` (Outgoing-to-Phase-1 for VS-002 lists `policy WU-0C-04; configuration WU-0C-08..WU-0C-10; render WU-0C-21..WU-0C-26; UI WU-0C-36; and audit WU-0C-37` — IPC excluded by design). Upstream-of-Phase-1 omission, not a Phase 1 dependency-encoding regression.
2. **VS-003 WUs do not list UI-shell primitives (WU-0A-11..13, WU-0C-36).** Re-checked at `ai-roadmap-phase-1.md:2128–2183` (VS-003 incoming). VS-003 incoming omits WU-0C-36/36a — bidirectionally consistent with `engineering-roadmap.md:110` (VS-003 Foundation deps omits "UI shell") and `ai-roadmap-phase-0c.md:3659` (Outgoing-to-Phase-1 for VS-003 omits WU-0C-36). Upstream-of-Phase-1 omission.

Neither rises to MEDIUM in r3 for the same reason as in r1/r2: both edges are absent from the engineering-roadmap and Phase 0C in the first place, so adding them in Phase 1 r3 would create unilateral asymmetry. The F-5 caveat about WU-0C-N1/N2/N3/N4 producer-side enumeration is the analogous downstream-of-upstream-omission flag for the new SessionOverrideContract layer.

**Recommendation:** No action required for the Phase 1 r3 dependency gate. Carry forward as a note for any future engineering-roadmap revision cycle that may also re-symmetrize Phase 0C-r4 producer-side enumeration of Phase 1 r3 read-only consumers.

**Oscillation classification:** None — same downstream-of-upstream-omission classification as r1/r2.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R3-DEP-F01 | Internal Phase 1 graph remains acyclic; topological sort succeeds with 56 nodes; no Phase 1-local edge added or removed | LOW |
| R3-DEP-F02 | SessionOverrideContract incoming overlay correctly encoded as cross-phase incoming, not new Phase 1-local edges; 23 (WU-0C-N*, WU-1-XX) edges enumerated at three consistent locations | LOW |
| R3-DEP-F03 | Affected-WU SessionOverrideContract envelopes scoped to actual contract surface; no over-broad attachment | LOW |
| R3-DEP-F04 | WU-1-10's full N1..N5 envelope is correct: central transcript read seam needs every WU-0C-N* primitive; descendants like WU-1-13 receive only required subset | LOW |
| R3-DEP-F05 | Bidirectional consistency vs Phase 0C-r4 holds at the macro / generic-downstream layer; per-WU producer enumeration is uneven but explicitly authorized by lines 3645/3687 and by the per-WU N5 line at 1897 | LOW (with INFO caveat) |
| R3-DEP-F06 | Pre-existing Phase 0A/0B/0C non-N incoming edges remain byte-for-byte from r2 across every VS | LOW |
| R3-DEP-F07 | Parallelization Map remains a valid 7-wave topological partition (18+12+9+5+7+3+2 = 56); SessionOverrideContract edges are cross-phase, not Phase 1-local | LOW |
| R3-DEP-F08 | Outgoing-To-Phase-2+ block preserves all 24 (Phase 1 VS, Phase 2+ VS) pairs; new Blocked-on annotations on (VS-003, VS-018) and (VS-006, VS-015) are precise about substrate features | LOW |
| R3-DEP-F09 | Critical Path is unchanged; r3 re-check note honest about Phase 0 critical incoming gates | LOW |
| R3-DEP-F10 | D3 Run Report row honest; D4 row honestly classifies r3 fix-created-family generation-0 reopen due to external cascade | LOW |
| R3-DEP-F11 | Two presentation observations (VS-002 IPC, VS-003 UI shell) carry forward from r1/r2 unchanged | INFO |

## What LOW requires

For this LOW rating to remain valid in any future revision, the following conditions must hold:

1. The internal Phase 1 dependency graph remains acyclic; every internal edge resolves to an earlier-wave WU. WU-1-56 remains a Wave 1 leaf; SessionOverrideContract edges remain cross-phase incoming, not Phase 1-internal.
2. Every Phase 1 WU continues to declare cross-phase incoming edges as specific WU-0A-NN / WU-0B-NN / WU-0C-NN / WU-0C-N* IDs. No vague references (e.g. "Phase 0 substrate", "SessionOverrideContract upstream") appear.
3. Per-WU SessionOverrideContract envelopes remain scoped to actual contract surface: WU-1-10 keeps the full N1..N5 envelope; WU-1-12/13/28 stay N5-only; WU-1-23/25 stay N2/N5; WU-1-45/48/49/50 stay N4/N5; WU-1-03/04/05 stay N2-only. No WU silently broadens to claim more N* primitives than its contract reads.
4. The `Round 3 SessionOverrideContract incoming overlay` block in the Dependency Graph and the corresponding overlay block in Stitch Notes remain bidirectionally consistent with the per-WU `Cross-phase incoming (SessionOverrideContract):` lines.
5. For each Phase 1 r3 SessionOverrideContract edge, either (a) the corresponding Phase 0C `Produces` line for the named WU-0C-N* enumerates the Phase 1 consumer per VS, or (b) the line 3645 generic-downstream-Phase-1/2/3 clause and the line 3687 standing instruction continue to authorize generic-downstream consumers. If a future Phase 0C round removes either authorization, the Phase 1 r3 incoming declarations must be re-derived.
6. The Phase 0A/0B/0C non-N envelope (the r2-converged 76-internal-edge / per-VS incoming cross-phase block) remains byte-for-byte. If a future Phase 0X cell is added or removed, Phase 1 incoming updates in lockstep.
7. The Parallelization Map continues to be a valid topological-level partition: every WU's wave number equals 1 + max(wave number of declared internal parents). No "Parallelizable with" line lists a same-wave WU that is also a declared dependency. The 7-wave skeleton 18+12+9+5+7+3+2 = 56 is preserved.
8. The Outgoing-To-Phase-2+ block continues to enumerate exactly 24 (Phase 1 VS, Phase 2+ VS) pairs; WU-1-56 remains excluded from every VS-006 → Phase 2+ foundation set. Blocked-on annotations on (VS-003, VS-018) and (VS-006, VS-015) continue to name the precise substrate features (`agents session locate / export / import-replace`, `pause-handshake`, thin worker launcher with `--session-id` forced-flag) and remain bidirectionally consistent with the Phase 0C-r4 VS-018 / VS-015 / VS-021 outgoing-to-Phase-2+ Blocked-on annotations at lines 3677/3674/3680.
9. The D3 Regression-check row continues to describe only checks actually performed and does not overstate (e.g. claims of running implementation tests when only artifact assertions were performed). The new five-check enumeration (WU count, internal dependency IDs, topological wave acyclicity, SessionOverrideContract cross-phase overlay, blocked-on annotations) remains the honest description.
10. The D4 row continues to honestly classify each watch family. In particular: `bundling-family` remains "closed" (a re-fire would reopen it); `state-machine-criteria-family` remains "addressed"; `dependency-encoding-family` remains "addressed and updated for WU-0C-N* edges" (silent removal of the SessionOverrideContract overlay would break this claim); `fix-created-family` remains classified as "generation 0 externally-driven" until the next round closes it (a same-family local re-fire in r4+ would push it to generation 1 with a triggered hard-decompose per audit-history convention).
