# AI — Decomposition Risk Assessment (Phase 1)

**Rating: LOW**

Round-3 brownfield cascade integrating proposal-r5 / engineering-roadmap-r4 / Phase 0C-r4 `SessionOverrideContract` axiom into the converged 56-WU Phase 1 artifact (`product-strategy/ai-roadmap-phase-1.md`, 3031 lines). No WU was added, removed, merged, split, or renamed in r3; the WU count remains 56 with the same 7-wave 18+12+9+5+7+3+2 partition. Thirteen affected WUs (WU-1-03/04/05/10/12/13/23/25/28/45/48/49/50) gained narrowing read-only acceptance criteria plus declared `Cross-phase incoming (SessionOverrideContract)` dependencies on subsets of WU-0C-N1..WU-0C-N5. Per-schema-object granularity (D1), binary acceptance criteria (D2), test/code separation, and single-concern PR-ability are preserved. The proposer self-classifies r3 as `fix-created-family` gen 0, externally driven by the upstream cascade rather than by a local reviewer-discovered defect, and the audit history (`plans/audit/ai-roadmap-phase-1.md` round-3 entry) corroborates this scoping.

The decomposition rubric is applied per the audit instruction: per-schema-object ownership, binary criteria, no blob WUs created by the cascade, refactored-WU coherence, and Stitch Notes preservation. Coverage and Dependency are intentionally out of scope and are left to the parallel `ai-coverage-phase-1` and `ai-dependency-phase-1` gates.

## Findings

### R3-DECOMP-F01. No blob WU created by the SessionOverrideContract cascade

**Severity: NONE**

The r3 cascade introduces a new architectural axiom (`SessionOverrideContract` is the only harness write-back path into an `agents`-owned transcript). A naive reaction would be to add a single Phase 1 owner for transcript override / worker reintegration / orchestrator-bridge mutation. The artifact does not do this:

- The "Round 3 SessionOverrideContract Audit" table (`product-strategy/ai-roadmap-phase-1.md` lines 100-118) explicitly states: "no WU in this Phase 1 artifact owns worker launch, worker-output reintegration, orchestrator turn execution, or session transcript mutation."
- The Run Report self-classification (lines 2548-2549) reaffirms: "Phase 1 r2 contains no worker-launcher, worker-output-reintegration, or orchestrator-bridge mutation WU, so the refactor narrows read-only transcript/provenance/provider surfaces and records downstream blockers instead of collapsing WUs."
- The Non-Ownership Notes (lines 3025-3031) declare: "Worker-launcher WUs and worker-output-reintegration WUs are absent from this Phase 1 artifact. When downstream artifacts introduce them, launchers must not manipulate session storage paths and reintegration WUs that mutate orchestrator session content must depend on WU-0C-N1 and the active adapter (WU-0C-N3 until v2)."
- Engineering-roadmap-r4 lines 663-671 bind the override-mutation owners to Phase 0C-r4 (`AgentRunnerDbAdapter` v1 + trait) and to downstream value slices (VS-010, VS-012, VS-018, VS-020, VS-021), explicitly carving Phase 1 out of mutation: "Phase 1 | Evidence/audit inspector support for override receipts and refusal reasons, so VS-001/VS-003 can show what was imposed or refused without teaching UI code per-CLI storage details." That carve-out is the same boundary the artifact enforces.

The 13 affected WUs are narrowed in place rather than collapsed into a new omnibus owner. WU count, parent-slice mapping, and the WU-1-45 / WU-1-56 split (closed in r2) all stay byte-stable, as the Run Report D1 audit table at lines 2480-2538 confirms (56 rows, one per WU, parent slices unchanged).

**Recommendation:** No action required.

---

### R3-DECOMP-F02. Per-schema-object granularity (D1) preserved across all 13 affected WUs

**Severity: NONE**

Each narrowing WU still maps to one schema object, service, adapter, or method-bearing UI surface, matching the systemic-from-start D1 pattern that closed Phase 0C r3. Spot check by category:

| Category | Affected WUs | Owns | D1 verdict |
|---|---|---|---|
| Provider-shape normalizer (one per CLI) | WU-1-03 (Claude), WU-1-04 (Codex), WU-1-05 (opencode) | one method `normalize_*_tool_call(...) -> ToolCallNormalizedEvent` (lines 190, 225, 260) | Each CLI normalizer remains its own WU; r3 narrows the method's permitted inputs to "already normalized turn/hook/trace evidence" without bundling. |
| Read-only adapter / ingester | WU-1-10 TranscriptIngestionAdapter | `ingest_transcript(...)`, `mark_partial_ingest(...)` (line 419) | Two methods on one adapter, consistent with prior multi-method services such as WU-1-06 (ToolCallProvenanceWriter: `write_tool_call_provenance` + `reconcile_tool_call`). Concern is "session-derived evidence" only; no second concern absorbed. |
| Backend service narrowing | WU-1-45 RedactedProviderProbeService, WU-1-48 RouteEligibilityResolver, WU-1-49 RouteDenialReasonClassifier | `run_redacted_provider_probe`, `resolve_route_eligibility`, `classify_route_denial` | Each owns one method; r3 adds negative-scope guards forbidding routing/quota/auth/session-storage ownership but does not introduce a new method or DTO. |
| Audit / evidence linkage | WU-1-12 ToolCallAuditEmitter, WU-1-23 RenderEvidencePointerService, WU-1-28 RenderAuditSubscription | one method (or two, in WU-1-23/28) over existing audit/evidence objects | r3 adds the ability to *link* WU-0C-N5 override receipts/refusals as opaque pointers, not the ability to manage them. Single concern preserved. |
| UI surface | WU-1-13 ToolCallDrilldownComponent, WU-1-25 WorkingSetInspectorPane, WU-1-50 ProviderPreflightPane | one named view-model with declared fields | r3 explicitly forbids each pane/component from exposing controls for session import / replace / truncate / append / reroute / resume. UI WUs remain display-only, matching the Phase 1 scope contract at lines 16-22. |

