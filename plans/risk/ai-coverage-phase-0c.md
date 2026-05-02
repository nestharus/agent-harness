# AI — Coverage Risk Assessment (Phase 0C, rounds 4–5)

**Round 4 rating: LOW** (preserved). **Round 5 rating: LOW.**

## Scope and inputs

- Artifact: `product-strategy/ai-roadmap-phase-0c.md` (round 4 brownfield, 77 WUs — `### WU-0C-` count = 77; round 3 was 72 WUs).
- Cascade: `product-strategy/proposal.md` (proposal-r5, commit `c9ee3f9`) introduces the `SessionOverrideContract` axiom + agent-runner boundary; `product-strategy/engineering-roadmap.md` (engineering-roadmap-r4, commit `b58d8c9`) consumes the trait at VS-010/012/018/020/021 and binds the 5 SessionOverrideContract WUs to Phase 0C-r4.
- Per-phase audit history: `plans/audit/ai-roadmap-phase-0c.md` (r1 = MEDIUM/MEDIUM/MEDIUM; r2 = LOW/LOW/MEDIUM; r3 = LOW/LOW/LOW Phase 0C convergence; r4 added 5 WUs and refactored 11 existing WUs to remove per-CLI / agents-owned responsibilities).
- Round 4 mandate: brownfield cascade-driven by proposal-r5 + engineering-roadmap-r4. New WU IDs: WU-0C-N1 (trait), WU-0C-N2 (DTOs), WU-0C-N3 (v1 `AgentRunnerDbAdapter`), WU-0C-N4 (`AgentRunnerSchemaProbe`), WU-0C-N5 (`SessionOverrideStore` registry). Refactor ledger at lines 3260–3278 itemizes 11 narrowed WUs. Per-phase finding IDs use the `R4-COVERAGE-F<NN>` prefix.
- Convergence rule from `~/ai/conventions/audit-history.md`: r1=LOW, r2=LOW, r3=LOW for Coverage in this per-phase loop. r4 is brownfield-driven by an external cascade (proposal-r5/engineering-roadmap-r4); coverage rules are re-checked against the new WU surface and the refactored existing WUs.

## Round 4 diff scope (verified)

The r4 commit `aef7a5667ab617dc92ad44554f8aa8fdd3071117` adds 5 new WUs (`WU-0C-N1..WU-0C-N5`) under the new "SessionOverrideContract and v1 adapter foundation" group; revises 11 existing WUs (`WU-0C-11a`, `WU-0C-13`, `WU-0C-13a`, `WU-0C-13b`, `WU-0C-15a`, `WU-0C-15b`, `WU-0C-15c`, `WU-0C-15d`, `WU-0C-18`, `WU-0C-31`, `WU-0C-34`) per the Round 4 Refactor Ledger; expands the Dependency Graph with WU-0C-N1..N5 incoming/outgoing edges; re-derives the Parallelization Map (10 waves, sum 77); expands Stitch Notes "Outgoing edges by engineering-roadmap Phase 0 foundation row" with a new SessionOverrideContract row (line 3645); adds explicit SessionOverrideContract entries to per-VS outgoing blocks for VS-010, VS-012, VS-018, VS-020, VS-021 (lines 3669, 3671, 3677, 3679, 3680); and adds three Phase 1/2/3 cross-phase outgoing edges (lines 3684–3687).

## Findings

### R4-COVERAGE-F01. Proposal-r5 contract operations are fully covered with binary criteria

**Severity: NONE (positive)**

