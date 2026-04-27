# Agent Harness Proposal

## Scope

This proposal defines the `agent-harness` system: a single-user Tauri v2 desktop application that keeps one long-lived orchestrator productive across many concurrent initiatives by imposing a bounded graph-derived working set on each orchestrator turn.

Reference notation:

- `D1` through `D16` refer to the numbered difficulties in [problem.md](problem.md).
- `P1` through `P16` refer to the numbered principles in [philosophy.md](philosophy.md).

This document is not a roadmap. It does not define build order, MVP scope, deployment timing, or measurement targets. It describes the system as designed.

## Fixed Substrate

The following constraints are givens, not design choices:

- `agent-runner` is already installed as `/home/nes/.local/bin/agents`. The harness uses it for provider load balancing, invocation, resume, session ingestion, and quota state. The harness does not replace, fork, or duplicate it. This follows `P15` and the anti-goal against replacing `agent-runner`, while addressing cross-session trace and resume boundaries in `D7`, `D8`, `D10`, and `D16`.
- The user-facing application is a Tauri v2 desktop app. The frontend inherits the server-manager reference stack: Bun, Turbo, React 19, TanStack Router SPA, TanStack Query 5, Tailwind v4, xo-typescript ESLint, Prettier, Vitest, Playwright, Lefthook, Changesets, and Commitlint. The backend uses Rust, Tokio, and SQLite. This supports local control in `P15` and single-user desktop scope in `P16`.
- The orchestrator runs on `claude-opus`, or `claude-sonnet` when long-context behavior is more important than reasoning depth. The continuous context optimizer runs on `glm`. Sub-agents are dispatched by configured model name, including `gpt-high`, `claude-opus`, `claude-sonnet`, `gemini-high`, and other names available to `agent-runner`. This grounds model use without inventing a separate agent platform, matching `P8`, `P10`, `P12`, and `P16`.

## Design Commitments

### Imposed Working Set

The harness renders every orchestrator turn from the context graph. The orchestrator can request focus changes through pack, unpack, and focus tools, but it does not own the graph topology or decide which optimizer edits exist. When it sees a curation need, it emits an advisory `OptimizerRequest`; only the optimizer can convert that request into an `OptimizerEdit`. The renderer imposes the current working set `W` and records exactly what was imposed. This addresses `D1`, `D3`, `D5`, and `D15`, and implements `P1`, `P2`, `P5`, and `P6`.

`W` is governed by an effective-reasoning budget, not by the model's advertised context window. The budget is expressed as node-count, evidence-count, depth, priority, and token ceilings. A render that fits the vendor limit but contains low-value distractors is invalid. This addresses `D1` and `D13`, and implements `P2` and `P12`.

### Snapshot-Walk-Then-Merge

Each orchestrator turn reads a stable `GraphSnapshot`. The optimizer may continue preparing edits while the orchestrator thinks, but those edits merge only after the turn boundary. The next turn sees either the previous graph version plus accepted optimizer edits, or a conflict surface. This addresses concurrent mutation in `D3`, identity stability in `D4`, and recovery in `D16`; it implements `P4`, `P6`, and `P14`.

The harness does not lock the graph for the duration of a turn. It uses immutable snapshot reads, append-only optimizer mutation proposals, optimistic merge, and explicit conflict records. This preserves live optimizer operation while keeping the agent's per-turn view deterministic. The tradeoff against stronger locking is cost and latency, which is the known `P6` versus `P12` tension.

### Summary Contract

Every summary shown in `W` conforms to a machine-checkable `SummaryContract`. A summary must state focus, status, uncertainty, blockers, evidence pointers, stale markers, and unpack affordances. If those fields cannot be produced from evidence, the node becomes contract-invalid rather than confidently summarized. This addresses `D2`, `D10`, `D12`, and `D15`; it implements `P3` and `P7`.

The renderer may show contract-invalid nodes, but it marks them as invalid and prevents them from being treated as settled state. This makes summary drift visible rather than hiding it behind fluent prose, following `P1`, `P3`, and `P13`.

### Stable Identity

Every graph object with operational meaning has a stable identifier independent of text, path, parent, or current summary. Splits, merges, re-parents, and summary regeneration produce identity records, not silent replacement. This addresses `D4`, `D6`, `D8`, `D9`, and `D16`; it implements `P4` and `P15`.

Identity forwarding is part of normal operation. A merge keeps all prior node IDs resolving to the merged node until references are rewritten. A split keeps the original ID as the continuation node chosen by the split operation and creates child IDs for extracted concerns. Re-parenting preserves IDs. Identity drift is corruption.

### Provenance Beside Summaries

Raw evidence is stored separately from summaries and linked into the graph through provenance pointers. Evidence includes CLI transcripts, tool calls, tool results, file reads, patches, commands, user answers, optimizer prompts, reviewer outputs, and sub-agent outputs. This addresses `D2`, `D8`, `D10`, `D11`, and `D12`; it implements `P7`, `P10`, and `P15`.

Summaries are never the only copy of evidence. Old evidence may be packed deeper, compressed for render, or omitted from `W`, but it is not deleted merely because a summary exists. User deletion is explicit and audited.

### Cross-CLI Adaptation

The harness uses each CLI's strongest available context injection surface:

- Claude orchestrator sessions use rich hook-based prompt injection where available.
- Codex workers use their configured hook surface and wrapper-layer launch/resume staging.
- Opencode workers use plugin transforms where available and wrapper-layer session management.
- MCP is used for portable graph tools, resource reads, and affordances, but not as the only context mechanism.

This addresses `D7`, `D8`, `D9`, and `D10`; it implements `P8` and `P11`.

The UI exposes a per-session `CapabilityFingerprint` so the user and orchestrator can see when a worker has weaker context injection, weaker tool interception, or weaker resume guarantees. The harness does not pretend all CLIs are equivalent.

### No In-Product Compaction

The harness replaces `/compact` and equivalent in-product compaction mechanisms. Render size, prompt prefix stability, and working-set discipline are designed so CLI auto-compaction does not fire. If an underlying CLI compacts anyway, the resulting session state is marked corrupt until reconciled from graph evidence. This addresses `D1`, `D10`, `D15`, and `D16`; it implements `P9`, `P7`, and `P14`.

### Questions as Continuations

`NEEDS_INPUT` is not chat text. It is a `QuestionArtifact` attached to a worker session, blocked output, graph slice, render, and resume correlation key. User answers resume the blocked work through `agents resume` or the CLI-specific equivalent, and the harness waits for child acceptance rather than only recording a resume attempt. This addresses `D9` and `D16`; it implements `P11` and `P14`.

### Cost as Correctness

The optimizer, reviewer, renderer, and sub-agent dispatcher all spend tokens and latency. The harness records budget usage per initiative, worker, optimizer pass, reviewer pass, render, and CLI. Budget overrun changes behavior: optimizer cadence narrows, reviewer sampling becomes stricter, and new sub-agent dispatch requires explicit user approval when configured thresholds are exceeded. This addresses `D13` and implements `P12`.

The system does not treat cost as billing-only telemetry. A graph that is too expensive to maintain is operationally wrong because it cannot sustain long-lived work.

## Architecture

### Components

`DesktopShell`

- Tauri v2 desktop shell.
- Owns local window state, notification presentation, file access prompts, and user-visible recovery dialogs.
- Exists for `P13`, `P14`, `P15`, and `P16`; addresses `D14` and `D16`.

`HarnessBackend`

