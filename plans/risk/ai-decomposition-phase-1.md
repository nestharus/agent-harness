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
