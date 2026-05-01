# WU-0C-33a: PluginCapability DTO

## Phase Header

**Phase:** Phase 0C - Shared Engines and Integration Shells  
**Work unit:** WU-0C-33a  
**Parent initiative:** Hook/MCP/plugin capability boundary  
**Implementation wave:** Wave 5 from the Phase 0C r4 Parallelization Map

## Source Artifact

- Primary: `worktrees/phase-0c-ai-roadmap-r4/product-strategy/ai-roadmap-phase-0c.md` -> `### WU-0C-33a: PluginCapability DTO`
- Audit: `worktrees/phase-0c-ai-roadmap-r4/plans/audit/ai-roadmap-phase-0c.md` -> Round summaries for r1-r4 cascade history
- Proposal cascade: `worktrees/proposal-r5/product-strategy/proposal.md` -> SessionOverrideContract axiom where relevant
- Engineering cascade: `worktrees/engineering-roadmap-r4/product-strategy/engineering-roadmap.md` -> Phase 0C shared-engine scope and round-4 SessionOverrideContract cascade
- Shape reference: `worktrees/tickets-phase-0c/plans/tickets/phase-0c/` -> superseded r3-derived ticket format only

## Scope

Contract verbatim from the Phase 0C r4 artifact:

```text
schema_object: Rust struct

PluginCapability {
  cli: "claude" | "codex" | "opencode",
  injection_surface: "hooks" | "plugin_transform" | "wrapper_staging" | "mcp" | "none",
  supports_prompt_transform: boolean,
  supports_tool_intercept: boolean,
  supports_mcp_resource: boolean,
  unsupported_reason?: string
}

validate_plugin_capability(capability: PluginCapability) -> Result<PluginCapability, HookError>
```

## Acceptance Criteria

- [ ] `PluginCapability` round-trips through serde with CLI, injection surface, capability flags, and unsupported reason preserved.
- [ ] Every `injection_surface` variant is reachable through a documented fixture.
- [ ] Calling `validate_plugin_capability(capability)` rejects unsupported CLIs that report transform/intercept/MCP support as true.
- [ ] DTO validation does not install plugins, create MCP resources, launch agents, or mutate CapabilityFingerprint rows.

## Dependencies On Earlier Slices

**Roadmap dependency line, verbatim:** WU-0C-32; incoming WU-0B-19 `CapabilityFingerprint`.

**Detailed dependency graph line, verbatim:** WU-0C-33a <- WU-0C-32, WU-0B-19

**Phase 0A upstream WUs:**
- none

**Phase 0B upstream WUs:**
- WU-0B-19

**Intra-Phase 0C predecessors:**
- WU-0C-32

## Test Boundary

`product-strategy/contracts/wu-0c-33a-plugin-capability.md`, `src-tauri/src/contracts/plugin_capability.rs`

## Code Boundary

`src-tauri/src/hooks/plugin_capability.rs`, `src-tauri/src/contracts/plugin_capability.rs`, `src-tauri/tests/wu_0c_33a_plugin_capability_contract.rs`

## Revision Rationale

Greenfield Phase 0C r1 (37 WUs) -> r2 systemic split -> r3 Stitch Notes systematic enumeration (converged at LOW). r4 cascade did not touch this WU's scope. Reference `plans/audit/ai-roadmap-phase-0c.md` round 1-4 entries.

## Handoff Notes

- Pipeline phases: Core path Phases 2.5 through 10 as described in the Pipeline Reference.
- Produces: Plugin capability DTO consumed by WU-0C-33 and provider capability surfaces.
- Parallelizable with: WU-0C-29, WU-0C-31 after prerequisites.
- Keep this WU contract-first and independently mergeable; do not bundle another WU into the implementing PR.
- Preserve the contract, acceptance criteria, dependency lines, test boundary, and code boundary above exactly when drafting implementation handoff artifacts.
- Do not add operator-visible value-slice behavior, redefine Phase 0B schemas, reinterpret Phase 0B enum/state meanings, or introduce a second runtime substrate for policies, renders, agent-runner access, IPC events, UI panes, or audit emission.
