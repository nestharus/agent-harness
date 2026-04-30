# Engineering Roadmap Audit History

This file records engineering-roadmap-local revision rounds. It was created during round 4 because no prior `plans/audit/engineering-roadmap.md` existed in this worktree; rounds 1-3 are represented by the current roadmap body and the proposal audit history.

## Round 4 - Consume SessionOverrideContract

Classification: `fix-created-family` at gen 0 in the engineering-roadmap-local loop, externally driven by proposal-r5's `SessionOverrideContract` cascade.

Round summary: revised the r3 engineering roadmap to consume the proposal-r5 boundary between the harness and `agent-runner`. Added Phase 0C-r4 `SessionOverrideContract` work units, v1 `AgentRunnerDbAdapter`, future v2 `AgentRunnerCliAdapter`, agent-runner supported-surface feature requests, and anti-scope language that keeps per-CLI JSONL format details out of general harness slices. Revised VS-010, VS-012, VS-018, and VS-015 worker launcher language so write-back flows through the contract and worker launch stays thin over `agents -m <model> -p <project> -f <prompt>`.

D1 decomposition risk: medium. The new boundary reduces direct-jsonl leakage in value slices, but v1 still contains direct `state.db` plus JSONL mutation inside one adapter. Watch for adapter work units growing into provider routing, session migration, or CLI-specific resume behavior that belongs in `agent-runner`.

D2 coverage risk: medium. The named impact slices were updated, and Phase 0C now names the trait, v1 adapter, v2 feature requests, and evidence/audit support. Residual risk remains in downstream recovery and reroute slices that may need more explicit `SessionOverrideContract` call paths in later rounds.

D3 dependency risk: high. The roadmap now depends on a pinned local agent-runner state surface: `state.db`, `invocations`, `session_turns`, `session_chains`, `session_chain_segments`, transcript locator scripts, and `trace --json`. The stable v2 surface does not exist yet, so `agents session locate/export/import-replace`, pause-handshake, and schema/supported-surface probe are external dependencies.

Watch signals:

- new family introduced: `SessionOverrideContract` foundation / adapter family.
- v1 adapter begins prescribing per-CLI transcript formats outside pinned adapter tests.
- worker launcher code learns transcript paths, JSONL record shapes, or provider-specific storage layouts.
- VS-010, VS-012, or VS-018 reintroduce direct session file writes instead of calling `replace_transcript` or `append_turns`.
- Phase 0C-r4 work units are not added before value slices depend on them.
- upstream `agent-runner` changes state schema without a detectable supported-surface probe.
