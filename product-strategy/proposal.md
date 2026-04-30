# Agent Harness Proposal

## Scope

This proposal defines the `agent-harness` system: a single-user Tauri v2 desktop application that keeps one long-lived orchestrator productive across many concurrent initiatives by imposing a bounded graph-derived working set on each orchestrator turn.

Reference notation:

- `D1` through `D18` refer to the numbered difficulties in [problem.md](problem.md).
- `P1` through `P16` refer to the numbered principles in [philosophy.md](philosophy.md).

This document is not a roadmap. It does not define build order, MVP scope, deployment timing, or measurement targets. It describes the system as designed.

## Fixed Substrate

The following constraints are givens, not design choices:

- `agent-runner` is already installed as `/home/nes/.local/bin/agents`. The harness uses it for provider load balancing, invocation, resume, cross-provider session porting, session-id capture, session ingestion, per-CLI session storage location knowledge, and quota state. The harness does not replace, fork, or duplicate it. This follows `P15` and the anti-goal against replacing `agent-runner`, while addressing cross-session trace and resume boundaries in `D7`, `D8`, `D10`, and `D16`.
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

### Configuration as Memory Semantics

Graph and memory configuration is treated as part of the memory contract, not setup preference. Schema choices, summary contract templates, optimizer cadence and scope, render budget caps, capability fingerprints, provider routing, memory-policy defaults, index scopes, and default unpack policies all change what the agent can see and what failures mean. They are recorded as versioned configuration state and cited by renders, optimizer edits, and audit events. This addresses `D17`; it implements `P1`, `P3`, `P7`, `P13`, and `P15`.

The harness ships with conservative defaults that produce a usable graph without user configuration, but defaults are never invisible. Every render and inspection surface can distinguish configured values from defaulted values, inherited values, and system-required values. The user can ask two pre-operational questions before trusting the graph: "what would happen on an empty graph?" and "why is the graph in this shape?" Both answers are computed from the same policies used by live renders and optimizer edits, not from explanatory prose detached from execution.

When the agent makes a bad decision, failure attribution must include the substrate. The harness records enough state to distinguish agent reasoning failure from an empty graph, badly-shaped graph, incomplete index, optimizer mis-curation, wrong provider route, missing entitlement, or field-meaning disagreement. This is a governance requirement, not a help-text feature.

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

The UI exposes a per-session `CapabilityFingerprint` so the user and orchestrator can see when a worker has weaker context injection, weaker tool interception, weaker resume guarantees, missing provider features, missing account entitlements, unavailable local runtimes, or sandbox constraints. The harness does not pretend all CLIs or providers are equivalent.

### SessionOverrideContract Boundary

The harness and `agent-runner` meet at a versioned session-override boundary. The harness owns the domain layer: context graph nodes and edges, summaries, revisions, working set, repack planner, render policy and outputs, orchestrator turn state, evidence, provenance, audit, optimizer queue, recovery, question routing, budget ledger, worker dispatch state, policy, and configuration. `agent-runner` owns the execution substrate: provider routing, multi-account load balancing, quota tracking, auth refresh delegation, `--resume` mechanics, cross-provider session porting, per-CLI session storage location knowledge, and session-id generation or capture.

The seam is the Rust `SessionOverrideContract`. It is the only harness write-back path that may replace, truncate, or extend an `agents`-owned session transcript. The repack planner and detail-injection router produce a packed transcript; the session-override implementation locates the session, probes supported schema/storage shape, writes through the selected adapter, and emits audit/provenance records. This addresses `D7`, `D10`, `D15`, and `D16`; it implements `P8`, `P14`, and `P15`.

The contract operations are:

- `schema_version_probe()`: identify the `agent-runner` binary and state schema surface before any write. Unsupported or unknown schema returns a refusal, not a best-effort mutation.
- `locate_session(session_ref)`: resolve a harness session reference to provider name, storage kind, raw transcript path if available, active chain/segment metadata when known, and transcript mutability state.
- `read_transcript(session_ref)`: return canonical transcript records plus source offsets and content hashes for audit.
- `replace_transcript(session_ref, packed_transcript)`: atomically replace the transcript with a harness-rendered packed transcript.
- `truncate_after(session_ref, turn_ref)`: remove later transcript turns after deterministic preflight.
- `append_turns(session_ref, turns)`: append canonical turns while preserving provider-specific record shape through the adapter.
- `get_session_metadata(session_ref)`: return provider, chain, segment, capture method, transcript state, compaction-boundary state, resume acceptance state, and last observed turn.

The adapter stack has one harness trait, `SessionOverrideStore`, and two implementations:

- `AgentRunnerDbAdapter` is v1. It reads `~/.local/share/oulipoly-agent-runner/state.db`, locates active `session_chains`, `session_chain_segments`, `session_turns`, `invocations`, configured session storage, and transcript locator output, then writes only known per-CLI JSONL files plus the minimal `agent-runner` state rows required for consistency. It is schema-version-pinned to a tested `agents` binary range and refuses writes outside that range.
- `AgentRunnerCliAdapter` is v2. It calls future `agents session locate`, `agents session export`, `agents session import-replace`, and pause/lock commands. It implements the same harness-side trait, so call sites do not change when the upstream CLI surface lands.

Atomicity is required because every override spans SQLite state and a JSONL file. The write protocol is two-phase: acquire a session-idle lock, probe schema, read current hashes, write a replacement JSONL to a same-directory temp file, write a pending override row or audit marker in the harness store, rename the JSONL atomically, update `agent-runner` state rows in one SQLite transaction when needed, then mark the override committed. On startup, crash recovery resolves pending overrides by comparing preimage hash, temp file, final file, and state row: complete the commit when both sides agree, roll back the temp file when no state change occurred, or quarantine the session when the two sides disagree.

Race handling fails closed. v1 does not support concurrent override during an in-flight `agents` session write. It must observe session-idle through filesystem locks, SQLite busy/transaction probes, and provider transcript mtime stability before writing. The intended stronger surface is a future `agents` pause-handshake or mid-session lock; until that exists, the harness waits for idle or blocks the override.

### Provider State as Observable State

Provider availability is first-class local state. Auth presence, account identity, billing/quota health, model entitlement, feature support, local runtime availability, network reachability, sandbox boundaries, and provider-specific restrictions are represented as inspectable records before work is routed. This addresses `D18`; it implements `P8`, `P12`, `P13`, `P14`, and `P15`.

The harness does not own vendor auth stores such as `~/.claude/`, `~/.codex/`, `~/.config/opencode/`, or local runtime registries. It reads and audits them through documented or observable local boundaries, stores only derived `ProviderState` and entitlement snapshots, redacts secrets, and records the freshness and confidence of those observations. Local control means the harness controls its graph and provenance; it does not require taking control of every provider's credential store.

Provider routing is denied, degraded, or revalidated when state is not sufficient for the workload. A worker cannot be sent to a provider that lacks a required feature, is unauthenticated, is locked by quota or billing, cannot reach the network path the task needs, lacks the local runtime, or is constrained by a sandbox in a way that would make the task misleading. Mid-run provider failure creates a `RecoveryAction` with explicit provider cause, preserved trace, replayability classification, and possible reroute targets.

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
- Talks to `agents` as a subprocess boundary and reads or mutates `agent-runner` session state only through `SessionOverrideContract` and documented local files or SQLite access.
- Addresses `D7`, `D10`, `D13`, `D16`; implements `P8`, `P12`, `P15`.

`GraphStore`

- Local SQLite database plus filesystem-backed evidence blobs.
- Stores graph objects, revisions, evidence, renders, worker slices, questions, budgets, configuration records, provider-state snapshots, recovery actions, and audit events.
- The harness graph is canonical. CLI transcripts and `agent-runner` state are evidence sources, not canonical graph state.
- Addresses `D4`, `D10`, `D12`, `D15`, `D16`, `D17`, and `D18`; implements `P4`, `P7`, `P15`.

`RenderEngine`

- Converts a graph snapshot into CLI-specific prompt material and tool affordances.
- Applies working-set policy, summary contract validation, configuration defaults, privilege labels, cost constraints, provider state, and capability fingerprints.
- Addresses `D1`, `D5`, `D7`, `D15`, `D17`, and `D18`; implements `P1`, `P2`, `P3`, `P8`, `P12`.

`OrchestratorBridge`

- Runs and resumes the `claude-opus` or `claude-sonnet` orchestrator through `agents`.
- Injects rendered context, exposes pack/unpack/focus and graph query tools, captures tool calls and model outputs, writes turn records, and emits bounded foreground graph actions.
- Foreground graph actions are append-only records or advisory optimizer requests. They never create, revise, delete, split, merge, re-parent, repack, summarize, cross-reference, or otherwise curate graph topology.
- Addresses `D7`, `D10`, `D16`; implements `P5`, `P8`, `P9`, `P14`.

