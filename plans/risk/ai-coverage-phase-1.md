# AI — Coverage Risk Assessment (Phase 1, round 3)

**Rating: LOW**

Reviewer: `claude-opus`. Round 3 brownfield, cascade-driven from `proposal-r5` / `engineering-roadmap-r4` / Phase 0C-r4 (commits c9ee3f9, b58d8c9, ea47467).

## Scope and inputs

- Artifact under review: `product-strategy/ai-roadmap-phase-1.md` (3031 lines, 56 WUs — same WU count as the converged r2 artifact). `### WU-1-` count = 56; Inventory table at lines 39-97 lists 56 rows; Run Report D1 audit table (lines 2480-2537) lists 56 rows; Dependency Graph internal block (lines 1909-1965) lists 56 entries.
- Upstream cascade sources:
  - `product-strategy/proposal.md` (proposal-r5) §SessionOverrideContract Boundary lines 75-97 plus §SessionOverrideContract Trait lines 1562-1593 plus the v2 agent-runner feature register at lines 1719-1729.
  - `product-strategy/engineering-roadmap.md` (engineering-roadmap-r4) lines 623-678 (SessionOverrideContract foundation rows + Phase binding) and lines 824-856 (cross-phase dependency edges).
  - `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` lines 92, 139-143 (WU-0C-N1..N5 inventory) and lines 3645, 3655-3697 (Phase 0C-r4 Outgoing-to-Phase-1 + cross-phase outgoing edges added in round 4).
- Per-phase audit history: `plans/audit/ai-roadmap-phase-1.md` r1 = Decomp MEDIUM / Coverage LOW / Dep LOW; r2 = LOW/LOW/LOW (converged); r3 directive lines 71-79 dispatches three independent gates against the cascade integration with `fix-created-family` reopened at generation 0.
- Round 3 mandate: integrate Phase 0C-r4 SessionOverrideContract surface into Phase 1 without adding/removing/merging WUs. Coverage gate at r3 verifies that (a) refactored WUs retain functional ACs from r2, (b) the seven SessionOverrideContract operations are correctly consumed, (c) `Blocked-on` annotations name the agent-runner feature register surfaces from proposal-r5, (d) no orphan WUs were introduced, and (e) cross-phase incoming-from-Phase-0C remains systematic, including the new WU-0C-N* edges.

## Round 3 diff scope (verified)

`git diff d377716..ea47467 -- product-strategy/ai-roadmap-phase-1.md` shows the following touched WUs and structural sections, with no WU added, removed, merged, or renamed:

1. **Phase 1 Scope** preamble (lines 18-20): adds `SessionOverrideContract` to the consumed Phase 0C runtime list and adds a r3 read-only-by-default boundary statement that names the forbidden surfaces (open/locate/parse/rewrite/truncate/append/migrate per-CLI session JSONL; `agent-runner` provider routing, account selection, quota balancing, auth refresh, `--resume` composition, cross-provider porting, session-id capture).
2. **Round 3 SessionOverrideContract Audit** (lines 98-118): new section. 13-row table of (WU, r2 risk found, r3 refactor, SessionOverrideContract dependency); explicit list of 42 unaffected WUs.
3. **Per-WU acceptance-criteria + Dependencies blocks** for the following 13 WUs:
   - **VS-003 normalizers**: WU-1-03 ClaudeToolCallNormalizer (lines 197-204), WU-1-04 CodexToolCallNormalizer (lines 232-239), WU-1-05 OpencodeToolCallNormalizer (lines 267-274).
   - **VS-003 transcript reader**: WU-1-10 TranscriptIngestionAdapter (lines 426-447, including new `Blocked-on` line 447 and `Revision rationale` line 449).
   - **VS-003 audit/drilldown**: WU-1-12 ToolCallAuditEmitter (lines 494-501); WU-1-13 ToolCallDrilldownComponent (lines 528-537).
   - **VS-001 evidence/inspector**: WU-1-23 RenderEvidencePointerService (lines 842-850); WU-1-25 WorkingSetInspectorPane (lines 909-917); WU-1-28 RenderAuditSubscription (lines 1007-1015).
   - **VS-006 provider preflight**: WU-1-45 RedactedProviderProbeService (lines 1531-1538); WU-1-48 RouteEligibilityResolver (lines 1658-1665); WU-1-49 RouteDenialReasonClassifier (lines 1693-1700); WU-1-50 ProviderPreflightPane (lines 1728-1736).
4. **Round 3 SessionOverrideContract incoming overlay** under Dependency Graph (lines 2423-2433); **Critical Path** r3 re-check (line 2447); **Parallelization Map** r3 re-derive (line 2455); **Run Report** D3/D4 rows (lines 2473-2474); **Self-classification** r3 entry (line 2549); **Stitch Notes Round 3 SessionOverrideContract incoming edges** block (lines 2972-2982); **Outgoing-To-Phase-2+** Blocked-on additions for (VS-003, VS-018) and (VS-006, VS-015) (lines 2994, 3005); **Non-Ownership Notes** worker-launcher/reintegration sentinel (line 3031).

