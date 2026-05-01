# WU-0C-N3: AgentRunnerDbAdapter v1

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N3  
**Parent initiative:** SessionOverrideContract v1 adapter  
**Implementation wave:** Wave 4 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N3: AgentRunnerDbAdapter v1`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust service object implementing SessionOverrideContract

AgentRunnerDbAdapter::new(state_db_path, sessions_config_ref, agents_binary, lock_root) -> AgentRunnerDbAdapter

Implements:
- schema_version_probe
- locate_session
- read_transcript
- replace_transcript
- truncate_after
- append_turns
- get_session_metadata

v1 write protocol:
- refuse outside pinned SchemaProbe supported range
- locate exactly one active session/provider/segment
- prove session idle by lock + stable mtime + SQLite non-busy observations
- write same-directory temp JSONL and fsync where available
- record pending override before rename
- atomic rename final transcript
- update minimum state rows in one SQLite transaction
- commit or quarantine pending override after crash recovery
```

## Acceptance Criteria

- [ ] `locate_session(session_id)` reads fixture `state.db` rows plus configured transcript locator output and returns exactly one `SessionLocation` for a supported session.
- [ ] `locate_session(session_id)` returns `SessionNotFound`, `AmbiguousSession`, or `UnsupportedStorage` for missing, duplicate, or unsupported fixtures and performs no file mutation.
- [ ] `read_transcript(session_id)` returns ordered `TranscriptTurn` fixtures with source offsets and hashes preserved for supported plaintext JSONL sessions; malformed records become `unsupported_record` turns rather than being dropped.
- [ ] `replace_transcript(session_id, new_jsonl, preconditions)` refuses on unsupported schema, busy session, preimage mismatch, unsupported storage, or adapter render failure before renaming any file.
- [ ] Successful `replace_transcript` writes a same-directory temp file, records a pending override, atomically renames it, updates only required session-turn/chain consistency rows, and returns an `OverrideReceipt` with preimage/postimage hashes.
- [ ] `truncate_after(session_id, turn_index, preconditions)` truncates only at a valid turn boundary and rejects boundaries that split a tool-call/result dependency or compaction-boundary invariant.
- [ ] `append_turns(session_id, turns, preconditions)` appends only adapter-renderable `TranscriptTurn` records, rejects duplicate turn IDs, and refuses unsupported append cases before file mutation.
- [ ] Crash-injection fixtures after temp write, after rename, and after DB transaction recover deterministically to committed, rolled-back, or `QuarantinedStorageConflict` states.
- [ ] The adapter uses flock-style/session-idle locking and returns `SessionBusy` when it cannot prove no in-flight `agents` write owns the same session.
- [ ] The adapter never edits `agents`, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, or cross-provider migration settings.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16; incoming WU-0A-03 `LocalStorageLayout`, WU-0A-15 `FakeAgentsFixture`, WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`.

**Detailed dependency graph line, verbatim:** WU-0C-N3 <- WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16, WU-0A-03, WU-0A-15, WU-0B-09, WU-0B-15

**Phase 0A upstream WUs:**
- WU-0A-03
- WU-0A-15

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-15

**Intra-Phase 0C predecessors:**
- WU-0C-16
- WU-0C-N1
- WU-0C-N2
- WU-0C-N4
- WU-0C-N5

## Test Boundary

`product-strategy/contracts/wu-0c-n3-agent-runner-db-adapter.md`, `src-tauri/src/contracts/agent_runner_db_adapter.rs`

## Code Boundary

`src-tauri/src/session_override/agent_runner_db_adapter.rs`, `src-tauri/src/session_override/jsonl_render.rs`, `src-tauri/src/contracts/agent_runner_db_adapter.rs`, `src-tauri/tests/wu_0c_n3_agent_runner_db_adapter_contract.rs`

## Blocked-on

**Blocked-on:** v1 ships now under schema-version pinning and idle-only writes. v2 swap-later is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe. Atomic mid-session override remains blocked on `agents pause-handshake`.

## Revision Rationale

Phase 0C r4 brownfield-cascade addition: introduced by SessionOverrideContract axiom from proposal-r5 + engineering-roadmap-r4. Owns one of the 5 new contract concerns (trait / DTOs / v1 adapter / schema probe / override registry). r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Risk gates LOW/LOW/LOW (decomp / coverage / dep). Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Shipping v1 `SessionOverrideContract` implementation consumed by VS-010, VS-012, VS-018, VS-020, and VS-021 until the v2 CLI adapter lands.
- Parallelizable with: WU-0C-17, WU-0C-20a, WU-0C-23, WU-0C-24 after WU-0C-N1/N2/N4 and WU-0C-16 land.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
- Preserve the `**Blocked-on:**` annotation above in implementation handoffs and PR descriptions until the named `agents` features land.