`product-strategy/proposal.md` lines 1562–1593 (and the harness boundary table at lines 75–88) define the 7 operations of `SessionOverrideContract`: `schema_version_probe`, `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, `append_turns`, `get_session_metadata`. The Phase 0C r4 artifact binds each operation to a Phase 0C WU with binary acceptance criteria:

| Proposal-r5 operation | Owner WU | Binary criterion |
|---|---|---|
| `schema_version_probe` | WU-0C-N4 (`AgentRunnerSchemaProbe`) + WU-0C-N1 trait crit 4 | "probe(state_db_path, agents_binary) ... returns supported = true"; "schema_version_probe is invoked before mutation by failing the operation when the fake probe returns UnsupportedSchema." |
| `locate_session` | WU-0C-N1 trait crit 2 + WU-0C-N3 v1 crits 1–2 | "fake in-memory adapter can return success for locate_session"; "returns SessionNotFound, AmbiguousSession, or UnsupportedStorage for missing, duplicate, or unsupported fixtures and performs no file mutation." |
| `read_transcript` | WU-0C-N1 trait crit 2 + WU-0C-N3 v1 crit 3 | "returns ordered TranscriptTurn fixtures with source offsets and hashes preserved ... malformed records become unsupported_record turns rather than being dropped." |
| `replace_transcript` | WU-0C-N1 trait crits 2/4/5 + WU-0C-N3 v1 crits 4–5 | refusal-on-unsupported-schema / busy / preimage / storage / render before rename; success-path same-directory-temp + pending-record + atomic-rename + DB-transaction + receipt. |
| `truncate_after` | WU-0C-N1 trait crits 2/4/5 + WU-0C-N3 v1 crit 6 | "truncates only at a valid turn boundary and rejects boundaries that split a tool-call/result dependency or compaction-boundary invariant." |
| `append_turns` | WU-0C-N1 trait crits 2/4/5 + WU-0C-N3 v1 crit 7 | "appends only adapter-renderable TranscriptTurn records, rejects duplicate turn IDs, and refuses unsupported append cases before file mutation." |
| `get_session_metadata` | WU-0C-N1 trait crit 2 (read-only) | "fake in-memory adapter can return success for ... get_session_metadata ... using WU-0C-N2 DTO fixtures." |

Every operation has at least one binary criterion that names success and at least one named-error variant. WU-0C-N4 carries 7 method-level criteria for the probe surface (column-level fingerprinting, refusal on unsupported schema/binary, deterministic fingerprints, read-only SQLite open, no PRAGMA mutation). WU-0C-N1 carries 8 trait-level criteria; WU-0C-N3 carries 10 adapter-level criteria (5 method-paths + crash-recovery + flock-locking + agents/config no-edit invariant). No method, enum, or state machine in this family is left without a binary criterion.

**Recommendation:** No action required.

---

### R4-COVERAGE-F02. Engineering-roadmap-r4 slice references resolve to specific WUs

**Severity: NONE (positive)**

`product-strategy/engineering-roadmap.md` line 36 names the SessionOverrideContract Phase 0C foundation row with consumers `VS-010, VS-012, VS-018, VS-020, VS-021`. Line 661 confirms: "VS-010, VS-012, VS-018, VS-020, and VS-021 depend on this trait, not on either adapter." The Phase 0C r4 ai-roadmap stitches every consumer slice to the new WUs:

| Consumer slice | Stitch Notes line | Cited WUs |
|---|---|---|
| VS-010 turn-decomposition / detail-injection write-back | 3669 | "SessionOverrideContract WU-0C-N1..WU-0C-N5 for turn-decomposition/detail-injection write-back" |
| VS-012 repack transcript replacement | 3671 | "SessionOverrideContract WU-0C-N1..WU-0C-N5 for repack transcript replacement" |
| VS-018 accepted worker-output write-back | 3677 | "SessionOverrideContract WU-0C-N1..WU-0C-N5 for accepted worker-output write-back" |
| VS-020 refusal/quarantine metadata | 3679 | "SessionOverrideContract WU-0C-N1..WU-0C-N5 for refusal/quarantine metadata" |
| VS-021 changed-session-contract evidence | 3680 | "SessionOverrideContract WU-0C-N1..WU-0C-N5 for changed-session-contract evidence" |

The orchestration prompt's reference to `VS-015` is correctly NOT carried as a SessionOverrideContract dependency: engineering-roadmap-r4 lines 36, 343–361, and 661 keep VS-015 worker dispatch read-only over `agents -m <model> -p <project> -f <prompt>` without per-CLI session-jsonl knowledge in the harness, so the absence of VS-015 from the WU-0C-N1..N5 consumer list is correct, not a gap. VS-015's per-VS block at line 3674 lists only read-only foundations (policy, configuration, budget, render, agent-runner, provider, hook/plugin, audit), matching engineering-roadmap line 36's omission of VS-015.

The cross-phase outgoing block at lines 3684–3687 also pre-stitches Phase 1 worker-launcher/output reintegration, Phase 2 turn-decomposition/detail-injection-router, and Phase 3 repack-planner consumers, so Phase 1+ AI roadmaps will not need to back-fill the trait dependency.

**Recommendation:** No action required.

---

### R4-COVERAGE-F03. Every SessionOverrideError variant is reachable through fixtures

**Severity: NONE (positive)**

WU-0C-N2's contract block (lines 1646–1659) enumerates 13 `SessionOverrideError` variants; the proposal-r5 failure-defaults table at proposal lines 1670–1677 names 8 (`unsupported_schema`, `session_not_found`, `ambiguous_session`, `unsupported_storage`, `session_busy`, `preimage_mismatch`, `post_rename_db_failure`, `adapter_render_failure`). The Phase 0C r4 artifact extends with `InvalidTurnBoundary`, `UnsupportedAppend`, `IoFailure`, `DbFailure`, `QuarantinedStorageConflict` to cover boundary, append, IO, DB, and crash-recovery failure modes called out in proposal lines 1660–1664 (race handling) and proposal lines 1664+ (failure defaults).

WU-0C-N2 acceptance criterion 2 binds reachability: "Every role, storage-kind, mutability, transcript-state, compaction-state, operation, and error variant is reachable through a named fixture." That criterion explicitly enumerates `error variant` as one of the seven required reachability dimensions, making the binary check enforceable in the test agent without seeing the implementation.

WU-0C-N1 trait criterion 3 reinforces the binding at the consuming surface: "A fake adapter error fixture maps each `SessionOverrideError` variant to a stable Rust enum discriminant and TypeScript string without lossy catch-all mapping." This forbids the family-level "reduce to one error" anti-pattern that fired in earlier monolithic-roadmap rounds.

The operation-specific failure modes from proposal-r5 are bound to specific WU-0C-N3 criteria: `replace_transcript` refusal on unsupported_schema/session_busy/preimage_mismatch/unsupported_storage/adapter_render_failure (crit 4); `truncate_after` rejection at invalid boundary (crit 6, returns `InvalidTurnBoundary`); `append_turns` rejection on duplicate/unsupported (crit 7, returns `UnsupportedAppend`); crash recovery to `QuarantinedStorageConflict` (crit 8); flock failure to `SessionBusy` (crit 9). Each named error path has a binary criterion at the v1 adapter, plus a fake-adapter discriminant test at the trait.

`TempHarnessError` named in the orchestration prompt belongs to Phase 0A WU-0A-14a (`src-tauri/src/test_harness/temp_harness.rs`) and is out of scope for Phase 0C; it is not a Phase 0C error type and the round-4 cascade does not reuse the name. Coverage of `TempHarnessError` was confirmed in the Phase 0A-converged risk gates (round 2 of that loop) and is not Phase 0C's responsibility.

**Recommendation:** No action required.

---

### R4-COVERAGE-F04. v1 adapter invariants each have binary criteria

**Severity: NONE (positive)**

The proposal-r5 v1-adapter axiom (proposal lines 647, 1660–1679, 1731–1738) requires four invariants. Each maps to a binary acceptance criterion in WU-0C-N3 and/or WU-0C-N1:

| v1 invariant | Owner criterion | Binary form |
|---|---|---|
| Schema-version pinning | WU-0C-N4 crit 1–7; WU-0C-N1 crit 4; WU-0C-N3 crit 4 | Probe returns `supported = true` only for pinned table/column fingerprint set; trait write methods fail when fake probe returns `UnsupportedSchema`; v1 `replace_transcript` refuses outside the pinned range before any rename. |
| Refuse-rather-than-corrupt | WU-0C-N3 crits 4, 6, 7; WU-0C-N1 crit 5 | All three write methods return typed error variants (`UnsupportedSchema`, `SessionBusy`, `PreimageMismatch`, `UnsupportedStorage`, `AdapterRenderFailure`, `InvalidTurnBoundary`, `UnsupportedAppend`) before any file/DB mutation; trait write methods reject missing `OverridePreconditions` before invoking the fake mutation hook. |
| Two-phase-write recovery | WU-0C-N3 crits 5, 8 | Successful `replace_transcript` writes a same-directory temp file → records pending override → atomically renames → updates only required state-row consistency in one SQLite transaction → returns `OverrideReceipt` with preimage/postimage hashes. Crash-injection fixtures after temp-write, after rename, and after DB transaction recover deterministically to `committed`, `rolled_back`, or `QuarantinedStorageConflict` states. |
| flock locking | WU-0C-N3 crit 9 | "uses flock-style/session-idle locking and returns `SessionBusy` when it cannot prove no in-flight `agents` write owns the same session." |

The "no edits to `agents`, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, or cross-provider migration settings" anti-scope from proposal lines 1733–1737 is bound to WU-0C-N3 crit 10 as a binary no-write invariant.

WU-0C-N5's `SessionOverrideStore` registry crits 1–8 carry the 5 lifecycle transitions (`begin_pending`, `commit pending->committed`, `rollback pending->rolled_back`, `quarantine pending|crash-recovery->quarantined_storage_conflict`, `list_by_session ordered`), so the recovery state machine that backs invariant 3 is itself a state-machine-with-binary-transition criterion (R3-style D2 satisfied at the per-WU level).

**Recommendation:** No action required.

---

### R4-COVERAGE-F05. Block-on annotations match the agent-runner feature-request register

**Severity: NONE (positive)**

The proposal-r5 agent-runner feature-request register at `proposal.md` lines 1717–1729 enumerates 5 supported-surface features:

1. `agents session locate <id>`
2. `agents session export <id>`
3. `agents session import-replace <id>`
4. Pause / lock handshake
5. Schema-version / supported-surface probe

Engineering-roadmap-r4 carries the same table at lines 654–659. The Phase 0C r4 WUs annotate `Blocked-on` consistently with this register:

- WU-0C-N1 trait (line 1738): "v2 adapter migration is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe."
- WU-0C-N3 v1 adapter (line 1845): "v2 swap-later is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe. Atomic mid-session override remains blocked on `agents pause-handshake`."
- WU-0C-N4 schema probe (line 1786): "Replacement by an upstream supported-surface probe is blocked on `agents schema-version probe` or equivalent."
- WU-0C-N5 registry (line 1903): "v2 adapter-kind activation is blocked on `agents session locate/export/import-replace` and schema-version probe."

The corresponding consumer-VS Stitch-Notes blocks carry matching `Blocked-on:` lines: VS-010 (line 3669) "v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`"; VS-012 (line 3671), VS-018 (line 3677) — same three; VS-020 (line 3679) — pause-handshake + import-replace; VS-021 (line 3680) — locate/export/import-replace + schema-version probe.

The block-on graph is internally consistent: the v1 adapter is permitted to ship today; the v2 adapter and atomic mid-session override are gated on the named upstream features; the consumer slices inherit the blocks transitively. No WU references an unnamed upstream feature, and no upstream feature in the proposal-r5 register is left without a corresponding `Blocked-on:` clause in the AI roadmap.

**Recommendation:** No action required.

---

### R4-COVERAGE-F06. New WUs and refactored WUs have inbound and outbound edges; no orphans

**Severity: NONE (positive)**

Each of the 5 new WUs declares both incoming and outgoing edges:

| WU | Incoming | Outgoing |
|---|---|---|
| WU-0C-N2 (DTOs) | WU-0B-09, WU-0B-15, WU-0B-21 | WU-0C-N1, WU-0C-N3, WU-0C-N4, WU-0C-N5; VS-010, VS-012, VS-018, VS-020, VS-021 |
| WU-0C-N1 (trait) | WU-0C-N2; WU-0B-09, WU-0B-15 | WU-0C-N3, WU-0C-N5; VS-010, VS-012, VS-018, VS-020, VS-021 |
| WU-0C-N4 (probe) | WU-0C-N2; WU-0A-03, WU-0A-15 | WU-0C-N3; WU-0C-N1 v1-refusal tests |
| WU-0C-N3 (v1 adapter) | WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16; WU-0A-03, WU-0A-15, WU-0B-09, WU-0B-15 | VS-010, VS-012, VS-018, VS-020, VS-021 |
| WU-0C-N5 (registry) | WU-0C-N1, WU-0C-N2; WU-0B-09, WU-0B-15, WU-0B-31 | WU-0C-N3 crash recovery; VS-001, VS-003, VS-020, VS-021 |