- Rust/Tokio service embedded in the Tauri app.
- Owns the harness SQLite database, graph mutation transactions, render generation, policy enforcement, and event streaming to the UI.
- Talks to `agents` as a subprocess boundary and reads `agent-runner` state only through documented local files or SQLite access.
- Addresses `D7`, `D10`, `D13`, `D16`; implements `P8`, `P12`, `P15`.

`GraphStore`

- Local SQLite database plus filesystem-backed evidence blobs.
- Stores graph objects, revisions, evidence, renders, worker slices, questions, budgets, recovery actions, and audit events.
- The harness graph is canonical. CLI transcripts and `agent-runner` state are evidence sources, not canonical graph state.
- Addresses `D4`, `D10`, `D12`, `D15`, `D16`; implements `P4`, `P7`, `P15`.

`RenderEngine`

- Converts a graph snapshot into CLI-specific prompt material and tool affordances.
- Applies working-set policy, summary contract validation, privilege labels, cost constraints, and capability fingerprints.
- Addresses `D1`, `D5`, `D7`, `D15`; implements `P1`, `P2`, `P3`, `P8`, `P12`.

`OrchestratorBridge`

- Runs and resumes the `claude-opus` or `claude-sonnet` orchestrator through `agents`.
- Injects rendered context, exposes pack/unpack/focus and graph query tools, captures tool calls and model outputs, writes turn records, and emits bounded foreground graph actions.
- Foreground graph actions are append-only records or advisory optimizer requests. They never create, revise, delete, split, merge, re-parent, repack, summarize, cross-reference, or otherwise curate graph topology.
- Addresses `D7`, `D10`, `D16`; implements `P5`, `P8`, `P9`, `P14`.

`WorkerDispatcher`

- Creates `WorkerSlice` records, selects configured model names, starts sessions through `agents`, tracks acceptance, and captures session evidence through `agent-runner` ingestion.
- Does not directly merge worker output into graph truth. It stages output for reintegration.
- Addresses `D8`, `D9`, `D13`; implements `P8`, `P10`, `P11`, `P12`.

`Optimizer`

- Background `glm` actor that proposes summary regenerations, stale markers, cross-references, topology edits, provenance repair, and repacking.
- Writes `OptimizerEdit` drafts against a base snapshot. The backend validates and merges them.
- May consume `OptimizerRequest` artifacts from the orchestrator, workers, backend, or user surface, but evaluates them as advisory inputs rather than delegated instructions.
- Addresses `D2`, `D3`, `D4`, `D6`, `D12`, `D13`, `D15`; implements `P3`, `P4`, `P6`, `P7`, `P12`.

`WorkflowReviewer`

- LLM reviewer, usually `gpt-high`, used as an evidence channel over selected optimizer edits, worker reintegration plans, and anomalous traces.
- Its output never becomes authoritative by itself.
- Addresses `D11` and implements `P10` and `P12`.

`PolicyEngine`

- Deterministic validator for schema constraints, privilege boundaries, tool-call protocol integrity, render limits, identity invariants, and budget gates.
- Owns policy versioning and attaches `policy_version` to decisions.
- Addresses `D10`, `D11`, `D12`, `D13`, `D16`; implements `P10`, `P12`, `P14`, `P15`.

`UserSurface`

- Single-tab UI with structured panes for initiatives, current focus, working set, questions, workers, optimizer edits, cost, recovery, and evidence drill-down.
- It is not a sidebar of separate chats.
- Addresses `D14`; implements `P1`, `P13`, and `P16`.

## Data Model

All durable identifiers use opaque stable IDs. IDs are not content hashes and do not encode parent path. All records carry `created_at`, `updated_at`, `actor`, and `policy_version` unless stated otherwise.

### GraphWorkspace

Purpose: root container for the one user's one harness graph.

Fields:

- `workspace_id`: stable local ID.
- `schema_version`: graph schema version.
- `active_orchestrator_id`: current orchestrator record.
- `current_graph_version`: latest accepted graph version.
- `storage_root`: filesystem root for evidence blobs.
- `policy_set_id`: active policy bundle.

Relationships:

- Owns `GraphNode`, `GraphEdge`, `GraphSnapshot`, `AuditEvent`, and `BudgetLedger`.
- Has exactly one active orchestrator, matching `P16`.

Addresses `D15` and `D16`; implements `P15` and `P16`.

### GraphNode

Purpose: stable logical object the agent can see, unpack, reference, assign, or derive from.

Fields:

- `node_id`: stable identity.
- `kind`: `initiative`, `work_unit`, `summary`, `evidence`, `question`, `decision`, `blocker`, `worker_output`, `recovery`, `policy_note`, `archive`.
- `title`: human-readable label, not identity.
- `lifecycle_state`: `active`, `packed`, `unpacked`, `blocked`, `stale`, `recovering`, `archived`, `quarantined`, `deleted`.
- `current_revision_id`: points to latest accepted `NodeRevision`.
- `canonical_parent_edge_id`: current containment edge if one exists.
- `privilege_origin`: `system`, `user`, `tool`, `model`, `worker`, `optimizer`, `reviewer`.
- `trust_state`: `trusted`, `derived`, `unverified`, `suspect`, `poison_quarantined`.
- `deleted_at`: null unless explicitly deleted.

Relationships:

- Has many `NodeRevision`.
- Connected through `GraphEdge`.
- May have one current `SummaryContract` revision.
- May be referenced by `WorkerSlice`, `QuestionArtifact`, `WorkingSetSnapshot`, and `OptimizerEdit`.

Identity semantics:

- `node_id` remains stable across summary regeneration and moves.
- Split and merge behavior is recorded in `IdentityEvent`, not hidden.

Addresses `D4`, `D6`, `D8`, `D9`; implements `P4`, `P7`, `P15`.

### NodeRevision

Purpose: immutable content revision for a node.

Fields:

- `revision_id`: stable revision ID.
- `node_id`: owning node.
- `graph_version`: version where revision became current.
- `content_ref`: pointer to structured content or evidence blob.
- `summary_contract_id`: summary for this revision if applicable.
- `change_reason`: `optimizer_edit`, `worker_reintegration`, `user_answer`, `recovery`, `manual_user_edit`, `policy_repair`.
- `base_revision_ids`: parent revisions used to derive this revision.
- `evidence_ids`: evidence supporting the revision.
- `valid_from_graph_version`, `valid_to_graph_version`.

Relationships:

- Revisions are append-only.
- Summaries and provenance point to revisions, not just nodes, when claim-level precision matters.

Addresses `D2`, `D3`, `D10`, `D16`; implements `P6`, `P7`, `P14`.

### GraphSnapshot

Purpose: immutable read view used for renders, optimizer drafts, worker slices, and recovery preflight.

Fields:

- `graph_snapshot_id`.
- `graph_version`.
- `node_revision_ids`: node to current revision mapping at snapshot time.
- `edge_ids`: active edge set at snapshot time.
- `identity_resolution_id`: forwarding map used by this snapshot.
- `policy_set_id`.
- `sealed_at`.
- `created_for`: `orchestrator_turn`, `worker_slice`, `optimizer_edit`, `recovery_preflight`, `ui_inspection`.
- `base_transaction_id`.

Relationships:

- Referenced by `WorkingSetSnapshot`, `OptimizerEdit`, `WorkerSlice`, `ConflictRecord`, and `RecoveryAction`.
- Never changes after creation.

Identity semantics:

- A snapshot resolves IDs according to its own identity map. Later forwarding does not rewrite what a prior snapshot meant.