`WorkerDispatcher`

- Creates `WorkerSlice` records, selects configured model names, starts sessions through `agents`, tracks acceptance, and captures session evidence through `agent-runner` ingestion.
- Preflights target provider state and capability fingerprints before launch, and records explicit denial reasons when routing cannot satisfy the workload.
- Delegates provider routing, account choice, quota state, resume mechanics, session porting, and per-CLI storage lookup to `agent-runner`. Worker launcher work units are therefore thin process/state orchestration over `agents`, not per-CLI transcript machinery.
- Does not directly merge worker output into graph truth. It stages output for reintegration.
- Addresses `D8`, `D9`, `D13`, and `D18`; implements `P8`, `P10`, `P11`, `P12`, and `P13`.

`SessionOverrideStore`

- Harness-side trait that implements `SessionOverrideContract` for session transcript read and write-back.
- Has `AgentRunnerDbAdapter` for v1 direct state.db plus JSONL mutation and `AgentRunnerCliAdapter` for the future `agents session` command surface.
- Exists only to apply harness-owned packed transcripts to `agents`-owned session storage. It does not route providers, refresh auth, balance accounts, mint session IDs, or port sessions across providers.
- Addresses `D7`, `D10`, `D15`, and `D16`; implements `P8`, `P14`, and `P15`.

`Optimizer`

- Background `glm` actor that proposes summary regenerations, stale markers, cross-references, topology edits, provenance repair, and repacking.
- Writes `OptimizerEdit` drafts against a base snapshot. The backend validates and merges them.
- May consume `OptimizerRequest` artifacts from the orchestrator, workers, backend, or user surface, but evaluates them as advisory inputs rather than delegated instructions.
- Receives configuration records as explicit inputs so summary templates, schema fields, optimizer cadence, and memory-policy defaults are not hidden prompt assumptions.
- Addresses `D2`, `D3`, `D4`, `D6`, `D12`, `D13`, `D15`, and `D17`; implements `P3`, `P4`, `P6`, `P7`, `P12`.

`WorkflowReviewer`

- LLM reviewer, usually `gpt-high`, used as an evidence channel over selected optimizer edits, worker reintegration plans, and anomalous traces.
- Its output never becomes authoritative by itself.
- Addresses `D11` and implements `P10` and `P12`.

`PolicyEngine`

- Deterministic validator for schema constraints, privilege boundaries, tool-call protocol integrity, render limits, identity invariants, and budget gates.
- Owns policy versioning and attaches `policy_version` to decisions.
- Validates configuration provenance and provider readiness before renders, optimizer edits, worker launches, and recovery actions.
- Addresses `D10`, `D11`, `D12`, `D13`, `D16`, `D17`, and `D18`; implements `P10`, `P12`, `P14`, `P15`.

`ConfigurationRegistry`

- Owns versioned graph and memory configuration: graph schema fields, summary contract templates, optimizer cadence and scope, render budget caps, memory-policy defaults, index scopes, provider routing defaults, and explanation labels for configured fields.
- Computes defaulted configuration for an empty graph and exposes configuration provenance for every effective value.
- Addresses `D17`; implements `P1`, `P3`, `P7`, `P13`, and `P15`.

`ProviderStateMonitor`

- Reads provider state from `agent-runner`, CLI capability probes, documented local files, environment boundaries, and local runtime probes without taking ownership of vendor credential stores.
- Produces redacted `ProviderState` and `EntitlementSnapshot` records used by routing, capability fingerprints, budget policy, user surface badges, and recovery.
- Addresses `D18`; implements `P8`, `P12`, `P13`, `P14`, and `P15`.

`UserSurface`

- Single-tab UI with structured panes for initiatives, current focus, working set, configuration provenance, provider state, questions, workers, optimizer edits, cost, recovery, and evidence drill-down.
- It is not a sidebar of separate chats.
- Addresses `D14`, `D17`, and `D18`; implements `P1`, `P13`, and `P16`.

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
- `active_configuration_id`: active graph and memory configuration.

Relationships:

- Owns `GraphNode`, `GraphEdge`, `GraphSnapshot`, `GraphConfiguration`, `ProviderState`, `AuditEvent`, and `BudgetLedger`.
- Has exactly one active orchestrator, matching `P16`.

Addresses `D15`, `D16`, `D17`, and `D18`; implements `P15` and `P16`.

### GraphConfiguration

Purpose: versioned configuration that defines how graph memory becomes usable working context.

Fields:

- `configuration_id`.
- `workspace_id`.
- `configuration_version`.
- `schema_profile`: enabled node kinds, edge kinds, required fields, and field meaning descriptions.
- `summary_contract_template_ids`: templates by node kind or initiative type.
- `optimizer_policy_ref`: cadence, scope limits, edit-type allowlist, stale thresholds, and review sampling hooks.
- `render_policy_ref`: node, evidence, depth, priority, and token caps.
- `memory_policy_ref`: promotion defaults, archival rules, index scopes, default unpack policy, and stale-index handling.
- `provider_routing_policy_ref`: preferred providers, hard exclusions, fallback ordering, and feature requirements.
- `capability_fingerprint_policy_ref`: which provider and CLI dimensions must be probed.
- `effective_value_sources`: per-field source map of `system_required`, `default`, `inherited`, `user_configured`, or `recovered`.
- `created_from_configuration_id`: prior configuration if this is a revision.
- `validation_state`: `valid`, `valid_with_warnings`, `invalid_schema`, `invalid_provider_route`, `invalid_budget`, `invalid_index`, `needs_user_attention`.

Relationships:

- Referenced by `GraphSnapshot`, `WorkingSetSnapshot`, `OptimizerEdit`, `PolicySet`, `AuditEvent`, and UI inspection records.
- Changes through append-only configuration revisions; existing snapshots keep their original configuration.

Inspection semantics:

- Empty-graph inspection runs render, optimizer scoping, indexing, and provider-routing preflight against this configuration without creating graph truth.
- Shape explanation traces any node kind, field, edge, summary template, index, or routing decision back to its effective value source and policy version.

Addresses `D17` and `D18`; implements `P1`, `P3`, `P7`, `P13`, and `P15`.

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
- `configuration_id`.
- `sealed_at`.
- `created_for`: `orchestrator_turn`, `worker_slice`, `optimizer_edit`, `recovery_preflight`, `ui_inspection`.
- `base_transaction_id`.

Relationships:

- Referenced by `WorkingSetSnapshot`, `OptimizerEdit`, `WorkerSlice`, `ConflictRecord`, configuration inspections, and `RecoveryAction`.
- Never changes after creation.

Identity semantics:

- A snapshot resolves IDs according to its own identity map. Later forwarding does not rewrite what a prior snapshot meant.

Addresses `D3`, `D4`, `D5`, `D16`, and `D17`; implements `P4`, `P6`, `P14`.

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
- `template_source`: `system_default`, `workspace_default`, `node_kind_default`, `user_configured`, `recovered`.
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
- Its template source is part of failure attribution when summaries omit fields or expose disputed field meanings.

Addresses `D2`, `D12`, `D15`, and `D17`; implements `P3`, `P7`.

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
- `configuration_id`.
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
- `provider_state_id`: provider state used for the route, if applicable.
- `configuration_explanation_ref`: compact provenance of configured/defaulted values that affected this render.
- `rendered_blob_ref`.

Relationships:

- Referenced by every orchestrator turn, worker launch, and question artifact.
- Provides the answer to "what did the model see?"
- Provides the answer to "which configuration and provider state caused this render to look this way?"

Addresses `D1`, `D5`, `D7`, `D15`, `D17`, and `D18`; implements `P1`, `P2`, `P8`, `P12`, and `P13`.

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
- `required_provider_features`: model features, tool support, network needs, local runtime needs, and sandbox assumptions required by the workload.
- `provider_state_id`: preflight state used to approve or deny launch.
- `state`: `draft`, `validated`, `launched`, `accepted`, `running`, `needs_input`, `completed`, `failed`, `cancelled`, `reintegrating`, `integrated`, `conflicted`.

Relationships:

- Owned by `WorkerRun`.
- May produce `QuestionArtifact`, `WorkerOutput`, `OptimizerRequest`, and `ConflictRecord`.

Addresses `D8`, `D9`, `D13`, and `D18`; implements `P8`, `P11`, `P12`, and `P13`.

### WorkerRun

Purpose: provider-level execution record for a worker.

Fields:

- `worker_id`.
- `slice_id`.
- `agent_runner_invocation_id`.
- `session_id`.
- `provider_state_id`.
- `resume_supported`: boolean from capability fingerprint and observed acceptance.
- `acceptance_state`: `unknown`, `accepted`, `rejected`, `timed_out`, `ambiguous`.
- `last_ingested_message_id`.
- `state`: `starting`, `running`, `blocked`, `completed`, `failed`, `lost`, `cancelled`.
- `cost_ledger_id`.

