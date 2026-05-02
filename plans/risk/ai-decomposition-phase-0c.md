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

---

## Round 5 (brownfield, Option A simplification — 77 → 76 WUs)

**Rating: LOW.**

Round 5 applies the proposal-r6 / engineering-roadmap-r5 Option A simplification after the upstream `agent-runner` `session locate` / `export` / `import-replace` / `pause-handshake` / `resume-handshake` / `schema-probe` features landed. WU-0C-N3 is rescoped from the v1 `AgentRunnerDbAdapter` (direct `state.db` + JSONL writes) to the v2 `AgentRunnerCliAdapter` (subprocess-only calls into the `agents` binary). WU-0C-N4 (`AgentRunnerSchemaProbe`) is removed because schema compatibility is a single CLI call enforced at adapter construction. All `Blocked-on:` annotations are removed across the file (verified: zero matches for `Blocked-on` in `ai-roadmap-phase-0c.md`). The decomposition gate audit below applies the round-5 prompt's five sub-checks: per-schema-object ownership (verify rescoped N3 still owns one concept = `AgentRunnerCliAdapter`), binary acceptance criteria, no-blob WUs, refactored-WU coherence, and Stitch Notes preservation.

### Findings

#### R5-DECOMP-F01. Per-schema-object ownership across rescoped WU-0C-N3 and removed WU-0C-N4.

**Severity: LOW.**

Three independent verifications were run for the round-5 simplification:

1. **WU-0C-N3 owns one schema_object.** The contract block at lines 1743-1761 declares a single `schema_object: Rust service object implementing SessionOverrideContract`, named `AgentRunnerCliAdapter`. The "Implements" subblock lists exactly the seven trait methods from WU-0C-N1 (`schema_version_probe`, `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after` / `append_turns`, `get_session_metadata`) each mapped to the corresponding `agents session` subcommand. No second service object is bundled (no separate registry, no separate parser, no separate IPC route). The single-concern PR constraint at line 1788 enumerates explicit anti-scope including "a harness-side schema-probe wrapper WU," which proactively forbids re-creating a WU-0C-N4-shaped concern.

2. **WU-0C-N4 removal is clean.** The r4 `AgentRunnerSchemaProbe` schema_object name no longer appears anywhere in the artifact except in the removal note at line 1737 ("The r4 `AgentRunnerSchemaProbe` existed only to protect the v1 direct `state.db` / JSONL adapter") and the revision rationale at line 1790 (historical context). The D1 audit table at lines 3415-3492 contains no row for `AgentRunnerSchemaProbe`. The placeholder section heading "WU-0C-N4: Removed in r5" at line 1735 is preserved as a redirect so external references resolve to the removal rationale rather than 404.

