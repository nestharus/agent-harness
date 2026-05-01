# WU-0C-N2: TranscriptTurn, SessionLocation, and SessionMetadata DTOs

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-N2  
**Parent initiative:** SessionOverrideContract foundation  
**Implementation wave:** Wave 1 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-N2: TranscriptTurn, SessionLocation, and SessionMetadata DTOs`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust structs + TypeScript DTOs

TranscriptTurn {
  turn_id: string,
  turn_index: u32,
  role: "system" | "user" | "assistant" | "tool" | "synthetic",
  content_ref: string,
  tool_call_refs: string[],
  source_offsets: SourceOffset[],
  source_hash: string,
  provider_native_kind?: string,
  renderability: "renderable" | "unsupported_record" | "requires_replace"
}

SessionLocation {
  session_id: string,
  provider_name: string,
  storage_kind: "claude_code_jsonl" | "codex_jsonl" | "agent_runner_cli" | "unsupported",
  transcript_path?: string,
  active_chain_id?: string,
  active_segment_id?: string,
  locator_evidence_ref?: string,
  source_hash: string,
  mutability: "idle_writable" | "busy" | "read_only" | "unsupported",
  idle_evidence_ref?: string
}

SessionMetadata {
  session_id: string,
  provider_name: string,
  active_chain_id?: string,
  active_segment_id?: string,
  capture_method?: string,
  resume_acceptance_status?: string,
  transcript_state: "available" | "missing" | "no_locator" | "unsupported" | "quarantined",
  compaction_state: "none" | "has_boundary" | "unknown",
  last_observed_turn_id?: string,
  schema_probe_ref: string
}

OverridePreconditions {
  expected_preimage_hash: string,
  expected_graph_revision_id?: string,
  expected_active_segment_id?: string,
  require_idle: boolean
}

OverrideReceipt {
  override_id: string,
  session_id: string,
  operation: "replace_transcript" | "truncate_after" | "append_turns",
  preimage_hash: string,
  postimage_hash: string,
  schema_probe_ref: string,
  audit_event_draft_ref: string,
  evidence_ref: string
}

SessionOverrideError:
- UnsupportedSchema
- SessionNotFound
- AmbiguousSession
- UnsupportedStorage
- SessionBusy
- PreimageMismatch
- InvalidTurnBoundary
- UnsupportedAppend
- AdapterRenderFailure
- IoFailure
- DbFailure
- PostRenameDbFailure
- QuarantinedStorageConflict
```

## Acceptance Criteria

- [ ] `TranscriptTurn`, `SessionLocation`, `SessionMetadata`, `OverridePreconditions`, `OverrideReceipt`, and `SessionOverrideError` round-trip between Rust JSON fixtures and TypeScript DTO fixtures without field loss.
- [ ] Every role, storage-kind, mutability, transcript-state, compaction-state, operation, and error variant is reachable through a named fixture.
- [ ] `TranscriptTurn` validation rejects empty `turn_id`, missing `content_ref`, missing `source_hash`, negative/duplicate `turn_index`, and unsupported role strings with documented errors.
- [ ] `SessionLocation` validation rejects `mutability = idle_writable` without a storage kind that the active adapter declares writable.
- [ ] `OverridePreconditions` validation rejects empty `expected_preimage_hash` and permits missing graph/segment refs only for documented read-only operations.
- [ ] DTO validation treats `provider_native_kind`, `source_offsets`, and `transcript_path` as adapter-owned evidence and never parses provider JSONL record bodies.
- [ ] DTO validation writes no transcript files, state.db rows, EvidenceArtifact rows, AuditEvent rows, GraphNode rows, or SessionOverrideStore registry rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** Incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-21 `WorkingSetSnapshot`.

**Detailed dependency graph line, verbatim:** WU-0C-N2 <- WU-0B-09, WU-0B-15, WU-0B-21

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-09
- WU-0B-15
- WU-0B-21

**Intra-Phase 0C predecessors:**
- none

## Test Boundary

`product-strategy/contracts/wu-0c-n2-session-override-dtos.md`, `src-tauri/src/contracts/session_override_dtos.rs`, `src/contracts/session-override-dtos.ts`

## Code Boundary

`src-tauri/src/session_override/dtos.rs`, `src-tauri/src/contracts/session_override_dtos.rs`, `src/contracts/session-override-dtos.ts`, `src-tauri/tests/wu_0c_n2_session_override_dtos_contract.rs`, `src/test/session-override-dtos.test.ts`

## Blocked-on

**Blocked-on:** None for the DTO contract. v2 population fields are placeholders until `agents session locate/export/import-replace`, pause-handshake, and schema-version probe land.

## Revision Rationale

Phase 0C r4 brownfield-cascade addition: introduced by SessionOverrideContract axiom from proposal-r5 + engineering-roadmap-r4. Owns one of the 5 new contract concerns (trait / DTOs / v1 adapter / schema probe / override registry). r4 is fix-created-family at gen 0 in Phase 0C-local loop, externally-driven from upstream cascade. Risk gates LOW/LOW/LOW (decomp / coverage / dep). Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Canonical session-override DTOs consumed by WU-0C-N1, WU-0C-N3, WU-0C-N4, WU-0C-N5, VS-010, VS-012, VS-018, VS-020, and VS-021.
- Parallelizable with: WU-0C-11a, WU-0C-15a, WU-0C-16a, WU-0C-19, WU-0C-21 after shared 0A/0B dependencies exist.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
- Preserve the `**Blocked-on:**` annotation above in implementation handoffs and PR descriptions until the named `agents` features land.
