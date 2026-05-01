# AI Roadmap (Phase 0C) — Decomposition Risk Assessment

Per-phase decomposition risk file for Phase 0C (Shared Engines and Integration Shells). Risk type spec: `product-strategy/roadmap-risk-types.md` "AI Roadmap Risks - 1. Decomposition Risk." Artifact under review: `product-strategy/ai-roadmap-phase-0c.md`. Audit history cross-reference: `plans/audit/ai-roadmap-phase-0c.md`.

---

## Round 1 (greenfield, 37 WUs)

**Rating: MEDIUM.**

Findings (per audit history):

- **R1-DECOMP-F01.** WU-0C-15 family-level input/output DTO bundling (verbatim repeat of parent-loop R6-DECOMP-F01).
- **R1-DECOMP-F02.** Systemic family-level bundling across agent-runner / render / optimizer / recovery / IPC families.
- **R1-DECOMP-F03.** WU-0C-11 bundled two distinct schema objects + two functions under an unlisted exception.
- **R1-DECOMP-F04.** D1 audit table did not reflect splits.

Resolution: brownfield to apply Rule D1 strictly (per-schema-object splits) and re-derive the D1 table.

---

## Round 2 (brownfield, 37 → 72 WUs)

**Rating: LOW.**

- F-1/F-2/F-3 splits applied per Rule D1: agent-runner / render / optimizer / recovery / IPC families decomposed into per-schema-object WUs (37 → 72).
- F-4 D1 audit table now lists 72 entries matching inventory.
- Identity/conflict shell added as WU-0C-29c (`IdentityResolverRuntime`) and WU-0C-29d (`ConflictRecordWriterShell`). (Carried in Coverage gate, not Decomposition; recorded here for traceability only.)
- `bundling-family` watch retired at gen 1 (split applied).

---

## Round 3 (brownfield, Stitch Notes only — 72 WUs unchanged)

**Rating: LOW (regression-only confirmed).**

- WU count 72 unchanged; D1 ownership table 72 rows unchanged; r3 diff has 0 hits in `Contract:` / `Test boundary` / `Code boundary` blocks.
- All round-2 closures preserved by construction (no Contract modified).
- `bundling-family` and `state-machine-criteria-family` watches dormant; round 3 was Stitch-Notes-scoped under the Dependency gate.

---

## Round 4 (brownfield, SessionOverrideContract integration — 72 → 77 WUs)

**Rating: LOW.**

Round 4 cascades the proposal-r5 / engineering-roadmap-r4 `SessionOverrideContract` axiom into Phase 0C. Five new WUs were added (WU-0C-N1..WU-0C-N5) and 11 existing WUs were narrowed. The decomposition gate audit below applies the round-4 prompt's five sub-checks: per-schema-object ownership, binary acceptance criteria, no-blob WUs, refactored-WU coherence, and Stitch Notes outgoing-edge coherence.

### Findings

#### R4-DECOMP-F01. Per-schema-object ownership across new WU-0C-N1..N5.

**Severity: LOW.**

The five new WUs each own exactly one concept and appear as discrete rows in the D1 audit table at `ai-roadmap-phase-0c.md:3512-3516`:

- WU-0C-N1 owns the `SessionOverrideContract` Rust trait + TypeScript interface surface (one trait surface; method bodies excluded from this WU per single-concern PR constraint at line 1736).
- WU-0C-N2 owns the override DTO surface (`TranscriptTurn`, `SessionLocation`, `SessionMetadata`, `OverridePreconditions`, `OverrideReceipt`, plus the subordinate `SessionOverrideError` enum). See discussion in F-02.
- WU-0C-N3 owns the v1 `AgentRunnerDbAdapter` service object — the only WU permitted direct `state.db` / JSONL writes, pinned by WU-0C-N4 (line 1843, 3696).
- WU-0C-N4 owns the `AgentRunnerSchemaProbe` service object (read-only SQLite probe, no mutation per line 1774).
- WU-0C-N5 owns the `SessionOverrideStore` registry record + service (lifecycle transitions only, no transcript writes per line 1891 and 1900).

