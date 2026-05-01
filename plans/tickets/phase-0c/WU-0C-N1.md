# WU-0C-N1: SessionOverrideContract Trait

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N1  
**Parent initiative:** SessionOverrideContract foundation  
**Implementation wave:** Wave 2 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N1: SessionOverrideContract Trait`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust trait + TypeScript interface

SessionOverrideContract {
  schema_version_probe() -> Result<SchemaProbe, SessionOverrideError>
  locate_session(session_id: string) -> Result<SessionLocation, SessionOverrideError>
  read_transcript(session_id: string) -> Result<Vec<TranscriptTurn>, SessionOverrideError>
  replace_transcript(session_id: string, new_jsonl: string, preconditions: OverridePreconditions) -> Result<OverrideReceipt, SessionOverrideError>
  truncate_after(session_id: string, turn_index: u32, preconditions: OverridePreconditions) -> Result<OverrideReceipt, SessionOverrideError>
  append_turns(session_id: string, turns: Vec<TranscriptTurn>, preconditions: OverridePreconditions) -> Result<OverrideReceipt, SessionOverrideError>
  get_session_metadata(session_id: string) -> Result<SessionMetadata, SessionOverrideError>
}

SessionOverrideContract rules:
- all writes call schema_version_probe before file or DB mutation
- all writes require preimage and idle/lease preconditions
- all operations return typed SessionOverrideError variants only
- trait callers never receive provider-native mutable file handles
```

## Acceptance Criteria

- [ ] The Rust trait and TypeScript interface expose exactly the seven operations listed in the contract with the documented argument and result DTOs.
- [ ] A fake in-memory adapter can return success for `locate_session`, `read_transcript`, `get_session_metadata`, and each write operation using WU-0C-N2 DTO fixtures.
- [ ] A fake adapter error fixture maps each `SessionOverrideError` variant to a stable Rust enum discriminant and TypeScript string without lossy catch-all mapping.
- [ ] `replace_transcript`, `truncate_after`, and `append_turns` test fixtures prove `schema_version_probe()` is invoked before mutation by failing the operation when the fake probe returns `UnsupportedSchema`.
- [ ] Write methods reject missing `OverridePreconditions` and return `PreimageMismatch` or `SessionBusy` before invoking the fake mutation hook when the preconditions fail.
- [ ] `read_transcript` returns `Vec<TranscriptTurn>` and does not expose raw mutable file descriptors, SQLite connections, or provider-specific parser objects.
- [ ] The trait module contains no provider routing, quota/account selection, auth refresh, resume composition, cross-provider porting, or session-id generation logic.
- [ ] The TypeScript interface is usable by tests without importing Tauri command-router modules.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-N2; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`.

**Detailed dependency graph line, verbatim:** WU-0C-N1 <- WU-0C-N2, WU-0B-09, WU-0B-15

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-15

**Intra-Phase 0C predecessors:**
- WU-0C-N2

## Test Boundary

`product-strategy/contracts/wu-0c-n1-session-override-contract.md`, `src-tauri/src/contracts/session_override_contract.rs`, `src/contracts/session-override-contract.ts`

## Code Boundary

`src-tauri/src/session_override/contract.rs`, `src-tauri/src/contracts/session_override_contract.rs`, `src/contracts/session-override-contract.ts`, `src-tauri/tests/wu_0c_n1_session_override_contract.rs`, `src/test/session-override-contract.test.ts`

## Blocked-on

**Blocked-on:** None for v1 trait acceptance. v2 adapter migration is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe.

## Revision Rationale

Phase 0C r4 brownfield-cascade addition: introduced by SessionOverrideContract axiom from proposal-r5 + engineering-roadmap-r4. Owns one of the 5 new contract concerns (trait / DTOs / v1 adapter / schema probe / override registry). r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Risk gates LOW/LOW/LOW (decomp / coverage / dep). Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Versioned session write-back trait consumed by WU-0C-N3, WU-0C-N5, VS-010 turn-decomposition/detail-injection, VS-012 repack planner, VS-018 worker-output reintegration, VS-020 recovery, and VS-021 reroute governance.
- Parallelizable with: WU-0C-12, WU-0C-13, WU-0C-14, WU-0C-16 after WU-0C-N2 lands.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
- Preserve the `**Blocked-on:**` annotation above in implementation handoffs and PR descriptions until the named `agents` features land.