Addresses `D3`, `D4`, `D5`, `D16`; implements `P4`, `P6`, `P14`.

### GraphEdge

Purpose: typed relationship between graph nodes.

Fields:

- `edge_id`: stable edge ID.
- `from_node_id`, `to_node_id`.
- `edge_type`: `contains`, `derives_from`, `evidences`, `blocks`, `answers`, `assigned_to`, `produced_by`, `cross_ref`, `supersedes`, `forwards_to`, `resumes`, `conflicts_with`, `packs_into`, `unpacks_to`.
- `state`: `active`, `stale`, `candidate`, `rejected`, `archived`.
- `weight`: optional relevance or confidence score.
- `validity`: `current`, `historical`, `forwarding`.
- `created_by_edit_id`: optional optimizer edit or recovery action.
- `evidence_ids`: support for the edge.

Relationships:

- Containment edges define current topology.
- Non-containment edges define provenance, blockers, worker ownership, and cross-references.

Addresses `D4`, `D6`, `D8`, `D10`; implements `P4`, `P7`.

### IdentityEvent

Purpose: durable record of identity-preserving topology changes.

Fields:

- `identity_event_id`.
- `event_type`: `created`, `summary_regenerated`, `moved`, `split`, `merged`, `forwarded`, `restored`, `deleted`.
- `input_node_ids`.
- `output_node_ids`.
- `continuation_node_id`: required for splits.
- `forwarding_map`: prior ID to current ID mappings.
- `base_graph_version`, `result_graph_version`.
- `reason`.

Relationships:

- Used by lookup resolution before every graph dereference.
- Referenced by `RecoveryAction` and `AuditEvent`.

Addresses `D4`, `D16`; implements `P4`, `P14`.

### SummaryContract

Purpose: machine-checkable summary rendered when detail is packed.

Fields:

- `summary_contract_id`.
- `node_id`, `revision_id`.
- `contract_version`.
- `focus`: current purpose of the node.
- `status`: `not_started`, `active`, `blocked`, `waiting_for_user`, `waiting_for_worker`, `recovering`, `complete`, `archived`, `invalid`.
- `decision_state`: decisions made, deferred, or disputed.
- `uncertainty`: explicit unknowns and confidence notes.
- `open_blockers`: blocker node IDs or question IDs.
- `evidence_pointers`: provenance pointer IDs.
- `stale_markers`: reasons the summary may no longer match evidence.
- `unpack_affordances`: named reasons to unpack, with child node IDs or query handles.
- `omitted_detail_classes`: categories intentionally omitted from render.
- `poison_risk`: privilege or prompt-injection concern if known.
- `validation_state`: `valid`, `invalid_missing_evidence`, `invalid_conflict`, `invalid_stale`, `invalid_policy`, `needs_review`.
- `generated_by`: `optimizer`, `manual_user_edit`, `import`.

Relationships:

- Must cite `EvidenceArtifact` or `NodeRevision` through `ProvenancePointer`.
- Consumed by `RenderEngine` and inspected by `PolicyEngine`.

Addresses `D2`, `D12`, `D15`; implements `P3`, `P7`.

### EvidenceArtifact

Purpose: raw or normalized source content that summaries derive from.

Fields:

- `evidence_id`.
- `source_type`: `cli_transcript`, `tool_call`, `tool_result`, `file_read`, `file_patch`, `command_output`, `user_message`, `worker_output`, `optimizer_prompt`, `reviewer_output`, `external_document`.
- `source_uri`: local path, session URI, or internal blob URI.
- `source_session_id`: CLI or `agent-runner` session ID if applicable.
- `tool_protocol`: `claude`, `codex`, `opencode`, `mcp`, `shell`, `none`.
- `correlation_key`: tool call ID, deferred tool ID, request ID, or message ID.
- `content_hash`.
- `blob_ref`.
- `privilege_origin`.
- `capture_state`: `captured`, `partial`, `failed`, `redacted`, `quarantined`.
- `captured_at`.

Relationships:

- Evidence can support many summaries, edges, decisions, and recovery actions.
- Evidence is never replaced by summaries.

Addresses `D10`, `D12`, `D15`; implements `P7`, `P15`.

### ProvenancePointer

Purpose: claim-level pointer from derived graph content to evidence.

Fields:

- `provenance_id`.
- `claim_id`: local identifier inside a summary, decision, edge, or revision.
- `evidence_id`.
- `evidence_locator`: byte range, line range, JSON path, message index, or tool result section.
- `derivation_type`: `quoted`, `paraphrased`, `inferred`, `computed`, `model_judged`.
- `confidence`: bounded numeric or enum confidence.
- `privilege_transform`: whether lower-privilege content affected higher-level summary text.

Relationships:

- Used by contract validation, reviewer sampling, and evidence drill-down.

Addresses `D2`, `D10`, `D11`, `D12`; implements `P7`, `P10`.

### WorkingSetSnapshot

Purpose: exact rendered context imposed on an agent turn.

Fields:

- `working_set_id`.
- `graph_snapshot_id`.
- `turn_id`.
- `target_actor`: `orchestrator` or `worker`.
- `target_cli`, `target_model`.
- `render_policy_id`.
- `pinned_node_ids`.
- `unpacked_node_ids`.
- `summary_node_ids`.
- `evidence_pointer_ids`.
- `recursive_depth_by_node`.
- `evicted_node_ids`.
- `token_estimate`.
- `reasoning_budget_class`: `small`, `standard`, `large`, `exception`.
- `prefix_hash`: prompt-cache locality key.
- `capability_fingerprint_id`.
- `rendered_blob_ref`.

Relationships:

- Referenced by every orchestrator turn, worker launch, and question artifact.
- Provides the answer to "what did the model see?"

Addresses `D1`, `D5`, `D7`, `D15`; implements `P1`, `P2`, `P8`, `P12`.

### GraphAction

Purpose: bounded foreground graph-adjacent write emitted during an orchestrator turn.

Fields:

- `graph_action_id`.
- `source_turn_id`.
- `base_graph_snapshot_id`.
- `actor`: normally `orchestrator`; backend-created actions may record capture or policy results.
- `action_type`: `record_model_output`, `record_tool_provenance`, `attach_audit_note`, `emit_user_facing_output`, `create_optimizer_request`.
- `target_refs`: existing turn, evidence, tool, node, or working-set IDs referenced by the action.
- `payload_ref`: local blob or structured payload.
- `optimizer_request_id`: present only when `action_type` is `create_optimizer_request`.
- `deterministic_validation_state`: `pending`, `passed`, `failed`.
- `commit_state`: `draft`, `committed`, `rejected`.

Allowed effects:

- Create `EvidenceArtifact` records for orchestrator output or tool evidence.
- Create `ToolCallProvenance` records for protocol-sensitive tool state.
- Create `AuditEvent` records that explain foreground decisions.
- Create `OptimizerRequest` artifacts that ask the optimizer to consider graph curation.

Disallowed effects:

- Create, update, delete, split, merge, re-parent, repack, or forward `GraphNode` identity.
- Create or modify `GraphEdge`, including containment, `cross_ref`, `packs_into`, and `unpacks_to`.
- Create or modify `NodeRevision`, `SummaryContract`, or `IdentityEvent`.
- Regenerate summaries, mark stale state as graph truth, repair provenance, quarantine nodes, or mutate topology.

Relationships:

- Referenced by `OrchestratorTurn`.
- May reference existing graph IDs but cannot change their graph version.
- May produce `OptimizerRequest`; only a later `OptimizerEdit` can mutate graph truth.