Dependency Graph block at lines 3332–3336 enumerates the same incoming edges; Stitch Notes line 3645 enumerates the foundation-row outgoing edge "SessionOverrideContract WU-0C-N1..WU-0C-N5 feed VS-010, VS-012, VS-018, VS-020, VS-021"; Stitch Notes lines 3684–3687 enumerate the cross-phase outgoing edges.

Refactored WUs were narrowed in scope (provider routing / per-CLI session storage / direct transcript writes removed) but all retained their pre-r3 incoming and outgoing edges. The Round 4 Refactor Ledger at lines 3260–3278 explicitly tracks "Kept" status for all 11 narrowed WUs ("No WU became empty after refactor. No WU was removed or merged" — line 3278). Spot-check confirms WU-0C-13 (refactored) keeps incoming WU-0C-11a/11b/13a/13b + WU-0A-15 + WU-0B-20-future, and outgoing to WU-0C-18; WU-0C-15d keeps incoming WU-0C-13/15a/15b/15c + WU-0B-09/20, and outgoing to provenance/worker-evidence/recovery slices; WU-0C-18 keeps incoming WU-0C-12/13/14/15d/16/17 and outgoing as the unified facade.

The Critical-Path block at lines 3420–3425 stitches the new WUs into the foundation chain: `WU-0C-N2 -> WU-0C-N1 -> WU-0C-N5` and `WU-0C-N2 -> WU-0C-N4 -> WU-0C-N3`, with WU-0C-N3 sitting in wave 4 of the 10-wave Parallelization Map (line 3454). No new WU is parallel-isolated or trapped in a downstream-only edge cone.

**Recommendation:** No action required.

---

### R4-COVERAGE-F07. Refactored WUs retain functional acceptance criteria after losing per-CLI scope

**Severity: NONE (positive)**

The 11 refactored WUs from the Round 4 Refactor Ledger (lines 3260–3278) all carry binary functional acceptance criteria after the narrowing. Spot-check across a representative sample of method-bearing service WUs:

- **WU-0C-13** `AgentSessionCapture` (line 1026) keeps 6 ACs: success path for `agents`-reported-session, success path for trace-session, `substrate_gap` path, `MissingCaptureSource` error path, the no-mutation invariant, and the no-WorkerRun/OrchestratorTurn/QuestionArtifact/RecoveryAction-row invariant. The function `capture_agent_runner_session(request)` retains a typed-result contract.
- **WU-0C-15d** `SessionTurnsReader` (line 1291) keeps 6 ACs: normalized-turn read success, parent/sidechain/compaction metadata preservation through opaque refs, `missing_locator_state` explicit, `substrate_gap_state` explicit, read-only invariant, and the new SessionOverrideContract-no-mutation invariant ("never calls `replace_transcript`, `truncate_after`, or `append_turns`").
- **WU-0C-18** `AgentRunnerClient` Facade (line 1536) keeps 11 ACs: per-method delegation criterion for each of the 8 facade methods (`spawn`, `capture_session`, `read_trace_tree`, `read_session_turns`, `read_config_snapshot`, `provider_diagnostics`, `cancel`, `resume`); the no-transcript-write invariant ("exposes no method that opens, rewrites, truncates, appends, or migrates provider transcript files"); the no-provider-routing invariant ("does not choose provider accounts, rewrite providers.toml or sessions.toml, mutate quota scripts, or implement cross-provider session porting"); and a fake-backed success + invalid-input/error fixture per method.
- **WU-0C-31** `RecoveryActionProcessorSkeleton` and **WU-0C-34** `TauriIpcCommandRouter` retain their pre-r3 binary criteria; the only delta is an additive incoming dependency (WU-0C-N5 metadata for recovery; WU-0C-N1/N5 read commands for IPC) — no criterion was removed or weakened. The IPC router's revised constraint "no UI command for replace/truncate/append in Phase 0C" is itself a binary scope-deny check.

Each DTO refactor (WU-0C-11a, WU-0C-13a, WU-0C-13b, WU-0C-15a, WU-0C-15b, WU-0C-15c) keeps its serde/JSON round-trip criterion plus a validator success and at least one named-error path. None of the refactored WUs lost a method-bearing surface; the narrowing replaced provider-CLI-specific success paths with provider-neutral `agents`/`agent-runner`-evidence success paths and added explicit no-mutation invariants. The functional-acceptance bar for D2 is not regressed.

**Recommendation:** No action required.

---

### R4-COVERAGE-F08. WU-0C-N3 v1 adapter does not list a method-level criterion for `get_session_metadata`

**Severity: INFO**

WU-0C-N3's contract block at lines 1798–1805 declares the v1 adapter implements all 7 SessionOverrideContract operations including `get_session_metadata`. The 10 acceptance criteria explicitly cover `locate_session` (crits 1–2), `read_transcript` (crit 3), `replace_transcript` (crits 4–5), `truncate_after` (crit 6), `append_turns` (crit 7), crash recovery (crit 8), flock locking (crit 9), and the agents-config no-edit invariant (crit 10). There is no v1-adapter-level binary criterion that names `get_session_metadata` as the call under test.