Relationships:

- Links harness slice state to `agent-runner` invocation/session state.

Addresses `D7`, `D8`, `D16`, and `D18`; implements `P8`, `P11`, `P14`, and `P15`.

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
- `request_type`: `consider_summary_refresh`, `consider_stale_mark`, `consider_cross_reference`, `consider_repack`, `consider_split`, `consider_merge`, `consider_reparent`, `consider_provenance_repair`, `consider_quarantine`, `consider_configuration_warning`, `consider_provider_route_warning`.
- `rationale_ref`: evidence, turn output, worker output, or audit note that motivated the request.
- `priority_hint`: `low`, `normal`, `high`, `urgent`.
- `advisory_state`: `queued`, `accepted_for_scoping`, `ignored`, `superseded`, `converted_to_optimizer_edit`.
- `converted_optimizer_edit_id`: null unless the optimizer independently drafts an edit from the request.

Relationships:

- May be produced by a foreground `GraphAction`, worker reintegration staging, recovery, or UI action.
- Is input to the optimizer queue, not a graph mutation.
- Does not authorize the source actor to decide that nodes, revisions, edges, summaries, or topology changes should exist.
- If the optimizer acts on the request, it creates a separate `OptimizerEdit` with its own scope, evidence, validation, reviewer state, audit trail, and actor attribution.

Addresses `D3`, `D4`, `D5`, `D6`, `D15`, `D17`, and `D18`; implements `P1`, `P4`, `P5`, `P6`, and `P13`.

### OptimizerEdit

Purpose: proposed background graph mutation.

Fields:

- `optimizer_edit_id`.
- `base_graph_snapshot_id`.
- `configuration_id`.
- `actor_model`: normally `glm`.
- `edit_type`: `summary_regeneration`, `stale_mark`, `cross_reference`, `repack`, `split`, `merge`, `reparent`, `provenance_repair`, `poison_quarantine`.
- `touched_node_ids`.
- `operation_payload_ref`.
- `evidence_ids`.
- `configuration_refs`: configuration values that constrained or motivated the edit.
- `expected_invariants`.
- `deterministic_validation_state`: `pending`, `passed`, `failed`.
- `reviewer_state`: `not_required`, `sampled_pending`, `passed`, `flagged`, `inconclusive`.
- `merge_state`: `draft`, `validated`, `merged`, `conflicted`, `rejected`, `reverted`.
- `result_graph_version`.

Relationships:

- Produces `IdentityEvent`, `NodeRevision`, `GraphEdge`, and `AuditEvent` only after merge.
- May cite `OptimizerRequest` as an input, but the edit remains an optimizer-owned curation decision and is shown in the user surface as such.

Addresses `D3`, `D4`, `D6`, `D11`, `D12`, `D13`, and `D17`; implements `P1`, `P4`, `P5`, `P6`, `P10`, `P12`, `P14`.

### ConflictRecord

Purpose: explicit record when optimistic merge cannot safely apply.

Fields:

- `conflict_id`.
- `base_graph_snapshot_id`.
- `current_graph_snapshot_id`.
- `actor_a`, `actor_b`.
- `conflict_type`: `identity`, `content`, `summary_contract`, `configuration`, `edge`, `worker_overlap`, `question_route`, `tool_protocol`, `provider_state`, `budget`.
- `affected_node_ids`.
- `resolution_state`: `open`, `auto_resolved`, `needs_orchestrator`, `needs_user`, `rejected`, `superseded`.
- `resolution_edit_id`.

Relationships:

- Blocks merge or reintegration until resolved.

Addresses `D3`, `D4`, `D8`, `D16`, `D17`, and `D18`; implements `P6`, `P14`.

### ProviderState

Purpose: redacted, observable state of a provider/CLI/account/runtime route.

Fields:

- `provider_state_id`.
- `provider`: `anthropic`, `openai`, `google`, `local_runtime`, `openai_compatible`, `other`.
- `cli`: `claude`, `codex`, `opencode`, or `agent_runner`.
- `account_ref`: redacted account or profile identifier.
- `auth_state`: `present`, `missing`, `expired`, `invalid`, `unknown`.
- `billing_state`: `healthy`, `near_limit`, `over_limit`, `payment_required`, `unknown`.
- `quota_state`: `available`, `rate_limited`, `exhausted`, `unknown`.
- `network_state`: `available`, `blocked_by_sandbox`, `blocked_by_host`, `degraded`, `unknown`.
- `runtime_state`: `installed`, `missing`, `wrong_version`, `unreachable`, `not_applicable`, `unknown`.
- `sandbox_constraints`: declared or detected network, filesystem, process, or tool limits.
- `store_locations_checked`: redacted paths or sources checked, such as vendor config directories, `agent-runner` state, environment, and runtime registries.
- `secret_material_stored`: always false for harness-owned records.
- `freshness`: `fresh`, `stale`, `probe_failed`, `manual`.
- `confidence`: `high`, `medium`, `low`.
- `last_probe_at`.

Relationships:

- Has many `EntitlementSnapshot`.
- Referenced by `CapabilityFingerprint`, `WorkerSlice`, `WorkerRun`, `WorkingSetSnapshot`, `BudgetLedger`, `RecoveryAction`, and `AuditEvent`.
- Produced by `ProviderStateMonitor`, not by the orchestrator.

Addresses `D18`; implements `P8`, `P13`, `P14`, and `P15`.

### EntitlementSnapshot

Purpose: feature and model availability observed for a provider/account/runtime at a point in time.

Fields:

- `entitlement_snapshot_id`.
- `provider_state_id`.
- `model_id`.
- `feature_matrix`: context length class, tool calling, MCP support, file/image support, reasoning controls, structured output, streaming, resume, approval, and local runtime features.
- `entitlement_state`: `available`, `not_entitled`, `billing_locked`, `region_locked`, `disabled_by_policy`, `unknown`.
- `requires_network`: boolean.
- `requires_local_runtime`: boolean.
- `observed_limit_ref`: quota, rate limit, credit, or plan note if available.
- `probe_method`: `agent_runner`, `cli_probe`, `local_file_audit`, `dry_run`, `manual`, `provider_error`.
- `evidence_id`: redacted evidence for the observation.
- `valid_from`, `valid_to`.

Relationships:

- Feeds `CapabilityFingerprint`, provider routing policy, worker launch preflight, and recovery reroute decisions.
- Does not store tokens or credential material.

Addresses `D18`; implements `P8`, `P12`, `P13`, and `P15`.

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
- `provider_state_id`.
- `entitlement_snapshot_ids`.
- `account_state`: auth, billing, quota, and account-profile readiness summarized from provider state.
- `feature_support`: workload-relevant features that are supported, unsupported, or unknown.
- `runtime_support`: local runtime availability and version class when relevant.
- `sandbox_support`: network, filesystem, process, and tool constraints relevant to the session.
- `route_state`: `eligible`, `eligible_with_warnings`, `degraded`, `blocked`, `unknown`.
- `route_denial_reasons`: explicit reasons a workload cannot be routed to this provider or CLI.
- `known_asymmetries`.
- `observed_failures`.

Relationships:

- Attached to renders, worker runs, provider state, and UI session badges.

Addresses `D7`, `D9`, `D10`, and `D18`; implements `P8`, `P11`, `P13`, and `P15`.

### BudgetLedger

Purpose: cost and latency control record.

Fields:

- `budget_ledger_id`.
- `scope_type`: `workspace`, `initiative`, `orchestrator_turn`, `worker_run`, `optimizer_pass`, `reviewer_pass`, `render`.
- `scope_id`.
- `input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`.
- `latency_ms`.
- `provider_cost_estimate`.
- `provider_state_id`: optional provider state used for cost/quota interpretation.
- `cache_prefix_hash`.
- `budget_state`: `within`, `near_limit`, `exceeded`, `blocked`.
- `policy_action`: `none`, `warn`, `narrow_scope`, `require_user_approval`, `block`.

Relationships:

- Consulted by renderer, optimizer, worker dispatcher, and reviewer sampler.

Addresses `D13` and `D18`; implements `P12`.

### RecoveryAction

Purpose: auditable operation that changes state during resume, rollback, revert, or cancellation.

Fields:

- `recovery_action_id`.
- `action_type`: `resume_session`, `retry_recording`, `retry_execution`, `rollback_graph`, `revert_optimizer_edit`, `cancel_worker`, `restore_question`, `quarantine_session`, `fresh_worker_substitution`.
- `precondition_ref`.
- `affected_session_ids`.
- `affected_node_ids`.
- `affected_edit_ids`.
- `affected_provider_state_ids`.
- `provider_failure_cause`: `none`, `auth`, `billing`, `quota`, `entitlement`, `network`, `sandbox`, `runtime`, `feature_missing`, `unknown`.
- `reroute_candidate_refs`: alternate capability fingerprints or provider states if substitution is possible.
- `side_effect_classification`: `deferred_recording`, `deferred_execution`, `unknown`.
- `user_confirmation_state`: `not_required`, `required`, `granted`, `denied`.
- `result_state`: `planned`, `applied`, `failed`, `partially_applied`, `reverted`.
- `preserved_ref`, `replayed_ref`, `discarded_ref`.

Relationships:

- Produces audit events and may produce conflict records.

Addresses `D16` and `D18`; implements `P14`, `P15`.

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
- `configuration_policy_version`.
- `provider_policy_version`.

Relationships:

- Referenced by all decisions and audit events.
- Allows later explanation of why a decision was accepted at the time.

Addresses `D11`, `D12`, `D13`, `D16`, `D17`, and `D18`; implements `P10`, `P12`, `P14`.

### AuditEvent

Purpose: append-only local record of decisions and state changes.

Fields:

- `audit_event_id`.
- `event_type`.
- `actor`.
- `policy_set_id`.
- `configuration_id`: optional effective configuration involved in the decision.
- `provider_state_id`: optional provider state involved in the decision.
- `input_refs`.
- `output_refs`.
- `decision`: `accepted`, `rejected`, `deferred`, `quarantined`, `user_required`.
- `reason_code`.
- `created_at`.

Relationships:

- Every graph mutation, render, configuration inspection, provider probe, routing denial, optimizer edit, reviewer decision, question route, and recovery action emits audit events.

Addresses `D11`, `D12`, `D14`, `D16`, `D17`, and `D18`; implements `P1`, `P10`, `P13`, `P14`, `P15`.

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

Configuration participates in working-set policy. The renderer records which budget caps, recursive-depth defaults, node-kind allowlists, summary templates, index scopes, provider routing defaults, and memory promotion rules affected `W`. If an empty or thin render is caused by configuration rather than graph content, the working-set inspector labels that cause directly. This addresses `D17` and `D18`; it implements `P1`, `P2`, `P8`, and `P13`.

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
- `snapshotting -> rendering` after identity forwarding, configuration resolution, provider-state preflight, and policy validation.
- `rendering -> launching_or_resuming` if the render is within budget and the target provider route is eligible.
- `rendering -> blocked` if required context exceeds budget, provider state is insufficient, configuration is invalid for the render, or graph state has unreconciled corruption.
- `launching_or_resuming -> thinking` when the CLI accepts the session.
- `thinking -> tool_pending` when the model calls a tool.
- `tool_pending -> capturing` when the result is available or fails.
- `thinking -> capturing` on final model output.
- `capturing -> committing` after evidence and protocol records are durable.
- `committing -> optimizer_enqueue` after bounded `GraphAction` records pass policy and any `OptimizerRequest` artifacts are queued as advisory input.
- `optimizer_enqueue -> complete` after queues and audit events are written.
- Any state -> `recovering` on session loss, protocol corruption, provider lockout, entitlement change, sandbox denial, runtime loss, or storage conflict.

`GraphAction` in this lifecycle is not a topology mutation path. The allowed set is limited to recording orchestrator output, recording tool provenance, attaching audit notes, emitting user-facing output, and creating advisory `OptimizerRequest` artifacts. It cannot create or change `GraphNode`, `GraphEdge`, `NodeRevision`, `SummaryContract`, or `IdentityEvent`; it cannot summarize, cross-reference, repack, split, merge, re-parent, forward identity, repair provenance, or quarantine graph truth.

If the orchestrator believes one of those curation operations is needed, it emits an `OptimizerRequest`; the optimizer later decides whether to draft an `OptimizerEdit`, and any resulting user-visible topology or summary change is attributed to the optimizer.

This lifecycle addresses `D1`, `D3`, `D4`, `D5`, `D7`, `D10`, `D13`, `D16`, `D17`, and `D18`; it implements `P1`, `P2`, `P4`, `P5`, `P6`, `P8`, `P12`, and `P14`.

### Configuration Inspection and Validation

States:

- `resolved`: effective configuration is assembled from system-required values, defaults, inherited values, user-configured values, and recovered values.
- `empty_graph_simulated`: renderer and optimizer scoping run against an empty graph with this configuration.
- `shape_explained`: existing topology, summary templates, indexes, provider routes, and memory policies are traced to the configuration values that produced them.
- `validated`: deterministic checks accept the configuration for render, optimizer, indexing, routing, and recovery use.
- `warning`: configuration is usable but may create ambiguous memory semantics.
- `invalid`: configuration would produce unusable graph state, invalid summaries, incomplete indexing, impossible provider routing, or over-budget renders.
- `superseded`: a newer configuration revision replaced it.

The inspection output is operational, not tutorial text. "What would happen on an empty graph?" shows initial roots, default node kinds, summary templates, render caps, provider routes, optimizer scope, and indexes that would exist before any user data. "Why is the graph in this shape?" traces each node kind, required field, containment rule, cross-reference, stale marker, index, and optimizer edit back to configuration, evidence, or explicit user action.

Configuration warnings create `AuditEvent` records and may create advisory `OptimizerRequest` records when existing graph shape appears inconsistent with current configuration. They do not silently rewrite graph truth. This addresses `D17`; it implements `P1`, `P3`, `P7`, `P13`, and `P15`.

### Provider Preflight and Routing

States:

- `unprobed`: provider state is not current enough for routing.
- `probing`: the monitor checks `agent-runner`, CLI probes, local files, environment, runtime availability, network boundaries, and sandbox constraints.
- `ready`: required auth, entitlement, feature, runtime, and network state is available.
- `degraded`: the route can run only with weaker context, weaker tool support, higher cost risk, or missing optional features.
- `blocked`: the route lacks a required auth, entitlement, feature, runtime, network path, quota, or sandbox permission.
- `running`: a session has been launched using this state.
- `failed_mid_run`: provider state changed or was discovered wrong during execution.
- `revalidating`: provider state is refreshed before retry, resume, or reroute.

Routing decisions are workload-specific. A provider can be ready for a text-only review and blocked for a task requiring MCP, network, file patches, local model runtime, long context, or a specific resume surface. The user surface shows "this work cannot route to provider X because [reason]" using `route_denial_reasons`, not a generic failure.

If provider failure occurs mid-run, the harness preserves the trace captured so far, classifies side effects, revalidates the graph snapshot and worker slice, and opens a `RecoveryAction`. Reroute is allowed only if the alternate provider's capability fingerprint satisfies the original slice requirements or the user accepts a changed execution contract. This addresses `D18` and `D16`; it implements `P8`, `P12`, `P13`, `P14`, and `P15`.

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
- `slice_validated`: policy checks overlap, evidence, capability, provider state, entitlement, runtime, sandbox, and budget.
- `launched`: `agents` invocation created.
- `accepted`: child session acceptance observed.
- `running`: worker is producing trace.
- `needs_input`: worker emitted a valid question artifact.
- `completed`: worker finished without pending question.
- `failed`: worker failed, provider route failed, or session evidence is incomplete.
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

This addresses `D3`, `D8`, `D9`, `D10`, `D11`, `D13`, and `D18`; it implements `P5`, `P6`, `P7`, `P8`, `P10`, `P11`, and `P12`.

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

- `detected`: inconsistency, session loss, provider failure, entitlement lockout, failed resume, corrupt compact, or merge failure is detected.
- `classified`: backend classifies deferred recording versus deferred execution, consequence level, and side-effect class.
- `preflight`: policy checks current graph, session availability, provider state, entitlement snapshot, runtime availability, sandbox constraints, and tool protocol state.
- `requires_user`: high-consequence or unknown-side-effect actions wait for confirmation.
- `applying`: recovery action runs.
- `reconciled`: graph, evidence, and audit records agree.
- `partial`: some state was preserved and some was discarded.
- `failed`: recovery could not complete.

Recovery actions show preserved, replayed, and discarded state. Fresh worker substitution is explicit and visible; it is never reported as a successful resume. If recovery reroutes to another provider, the user sees which original contract changed: model, feature set, tool surface, runtime, cost/quota exposure, or sandbox boundary. This addresses `D16` and `D18`; it implements `P14`, `P13`, and `P15`.

### User Surface

The single-tab UI has structured regions backed by the same graph:

- Initiative map: initiative roots with `active`, `blocked`, `waiting`, `recovering`, `stale`, and `archived` states.
- Current focus: the orchestrator's active focus path, pins, unpacked nodes, and evicted nodes.
- Working set inspector: exact nodes and evidence pointers imposed on the current or last turn.
- Configuration inspector: effective configuration, configured/defaulted value sources, empty-graph simulation, index state, render caps, summary templates, and graph-shape explanations.
- Provider state panel: redacted account state, entitlement snapshots, local runtime state, sandbox/network state, route eligibility, and route denial reasons.
- Question queue: action-needed `QuestionArtifact` records across all initiatives.
- Worker board: active and blocked `WorkerSlice` records with capability fingerprints.
- Optimizer log: advisory requests plus accepted, rejected, conflicted, and reverted optimizer-owned edits.
- Evidence drill-down: summary claim to provenance to raw local evidence.
- Cost surface: budget ledgers by initiative, worker, optimizer pass, reviewer pass, and render.
- Recovery surface: active and historical recovery actions, including provider-caused failures and reroute decisions.

The UI separates action-needed notifications from passive progress events. Configuration warnings and provider route denials are visible state, but only consequential cases interrupt the user. This addresses `D14`, `D15`, `D16`, `D17`, and `D18`; it implements `P1`, `P13`, `P14`, `P15`, and `P16`.

## Governance

### Deterministic Gates

The `PolicyEngine` enforces:

- Summary contract completeness.
- Evidence pointer existence and locator validity.
- Stable identity resolution.
- No unresolved forwarding cycles.
- Configuration schema validity and effective-value provenance.
- Empty-graph simulation validity before a configuration is trusted.
- Foreground `GraphAction` allowed-effect boundary.
- `OptimizerRequest` advisory-only boundary and optimizer attribution.
- Worker write-scope compliance.
- Provider route eligibility, entitlement availability, runtime presence, and sandbox compatibility.
- Session override schema probe, adapter support, session-idle lock, and two-phase commit preconditions.
- Question correlation validity.
- Tool-call protocol completeness.
- Privilege-origin preservation.
- Poison-risk quarantine.
- Render budget and required-pin handling.
- Recovery side-effect classification, including provider-caused failures and reroute contract changes.

These gates address `D3`, `D4`, `D5`, `D10`, `D11`, `D12`, `D13`, `D16`, `D17`, and `D18`; they implement `P1`, `P3`, `P4`, `P5`, `P7`, `P8`, `P10`, `P12`, `P13`, and `P14`.

### Reviewer Sampling

The workflow reviewer is invoked for:

- high-consequence optimizer edits, including splits, merges, re-parenting, and poison quarantine reversals.
- worker reintegration that changes decisions, blockers, or user-visible status.
- anomaly-triggered cases, such as missing evidence, high uncertainty, repeated failed resumes, or protocol mismatches.
- configuration-shape anomalies, such as configured fields with unclear meaning, empty graph renders that hide required roots, or optimizer edits that contradict memory-policy defaults.
- provider anomalies, such as repeated route denials, provider feature mismatches, or recovery that proposes changing the execution contract.
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

This addresses reviewer fallibility in `D11` while supporting `D17` and `D18`; it implements `P10`, `P12`, and `P13`.

### Privilege and Poisoning Controls

Every content-bearing object carries `privilege_origin`. Lower-privilege tool or worker text cannot become higher-privilege instruction text through summary regeneration. Optimizer prompts include privilege labels and must return provenance for any promoted claim. Prompt-like content from tool outputs is rendered as evidence, not instructions. Suspect evidence can be quarantined, causing dependent summaries to become stale or invalid.

This addresses `D12` and implements `P1`, `P7`, `P10`, and `P15`.

### Audit and Reversibility

All graph mutations are append-only at the revision layer. Reverts are new recovery actions that restore prior revisions and mark superseded edges; they do not erase history. The user can inspect the edit, evidence, policy version, reviewer signal, and recovery action that produced current state.

This addresses `D3`, `D4`, `D12`, and `D16`; it implements `P6`, `P14`, and `P15`.

### Configuration and Provider Accountability

Configuration changes are append-only revisions with effective-value provenance. A changed summary template, optimizer cadence, render cap, index scope, memory-promotion rule, or provider route cannot be treated as ambient state; later renders and optimizer edits cite the configuration revision that governed them. Configuration inspection is therefore part of audit and reversibility: the user can compare graph shape before and after a configuration change without relying on the optimizer's explanation alone.

Provider state is audited as observation, not ownership. The harness records which store or probe was checked, when it was checked, what non-secret state was derived, and how confident the result was. It does not copy credentials or require vendor stores to move under harness control. Provider failures, route denials, entitlement gaps, and sandbox conflicts create audit events that are visible beside worker and recovery state.

This addresses `D17` and `D18`; it implements `P8`, `P13`, `P14`, and `P15`.

## Observability

The harness records:

- rendered working sets and cache prefix hashes.
- configuration revisions, effective-value sources, empty-graph simulations, shape explanations, and index states.
- provider states, entitlement snapshots, route eligibility, route denials, probe freshness, and sandbox/runtime constraints.
- session override probes, transcript preimage hashes, packed transcript hashes, adapter version, idle-lock result, and crash-recovery disposition.
- node and evidence counts per render.
- summary contract validation outcomes.
- foreground graph-action validation outcomes and optimizer-request disposition.
- optimizer queue depth, edit types, merge success, conflicts, and reverts.
- worker session acceptance, completion, failures, and questions.
- tool-call protocol states.
- reviewer sample rates and outcomes.
- budget ledger usage.
- recovery actions and results, including provider-caused failures and reroute contract changes.

Observability is graph-addressed, not just log-addressed. A user can start at an initiative, node, question, worker, configuration value, provider route, or edit and drill to relevant events. This addresses `D14`, `D16`, `D17`, and `D18`; it implements `P1`, `P13`, `P14`, and `P15`.

## AI and ML Use

### Orchestrator

Actor: `claude-opus` by default, `claude-sonnet` when long-context capability matters more than reasoning depth.

Inputs:

- `WorkingSetSnapshot`.
- pack/unpack/focus tools.
- sub-agent dispatch affordances.
- question queue relevant to current focus.
- recovery and conflict notices.
- configuration and provider warnings relevant to the current focus.

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
- It cannot override invalid configuration or blocked provider routes.
- It cannot directly create or mutate graph topology, summaries, cross-references, node revisions, identity events, repacks, splits, merges, re-parents, provenance repairs, or quarantine state.
- Its curation suggestions are advisory `OptimizerRequest` artifacts; accepted topology or summary changes remain optimizer-authored decisions in the user surface.
- It may misunderstand lower-privilege evidence, so privilege labels and provenance must remain visible.

Addresses `D1`, `D3`, `D4`, `D5`, `D12`, `D15`, `D17`, and `D18`; implements `P1`, `P2`, `P4`, `P5`, `P6`, `P7`, `P13`.

### Sub-Agent Dispatch

Actor: selected configured model via `agents`, based on task shape and role definitions outside this proposal.

Inputs:

- `WorkerSlice`.
- CLI-specific render.
- capability fingerprint.
- provider state and entitlement snapshot.
- write-scope instructions.
- question envelope contract.

Outputs:

- worker trace evidence.
- final or incremental output.
- `NEEDS_INPUT` artifacts.
- staged reintegration candidates and advisory `OptimizerRequest` artifacts.

Limitations:

- Workers have asymmetric context injection depending on CLI.
- Workers have asymmetric provider entitlements, feature support, runtime availability, and sandbox boundaries.
- Final output is not trusted without trace/provenance.
- Resume acceptance may fail or be ambiguous.
- Parallel workers can conflict.

Addresses `D7`, `D8`, `D9`, `D10`, and `D18`; implements `P8`, `P10`, `P11`, and `P13`.

### Continuous Optimizer

Actor: `glm`.

Inputs:

- bounded graph snapshot.
- changed node set.
- advisory `OptimizerRequest` artifacts.
- summary contracts.
- graph configuration records.
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
- It can mis-curate when configuration semantics are wrong or unclear, so edits cite configuration and shape explanations.
- It cannot mutate the orchestrator's current snapshot.

Addresses `D2`, `D3`, `D4`, `D6`, `D12`, `D13`, `D15`, and `D17`; implements `P1`, `P3`, `P4`, `P5`, `P6`, `P7`, `P12`.

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
- summary template source and effective configuration.

Outputs:

- new `SummaryContract`.
- provenance pointers.
- stale or invalid markers.

Limitations:

- Missing evidence yields invalid summary, not confident prose.
- Contradictory evidence yields uncertainty or conflict.
- Regeneration cannot change node identity.
- Template or field-meaning disagreement yields configuration warning or invalid summary, not silent prose repair.