The 43 unchanged WUs (Shared WU-1-01; VS-001 reader/writer/render/label/budget/cache surfaces and budget/cost panels WU-1-14..WU-1-22, WU-1-24, WU-1-26, WU-1-27; all eight VS-002 summary-contract WUs WU-1-29..WU-1-36; all eight VS-005 configuration WUs WU-1-37..WU-1-44; VS-006 entitlement/capability writers WU-1-46/47 and FakeProviderProbeFixture WU-1-56; all five VS-007 WUs WU-1-51..WU-1-55; VS-003 supporting WUs WU-1-02, WU-1-06, WU-1-07, WU-1-08, WU-1-09, WU-1-11) carry byte-identical Acceptance-criteria blocks to r2. Verified by spot-check on the Wave-1 service WUs (WU-1-14 BudgetLedgerScopeWriter lines 565-572, WU-1-32 SummaryTemplateRegistry, WU-1-37 ConfigurationInspectorService, WU-1-51 InitiativeRootService, WU-1-53 AgentWalkStateConsumerAdapter) and on the WU-1-56 fixture pack (lines 1566-1571 — the five per-state binary criteria from R2-COV-F02 are preserved).

## Findings

### R3-COVERAGE-F01. Refactored WUs retain functional ACs from r2; r3 changes are additive boundary criteria

**Severity: LOW**

For each of the 13 affected WUs, the r2 method/state acceptance criteria are preserved verbatim and the r3 cascade only appends boundary criteria. The two-line systemic generic block ("Each declared method returns the documented Result/DTO type for valid inputs and a documented error variant for invalid inputs"; "Every declared enum or state-machine value is reached by at least one fixture or input path"; "Every documented invalid transition or denied state is rejected without mutating unrelated graph, provider, budget, or audit records") is retained on every method-bearing WU it appeared on in r2; named-method binary criteria (`normalize_claude_tool_call`, `normalize_codex_tool_call`, `normalize_opencode_tool_call`, `ingest_transcript`, `mark_partial_ingest`, `emit_tool_call_audit`, `list_render_evidence`, `open_render_evidence`, `subscribe_render_audit`, `acknowledge_render_audit_event`, `resolve_route_eligibility`, `classify_route_denial`) are retained verbatim; UI generics (`render states`, `reject unknown state`, `render pane states`, `reject unknown label`, `render provider states`) are retained verbatim; the WU-1-45 redaction/audit invariant block established in R2-COV-F03 (criteria 1-4: success path, denial path, redaction surface, exactly-one-audit-event) is retained verbatim; the single-concern-PR criterion is retained verbatim on every affected WU.

The added boundary criteria are uniformly framed as negative invariants ("never opens X", "never calls Y", "exposes no Z control") plus positive scope clauses ("consumes normalized evidence only", "displays override metadata only via opaque pointers"). They strengthen the existing boundary without rewriting any binary fixture assertion. Mapping per WU (cite WU-1-NN; r3 additions enumerated):

