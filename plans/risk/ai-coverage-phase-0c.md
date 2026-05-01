# AI — Coverage Risk Assessment (Phase 0C, round 4)

**Rating: LOW**

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