The Run Report (line 2471) records: "D1 per-object granularity | Applied | 56 WUs; service objects, DTOs, enum taxonomies, adapters, UI components, and fixture packs are separate rather than bundled per VS." The r3 audit does not add or remove a WU, so the D1 inventory is unchanged from r2's converged state.

**Recommendation:** No action required.

---

### R3-DECOMP-F03. Binary acceptance criteria (D2) preserved; new SessionOverrideContract scope guards are binary and testable

**Severity: NONE**

D2 is verified per Contract method/enum/state-machine, plus the new negative-scope guards introduced in r3. Each affected WU's revised acceptance criteria are binary and bind to the test boundary:

- **WU-1-03/04/05** (lines 198-204, 233-239, 268-274): each retains its method-level binary criterion (`normalize_*_tool_call has a binary fixture assertion covering its valid path and its documented invalid or denied path`) and adds two binary scope guards: "consumes normalized SessionTurnRef/TranscriptTurn/hook/trace/rollout evidence ... never opens [CLI] JSONL paths, state.db, locator scripts, or provider-native session files" and "never calls `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, or `append_turns`." Each is verifiable by a contract-level fake of WU-0C-N1 / WU-0C-N2 that records call sites and asserts the forbidden methods are not invoked.
- **WU-1-10** (lines 426-435): adds three binary criteria — read path restricted to WU-0C-15d normalized turn evidence or WU-0C-N1 `read_transcript` / `get_session_metadata`, `session_id` is the lookup key with no filesystem path inputs, refusal evidence emitted as evidence artifacts without per-CLI JSONL fallback, and no calls to mutation methods. Each is binary and testable in fixture form against the WU-0C-N1 / WU-0C-N3 fakes that Phase 0C-r4 owns.
- **WU-1-12** (lines 499-501): adds two binary criteria — `tool-call audit events can link to WU-0C-N5 override registry records by opaque override ID`, and the emitter `never begins, commits, rolls back, quarantines, replaces, truncates, or appends session overrides`. Both are observable on the AuditEvent fixture and on a fake override registry recorder.
- **WU-1-13/25/50** (lines 535-536, 915-916, 1734-1735): each UI WU adds two binary fixture-backed criteria — the view *can* surface override receipt/refusal pointers as registered evidence; the view *exposes no control* that invokes any mutation/reroute/import/export/resume/account-picker/auth/quota command. The "exposes no control" criterion is testable by a UI fixture that asserts the rendered DOM/IPC surface contains no handler bound to the forbidden command names.
- **WU-1-23/28** (lines 848-849, 1013-1014): same pattern — additive evidence-pointer / audit-stream support for override metadata, plus a binary criterion that acknowledgement / opening evidence does not commit, roll back, replace, truncate, append, or locate sessions.
- **WU-1-45/48/49/50** (lines 1532-1537, 1663-1664, 1698-1699, 1734-1735): each adds binary criteria distinguishing harness preflight from `agent-runner` ownership domains (account selection, quota balancing, auth refresh, resume composition, cross-provider porting, per-CLI storage adapters, override mutation), and references override capability/refusal evidence by opaque WU-0C-N4 / WU-0C-N5 IDs only.

The Run Report (line 2472) records: "D2 binary criteria | Applied | Each WU has method, enum, state-machine, or fixture-state criteria plus binary fixture assertions generated from its Contract." That holds for the r3 additions: every new bullet has a yes/no shape ("X is the only path", "Y is never called", "no control invoking Z", "links by opaque ID only"). No subjective phrase ("the adapter is robust", "the pane is clean") was introduced. `state-machine-criteria-family` does not re-fire.

**Recommendation:** No action required.

---

### R3-DECOMP-F04. Refactored-WU coherence — each affected WU still maps to one single-concern PR

**Severity: NONE**

The audit instruction ("whether affected WUs are truly thin/read-only") maps to the decomposition single-concern test from `roadmap-risk-types.md` lines 333-335. For each affected WU, the concern statement still passes the "describable in one sentence as 'add [one thing]'" test:

| WU | Single concern (post-r3) | Single-concern PR? |
|---|---|---|
| WU-1-03 | "normalize Claude-shaped tool evidence into ToolCallNormalizedEvent" | Yes — one method, one DTO output, no second concern. |
| WU-1-04 | "normalize Codex-shaped tool evidence into ToolCallNormalizedEvent" | Yes. |
| WU-1-05 | "normalize opencode-shaped tool evidence into ToolCallNormalizedEvent" | Yes. |
| WU-1-10 | "ingest session-derived tool-call evidence read-only and emit refusal evidence" | Yes — `ingest_transcript` + `mark_partial_ingest` are the read-and-record halves of one ingestion concern, mirroring prior precedents (e.g., WU-1-06's write+reconcile pair). |
| WU-1-12 | "emit tool-call audit events, optionally linking override refs" | Yes — only AuditEvent emission; override lifecycle stays in WU-0C-N5. |
| WU-1-13 | "render the tool-call drill-down view from contract fixtures" | Yes — display only. |
| WU-1-23 | "list/open render evidence pointers, including opaque override pointers" | Yes — read path only. |
| WU-1-25 | "render the working-set inspector pane from contract fixtures" | Yes — display only. |
| WU-1-28 | "stream + acknowledge render audit events, including override metadata" | Yes — subscription/ack only. |
| WU-1-45 | "run a redacted provider probe and return ProviderProbeObservation" | Yes — observation only. |
| WU-1-48 | "resolve harness route eligibility against capability fingerprint" | Yes — eligibility decision only. |
| WU-1-49 | "classify route denial reasons from eligibility + probe" | Yes — labelling only. |
| WU-1-50 | "render the provider preflight pane from contract fixtures" | Yes — display only. |

In every case the r3 negative-scope criteria are *narrowing* the existing concern (preventing scope creep into mutation / routing / session-storage ownership) rather than introducing a second concern. Negative-scope guards are part of the same single-concern PR; they shape what code goes inside the WU's boundary, not add a parallel deliverable. This matches the audit history's explicit gate focus (line 79): "(1) whether affected WUs are truly thin/read-only … (3) whether blocked-on annotations are precise enough for future VS-015/VS-018 owners." Both hold.

The blocked-on annotations are precise where present. WU-1-10's blocked-on note (line 447) names the specific upstream commands that gate v2 migration (`agents session locate`, `agents session export`) and explicitly excludes `import-replace` / `pause-handshake` because Phase 1 ingestion is read-only — that precision is exactly what the audit history asked for. Stitch Notes outgoing edges to VS-015 (line 3005) and VS-018 (line 2994) carry the inverse precision: "the downstream worker launcher remains thin and must spawn `agents -m <model> -p <project> -f <prompt>` while capturing the spawned `session_id` via the `agents` `--session-id` forced-flag mechanism … v2 SessionOverrideContract adapter migration needs `agents session locate / export / import-replace`" and "accepted worker-output session write-back must consume WU-0C-N1..WU-0C-N5; v2 adapter migration needs `agents session locate / export / import-replace`; atomic mid-session reintegration needs `agents pause-handshake`." Future VS-015 / VS-018 owners read these as exact prerequisites rather than as vague "Phase 0C substrate" gestures.

**Recommendation:** No action required.

---

### R3-DECOMP-F05. Stitch Notes preservation — r2 incoming/outgoing inventory intact, r3 overlay is additive

**Severity: NONE**

The Stitch Notes section (lines 2551-3031) preserves all r1/r2 incoming and outgoing pairs byte-stable. The r3 cascade is encoded as one additional sub-section ("Round 3 SessionOverrideContract incoming overlay", lines 2972-2982) plus matching `Cross-phase incoming (SessionOverrideContract)` lines on each affected WU and matching narrative bullets in the per-VS Incoming sections.

Cross-checks against the r3 audit table (lines 102-116) and the Round 3 SessionOverrideContract incoming overlay (lines 2974-2980):

- 13 affected WUs in the audit table → 13 WUs with `Cross-phase incoming (SessionOverrideContract):` lines (WU-1-03/04/05/10/12/13/23/25/28/45/48/49/50). 1:1 match.
- WU-0C-N* edge multiplicities in the overlay: N2→{03,04,05}, N1/N2/N3/N4/N5→{10}, N5→{12,13,28}, N2/N5→{23,25}, N4/N5→{45,48,49,50}. Each edge is reproduced on the corresponding WU's dependency line.
- Bidirectional consistency: the bidirectional check at lines 2433 and 2982 records that Phase 0C-r4 names WU-0C-N5 as feeding VS-001 evidence inspectors and VS-003 audit surfaces, and names WU-0C-N1..WU-0C-N5 as feeding worker-launcher / worker-output-reintegration successors. Phase 1 r3 wires the VS-001/VS-003/VS-006 read-only portions and routes the VS-015 / VS-018 mutation prerequisites to the Outgoing-To-Phase-2+ blocked-on notes. This is consistent with the engineering-roadmap-r4 phase-binding table (lines 663-671) which puts mutation in Phase 0C-r4 and Phase 5 (VS-018), not Phase 1.

The Outgoing To Phase 2+ section (lines 2984-3009) preserves the converged 24 (Phase 1 VS, Phase 2+ VS) pairs from r2; the Round 3 modifications are scoped to two enriched blocked-on annotations on the (VS-003, VS-018) and (VS-006, VS-015) rows. No Phase 1 → Phase 2+ pair was added or dropped, and WU-1-56 remains correctly excluded from foundation-edge enumeration (it is a contract-only fixture pack, not a downstream-foundation surface).

The Parallelization Map (lines 2451-2466) is unchanged at 18+12+9+5+7+3+2 = 56. The r3 re-derivation note at line 2455 is honest: "the topological waves remain 18+12+9+5+7+3+2 because SessionOverrideContract edges are cross-phase incoming gates, not new Phase 1-local edges." This is the systematic-from-start pattern lesson from Phase 0C r3 applied to a brownfield cascade.

**Recommendation:** No action required.

---

### R3-DECOMP-F06. Negative-scope criteria are testable but lean on Phase 0C-r4 fakes

**Severity: INFO**

The new criteria of the form "X never calls Y" or "the pane offers no control invoking Z" are binary and verifiable, but verification depends on test fakes that record call sites on the Phase 0C-r4 SessionOverrideContract surface. Concretely, to verify WU-1-10's bullet "The adapter never calls WU-0C-N1 mutation methods (`replace_transcript`, `truncate_after`, `append_turns`) and never edits `agents` state rows or provider session files," the test agent needs a SessionOverrideContract recorder fake whose contract is owned by WU-0C-N1 and whose default fixture exists in WU-0C-N3 (the v1 adapter binding). For the UI WUs (WU-1-13, WU-1-25, WU-1-50), the equivalent assertion is "no rendered handler bound to the forbidden command names," which is straightforward via fixture-based DOM/IPC inspection.

This is not a decomposition violation: WU-1-10 already declares WU-0C-N1..WU-0C-N5 as cross-phase incoming dependencies (line 441), so the test agent has the right contract surface available, and the test boundary at line 422 properly references the contract docs for those upstream WUs. The WUs that only depend on a subset (e.g., WU-1-03/04/05 only on WU-0C-N2) similarly have the right surface for their negative-scope assertions.

The note here is a forward-looking observation for the Phase 6b test agent and the Phase 5 hookpoints agent: the `tests/wu_1_10_contract.rs` test must be wired against a SessionOverrideContract fake (likely the WU-0C-N3 fake adapter delivered in Phase 0C-r4), not against an in-test mock invented in WU-1-10's own boundary. This routes through the hookpoints/contract-handoff phases of the implementation pipeline rather than back into the AI roadmap, so the gate verdict stays LOW.

**Recommendation:** No action required at the AI-roadmap layer. When Phase 5 hookpoints are produced for WU-1-10 and the UI affected WUs (WU-1-13, WU-1-25, WU-1-50), declare the SessionOverrideContract recorder fake as a test-boundary fixture sourced from Phase 0C-r4 (most likely WU-0C-N3 / WU-0C-N1 contract artifacts), and ensure the negative-scope criteria are realized as call-recorder assertions or DOM/IPC absence assertions rather than as in-WU mock scaffolding.

---

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R3-DECOMP-F01 | No blob WU created by SessionOverrideContract cascade; mutation WUs explicitly remain Phase 0C-r4 / VS-015 / VS-018 territory. | NONE |
| R3-DECOMP-F02 | Per-schema-object granularity (D1) preserved across all 13 affected WUs; WU-1-45 / WU-1-56 split intact; 56-row D1 audit table consistent with inventory. | NONE |
| R3-DECOMP-F03 | Binary acceptance criteria (D2) preserved; new SessionOverrideContract scope guards have binary fixture-backed shape and route through existing test boundaries. | NONE |
| R3-DECOMP-F04 | Each affected WU still maps to a single-concern PR; negative-scope guards narrow the existing concern rather than adding a second one; blocked-on annotations on WU-1-10 and Stitch Notes outgoing edges to VS-015 / VS-018 are precise. | NONE |
| R3-DECOMP-F05 | Stitch Notes preservation — r2 incoming/outgoing inventory intact, r3 SessionOverrideContract overlay is additive and bidirectional with Phase 0C-r4. | NONE |
| R3-DECOMP-F06 | Negative-scope criteria are testable but require Phase 0C-r4-owned recorder fakes for WU-1-10 and DOM/IPC absence assertions for the UI affected WUs; forward-looking note for the hookpoints/test agents, not a decomposition issue. | INFO |

## What LOW requires

The LOW rating holds while these conditions all hold. Each is verifiable by re-reading the cited artifact section.

- **No blob WU.** Phase 1 contains no worker-launcher, worker-output-reintegration, orchestrator-bridge mutation, or session-transcript-replace WU. The Round 3 SessionOverrideContract Audit table (lines 100-118), Run Report self-classification (lines 2548-2549), and Non-Ownership Notes (lines 3025-3031) all explicitly confirm this absence.
- **WU count and split stability.** Total WUs = 56. The WU-1-45 RedactedProviderProbeService / WU-1-56 FakeProviderProbeFixture split (closed in r2) remains intact. Run Report D1 audit table (lines 2480-2538) carries 56 rows matching the inventory at lines 39-96.
- **Per-schema-object granularity.** Each affected WU still owns exactly one schema object, service, adapter, fixture pack, or method-bearing UI surface. Multi-method services (WU-1-06, WU-1-10, WU-1-23, WU-1-28) bundle only the read-and-record / list-and-open pairs of one concern, consistent with the r2-converged D1 pattern.
- **Binary acceptance criteria.** Every newly added bullet on each of the 13 affected WUs has yes/no shape: scope guards forbid named API calls, evidence linkage uses opaque IDs, UI controls are absent for named commands. No subjective wording was introduced.
- **Single-concern PR-ability.** Each affected WU's concern statement still fits one sentence ("normalize X-shaped evidence", "ingest read-only", "emit audit", "render pane", "run probe", "resolve eligibility", "classify denial"). Negative-scope criteria narrow rather than multiply concerns.
- **Test/code separation preserved.** Test boundaries continue to reference contracts/DTOs/fixtures only (e.g., `product-strategy/contracts/wu-1-NN-*.md`, `src-tauri/src/contracts/wu_1_NN.rs`). Negative-scope assertions are realized as fakes/recorders/DOM-IPC absence checks, not by leaking implementation-side code into the test boundary.
- **Stitch Notes preservation.** All r1/r2 (Phase 0X WU, Phase 1 VS) incoming pairs and all 24 (Phase 1 VS, Phase 2+ VS) outgoing pairs from r2 remain byte-stable. The r3 SessionOverrideContract overlay is additive and bidirectional with Phase 0C-r4's outgoing declarations to N2/N5 → VS-001 / VS-003 evidence-and-audit surfaces and N1..N5 → VS-015 / VS-018 successors.
- **Parallelization Map preserved.** Wave partition stays at 18+12+9+5+7+3+2 = 56; the r3 re-derive note correctly identifies the SessionOverrideContract edges as cross-phase incoming, not Phase 1-local.
- **Blocked-on annotation precision.** WU-1-10's blocked-on note explicitly enumerates the upstream `agents session locate` / `agents session export` gates for v2 read migration and explicitly excludes `import-replace` / `pause-handshake` (read-only justification). Outgoing-to-Phase-2+ blockers on (VS-003, VS-018) and (VS-006, VS-015) name the specific `agents` features each downstream owner depends on.

If a future round adds a worker-launcher, worker-output-reintegration, orchestrator-bridge, or session-transcript-mutation WU into Phase 1 (rather than into VS-015 / VS-018 / Phase 0C-r4), the LOW rating must be re-evaluated against R3-DECOMP-F01. If a future round merges the WU-1-45 / WU-1-56 split or collapses any per-CLI normalizer (WU-1-03 / WU-1-04 / WU-1-05) into a single multi-CLI WU, the rating must be re-evaluated against R3-DECOMP-F02. Otherwise the LOW verdict on Decomposition stands; the parallel `ai-coverage-phase-1` and `ai-dependency-phase-1` gates carry the audit's residual-MEDIUM watch on dependency completeness (audit lines 77-79).

---

## Round 4

**Rating: LOW**

Reviewer: `claude-opus`. Round-4 brownfield is a small-scope simplification cascade driven by `proposal-r6` / `engineering-roadmap-r5` / Phase 0C-r5: the agent-runner v2 SessionOverrideContract feature requests landed, so WU-0C-N4 (the v1 transitional adapter) is dropped, schema-probe needs route to WU-0C-N3 `schema_version_probe`, and the prior `**Blocked-on:**` directives on WU-1-10 and on the (VS-003, VS-018) and (VS-006, VS-015) Outgoing-to-Phase-2+ rows are removed because there is no longer anything to block on. The artifact is `product-strategy/ai-roadmap-phase-1.md` (3034 lines, 56 WUs — `### WU-1-` count = 56, inventory table at lines 39-97 unchanged, Run Report D1 audit at lines 2483-2540 still 56 rows). The audit history (`plans/audit/ai-roadmap-phase-1.md` round-4 entry, lines 81-89) classifies r4 as `fix-created-family` gen 0 externally driven and dispatches the three risk gates again at this scope.

Decomposition is the cleanest of the three gates for this revision: r4 is *removal-only* relative to r3, no WU is added, removed, merged, or split, and every r3 negative-scope guard is preserved verbatim. The r4 audit instruction is therefore "verify the simplification did not silently violate D1, D2, or single-concern PR-ability." It did not.

### R4-DECOMP-F01. WU count, split, and per-schema-object inventory unchanged at 56

**Severity: NONE**

The r4 brownfield does not touch any WU header, contract block, test boundary, code boundary, or method/DTO inventory. Verification points:

- `### WU-1-` heading count = 56, matching r2 / r3 (the converged 56-WU artifact).
- Inventory table at lines 39-97 is byte-stable from r3 (no row added or removed; WU-1-45 / WU-1-56 split intact).
- Run Report D1 audit table at lines 2483-2540 still carries 56 rows with the explicit `D1 audit count: 56 rows = 56 WUs` assertion preserved at line 2541.
- Round 4 SessionOverrideContract Audit table (lines 102-117) lists exactly the same 13 affected WUs as r3 (WU-1-03/04/05/10/12/13/23/25/28/45/48/49/50). No new WU was introduced to absorb override-related responsibilities; WU-0C-N4 was simply replaced with WU-0C-N3 / WU-0C-N5 in the four provider-preflight rows (WU-1-45/48/49/50) and dropped from WU-1-10's overlay row.
- The Run Report self-classification at line 2552 records: "Round 4 brownfield: classification `fix-created-family` gen 0, externally driven by proposal-r6 / engineering-roadmap-r5 / Phase 0C-r5. No WU was removed or merged."

**Recommendation:** No action required.

---

### R4-DECOMP-F02. D1 per-schema-object granularity preserved across all 13 r3-affected WUs and all 43 unaffected WUs

**Severity: NONE**

D1 is verified by re-running the r3 spot-check against the r4 artifact. The per-CLI normalizer split (WU-1-03 Claude / WU-1-04 Codex / WU-1-05 opencode) is intact at lines 184-294; the read-and-record adapter pair (`ingest_transcript` + `mark_partial_ingest`) on WU-1-10 is intact at line 419; the WU-1-45 RedactedProviderProbeService / WU-1-56 FakeProviderProbeFixture split is intact at lines 1525-1551 and at the inventory table; the audit/evidence-linkage WUs (WU-1-12 / WU-1-23 / WU-1-28) and the UI WUs (WU-1-13 / WU-1-25 / WU-1-50) remain single-method or method-pair surfaces.

The 43 r3-unaffected WUs (WU-1-01/02/06/07/08/09/11/14/15/16/17/18/19/20/21/22/24/26/27/29/30/31/32/33/34/35/36/37/38/39/40/41/42/43/44/46/47/51/52/53/54/55/56) carry no r4 edits at all. The Round 4 SessionOverrideContract Audit narrative at line 119 explicitly records: "Unaffected WUs: WU-1-01/02/06/07/08/09/11/14/15/16/17/18/19/20/21/22/24/26/27/29/30/31/32/33/34/35/36/37/38/39/40/41/42/43/44/46/47/51/52/53/54/55/56 do not encode per-CLI session storage, provider routing, quota, resume, cross-provider porting, or session mutation behavior. No WU scope collapsed enough to merge into a sibling." `bundling-family` does not re-fire.

**Recommendation:** No action required.

---

### R4-DECOMP-F03. D2 binary acceptance criteria preserved; WU-0C-N4 → WU-0C-N3 substitution keeps yes/no shape

**Severity: NONE**

The r3 negative-scope acceptance criteria are intact. Spot checks against the affected WUs:

- **WU-1-10** (lines 432-434): "`ingest_transcript` reads session content only through WU-0C-15d normalized turn evidence or WU-0C-N1 `read_transcript` / `get_session_metadata`; it accepts `session_id` as the lookup key and does not accept filesystem transcript paths as product inputs"; "The adapter records unsupported schema, unsupported storage, busy session, missing session, and adapter refusal as evidence artifacts without attempting fallback per-CLI JSONL parsing"; "The adapter never calls WU-0C-N1 mutation methods (`replace_transcript`, `truncate_after`, `append_turns`) and never edits `agents` state rows or provider session files." All three are byte-stable from r3.
- **WU-1-45** (line 1535): the previous "Any session-override compatibility shown beside provider state comes from WU-0C-N4/WU-0C-N5 evidence, not probe-owned JSONL or `state.db` inspection" was rewritten to "Any session-override compatibility shown beside provider state comes from WU-0C-N3/WU-0C-N5 evidence, not probe-owned JSONL or `state.db` inspection." Identical yes/no shape, identical negative-scope clause; only the upstream WU IDs changed.
- **WU-1-48** (line 1662): "Workload requirements may include a need for session override capability by opaque WU-0C-N3/WU-0C-N5 evidence refs, but the resolver does not probe or mutate sessions directly" — same substitution, same binary "by opaque ID only" shape.
- **WU-1-50** (line 1731): "The pane can display session-override capability/refusal metadata from WU-0C-N3/WU-0C-N5 as read-only evidence and never exposes raw transcript paths, provider-native JSONL, or adapter temp files" — same substitution; the companion "no account picker, quota-balancing control, auth refresh control, resume/import control, provider reroute command, or SessionOverrideContract mutation command" criterion at the next bullet is byte-stable.
- **WU-1-03/04/05** (per-CLI normalizers), **WU-1-12/13/23/25/28** (audit/evidence linkage and UI display) — none of their r3 binary criteria reference WU-0C-N4, so r4 leaves them byte-stable.

The substitution is binary (one upstream ID for another), and `WU-0C-N3 schema_version_probe` is verifiable by the same fixture/recorder pattern that r3 named for the WU-0C-N1 mutation guards (R3-DECOMP-F06 forward note). No subjective wording was introduced. `state-machine-criteria-family` does not re-fire.

The Run Report D2 row at line 2475 still records "D2 binary criteria | Applied | Each WU has method, enum, state-machine, or fixture-state criteria plus binary fixture assertions generated from its Contract."

**Recommendation:** No action required.

---

### R4-DECOMP-F04. Refactored-WU coherence preserved — removing `**Blocked-on:**` annotations does not touch any WU's single concern

**Severity: NONE**

The r3 audit instruction asked the gate to verify (paraphrasing audit history line 79) that "blocked-on annotations are precise enough for future VS-015/VS-018 owners." The r4 cascade resolves that concern by *removing* the directive entirely once the upstream `agent-runner` feature requests land, not by changing what the WUs do. Verification:

- **WU-1-10** (lines 446-447): the prior r3 `**Blocked-on:** v1 read behavior can use WU-0C-N3 after schema probe. v2 adapter migration is blocked on `agents session locate` and `agents session export` ...` was deleted. The Revision rationale on line 447 retains "Round 3 makes this the central Phase 1 reader over normalized session evidence. It no longer owns transcript path discovery, per-CLI JSONL parsing fallback, session storage mutation, resume composition, or provider/session porting." plus an appended r4 cascade note. The single-concern statement ("ingest session-derived tool-call evidence read-only and emit refusal evidence") is byte-stable.
- **(VS-003, VS-018) Outgoing-to-Phase-2+ row** (line 2997): the prior r3 `**Blocked-on:** accepted worker-output session write-back must consume WU-0C-N1..WU-0C-N5; v2 adapter migration needs `agents session locate / export / import-replace`; atomic mid-session reintegration needs `agents pause-handshake`.` was deleted. The Phase 1 foundation roster (`WU-1-02, WU-1-03, ..., WU-1-13`) is unchanged; the (Phase 1 VS, Phase 2+ VS) pair survives.
- **(VS-006, VS-015) Outgoing-to-Phase-2+ row** (line 3008): the prior r3 `**Blocked-on:** the downstream worker launcher remains thin and must spawn `agents -m <model> -p <project> -f <prompt>` ...` was deleted. The Phase 1 foundation roster (`WU-1-45, WU-1-46, WU-1-47, WU-1-48, WU-1-49, WU-1-50`) is unchanged; the (Phase 1 VS, Phase 2+ VS) pair survives.

No WU's concern statement changed. The single-concern PR-ability test (decomposition rule from `roadmap-risk-types.md` lines 333-335) still passes for every affected WU: "normalize Claude-shaped tool evidence", "ingest session-derived tool-call evidence read-only and emit refusal evidence", "emit tool-call audit events optionally linking override refs", "render the tool-call drill-down view", "run a redacted provider probe", "resolve harness route eligibility", "classify route denial reasons", "render the provider preflight pane". Negative-scope guards still narrow rather than multiply concerns.

The blocked-on annotations were narrative metadata aimed at downstream VS-015 / VS-018 owners; their removal is a documentation-level simplification, not a decomposition-level scope change. The 13 affected WUs' Phase 1 read-only ownership is preserved by the negative-scope acceptance criteria (R4-DECOMP-F03 above), independent of the blocked-on text.

**Recommendation:** No action required.

---

### R4-DECOMP-F05. Stitch Notes preservation — Outgoing-to-Phase-2+ pair list intact at 24; Round-4 incoming overlay drops WU-0C-N4 cleanly

**Severity: NONE**

The Stitch Notes section (lines 2554-3033) preserves the converged r2 / r3 inventory. Verification:

- **Outgoing To Phase 2+** (lines 2987-3012): the 24 (Phase 1 VS, Phase 2+ VS) pairs from r2 / r3 remain. Only the directive text after "consumes Phase 1 foundations: ..." was removed on the (VS-003, VS-018) and (VS-006, VS-015) rows. No pair was added or dropped, and WU-1-56 remains correctly excluded from foundation-edge enumeration.
- **Round 4 SessionOverrideContract incoming overlay** (lines 2421-2434, 2974-2986): the section was renamed from "Round 3" to "Round 4", and the WU-0C-N4 edge to WU-1-10 plus the `(WU-0C-N4, WU-1-45/48/49/50)` pairs were dropped. The remaining edges are the same r3 set minus N4: `WU-0C-N2 -> WU-1-03/04/05`; `WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5 -> WU-1-10`; `WU-0C-N5 -> WU-1-12, WU-1-13, WU-1-28`; `WU-0C-N2, WU-0C-N5 -> WU-1-23, WU-1-25`; `WU-0C-N3, WU-0C-N5 -> WU-1-45, WU-1-48, WU-1-49, WU-1-50`. WU-0C-N4 references survive only as narrative explaining the removal (lines 100, 2423, 2447, 2457, 2552); no live dependency declaration on WU-0C-N4 remains.
- **Bidirectional consistency** (line 2986): r4 records "Phase 0C-r5 names WU-0C-N5 as feeding VS-001 evidence inspectors and VS-003 audit surfaces, and names v2-only SessionOverrideContract edges as feeding Phase 1 worker-launcher / worker-output-reintegration successors. Phase 1 r4 wires the existing VS-001/VS-003/VS-006 read-only surfaces. The worker-launcher and worker-output-reintegration edges are intentionally carried forward as outgoing/downstream edges because those WUs are not present in this 56-WU Phase 1 artifact." The earlier r3 wording "downstream blockers" is replaced with "downstream edges" — accurate now that the v2 features have landed.
- **Parallelization Map** (lines 2451-2466): unchanged at 18+12+9+5+7+3+2 = 56. The r4 re-derive note at line 2457 is honest: "Round 4 re-derive: removing WU-0C-N4 and former block-on annotations adds no Phase 1-local edge; waves remain 18+12+9+5+7+3+2 = 56."
- **Critical Path** (line 2447): the r4 re-check is consistent: "Phase 0C-r5 removes WU-0C-N4 and makes SessionOverrideContract v2-only; the Phase 1-local critical path remains unchanged."

**Recommendation:** No action required.

---

### R4-DECOMP-F06. R3-DECOMP-F06 forward-looking note still applies; Phase 0C-r5 is now the test-fixture source

**Severity: INFO**

R3-DECOMP-F06 noted that the negative-scope criteria of the form "X never calls Y" require a SessionOverrideContract recorder fake delivered by Phase 0C-r4 (most likely the WU-0C-N3 v1 adapter or a WU-0C-N1 contract recorder). After r4, WU-0C-N4 is dropped and Phase 0C-r5 makes SessionOverrideContract v2-only with the agent-runner feature requests landed. The forward-looking guidance is therefore unchanged in shape: the Phase 5 hookpoints agent and the Phase 6b test agent still need to source the recorder fake from Phase 0C (now r5), only the upstream version label moves from "v1 adapter" to "v2-only contract surface." The negative-scope criteria themselves remain testable by call-recorder assertions on the v2 SessionOverrideContract surface and by DOM/IPC absence assertions for the UI WUs.

This is informational only and does not affect the AI-roadmap decomposition layer.

**Recommendation:** No action required at the AI-roadmap layer. When Phase 5 hookpoints are produced for WU-1-10 and the UI affected WUs (WU-1-13, WU-1-25, WU-1-50), declare the SessionOverrideContract recorder fake as a test-boundary fixture sourced from Phase 0C-r5 (the v2-only contract surface) rather than the r4 v1-adapter mix.

---

## Round 4 summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-DECOMP-F01 | WU count 56 unchanged; per-WU split inventory (WU-1-45 / WU-1-56, WU-1-03 / WU-1-04 / WU-1-05) byte-stable; r4 cascade self-classified `fix-created-family` gen 0 externally driven. | NONE |
| R4-DECOMP-F02 | D1 per-schema-object granularity preserved across 13 r3-affected WUs and 43 r3-unaffected WUs; no WU scope collapsed enough to merge into a sibling. | NONE |
| R4-DECOMP-F03 | D2 binary acceptance criteria preserved; the WU-0C-N4 → WU-0C-N3 / WU-0C-N5 substitution on WU-1-10/45/48/50 keeps the same yes/no shape; r3 negative-scope guards on WU-1-10 are byte-stable. | NONE |
| R4-DECOMP-F04 | Each r3-affected WU still maps to one single-concern PR; removal of `**Blocked-on:**` annotations on WU-1-10 and on the (VS-003, VS-018) / (VS-006, VS-015) Outgoing-to-Phase-2+ rows does not touch any concern statement. | NONE |
| R4-DECOMP-F05 | Stitch Notes preservation — 24 Outgoing-to-Phase-2+ pairs intact; Round-4 SessionOverrideContract incoming overlay drops the WU-0C-N4 edge cleanly with no live N4 dependency declaration anywhere; Parallelization Map and Critical Path unchanged at 18+12+9+5+7+3+2 = 56. | NONE |
| R4-DECOMP-F06 | r3 forward-looking note about Phase 0C recorder fakes still applies; the test-fixture source moves from Phase 0C-r4 v1-adapter mix to Phase 0C-r5 v2-only contract surface. Informational only. | INFO |

## What LOW requires (round 4)

The r3 LOW conditions all still hold and a few are tightened by simplification:

- **No blob WU added, removed, merged, or collapsed.** Phase 1 r4 retains the same 56 WUs as r3 / r2 and the same WU-1-45 / WU-1-56 split. The Round 4 SessionOverrideContract Audit table (lines 100-117), Run Report self-classification at line 2552, and Non-Ownership Notes (carried forward from r3) all confirm this.
- **Per-schema-object granularity preserved.** Each r3-affected WU still owns exactly one schema object, service, adapter, fixture pack, or method-bearing UI surface; the 43 unaffected WUs are byte-stable from r3.
- **Binary acceptance criteria preserved.** The WU-0C-N4 → WU-0C-N3 / WU-0C-N5 substitution on WU-1-10 (line 441 dependency line) and on WU-1-45 / WU-1-48 / WU-1-49 / WU-1-50 (lines 1535, 1543, 1670, 1705, 1731, 1741) keeps every affected acceptance criterion in yes/no form. R3 negative-scope guards on WU-1-10 (lines 432-434) are byte-stable.
- **Single-concern PR-ability preserved.** Removing `**Blocked-on:**` annotations is a documentation-level simplification. The Phase 1 read-only boundary is enforced by the negative-scope acceptance criteria (R4-DECOMP-F03) and by the Round 4 boundary preamble at line 100 ("no WU in this Phase 1 artifact owns worker launch, worker-output reintegration, orchestrator turn execution, or session transcript mutation"), independent of blocked-on text.
- **Test/code separation preserved.** Test boundaries continue to reference contracts/DTOs/fixtures only. The SessionOverrideContract recorder-fake source moves from Phase 0C-r4 (v1 adapter) to Phase 0C-r5 (v2-only) at the hookpoints/Phase 5 layer, not inside Phase 1 WUs.
- **Stitch Notes preservation.** All r2 (Phase 0X WU, Phase 1 VS) incoming pairs and all 24 (Phase 1 VS, Phase 2+ VS) outgoing pairs from r2 / r3 remain. The r4 incoming overlay drops only the WU-0C-N4 edges and renames the section header from "Round 3" to "Round 4"; bidirectional consistency holds against Phase 0C-r5.
- **Parallelization Map and Critical Path unchanged.** Wave partition stays at 18+12+9+5+7+3+2 = 56. The r4 re-derive note correctly identifies that removing WU-0C-N4 and former block-on annotations adds no Phase 1-local edge.
- **No live WU-0C-N4 dependency declaration anywhere.** A grep over the artifact finds WU-0C-N4 only in five narrative lines (100, 2423, 2447, 2457, 2552) explaining the removal. Every prior dependency-declaration occurrence on WU-1-10's `Cross-phase incoming:` line, on WU-1-45 / WU-1-48 / WU-1-49 / WU-1-50's `Cross-phase incoming (SessionOverrideContract):` line, on the WU-1-45 / WU-1-50 acceptance criteria, and on the Round 3 SessionOverrideContract incoming overlay was either rewritten to WU-0C-N3 or dropped.
- **No live `**Blocked-on:**` annotation anywhere.** A grep over the artifact finds zero `**Blocked-on:**` directive lines; the only `block-on` mentions are five narrative lines noting their removal.

If a future round re-introduces a worker-launcher, worker-output-reintegration, orchestrator-bridge, or session-transcript-mutation WU into Phase 1, the LOW rating must be re-evaluated against R3-DECOMP-F01 / R4-DECOMP-F01. If the WU-1-45 / WU-1-56 split or any per-CLI normalizer split is collapsed, the rating must be re-evaluated against R3-DECOMP-F02 / R4-DECOMP-F02. Otherwise the LOW verdict on Decomposition stands at round 4; the parallel `ai-coverage-phase-1` and `ai-dependency-phase-1` gates carry the residual review of dependency / coverage completeness for the simplification.