This is INFO rather than MEDIUM under the same standard as R2-COVERAGE-F06 (the prior round's INFO on WU-0C-20 success-path freshness) for two reasons:

1. WU-0C-N1 trait criterion 2 binds method-level reachability for `get_session_metadata` at the trait surface ("fake in-memory adapter can return success for `locate_session`, `read_transcript`, `get_session_metadata`, and each write operation using WU-0C-N2 DTO fixtures"). The v1 adapter implements the trait, so the trait test exercises the adapter.
2. `get_session_metadata` is read-only (returns `SessionMetadata`); WU-0C-N2 crit 2 already requires every `transcript_state`, `compaction_state`, and `capture_method` value to be reachable through a named DTO fixture, so the variant reachability is preserved at the per-schema-object level even without an explicit v1-adapter method test.

No family-level extension. The criterion satisfies D2 because every method has a binary criterion *somewhere in the contract surface*, which the test agent reads as one combined surface.

**Recommendation:** Optional: add a one-line WU-0C-N3 criterion of the form "Calling `get_session_metadata(session_id)` against a supported fixture returns a `SessionMetadata` with the expected `transcript_state` and `last_observed_turn_id`, and returns `SessionNotFound` for an unknown session." Not required for LOW.

---

### R4-COVERAGE-F09. WU-0C-N1 trait write-method criteria collapse three operations into shared fixtures

**Severity: INFO**

WU-0C-N1 trait criterion 4 binds schema-probe-before-mutation for "`replace_transcript`, `truncate_after`, and `append_turns`" jointly; criterion 5 binds preimage / idle precondition handling for "Write methods" (also jointly). The trait surface treats the three write methods as a family — the schema-probe and precondition checks are common, so a shared fixture set is the natural test layout.

A strict reading of D2 ("Method criteria name the exact call, valid result shape, and at least one error variant") would prefer one explicit success fixture per write method at the trait level. The trait test at criterion 2 ("fake in-memory adapter can return success for ... each write operation using WU-0C-N2 DTO fixtures") collectively exercises all three; the per-method success path is covered, but the criterion phrasing is "each write operation" rather than three distinct named fixtures. Per-method failure paths are exercised at WU-0C-N3 (crits 4 / 6 / 7) for the v1 adapter, so the contract surface is binary at the consuming WU.

INFO under the same threshold as R4-COVERAGE-F08: family-level binding + per-WU reinforcement keeps every method binary; no family-level gap is introduced.

**Recommendation:** Optional: split WU-0C-N1 criterion 2 into one criterion per write method, or add a "trait golden fixtures table" line that lists `replace_transcript`, `truncate_after`, and `append_turns` separately. Not required for LOW.

---

### R4-COVERAGE-F10. Round-2 INFO findings remain INFO and do not extend into a family pattern

**Severity: INFO**

The two prior-round INFO findings carry forward unchanged:

- **R2-COVERAGE-F06** (WU-0C-20 success-path freshness implicit at service level; DTO covers all four `freshness_state` variants but the service method criteria do not name `fresh` and `manual` explicitly). The r4 diff does not touch WU-0C-20 contracts or criteria, so the INFO posture is preserved. No family-level extension into the new SessionOverrideContract WUs.
- **R2-COVERAGE-F07** (WU-0C-17b/WU-0C-17 treat `route_denial_reasons` taxonomy as inherited string list, delegating enumeration to Phase 0B WU-0B-19 `CapabilityFingerprint`). The r4 diff does not touch these WUs; the boundary remains consistent with engineering-roadmap pushback P-5 and the round-3 closure.

R4-COVERAGE-F08 (get_session_metadata not at v1-adapter level) and R4-COVERAGE-F09 (trait write-methods share fixtures) are the round-4 INFO additions. None of the four INFOs together form a family-level pattern that would warrant escalation.

**Recommendation:** No action required.

---

## Oscillation classification

Per `~/ai/conventions/audit-history.md`:

- **Same-label**: 0. No prior Phase 0C round's Coverage finding recurs at MEDIUM in r4. R3 was LOW with no MEDIUMs; r2 was LOW; r1 had one MEDIUM (`R1-COV-F01` identity/conflict shell missing) closed in r2.
- **Same-family** (`state-machine-criteria-family`, `bundling-family`): 0. No method/enum/state-machine binding declared in any prior round has been weakened or removed in r4. The 13-state `OptimizerCycleStateMachine` at WU-0C-28, the `ConflictRecord` workflow at WU-0C-29d, and the `RecoveryAction` transitions at WU-0C-30 are byte-untouched. The new state-machine-with-transitions in WU-0C-N5 (`SessionOverrideRecord` lifecycle: `pending` → `committed` / `rolled_back` / `quarantined_storage_conflict`) is decomposed as a service-level state machine with binary transition criteria from the start (crits 3–5 of WU-0C-N5).
- **Fix-created**: gen 0. The cascade is externally driven (proposal-r5 + engineering-roadmap-r4); no Phase 0C-internal fix in r4 created an orphan or coverage gap. The 5 new WUs are decomposed by concern (DTOs / trait / probe / adapter / registry) per the round-2 systemic D1 split rule. No refactored WU lost a method, enum, state machine, or routing invariant; each retained a binary acceptance criterion or replaced a per-CLI criterion with a provider-neutral one.
- **Two-generation / named three-generation / named four-generation**: not fired. The parent monolithic loop's `state-machine-criteria-family` precedent (named four-generation) is contained — the per-phase decomposition rule "every method gets binary criteria" is honored systemically across the new 5 WUs.

The `session-override-boundary-family` watch newly registered in the r4 audit history (lines 64–65 of `plans/audit/ai-roadmap-phase-0c.md`) is a forward-looking watch on the harness/agent-runner ownership boundary, not a Coverage-gate firing. Coverage emits no MEDIUM in r4, so no escalation is warranted.

## Summary table

| ID | Finding | Severity |
|----|---------|----------|
| R4-COVERAGE-F01 | All 7 proposal-r5 SessionOverrideContract operations covered with binary criteria across WU-0C-N1/N3/N4 | NONE (positive) |
| R4-COVERAGE-F02 | Engineering-roadmap-r4 SessionOverrideContract consumer slices VS-010/012/018/020/021 stitched at per-VS outgoing blocks; VS-015 correctly excluded | NONE (positive) |
| R4-COVERAGE-F03 | Every SessionOverrideError variant (8 proposal-named + 5 expansions) reachable through fixtures via WU-0C-N2 crit 2 + WU-0C-N1 crit 3; TempHarnessError out of scope (Phase 0A) | NONE (positive) |
| R4-COVERAGE-F04 | v1 adapter invariants (schema pinning, refuse-rather-than-corrupt, 2-phase-write recovery, flock locking) each have a binary AC across WU-0C-N3/N4/N1 | NONE (positive) |
| R4-COVERAGE-F05 | Block-on annotations match agent-runner feature-request register (locate / export / import-replace / pause-handshake / schema-version probe) at every consuming WU and per-VS block | NONE (positive) |
| R4-COVERAGE-F06 | All 5 new WUs and 11 refactored WUs have both inbound and outbound edges; no orphan WUs in the 77-WU graph | NONE (positive) |
| R4-COVERAGE-F07 | Refactored WUs retain functional binary acceptance criteria after narrowing; per-CLI scope replaced with provider-neutral evidence and explicit no-mutation invariants | NONE (positive) |
| R4-COVERAGE-F08 | WU-0C-N3 v1 adapter criteria do not name `get_session_metadata` at method level; trait surface (WU-0C-N1 crit 2) covers it | INFO |
| R4-COVERAGE-F09 | WU-0C-N1 trait write-method criteria 4–5 are shared across `replace_transcript`/`truncate_after`/`append_turns`; per-WU reinforcement at WU-0C-N3 keeps each method binary | INFO |
| R4-COVERAGE-F10 | Round-2 INFO findings (R2-COVERAGE-F06 freshness, R2-COVERAGE-F07 denial-reason passthrough) unchanged at INFO; no family-level extension | INFO |

## What LOW requires

The LOW rating depends on the following conditions remaining true in subsequent rounds:

1. Every Phase 0C work unit declaring a method, enum, state machine, or routing invariant continues to carry a binary acceptance criterion that names the call, valid result, and at least one error variant or invalid-transition path. The 5 new WUs (WU-0C-N1..N5) and the 11 refactored WUs all satisfy this as of r4.
2. Future revisions do not reintroduce generic "service works" or "schema migrates" language to satisfy criteria.
3. Inherited Phase 0B enum taxonomies (`OptimizerRequest`, `IdentityEvent`, `ConflictRecord`, `RecoveryAction`) and the new `SessionOverrideError` taxonomy continue to be bound at the consuming Phase 0C WU through "Every variant from WU-0X-NN is reachable through fixtures" criteria.
4. New WUs added to Phase 0C in any subsequent revision are audited against the same rule before merge; the round-2 systemic application of D1 and the round-4 SessionOverrideContract decomposition do not regress under future targeted fixes.
5. The four INFO findings (R4-COVERAGE-F08, F09, F10 = R2-COVERAGE-F06+F07) remain at INFO and do not extend into a family-level pattern. If a future revision adds new write-back service WUs without per-method criteria, those WUs must satisfy method-level binary criteria at the v1/v2 adapter or directly at the trait test surface, not implicitly.
6. The proposal-r5 SessionOverrideContract operation set (`schema_version_probe`, `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, `append_turns`, `get_session_metadata`) is unchanged. If proposal-r6+ adds a new operation or a new error variant, the AI roadmap must add a corresponding criterion before the next coverage gate.
7. The agent-runner feature-request register (5 named features) is unchanged. If the register grows or shrinks, the `Blocked-on:` clauses on WU-0C-N1..N5 and on per-VS blocks must be updated in lockstep.
8. The Round 4 Refactor Ledger (lines 3260–3278) remains the audit trail for any future narrowing of agent-runner / session-turn WUs. Any future narrowing must record (1) the violation under repair, (2) the fix, (3) the SessionOverrideContract dependency, and (4) the kept/merged/removed disposition. No WU may be silently emptied or merged.
9. The cross-phase outgoing edges added in r4 (Phase 1 worker launcher / output reintegration; Phase 2 turn-decomposition / detail-injection-router; Phase 3 repack-planner) are honored when those Phase 1+ AI roadmaps are produced — downstream WUs must declare incoming edges from WU-0C-N1..N5 rather than re-implementing transcript write-back.

---

# Round 5 entry — Coverage Risk Assessment

**Round 5 rating: LOW.**

## Round 5 scope and inputs

- Artifact: `product-strategy/ai-roadmap-phase-0c.md` at commit `1548cf4` (round 5 brownfield, Option A).
- Cascade: proposal-r6 + engineering-roadmap-r5 — agent-runner shipped all five SessionOverrideContract feature requests (`agents session locate`, `agents session export`, `agents session import-replace`, `agents session pause-handshake`/`resume-handshake`, `agents session schema-probe`). The v1 direct `AgentRunnerDbAdapter` is dropped; `AgentRunnerCliAdapter` is the only `SessionOverrideContract` implementation.
- Per-WU edits in r5: WU-0C-N3 rescoped to `AgentRunnerCliAdapter` against landed `agents session …` surfaces; WU-0C-N4 dropped (replaced with a "Removed in r5" stub that redirects to WU-0C-N3 schema-probe AC); all `Blocked-on:` annotations removed across Phase 0C; WU count 77 → 76 (the WU-0C-N4 stub heading remains for diff-historical reference but contains no Contract / Test boundary / Code boundary / acceptance criteria — it is not a WU under D1 ownership and is excluded from the Parallelization Map and dependency graph). Wave sums re-derived to 23+18+13+6+5+4+2+2+2+1 = 76 (line 3388 of the artifact).
- Convergence rule from `~/ai/conventions/audit-history.md`: r1=LOW, r2=LOW, r3=LOW, r4=LOW for Coverage in this per-phase loop. r5 is brownfield-driven by an external cascade (proposal-r6/engineering-roadmap-r5); coverage rules are re-checked against the rescoped WU-0C-N3 surface, the dropped WU-0C-N4, and the unchanged remainder of the inventory.

## Round 5 diff scope (verified)

The r5 commit `1548cf4d13f2159fe600d5cb0ca040a98d89a00d` makes only the following diffs to `product-strategy/ai-roadmap-phase-0c.md`:

1. WU-0C-N3 contract block (lines 1739–1790) replaced: schema_object now `Rust service object implementing SessionOverrideContract` with constructor `AgentRunnerCliAdapter::new(agents_binary, evidence_writer, audit_writer, recovery_writer)`; the trait→CLI mapping enumerates `schema_version_probe → agents session schema-probe`, `locate_session → agents session locate <id> [--json]`, `read_transcript → agents session export <id> [--format canonical-jsonl]`, `replace_transcript → agents session import-replace <id> --from-file <path> [--preimage-sha256 <hex>]`, `truncate_after / append_turns → read_transcript + canonical JSONL edit + replace_transcript`, `get_session_metadata → agents session locate <id> --json mapped to the metadata subset`, plus the atomic-mid-session-override `pause-handshake` → `import-replace` → `resume-handshake` triple.
2. WU-0C-N3 acceptance criteria replaced: 9 method-level binary criteria (line 1769–1778; see F01 below), revised single-concern PR constraint (no harness-side schema-probe wrapper WU), and a new "Revision rationale" note that classifies the change as externally driven `fix-created-family` gen 0.
3. WU-0C-N3 dependencies and Parallelizable-with lines reduced: removes incoming `WU-0C-N4` and `WU-0C-16`; keeps `WU-0C-N1`, `WU-0C-N2`, `WU-0A-15`, `WU-0B-09`, `WU-0B-15`, `WU-0B-31`. Wave moves from 4 → 3.
4. WU-0C-N4 contract block (formerly lines 1735–1790 in r4) replaced with a single removed-in-r5 paragraph (line 1735–1737). No Contract, Test boundary, Code boundary, acceptance criteria, dependencies, parallelization, or single-concern PR constraint remain.
5. All `Blocked-on:` annotations across Phase 0C removed. `grep -in "blocked-on\|Blocked on\|blocked on" product-strategy/ai-roadmap-phase-0c.md` returns 0 hits; remaining `blocked` strings are domain values (`budget_state = blocked`, `blocked_refs`, "poison-quarantined … nodes are blocked unless policy explicitly permits …"), not annotations.
6. Round 4 Refactor Ledger gains a "Round 5 update" footnote (line 3221) clarifying the WU-0C-N4 removal and WU-0C-N3 rescope; the 11 ledger rows themselves are unchanged.
7. Dependency Graph block (lines 3275–3278) updated to the four-line N-family edge set: N2 ← 0B; N1 ← N2 + 0B; N3 ← N1, N2, 0A-15, 0B-09, 0B-15, 0B-31; N5 ← N1, N2, 0B. WU-0C-N4 row removed.
8. Parallelization Map (lines 3392–3401) re-derived: Wave 1 (23 WUs) carries WU-0C-N2; Wave 2 (18 WUs) carries WU-0C-N1; Wave 3 (13 WUs) carries WU-0C-N3 and WU-0C-N5; remaining waves unchanged. Sum 76.
9. Critical Path "Session override path" block (lines 3361–3366) replaced: `WU-0C-N2 → WU-0C-N1 → WU-0C-N5` and `WU-0C-N1 → WU-0C-N3` (two short paths; no WU-0C-N4 node).
10. Stitch Notes "SessionOverrideContract WU-0C-N1/N2/N3/N5 …" lines updated across foundation row (3586), per-VS blocks for VS-010/012/018/020/021 (3610, 3612, 3618, 3620, 3621), cross-phase outgoing block (3625–3628), and explicit non-ownership notes (3637). All references switched from "WU-0C-N1..N5" to "WU-0C-N1/N2/N3/N5"; no Phase 0C consumer or cross-phase outgoing edge references the dropped WU-0C-N4.
11. Run Report family-watch table (lines 3531, 3539, 3541, 3552) records the round-5 fix-created-family gen 0 classification and the simplification-drift watch.

No other Contract, Test boundary, Code boundary, acceptance criterion, dependency, parallelization, or stitch line is changed in r5. The 70+ WUs outside the WU-0C-N1..N5 family are byte-untouched (verified by `git show 1548cf4 --stat`: only `plans/audit/ai-roadmap-phase-0c.md` and `product-strategy/ai-roadmap-phase-0c.md` change; the diff localizes to the WU-0C-N3/N4 contract blocks, the dependency-graph N-family rows, the Parallelization Map header/wave-1-3 rows, the Critical Path session-override block, the Refactor Ledger footnote, the Run Report family-watch rows, and the Stitch Notes SessionOverrideContract references).

## Findings

### R5-COVERAGE-F01. WU-0C-N3 carries ≥6 binary acceptance criteria covering each landed CLI surface

**Severity: NONE (positive)**

WU-0C-N3 acceptance criteria (`product-strategy/ai-roadmap-phase-0c.md` lines 1769–1778) total 9 binary checks. Each of the 5 named CLI surfaces from the agent-runner feature register has at least one binary criterion:

| CLI surface | Owning WU-0C-N3 criterion(s) | Binary form |
|---|---|---|
| `agents session locate <id> [--json]` | crit 1 ("Each trait method invokes the corresponding `agents session` subcommand and maps stdout/stderr JSON into the WU-0C-N2 DTOs and `SessionOverrideError` variants"); crit 2 ("`unsupported-storage` exit/error code from locate/export/import surfaces as `SessionOverrideError::UnsupportedStorage` and performs no harness-side transcript mutation") | Trait `locate_session` and `get_session_metadata` both invoke `agents session locate`; success returns a `SessionLocation`/`SessionMetadata` DTO; named error variants `UnsupportedStorage`, `SessionNotFound`, `AmbiguousSession` reachable via the WU-0C-N1 trait fixture set. |
| `agents session export <id> [--format canonical-jsonl]` | crit 1; crit 2 | `read_transcript` invokes `agents session export`; success returns ordered `TranscriptTurn` DTOs with source offsets and hashes; `unsupported-storage` exit maps to typed error before any harness-side mutation. |
| `agents session import-replace <id> --from-file <path> [--preimage-sha256 <hex>]` | crit 1; crit 2; crit 3 (`session-busy` exit code 13 surfaces as `SessionOverrideError::SessionBusy`); crit 4 (`--preimage-sha256` mismatch surfaces as `SessionOverrideError::PreimageMismatch`); crit 7 (truncate/append compose `read_transcript` + canonical JSONL edit + `replace_transcript`) | `replace_transcript` invokes `agents session import-replace`; `session-busy` exit 13 maps to `SessionBusy`; preimage mismatch maps to `PreimageMismatch`; truncate/append composition reuses this surface so all three write trait methods exercise the same CLI. |
| `agents session pause-handshake <id> [--ttl-ms <ms>]` / `resume-handshake <id> --token <token>` | crit 3 (`session-busy` from pause-handshake or import-replace); crit 5 (`pause-handshake` lease TTL is respected for atomic mid-session override, and `resume-handshake` is always called on adapter drop when a lease token is held) | Lease lifecycle bound at adapter level: TTL respected; resume-handshake always called on drop; busy returned as `SessionBusy` before any file write. |
| `agents session schema-probe` | crit 1; crit 6 (Adapter construction calls `agents session schema-probe` and refuses operation if the output is malformed, required feature flags are absent, or `safe_for_import_replace` is false) | Construction-time gate; refusal is binary on three named conditions (malformed output, missing required feature flags, `safe_for_import_replace = false`). |

Auxiliary criteria reinforce coverage without per-surface mapping: crit 8 ("fake `agents` binary fixture covers success, unsupported storage, busy lease, preimage mismatch, unsafe schema probe, malformed JSON, and missing subcommand cases"); crit 9 (records evidence/audit/recovery refs through WU-0B-09 / WU-0B-15 / WU-0B-31); crit 10 (no direct writes to `state.db`, provider transcript files, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, or cross-provider migration settings).

The required ≥6 binary criteria are met (9 ACs total, with each CLI surface bound by at least one named-call + named-error-variant criterion). No CLI surface is left to a generic "service works" line.

**Recommendation:** No action required.

---

### R5-COVERAGE-F02. All 7 SessionOverrideContract operations remain consumed via WU-0C-N3 only

**Severity: NONE (positive)**

The proposal-r6 contract surface still names exactly 7 operations (`product-strategy/proposal.md` lines 83–89, 1568–1591). Each operation is implemented by WU-0C-N3 against a landed `agents session …` surface, with binary criteria across WU-0C-N1 (trait) and WU-0C-N3 (adapter). WU-0C-N4 is no longer in the implementation chain; its r4 role (harness-owned schema probe) collapses to WU-0C-N3 crit 6.

| Proposal-r6 operation | Owner WU(s) | Binary criterion |
|---|---|---|
| `schema_version_probe` | WU-0C-N1 crit 4 + WU-0C-N3 crits 1, 6 | "`replace_transcript`, `truncate_after`, and `append_turns` test fixtures prove `schema_version_probe()` is invoked before mutation by failing the operation when the fake probe returns `UnsupportedSchema`" (trait); "Adapter construction calls `agents session schema-probe` and refuses operation if the output is malformed, required feature flags are absent, or `safe_for_import_replace` is false" (adapter). |
| `locate_session` | WU-0C-N1 crits 2/3 + WU-0C-N3 crits 1, 2 | trait fake-adapter success/error reachability + adapter `agents session locate` mapping with `unsupported-storage` typed error. |
| `read_transcript` | WU-0C-N1 crits 2/6 + WU-0C-N3 crits 1, 2 | trait returns `Vec<TranscriptTurn>` without raw mutable handles + adapter `agents session export` mapping with unsupported-storage refusal. |
| `replace_transcript` | WU-0C-N1 crits 4/5 + WU-0C-N3 crits 1, 2, 3, 4 | trait schema-probe-before-mutation + preimage/idle precondition refusal + adapter `agents session import-replace` mapping with `session-busy` exit 13 → `SessionBusy`, `--preimage-sha256` mismatch → `PreimageMismatch`. |
| `truncate_after` | WU-0C-N1 crits 4/5 + WU-0C-N3 crits 1, 7 | trait write-method preconditions + adapter composition (`read_transcript` + canonical JSONL boundary edit + `replace_transcript`); this composition path inherits the `replace_transcript` typed-error coverage from crits 2/3/4. |
| `append_turns` | WU-0C-N1 crits 4/5 + WU-0C-N3 crits 1, 7 | trait write-method preconditions + adapter composition; same inheritance as `truncate_after`. |
| `get_session_metadata` | WU-0C-N1 crit 2 + WU-0C-N3 crit 1 | trait fake-adapter reachability ("each write operation using WU-0C-N2 DTO fixtures" and read methods named) + adapter `agents session locate <id> --json` mapped to the metadata subset. |

Every operation has at least one binary criterion at the trait surface (WU-0C-N1) and at least one at the adapter surface (WU-0C-N3). The r4 INFO finding R4-COVERAGE-F08 (no v1-adapter-level criterion for `get_session_metadata`) is replaced in r5 by an explicit adapter-side mapping line in the WU-0C-N3 contract block ("`get_session_metadata -> agents session locate <id> --json, mapped to the metadata subset`") plus crit 1 ("Each trait method invokes the corresponding `agents session` subcommand …"), so the operation now has a named adapter-level call. The composed-write criterion (crit 7) similarly tightens R4-COVERAGE-F09 by giving `truncate_after` and `append_turns` a single, named composition path through `read_transcript` + canonical JSONL edit + `replace_transcript`.

**Recommendation:** No action required.

---

### R5-COVERAGE-F03. WU-0C-N4 drop introduces no orphan WU and no broken edge

**Severity: NONE (positive)**

The r4 incoming-edge set for WU-0C-N4 was `WU-0C-N2; WU-0A-03, WU-0A-15`, and its outgoing-edge set was `WU-0C-N3; WU-0C-N1 v1-refusal tests`. In r5:

- **WU-0C-N3** drops its incoming dependency on WU-0C-N4 (line 3277: `WU-0C-N3 <- WU-0C-N1, WU-0C-N2, WU-0A-15, WU-0B-09, WU-0B-15, WU-0B-31` — N4 is gone) and gains an explicit construction-time call to `agents session schema-probe` as its own crit 6 binary AC. The schema-probe-before-mutation invariant is preserved and is now bound at a single WU rather than split across two.
- **WU-0C-N1** trait crit 4 still requires schema-probe-before-mutation through the fake adapter ("`replace_transcript`, `truncate_after`, and `append_turns` test fixtures prove `schema_version_probe()` is invoked before mutation by failing the operation when the fake probe returns `UnsupportedSchema`"). The trait test does not name WU-0C-N4 specifically; the test exercises the trait surface, so removing WU-0C-N4 does not invalidate the test.
- **WU-0A-03** (`AgentRunnerStateDbProbe`) and **WU-0A-15** (`FakeAgentsFixture`): WU-0A-03's role as a Phase 0A read-only state.db probe remains unchanged; it had no other Phase 0C edge that depended on WU-0C-N4 specifically. WU-0A-15's outgoing edge to WU-0C-N3 (fake-binary integration tests) is preserved at line 3277. Neither Phase 0A WU becomes an orphan.
- **WU-0C-N1's "v1-refusal tests" outgoing edge** from WU-0C-N4 was a forward-looking edge into the trait fixtures; in r5 the trait fixtures still test `UnsupportedSchema` behavior through the fake-adapter probe (crit 4), with WU-0C-N3 crit 6 supplying the real-binary refusal. No trait-level test loses its upstream fixture.
- **No other Phase 0C WU** referenced WU-0C-N4 in r4: the r4 risk file's R4-COVERAGE-F06 owner-edge table named WU-0C-N4 only as incoming to WU-0C-N3 (now removed) and via Stitch Notes references to "WU-0C-N1..N5" (now rewritten to "WU-0C-N1/N2/N3/N5"). A grep of r5 for `WU-0C-N4` returns only the removed-in-r5 stub at line 1735, the WU-0C-N3 revision-rationale paragraph at line 1790, the Refactor Ledger update at line 3221, and family-watch / round-summary rows at 3531/3539/3552.

The 76-WU graph after r5 has every WU declaring both incoming and outgoing edges. Spot-check on the four remaining N-family WUs:

| WU | Incoming | Outgoing |
|---|---|---|
| WU-0C-N2 (DTOs) | WU-0B-09, WU-0B-15, WU-0B-21 | WU-0C-N1, WU-0C-N3, WU-0C-N5; VS-010, VS-012, VS-018, VS-020, VS-021 |
| WU-0C-N1 (trait) | WU-0C-N2; WU-0B-09, WU-0B-15 | WU-0C-N3, WU-0C-N5; VS-010, VS-012, VS-018, VS-020, VS-021 |
| WU-0C-N3 (adapter) | WU-0C-N1, WU-0C-N2; WU-0A-15, WU-0B-09, WU-0B-15, WU-0B-31 | VS-010, VS-012, VS-018, VS-020, VS-021 |
| WU-0C-N5 (registry) | WU-0C-N1, WU-0C-N2; WU-0B-09, WU-0B-15, WU-0B-31 | WU-0C-31 recovery metadata; VS-001, VS-003, VS-020, VS-021 |

WU-0C-N4 itself is no longer a WU in the D1 ownership table (the r5 table at lines 3453–3456 lists only `TranscriptTurn`/`SessionLocation`/`SessionMetadata` (WU-0C-N2), `SessionOverrideContract` trait (WU-0C-N1), `AgentRunnerCliAdapter` (WU-0C-N3), and `SessionOverrideStore` registry (WU-0C-N5) — no AgentRunnerSchemaProbe row). The "Removed in r5" heading at line 1735 is a diff marker, not a WU.

**Recommendation:** No action required.

---

### R5-COVERAGE-F04. Cross-phase incoming-from-Phase-0C edges to Phase 1/2/3 still resolve

**Severity: NONE (positive)**

The cross-phase outgoing block at lines 3623–3628 of the r5 artifact references only existing WUs:

- "SessionOverrideContract WU-0C-N1/N2/N3/N5 -> Phase 1 worker-launcher and worker-output-reintegration WUs …" (line 3625) — all four WU IDs exist in the r5 inventory.
- "SessionOverrideContract WU-0C-N1/N2/N3/N5 -> Phase 2 turn-decomposition and detail-injection-router WUs …" (line 3626) — all four WU IDs exist.
- "SessionOverrideContract WU-0C-N1/N2/N3/N5 -> Phase 3 repack-planner WUs …" (line 3627) — all four WU IDs exist.
- "Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1/N2/N3/N5 …" (line 3628) — same four WU IDs.

No reference to WU-0C-N4 appears in the cross-phase block. Phase 1/2/3 ai-roadmaps are not yet authored, so there are no live incoming Phase-1+ → Phase-0C edges to invalidate; the prospective edges that the round-4 cross-phase block created point only to the four WUs that remain. The r5 commit message and the r5 audit-history "watch signals" row both record the simplification-drift watch on this surface ("do not recreate a harness-side DB adapter, harness-side schema wrapper WU, or per-CLI storage parser unless upstream removes the `agents session` contract").

The Stitch Notes per-VS blocks for VS-010/012/018/020/021 have been rewritten from "WU-0C-N1..N5" to "WU-0C-N1/N2/N3/N5" (lines 3586, 3610, 3612, 3618, 3620, 3621), confirming WU-0C-N4 is excluded everywhere a slice consumer is enumerated.

**Recommendation:** No action required.

---

### R5-COVERAGE-F05. Block-on annotations fully removed; no upstream-feature gating remains

**Severity: NONE (positive)**

`grep -in "blocked-on\|Blocked on\|blocked on" product-strategy/ai-roadmap-phase-0c.md` returns 0 hits. The remaining `blocked` strings in the artifact are domain-level values inside DTO contracts:

- `budget_state: "within" | "near_limit" | "exceeded" | "blocked"` (line 454) and downstream criteria at lines 473, 625.
- `blocked_refs: { ref_id: string, reason_code: string }[]` (line 2129) and downstream criteria at lines 2142–2178.
- Documentary "poison-quarantined, deleted, and unresolved-conflict nodes are blocked unless policy explicitly permits a labeled evidence-only render" (line 2166).

None of these is a `Blocked-on:` annotation in the r4 sense (where each of WU-0C-N1, WU-0C-N3, WU-0C-N4, WU-0C-N5 carried a "Blocked-on: …" line naming `agents session locate / export / import-replace / pause-handshake / schema-probe`, and per-VS Stitch-Notes blocks repeated the constraints). The r5 commit message records the rationale: "All Blocked-on annotations removed across Phase 0C" because upstream agent-runner features have landed.

The r4 R4-COVERAGE-F05 finding ("Block-on annotations match the agent-runner feature-request register") is therefore preserved by removal: the register collapses to "all features landed", and the matching constraint becomes a binary refusal at adapter construction (WU-0C-N3 crit 6: refuse if `safe_for_import_replace` is false). Schema/feature-flag fail-closed behavior is now a runtime check at WU-0C-N3 rather than a roadmap-level annotation, which is the correct shape for a converged contract.

**Recommendation:** No action required.

---

### R5-COVERAGE-F06. r4 closures preserved for the WUs not touched in r5

**Severity: NONE (positive)**

The r5 diff is localized to the WU-0C-N3/N4 contract blocks and the dependency / parallelization / stitch references that mention WU-0C-N4 or wave membership. The 11 WUs in the Round 4 Refactor Ledger (WU-0C-11a, WU-0C-13, WU-0C-13a, WU-0C-13b, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0C-15d, WU-0C-18, WU-0C-31, WU-0C-34) and the remaining ~60 unaffected WUs (everything outside the WU-0C-N1..N5 family) carry their r4 binary acceptance criteria, code/test boundaries, and contracts byte-for-byte. Spot-check across the WUs flagged as method-bearing in the r4 risk file:

- **WU-0C-13** `AgentSessionCapture` (line 1025–1058) keeps its 6 r4 ACs (success path for `agents`-reported-session, success path for trace-session, `substrate_gap` path, `MissingCaptureSource` error path, no-mutation invariant, no-WorkerRun/OrchestratorTurn/QuestionArtifact/RecoveryAction-row invariant). The r5 diff does not touch this block.
- **WU-0C-15d** `SessionTurnsReader` (line 1290–1323) keeps its 6 r4 ACs including the SessionOverrideContract no-mutation invariant ("never calls `replace_transcript`, `truncate_after`, or `append_turns`"). Untouched.
- **WU-0C-18** `AgentRunnerClient` Facade (line 1535–1580) keeps its 11 r4 ACs (per-method delegation criterion for each of the 8 facade methods + no-transcript-write invariant + no-provider-routing invariant + per-method fixture set). Untouched.
- **WU-0C-31** `RecoveryActionProcessorSkeleton` (line 2795–2827) keeps its r4 binary criteria; the WU-0C-N5 metadata input edge is preserved at line 3308.
- **WU-0C-34** `TauriIpcCommandRouter` (line 2941–2991) keeps its r4 binary criteria including the "no UI command for replace/truncate/append in Phase 0C" scope-deny check; line 3314 preserves its incoming WU-0C-N1/WU-0C-N5 edges.
- **WU-0C-N1**, **WU-0C-N2**, **WU-0C-N5** are unchanged from r4 (verified by absence in r5 diff scope items 1–11 above). Their 8 / 7 / 8 binary acceptance criteria respectively remain in place; no criterion is weakened or removed.

The r4 findings R4-COVERAGE-F01 (proposal contract operations covered), F03 (every `SessionOverrideError` variant reachable), F04 (v1 adapter invariants — now reinterpreted as v2 adapter invariants under the same coverage rule, with schema-pinning collapsed to construction-time refusal at WU-0C-N3 crit 6, refuse-rather-than-corrupt preserved at WU-0C-N3 crits 2/3/4 and WU-0C-N1 crit 5, and flock-locking superseded by the upstream `agents session pause-handshake` lease at WU-0C-N3 crits 3/5), F06 (no orphan WU), F07 (refactored WUs retain functional ACs), and F10 (round-2 INFOs unchanged) all carry forward. R4-COVERAGE-F05 is replaced by R5-COVERAGE-F05 (block-on removal). R4-COVERAGE-F08 and F09 (the two r4 INFO findings) are addressed in r5 by the explicit adapter-level mapping for `get_session_metadata` and the explicit composition path for `truncate_after` / `append_turns`, so they downgrade from INFO to closed.

**Recommendation:** No action required.

---

### R5-COVERAGE-F07. v2-only adapter invariants each retain a binary criterion

**Severity: NONE (positive)**

The proposal-r6 axiom for the v2 CLI adapter requires four invariants (proposal-r6 lines 1693–1738, agent-runner feature-request register at lines 1717–1729). Each maps to a WU-0C-N3 or WU-0C-N1 binary criterion:

| v2 invariant | Owner criterion | Binary form |
|---|---|---|
| Schema-version pinning at construction | WU-0C-N3 crit 6 | "Adapter construction calls `agents session schema-probe` and refuses operation if the output is malformed, required feature flags are absent, or `safe_for_import_replace` is false." |
| Refuse-rather-than-corrupt on storage / busy / preimage / render | WU-0C-N3 crits 2, 3, 4; WU-0C-N1 crit 5 | All three write-trait methods return typed `SessionOverrideError` variants (`UnsupportedStorage`, `SessionBusy`, `PreimageMismatch`, plus the upstream `unsupported_append` / `invalid_turn_boundary` returned through the composition path) before any harness-side mutation; trait write methods reject missing `OverridePreconditions`. |
| Atomic mid-session override via `pause-handshake` → `import-replace` → `resume-handshake` | WU-0C-N3 crit 5 | "`pause-handshake` lease TTL is respected for atomic mid-session override, and `resume-handshake` is always called on adapter drop when a lease token is held." |
| No direct harness writes outside the documented `agents session …` surface | WU-0C-N3 crit 10 | "The adapter never opens or writes `agent-runner` `state.db`, provider transcript files, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, or cross-provider migration settings directly." |

The r4 invariant "two-phase-write recovery" (file temp → atomic rename → SQLite transaction → receipt + crash-injection deterministic recovery) is now an upstream-owned invariant of `agents session import-replace`. WU-0C-N3 crit 1 binds the adapter to the upstream call's stdout/stderr JSON contract; crit 8 (fake `agents` binary fixture) covers success / unsupported storage / busy lease / preimage mismatch / unsafe schema probe / malformed JSON / missing subcommand; crit 9 (evidence/audit/recovery refs through WU-0B-09 / WU-0B-15 / WU-0B-31) covers the harness-side audit trail. WU-0C-N5's `SessionOverrideStore` registry retains its 5 lifecycle transitions (`begin_pending`, `commit pending->committed`, `rollback pending->rolled_back`, `quarantine pending|crash-recovery->quarantined_storage_conflict`, `list_by_session ordered`) at lines 1827–1836 unchanged, so the harness-visible recovery state machine is still binary at the per-WU level.

The r4 watch family `session-override-boundary-family` (registered in `plans/audit/ai-roadmap-phase-0c.md` r4 entry) is reinforced rather than weakened: with the v1 DB adapter dropped, the boundary collapses to "Phase 0C never writes provider storage directly" and is enforced by WU-0C-N3 crit 10 + the explicit non-ownership note at line 3637.

**Recommendation:** No action required.

---

## Round 5 oscillation classification

Per `~/ai/conventions/audit-history.md`:

- **Same-label**: 0. No prior-round Coverage finding recurs at MEDIUM or higher in r5. r4 was LOW; r3/r2 LOW; r1's only MEDIUM was closed in r2.
- **Same-family** (`state-machine-criteria-family`, `bundling-family`, `session-override-boundary-family`): 0. No method, enum, or state-machine binding declared in any prior round has been weakened in r5. Specifically, the `OptimizerCycleStateMachine` (WU-0C-28), `ConflictRecord` workflow (WU-0C-29d), `RecoveryAction` transitions (WU-0C-30), `SessionOverrideRecord` lifecycle (WU-0C-N5 crits 3–5), and `SessionOverrideError` variant set (WU-0C-N2 lines 1645–1659; 13 variants) are byte-untouched. The two r4 INFO findings (R4-COVERAGE-F08, F09) are addressed and closed; they did not become a family.
- **Fix-created**: gen 0. The cascade is externally driven (proposal-r6 + engineering-roadmap-r5 + landed agent-runner CLI features). No Phase 0C-internal fix in r5 created a coverage gap. WU-0C-N4 was removed because its sole responsibility (harness-side schema probe) collapsed to a one-line CLI call inside WU-0C-N3 — a structural simplification, not a fix-created orphan. The Phase 0C audit history records this as `fix-created-family` gen 0 explicitly (commit `1548cf4` audit-history line 74; risk-file Run Report line 3539 of the artifact).
- **Two-generation / named three-generation / named four-generation**: not fired.
- **Simplification-drift watch** (newly registered in the r5 audit history): forward-looking watch on whether future revisions recreate a harness-side DB adapter, harness-side schema wrapper WU, or per-CLI storage parser. Not a Coverage-gate firing.

Coverage emits no MEDIUM or higher in r5. The two r4 INFO findings are closed. No new INFO findings are introduced.

## Round 5 summary table

| ID | Finding | Severity |
|----|---------|----------|
| R5-COVERAGE-F01 | WU-0C-N3 carries 9 binary acceptance criteria, ≥1 per landed CLI surface (locate / export / import-replace / pause-handshake / schema-probe) — exceeds the ≥6 bar | NONE (positive) |
| R5-COVERAGE-F02 | All 7 SessionOverrideContract operations remain consumed; each has trait-level + adapter-level binary criteria via WU-0C-N1 + WU-0C-N3 only (no remaining WU-0C-N4 dependency) | NONE (positive) |
| R5-COVERAGE-F03 | WU-0C-N4 drop introduces no orphan WU and no broken edge; the four remaining N-family WUs and the 11 r4 refactored WUs all retain incoming + outgoing edges | NONE (positive) |
| R5-COVERAGE-F04 | Cross-phase outgoing-to-Phase-1/2/3 block reference only WU-0C-N1/N2/N3/N5; no edge to dropped WU-0C-N4; per-VS blocks for VS-010/012/018/020/021 rewritten consistently | NONE (positive) |
| R5-COVERAGE-F05 | All `Blocked-on:` annotations removed; remaining `blocked` strings are domain values inside DTO criteria; fail-closed behavior collapses to WU-0C-N3 crit 6 construction-time refusal | NONE (positive) |
| R5-COVERAGE-F06 | r4 closures preserved for the ~70 WUs outside the WU-0C-N1..N5 family; r4 INFO findings F08 and F09 closed by r5 explicit adapter mappings | NONE (positive) |
| R5-COVERAGE-F07 | v2-only adapter invariants (schema pinning at construction, refuse-rather-than-corrupt, atomic pause/import-replace/resume, no direct harness writes) each have a binary AC across WU-0C-N3 / WU-0C-N1 / WU-0C-N5 | NONE (positive) |

## What LOW requires (round 5 update)

The round-4 conditions 1–9 carry forward unchanged, with the following round-5 amendments:

10. **r5 amendment to condition 6.** The proposal-r6 SessionOverrideContract operation set is unchanged from proposal-r5 (still `schema_version_probe`, `locate_session`, `read_transcript`, `replace_transcript`, `truncate_after`, `append_turns`, `get_session_metadata` — 7 operations). If proposal-r7+ adds a new operation or a new error variant, the AI roadmap must add a corresponding criterion at both the trait (WU-0C-N1) and the adapter (WU-0C-N3) before the next coverage gate.
11. **r5 amendment to condition 7.** The agent-runner feature-request register has collapsed from "5 named features blocked-on" to "all 5 features landed; runtime refusal at construction". If the `agents session …` CLI shape changes (renamed subcommand, new exit code, new `--flag` argument), WU-0C-N3 acceptance criteria 1–7 must be updated in lockstep, and the simplification-drift watch must explicitly classify whether a harness-side wrapper is still excluded.
12. **r5 amendment to condition 8.** The Round 4 Refactor Ledger remains the audit trail for the 11 narrowed WUs. The round-5 update footnote at line 3221 is the audit trail for the WU-0C-N3 rescope and the WU-0C-N4 removal. Any future r6+ narrowing must extend the ledger with (1) the violation under repair, (2) the fix, (3) the SessionOverrideContract dependency, and (4) the kept/merged/removed disposition. WU-0C-N4 may not be silently restored without re-justifying a harness-owned schema-probe responsibility.
13. **r5 amendment to condition 9.** Cross-phase outgoing edges now reference WU-0C-N1/N2/N3/N5 only. Phase 1+ ai-roadmaps must declare incoming edges from this exact set (no WU-0C-N4 reference); a Phase 1/2/3 revision that re-introduces a harness-side schema-probe WU must justify it under the simplification-drift watch.
14. **r5 simplification-drift watch.** Coverage at LOW depends on Phase 0C not recreating, in any subsequent round, a harness-side DB adapter, harness-side schema wrapper WU, or per-CLI storage parser unless upstream `agent-runner` removes the `agents session` contract. If a future revision does so, the new WU(s) must satisfy method-level binary criteria at the trait test surface (WU-0C-N1) and at the new adapter, not implicitly through aggregation.