Addresses `D2`, `D4`, `D10`, and `D17`; implements `P3`, `P4`, `P7`.

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
- Configuration is explicit state. The loss is that graph/memory setup cannot be hidden behind a purely magical experience; the gain is that empty graphs, bad schema shape, incomplete indexes, and field-meaning disagreements can be attributed. Addresses `D17`; implements `P1`, `P3`, `P13`, and `P15`.
- Provider state is observed, not owned. The loss is that the harness cannot guarantee every vendor auth store, entitlement, billing state, or local runtime will be repairable from inside the app; the gain is honest routing, auditable failure causes, and local graph control without copying credentials. Addresses `D18`; implements `P8`, `P13`, `P14`, and `P15`.
- `/compact` is treated as corruption, not a fallback. The loss is compatibility with native compaction workflows; the gain is one coherent memory system. Addresses `D15` and `D16`; implements `P9`.
- Reviewer coverage is sampled. The loss is incomplete LLM review coverage; the gain is bounded cost and reliance on deterministic gates. Addresses `D11` and `D13`; implements `P10` and `P12`.

## Non-Goals

- General-purpose agent framework. The design is a personal orchestrator harness, not a library for arbitrary multi-agent systems. This follows `P16` and the anti-goals.
- Replacing `agent-runner`. The harness depends on `agents` for invocation, balancing, resume, cross-provider session porting, session-id capture, per-CLI storage knowledge, quota tracking, and session ingestion. This follows `P15`.
- Reimplementing provider routing, multi-account load balancing, quota tracking, auth refresh, `--resume` mechanics, cross-provider session porting, or session-id capture. Those remain delegated to `agent-runner`; the harness only records their observable effects where needed for graph, audit, provider state, and recovery.
- Mutating `agent-runner` configuration or binary from the v1 session-override adapter. `AgentRunnerDbAdapter` reads `state.db` and writes only known transcript JSONL files and required state rows at paths derived from `agent-runner` state or configured transcript locators. It does not edit `providers.toml`, `sessions.toml`, model TOMLs, auth stores, or the `agents` executable.
- Concurrent transcript override during an in-flight `agents` write. Until `agent-runner` exposes a pause-handshake or mid-session lock, the harness must wait for session-idle or refuse the override.
- Multi-tenant SaaS or web collaboration. This is excluded by `P16`.
- Sidebar-of-many-chats UI. Multiple initiatives are graph roots under one orchestrator, not separate chat sessions. This follows `P13` and `P16`.
- `/compact` interoperability. Native compaction conflicts with graph provenance and summary contracts. This follows `P9`.
- Treating AI reviewers as policy engines. Reviewer output is evidence only. This follows `P10`.
- Infinite memory. Packing depth does not mean unlimited retention. Explicit deletion, archive, and render exclusion remain possible under policy. This follows `P2`, `P7`, `P12`, and `P15`.
- Credential management or provider account ownership. The harness audits provider state and guides recovery, but it does not become the canonical store for vendor credentials, billing accounts, or local runtime installers. This follows `P15` while addressing `D18`.
- Invisible auto-configuration. Defaults exist, but the harness does not hide which schema, memory, index, routing, or render policy is shaping the graph. This follows `P1` and `P13` while addressing `D17`.
- AI-owned product strategy. The orchestrator may organize work, but it does not own product direction. This follows the philosophy anti-goals.

## Philosophy Gaps and Tensions

No difficulty in `problem.md` appears ungrounded by the current philosophy. The main tensions are already named in [philosophy.md](philosophy.md), and this design resolves them as follows:

- `P1` versus `P5`: optimizer imposition is limited to graph curation and render shape; agent-owned focus is expressed through walk-state tools.
- `P6` versus `P12`: snapshot-walk-then-merge is the weakest coordination semantic that still gives stable per-turn views.
- `P10` versus `P12`: reviewer use is sampled and anomaly-triggered, with deterministic gates carrying the hard guarantees.
- `P2` versus `P3`: bounded renders rely on contract-valid summaries and explicit invalid states when detail cannot be faithfully compressed.
- `P8` versus `P16`: per-CLI asymmetry is represented through concise capability fingerprints rather than exposing every mechanism in the main UI.
- `P13` versus `P14`: recovery is visible, but only consequential recovery interrupts the user.
- `P1` and `P13` versus configuration burden: defaults make the graph usable without pre-work, but every default remains inspectable because configuration changes memory semantics.
- `P15` versus provider reality: the harness keeps graph/provenance under local control while treating vendor credential stores as external evidence sources, not state it must own.

## Alignment Matrix

| Difficulty | Primary design mechanisms | Principles |
|---|---|---|
| `D1` effective working set | `WorkingSetSnapshot`, reasoning budget, required pins, eviction order | `P1`, `P2`, `P12` |
| `D2` summary contract | `SummaryContract`, provenance pointers, invalid states | `P3`, `P7` |
| `D3` concurrent mutation | snapshot-walk-then-merge, bounded `GraphAction`, advisory `OptimizerRequest`, `OptimizerEdit`, `ConflictRecord` | `P1`, `P5`, `P6`, `P14` |
| `D4` stable identity | stable IDs, `IdentityEvent`, forwarding map, optimizer-owned topology edits | `P1`, `P4`, `P5`, `P15` |
| `D5` working-set policy | `AgentWalkState`, pack/unpack/focus tools, bounded foreground actions, eviction | `P2`, `P5` |
| `D6` hierarchical packing | recursive unpack bounds, containment edges, repack edits | `P2`, `P4`, `P6` |
| `D7` cross-CLI asymmetry | `CapabilityFingerprint`, CLI-specific render paths, `SessionOverrideContract` adapter boundary | `P8`, `P13`, `P15` |
| `D8` sub-agent supervision | `WorkerSlice`, worker reintegration states, advisory `OptimizerRequest`, overlap policy | `P5`, `P7`, `P8`, `P10` |
| `D9` user-question routing | `QuestionArtifact`, child acceptance, resume state machine | `P11`, `P14` |
| `D10` tool-call provenance | `ToolCallProvenance`, evidence artifacts, protocol validation | `P7`, `P10`, `P14` |
| `D11` reviewer reliability | deterministic gates plus sampled reviewer evidence | `P10`, `P12` |
| `D12` graph poisoning | privilege labels, quarantine, provenance, policy validation | `P1`, `P7`, `P10`, `P15` |
| `D13` cost/resource tails | `BudgetLedger`, budget gates, cache prefix hashes | `P12` |
| `D14` multi-workstream legibility | single-tab structured panes, status states, notification classes | `P13`, `P16` |
| `D15` imposed context precedent gap | imposed render contract, optimizer-owned curation, bounded foreground actions, local graph source of truth, `SessionOverrideContract`, no `/compact` | `P1`, `P5`, `P9`, `P15` |
| `D16` recovery surfaces | `RecoveryAction`, `SessionOverrideContract` atomicity, explicit preserved/replayed/discarded records | `P14`, `P15` |
| `D17` graph and memory configuration overhead | `GraphConfiguration`, configuration provenance, empty-graph simulation, shape explanations, configuration validation gates | `P1`, `P3`, `P7`, `P13`, `P15` |
| `D18` provider/account/entitlement friction | `ProviderState`, `EntitlementSnapshot`, extended `CapabilityFingerprint`, provider preflight, route denial reasons, provider-aware recovery | `P8`, `P12`, `P13`, `P14`, `P15` |

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

## Layer 0 Context Management and Session Override

This section is an additive operational-policy layer over the graph, summaries, pack/unpack flow, background optimizer, and `agent-runner` substrate. It preserves the round-4 model assignment, turn chunking, and node-size bounds. Round 5 changes only the write-back mechanism: the harness no longer treats per-CLI JSONL manipulation as a general harness responsibility. Repack and detail-injection still produce packed transcript material, but mutation of an `agents` session goes through `SessionOverrideContract`.

### Architectural Axioms

Axiom 1: transcript ingestion is turn-chunked. The optimizer treats a turn as an immutable exchange bundle containing user message, assistant-visible response items, tool-call intents, tool-call inputs, tool results, tool errors, file or command provenance, follow-up assistant text, and foreground advisory optimizer requests. Turn decomposition operates on one turn, or a small adjacent batch from one bundle stream, rather than on an unbounded transcript. The round-4 operating projection batches four turn bundles per LLM invocation for high-frequency work.

Axiom 2: graph nodes are size-bounded by the repack planner below 200K total prompt budget. Repack planning decides how to split, pack, or rebalance context graph nodes, and its operating constraint is that no regeneration unit should exceed the budget for `summary + cross_references + full_content` plus output reserve. The planner enforces a ceiling below 200K total tokens, with a preferred full-regeneration budget of 180K input plus 8K output reserve, so a 204K-class model can process a graph node in the normal path. Split, merge, reparent, and rollup decisions are driven by this size budget.