The trait (N1), v1 adapter (N3), schema probe (N4), and registry (N5) are clearly separate WUs — no blob WU bundles them. The trait WU explicitly forbids state.db access, JSONL rendering, crash recovery, registry persistence, or IPC routes (line 1736). This satisfies the prompt's "no blob WU" check that the trait should not bundle the v1 adapter.

**Recommendation:** No action required.

#### R4-DECOMP-F02. WU-0C-N2 bundles five DTO structs plus an error taxonomy.

**Severity: INFO.**

WU-0C-N2's contract block (lines 1582-1660) declares five distinct Rust structs (`TranscriptTurn`, `SessionLocation`, `SessionMetadata`, `OverridePreconditions`, `OverrideReceipt`) plus the `SessionOverrideError` enum. Round-2 brownfield split equivalent multi-DTO families per Rule D1 — for example, `SessionTurns*` was decomposed into WU-0C-15a/15b/15c/15d (request DTO, response DTO, turn-ref DTO, reader service). Strict application of that precedent would split N2 into five DTO WUs.

Mitigations supporting the bundled WU:

- The five DTOs are co-referenced inside the single trait method signature surface (`SessionOverrideContract`); they do not have independent consumers in Phase 0C.
- The `SessionOverrideError` enum is explicitly permitted to ride along under the Phase 0C ownership rule at line 30: "unless it is only an error taxonomy subordinate to that WU's single contract."
- The single-concern PR constraint at line 1684 is narrow: "may add only DTOs, validators, fixture JSON, and generated TypeScript types. It may not add adapter file/DB access, trait methods, IPC commands, or registry persistence." A reviewer applying the multi-concern PR rule from `AGENTS.md` would not split this PR — every change is a single contract-surface declaration.
- The D1 audit table line 3512 is explicit: "TranscriptTurn / SessionLocation / SessionMetadata DTO family | WU-0C-N2 | Own WU." The bundling is intentional and disclosed.
- The acceptance criteria at lines 1668-1674 enumerate every variant for round-trip / validator coverage and explicitly forbid side effects (no transcript writes, no state.db rows, no audit/evidence/registry rows), which keeps the test-only DTO scope tight.

This pattern is a deviation from the Phase 0C r2 systematic per-DTO split, but does not violate the single-concern PR rule and does not bundle distinct schema objects with distinct downstream consumers. Classifying INFO rather than MEDIUM because:

1. The bundling is co-declared at the audit table level (no D1 exception is hidden).
2. The five DTOs cannot be defined or tested independently — each appears in the others' field shapes (e.g., `OverrideReceipt.preimage_hash` mirrors `OverridePreconditions.expected_preimage_hash`; `SessionMetadata.last_observed_turn_id` references `TranscriptTurn.turn_id`).
3. The prompt's binary criteria for LOW say "per-schema-object ownership" with the qualifier "each WU owns at most one durable schema object / command / event / parser." None of N2's structs are durable schema objects (i.e., none are GraphStore tables) — they are non-durable DTOs co-defining one contract surface.

**Recommendation:** No action required for round 4. Add a watch under `bundling-family` gen 0 if a future round adds distinct downstream consumers that pull individual DTOs for orthogonal purposes — at that point the systematic split should be applied.

#### R4-DECOMP-F03. Binary acceptance criteria across new WU-0C-N1..N5.

**Severity: LOW.**

Acceptance criteria for the five new WUs are concrete, binary, and do not rely on subjective phrasing:

- WU-0C-N2 (lines 1668-1674): 7 criteria, all binary — round-trip parity, validator rejections of named invalid inputs, no-side-effect statements with enumerated row types.
- WU-0C-N1 (lines 1719-1726): 8 criteria, all binary — exact method count, fake-adapter behavior, error-discriminant stability, schema-probe-before-mutation invariant, exclusion clauses for provider routing/quota/auth/resume/porting/session-id.
- WU-0C-N4 (lines 1768-1774): 7 criteria, all binary — supported/unsupported probe outcomes, named columns enumerated, read-only constraint with explicit no-migration / no-PRAGMA / no-write list.
- WU-0C-N3 (lines 1824-1833): 10 criteria, all binary — locate/read/replace/truncate/append outcomes for valid + named-error fixtures, crash-injection states enumerated (`committed`, `rolled_back`, `QuarantinedStorageConflict`), explicit non-mutation list for `agents`, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider routing.
- WU-0C-N5 (lines 1884-1891): 8 criteria, all binary — record round-trip, lifecycle transitions enumerated (`pending->committed`, `pending->rolled_back`, `pending->quarantined_storage_conflict`), explicit no-side-effect list.