| WU | r2 functional ACs preserved | r3 boundary additions |
|---|---|---|
| WU-1-03 ClaudeToolCallNormalizer (l.197-204) | 3 systemic generics + `normalize_claude_tool_call` named method + single-concern PR | (a) consumes WU-0C-15a..15d / WU-0C-N2 normalized evidence only — never opens Claude JSONL paths, `state.db`, locator scripts, or provider-native session files; (b) never calls `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, or `append_turns` |
| WU-1-04 CodexToolCallNormalizer (l.232-239) | identical to WU-1-03 with `normalize_codex_tool_call` | (a) Codex variant of (a) above; (b) same forbidden-method list |
| WU-1-05 OpencodeToolCallNormalizer (l.267-274) | identical to WU-1-03 with `normalize_opencode_tool_call` | (a) opencode variant of (a) above; (b) same forbidden-method list |
| WU-1-10 TranscriptIngestionAdapter (l.426-435) | 3 systemic generics + `ingest_transcript` named method + `mark_partial_ingest` named method + single-concern PR | (a) reads through WU-0C-15d normalized turn evidence or WU-0C-N1 `read_transcript` / `get_session_metadata`, accepts `session_id` only — does not accept filesystem transcript paths as product inputs; (b) records unsupported schema, unsupported storage, busy session, missing session, and adapter refusal as evidence artifacts without fallback per-CLI JSONL parsing; (c) never calls `replace_transcript`, `truncate_after`, `append_turns` and never edits `agents` state rows |
| WU-1-12 ToolCallAuditEmitter (l.494-501) | 3 systemic generics + `emit_tool_call_audit` named method + single-concern PR | (a) audit events can link to WU-0C-N5 override registry records by opaque override ID when an upstream session read/refusal is the evidence source; (b) emitter never begins, commits, rolls back, quarantines, replaces, truncates, or appends session overrides — links only |
| WU-1-13 ToolCallDrilldownComponent (l.528-537) | 3 UI generics + `render states` named + `reject unknown state` named + single-concern PR | (a) view can show override receipt/refusal pointer from WU-0C-N5 as linked evidence without rendering raw JSONL/filesystem paths/adapter temp paths/provider-native bodies; (b) component exposes no control invoking `replace_transcript`, `truncate_after`, `append_turns`, provider reroute, resume, or session import/export |
| WU-1-23 RenderEvidencePointerService (l.842-850) | 3 systemic generics + `list_render_evidence` named + `open_render_evidence` named + single-concern PR | (a) pointers may include WU-0C-N5 override receipt/refusal IDs and WU-0C-N2 session metadata refs without exposing raw transcript paths or JSONL bodies; (b) opening override-derived evidence returns registered evidence/audit metadata only — never locates sessions or calls SessionOverrideContract mutation |
| WU-1-25 WorkingSetInspectorPane (l.909-917) | 3 UI generics + `render pane states` named + `reject unknown label` named + single-concern PR | (a) pane can display override receipt/refusal status from WU-0C-N5 while hiding raw per-CLI paths and provider-native bodies; (b) pane exposes no control for session import, transcript replacement, provider reroute, resume, or cross-provider porting |
| WU-1-28 RenderAuditSubscription (l.1007-1015) | 3 systemic generics + `subscribe_render_audit` named + `acknowledge_render_audit_event` named + single-concern PR | (a) stream may include SessionOverrideStore audit events from WU-0C-N5 by opaque override ID + operation + state + refusal reason + evidence ref; (b) acknowledgement updates Phase 1 subscription state only — never commits/rolls back/quarantines/imports/replaces/truncates/appends an override |
| WU-1-45 RedactedProviderProbeService (l.1531-1538) | 4 named-behavior service criteria from R2-COV-F03 (success, denial, redaction surface, exactly-one-audit-event) + single-concern PR | (a) probe does not select accounts, rebalance quota lanes, refresh auth, mutate provider configuration, compose `--resume`, port sessions, or infer per-CLI session storage support; (b) any session-override compatibility shown beside provider state comes from WU-0C-N4/WU-0C-N5 evidence — not from probe-owned JSONL/`state.db` inspection |
| WU-1-48 RouteEligibilityResolver (l.1658-1665) | 3 systemic generics + `resolve_route_eligibility` named + single-concern PR | (a) eligibility returns harness policy/readiness decisions only — never chooses concrete provider accounts, quota windows, auth profiles, resume strategies, session ports, or per-CLI storage adapters; (b) workload reqs may include need for session override capability by opaque WU-0C-N4/N5 refs but resolver does not probe or mutate sessions |
| WU-1-49 RouteDenialReasonClassifier (l.1693-1700) | 3 systemic generics + `classify_route_denial` named + single-concern PR | (a) denial reasons distinguish harness preflight policy/capability denials from `agent-runner` routing/quota/auth/resume/session-porting/SessionOverrideContract refusal reasons; (b) override refusal reasons referenced by opaque WU-0C-N5 registry/evidence refs — not recomputed from provider-native files |
| WU-1-50 ProviderPreflightPane (l.1728-1736) | 3 UI generics + `render provider states` named + `reject unknown state` named + single-concern PR | (a) pane displays override capability/refusal metadata from WU-0C-N4/N5 as read-only evidence and never exposes raw transcript paths or adapter temp files; (b) pane offers no account picker, quota-balancing control, auth refresh control, resume/import control, provider reroute command, or SessionOverrideContract mutation command |

Every "Revision rationale" paragraph the proposer added (WU-1-03 line 217, WU-1-04 line 252, WU-1-05 line 287, WU-1-10 line 449, WU-1-12 line 514, WU-1-13 line 550, WU-1-23 line 863, WU-1-25 line 930, WU-1-28 line 1028, WU-1-45 line 1551, WU-1-48 line 1678, WU-1-49 line 1713, WU-1-50 line 1749) restates the narrowing in plain prose; none weakens or replaces an r2 criterion. No method or state-machine value previously covered by a binary fixture assertion has been dropped.

**Recommendation:** No action required.

---

### R3-COVERAGE-F02. Seven SessionOverrideContract operations are consumed correctly: read paths landed, mutation paths fenced

**Severity: LOW**

The proposal-r5 `SessionOverrideContract` trait declares seven operations (proposal lines 1562-1593, plus `get_session_metadata` referenced in WU-0C-N1 acceptance criteria at phase-0c-ai-roadmap-r4 line 1720): `schema_version_probe`, `locate_session`, `read_transcript`, `get_session_metadata`, `replace_transcript`, `truncate_after`, `append_turns`. Phase 1 r3 must consume the read surface and fence the mutation surface. Coverage by Phase 1 WU:

| Operation | Direction in Phase 1 | Owning Phase 1 WU | Evidence |
|---|---|---|---|
| `schema_version_probe` | indirect (probe runs in WU-0C-N4; Phase 1 surfaces refusal evidence) | WU-1-45 / WU-1-48 / WU-1-49 / WU-1-50 cross-phase incoming WU-0C-N4 (l.1545, 1672, 1707, 1743); WU-1-10 cross-phase incoming WU-0C-N4 via WU-0C-N1..N5 dependency block (l.441) | Refusal/capability evidence is read-only; no Phase 1 WU runs the probe |
| `locate_session` | not invoked by Phase 1; explicitly forbidden | WU-1-03/04/05 forbid call (l.203, 238, 273); WU-1-23 forbids call (l.849) | No Phase 1 WU calls it; WU-1-10 passes `session_id` to WU-0C-N1 implementations and lets the adapter own location |
| `read_transcript` | invoked by exactly one Phase 1 WU (WU-1-10) | WU-1-10 (l.432) explicitly: "reads session content only through WU-0C-15d normalized turn evidence or WU-0C-N1 `read_transcript` / `get_session_metadata`" | Single-owner read path; normalizers WU-1-03/04/05 are pure transformers over the result |
| `get_session_metadata` | invoked by WU-1-10 only | WU-1-10 (l.432, same criterion) | Same single-owner property |
| `replace_transcript` | not invoked by Phase 1; explicitly forbidden | Forbidden in WU-1-03/04/05 (l.203, 238, 273), WU-1-10 (l.434), WU-1-13 (l.536), WU-1-25 (l.916), WU-1-28 (l.1014), WU-1-50 (l.1735); restated at Phase 1 Scope l.20 and Non-Ownership Notes l.3028 | All Phase 1 read-only |
| `truncate_after` | not invoked by Phase 1; explicitly forbidden | Forbidden in same WU set as `replace_transcript` | All Phase 1 read-only |
| `append_turns` | not invoked by Phase 1; explicitly forbidden | Forbidden in same WU set as `replace_transcript` | All Phase 1 read-only |

Additionally, the WU-0C-N5 `SessionOverrideStore` registry ("Workspace-level override ledger consumed by VS-001 evidence inspectors, VS-003 audit surfaces, VS-020 recovery, VS-021 reroute governance" — phase-0c-ai-roadmap-r4 l.1897) is wired into Phase 1's evidence/audit/display surfaces: WU-1-12, WU-1-13, WU-1-23, WU-1-25, WU-1-28, WU-1-45, WU-1-48, WU-1-49, WU-1-50 each take WU-0C-N5 as a SessionOverrideContract incoming edge (lines 508, 544, 857, 924, 1022, 1545, 1672, 1707, 1743). The WU-0C-N2 canonical DTO (`TranscriptTurn`, `SessionLocation`, `SessionMetadata`) is consumed by WU-1-03/04/05 for shape-only normalization (lines 211, 246, 281), by WU-1-10 as the canonical transcript shape (line 441), and by WU-1-23/25 for opaque inspector evidence pointers (lines 857, 924).

The single-owner property on `read_transcript`/`get_session_metadata` matters because it ensures the SessionOverrideContract read entrypoint cannot drift into per-CLI normalizers (WU-1-03/04/05 forbid the call explicitly) or into UI display surfaces (WU-1-13, WU-1-25, WU-1-28, WU-1-50 forbid mutation and never invoke the read entrypoint themselves; they consume already-normalized evidence pointers). This matches Phase 0C-r4's design that WU-0C-N1 mutation methods are exclusive to WU-0C-N1 implementers (phase-0c-ai-roadmap-r4 l.1313).

**Recommendation:** No action required.

---

### R3-COVERAGE-F03. `Blocked-on` annotations name exactly the agent-runner v2 feature register from proposal-r5

**Severity: LOW**

Proposal-r5 lines 1719-1729 enumerate the `agent-runner` v2 features that unlock the v2 SessionOverrideContract adapter and remove v1 idle-only constraints: `agents session locate <id>`, `agents session export <id>`, `agents session import-replace <id>`, plus `agents pause-handshake` (proposal §"Concurrent transcript override during an in-flight `agents` write" l.1489), the `--session-id` forced-flag mechanism (engineering-roadmap-r4 l.723), and the `schema_version_probe` precondition. Phase 1 r3 must name these surfaces exactly where they are blocking; Coverage gate confirms by-name match.

| Phase 1 site | Annotation | Match against proposal/engineering-roadmap feature register |
|---|---|---|
| WU-1-10 `Blocked-on` (l.447) | "v1 read behavior can use WU-0C-N3 after schema probe. v2 adapter migration is blocked on `agents session locate` and `agents session export`; this WU is not blocked on `agents session import-replace` or `agents pause-handshake` because Phase 1 transcript ingestion is read-only." | Exact match. v2 read migration correctly names the two read-side commands and explicitly excludes the two write-side commands as inapplicable. The schema-probe precondition is correctly attributed to WU-0C-N3/WU-0C-N4. |
| Outgoing-to-Phase-2+ (VS-003, VS-018) `Blocked-on` (l.2994) | "accepted worker-output session write-back must consume WU-0C-N1..WU-0C-N5; v2 adapter migration needs `agents session locate / export / import-replace`; atomic mid-session reintegration needs `agents pause-handshake`." | Exact match. Reintegration crosses the write boundary, so all three session subcommands plus pause-handshake are correctly named. |
| Outgoing-to-Phase-2+ (VS-006, VS-015) `Blocked-on` (l.3005) | "the downstream worker launcher remains thin and must spawn `agents -m <model> -p <project> -f <prompt>` while capturing the spawned `session_id` via the `agents` `--session-id` forced-flag mechanism and storing it on `WorkerRun`; v2 SessionOverrideContract adapter migration needs `agents session locate / export / import-replace`." | Exact match. The thin-launcher invocation pattern matches engineering-roadmap-r4 l.723; the `--session-id` forced-flag matches the agent-runner capture mechanism; v2 migration features match the three session subcommands. |
| Round 3 SessionOverrideContract overlay (l.2425, 2433) | "WU-0C-N1..WU-0C-N5 explicitly feed Phase 1 worker-launcher / worker-output-reintegration successors. This Phase 1 r3 artifact wires the VS-001/VS-003/VS-006 read-only portions now and leaves VS-015/VS-018 mutation dependencies to their owning future phase artifacts." | Matches phase-0c-ai-roadmap-r4 l.3684: "SessionOverrideContract WU-0C-N1..WU-0C-N5 -> Phase 1 worker-launcher and worker-output-reintegration WUs for override receipt display, refusal surfacing, and accepted-output write-back prerequisites." Phase 1 r3 correctly carries this forward as an outgoing/downstream block because the worker-launcher and worker-output-reintegration WUs are absent from this 56-WU artifact. |

No `Blocked-on` annotation in Phase 1 r3 names a feature that does not exist in the proposal-r5 / engineering-roadmap-r4 register, and no agent-runner feature listed in the register that is materially blocking on Phase 1 reads is missing from a `Blocked-on` annotation. The pause-handshake exclusion in WU-1-10 is load-bearing: it preserves Phase 1's read-only-by-default property by refusing to inherit a write-side block.

**Recommendation:** No action required.

---

### R3-COVERAGE-F04. No orphan WUs introduced; the four canonical inventories agree on 56

**Severity: LOW**

Coverage requires every Phase 1 WU to appear in (a) the per-VS Inventory table, (b) the per-WU ownership table, (c) the Run Report D1 audit table, (d) the Dependency Graph internal block, and (e) exactly one wave of the Parallelization Map. r3 changed none of these structurally; the round-2 audit had verified all five at 56. Re-verified for r3:

| Inventory surface | Location | Count |
|---|---|---:|
| Per-VS Inventory table | lines 28-37 | 1 + 12 + 6 + 9 + 8 + 8 + 7 + 5 = **56** |
| Per-WU ownership table | lines 39-97 | **56** rows (WU-1-01..WU-1-56, with WU-1-56 inserted between WU-1-45 and WU-1-46 per the r2 split) |
| Round 3 SessionOverrideContract Audit table | lines 102-116 | 13 affected WUs explicitly enumerated; remaining **43** unaffected WUs explicitly listed inline at line 118 → 13 + 43 = **56** |
| Run Report D1 audit table | lines 2480-2537 | **56** rows; explicit assertion `D1 audit count: 56 rows = 56 WUs` at line 2478 |
| Dependency Graph internal block | lines 1909-1965 | **56** entries; one "WU-1-NN <- ..." line per WU |
| Parallelization Map waves | lines 2459-2465 | 18 + 12 + 9 + 5 + 7 + 3 + 2 = **56**; spot-check confirms WU-1-56 in Wave 1 and WU-1-50 in Wave 6 (matching r2's wave assignments) |
| Critical Path | l.2440 | unchanged 11-WU path (WU-1-02 → WU-1-25); does not touch any of the 13 cascade-affected WUs except via WU-1-25 which retains its r2 dependencies |

The 13 cascade-affected WUs all appear in their correct waves with no reshuffle: WU-1-03/04/05 in Wave 2, WU-1-10 in Wave 3, WU-1-12 in Wave 4, WU-1-13 in Wave 5, WU-1-23 in Wave 2, WU-1-25 in Wave 7, WU-1-28 in Wave 5, WU-1-45 in Wave 1, WU-1-48 in Wave 4, WU-1-49 in Wave 5, WU-1-50 in Wave 6. The proposer's claim at l.2455 ("the topological waves remain 18+12+9+5+7+3+2 because SessionOverrideContract edges are cross-phase incoming gates, not new Phase 1-local edges") is correct: every WU-0C-N* dependency is in the cross-phase block, not the Phase 1 internal block, and the 56-entry Phase 1-local graph at lines 1909-1965 has identical edges to r2.

**Recommendation:** No action required.

---

### R3-COVERAGE-F05. Cross-phase incoming-from-Phase-0C is systematic and bidirectionally consistent with Phase 0C-r4

**Severity: LOW**

Coverage at r3 requires every Phase 0C-r4 outgoing-to-Phase-1 edge to land somewhere in this artifact and every Phase 1 cross-phase incoming claim to be reachable on the Phase 0C-r4 side. The new SessionOverrideContract cascade adds five upstream WUs (WU-0C-N1..WU-0C-N5) with one cross-phase outgoing block (phase-0c-ai-roadmap-r4 lines 3645, 3684-3687); Phase 1 r3 must echo these.

**Phase 0C-r4 → Phase 1 r3 forward check.** Phase 0C-r4 l.3645 declares "SessionOverrideContract WU-0C-N1..WU-0C-N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata." Phase 1 only owns evidence/audit/display read-side consumers, so the relevant landings are WU-1-03/04/05 (canonical TranscriptTurn shape), WU-1-10 (read entrypoint), WU-1-12/13/28 (audit + display of override receipts), WU-1-23/25 (evidence pointer + inspector), WU-1-45/48/49/50 (provider-side capability/refusal display). All ten landing WUs declare a `Cross-phase incoming (SessionOverrideContract): WU-0C-N*` line listing exactly the WU-0C-N IDs they consume:

| Phase 1 WU | Declared SessionOverrideContract incoming | Phase 0C-r4 outgoing block agreement |
|---|---|---|
| WU-1-03 (l.211), WU-1-04 (l.246), WU-1-05 (l.281) | WU-0C-N2 only | WU-0C-N2 `TranscriptTurn`/`SessionLocation`/`SessionMetadata` DTOs (phase-0c-ai-roadmap-r4 l.1582, "Produces: Canonical session-override DTOs consumed by ... VS-010, VS-012, VS-018, VS-020, VS-021"); proposer correctly extends "VS-003 normalizers" as a Phase 1 read-shape consumer in the r3 stitch overlay l.2974-2976 |
| WU-1-10 (l.441) | WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N4, WU-0C-N5 | All five upstream WUs are needed by a read-only ingestion adapter: trait (N1), DTOs (N2), v1 adapter (N3), schema probe (N4), override registry (N5). r3 stitch overlay l.2977 echoes the exact same five edges. |
| WU-1-12 (l.508), WU-1-13 (l.544), WU-1-28 (l.1022) | WU-0C-N5 only | WU-0C-N5 SessionOverrideStore registry "consumed by ... VS-001 evidence inspectors, VS-003 audit surfaces" (phase-0c-ai-roadmap-r4 l.1897). VS-001 evidence stream → WU-1-28; VS-003 audit surfaces → WU-1-12 (emit) + WU-1-13 (drilldown). r3 overlay l.2978 cites these three pairs. |
| WU-1-23 (l.857), WU-1-25 (l.924) | WU-0C-N2, WU-0C-N5 | Inspector evidence pointers need both canonical DTO shape (N2) and override receipts (N5). r3 overlay l.2979 cites both. |
| WU-1-45 (l.1545), WU-1-48 (l.1672), WU-1-49 (l.1707), WU-1-50 (l.1743) | WU-0C-N4, WU-0C-N5 | Provider preflight needs schema-probe (N4) and refusal-registry (N5) for capability/refusal display. r3 overlay l.2980 cites all four pairs. |

**Phase 1 r3 → Phase 0C-r4 reverse check.** Every cross-phase incoming edge declared in Phase 1 r3 maps back to a Phase 0C-r4 WU that exists. The legacy non-N edges (WU-0A-*, WU-0B-*, WU-0C-04, WU-0C-05..WU-0C-37 etc.) are unchanged from the r1 systematic enumeration; r2 left them byte-identical; r3 leaves them byte-identical. The new WU-0C-N1..N5 IDs all exist in phase-0c-ai-roadmap-r4 (lines 1688, 1582, 1788, 1740, 1847 respectively).

**Stitch Notes incoming-from-Phase-0C systematic enumeration.** The r1-established Stitch Notes "Incoming From Phase 0C" block (lines 2735-2970, ~236 (WU-0C-*, VS-*) pairs grouped by VS) is unchanged. The r3 cascade adds a separate "Round 3 SessionOverrideContract incoming edges" sub-block (lines 2972-2980) with 18 explicit (WU-0C-N*, WU-1-NN) pairs — encoded at WU-level rather than VS-level because the SessionOverrideContract overlay is sparser than the foundation-row overlay and benefits from explicit per-WU naming. The bidirectional consistency note at l.2982 correctly observes that worker-launcher and worker-output-reintegration cross-phase edges are absent from this artifact and are carried forward as outgoing/downstream blockers.

**Phase 0C-r4 cross-phase outgoing edges added in round 4 not covered by Phase 1 r3.** Phase 0C-r4 l.3686-3687 names additional cross-phase outgoing edges to Phase 2 turn-decomposition / detail-injection-router and Phase 3 repack-planner consumers. None of these are Phase 1 owned; the r3 artifact correctly does not encode them, leaving them to Phase 2 / Phase 3 r-N artifacts.

**Recommendation:** No action required.

---

### R3-COVERAGE-F06. Engineering-roadmap Phase 1 row coverage unchanged; r3 is scope-narrowing only

**Severity: LOW**

The engineering-roadmap Phase 1 section (lines 53-192) defines seven value slices (VS-001..VS-007) plus the foundation-table rows from lines 23-46. r2 closed coverage at LOW with every engineering-roadmap row owned by ≥1 WU. r3 makes no change to the per-VS Inventory table (lines 28-37), the per-VS "What is new" coverage (engineering-roadmap lines 56-191 unchanged on the consuming side), or the foundation-row partition (engineering-roadmap lines 26-46 + the round-4 SessionOverrideContract row at line 36). The r3 cascade's only structural addition is the boundary that (i) Phase 1 does not redefine SessionOverrideContract; (ii) Phase 1 reads override evidence read-only via WU-0C-N1..N5; (iii) accepted future write-back is downstream (VS-015 worker dispatch, VS-018 worker-output reintegration). All three are scope-narrowing rather than scope-changing — they confirm what was implicit in r2.

The "fake provider probes" foundation-row item from R2-COV-F04 (engineering-roadmap line 170) remains owned by WU-1-56 unchanged. The five-state probe taxonomy (ready, degraded, blocked, stale, probe_failed) on WU-1-56 (lines 1567-1571) preserves all five binary criteria.

**Recommendation:** No action required.

---

### R3-COVERAGE-F07. INFO sub-findings carried forward from R2-COV (untouched by r3 cascade)

**Severity: INFO**

R2-COV-F02 INFO: WU-1-56 FakeProviderProbeFixture lacks the WU-1-35-style fixture-pack systemic lines ("fixtures resolve only through declared contract IDs," "invalid fixtures fail with documented contract errors," single-concern PR). r3 does not touch WU-1-56 (it is in the unaffected-WU list at line 118), so this consistency-tightening note is preserved unchanged. Not blocking; not a coverage gap because the five per-state criteria carry the load-bearing no-credential-material invariant.

R2-COV-F03 INFO: WU-1-45 RedactedProviderProbeService lacks the three D2 systemic generic lines that other VS-006 service WUs carry. r3 does not touch this stylistic asymmetry; it adds two boundary criteria (the "no account selection / quota / auth / resume / port / storage" invariant and the "session-override compatibility from WU-0C-N4/N5 only" invariant) on top of the four r2 named-behavior criteria. Coverage remains substantively complete via the five behavior criteria + WU-1-56's exhaustive five per-state taxonomy. Not blocking.

R3 does not introduce any new INFO sub-findings that would warrant a brownfield round.

**Recommendation:** Optional consistency tightening from R2-COV remains optional. Not blocking.

---

## Oscillation classification

Per audit-history rule for Phase 1 r3 (`fix-created-family` reopened at generation 0, externally driven by the proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 cascade):

- `fix-created-family` is at generation 0 in the Phase 1 local loop because r2 contained no worker-launcher, worker-output-reintegration, or orchestrator-bridge mutation WU; the cascade narrows existing read-only WUs and records downstream blockers, rather than introducing a new bundle or fixing a Phase 1-local defect. From the Coverage perspective, no fix-created drift — the boundary criteria are additive to r2 ACs, no r2 functional AC was rewritten or weakened, and no new bundle was created (R3-COVERAGE-F01 + R3-COVERAGE-F04 verified). **`fix-created-family` closes at generation 0 in the Phase 1 local loop after this gate review.**
- `bundling-family` does not re-fire. r3 added boundary criteria to existing concerns without merging or splitting any WU; the WU-1-45 / WU-1-56 split established in r2 is preserved. The 56-WU partition holds. **`bundling-family` remains closed at generation 1 in the Phase 1 local loop.**
- `state-machine-criteria-family` does not re-fire. The five-state probe taxonomy (WU-1-56), the ten-state ImposedRenderLabel (WU-1-01), the SummaryContract validation states (WU-1-29/30/31/32), the BudgetLedger budget_state/policy_action enums (WU-1-14), and the side_effect/approval/protocol/state taxonomies (WU-1-08/09/02) are all unchanged. The new boundary criteria do not introduce new enums or states. **`state-machine-criteria-family` remains closed.**
- `dependency-encoding-family` does not re-fire. The 18 new (WU-0C-N*, WU-1-NN) edges are explicitly enumerated as a Round 3 SessionOverrideContract incoming overlay (line 2423) and as a Stitch Notes Round 3 SessionOverrideContract incoming-edge block (line 2972), with bidirectional consistency to phase-0c-ai-roadmap-r4 verified per-edge in R3-COVERAGE-F05. The 56-WU Phase 1-local graph is unchanged; the 7-wave Parallelization Map is unchanged. **`dependency-encoding-family` remains closed.**

No same-label oscillation. No fix-created. No two-generation in-gate. The r3 cascade integrates cleanly from the Coverage perspective.

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R3-COVERAGE-F01 | 13 cascade-affected WUs retain r2 functional ACs verbatim; r3 changes are additive boundary criteria (negative invariants + opaque-pointer scoping) | LOW |
| R3-COVERAGE-F02 | All seven SessionOverrideContract operations (`schema_version_probe`, `locate_session`, `read_transcript`, `get_session_metadata`, `replace_transcript`, `truncate_after`, `append_turns`) consumed correctly: WU-1-10 owns the read entrypoint; mutation operations explicitly forbidden across the 13 read-only surfaces | LOW |
| R3-COVERAGE-F03 | `Blocked-on` annotations name exactly the agent-runner v2 feature register from proposal-r5 lines 1719-1729 (`agents session locate / export / import-replace`, `agents pause-handshake`, `--session-id` forced-flag) with read/write-side discrimination intact | LOW |
| R3-COVERAGE-F04 | No orphan WUs: 56 in per-VS inventory, 56 in per-WU table, 56 in Run Report D1 audit, 56 in Dependency Graph internal block, 18+12+9+5+7+3+2 = 56 across waves; 13 affected + 43 unaffected = 56 in r3 audit table | LOW |
| R3-COVERAGE-F05 | Cross-phase incoming-from-Phase-0C systematic and bidirectionally consistent with phase-0c-ai-roadmap-r4: 18 explicit (WU-0C-N*, WU-1-NN) edges echoed in Stitch Notes; legacy ~236 (WU-0C-*, VS-*) pairs unchanged; downstream worker-launcher/reintegration edges correctly carried forward as outgoing blockers | LOW |
| R3-COVERAGE-F06 | Engineering-roadmap Phase 1 row coverage unchanged; r3 is scope-narrowing only (no new VS, no new "What is new" item, no foundation row redefined) | LOW |
| R3-COVERAGE-F07 | R2-COV-F02 / R2-COV-F03 INFO sub-findings carried forward (WU-1-56 missing fixture-pack systemic lines; WU-1-45 missing D2 systemic generics); r3 does not touch either | INFO |

## What LOW requires

For this round-3 LOW rating to remain valid, the conditions from R2-COV's "What LOW requires" continue to hold, plus three r3-specific additions:

1. **The 13 cascade-affected WUs retain every r2 functional AC byte-identical.** Removing any named-method binary line (`normalize_*_tool_call`, `ingest_transcript`, `mark_partial_ingest`, `emit_tool_call_audit`, `list_render_evidence`, `open_render_evidence`, `subscribe_render_audit`, `acknowledge_render_audit_event`, `resolve_route_eligibility`, `classify_route_denial`) or any UI generic (`render states`, `reject unknown state`, `render pane states`, `reject unknown label`, `render provider states`) or any of the WU-1-45 four named-behavior criteria (success path, denial path, redaction surface, exactly-one-audit-event) would re-fire `state-machine-criteria-family`.
2. **The seven SessionOverrideContract operations stay correctly fenced.** `read_transcript` and `get_session_metadata` invocations remain exclusive to WU-1-10. The four mutation methods (`replace_transcript`, `truncate_after`, `append_turns` plus by extension the WU-0C-N5 lifecycle commit/rollback/quarantine paths from the WU-1-12/28 negative criteria) remain explicitly forbidden in the 13 affected WUs and remain absent from the artifact entirely. Any future Phase 1 brownfield that introduces a write path on a Phase 1 surface must instead block on a downstream artifact owning the worker-launcher or worker-output-reintegration WU; doing it inside Phase 1 would re-fire `fix-created-family` at generation 1.
3. **`Blocked-on` annotations stay aligned with the agent-runner feature register.** Future revisions to WU-1-10 `Blocked-on` (l.447), the (VS-003, VS-018) Outgoing-to-Phase-2+ block (l.2994), and the (VS-006, VS-015) Outgoing-to-Phase-2+ block (l.3005) must keep naming the exact `agents session locate / export / import-replace`, `agents pause-handshake`, and `--session-id` forced-flag surfaces. If the upstream `agent-runner` feature register changes (proposal-r6 or later), these annotations must be re-derived rather than patched.

If a future cascade adds new WU-0C-N* IDs or new SessionOverrideContract operations, the Round 3 SessionOverrideContract incoming overlay (l.2423-2433) and the Stitch Notes Round 3 SessionOverrideContract incoming-edge block (l.2972-2982) must be re-derived against the upstream artifact rather than patched edge-by-edge — same systematic-from-start posture that closed r1 dependency-encoding.

The r3 cascade integrates cleanly. Coverage gate verdict for round 3: **LOW**.