3. **D1 audit table inventory matches the WU count.** The Phase 0C inventory at line 84 declares "Total Phase 0C WUs: **76**." The "SessionOverrideContract and CLI adapter foundation" group row at line 92 enumerates "WU-0C-N1, WU-0C-N2, WU-0C-N3, WU-0C-N5 | 4" (down from r4's "WU-0C-N1..WU-0C-N5 | 5"). The D1 ownership table lists exactly the four session-override WUs (lines 3453-3456) and 72 non-override WUs for a total of 76 owned objects/services, consistent with the inventory total.

The N1 trait, N2 DTOs, N3 CLI adapter, and N5 registry remain four discrete WUs each owning one trait surface / one DTO bundle / one service object / one registry. No blob WU bundles them. The trait WU still explicitly forbids `state.db` access, JSONL rendering, crash recovery, registry persistence, or IPC routes (line 1733); the rescoped adapter WU still forbids direct SQLite writes, direct provider transcript writes, and the re-creation of a separate harness-side schema-probe wrapper WU (line 1788).

**Recommendation:** No action required.

#### R5-DECOMP-F02. WU-0C-N2 DTO bundling unchanged from round 4 (INFO watch dormant).

**Severity: INFO.**

Round 5 does not modify WU-0C-N2's contract block (lines 1581-1683); the five-DTO-plus-error-taxonomy bundling pattern flagged as INFO in R4-DECOMP-F02 carries over verbatim. The conditions for upgrading the watch — distinct downstream consumers pulling individual DTOs for orthogonal purposes — have not materialized. The new WU-0C-N3 contract co-references all five DTOs (`SchemaProbe` via `schema_version_probe()`, `SessionLocation` via `locate_session()`, `TranscriptTurn` via `read_transcript()`, `OverridePreconditions` + `OverrideReceipt` via `replace_transcript()` / `truncate_after()` / `append_turns()`, `SessionMetadata` via `get_session_metadata()`), which keeps the bundled DTOs anchored to the same single trait surface that justified the round-4 disposition.

The R4 LOW preservation conditions for N2 still hold:
- N2 ACs continue to forbid side effects (line 1673: "DTO validation writes no transcript files, state.db rows, EvidenceArtifact rows, AuditEvent rows, GraphNode rows, or SessionOverrideStore registry rows.").
- The bundled DTOs cannot be defined or tested independently (e.g., `OverrideReceipt.preimage_hash` mirrors `OverridePreconditions.expected_preimage_hash`; `SessionMetadata.last_observed_turn_id` references `TranscriptTurn.turn_id`; `SessionLocation.storage_kind` enumerates the same `claude_code_jsonl` / `codex_jsonl` / `agent_runner_cli` / `unsupported` set the adapter validates).

**Recommendation:** No action required. `bundling-family` watch remains dormant at gen 0 in the Phase 0C local loop. Re-evaluate if a future round adds a DTO-only consumer that imports `TranscriptTurn` without the rest of the bundle.

#### R5-DECOMP-F03. Binary acceptance criteria across rescoped WU-0C-N3.

**Severity: LOW.**

WU-0C-N3 acceptance criteria (lines 1769-1778) are 10 bullets, each binary:

- AC1 — every trait method invokes the corresponding `agents session` subcommand and maps stdout/stderr JSON into named WU-0C-N2 DTOs and `SessionOverrideError` variants. Test by inspecting subprocess invocations against fake-binary fixtures.
- AC2 — `unsupported-storage` exit/error code surfaces as `SessionOverrideError::UnsupportedStorage` with no harness-side transcript mutation. Binary: the named error variant is produced and no file is written.
- AC3 — `session-busy` exit code 13 surfaces as `SessionOverrideError::SessionBusy`. Binary: named exit code maps to named variant.
- AC4 — `--preimage-sha256` mismatch surfaces as `SessionOverrideError::PreimageMismatch`. Binary.
- AC5 — `pause-handshake` lease TTL respected for atomic mid-session override; `resume-handshake` always called on adapter drop when a lease token is held. Binary: testable via fake-binary lease-token observation + Drop instrumentation.
- AC6 — adapter construction calls `agents session schema-probe` and refuses operation when the output is malformed, required feature flags are absent, or `safe_for_import_replace` is false. Binary: three named failure conditions, each producing a refusal observable from the constructor result.
- AC7 — `truncate_after` and `append_turns` compose `read_transcript` + canonical JSONL boundary edit + `replace_transcript`. Binary on the composition observation; the forward-looking "if `agent-runner` gains finer-grained surfaces later, only this adapter mapping changes" qualifier is not a separate test criterion but a maintenance-scope statement co-located with the binary one.
- AC8 — fake `agents` binary fixture covers seven enumerated cases (success, unsupported storage, busy lease, preimage mismatch, unsafe schema probe, malformed JSON, missing subcommand). Binary: each named case is either present in the fixture or absent.
- AC9 — adapter records invocation evidence through WU-0B-09, override/audit receipts through WU-0B-15-compatible drafts, and write-failure recovery metadata through WU-0B-31 refs. Binary: three named append targets each verified or not.
- AC10 — explicit non-mutation list for `state.db`, provider transcript files, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, cross-provider migration settings. Binary: each named target is either touched or not.

No "service works" or "consistent" / "clean" subjective phrasing. Every method-bearing criterion names the call, valid result shape, and an enumerated error variant or invariant. AC7's forward-looking second clause is informational and does not undermine the binary first clause.

D2 self-classification at line 3502 ("Session override | DTO validators, trait-level fake adapter criteria, CLI schema-probe compatibility criteria, CLI adapter locate/export/import-replace/pause-handshake criteria, and registry lifecycle criteria.") is consistent with the per-WU AC blocks for N1 (lines 1714-1723), N2 (lines 1665-1673), N3 (lines 1769-1778), and N5 (lines 1829-1836).

**Recommendation:** No action required.

#### R5-DECOMP-F04. Refactored existing WUs remain coherent (no empty / overly-thin WUs after Option A).

**Severity: LOW.**

The round-5 simplification is subtractive at the new-WU level (drops N4) and rewrite-in-place at the existing-WU level (rescopes N3 contract + ACs + dependencies). It does not narrow any other WU. Spot-check confirms each round-4-narrowed WU retains the round-4-validated single-concern scope:

- **WU-0C-13 `AgentSessionCapture`** (line 1054 onward): revision-rationale pointer at line 1057 now reads "override writes use WU-0C-N1/N2/N3/N5" (was N1..N5 in r4). No AC change. Still owns `capture_agent_runner_session(request)` with binary ACs and explicit no-locate/parse/append/truncate/replace constraint.
- **WU-0C-15a/15b/15c/15d** (session-turn DTO + reader family): r5 only updates revision-rationale prose to point at "WU-0C-N3's CLI adapter behind WU-0C-N1" (line 1245) instead of "v1 override adapter." Code/test/AC blocks unchanged. No empty WU.
- **WU-0C-18 `AgentRunnerClient`** (line 1536 onward): revision-rationale pointer at line 1579 now reads "transcript write-back moved to WU-0C-N1/N2/N3/N5" (was N1..N5). No AC change. Still 11 binary ACs across 8 facade methods.
- **WU-0C-31 `RecoveryActionProcessorSkeleton`**: refactor-ledger row at line 3218 now reads "live replacement uses WU-0C-N3's pause-handshake/import-replace path" (was "live replacement remains blocked on pause-handshake" in r4). The skeleton's no-side-effect scope is preserved.
- **WU-0C-34 `TauriIpcCommandRouter`**: refactor-ledger row at line 3219 unchanged from r4 — still adds locate/metadata/registry read commands only, with the explicit no-replace/truncate/append-UI-command criterion preserved.

The refactor ledger's closing statement at line 3221 ("WU-0C-N4 was removed because schema probing is now the `agents session schema-probe` call inside WU-0C-N3. WU-0C-N3 was rescoped from direct DB/JSONL mutation to the CLI adapter.") is consistent with line-by-line inspection. No WU became empty after refactor; the placeholder "WU-0C-N4: Removed in r5" stub is intentionally a redirect, not an empty WU consuming inventory slots (it is correctly excluded from the inventory total of 76 and from the D1 audit table).

The single-concern PR constraint on rescoped N3 (line 1788) was strengthened relative to r4 — it now explicitly enumerates "direct SQLite writes, direct provider transcript writes, or a harness-side schema-probe wrapper WU" as out-of-scope, which closes the door on simplification drift.

**Recommendation:** No action required.

#### R5-DECOMP-F05. Stitch Notes preserves rounds 2-4 closure under r5 simplification.

**Severity: LOW.**

The round-3 closure pattern (foundation-row blocks + per-VS blocks + OptimizerRequest cross-slice contract) is preserved byte-for-byte in shape. Round-5 changes are uniformly substitutive (rewrite N1..N5 as N1/N2/N3/N5) and removal of `Blocked-on:` annotations whose upstream dependencies have landed. Specifically:

- **Foundation-row block.** SessionOverrideContract row at line 3586 reads "WU-0C-N1/N2/N3/N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs..." — the consumer VS list is unchanged from r4. No foundation row was removed; the row's consumer set is preserved.
- **Per-VS blocks.** The five SessionOverrideContract-consuming VS lines (VS-010 line 3610, VS-012 line 3612, VS-018 line 3618, VS-020 line 3620, VS-021 line 3621) each retained the SessionOverrideContract clause and removed the trailing `**Blocked-on:** ... v2 adapter migration needs ... pause-handshake ...` annotation. The annotation removal is consistent with the upstream-feature-landings premise: the agents binary now ships the named commands (lines 74-80 confirm), so the block-on conditions are satisfied. The 16 non-override VS lines are unchanged.
- **OptimizerRequest cross-slice contract** at line 3594 is preserved verbatim from rounds 3-4 (six emitter→queue pairs enumerated).
- **Cross-phase outgoing edges.** The four bullets at lines 3625-3628 were updated from `WU-0C-N1..WU-0C-N5` to `WU-0C-N1/N2/N3/N5`. The section heading at line 3623 still reads "Cross-phase outgoing edges added in round 4," which is historically accurate (these edges were added in r4 and rewritten in r5 to drop N4); not a content concern.
- **Refactor-incoming edges** at line 3563 dropped the WU-0A-03 `LocalStorageLayout` reference for WU-0C-N3 (consistent — N3 no longer reads local SQLite directly) and at line 3575 was rewritten as `WU-0C-N1/N2/N3/N5` consuming WU-0B-09/WU-0B-15/WU-0B-31 refs. No incoming edge was silently dropped — the WU-0A-03 removal aligns with N3's CLI-only scope.
- **Explicit non-ownership notes** at lines 3630-3637 add a definitive "Phase 0C does not perform direct `state.db` or per-CLI JSONL mutation. WU-0C-N3 `AgentRunnerCliAdapter` delegates locate/export/import-replace/pause-handshake/resume-handshake/schema-probe to the installed `agents` binary." This converts the round-4 conditional ("Direct `state.db` and per-CLI JSONL mutation is allowed only inside WU-0C-N3 ... pinned by WU-0C-N4 and replaceable by a future WU-0C-N1-compatible `AgentRunnerCliAdapter` after ... land.") into an unconditional invariant, which strengthens the stitching contract for downstream Phase 1/2/3 consumers.

The Stitch Notes block satisfies round-3 closure (foundation/per-VS/cross-slice triple), round-4 additivity (SessionOverrideContract foundation row + per-VS additions + cross-phase handoff), and round-5 simplification (block-on annotations dropped consistently with upstream landings).

**Recommendation:** No action required.

### Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R5-DECOMP-F01 | Per-schema-object ownership across rescoped N3 and removed N4 | LOW |
| R5-DECOMP-F02 | WU-0C-N2 DTO bundling unchanged from r4 (watch dormant) | INFO |
| R5-DECOMP-F03 | Binary acceptance criteria across rescoped WU-0C-N3 | LOW |
| R5-DECOMP-F04 | Refactored WUs remain coherent (no empty / thin WUs after Option A) | LOW |
| R5-DECOMP-F05 | Stitch Notes preserves rounds 2-4 closure under r5 simplification | LOW |

### What LOW requires

For Phase 0C round 5 Decomposition LOW to remain valid, the following conditions must hold:

1. WU count remains 76 with WU-0C-N1, WU-0C-N2, WU-0C-N3, and WU-0C-N5 each owning the schema_object/trait/service named in the D1 table (lines 3453-3456). WU-0C-N4 remains a redirect-only placeholder excluded from the inventory total.
2. WU-0C-N3 owns only `AgentRunnerCliAdapter` and its command mapping; it does not absorb `state.db` access, JSONL rendering, registry persistence, or IPC routes. The single-concern PR constraint at line 1788 must continue to forbid "direct SQLite writes, direct provider transcript writes, or a harness-side schema-probe wrapper WU."
3. WU-0C-N1 trait WU does not absorb adapter implementation; the trait module continues to forbid `state.db` access, JSONL rendering, crash recovery, registry persistence, and IPC routes (line 1733).
4. WU-0C-N2 acceptance criteria continue to forbid side effects (no transcript files, no state.db rows, no EvidenceArtifact / AuditEvent / GraphNode / SessionOverrideStore rows). N2 bundling stays bounded to the trait method signature surface; if a future round introduces a DTO-only consumer that imports individual N2 DTOs without the rest of the bundle, R5-DECOMP-F02 escalates from INFO to MEDIUM and triggers the round-2 systematic per-DTO split.
5. WU-0C-13/15a/15b/15c/15d/18/31/34 retain their round-4-narrowed-but-coherent scopes; refactor-ledger entries at lines 3211-3221 stay subtractive (no merging of concerns into these existing WUs). No empty WU emerges.
6. Refactored WUs' "never calls `replace_transcript`/`truncate_after`/`append_turns`" / "no provider routing / quota / account / auth / resume / porting" exclusion clauses remain in their acceptance criteria so the SessionOverrideContract boundary is enforceable at PR review.
7. Stitch Notes preserves the round-3 foundation-row + per-VS + OptimizerRequest cross-slice triple. SessionOverrideContract foundation row (line 3586), per-VS additions (lines 3610, 3612, 3618, 3620, 3621), and cross-phase edges (lines 3625-3628) remain in shape; per-VS clauses do not silently re-introduce harness-side raw-storage write paths.
8. The `bundling-family`, `state-machine-criteria-family`, `fix-created-family`, `dependency-encoding-family`, and `session-override-boundary-family` watches stay active. `fix-created-family` remains classified gen 0 externally driven (proposal-r6 / engineering-roadmap-r5 cascade plus upstream agent-runner feature landings). The `session-override-boundary-family` watch must propagate the simplification-drift item: do not recreate a harness-side DB adapter, harness-side schema wrapper WU, or per-CLI storage parser unless upstream removes the `agents session` contract.
9. The agents binary continues to expose `session locate`, `session export --format canonical-jsonl`, `session import-replace --preimage-sha256`, `session pause-handshake`, `session resume-handshake`, and `session schema-probe`. If any of these surfaces is withdrawn or breaking-changed upstream, the `Blocked-on:` annotations dropped in round 5 must be reinstated and N3 ACs revisited; that revision would re-fire `fix-created-family` at gen 1.

If condition (1), (2), (3), or (5) is violated, MEDIUM is the correct re-rating. If (2) and (3) are both violated, or if a new blob WU appears that bundles trait + adapter + registry, escalate to HIGH. If condition (9) is violated by upstream regression, reopen the round-4 finding set rather than re-rating round-5; the simplification premise no longer holds and the v1 adapter or its equivalent must be re-introduced as a separate WU.