No "the service works" language. No subjective "clean" / "consistent" criteria. Every method-bearing criterion names the call, valid result shape, and an enumerated error variant or invariant.

**Recommendation:** No action required.

#### R4-DECOMP-F04. Refactored existing WUs remain coherent (no empty / overly-thin WUs).

**Severity: LOW.**

The Round 4 Refactor Ledger (lines 3262-3278) lists 11 existing WUs narrowed by the boundary refactor. Spot-check confirms each retains a meaningful single-concern scope:

- **WU-0C-13 `AgentSessionCapture`** (line 1026): still owns `capture_agent_runner_session(request)`; 6 binary ACs covering each `capture_method` variant + substrate-gap fixture; explicit no-locate/parse/append/truncate/replace constraint.
- **WU-0C-13a/13b** (lines 940-1024): single DTO each, retained validator + variant-reachability ACs.
- **WU-0C-15a `SessionTurnRef` DTO** (line 1164): 6 binary ACs including opaque source-offset/hash treatment; explicit non-parse-of-Claude/Codex/opencode-records constraint.
- **WU-0C-15b `SessionTurnsRequest` DTO** (line 1208): 5 binary ACs; raw transcript locator input removed; ownership now reads only normalized invocation/session/trace evidence.
- **WU-0C-15c `SessionTurnsRead` DTO** (line 1248): 6 binary ACs; provider-neutral missing-locator/substrate-gap fields replaced raw locator/opencode-gap fields; canonical transcript material delegated to WU-0C-N2.
- **WU-0C-15d `SessionTurnsReader`** (line 1291): 6 binary ACs including the explicit prohibition on calling `replace_transcript`/`truncate_after`/`append_turns` (line 1313). Read-only contract is concrete.
- **WU-0C-18 `AgentRunnerClient`** (line 1536): 11 binary ACs across 8 facade methods; explicit no-transcript-write criterion at line 1568; explicit no-provider-routing / no-quota / no-account-rewrite criterion at line 1569. Still a meaningful join WU.
- **WU-0C-31 `RecoveryActionProcessorSkeleton`** (line 2852): retains `preflight` + `classify` methods with 6 binary ACs; admits WU-0C-N5 metadata input per refactor ledger line 3275 while preserving the no-side-effect skeleton scope.
- **WU-0C-34 `TauriIpcCommandRouter`** (line 2998): retains 15 routed `HarnessCommand` additions including 3 new override-read commands (`locate_session_override`, `get_session_override_metadata`, `list_session_overrides`); 8 binary ACs; explicit AC at line 3038 forbidding `replace_transcript`/`truncate_after`/`append_turns` UI commands in Phase 0C.

The refactor narrowing is uniformly subtractive (removes per-CLI knowledge / write paths) without merging concerns or producing empty WUs. The refactor ledger's closing statement at line 3278 — "No WU became empty after refactor. No WU was removed or merged" — is consistent with line-by-line inspection.

**Recommendation:** No action required.

#### R4-DECOMP-F05. Stitch Notes outgoing-edge coherence preserves rounds 2-3 closure and adds SessionOverrideContract edges.

**Severity: LOW.**

The round 3 closure pattern (foundation-row blocks + per-VS blocks + OptimizerRequest cross-slice contract) is preserved byte-for-byte in shape. Round 4 additions are additive:

- **Foundation-row block.** A new SessionOverrideContract row is added at line 3645 listing VS-010, VS-012, VS-018, VS-020, VS-021 plus Phase 1/2/3 downstream WUs. No existing foundation row was removed or had consumers deleted.
- **Per-VS blocks.** VS-010 (line 3669), VS-012 (line 3671), VS-018 (line 3677), VS-020 (line 3679), and VS-021 (line 3680) each have explicit `SessionOverrideContract WU-0C-N1..WU-0C-N5` mentions with `**Blocked-on:**` annotations naming the missing `agents` features (`agents session locate/export/import-replace`, pause-handshake, schema-version probe). The 17 round-3 per-VS blocks not affected by round 4 are unchanged in form.
- **OptimizerRequest cross-slice contract** at line 3653 still enumerates all six emitter→queue pairs verbatim from round 3.
- **Cross-phase outgoing edges added in round 4** (lines 3682-3687) name Phase 1 worker-launcher / worker-output-reintegration, Phase 2 turn-decomposition / detail-injection-router, and Phase 3 repack-planner consumers, plus a directive that downstream Phase 1/2/3 revisions add explicit incoming-from-Phase-0C edges back to WU-0C-N1..WU-0C-N5.
- **Refactor incoming edges** at lines 3622, 3634 add SessionOverrideContract WU-0C-N1..WU-0C-N5 incoming consumption from WU-0A-03, WU-0A-15, WU-0B-09, WU-0B-15, WU-0B-31 without duplicating ownership.

The Stitch Notes block satisfies both round-3 closure (foundation/per-VS/cross-slice triple) and round-4 additivity (SessionOverrideContract foundation row + 5 per-VS additions + cross-phase handoff).

**Recommendation:** No action required.

### Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-DECOMP-F01 | Per-schema-object ownership across WU-0C-N1..N5 | LOW |
| R4-DECOMP-F02 | WU-0C-N2 bundles five co-referenced DTOs + error taxonomy | INFO |
| R4-DECOMP-F03 | Binary acceptance criteria across WU-0C-N1..N5 | LOW |
| R4-DECOMP-F04 | Refactored WUs remain coherent (no empty / thin WUs) | LOW |
| R4-DECOMP-F05 | Stitch Notes outgoing-edge coherence preserved + SessionOverrideContract added | LOW |

### What LOW requires

For Phase 0C round 4 Decomposition LOW to remain valid, the following conditions must hold:

1. WU count remains 77 with WU-0C-N1..N5 each owning the schema_object/trait/service named in the D1 table (lines 3512-3516).
2. WU-0C-N1 trait WU does not absorb adapter implementation (no `state.db` access, no JSONL rendering, no crash recovery, no IPC routes); WU-0C-N3 remains the only WU with direct DB/JSONL write authority, pinned by WU-0C-N4 schema probe.
3. WU-0C-N2 acceptance criteria continue to forbid side effects (no transcript files, no state.db rows, no EvidenceArtifact/AuditEvent rows, no GraphNode rows, no SessionOverrideStore rows).
4. WU-0C-N2's bundled-DTO pattern stays bounded to the trait method signature surface. If a future round adds distinct downstream consumers that pull individual DTOs (e.g., `TranscriptTurn` consumed without `OverrideReceipt`), apply the round-2 systematic per-DTO split and convert this INFO into a closed finding under `bundling-family` gen 1.
5. WU-0C-13/15a/15b/15c/15d/18/31/34 retain their narrowed-but-coherent scopes per the Refactor Ledger (lines 3262-3278). No empty WU emerges.
6. Every refactored WU's "never calls `replace_transcript`/`truncate_after`/`append_turns`" / "no provider routing / quota / account / auth / resume / porting" exclusion clauses remain in their acceptance criteria so the SessionOverrideContract boundary is enforceable at PR review.
7. Stitch Notes preserves the round-3 foundation-row + per-VS + OptimizerRequest cross-slice triple and the round-4 SessionOverrideContract foundation row (line 3645) + per-VS additions (lines 3669, 3671, 3677, 3679, 3680) + cross-phase edges (lines 3682-3687).
8. The `bundling-family`, `state-machine-criteria-family`, `fix-created-family`, `dependency-encoding-family`, and `session-override-boundary-family` watches stay active; `session-override-boundary-family` is new in round 4 and must propagate to Phase 1/2/3 revisions.

If condition (1), (2), or (5) is violated, MEDIUM is the correct re-rating. If both (1) and (2) are violated or a new blob WU appears, escalate to HIGH.
