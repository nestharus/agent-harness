# WU-0C-N5: SessionOverrideStore Registry

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N5  
**Parent initiative:** SessionOverrideContract audit/override registry  
**Implementation wave:** Wave 3 from the Phase 0C r5 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r5/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N5: SessionOverrideStore Registry`
- Audit: `worktrees/phase-0c-ai-roadmap-r5/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r5 cascade history
- Proposal cascade: `worktrees/proposal-r6/product-strategy/proposal.md` -> Option A SessionOverrideContract/agent-runner feature landing cascade where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r5/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-5 agent-runner feature cascade
- Shape reference: `worktrees/tickets-phase-0c-r2/plans/tickets/phase-0c/` -> superseded r4-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r5 artifact:

```text
schema_object: Rust repository/service object

SessionOverrideRecord {
  override_id: string,
  workspace_id: string,
  session_id: string,
  adapter_kind: "agent_runner_db_v1" | "agent_runner_cli_v2" | "fake",
  operation: "replace_transcript" | "truncate_after" | "append_turns",
  state: "pending" | "committed" | "rolled_back" | "quarantined_storage_conflict",
  preimage_hash: string,
  postimage_hash?: string,
  schema_probe_ref: string,
  audit_event_id?: string,
  evidence_ref?: string,
  created_at: string,
  committed_at?: string
}

SessionOverrideStore::begin_pending(record) -> Result<SessionOverrideRecord, SessionOverrideError>
SessionOverrideStore::commit(override_id, receipt) -> Result<SessionOverrideRecord, SessionOverrideError>
SessionOverrideStore::rollback(override_id, reason) -> Result<SessionOverrideRecord, SessionOverrideError>
SessionOverrideStore::quarantine(override_id, reason) -> Result<SessionOverrideRecord, SessionOverrideError>
SessionOverrideStore::list_by_session(session_id) -> Result<Vec<SessionOverrideRecord>, SessionOverrideError>
```

## Acceptance Criteria

- [ ] `SessionOverrideRecord` round-trips through Rust serde and TypeScript fixture JSON while preserving adapter kind, operation, state, hashes, schema probe, audit, evidence, and timestamps.
- [ ] `begin_pending(record)` creates exactly one pending record for a unique `override_id` and rejects duplicate IDs.
- [ ] `commit(override_id, receipt)` transitions only `pending -> committed`, records postimage/audit/evidence refs, and rejects commit attempts for rolled-back or quarantined records.
- [ ] `rollback(override_id, reason)` transitions only `pending -> rolled_back` and preserves preimage hash plus reason evidence.
- [ ] `quarantine(override_id, reason)` transitions pending or crash-recovery records to `quarantined_storage_conflict` and requires preserved/replayed/discarded placeholder refs.
- [ ] `list_by_session(session_id)` returns records in created-at order and never exposes raw transcript contents.
- [ ] Registry writes append AuditEvent drafts through WU-0C-37-compatible refs but do not require the durable audit pipeline to call back into adapter write methods.
- [ ] The registry does not mutate transcript files, run schema probes, locate sessions, or implement adapter crash recovery by itself.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-N1, WU-0C-N2; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-31 `RecoveryAction`.

**Detailed dependency graph line, verbatim:** WU-0C-N5 <- WU-0C-N1, WU-0C-N2, WU-0B-09, WU-0B-15, WU-0B-31

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-15
- WU-0B-31

**Intra-Phase 0C predecessors:**
- WU-0C-N1
- WU-0C-N2

## Test Boundary

`product-strategy/contracts/wu-0c-n5-session-override-store.md`, `src-tauri/src/contracts/session_override_store.rs`

## Code Boundary

`src-tauri/src/session_override/store.rs`, `src-tauri/src/contracts/session_override_store.rs`, `src-tauri/tests/wu_0c_n5_session_override_store_contract.rs`


## Revision Rationale

Phase 0C r4 brownfield-cascade addition introduced this SessionOverrideContract-family boundary from proposal-r5 + engineering-roadmap-r4. r5 Option A keeps the contract and acceptance criteria intact after the agent-runner locate/export/import-replace/pause-handshake/schema-probe features landed in the proposal-r6 / engineering-roadmap-r5 cascade, removes WU-0C-N4 from the active WU set, and clears the former blocked-on annotations. Classification remains externally driven `fix-created-family` gen 0.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Workspace-level override ledger consumed by VS-001 evidence inspectors, VS-003 audit surfaces, VS-020 recovery, VS-021 reroute governance, and WU-0C-31 recovery metadata reads.
- Parallelizable with: WU-0C-25, WU-0C-27, WU-0C-29d after WU-0C-N1/N2 land.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