Addresses `D3`, `D4`, `D5`, `D10`, and `D15`; implements `P1`, `P4`, `P5`, `P6`, and `P7`.

### AgentWalkState

Purpose: live focus state for the orchestrator.

Fields:

- `walk_state_id`.
- `orchestrator_id`.
- `current_focus_node_id`.
- `pinned_node_ids`.
- `unpacked_stack`: ordered node IDs and depths.
- `recent_pack_events`.
- `pending_focus_requests`.
- `last_stable_graph_snapshot_id`.
- `state`: `idle`, `rendering`, `thinking`, `awaiting_tool_result`, `recording_turn_artifact`, `blocked`, `recovering`.

Relationships:

- Produces `WorkingSetSnapshot`.
- Updated by pack, unpack, and focus tools. `GraphAction` records may cite walk state, but they do not modify graph topology.

Addresses `D5`, `D6`, `D15`; implements `P2` and `P5`.

### WorkerSlice

Purpose: bounded subgraph assigned to a sub-agent.

Fields:

- `slice_id`.
- `initiative_node_id`.
- `assigned_root_node_ids`.
- `allowed_adjacent_node_ids`.
- `excluded_node_ids`.
- `write_scope`: `none`, `proposal_only`, `owned_nodes`, `patch_files`, `question_only`.
- `overlap_policy`: `exclusive`, `shared_read`, `shared_write_requires_reintegration`.
- `base_graph_snapshot_id`.
- `render_id`.
- `worker_id`.
- `target_cli`, `target_model`.
- `state`: `draft`, `validated`, `launched`, `accepted`, `running`, `needs_input`, `completed`, `failed`, `cancelled`, `reintegrating`, `integrated`, `conflicted`.

Relationships:

- Owned by `WorkerRun`.
- May produce `QuestionArtifact`, `WorkerOutput`, `OptimizerRequest`, and `ConflictRecord`.

Addresses `D8`, `D9`, `D13`; implements `P8`, `P11`, `P12`.

### WorkerRun

Purpose: provider-level execution record for a worker.

Fields:

- `worker_id`.
- `slice_id`.
- `agent_runner_invocation_id`.
- `session_id`.
- `resume_supported`: boolean from capability fingerprint and observed acceptance.
- `acceptance_state`: `unknown`, `accepted`, `rejected`, `timed_out`, `ambiguous`.
- `last_ingested_message_id`.
- `state`: `starting`, `running`, `blocked`, `completed`, `failed`, `lost`, `cancelled`.
- `cost_ledger_id`.

Relationships:

- Links harness slice state to `agent-runner` invocation/session state.

Addresses `D7`, `D8`, `D16`; implements `P8`, `P11`, `P15`.

### OrchestratorTurn

Purpose: durable record of one foreground orchestrator interaction.

Fields:

- `turn_id`.
- `orchestrator_id`.
- `session_id`.
- `graph_snapshot_id`.
- `working_set_id`.
- `input_message_ref`.
- `output_message_ref`.
- `tool_event_ids`.
- `graph_action_refs`: refs to bounded `GraphAction` records only.
- `optimizer_request_ids`: advisory requests emitted during the turn.
- `state`: `rendered`, `accepted`, `tool_pending`, `captured`, `committed`, `blocked`, `recovering`, `failed`.
- `started_at`, `completed_at`.
- `cost_ledger_id`.

Relationships:

- Owns the foreground trace for one stable graph snapshot.
- Referenced by evidence artifacts, tool-call provenance, recovery actions, and audit events.

Addresses `D1`, `D3`, `D5`, `D10`, `D16`; implements `P1`, `P5`, `P6`, `P7`, `P14`.

### QuestionArtifact

Purpose: durable continuation for user input.

Fields:

- `question_id`.
- `node_id`: graph node representing or pointing to the question.
- `slice_id`.
- `worker_id`.
- `blocked_output_ref`.
- `render_id`.
- `request_payload_ref`: disk artifact following the external question convention when needed.
- `correlation_key`.
- `question_text`.
- `answer_payload_ref`.
- `state`: `created`, `presented`, `answered`, `resume_requested`, `child_accepted`, `resumed`, `failed_resume`, `cancelled`, `superseded`.
- `routing_attempts`.

Relationships:

- Blocks one or more nodes.
- Resumes exactly one blocked worker continuation unless explicitly superseded.

Addresses `D9`, `D16`; implements `P11` and `P14`.

### ToolCallProvenance

Purpose: first-class record of protocol-sensitive tool state.

Fields:

- `tool_event_id`.
- `evidence_id`.
- `session_id`.
- `turn_id`.
- `protocol`: `claude`, `codex`, `opencode`, `mcp`, `shell`.
- `tool_call_id`.
- `tool_name`.
- `tool_input_hash`.
- `tool_result_id`.
- `approval_state`: `not_required`, `pending`, `approved`, `rejected`, `expired`.
- `side_effect_class`: `read_only`, `local_write`, `external_write`, `process`, `network`.
- `retry_semantics`: `idempotent`, `non_idempotent`, `unknown`.
- `state`: `requested`, `approved`, `executed`, `result_recorded`, `failed`, `orphaned`, `reconciled`.

Relationships:

- Attached to evidence, recovery actions, reviewer checks, and summaries that derive from tool results.

Addresses `D10`, `D16`; implements `P7`, `P10`, `P14`.

### OptimizerRequest

Purpose: advisory request asking the optimizer to consider graph curation.

Fields:

- `optimizer_request_id`.
- `source_type`: `orchestrator_turn`, `worker_output`, `user_surface`, `backend_signal`, `recovery_action`.
- `source_ref`.
- `base_graph_snapshot_id`.
- `target_node_ids`.
- `request_type`: `consider_summary_refresh`, `consider_stale_mark`, `consider_cross_reference`, `consider_repack`, `consider_split`, `consider_merge`, `consider_reparent`, `consider_provenance_repair`, `consider_quarantine`.
- `rationale_ref`: evidence, turn output, worker output, or audit note that motivated the request.
- `priority_hint`: `low`, `normal`, `high`, `urgent`.
- `advisory_state`: `queued`, `accepted_for_scoping`, `ignored`, `superseded`, `converted_to_optimizer_edit`.
- `converted_optimizer_edit_id`: null unless the optimizer independently drafts an edit from the request.

Relationships:

- May be produced by a foreground `GraphAction`, worker reintegration staging, recovery, or UI action.
- Is input to the optimizer queue, not a graph mutation.
- Does not authorize the source actor to decide that nodes, revisions, edges, summaries, or topology changes should exist.
- If the optimizer acts on the request, it creates a separate `OptimizerEdit` with its own scope, evidence, validation, reviewer state, audit trail, and actor attribution.

Addresses `D3`, `D4`, `D5`, `D6`, and `D15`; implements `P1`, `P4`, `P5`, and `P6`.

### OptimizerEdit

Purpose: proposed background graph mutation.

Fields:

- `optimizer_edit_id`.
- `base_graph_snapshot_id`.
- `actor_model`: normally `glm`.
- `edit_type`: `summary_regeneration`, `stale_mark`, `cross_reference`, `repack`, `split`, `merge`, `reparent`, `provenance_repair`, `poison_quarantine`.
- `touched_node_ids`.
- `operation_payload_ref`.
- `evidence_ids`.
- `expected_invariants`.
- `deterministic_validation_state`: `pending`, `passed`, `failed`.
- `reviewer_state`: `not_required`, `sampled_pending`, `passed`, `flagged`, `inconclusive`.
- `merge_state`: `draft`, `validated`, `merged`, `conflicted`, `rejected`, `reverted`.
- `result_graph_version`.