Axiom 3: `SessionOverrideContract` delineates the harness/agents binary boundary. The harness owns context graph, repack planning, render policy, evidence, provenance, audit, optimizer, recovery, question routing, worker dispatch state, and policy. `agent-runner` owns provider routing, multi-account balancing, quota tracking, auth refresh delegation, resume mechanics, cross-provider session porting, session-id capture, per-CLI storage layout knowledge, and its internal session chain state. The harness may replace or extend an `agents` session transcript only through the versioned `SessionOverrideStore` trait and one of its adapters.

Together these axioms mean context-management tasks do not require larger-than-204K routine model windows, and they also do not require the harness to become a second agent runner. The harness keeps graph memory canonical; `agents` keeps provider/session execution canonical.

### SessionOverrideContract Trait

The harness-side trait is narrow and explicit:

```rust
trait SessionOverrideContract {
    fn schema_version_probe(&self) -> Result<SchemaProbe, SessionOverrideError>;
    fn locate_session(&self, session_ref: SessionRef) -> Result<SessionLocation, SessionOverrideError>;
    fn read_transcript(&self, session_ref: SessionRef) -> Result<CanonicalTranscript, SessionOverrideError>;
    fn replace_transcript(
        &self,
        session_ref: SessionRef,
        transcript: PackedTranscript,
    ) -> Result<OverrideCommit, SessionOverrideError>;
    fn truncate_after(
        &self,
        session_ref: SessionRef,
        turn_ref: TurnRef,
    ) -> Result<OverrideCommit, SessionOverrideError>;
    fn append_turns(
        &self,
        session_ref: SessionRef,
        turns: Vec<CanonicalTurn>,
    ) -> Result<OverrideCommit, SessionOverrideError>;
    fn get_session_metadata(
        &self,
        session_ref: SessionRef,
    ) -> Result<SessionOverrideMetadata, SessionOverrideError>;
}
```

`schema_version_probe()` is a precondition for every write. Because `agent-runner` currently uses idempotent schema-ensure helpers rather than a numbered `PRAGMA user_version`, v1 probes the installed `agents` binary version or commit identity plus the exact table/column/index surface it depends on. Unknown binary, missing expected columns, extra incompatible state, or unsupported storage kind returns `unsupported_schema` before any file write.

`SessionLocation` records provider name, storage kind, raw transcript path, active chain and segment where available, transcript locator evidence, source hashes, mutability state, and whether the session appears idle. `CanonicalTranscript` is a harness format with provider-specific source offsets preserved, not a claim that Claude, Codex, and other CLIs share a native schema.

### Adapter Layering

`SessionOverrideStore` is the dependency used by repack, detail injection, recovery, and tests.

`AgentRunnerDbAdapter` is the v1 implementation. It targets the current `agent-runner` state surface:

- SQLite state at `~/.local/share/oulipoly-agent-runner/state.db`.
- `invocations` rows with session capture and resume acceptance fields.
- `session_turns` rows with provider, session, turn, parent, sidechain, compaction-boundary, source file, and ingest timestamp.
- `session_chains` and `session_chain_segments` rows for stable conversation identity and active provider/session segment.
- Provider `session_storage` declarations for `claude_code` and `codex`, with v1 write support limited to known plaintext JSONL targets that are safe under the pinned range.
- Optional transcript locator scripts such as `claude-code-locate-transcript` and `codex-locate-transcript`.

The v1 adapter reads `state.db`, locates the source transcript, writes a temp JSONL beside the final file, atomically renames it, and updates only the minimum state rows required to keep `agent-runner` lookup and harness audit consistent. It does not modify `agents`, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, or provider routing policy.

`AgentRunnerCliAdapter` is the v2 implementation. It is contingent on upstream `agents session` commands and a pause/lock handshake. Once those commands exist, the harness keeps the same trait and swaps adapter configuration; repack, detail injection, recovery, and tests do not learn a second call pattern.

### Current Agent-Runner Surface

The r5 split is based on the current local `agent-runner` source and README surface:

- Headless CLI mode exists through `/home/nes/.local/bin/agents` / `oulipoly-agent-runner`.
- Persistent runner state lives in SQLite at `~/.local/share/oulipoly-agent-runner/state.db`.
- Provider routing and load balancing are account-aware and quota-aware, with provider state keyed by provider name such as `claude`, `claude2`, `codex`, or `codex2`.
- Quota refresh and auth refresh are delegated through provider configuration and scripts rather than owned by the harness.
- Non-interactive resume exists through `agents resume --session-id <id>` and interactive resume exists through `agents repl <model> --resume <id>`.
- Resume owner lookup uses ingested session state, chain identity, active segments, and provider/model validation.
- Session capture can be configured through mechanisms such as `forced_flag_verified` with `--session-id` and `stdout_json_event`.
- Session ingestion stores turn metadata in `session_turns`, including parent, sidechain, compaction-boundary, source file, and ingest timestamp fields.
- Stable conversation identity is represented through `session_chains` and `session_chain_segments`.
- Transcript locators can lazily resolve raw transcript paths for trace inspection.
- Claude-Code session migration currently has a JSONL copy path with compaction-boundary awareness under known storage declarations.
- Codex storage can participate in chain identity and same-provider resume, but cross-account Codex file-copy migration remains deferred.

