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

## Round 5 - Option A CLI Adapter Simplification

Classification: `fix-created-family` at gen 0 in the engineering-roadmap-local loop, externally driven by proposal-r6's agent-runner feature landing cascade.

Round summary: revised the r4 engineering roadmap for Option A. The planned v1 `AgentRunnerDbAdapter` is dropped, `AgentRunnerCliAdapter` is the only `SessionOverrideContract` implementation, and Phase 0C now consumes landed `agents session locate/export/import-replace/pause-handshake/schema-probe` surfaces with commit/PR references. VS-010, VS-012, VS-015, and VS-018 no longer carry block-on-upstream annotations; their bindings point directly at the CLI adapter and documented `agents session` commands.

D1 decomposition risk: improved. Removing direct DB mutation reduces the adapter family and keeps provider routing, storage layout, and atomic import logic inside `agent-runner`. Residual risk remains that harness-side `truncate_after` and `append_turns` implementations could grow hidden provider-native rendering instead of editing exported canonical JSONL.

D2 coverage risk: improved. The roadmap now names the consumed CLI surfaces, `schema-probe` feature gating, `safe_for_import_replace`, unsupported-storage refusal, pause-handshake lease use, and preimage-gated import. Remaining coverage risk is mostly test depth around non-happy-path CLI exits and receipt shapes.

D3 dependency risk: medium. The dependency shifted from unstable SQLite schema knowledge to stable-but-external `agents session` commands. The main risk is no longer upstream feature absence; it is compatibility drift in CLI JSON output, canonical JSONL semantics, advisory lock behavior, or import-replace receipts.

Watch signals:

- implementation work units reintroduce direct `state.db` reads or writes.
- `truncate_after` / `append_turns` start rendering provider-native transcript formats instead of editing exported canonical JSONL.
- tests cover only successful fake `agents` output and miss `unsupported-storage`, `session-busy` exit 13, `schema-incompatible`, and `preimage-mismatch`.
- external writers mutate session transcripts often enough that advisory lease plus preimage gating causes repeated deferrals.
- roadmap/ticket descendants keep old Phase 0C-r4 or v1/v2 language.
