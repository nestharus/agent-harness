# WU-0C-N5: SessionOverrideStore Registry

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N5  
**Parent initiative:** SessionOverrideContract audit/override registry  
**Implementation wave:** Wave 3 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N5: SessionOverrideStore Registry`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

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

## Blocked-on

**Blocked-on:** None for the v1 registry. v2 adapter-kind activation is blocked on `agents session locate/export/import-replace` and schema-version probe.

## Revision Rationale

Phase 0C r4 brownfield-cascade addition: introduced by SessionOverrideContract axiom from proposal-r5 + engineering-roadmap-r4. Owns one of the 5 new contract concerns (trait / DTOs / v1 adapter / schema probe / override registry). r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Risk gates LOW/LOW/LOW (decomp / coverage / dep). Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Workspace-level override ledger consumed by WU-0C-N3 crash recovery, VS-001 evidence inspectors, VS-003 audit surfaces, VS-020 recovery, and VS-021 reroute governance.
- Parallelizable with: WU-0C-25, WU-0C-27, WU-0C-29d after WU-0C-N1/N2 land.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
- Preserve the `**Blocked-on:**` annotation above in implementation handoffs and PR descriptions until the named `agents` features land.