Relationships:

- Produces `IdentityEvent`, `NodeRevision`, `GraphEdge`, and `AuditEvent` only after merge.
- May cite `OptimizerRequest` as an input, but the edit remains an optimizer-owned curation decision and is shown in the user surface as such.

Addresses `D3`, `D4`, `D6`, `D11`, `D12`, `D13`; implements `P1`, `P4`, `P5`, `P6`, `P10`, `P12`, `P14`.

### ConflictRecord

Purpose: explicit record when optimistic merge cannot safely apply.

Fields:

- `conflict_id`.
- `base_graph_snapshot_id`.
- `current_graph_snapshot_id`.
- `actor_a`, `actor_b`.
- `conflict_type`: `identity`, `content`, `summary_contract`, `edge`, `worker_overlap`, `question_route`, `tool_protocol`, `budget`.
- `affected_node_ids`.
- `resolution_state`: `open`, `auto_resolved`, `needs_orchestrator`, `needs_user`, `rejected`, `superseded`.
- `resolution_edit_id`.

Relationships:

- Blocks merge or reintegration until resolved.

Addresses `D3`, `D4`, `D8`, `D16`; implements `P6`, `P14`.

### CapabilityFingerprint

Purpose: visible per-CLI/session capability description.

Fields:

- `capability_fingerprint_id`.
- `cli`: `claude`, `codex`, `opencode`.
- `model`.
- `injection_surfaces`: hooks, plugin transforms, wrapper staging, MCP, headless API.
- `resume_surface`: session ID support, acceptance signal, known limits.
- `tool_interception_surface`.
- `context_strength`: `strong`, `medium`, `weak`, `unknown`.
- `known_asymmetries`.
- `observed_failures`.

Relationships:

- Attached to renders, worker runs, and UI session badges.

Addresses `D7`, `D9`, `D10`; implements `P8`, `P11`, `P13`.

### BudgetLedger

Purpose: cost and latency control record.

Fields:

- `budget_ledger_id`.
- `scope_type`: `workspace`, `initiative`, `orchestrator_turn`, `worker_run`, `optimizer_pass`, `reviewer_pass`, `render`.
- `scope_id`.
- `input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`.
- `latency_ms`.
- `provider_cost_estimate`.
- `cache_prefix_hash`.
- `budget_state`: `within`, `near_limit`, `exceeded`, `blocked`.
- `policy_action`: `none`, `warn`, `narrow_scope`, `require_user_approval`, `block`.

Relationships:

- Consulted by renderer, optimizer, worker dispatcher, and reviewer sampler.

Addresses `D13`; implements `P12`.

### RecoveryAction

Purpose: auditable operation that changes state during resume, rollback, revert, or cancellation.

Fields:

- `recovery_action_id`.
- `action_type`: `resume_session`, `retry_recording`, `retry_execution`, `rollback_graph`, `revert_optimizer_edit`, `cancel_worker`, `restore_question`, `quarantine_session`, `fresh_worker_substitution`.
- `precondition_ref`.
- `affected_session_ids`.
- `affected_node_ids`.
- `affected_edit_ids`.
- `side_effect_classification`: `deferred_recording`, `deferred_execution`, `unknown`.
- `user_confirmation_state`: `not_required`, `required`, `granted`, `denied`.
- `result_state`: `planned`, `applied`, `failed`, `partially_applied`, `reverted`.
- `preserved_ref`, `replayed_ref`, `discarded_ref`.

Relationships:

- Produces audit events and may produce conflict records.

Addresses `D16`; implements `P14`, `P15`.

### PolicySet

Purpose: versioned deterministic governance rules.

Fields:

- `policy_set_id`.
- `summary_contract_version`.
- `render_policy_version`.
- `identity_policy_version`.
- `privilege_policy_version`.
- `tool_protocol_policy_version`.
- `budget_policy_version`.
- `review_sampling_policy_version`.
- `recovery_policy_version`.

Relationships:

- Referenced by all decisions and audit events.
- Allows later explanation of why a decision was accepted at the time.

Addresses `D11`, `D12`, `D13`, `D16`; implements `P10`, `P12`, `P14`.

### AuditEvent

Purpose: append-only local record of decisions and state changes.

Fields:

- `audit_event_id`.
- `event_type`.
- `actor`.
- `policy_set_id`.
- `input_refs`.
- `output_refs`.
- `decision`: `accepted`, `rejected`, `deferred`, `quarantined`, `user_required`.
- `reason_code`.
- `created_at`.

Relationships:

- Every graph mutation, render, optimizer edit, reviewer decision, question route, and recovery action emits audit events.

Addresses `D11`, `D12`, `D14`, `D16`; implements `P1`, `P10`, `P13`, `P14`, `P15`.

## Working-Set Policy

The working set is represented as a rendered view over a graph snapshot. It has four layers:

- Required pins: system constraints, active initiative root, active focus node, unresolved user questions relevant to focus, and recovery warnings.
- Active detail: nodes the orchestrator explicitly unpacked and has not packed or lost by eviction.
- Visible summaries: siblings, parents, blockers, and cross-references needed to navigate without flooding detail.
- Evidence affordances: compact pointers that can be unpacked on demand.

This addresses `D1`, `D5`, `D6`, and `D15`; it implements `P1`, `P2`, `P3`, `P5`, and `P7`.

Eviction order:

1. Unpinned low-relevance sibling summaries.
2. Evidence affordances not touched in the current initiative.
3. Deep unpacked descendants whose parent summary is contract-valid and not blocked.
4. Older active detail outside the current focus path.
5. Anything beyond budget that is not a required pin.

Required pins cannot be evicted silently. If required pins exceed budget, the render enters `overfull_required_context` and asks the orchestrator or user to resolve focus, rather than rendering a noisy `W`. This addresses `D1` and `D5`; it implements `P2` and `P13`.

Recursive unpack is bounded by:

- `max_depth`.
- `max_child_count`.
- `max_evidence_items`.
- `node_kind_allowlist`.
- `budget_remaining`.
- privilege and poison state.

Recursive unpack never crosses into `poison_quarantined`, `deleted`, or `unresolved_conflict` nodes without an explicit tool action. This addresses `D6` and `D12`; it implements `P2`, `P6`, `P7`, and `P14`.

## Operational Lifecycle

### Orchestrator Turn

States:

- `idle`: no turn is currently being prepared.
- `snapshotting`: backend selects a stable graph snapshot.
- `rendering`: render engine creates CLI-specific `WorkingSetSnapshot`.
- `launching_or_resuming`: bridge calls `agents`.
- `thinking`: orchestrator owns the turn.
- `tool_pending`: a tool call, walk-state tool, or bounded `GraphAction` record is pending.
- `capturing`: backend ingests session output and tool provenance.
- `committing`: evidence, tool provenance, audit notes, and advisory optimizer requests are written.
- `optimizer_enqueue`: affected nodes are queued for optimizer consideration.
- `complete`: turn is durable.
- `blocked`: user input or recovery is required.
- `recovering`: recovery workflow owns the next state change.

Transitions:

- `idle -> snapshotting` when a turn is requested.
- `snapshotting -> rendering` after identity forwarding and policy validation.
- `rendering -> launching_or_resuming` if the render is within budget.
- `rendering -> blocked` if required context exceeds budget or has unreconciled corruption.
- `launching_or_resuming -> thinking` when the CLI accepts the session.
- `thinking -> tool_pending` when the model calls a tool.
- `tool_pending -> capturing` when the result is available or fails.
- `thinking -> capturing` on final model output.
- `capturing -> committing` after evidence and protocol records are durable.
- `committing -> optimizer_enqueue` after bounded `GraphAction` records pass policy and any `OptimizerRequest` artifacts are queued as advisory input.
- `optimizer_enqueue -> complete` after queues and audit events are written.
- Any state -> `recovering` on session loss, protocol corruption, or storage conflict.

`GraphAction` in this lifecycle is not a topology mutation path. The allowed set is limited to recording orchestrator output, recording tool provenance, attaching audit notes, emitting user-facing output, and creating advisory `OptimizerRequest` artifacts. It cannot create or change `GraphNode`, `GraphEdge`, `NodeRevision`, `SummaryContract`, or `IdentityEvent`; it cannot summarize, cross-reference, repack, split, merge, re-parent, forward identity, repair provenance, or quarantine graph truth.

If the orchestrator believes one of those curation operations is needed, it emits an `OptimizerRequest`; the optimizer later decides whether to draft an `OptimizerEdit`, and any resulting user-visible topology or summary change is attributed to the optimizer.

This lifecycle addresses `D1`, `D3`, `D4`, `D5`, `D7`, `D10`, `D13`, and `D16`; it implements `P1`, `P2`, `P4`, `P5`, `P6`, `P8`, `P12`, and `P14`.

### Pack, Unpack, and Focus Tools

The orchestrator has graph navigation tools:

- `focus(node_id, reason)`: sets current focus if the node is resolvable and allowed.
- `unpack(node_id, depth, reason)`: requests detail expansion under budget and privilege policy.
- `pack(node_id, reason)`: requests return to summary view.
- `pin(node_id, reason)`: requests persistence across turns; pins can be denied or downgraded if they violate budget policy.
- `unpin(node_id, reason)`: removes an agent-requested pin.

Tool states:

- `requested`.
- `validated`.
- `applied_to_walk_state`.
- `rendered_next_turn`.
- `denied_policy`.
- `denied_budget`.
- `denied_conflict`.

The tools modify `AgentWalkState`, not graph topology. This preserves the boundary between agent-owned focus and optimizer-owned curation, addressing `D5` and `D15` while implementing `P5`.

### Optimizer Cycle

States:

- `queued`: changed nodes, stale signals, or advisory `OptimizerRequest` artifacts are available.
- `scoped`: backend selects a bounded subgraph and budget.
- `snapshot_read`: optimizer receives immutable snapshot data.
- `drafting`: `glm` proposes edits.
- `schema_validation`: deterministic validation checks shape and identity rules.
- `contract_validation`: summary contract and provenance rules are checked.
- `policy_validation`: privilege, poison, budget, and tool-protocol policies run.
- `review_sampling`: reviewer may inspect selected edits.
- `merge_attempt`: backend applies edit against current graph if preconditions hold.
- `merged`: edit becomes visible next turn.
- `conflicted`: conflict record is opened.
- `rejected`: edit is discarded with reason.
- `reverted`: previously merged edit is undone by a recovery action.

Transitions:

- `queued -> scoped` only if budget policy allows optimizer work. Advisory requests may influence scope but do not force it.
- `scoped -> snapshot_read -> drafting`.
- `drafting -> schema_validation`.
- Any validation failure -> `rejected` or `conflicted` depending on consequence.
- Validation pass -> `review_sampling` when sampler selects the edit, otherwise `merge_attempt`.
- Reviewer flag -> `conflicted` unless deterministic policy already rejects.
- `merge_attempt -> merged` when base preconditions still hold.
- `merge_attempt -> conflicted` when foreground changes invalidate preconditions.

The optimizer cannot directly change the orchestrator's current turn. Its accepted edits become candidates for the next snapshot. This addresses `D3`, `D6`, `D11`, `D12`, and `D13`; it implements `P6`, `P10`, `P12`, and `P14`.

### Sub-Agent Dispatch and Reintegration

States:

- `slice_draft`: orchestrator or backend proposes a worker slice.
- `slice_validated`: policy checks overlap, evidence, capability, and budget.
- `launched`: `agents` invocation created.
- `accepted`: child session acceptance observed.
- `running`: worker is producing trace.
- `needs_input`: worker emitted a valid question artifact.
- `completed`: worker finished without pending question.
- `failed`: worker failed or session evidence is incomplete.
- `reintegrating`: output is mapped to graph operations.
- `integrated`: accepted output is merged.
- `conflicted`: output overlaps or contradicts active graph state.
- `cancelled`: user or orchestrator cancelled the slice.

Worker output reintegration is not final-response paste. It produces one or more of:

- staged `NodeRevision` update candidates.
- new evidence artifacts.
- summary invalidation candidates.
- new blocker or question candidates.
- cross-reference candidates.
- patch or command provenance.
- advisory `OptimizerRequest` artifacts for optimizer-owned curation.
- conflict records.

Curation-affecting candidates from reintegration follow the same boundary as orchestrator suggestions: they may become `OptimizerRequest` inputs, but only optimizer-owned `OptimizerEdit` records mutate summaries, cross-references, topology, identity, provenance repair, or quarantine state.

This addresses `D3`, `D8`, `D9`, `D10`, `D11`, and `D13`; it implements `P5`, `P6`, `P7`, `P8`, `P10`, `P11`, and `P12`.

Parallel workers may share read context, but overlapping write scope creates a `ConflictRecord` unless the slice policy explicitly allows staged merge. This addresses worker interference in `D8` and implements `P6`.

### NEEDS_INPUT Routing

States:

- `created`: worker emits a structured question.
- `validated`: harness verifies slice, worker, render, and correlation key.
- `presented`: UI shows the question in the global queue and relevant initiative.
- `answered`: user supplies answer.
- `resume_requested`: harness sends answer through the worker's resume surface.
- `child_accepted`: worker acceptance is observed.
- `resumed`: worker continues or completes.
- `failed_resume`: child did not accept or protocol state is corrupt.
- `superseded`: question no longer applies because graph or worker state changed.
- `cancelled`: user cancels blocked work.

Transitions:

- Invalid questions are quarantined, not routed as ordinary chat.
- A user answer cannot be applied to the orchestrator transcript as a substitute for child resume.
- If child acceptance cannot be observed, the question enters `failed_resume` and recovery decides whether to retry, substitute, or cancel.

This addresses `D9` and `D16`; it implements `P11` and `P14`.

### Recovery

States:

- `detected`: inconsistency, session loss, failed resume, corrupt compact, or merge failure is detected.
- `classified`: backend classifies deferred recording versus deferred execution, consequence level, and side-effect class.
- `preflight`: policy checks current graph, session availability, and tool protocol state.
- `requires_user`: high-consequence or unknown-side-effect actions wait for confirmation.
- `applying`: recovery action runs.
- `reconciled`: graph, evidence, and audit records agree.
- `partial`: some state was preserved and some was discarded.
- `failed`: recovery could not complete.

Recovery actions show preserved, replayed, and discarded state. Fresh worker substitution is explicit and visible; it is never reported as a successful resume. This addresses `D16`; it implements `P14` and `P13`.

### User Surface

The single-tab UI has structured regions backed by the same graph:

- Initiative map: initiative roots with `active`, `blocked`, `waiting`, `recovering`, `stale`, and `archived` states.
- Current focus: the orchestrator's active focus path, pins, unpacked nodes, and evicted nodes.
- Working set inspector: exact nodes and evidence pointers imposed on the current or last turn.
- Question queue: action-needed `QuestionArtifact` records across all initiatives.
- Worker board: active and blocked `WorkerSlice` records with capability fingerprints.
- Optimizer log: advisory requests plus accepted, rejected, conflicted, and reverted optimizer-owned edits.
- Evidence drill-down: summary claim to provenance to raw local evidence.
- Cost surface: budget ledgers by initiative, worker, optimizer pass, reviewer pass, and render.
- Recovery surface: active and historical recovery actions.

The UI separates action-needed notifications from passive progress events. This addresses `D14`, `D15`, and `D16`; it implements `P1`, `P13`, `P14`, and `P16`.

## Governance

### Deterministic Gates

The `PolicyEngine` enforces:

- Summary contract completeness.
- Evidence pointer existence and locator validity.
- Stable identity resolution.
- No unresolved forwarding cycles.
- Foreground `GraphAction` allowed-effect boundary.
- `OptimizerRequest` advisory-only boundary and optimizer attribution.
- Worker write-scope compliance.
- Question correlation validity.
- Tool-call protocol completeness.
- Privilege-origin preservation.
- Poison-risk quarantine.
- Render budget and required-pin handling.
- Recovery side-effect classification.

These gates address `D3`, `D4`, `D5`, `D10`, `D11`, `D12`, `D13`, and `D16`; they implement `P1`, `P3`, `P4`, `P5`, `P7`, `P10`, `P12`, and `P14`.

### Reviewer Sampling

The workflow reviewer is invoked for:

- high-consequence optimizer edits, including splits, merges, re-parenting, and poison quarantine reversals.
- worker reintegration that changes decisions, blockers, or user-visible status.
- anomaly-triggered cases, such as missing evidence, high uncertainty, repeated failed resumes, or protocol mismatches.
- sampled low-consequence edits according to the active review policy.

Reviewer output can:

- add flags.
- raise confidence in an already valid edit.
- request user or orchestrator attention.
- trigger conflict state.

Reviewer output cannot:

- bypass deterministic validation.
- convert missing evidence into valid evidence.
- authorize privilege escalation.
- prove an edit correct.

This addresses reviewer fallibility in `D11` and implements `P10` and `P12`.

### Privilege and Poisoning Controls

Every content-bearing object carries `privilege_origin`. Lower-privilege tool or worker text cannot become higher-privilege instruction text through summary regeneration. Optimizer prompts include privilege labels and must return provenance for any promoted claim. Prompt-like content from tool outputs is rendered as evidence, not instructions. Suspect evidence can be quarantined, causing dependent summaries to become stale or invalid.

This addresses `D12` and implements `P1`, `P7`, `P10`, and `P15`.

### Audit and Reversibility

All graph mutations are append-only at the revision layer. Reverts are new recovery actions that restore prior revisions and mark superseded edges; they do not erase history. The user can inspect the edit, evidence, policy version, reviewer signal, and recovery action that produced current state.

This addresses `D3`, `D4`, `D12`, and `D16`; it implements `P6`, `P14`, and `P15`.

## Observability

The harness records:

- rendered working sets and cache prefix hashes.
- node and evidence counts per render.
- summary contract validation outcomes.
- foreground graph-action validation outcomes and optimizer-request disposition.
- optimizer queue depth, edit types, merge success, conflicts, and reverts.
- worker session acceptance, completion, failures, and questions.
- tool-call protocol states.
- reviewer sample rates and outcomes.
- budget ledger usage.
- recovery actions and results.

Observability is graph-addressed, not just log-addressed. A user can start at an initiative, node, question, worker, or edit and drill to relevant events. This addresses `D14` and `D16`; it implements `P1`, `P13`, and `P14`.

## AI and ML Use

### Orchestrator

Actor: `claude-opus` by default, `claude-sonnet` when long-context capability matters more than reasoning depth.

Inputs:

- `WorkingSetSnapshot`.
- pack/unpack/focus tools.
- sub-agent dispatch affordances.
- question queue relevant to current focus.
- recovery and conflict notices.

Outputs:

- user-facing reasoning and decisions.
- graph navigation tool calls.
- worker dispatch requests.
- bounded `GraphAction` records for turn output, tool provenance, audit notes, user-facing output, and advisory `OptimizerRequest` creation.
- recovery decisions when policy routes to orchestrator.

Limitations:

- The orchestrator is still bounded by effective working-set quality, not nominal context.
- It cannot see evidence not rendered or unpacked.
- It cannot override deterministic graph policy.
- It cannot directly create or mutate graph topology, summaries, cross-references, node revisions, identity events, repacks, splits, merges, re-parents, provenance repairs, or quarantine state.
- Its curation suggestions are advisory `OptimizerRequest` artifacts; accepted topology or summary changes remain optimizer-authored decisions in the user surface.
- It may misunderstand lower-privilege evidence, so privilege labels and provenance must remain visible.

Addresses `D1`, `D3`, `D4`, `D5`, `D12`, `D15`; implements `P1`, `P2`, `P4`, `P5`, `P6`, `P7`.

### Sub-Agent Dispatch

Actor: selected configured model via `agents`, based on task shape and role definitions outside this proposal.

Inputs:

- `WorkerSlice`.
- CLI-specific render.
- capability fingerprint.
- write-scope instructions.
- question envelope contract.

Outputs:

- worker trace evidence.
- final or incremental output.
- `NEEDS_INPUT` artifacts.
- staged reintegration candidates and advisory `OptimizerRequest` artifacts.

Limitations:

- Workers have asymmetric context injection depending on CLI.
- Final output is not trusted without trace/provenance.
- Resume acceptance may fail or be ambiguous.
- Parallel workers can conflict.

Addresses `D7`, `D8`, `D9`, `D10`; implements `P8`, `P10`, `P11`.

### Continuous Optimizer

Actor: `glm`.

Inputs:

- bounded graph snapshot.
- changed node set.
- advisory `OptimizerRequest` artifacts.
- summary contracts.
- provenance pointers.
- budget policy.
- stale, conflict, and poison signals.

Outputs:

- `OptimizerEdit` drafts for summaries, cross-references, repacking, stale markers, splits, merges, re-parenting, provenance repair, and quarantine.

Limitations:

- It can draft but not merge its own edits.
- It may ignore, supersede, or reinterpret advisory requests; requests do not delegate curation authority to their source actor.
- It can introduce poisoning or drift if deterministic gates fail, so gates and audit are mandatory.
- It can destabilize cost and cache locality, so budget policy can narrow or block it.
- It cannot mutate the orchestrator's current snapshot.

Addresses `D2`, `D3`, `D4`, `D6`, `D12`, `D13`, `D15`; implements `P1`, `P3`, `P4`, `P5`, `P6`, `P7`, `P12`.

### Workflow Reviewer

Actor: `gpt-high` workflow-reviewer.

Inputs:

- selected optimizer edit or worker reintegration plan.
- relevant evidence pointers.
- policy checklist.
- compact trace excerpts.

Outputs:

- reviewer finding.
- confidence note.
- cited concerns.
- suggested conflict or approval signal.

Limitations:

- It is fallible and biased.
- It cannot inspect unlimited trace reliably.
- It is evidence, not ground truth.
- It is sampled because cost matters.

