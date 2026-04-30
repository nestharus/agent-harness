# Proposal Audit History

This file records proposal-local revision rounds. It is a handoff log for future proposer and reviewer passes, not a substitute for the proposal itself.

## Round 1 - Greenfield Proposal

Summary: established the original four-axiom proposal around imposed working set, snapshot-walk-then-merge, summary contract, and local graph/provenance ownership.

Watch signals:

- greenfield incompleteness.
- unsupported cross-CLI assumptions.
- hidden ownership of external session state.

## Round 2 - Brownfield Revision

Summary: tightened the foreground/optimizer ownership boundary after P5 review. The orchestrator can navigate focus and emit advisory optimizer requests, but optimizer-owned curation remains the only path for topology, summaries, cross-references, provenance repair, and repack edits.

Watch signals:

- foreground actions expanding into graph mutation.
- optimizer requests being treated as delegated commands.

## Round 3 - Brownfield Revision

Summary: absorbed the graph/memory configuration and provider/account/entitlement axes. Added configuration-as-memory semantics, provider state as observable state, entitlement snapshots, capability fingerprints, and provider-aware recovery.

Watch signals:

- configuration defaults becoming invisible.
- provider state drifting into credential ownership.

## Round 4 - Context-Management Model Assignment

Summary: added Layer 0 context-management operating policy: turn-chunked transcript decomposition, sub-200K node-size/repack axiom, per-task model assignment matrix, deterministic render shaping, and premium-lane limits.

Watch signals:

- model assignment changes accidentally changing graph ownership.
- 200K node-size bound weakened by routing oversized work to premium long-context models.
- turn decomposition drifting from sealed turn bundles to unbounded transcript prompts.

## Round 5 - SessionOverrideContract Boundary

Classification: `fix-created-family` at gen 0 in the proposal-local loop, externally driven by `agent-runner` maturation.

Summary: revised the harness/agent-runner boundary. `agent-runner` now owns provider routing, multi-account load balancing, quota tracking, auth refresh, `--resume` mechanics, cross-provider session porting, per-CLI session storage location knowledge, and session-id capture. The harness owns graph memory, repack planning, render policy, audit/provenance, recovery, question routing, budgets, worker dispatch state, policy, and configuration. The seam is the versioned `SessionOverrideContract` with v1 `AgentRunnerDbAdapter` and future v2 `AgentRunnerCliAdapter`.

Changes made:

- Added the SessionOverrideContract axiom and trait operations.
- Revised Layer 0 write-back language so repack/detail-injection call `replace_transcript`, `truncate_after`, or `append_turns` instead of writing per-CLI JSONL directly.
- Added v1/v2 adapter layering, schema pinning, atomic two-phase write, crash recovery, and race handling.
- Added the agent-runner feature-request register for `session locate`, `export`, `import-replace`, pause-handshake, and schema probe.
- Added anti-scope clarifications, assumption register, and test-intent track.

Watch signals:

- v1 adapter starts absorbing provider routing or porting logic that belongs in `agent-runner`.
- schema probing relies only on table presence and misses semantic migrations.
- idle-lock behavior proves insufficient under real in-flight `agents` writes.
- Codex or Claude JSONL format changes invalidate packed transcript writes before v2 lands.
- future roadmap/ticket work units keep old direct-jsonl responsibilities instead of depending on `SessionOverrideStore`.
