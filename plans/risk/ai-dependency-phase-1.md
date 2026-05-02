# Phase 1 — Dependency Risk Assessment (Round 4)

**Rating: LOW**

## Scope (Round 4)

Round 4 brownfield-cascade assessment of `product-strategy/ai-roadmap-phase-1.md` (56 WUs, unchanged from r2/r3) after the proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5 cascade made `SessionOverrideContract` v2-only, removed `WU-0C-N4` (`AgentRunnerSchemaProbe`), folded the schema probe into `WU-0C-N3` `schema_version_probe`, and removed the prior `Blocked-on:` substrate-feature annotations after the agent-runner feature requests landed. Per `plans/audit/ai-roadmap-phase-1.md:81–89` round-4 entry, the proposer integrated the upstream changes without adding, deleting, or merging Phase 1 WUs. The dependency surface change against r3 should be exactly:

1. Drop every `WU-0C-N4` token from per-WU `Cross-phase incoming (SessionOverrideContract):` lines (5 affected WUs: WU-1-10 / WU-1-45 / WU-1-48 / WU-1-49 / WU-1-50).
2. Replace the schema-probe role on the four VS-006 read-only preflight WUs (WU-1-45 / WU-1-48 / WU-1-49 / WU-1-50) by substituting `WU-0C-N3` for the dropped `WU-0C-N4`. WU-1-10 keeps `WU-0C-N3` from r3 and now drops `WU-0C-N4` only.
3. Rewrite the Dependency Graph SessionOverrideContract overlay block from the r3 form to the r4 form at `ai-roadmap-phase-1.md:2421–2431` enumerating the new 22-edge set.
4. Rewrite the Stitch Notes SessionOverrideContract incoming-edge block (`ai-roadmap-phase-1.md:2975–2985`) to enumerate the new 22-edge set keyed by Phase 0C-r5 outgoing declarations.
5. Remove every prior `Blocked-on:` substrate-feature annotation (the r3 (VS-003, VS-018) and (VS-006, VS-015) Outgoing-To-Phase-2+ rows, and the WU-1-10 dependency-line `Blocked-on:` annotation at the old r3 line 447). Phase 1 r4 carries no `Blocked-on:` text at all.
6. Add `r4 cascade:` revision-rationale lines on each of the 13 affected WUs, plus a Round 4 SessionOverrideContract Audit table at `ai-roadmap-phase-1.md:98–118`, plus r4 re-check notes on Critical Path (line 2447), Parallelization Map (line 2457), Run Report D3/D4 rows (lines 2475/2476), and Self-classification (line 2552).

The 76 internal Phase 1 edges, the 7-wave Parallelization Map skeleton (18+12+9+5+7+3+2 = 56), the 24-pair Outgoing-To-Phase-2+ block, and the byte-for-byte Phase 0A/0B/0C non-N incoming edges must remain unchanged from r3 (which were unchanged from r2).

Cross-references walked for r4:

- `product-strategy/ai-roadmap-phase-1.md` lines 22–96 (Inventory, 56 WUs unchanged), lines 98–118 (Round 4 SessionOverrideContract Audit table), lines 184–411 (WU-1-03/04/05/10 affected normalizer + transcript ingestion), lines 479–550 (WU-1-12/13 audit + drilldown), lines 827–931 (WU-1-23/25 evidence pointer + inspector pane), lines 992–1027 (WU-1-28 render audit subscription), lines 1516–1580 (WU-1-45/56 provider probe + fixture), lines 1643–1748 (WU-1-48/49/50 route + provider preflight pane), lines 1902–1963 (Dependency Graph; 76 internal edges unchanged), lines 1965–2419 (per-VS Incoming-from-Phase-0A/0B/0C; non-N incoming unchanged), lines 2421–2431 (new Round 4 SessionOverrideContract incoming overlay), lines 2433–2449 (Critical Path with r3+r4 re-check notes), lines 2451–2467 (Parallelization Map with r3+r4 re-derive notes), lines 2469–2547 (Run Report incl. updated D3/D4 rows for r4 and r2/r3/r4 self-classification), lines 2554–2974 (Stitch Notes Phase 0A/0B/0C non-N blocks unchanged), lines 2975–2985 (new Round 4 SessionOverrideContract incoming-edge block), lines 2987–3033 (Outgoing-To-Phase-2+ now without `Blocked-on:` annotations).
- `worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md` lines 1670–1846 (WU-0C-N2 / WU-0C-N1 / WU-0C-N4-removed / WU-0C-N3 / WU-0C-N5 declarations and per-WU produces blocks), line 1735 (WU-0C-N4 removal rationale), lines 1739–1790 (WU-0C-N3 rescoped from v1 `AgentRunnerDbAdapter` to v2 `AgentRunnerCliAdapter`, schema probe folded into adapter construction at line 1774), line 1842 (WU-0C-N5 produces consumed by VS-001 evidence inspectors and VS-003 audit surfaces — explicit per-VS Phase 1 named consumer), line 3586 (`SessionOverrideContract WU-0C-N1/N2/N3/N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata.` — generic-downstream authorization clause for r5), lines 3596–3603 (Outgoing-to-Phase-1 by VS, including VS-001 / VS-003 / VS-006 outgoing rows that exclude SessionOverrideContract from the per-VS named consumer set), line 3628 (standing instruction: `Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1/N2/N3/N5 where worker launch, worker reintegration, turn decomposition, detail injection, or repack planning consumes session override receipts/refusals.`).
- `product-strategy/proposal.md` (Phase 1 worktree copy, identical to proposal-r6) — SessionOverrideContract Boundary now treats all session reads/writes via the v2 adapter; substrate features `agents session locate / export / import-replace` and `pause-handshake` documented as landed.
- `product-strategy/engineering-roadmap.md` (Phase 1 worktree copy, identical to engineering-roadmap-r5) — SessionOverrideContract Foundation row consumed by VS-010 / VS-012 / VS-018 / VS-020 / VS-021 with `WU-0C-N4` dropped.
- `plans/audit/ai-roadmap-phase-1.md` round-4 entry (lines 81–89).
- `plans/risk/ai-dependency-phase-1.md` r3 baseline (this file's prior round) — for r3-vs-r4 diff verification.

## Findings (Round 4)

### R4-DEP-F01. Internal Phase 1 dependency graph remains acyclic; topological sort succeeds with 56 nodes; no Phase 1-local edge added or removed by the r4 cascade

**Severity: LOW**

The Dependency Graph block at `ai-roadmap-phase-1.md:1906–1963` enumerates every Phase 1 WU's internal parent set. Walking the complete 56-line edge listing:

- All 76 r3 internal edges are preserved verbatim. Sampled spot-checks: `WU-1-06 <- WU-1-02, WU-1-03, WU-1-04, WU-1-05` (line 1912), `WU-1-10 <- WU-1-03, WU-1-04, WU-1-05, WU-1-07` (line 1916), `WU-1-12 <- WU-1-06, WU-1-08, WU-1-09, WU-1-11` (line 1918), `WU-1-50 <- WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49` (line 1957) all match r3 byte-for-byte. No Phase 1 WU acquired or lost an internal parent in r4.
- `WU-1-56 <- none` (line 1952) is preserved; the fixture WU remains a Wave 1 leaf with no outgoing internal edges. Verified by scanning every `Phase 1 internal:` block on WU-1-46..50 at lines 1605, 1636, 1668, 1703, 1739: none lists WU-1-56.
- Topological sort over the 56 nodes yields 7 waves with the same memberships as r3 (re-verified in R4-DEP-F07).
- The r4 audit at lines 100–118 explicitly states no WU was added, removed, or merged. WU count (56) reconciles across header inventory (lines 41–96), Round 4 SessionOverrideContract Audit table (lines 102–118), Dependency Graph (lines 1906–1963), Run Report metric `Total Phase 1 WUs | 56` (line 2543), D1 audit count `56 rows = 56 WUs in the inventory` (line 2480), and per-WU D1 ownership table (lines 2484–2539).

The r3-acyclic 56-node graph remains acyclic in r4.

**Recommendation:** No action required.

**Oscillation classification:** None — `dependency-encoding-family` does not re-fire on the internal Phase 1 graph. The audit at `plans/audit/ai-roadmap-phase-1.md:88` keeps `fix-created-family` at generation 0 because the cascade is externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5; R4-DEP-F01 is the first gate confirming the externally-driven r4 brownfield does not introduce a new local dependency-encoding regression.

---

### R4-DEP-F02. SessionOverrideContract incoming overlay correctly drops WU-0C-N4 and substitutes WU-0C-N3 schema_version_probe; 22 (WU-0C-N*, WU-1-XX) edges enumerated at three consistent locations; no dangling N4 reference remains anywhere in the artifact

**Severity: LOW**

The r4 brownfield directive in `plans/audit/ai-roadmap-phase-1.md:81–89` requires `WU-0C-N4` to be removed everywhere it appeared in r3 (with the schema-probe role migrated to `WU-0C-N3` `schema_version_probe`) without leaving any dangling token in any per-WU dependency block, in any overlay, or in any Stitch Notes block. Verification of the three consistent locations:

1. **Per-WU `Cross-phase incoming (SessionOverrideContract):` lines.** Each affected WU declares its r4 SessionOverrideContract parents on a separate line. Sampled:
   - `WU-1-03` line 211: `WU-0C-N2.` (unchanged from r3) ✓
   - `WU-1-04` line 246: `WU-0C-N2.` (unchanged from r3) ✓
   - `WU-1-05` line 281: `WU-0C-N2.` (unchanged from r3) ✓
   - `WU-1-12` line 506: `WU-0C-N5.` (unchanged from r3) ✓
   - `WU-1-13` line 542: `WU-0C-N5.` (unchanged from r3) ✓
   - `WU-1-23` line 855: `WU-0C-N2, WU-0C-N5.` (unchanged from r3) ✓
   - `WU-1-25` line 922: `WU-0C-N2, WU-0C-N5.` (unchanged from r3) ✓
   - `WU-1-28` line 1020: `WU-0C-N5.` (unchanged from r3) ✓
   - `WU-1-45` line 1543: `WU-0C-N3, WU-0C-N5.` ✓ — **r3 was `WU-0C-N4, WU-0C-N5`; r4 substitutes N3 for N4.**
   - `WU-1-48` line 1670: `WU-0C-N3, WU-0C-N5.` ✓ — **N4 → N3 substitution.**
   - `WU-1-49` line 1705: `WU-0C-N3, WU-0C-N5.` ✓ — **N4 → N3 substitution.**
   - `WU-1-50` line 1741: `WU-0C-N3, WU-0C-N5.` ✓ — **N4 → N3 substitution.**
   - `WU-1-10` line 441 (folded into the main `Cross-phase incoming:` line): `..., WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5.` ✓ — **r3 had `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5`; r4 simply drops N4 since N3 was already declared.**
2. **Round 4 SessionOverrideContract incoming overlay** (`ai-roadmap-phase-1.md:2421–2431`) restates the same pairs at the cross-phase aggregation layer:
   - `WU-0C-N2 -> WU-1-03, WU-1-04, WU-1-05`
   - `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5 -> WU-1-10`
   - `WU-0C-N5 -> WU-1-12, WU-1-13, WU-1-28`
   - `WU-0C-N2, WU-0C-N5 -> WU-1-23, WU-1-25`
   - `WU-0C-N3, WU-0C-N5 -> WU-1-45, WU-1-48, WU-1-49, WU-1-50`
3. **Stitch Notes restatement** (`ai-roadmap-phase-1.md:2975–2985`) re-emits each (WU-0C-N*, WU-1-XX) pair as an enumerated incoming edge with a one-line scope rationale (e.g. line 2980 for WU-1-10 reads "read-only transcript ingestion through SessionOverrideContract v2, schema probing, and refusal evidence").

**Edge count math (r3 → r4):** r3 declared 23 edges; r4 declares 22 edges. The diff is `-5 N4 edges (N4→1-10, N4→1-45, N4→1-48, N4→1-49, N4→1-50) +4 N3 edges (N3→1-45, N3→1-48, N3→1-49, N3→1-50) = -1 net edge`. The N3→1-10 edge already existed in r3 (because r3 WU-1-10 needed v1 read behavior via N3) and is preserved in r4 (where N3 is the v2 `AgentRunnerCliAdapter`).

Per-WU r4 envelope sizes:
- N2 only: WU-1-03, WU-1-04, WU-1-05 (3 WUs × 1 = 3 edges).
- All five r3 → all four r4 (full envelope minus N4): WU-1-10 (4 edges).
- N5 only: WU-1-12, WU-1-13, WU-1-28 (3 WUs × 1 = 3 edges).
- N2 + N5: WU-1-23, WU-1-25 (2 WUs × 2 = 4 edges).
- N3 + N5: WU-1-45, WU-1-48, WU-1-49, WU-1-50 (4 WUs × 2 = 8 edges).

Total = 3 + 4 + 3 + 4 + 8 = **22 (WU-0C-N*, WU-1-XX) edges across the same 13 affected WUs as r3**, all encoded at three consistent locations.

**No dangling WU-0C-N4 reference remains.** Grepping the entire `ai-roadmap-phase-1.md` for `N4` returns five hits, all of which are commentary about the *removal* of the token (line 100 audit table preface; line 2423 overlay block preface; line 2445 Critical Path r3+r4 re-check note; line 2447 r4 re-check note; line 2457 Parallelization Map r4 re-derive note; line 2552 Self-classification r4 entry). No per-WU dependency block, no overlay, no Stitch Notes pair, no per-VS incoming block lists `WU-0C-N4` operationally. The token is fully retired.

Crucially, **no SessionOverrideContract edge appears as a Phase 1-internal parent on any WU**. The r4 edges flow only cross-phase incoming, matching the proposer's r3+r4 re-derive note ("the topological waves remain 18+12+9+5+7+3+2 because SessionOverrideContract edges are cross-phase incoming gates, not new Phase 1-local edges" — `ai-roadmap-phase-1.md:2455`, plus the r4 addendum at line 2457).

**Recommendation:** No action required.

**Oscillation classification:** `fix-created-family`, generation 0, externally driven. The proposer applied the correct r4 pattern in one pass: drop N4 everywhere, substitute N3 where the schema-probe role lived, keep all N1/N2/N5 wiring untouched, mirror the change in the overlay and Stitch Notes blocks.

---

### R4-DEP-F03. Affected-WU SessionOverrideContract incoming envelopes remain scoped to the WU's actual contract surface after WU-0C-N4 removal; the WU-0C-N3 substitute is exactly where schema-probe evidence is genuinely needed

**Severity: LOW**

The audit-history brownfield directive (`plans/audit/ai-roadmap-phase-1.md:87`) requires Phase 1 r4 to declare the WU-0C-N3-substituted dependencies only on WUs that actually consume the schema-probe role (now folded into N3 via `schema_version_probe`). Verifying scope precision per WU:

| Phase 1 WU | Contract surface | r3 WU-0C-N* | r4 WU-0C-N* | r3→r4 diff |
|---|---|---|---|---|
| WU-1-03 ClaudeToolCallNormalizer | `normalize_claude_tool_call(...)` over normalized turn/hook/trace evidence; never opens JSONL paths (line 202–203) | N2 | N2 | unchanged ✓ |
| WU-1-04 CodexToolCallNormalizer | `normalize_codex_tool_call(...)` over normalized turn/rollout/trace evidence (line 237–238) | N2 | N2 | unchanged ✓ |
| WU-1-05 OpencodeToolCallNormalizer | `normalize_opencode_tool_call(...)` over normalized hook/event/trace evidence (line 272–273) | N2 | N2 | unchanged ✓ |
| WU-1-10 TranscriptIngestionAdapter | `ingest_transcript(session_id, session_read_ref?)` reads through WU-0C-15d or WU-0C-N1 `read_transcript` / `get_session_metadata`; refusal recorded; never calls N1 mutation methods (line 432–434) | N1, N2, N3, N4, N5 | N1, N2, N3, N5 | `-N4` ✓ — schema-probe need now satisfied by `schema_version_probe()` on the N1 trait, implemented inside the N3 v2 `AgentRunnerCliAdapter`. |
| WU-1-12 ToolCallAuditEmitter | `emit_tool_call_audit(...)` may link to WU-0C-N5 override registry records (line 497); never mutates overrides (line 498) | N5 | N5 | unchanged ✓ |
| WU-1-13 ToolCallDrilldownComponent | View shows override receipt/refusal pointer from WU-0C-N5 (line 533); exposes no mutation control (line 534) | N5 | N5 | unchanged ✓ |
| WU-1-23 RenderEvidencePointerService | Pointers may include WU-0C-N5 override receipt/refusal IDs and WU-0C-N2 session metadata refs (line 846); no SessionOverrideContract mutation (line 847) | N2, N5 | N2, N5 | unchanged ✓ |
| WU-1-25 WorkingSetInspectorPane | Pane displays override receipt/refusal status from WU-0C-N5 and override-derived imposed-context evidence from WU-0C-N2 (line 913); no mutation control (line 914) | N2, N5 | N2, N5 | unchanged ✓ |
| WU-1-28 RenderAuditSubscription | Stream may include SessionOverrideStore audit events from WU-0C-N5 (line 1011); acknowledgement never mutates an override (line 1012) | N5 | N5 | unchanged ✓ |
| WU-1-45 RedactedProviderProbeService | Session-override compatibility comes from WU-0C-N3/WU-0C-N5 evidence, not probe-owned JSONL (line 1535); no resume/quota/auth ownership (line 1534) | N4, N5 | N3, N5 | `-N4 +N3` ✓ — r3 cited N4 for schema-probe outputs as adapter capability evidence; r4 cites N3 because schema-probe is now part of N3 adapter construction (Phase 0C-r5 line 1774). |
| WU-1-48 RouteEligibilityResolver | Workload requirements may include need for session override capability via opaque WU-0C-N3/WU-0C-N5 refs (line 1662); no direct probe/mutation (line 1661) | N4, N5 | N3, N5 | `-N4 +N3` ✓ — same migration as WU-1-45. |
| WU-1-49 RouteDenialReasonClassifier | Distinguishes harness preflight denials from agent-runner / SessionOverrideContract refusal reasons via opaque WU-0C-N5 registry/evidence refs (line 1696–1697) | N4, N5 | N3, N5 | `-N4 +N3` ✓ — same migration as WU-1-45. |
| WU-1-50 ProviderPreflightPane | Pane displays session-override capability/refusal metadata from WU-0C-N3/WU-0C-N5 as read-only evidence (line 1732–1733) | N4, N5 | N3, N5 | `-N4 +N3` ✓ — UI display surface for the VS-006 preflight quartet, same migration. |

**Scope claim:** The five `+N3 −N4` substitutions on WU-1-45 / 48 / 49 / 50 are correct for the new Phase 0C-r5 layering: schema-probe evidence is now an N3 adapter-construction-time artifact (`agents session schema-probe` invoked inside `AgentRunnerCliAdapter::new` per `ai-roadmap-phase-0c.md:1774`) rather than a separately-owned WU. The N4-only-drop on WU-1-10 is correct because r3 had WU-1-10 declaring both N3 (for v1 read behavior) and N4 (for schema probe); r4 simply drops N4 since the schema-probe role is now part of the N3 trait surface that WU-1-10 already declared.

The non-affected WUs in the audit table at `ai-roadmap-phase-1.md:118` (`WU-1-01/02/06/07/08/09/11/14..22/24/26/27/29..44/46/47/51..55/56`) declare zero SessionOverrideContract incoming edges in r4 — verified by scanning each for the literal substring `WU-0C-N`. None of these WUs encode session storage, provider routing, quota, resume, cross-provider porting, or session mutation behavior.

**Recommendation:** No action required. The audit-history concern that "schema-probe references move to WU-0C-N3 `schema_version_probe`" (`plans/audit/ai-roadmap-phase-1.md:87`) is satisfied.

**Oscillation classification:** `fix-created-family`, generation 0, applied tightly per WU.

---

### R4-DEP-F04. WU-1-10's r4 envelope (N1, N2, N3, N5) is correct: the central Phase 1 transcript read seam still needs every non-N4 WU-0C-N* primitive

**Severity: LOW**

WU-1-10 TranscriptIngestionAdapter is unique among the 13 affected WUs in declaring the largest WU-0C-N* envelope (now four primitives in r4, down from five in r3). Verifying envelope-versus-contract for r4:

- **WU-0C-N1 (`SessionOverrideContract` trait) → WU-1-10.** WU-1-10 calls `read_transcript` and `get_session_metadata` on the N1 trait surface for read-only ingestion (acceptance criterion at `ai-roadmap-phase-1.md:432`). Phase 0C-r5's `WU-0C-N1` produces line at `ai-roadmap-phase-0c.md:1729` enumerates VS-010/012/018/020/021 plus the line 3586 generic-downstream clause and the line 3628 standing instruction authorizing this Phase 1 r4 wiring. The trait is the only sanctioned read seam.
- **WU-0C-N2 (`TranscriptTurn`/`SessionLocation`/`SessionMetadata` DTOs) → WU-1-10.** WU-1-10's downstream events are the canonical session-override DTOs. N2 producer line at `ai-roadmap-phase-0c.md:1679` enumerates VS-010/012/018/020/021 plus the line 3586 generic-downstream clause. ✓
- **WU-0C-N3 (`AgentRunnerCliAdapter` v2) → WU-1-10.** In r3 N3 was the v1 `AgentRunnerDbAdapter` and WU-1-10 read through it as the only shipping adapter. In r4 N3 is rescoped to the v2 `AgentRunnerCliAdapter` which now also owns `schema_version_probe()` (Phase 0C-r5 line 1774 acceptance criterion: `Adapter construction calls 'agents session schema-probe' and refuses operation if the output is malformed, required feature flags are absent, or 'safe_for_import_replace' is false`). WU-1-10 still needs N3 because it is still the active SessionOverrideContract implementation. ✓
- **WU-0C-N5 (`SessionOverrideStore` registry) → WU-1-10.** WU-1-10 records refusal evidence as registered evidence/audit metadata, which is the registry's role. N5 produces line at `ai-roadmap-phase-0c.md:1842` explicitly enumerates VS-001 evidence inspectors and VS-003 audit surfaces — WU-1-10 is the VS-003 entry point producing override-derived evidence VS-001 inspectors then display. ✓

The dropped **WU-0C-N4** primitive is no longer needed by WU-1-10 because the schema-probe call that R3-DEP-F04 cited as the rationale for N4 inclusion is now invoked on N1 (via `schema_version_probe()` defined in the `SessionOverrideContract` trait at `ai-roadmap-phase-0c.md:1694` and implemented inside the N3 adapter constructor). The schema-probe surface that WU-1-10 actually consumes has not vanished — it has migrated from a separate WU to a method on the trait WU-1-10 already declared. Therefore the N4 drop is correct: it removes an obsolete reference rather than weakening WU-1-10's read-seam guarantees.

The strict subset N5 ⊂ {N1, N2, N3, N5} on WU-1-13 (the only WU internally downstream of WU-1-10 transitively via WU-1-12) reflects the layered design correctly: WU-1-13 still consumes only the N5 registry, not the N1 trait, the N2 DTOs, or the N3 adapter.

**Recommendation:** No action required.

**Oscillation classification:** None — full-envelope discipline holds at the read seam without leaking through to descendants. The r3-to-r4 transition shrinks the envelope by exactly one primitive (N4), which is the correct change.

---

### R4-DEP-F05. Bidirectional consistency vs Phase 0C-r5 holds at the macro / generic-downstream layer; the N5-to-VS-001/VS-003 producer-side enumeration tightens to per-VS for half the new edges

**Severity: LOW (with INFO caveat about per-VS producer-side enumeration on N1/N2/N3)**

For the bidirectional contract to hold, every (Phase 0C-r5 WU-0C-N*, Phase 1 WU) edge declared on the Phase 1 incoming side must have a producer-side justification on the Phase 0C side.

Mapping Phase 1 r4 edges against Phase 0C-r5 producer-side declarations:

| Phase 0C-r5 producer | Phase 1 r4 incoming consumers | Phase 0C-r5 declaration source |
|---|---|---|
| WU-0C-N5 SessionOverrideStore registry | WU-1-10, WU-1-12, WU-1-13, WU-1-23, WU-1-25, WU-1-28, WU-1-45, WU-1-48, WU-1-49, WU-1-50 | **Explicit per-VS `Produces` line** at `ai-roadmap-phase-0c.md:1842`: "Workspace-level override ledger consumed by VS-001 evidence inspectors, VS-003 audit surfaces, VS-020 recovery, VS-021 reroute governance, and WU-0C-31 recovery metadata reads." VS-001 evidence inspectors = WU-1-23/25; VS-003 audit surfaces = WU-1-12/13/28 (and the read seam WU-1-10). The VS-006 provider preflight quartet (WU-1-45/48/49/50) is not named at line 1842 but is authorized by the line 3586 generic-downstream clause. ✓ Strong direct authorization for VS-001/VS-003 surfaces; generic authorization for VS-006. |
| WU-0C-N2 TranscriptTurn/SessionLocation/SessionMetadata DTOs | WU-1-03, WU-1-04, WU-1-05, WU-1-10, WU-1-23, WU-1-25 | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1679`: "Canonical session-override DTOs consumed by WU-0C-N1, WU-0C-N3, WU-0C-N5, VS-010, VS-012, VS-018, VS-020, and VS-021." Phase 1 VS-003/VS-001 not named per-VS, but **authorized by the line 3586 generic-downstream-Phase-1/2/3 clause** ("downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata"). ✓ Generic authorization. |
| WU-0C-N1 SessionOverrideContract trait | WU-1-10 | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1729`: "Versioned session write-back trait consumed by WU-0C-N3, WU-0C-N5, VS-010 turn-decomposition/detail-injection, VS-012 repack planner, VS-018 worker-output reintegration, VS-020 recovery, and VS-021 reroute governance." VS-003 not named per-VS. **Authorized by line 3628** ("Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1/N2/N3/N5") and by line 3586. ✓ Generic authorization. |
| WU-0C-N3 AgentRunnerCliAdapter v2 (rescoped from v1 in r5) | WU-1-10, WU-1-45, WU-1-48, WU-1-49, WU-1-50 | **Per-WU `Produces` line** at `ai-roadmap-phase-0c.md:1784`: "Shipping `SessionOverrideContract` implementation consumed by VS-010, VS-012, VS-018, VS-020, and VS-021." Phase 1 not named per-VS. **Authorized by line 3586 generic-downstream and line 3628 instruction.** ✓ Generic authorization. |

**Caveat (INFO).** The per-WU Phase 0C-r5 `Produces` lines for WU-0C-N1/N2/N3 do not enumerate the specific Phase 1 r4 read-only consumers. Strict bidirectional consistency on those four edge classes holds only via:

1. The line 3586 generic-downstream clause: "downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata."
2. The line 3628 standing instruction: "Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1/N2/N3/N5 where worker launch, worker reintegration, turn decomposition, detail injection, or repack planning consumes session override receipts/refusals."

Phase 1 r4 honors line 3628 by declaring explicit incoming edges. The reciprocal — updating Phase 0C-r5's per-WU `Produces` lines to enumerate the new Phase 1 r4 consumers — is a Phase 0C upstream symmetry update, not a Phase 1 r4 dependency-encoding regression. Note that **the Phase 0C-r5 per-VS Outgoing-to-Phase-1 block at lines 3596–3603 explicitly excludes SessionOverrideContract from the named-per-VS consumer set on VS-001 / VS-003 / VS-006**, which is why Phase 1 r4 must lean on the line 3586 generic-downstream clause and the line 3628 standing instruction for the bidirectional read.

The WU-0C-N5 case remains the strongest: line 1842 explicitly names "VS-001 evidence inspectors, VS-003 audit surfaces" as consumers, which directly authorizes the Phase 1 r4 N5 wiring on WU-1-10/12/13/23/25/28. **The VS-006 preflight quartet edges (N3→1-45/48/49/50, N5→1-45/48/49/50) plus all N1/N2/N3 edges hold at the macro / generic-downstream layer.**

The r3-to-r4 transition does not regress bidirectional consistency: r3's R3-DEP-F05 noted exactly the same generic-downstream-only authorization for N1/N2/N3/N4 edges; r4 inherits that asymmetry from Phase 0C-r5 (which itself rescoped N3 and removed N4). The asymmetry is now slightly tighter overall because the dropped N4-related edges no longer require any producer-side justification at all.

**Recommendation:** No action required for the Phase 1 r4 dependency gate. If a future Phase 0C-r6 round closes the symmetry by enumerating Phase 1 r4 read-only consumers in WU-0C-N1/N2/N3 `Produces` lines — analogous to how line 1842 already enumerates VS-001/VS-003 for N5 — the bidirectional contract tightens to per-VS for those edges. Until then, the line 3586 generic-downstream clause is the load-bearing authorization. This is the same downstream-of-upstream-omission pattern noted in r1's R1-DEP-F10 and r3's R3-DEP-F05.

**Oscillation classification:** `fix-created-family`, generation 0 (externally driven). The bidirectional asymmetry pattern is *inherited* from Phase 0C-r5 (which was reshaped from Phase 0C-r4), not *introduced* by Phase 1 r4.

---

### R4-DEP-F06. Pre-existing Phase 0A/0B/0C non-N incoming edges remain byte-for-byte from r3 across every VS

**Severity: LOW**

The r4 brownfield directive (`plans/audit/ai-roadmap-phase-1.md:87`) requires that the existing non-N envelope across all 56 WUs be preserved. Verification:

- **Per-VS Incoming-from-Phase-0A/0B/0C blocks** at `ai-roadmap-phase-1.md:1965–2419`. Sampled per VS:
   - Shared (lines 1967–1988): 20 entries, identical to r3 set.
   - VS-001 (lines 1990–2080): 91 entries, identical to r3 set.
   - VS-002 (lines 2082–2124): 43 entries, identical to r3 set; no IPC or UI-shell additions.
   - VS-003 (lines 2126–2181): 56 entries, identical to r3 set.
   - VS-004 (lines 2183–2237): 55 entries, identical to r3 set.
   - VS-005 (lines 2239–2291): 53 entries, identical to r3 set.
   - VS-006 (lines 2293–2369): 77 entries, identical to r3 set; no per-WU non-N envelope extension.
   - VS-007 (lines 2371–2419): 49 entries, identical to r3 set.
- **Stitch Notes Incoming-from-Phase-0A/0B/0C blocks** (`ai-roadmap-phase-1.md:2554–2974`) re-emit the same per-(Phase-0X-WU, Phase-1-VS) edges as r3.
- **Per-WU `Cross-phase incoming:` lines** for the 13 affected WUs append SessionOverrideContract edges via a separate line (most cases) or extend the same line (WU-1-10 only). Sampled diff against r3:
   - WU-1-03 line 210 main `Cross-phase incoming:` block is byte-for-byte from r3; line 211 SessionOverrideContract line is unchanged (`WU-0C-N2.`).
   - WU-1-10 line 441 main `Cross-phase incoming:` block is r3 envelope minus `, WU-0C-N4`. Verified: the long Phase 0A/0B/0C non-N portion is byte-for-byte; only the trailing `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5` token sequence differs from r3's `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5`.
   - WU-1-45 line 1542 main block is byte-for-byte r3 envelope; line 1543 SessionOverrideContract line is `WU-0C-N3, WU-0C-N5.` (was `WU-0C-N4, WU-0C-N5.` in r3).

Phase 0A/0B/0C pre-N envelope preservation across all 56 WUs holds in r4.

**Recommendation:** No action required.

**Oscillation classification:** `dependency-encoding-family` remains dormant on the non-N envelope. The systematic-from-start discipline established in r1 (closed at generation 0) is unbroken across r2 → r3 → r4.

---

### R4-DEP-F07. Parallelization Map remains a valid topological-level partition; 7 waves of 18+12+9+5+7+3+2 = 56

**Severity: LOW**

The Parallelization Map at `ai-roadmap-phase-1.md:2451–2467` declares the same 7-wave partition as r3 (which was the same as r2). Recomputing each WU's depth as `1 + max(depth of declared Phase 1 internal parents)`:

| Wave | Declared WUs | Verified count | Sample wave-N+1 derivation check |
|---:|---|---:|---|
| 1 | WU-1-01, 02, 07, 08, 09, 11, 14, 15, 16, 20, 32, 37, 38, 39, 45, 51, 53, 56 | 18 | All have empty internal parent set in the Dependency Graph ✓ |
| 2 | WU-1-03, 04, 05, 17, 19, 23, 26, 27, 29, 40, 46, 52 | 12 | WU-1-03 ← WU-1-02 (W1) → W2 ✓; WU-1-23 ← WU-1-07, WU-1-20 (both W1) → W2 ✓ |
| 3 | WU-1-06, 10, 18, 24, 30, 31, 41, 47, 54 | 9 | WU-1-06 ← WU-1-02/03/04/05 (W1, W2, W2, W2) → W3 ✓; WU-1-10 ← WU-1-03/04/05/07 (W2, W2, W2, W1) → W3 ✓ |
| 4 | WU-1-12, 33, 42, 48, 55 | 5 | WU-1-12 ← WU-1-06/08/09/11 (W3, W1, W1, W1) → W4 ✓; WU-1-48 ← WU-1-47 (W3) → W4 ✓ |
| 5 | WU-1-13, 21, 28, 34, 43, 44, 49 | 7 | WU-1-21 ← WU-1-20/12/17 (W1, W4, W2) → W5 ✓; WU-1-49 ← WU-1-45/48 (W1, W4) → W5 ✓ |
| 6 | WU-1-22, 35, 50 | 3 | WU-1-22 ← WU-1-20/21 (W1, W5) → W6 ✓; WU-1-50 ← WU-1-45/46/47/48/49 (W1, W2, W3, W4, W5) → W6 ✓ |
| 7 | WU-1-25, 36 | 2 | WU-1-25 ← WU-1-22/23/24 (W6, W2, W3) → W7 ✓; WU-1-36 ← WU-1-34/35 (W5, W6) → W7 ✓ |

Total **18+12+9+5+7+3+2 = 56 = WU count ✓**. The Run Report metric `Maximum same-wave concurrency | 18` (line 2545) matches Wave 1's 18 WUs.

The `Parallelizable with: same topological wave` lines on each WU were spot-checked for the cascade-affected WUs:
- WU-1-03 line 215 lists 11 Wave 2 mates: `WU-1-04, WU-1-05, WU-1-17, WU-1-19, WU-1-23, WU-1-26, WU-1-27, WU-1-29, WU-1-40, WU-1-46, WU-1-52`. None are also declared internal parents of WU-1-03 (whose only parent is WU-1-02 in W1). ✓
- WU-1-10 line 445 lists 8 Wave 3 mates: `WU-1-06, WU-1-18, WU-1-24, WU-1-30, WU-1-31, WU-1-41, WU-1-47, WU-1-54`. None are declared parents of WU-1-10. ✓
- WU-1-50 line 1745 lists Wave 6 mates `WU-1-22, WU-1-35`; none are declared parents. ✓
- WU-1-45 line 1547 lists 17 Wave 1 mates including WU-1-56. ✓

The proposer's r3 re-derive note at line 2455 (preserved verbatim) and the r4 addendum at line 2457 ("removing WU-0C-N4 and former block-on annotations adds no Phase 1-local edge; waves remain 18+12+9+5+7+3+2 = 56") together correctly encode that r4 changes nothing about depth computation.

**Recommendation:** No action required. The Phase 0B-style Parallelization Map family does not re-fire.

**Oscillation classification:** `dependency-encoding-family` remains dormant on the Parallelization Map. The wave partition validity established in r1 F-6 / r2 F-6 / r3 F-7 is preserved.

---

### R4-DEP-F08. Outgoing-To-Phase-2+ block preserves all 24 (Phase 1 VS, Phase 2+ VS) pairs; all `Blocked-on:` substrate-feature annotations are absent in r4

**Severity: LOW**

The Outgoing-To-Phase-2+ block at `ai-roadmap-phase-1.md:2989–3012` lists the same 24 pairs as r1/r2/r3:

- (VS-001, VS-008), (VS-001, VS-009) — 2
- (VS-002, VS-008), (VS-002, VS-010), (VS-002, VS-014) — 3
- (VS-003, VS-009), (VS-003, VS-010), (VS-003, VS-014), (VS-003, VS-018), (VS-003, VS-019), (VS-003, VS-020) — 6
- (VS-004, VS-008), (VS-004, VS-009), (VS-004, VS-010), (VS-004, VS-015), (VS-004, VS-019) — 5
- (VS-005, VS-010), (VS-005, VS-011) — 2
- (VS-006, VS-009), (VS-006, VS-015), (VS-006, VS-017), (VS-006, VS-020), (VS-006, VS-021) — 5
- (VS-007, VS-015) — 1

Total **2+3+6+5+2+5+1 = 24** ✓. The `Engineering-roadmap dependency rows encoded here` recap (lines 3014–3026) preserves the r1/r2/r3 mapping byte-for-byte.

**`Blocked-on:` annotation removal verification.** Round 3 added two `Blocked-on:` annotations on (VS-003, VS-018) and (VS-006, VS-015), and added a `Blocked-on:` annotation on the WU-1-10 dependency line. Round 4 must remove all three per `plans/audit/ai-roadmap-phase-1.md:83` ("block-on annotations are removed after the agent-runner feature requests landed"). Grepping the entire `ai-roadmap-phase-1.md` for `Blocked-on:|Block-on:|block-on:` returns:

- Eight commentary mentions (lines 100, 217, 252, 287, 447, 512, 548, 861, 928, 1026, 1549, 1676, 1711, 1747, 2423, 2447, 2457, 2475, 2552) all of which read "block-on annotations removed" or paraphrase. No operational `Blocked-on:` rule.
- Zero operational `Blocked-on:` text on the (VS-003, VS-018) row at line 2997 (now reads simply "consumes Phase 1 foundations: WU-1-02, WU-1-03, WU-1-04, WU-1-05, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-10, WU-1-11, WU-1-12, WU-1-13.").
- Zero operational `Blocked-on:` text on the (VS-006, VS-015) row at line 3008 (now reads simply "consumes Phase 1 foundations: WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50.").
- Zero operational `Blocked-on:` text on the WU-1-10 dependency line at line 441 (the prior r3 line 447 annotation is gone; only the `r4 cascade:` revision-rationale at line 447 remains, which is not a `Blocked-on:` rule).

This matches Phase 0C-r5 line 3586's removal of the substrate-feature blockers from the producer side; r4 mirrors the upstream removal correctly.

The `Non-Ownership Notes` block at lines 3030–3033 explicitly notes that worker-launcher and worker-output-reintegration WUs are absent from this Phase 1 artifact. This forwards the line 3628 instruction correctly to the future VS-015/VS-018-owning artifacts: those downstream artifacts must add explicit incoming-from-Phase-0C edges to WU-0C-N1/N2/N3/N5 — but the `Blocked-on:` substrate-feature annotation is no longer needed on the Phase 1 outgoing side because the substrate features have landed in `agents`.

**Recommendation:** No action required.

**Oscillation classification:** None — `Blocked-on:` removal is the named r4 audit-history concern, and it is satisfied. No dependency-encoding regression — the substrate-feature dependency information has migrated entirely to the Phase 0C-r5 boundary owner (WU-0C-N3 acceptance criterion at line 1774 enforces `agents session schema-probe`; WU-0C-N3 contract at lines 1757–1761 documents `pause-handshake` / `resume-handshake` / `import-replace` substrate features).

---

### R4-DEP-F09. Critical Path is unchanged; r4 re-check note honest about Phase 0 critical incoming gates after WU-0C-N4 removal

**Severity: LOW**

The Critical Path block at `ai-roadmap-phase-1.md:2433–2449` declares the same 11-hop VS-001 acceptance path as r1/r2/r3: `WU-1-02 → WU-1-03 → WU-1-06 → WU-1-12 → WU-1-14 → WU-1-17 → WU-1-20 → WU-1-21 → WU-1-22 → WU-1-24 → WU-1-25`.

The r3 re-check note at line 2445 is preserved and amended for r4: "SessionOverrideContract added cross-phase prerequisites to WU-1-10 and read-only evidence/audit/display WUs, but no new Phase 1-local dependency edge. Round 4 removes WU-0C-N4 from that prerequisite set, so upstream readiness now requires WU-0C-N1/WU-0C-N2/WU-0C-N3/WU-0C-N5 before transcript-ingestion and override-derived evidence display are accepted."

The dedicated r4 re-check at line 2447: "Phase 0C-r5 removes WU-0C-N4 and makes SessionOverrideContract v2-only; the Phase 1-local critical path remains unchanged."

Verification:
- WU-1-10 is on Wave 3 but NOT on the declared critical path. The r3+r4 notes are correct that adding/removing cross-phase prerequisites on WU-1-10 does not change the Phase 1-local critical path (which runs through WU-1-12 ToolCallAuditEmitter for the VS-001 acceptance gate, not through WU-1-10).
- The r4 caveat "upstream readiness now additionally requires WU-0C-N1/WU-0C-N2/WU-0C-N3/WU-0C-N5" is honest scoping. It does not overclaim — it acknowledges that the cross-phase blocker tightens *when* WU-1-10 / override-derived display can be accepted, not that the Phase 1-local longest path changes. The list correctly omits WU-0C-N4 (which no longer exists in Phase 0C-r5).
- The Phase 0 critical incoming gates list at line 2449 is unchanged from r3 (no additional Phase 0 IDs are required for the critical path itself, since the path runs through audit / budget / render gates in Phase 0B/0C, not through SessionOverrideContract).

**Recommendation:** No action required.

**Oscillation classification:** None.

---

### R4-DEP-F10. D3 Run Report row honest about r4 four-check enumeration; D4 row honestly classifies r4 fix-created-family generation-0 reopen due to external Phase 0C-r5 cascade

**Severity: LOW**

The D3 row at `ai-roadmap-phase-1.md:2475` reads: "I checked WU count, internal dependency IDs, topological wave acyclicity, SessionOverrideContract cross-phase overlay, and removal of former block-on annotations by text scan. I did not run external reviewer tools or implementation tests because this is a roadmap artifact." The five named checks correspond to verifiable artifact properties (WU count = 56 verified in R4-DEP-F01; internal dependency IDs verified in R4-DEP-F01; topological wave acyclicity verified in R4-DEP-F07; SessionOverrideContract cross-phase overlay verified in R4-DEP-F02; block-on annotation removal verified in R4-DEP-F08). The disclaimer about implementation tests is appropriate. No overstatement — in particular, no claim of running external reviewer scripts or schema validators.

The D4 row at `ai-roadmap-phase-1.md:2476` reads: "bundling-family remains closed by the WU-1-45/WU-1-56 split; state-machine-criteria-family remains addressed by per-method criteria; dependency-encoding-family is updated for WU-0C-N1/WU-0C-N2/WU-0C-N3/WU-0C-N5 incoming edges; fix-created-family remains generation 0 for this externally-driven Round 4 cascade from proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5."

Per-family verification:

- **bundling-family:** R4-DEP-F01 confirmed WU-1-45/WU-1-56 split is preserved byte-for-byte from r2/r3; no new bundle introduced by r4 (no WU was merged or expanded). ✓ "closed" claim correct.
- **state-machine-criteria-family:** Spot-checked acceptance criteria on affected WUs:
   - WU-1-12 line 492–499: per-method binary criteria for `emit_tool_call_audit` plus override-link/no-mutation criteria preserved. ✓
   - WU-1-25 line 907–915: per-state binary criteria for view rendering plus no-mutation-control criteria. ✓
   - WU-1-45 line 1529–1536: per-method binary criteria for `run_redacted_provider_probe` plus six no-routing/no-quota/no-resume/no-mutation criteria. WU-1-45 line 1535 now reads "Any session-override compatibility shown beside provider state comes from WU-0C-N3/WU-0C-N5 evidence" (was WU-0C-N4/WU-0C-N5 in r3). ✓
   - WU-1-56 line 1564–1569: five per-state fixture criteria preserved from r2/r3. ✓
   "addressed by per-method criteria" claim correct.
- **dependency-encoding-family:** R4-DEP-F01, F02, F03, F04, F06, F07 above confirm the explicit-IDs + topological-waves discipline holds and now extends to the WU-0C-N1/N2/N3/N5 IDs (with N4 properly retired). The D4 row does not overstate — it claims "updated for WU-0C-N1/WU-0C-N2/WU-0C-N3/WU-0C-N5 incoming edges," which is exactly what is delivered. ✓
- **fix-created-family:** This is the third externally-driven brownfield in the Phase 1 local loop (after r2's WU-1-45/56 split and r3's SessionOverrideContract integration). The "remains generation 0" framing matches `plans/audit/ai-roadmap-phase-1.md:88` ("`fix-created-family` remains generation 0 in the Phase 1-local loop and externally driven. No same-family local oscillation is introduced by r4.") The r4 fix is tightly scoped (drop N4, substitute N3 for schema probe, remove `Blocked-on:` annotations, refresh overlay/Stitch Notes blocks) and does not create new local fixes elsewhere in the artifact (verified by R4-DEP-F01 internal-graph preservation, F06 non-N envelope preservation, F08 outgoing block preservation, F09 critical path preservation). ✓

The D4 row is honest about the externally-driven r4 reopen and does not inflate progress or overstate closure on any other family.

**Recommendation:** No action required.

**Oscillation classification:** `fix-created-family`, generation 0 in the Phase 1 local loop, externally driven (third externally-driven cascade in a row). No same-family oscillation, no two-generation in-gate.

---

### R4-DEP-F11. Two presentation observations carry forward from r1/r2/r3 unchanged (INFO)

**Severity: INFO**

The two earlier-round INFO observations remain factually accurate in r4 and require no r4 action:

1. **VS-002 WUs do not list IPC envelope primitives (WU-0A-05..08, WU-0C-34/35).** Re-checked at `ai-roadmap-phase-1.md:2082–2124` (VS-002 incoming). VS-002 incoming includes WU-0C-36/36a/37/37a but not WU-0C-34/35 — bidirectionally consistent with `ai-roadmap-phase-0c.md:3599` (Outgoing-to-Phase-1 for VS-002 omits IPC). Upstream-of-Phase-1 omission, not a Phase 1 dependency-encoding regression.
2. **VS-003 WUs do not list UI-shell primitives (WU-0A-11..13, WU-0C-36).** Re-checked at `ai-roadmap-phase-1.md:2126–2181` (VS-003 incoming). VS-003 incoming omits WU-0C-36/36a — bidirectionally consistent with `ai-roadmap-phase-0c.md:3600` (Outgoing-to-Phase-1 for VS-003 omits WU-0C-36). Upstream-of-Phase-1 omission.

Neither rises to MEDIUM in r4 for the same reason as in r1/r2/r3: both edges are absent from the engineering-roadmap and Phase 0C in the first place, so adding them in Phase 1 r4 would create unilateral asymmetry. The R4-DEP-F05 caveat about WU-0C-N1/N2/N3 producer-side enumeration is the analogous downstream-of-upstream-omission flag for the SessionOverrideContract layer.

**Recommendation:** No action required for the Phase 1 r4 dependency gate. Carry forward as a note for any future engineering-roadmap revision cycle.

**Oscillation classification:** None — same downstream-of-upstream-omission classification as r1/r2/r3.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-DEP-F01 | Internal Phase 1 graph remains acyclic; topological sort succeeds with 56 nodes; no Phase 1-local edge added or removed by the r4 cascade | LOW |
| R4-DEP-F02 | SessionOverrideContract incoming overlay correctly drops WU-0C-N4 and substitutes WU-0C-N3 schema_version_probe; 22 (WU-0C-N*, WU-1-XX) edges enumerated at three consistent locations; no dangling N4 reference remains | LOW |
| R4-DEP-F03 | Affected-WU SessionOverrideContract envelopes remain scoped to actual contract surface after WU-0C-N4 removal; the WU-0C-N3 substitute is exactly where schema-probe evidence is genuinely needed | LOW |
| R4-DEP-F04 | WU-1-10's r4 envelope (N1, N2, N3, N5) is correct: central transcript read seam still needs every non-N4 WU-0C-N* primitive; descendants like WU-1-13 receive only required subset | LOW |
| R4-DEP-F05 | Bidirectional consistency vs Phase 0C-r5 holds at the macro / generic-downstream layer; line 1842 explicitly authorizes N5→VS-001/VS-003; line 3586 generic clause + line 3628 standing instruction authorize the rest | LOW (with INFO caveat) |
| R4-DEP-F06 | Pre-existing Phase 0A/0B/0C non-N incoming edges remain byte-for-byte from r3 across every VS | LOW |
| R4-DEP-F07 | Parallelization Map remains a valid 7-wave topological partition (18+12+9+5+7+3+2 = 56); SessionOverrideContract edges are cross-phase, not Phase 1-local; r4 wave reshuffle = none | LOW |
| R4-DEP-F08 | Outgoing-To-Phase-2+ block preserves all 24 (Phase 1 VS, Phase 2+ VS) pairs; all prior `Blocked-on:` substrate-feature annotations absent in r4 (matching Phase 0C-r5 line 3586 substrate-feature landing) | LOW |
| R4-DEP-F09 | Critical Path is unchanged; r4 re-check note honest about Phase 0 critical incoming gates after WU-0C-N4 removal | LOW |
| R4-DEP-F10 | D3 Run Report row honest about r4 four-check enumeration; D4 row honestly classifies r4 fix-created-family generation-0 reopen due to external Phase 0C-r5 cascade | LOW |
| R4-DEP-F11 | Two presentation observations (VS-002 IPC, VS-003 UI shell) carry forward from r1/r2/r3 unchanged | INFO |

## What LOW requires

For this LOW rating to remain valid in any future revision, the following conditions must hold:

1. The internal Phase 1 dependency graph remains acyclic; every internal edge resolves to an earlier-wave WU. WU-1-56 remains a Wave 1 leaf; SessionOverrideContract edges remain cross-phase incoming, not Phase 1-internal.
2. Every Phase 1 WU continues to declare cross-phase incoming edges as specific WU-0A-NN / WU-0B-NN / WU-0C-NN / WU-0C-N* IDs. No vague references (e.g. "Phase 0 substrate", "SessionOverrideContract upstream") appear. **No `WU-0C-N4` token reappears anywhere in the artifact** (it is permanently retired in Phase 0C-r5).
3. Per-WU SessionOverrideContract envelopes remain scoped to actual contract surface: WU-1-10 keeps the N1/N2/N3/N5 envelope (no re-introduction of N4); WU-1-12/13/28 stay N5-only; WU-1-23/25 stay N2/N5; WU-1-45/48/49/50 stay N3/N5 (no re-introduction of N4); WU-1-03/04/05 stay N2-only. No WU silently broadens to claim more N* primitives than its contract reads.
4. The `Round 4 SessionOverrideContract incoming overlay` block in the Dependency Graph and the corresponding overlay block in Stitch Notes remain bidirectionally consistent with the per-WU `Cross-phase incoming (SessionOverrideContract):` lines. The 22-edge total is preserved unless a new Phase 1 surface legitimately requires a SessionOverrideContract incoming edge (which would also require a Phase 0C-side `Produces` update for symmetry).
5. For each Phase 1 r4 SessionOverrideContract edge, either (a) the corresponding Phase 0C `Produces` line for the named WU-0C-N* enumerates the Phase 1 consumer per VS (currently true only for N5 → VS-001 / VS-003 at `ai-roadmap-phase-0c.md:1842`), or (b) the line 3586 generic-downstream-Phase-1/2/3 clause and the line 3628 standing instruction continue to authorize generic-downstream consumers. If a future Phase 0C round removes either authorization, the Phase 1 r4 incoming declarations must be re-derived.
6. The Phase 0A/0B/0C non-N envelope (the r2-converged 76-internal-edge / per-VS incoming cross-phase block) remains byte-for-byte. If a future Phase 0X cell is added or removed, Phase 1 incoming updates in lockstep.
7. The Parallelization Map continues to be a valid topological-level partition: every WU's wave number equals 1 + max(wave number of declared internal parents). No "Parallelizable with" line lists a same-wave WU that is also a declared dependency. The 7-wave skeleton 18+12+9+5+7+3+2 = 56 is preserved.
8. The Outgoing-To-Phase-2+ block continues to enumerate exactly 24 (Phase 1 VS, Phase 2+ VS) pairs; WU-1-56 remains excluded from every VS-006 → Phase 2+ foundation set. **No `Blocked-on:` substrate-feature annotations reappear** unless a future engineering-roadmap revision establishes a new substrate-feature gap.
9. The D3 Regression-check row continues to describe only checks actually performed and does not overstate (e.g. claims of running implementation tests when only artifact assertions were performed). The current four-check enumeration (WU count, internal dependency IDs, topological wave acyclicity, SessionOverrideContract cross-phase overlay, removal of former block-on annotations) remains the honest description.
10. The D4 row continues to honestly classify each watch family. In particular: `bundling-family` remains "closed" (a re-fire would reopen it); `state-machine-criteria-family` remains "addressed"; `dependency-encoding-family` remains "addressed and updated for WU-0C-N1/WU-0C-N2/WU-0C-N3/WU-0C-N5 edges with N4 properly retired" (silent re-introduction of N4 or removal of the SessionOverrideContract overlay would break this claim); `fix-created-family` remains classified as "generation 0 externally-driven" until the next round closes it (a same-family local re-fire in r5+ would push it to generation 1 with a triggered hard-decompose per audit-history convention).