The current surface does not expose a stable `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, or explicit schema-version probe. That absence is why v1 exists and why v1 must be pinned and narrow.

### Write-Back Flow

The repack planner produces a `PackedTranscript` from graph state. It may split a node, roll detail into a contract-valid summary, preserve evidence pointers, and decide which prior transcript material remains model-visible. The planner does not open CLI session stores itself. Its terminal action is:

```text
SessionOverrideContract.replace_transcript(session_ref, packed_transcript)
```

The detail-injection router follows the same rule. It can classify new details into existing nodes, child detail lists, new node proposals, ambiguous parking, or cross-reference proposals. When a detail must become visible in a running or resumed session, it produces canonical turns or a packed transcript delta and calls `append_turns`, `truncate_after`, or `replace_transcript` through the contract.

The online renderer remains deterministic. It constructs foreground context from graph snapshot, summary contracts, evidence pointers, budget, provider state, and capability fingerprints. It does not use an LLM to rescue render failure, and it does not write JSONL files directly.

### Atomicity and Recovery

Every transcript replacement is a two-phase write because it crosses a JSONL file and SQLite state:

1. Probe `agents` schema/binary support and adapter capability.
2. Acquire the session-idle lock or fail closed.
3. Read current JSONL hash, transcript mtime, active segment, and relevant state rows.
4. Write the replacement transcript to a same-directory temp file with fsync where available.
5. Write a harness pending-override audit record with preimage hash, temp path, final path, state-row intent, and packed transcript hash.
6. Rename temp file to final JSONL path.
7. Update required state rows in one SQLite transaction when the operation changes chain/segment/turn visibility.
8. Mark the harness override committed and emit `AuditEvent` plus `EvidenceArtifact` records.

Crash recovery compares the pending record against the final file and `state.db`. Matching final file plus matching state commits the pending record. Temp file without state change rolls back by deleting temp and keeping the preimage. File/state disagreement marks the session `quarantined_storage_conflict` and blocks resume until recovery classifies preserved, replayed, and discarded material.

### Race Handling

v1 refuses concurrent override during in-flight `agents` writes. The adapter must prove session-idle by acquiring the agreed lock path when available, observing SQLite non-busy state, checking transcript mtime stability, and confirming no active runner process is writing the same session. If it cannot prove idle, it returns `session_busy`.

The preferred v2 surface is an `agents` pause-handshake: the harness asks `agents` to pause a session, `agents` drains or rejects in-flight writes, returns a lease token, and only then accepts import/replace. Until that lands, overrides happen only between turns or while the session is otherwise idle.

### Failure Defaults

Session override failures are recovery inputs, not invisible prompt degradation:

- `unsupported_schema`: block the override, record the `agents` binary/schema probe, and keep the existing session untouched.
- `session_not_found`: route to recovery with evidence of the lookup path; do not synthesize a new upstream session.
- `ambiguous_session`: require an explicit chain/session disambiguation; do not choose by heuristic inside the harness when `agent-runner` cannot.
- `unsupported_storage`: keep the graph update as harness evidence and avoid mutating the provider transcript.
- `session_busy`: wait, ask for user action, or defer until session-idle; do not race the wrapped CLI.
- `preimage_mismatch`: abort before rename and re-read the session because another writer changed the transcript after probe.
- `post_rename_db_failure`: enter crash-recovery classification and block resume until preserved/replayed/discarded state is explicit.
- `adapter_render_failure`: refuse `append_turns` or `replace_transcript` if the adapter cannot render provider-native records for the pinned range.

These defaults keep the graph source of truth intact while avoiding silent corruption of `agents` session state.

### Operation Ownership

| Concern | Owner | Harness interaction |
|---|---|---|
| Context graph nodes, edges, summaries, revisions | Harness | Canonical graph state; may produce packed transcript material. |
| Working set and render policy | Harness | Determines what should be visible to an agent turn. |
| Repack planning | Harness | Produces `PackedTranscript`; calls `replace_transcript` only through the contract. |
| Detail injection routing | Harness | Produces canonical turns or packed deltas; calls contract append/truncate/replace operations. |
| Provider/account selection | `agent-runner` | Harness observes selected provider and records state; it does not choose accounts directly. |
| Quota tracking and auth refresh | `agent-runner` | Harness consumes provider state snapshots and denial reasons. |
| Session-id generation/capture | `agent-runner` | Harness stores IDs as evidence/correlation keys. |
| Cross-provider session porting | `agent-runner` | Harness treats chain/segment changes as observed execution substrate state. |
| Per-CLI storage layout | `agent-runner` | Harness v1 adapter reads pinned state/locators; v2 asks `agents session locate`. |
| Transcript override | Shared boundary | Harness supplies packed transcript; `SessionOverrideStore` applies it under pinned adapter rules. |

### Per-Task Assignment Matrix

The round-4 operating policy remains unchanged. Model assignment constrains which model route may be used when existing mechanisms call for LLM-backed context-management work; it does not change graph ownership or session override ownership.

| # | Task class | Primary model | Fallback model |
|---:|---|---|---|
| 1 | Turn decomposition | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 2 | Detail injection routing | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 3 | Incremental summary update | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 4 | Full summary regeneration | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 5 | Stale-mark detection | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 6 | Cross-reference discovery | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 7 | Conflict-on-stale-base classification | Claude Opus 4.7 | GPT-5.5 with human gate |
| 8 | Repack planning | MiniMax-M2.7 | MiniMax-M2.7-highspeed |
| 9 | Reviewer sampling | GPT-5.5 | GPT-5.5 retry / queue |
| 10 | Render-time context shaping | Deterministic engine | MiniMax-M2.7 offline diagnostics |
| 11 | Provider routing for orchestrator lead | Claude Sonnet 4.6 | Claude Sonnet 4.6 retry / queue |
| 12 | Provider routing for sub-agents | MiniMax-M2.7 | MiniMax-M2.7-highspeed |

Render-time context shaping uses no model in the normal online path. Provider routing lanes choose configured model names and constraints, but concrete account/provider selection, quota-aware balancing, auth refresh, session porting, and resume composition remain delegated to `agent-runner`.

### Agent-Runner Feature-Request Register

The v1 adapter ships against known local `agent-runner` state and refuses outside its pinned range. These upstream features allow the same harness trait to move to v2:

| Feature request | Needed behavior | Harness impact when landed |
|---|---|---|
| `agents session locate <id>` | Return JSONL path, storage type, provider name, active chain/segment, and mutability state. | Replaces direct `state.db` and locator-script reads in `locate_session`. |
| `agents session export <id>` | Emit canonical JSONL or normalized transcript to stdout with source metadata. | Replaces direct JSONL reads in `read_transcript`. |
| `agents session import-replace <id>` | Atomically replace the transcript and update runner state. | Replaces v1 two-phase file/db write in `replace_transcript`. |
| Pause-handshake / mid-session lock | Coordinate a transcript override with in-flight runner writes and return a lease token. | Removes v1 idle-only restriction and gives a first-class race boundary. |
| Schema-version probe | Expose state schema and supported session-storage adapter versions. | Replaces v1 table/column/binary probing with an upstream compatibility check. |

Until these land, `AgentRunnerDbAdapter` implements equivalents internally and pins to a known `agents` binary and schema range. If `agent-runner` ships frequent breaking state migrations, v1 becomes unacceptable and the harness must block session override work until v2 or another stable upstream API exists.

### Anti-Scope Clarifications

- The harness does not reimplement provider routing, multi-account load balancing, quota tracking, auth refresh, `--resume` mechanics, cross-provider session porting, per-CLI session storage knowledge, or session-id generation/capture.
- v1 direct DB/JSONL access is a temporary adapter, not a claim that `agent-runner` state belongs to the harness.
- v1 writes only known transcript files and required consistency rows. It does not alter `agents` binary, `providers.toml`, `sessions.toml`, model TOMLs, quota scripts, auth stores, provider credentials, or local runtime registries.
- v1 does not support concurrent override during an in-flight `agents` session write. It waits for session-idle or refuses with `session_busy`.
- Codex session-chain identity and same-provider resume can be observed through `agent-runner`; cross-account Codex transcript replacement remains unsupported unless upstream exposes a documented import/replace or state-aware migration surface.

### Assumption Register

| ID | Assumption | Invalidator |
|---|---|---|
| SO-A1 | `agent-runner` state.db schema is stable enough that v1 pinning to a known binary/schema range is acceptable until v2 lands. | `agents` ships breaking schema migrations frequently or without detectable version/surface changes. |
| SO-A2 | Per-CLI JSONL format for supported write targets is stable enough for v1 until v2 lands. | Claude or Codex changes JSONL format, compaction record shape, or resume loader behavior incompatibly. |
| SO-A3 | In-flight session-write race is acceptably handled by flock-style/session-idle locking until pause-handshake lands. | `agents` or the wrapped CLI holds transactions longer than expected, writes without observable locks, or the harness sees stale state after probe. |
| SO-A4 | Transcript locators and `session_storage` declarations can identify the correct raw transcript for supported providers. | Locator scripts return ambiguous, stale, non-absolute, missing, or wrong-session paths. |
| SO-A5 | Packed transcript replacement can preserve enough provider-native shape to resume under the pinned range. | Upstream loader rejects harness-produced records even when hashes, session IDs, and order are preserved. |

### Test-Intent Track

| Contract operation | Change risk | Intended behavior | Level | Fixture source | Expected signal | Residual risk |
|---|---|---|---|---|---|---|
| `schema_version_probe` | Writes against a changed `agents` schema could corrupt sessions. | Accept only a pinned binary/schema surface; reject unknown table/column/index/storage shape before writes. | Per-adapter and per-implementation. | Temp `state.db` fixtures for current, missing-column, extra-incompatible, and unknown-binary cases. | `Ok(SchemaProbe)` for pinned shape; `unsupported_schema` before file mutation for all others. | Does not prove future migrations are semantically compatible when surface shape remains similar. |
| `locate_session` | Wrong provider/path can overwrite another conversation. | Resolve provider, storage kind, active chain/segment, transcript path, mutability, and source hashes for exactly one session. | Per-trait and per-adapter. | `agent-runner` DB fixtures with single chain, duplicate session IDs, missing locator, Claude storage, Codex storage. | Correct `SessionLocation`; ambiguity or unsupported storage fails closed. | Real user stores can contain orphaned files not represented in DB. |
| `read_transcript` | Parser drift can drop tool or compaction records. | Return canonical turns with provider-native source offsets, hashes, and unsupported-record markers rather than silently normalizing away data. | Per-implementation. | Claude JSONL and Codex JSONL samples from local transcript fixtures plus malformed-line fixtures. | Turn count, hashes, offsets, and unsupported markers match goldens. | Goldens may lag new CLI formats. |
| `replace_transcript` | Cross-file/db write can leave half-applied state. | Two-phase write produces atomic JSONL replacement, state consistency, committed audit, and recoverable pending record. | Per-adapter integration. | Temp transcript dirs plus temp `state.db`; crash injection after temp write, after rename, and after DB transaction. | Final/preimage hashes and pending records resolve deterministically; no silent partial success. | Filesystem rename/fsync behavior differs across platforms. |
| `truncate_after` | Truncation can remove needed tool protocol state. | Truncate only after a valid turn boundary and preserve audit preimage; reject truncation inside tool-call/result dependency. | Per-trait and policy integration. | Transcript fixtures with linear turns, pending tool calls, sidechains, and compaction boundary records. | Valid boundary truncates and audits; invalid boundary returns policy error with no file change. | Provider-specific hidden dependencies may not be visible in JSONL. |
| `append_turns` | Appended turns may not match provider-native record order or IDs. | Append canonical turns only when adapter can render valid provider-native records; otherwise refuse and route through replace or recovery. | Per-implementation. | Provider JSONL fixtures with valid append, duplicate turn ID, stale mtime, and unsupported record kind. | Append succeeds with hash update or fails with `unsupported_append` before mutation. | Some CLIs may accept stricter or looser record shapes than fixtures show. |
| `get_session_metadata` | Recovery and UI may show stale session state. | Return provider, chain, segment, capture method, resume acceptance, transcript state, compaction state, idle state, and last observed turn from one consistent snapshot. | Per-adapter. | DB fixtures covering resumed, migrated, accepted, rejected, compaction-boundary, and no-locator sessions. | Metadata fields match fixture rows and transcript existence. | Snapshot can become stale immediately after return without pause-handshake. |