Addresses `D11` and `D13`; implements `P10` and `P12`.

### Summary Regeneration

Actor: optimizer plus deterministic contract validator.

Inputs:

- node revision.
- raw evidence.
- prior summary.
- stale markers.
- contract version.

Outputs:

- new `SummaryContract`.
- provenance pointers.
- stale or invalid markers.

Limitations:

- Missing evidence yields invalid summary, not confident prose.
- Contradictory evidence yields uncertainty or conflict.
- Regeneration cannot change node identity.

Addresses `D2`, `D4`, `D10`; implements `P3`, `P4`, `P7`.

### Fact Extraction

Actor: optimizer or reviewer-assisted extraction under deterministic schema validation.

Inputs:

- evidence artifacts.
- target schema for decisions, blockers, status, tool results, questions, and provenance claims.

Outputs:

- structured claims in summaries, edges, or node revisions.
- confidence and derivation type.

Limitations:

- Extracted facts are derived claims, not raw evidence.
- Inferred claims must be labeled.
- Lower-privilege content cannot become instruction.

Addresses `D10` and `D12`; implements `P7`, `P10`.

## Constraints and What Is Lost

- Single user, single tab, one orchestrator. This excludes team workflows, access-control matrices, shared dashboards, and multiple concurrent lead agents. The loss is collaboration breadth; the gain is coherent graph ownership and simpler recovery. Addresses `D14`; implements `P16`.
- Local graph state is canonical. Vendor sessions remain evidence, not source of truth. The loss is that the harness cannot rely on vendor-native session state as complete truth; the gain is exportable long-term memory. Addresses `D15` and `D16`; implements `P15`.
- Snapshot-walk-then-merge accepts delayed optimizer visibility. The loss is that a helpful optimizer edit may wait until the next turn; the gain is deterministic foreground reasoning. Addresses `D3`; implements `P6`.
- Summary contracts may mark nodes invalid instead of useful. The loss is less fluent continuity when evidence is poor; the gain is no confident drift. Addresses `D2`; implements `P3`.
- Cross-CLI capability differences are exposed. The loss is a less uniform mental model; the gain is honest operational semantics. Addresses `D7`; implements `P8`.
- `/compact` is treated as corruption, not a fallback. The loss is compatibility with native compaction workflows; the gain is one coherent memory system. Addresses `D15` and `D16`; implements `P9`.
- Reviewer coverage is sampled. The loss is incomplete LLM review coverage; the gain is bounded cost and reliance on deterministic gates. Addresses `D11` and `D13`; implements `P10` and `P12`.

## Non-Goals

- General-purpose agent framework. The design is a personal orchestrator harness, not a library for arbitrary multi-agent systems. This follows `P16` and the anti-goals.
- Replacing `agent-runner`. The harness depends on `agents` for invocation, balancing, resume, and session ingestion. This follows `P15`.
- Multi-tenant SaaS or web collaboration. This is excluded by `P16`.
- Sidebar-of-many-chats UI. Multiple initiatives are graph roots under one orchestrator, not separate chat sessions. This follows `P13` and `P16`.
- `/compact` interoperability. Native compaction conflicts with graph provenance and summary contracts. This follows `P9`.
- Treating AI reviewers as policy engines. Reviewer output is evidence only. This follows `P10`.
- Infinite memory. Packing depth does not mean unlimited retention. Explicit deletion, archive, and render exclusion remain possible under policy. This follows `P2`, `P7`, `P12`, and `P15`.
- AI-owned product strategy. The orchestrator may organize work, but it does not own product direction. This follows the philosophy anti-goals.

## Philosophy Gaps and Tensions

No difficulty in `problem.md` appears ungrounded by the current philosophy. The main tensions are already named in [philosophy.md](philosophy.md), and this design resolves them as follows:

- `P1` versus `P5`: optimizer imposition is limited to graph curation and render shape; agent-owned focus is expressed through walk-state tools.
- `P6` versus `P12`: snapshot-walk-then-merge is the weakest coordination semantic that still gives stable per-turn views.
- `P10` versus `P12`: reviewer use is sampled and anomaly-triggered, with deterministic gates carrying the hard guarantees.
- `P2` versus `P3`: bounded renders rely on contract-valid summaries and explicit invalid states when detail cannot be faithfully compressed.
- `P8` versus `P16`: per-CLI asymmetry is represented through concise capability fingerprints rather than exposing every mechanism in the main UI.
- `P13` versus `P14`: recovery is visible, but only consequential recovery interrupts the user.

## Alignment Matrix

| Difficulty | Primary design mechanisms | Principles |
|---|---|---|
| `D1` effective working set | `WorkingSetSnapshot`, reasoning budget, required pins, eviction order | `P1`, `P2`, `P12` |
| `D2` summary contract | `SummaryContract`, provenance pointers, invalid states | `P3`, `P7` |
| `D3` concurrent mutation | snapshot-walk-then-merge, bounded `GraphAction`, advisory `OptimizerRequest`, `OptimizerEdit`, `ConflictRecord` | `P1`, `P5`, `P6`, `P14` |
| `D4` stable identity | stable IDs, `IdentityEvent`, forwarding map, optimizer-owned topology edits | `P1`, `P4`, `P5`, `P15` |
| `D5` working-set policy | `AgentWalkState`, pack/unpack/focus tools, bounded foreground actions, eviction | `P2`, `P5` |
| `D6` hierarchical packing | recursive unpack bounds, containment edges, repack edits | `P2`, `P4`, `P6` |
| `D7` cross-CLI asymmetry | `CapabilityFingerprint`, CLI-specific render paths | `P8`, `P13` |
| `D8` sub-agent supervision | `WorkerSlice`, worker reintegration states, advisory `OptimizerRequest`, overlap policy | `P5`, `P7`, `P8`, `P10` |
| `D9` user-question routing | `QuestionArtifact`, child acceptance, resume state machine | `P11`, `P14` |
| `D10` tool-call provenance | `ToolCallProvenance`, evidence artifacts, protocol validation | `P7`, `P10`, `P14` |
| `D11` reviewer reliability | deterministic gates plus sampled reviewer evidence | `P10`, `P12` |
| `D12` graph poisoning | privilege labels, quarantine, provenance, policy validation | `P1`, `P7`, `P10`, `P15` |
| `D13` cost/resource tails | `BudgetLedger`, budget gates, cache prefix hashes | `P12` |
| `D14` multi-workstream legibility | single-tab structured panes, status states, notification classes | `P13`, `P16` |
| `D15` imposed context precedent gap | imposed render contract, optimizer-owned curation, bounded foreground actions, local graph source of truth, no `/compact` | `P1`, `P5`, `P9`, `P15` |
| `D16` recovery surfaces | `RecoveryAction`, explicit preserved/replayed/discarded records | `P14`, `P15` |

## Quality Checks

- Every design decision is tied to one or more `D` references in section text or the alignment matrix.
- Every design decision is tied to one or more `P` references in section text or the alignment matrix.
- Constraints are explicit and include what is lost.
- Non-goals include rationale for exclusion.
- The proposal contains no phasing, MVP scope, measurement targets, deployment timelines, or cutover criteria.
- The proposal references the problem and philosophy instead of restating them.
- Schema objects are named with fields, relationships, and identity semantics.
- Workflows define states and transitions.
- AI/ML sections state actors, inputs, outputs, and limitations.
- Governance mechanisms are specific: deterministic gates, policy versions, audit events, reviewer sampling, privilege controls, and recovery records.
