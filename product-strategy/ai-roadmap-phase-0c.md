# Phase 0C AI Roadmap - Shared Engines and Integration Shells

## Pipeline Reference

Phase 0C work units use the project implementation pipeline as contract-first PR slices.
The test agent receives only the WU contract, DTO signatures, command signatures, service signatures, fixture shapes, and enum/state-machine definitions.
The code agent receives those tests plus the implementation boundary.
A WU is done only when it can merge independently without turning on value-slice behavior.

- Phase 1 research = `gpt-high`.
- Phase 2 synthesis = `gpt-high`.
- Phase 2.5 existing-state risk profile = `gpt-high`.
- Phase 3 implementation proposal = `gpt-high`.
- Phase 4 risk = three independent `claude-opus` judgment passes plus `gpt-high` audit reconciliation.
- Phase 5 hookpoints = `gpt-high`.
- Phase 6a contract handoff = orchestrator.
- Phase 6b tests = separate `gpt-high` invocation with contract-only access.
- Phase 6c code = separate `gpt-high` invocation with contracts plus tests.
- Phase 7 CodeRabbit = tool.
- Phase 8 PR review = test-audit/commit-hygiene `gpt-high` plus multi-concern/justification `claude-opus`.
- Phase 9 draft PR = automation.
- Phase 10 human promotion to ready-for-review.

Phase 0C inherits two converged precedents:

- Phase 0A split repository/runtime shell primitives from test fixture utilities and treated IPC, events, panes, settings, tracing, and fake `agents` as distinct contracts.
- Phase 0B split every durable GraphStore schema object into a unique WU and kept all feature behavior inert.

Phase 0C applies the same rules to method-bearing runtime components.
Every service object, state machine, IPC command router, event emitter, DTO, or adapter with its own behavior is a separate WU unless it is only an error taxonomy subordinate to that WU's single contract.

## Phase 0C Scope

Authoritative scope from `product-strategy/engineering-roadmap.md`:

> Implement RenderEngine core interfaces, PolicyEngine gate framework, BudgetLedger core service, `ConfigurationRegistry` skeleton/read API, CLI subprocess supervisor around `agents`, provider probe adapters, hook/MCP/plugin abstraction points, optimizer queue shell, identity/conflict shell, recovery-action writer, Tauri IPC commands/Channel event streams, and seeded UI panes. This phase should include golden render fixtures per CLI shape and no value-slice-specific claims such as "workers can launch" or "optimizer refreshes summaries."

Scope interpretation for this per-phase decomposition:

- Included: shared runtime service interfaces, pure gate evaluation, deterministic render assembly, budget recording/gates, configuration runtime reads/writes/provenance, provider monitoring over redacted subprocess/config/status evidence, identity/conflict runtime shell over Phase 0B `IdentityEvent` and `ConflictRecord`, recovery-action write/processor shell, agent-runner subprocess supervision, the `SessionOverrideContract` trait plus v1 `AgentRunnerDbAdapter` foundation, hook/plugin boundary contracts, Tauri command routing, event emission backbone, UI pane routing, and audit emit pipeline.
- Excluded: operator-visible inspectors, worker launch UI, optimizer summary refresh behavior, orchestrator turn execution, topology mutation, recovery execution UI, question queue UI, provider reroute/substitution, reviewer sampling, and value-slice-specific `VS-*` apply/view/fixture triples.
- Phase 0C may write durable rows through Phase 0B repositories, but it may not redefine Phase 0B schemas or reinterpret their enum/state-machine meanings.
- Phase 0C freezes integration contracts so Phase 1+ slices consume one canonical service layer instead of inventing per-slice readers, gates, renderers, subprocess wrappers, session transcript writers, IPC routers, or event emitters.
- Phase 0C does not reimplement provider routing, multi-account balancing, quota tracking, auth refresh, `--resume` mechanics, cross-provider session porting, session-id capture, or general per-CLI storage layout knowledge. The only direct DB/JSONL write surface is the pinned v1 `AgentRunnerDbAdapter`, and it is a temporary implementation detail behind `SessionOverrideContract`.

Incoming Phase 0A dependencies:

- WU-0A-02 `HarnessSettings` and settings loader.
- WU-0A-03 `LocalStorageLayout`.
- WU-0A-04 `HarnessAppState`.
- WU-0A-05 `EventTopic`.
- WU-0A-06 `IpcEvent<T>`.
- WU-0A-07 `HarnessCommand`.
- WU-0A-08 `subscribe_workspace_events`.
- WU-0A-09 `TraceContext`.
- WU-0A-10 `BackendSpanEvent`.
- WU-0A-11 `PaneId`.
- WU-0A-12 `ShellRegionState`.
- WU-0A-13 workspace route shell.
- WU-0A-14a temp SQLite harness.
- WU-0A-14b `renderWithHarness`.
- WU-0A-15 `FakeAgentsFixture`.

Incoming Phase 0B dependencies:

- Canonical GraphStore repository foundation: WU-0B-01 through WU-0B-03.
- Policy/config/workspace state: WU-0B-04 through WU-0B-06 plus WU-0B-32.
- Graph, evidence, provenance, summary, revision, identity, snapshot state: WU-0B-07 through WU-0B-14.
- Audit and budget state: WU-0B-15 and WU-0B-16.
- Provider/capability state: WU-0B-17 through WU-0B-19.
- Tool, working-set, foreground action, walk-state, turn state: WU-0B-20 through WU-0B-24.
- Optimizer, conflict, worker, question, recovery state: WU-0B-25 through WU-0B-31.

Round 4 local `agent-runner` source confirmation for v1 adapter scope:

- `src-tauri/src/state/db.rs` creates and migrates `invocations`, `session_turns`, `session_chains`, and `session_chain_segments`; it uses schema-ensure helpers rather than a numbered stable schema-version surface.
- `src-tauri/src/config/model.rs` defines provider `session_storage` variants `ClaudeCode { projects_dir }` and `Codex { sessions_dir }`.
- `src-tauri/src/config/sessions.rs` defines `turn_script`, optional `transcript_locator`, and optional `state_dir` entries; transcript locators are adapter evidence, not a general harness session-write API.
- `src-tauri/src/sessions/mod.rs` exposes normalized turn ingestion and transcript location helpers; direct raw transcript mutation remains outside the general subprocess facade.
- `src-tauri/src/migration/mod.rs` currently has a Claude JSONL copy path and an explicit Codex migration-deferred guard. Phase 0C v1 write support must therefore be schema-version-pinned and storage-kind-limited, and v2 remains blocked on supported `agents session` commands.

## Work Unit Inventory

Total Phase 0C WUs: **77**.

| Group | WUs | Count |
|---|---:|---:|
| PolicyEngine and deterministic gate substrate | WU-0C-01..WU-0C-04 plus WU-0C-02a/WU-0C-04a | 6 |
| BudgetLedger runtime service and gates | WU-0C-05..WU-0C-07 plus WU-0C-06a/WU-0C-07a | 5 |
| ConfigurationRegistry runtime | WU-0C-08..WU-0C-10 plus WU-0C-09a | 4 |
| Agent-runner subprocess supervisor family | WU-0C-11a..WU-0C-18 plus split DTO WUs | 21 |
| SessionOverrideContract and v1 adapter foundation | WU-0C-N1..WU-0C-N5 | 5 |
| Provider monitor and diagnostics | WU-0C-19..WU-0C-20 plus WU-0C-20a | 3 |
| RenderEngine core | WU-0C-21..WU-0C-26 plus split DTO WUs | 12 |
| Optimizer shell | WU-0C-27..WU-0C-29 plus WU-0C-29a/WU-0C-29b | 5 |
| Identity/conflict shell | WU-0C-29c..WU-0C-29d | 2 |
| Recovery shell | WU-0C-30..WU-0C-31 plus WU-0C-30a/WU-0C-31a | 4 |
| Hook/plugin boundary | WU-0C-32..WU-0C-33 plus WU-0C-33a | 3 |
| Tauri IPC, UI shell wiring, audit emit pipeline | WU-0C-34..WU-0C-37 plus split DTO WUs | 7 |

| WU | Owns | Parent initiative |
|---|---|---|
| WU-0C-01 | `PolicyDecision` DTO | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-02a | `PolicyGateDescriptor` DTO | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-02 | `PolicyRegistry` | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-03 | `PolicyValidationReport` | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-04a | `PolicyGateInput` DTO | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-04 | `PolicyEngine::evaluate_gate` | PolicyEngine deterministic gate framework with versioned decisions |
| WU-0C-05 | `BudgetDecision` DTO | BudgetLedger core service |
| WU-0C-06a | `BudgetUsageDraft` DTO | BudgetLedger core service |
| WU-0C-06 | `BudgetLedgerAccountingService` | BudgetLedger core service |
| WU-0C-07a | `BudgetCheckRequest` DTO | BudgetLedger core service |
| WU-0C-07 | `BudgetGateService` | BudgetLedger core service |
| WU-0C-08 | `ConfigurationRegistryRuntimeRead` | ConfigurationRegistry runtime read API |
| WU-0C-09a | `ConfigurationRevisionDraft` DTO | ConfigurationRegistry runtime write API |
| WU-0C-09 | `ConfigurationRegistryWriteApi` | ConfigurationRegistry runtime write API |
| WU-0C-10 | `ConfigurationProvenanceResolver` | ConfigurationRegistry provenance resolver |
| WU-0C-11a | `AgentSpawnRequest` DTO | CLI subprocess supervisor |
| WU-0C-11b | `OulipolyInvocationRef` DTO and parser | CLI subprocess supervisor |
| WU-0C-12a | `AgentSpawnResult` DTO | CLI subprocess supervisor |
| WU-0C-12 | `AgentSubprocessSupervisor` spawn/cancel/timeout operation | CLI subprocess supervisor |
| WU-0C-13a | `SessionCaptureRequest` DTO | CLI subprocess supervisor |
| WU-0C-13b | `AgentRunnerSessionCapture` DTO | CLI subprocess supervisor |
| WU-0C-13 | `AgentSessionCapture` | CLI subprocess supervisor |
| WU-0C-14a | `AgentRunnerTraceEdge` DTO | CLI subprocess supervisor |
| WU-0C-14b | `AgentRunnerTraceTree` DTO | CLI subprocess supervisor |
| WU-0C-14 | `AgentTraceTreeReader` | CLI subprocess supervisor |
| WU-0C-15a | `SessionTurnRef` DTO | CLI subprocess supervisor |
| WU-0C-15b | `SessionTurnsRequest` DTO | CLI subprocess supervisor |
| WU-0C-15c | `SessionTurnsRead` DTO | CLI subprocess supervisor |
| WU-0C-15d | `SessionTurnsReader` | CLI subprocess supervisor |
| WU-0C-16a | `ConfigSnapshotRequest` DTO | CLI subprocess supervisor |
| WU-0C-16b | `AgentRunnerConfigSnapshot` DTO | CLI subprocess supervisor |
| WU-0C-16 | `AgentRunnerConfigSnapshotReader` | CLI subprocess supervisor |
| WU-0C-17a | `ProviderDiagnosticsRequest` DTO | CLI subprocess supervisor and provider diagnostics |
| WU-0C-17b | `ProviderDiagnosticsResult` DTO | CLI subprocess supervisor and provider diagnostics |
| WU-0C-17 | `ProviderDiagnosticsAdapter` | CLI subprocess supervisor and provider diagnostics |
| WU-0C-18 | `AgentRunnerClient` facade | Unified agent-runner wrapper API |
| WU-0C-N2 | `TranscriptTurn`, `SessionLocation`, `SessionMetadata`, and related override DTOs | SessionOverrideContract foundation |
| WU-0C-N1 | `SessionOverrideContract` trait and TypeScript DTO surface | SessionOverrideContract foundation |
| WU-0C-N4 | `AgentRunnerSchemaProbe` | SessionOverrideContract v1 compatibility probe |
| WU-0C-N3 | `AgentRunnerDbAdapter` v1 | SessionOverrideContract v1 adapter |
| WU-0C-N5 | `SessionOverrideStore` registry | SessionOverrideContract audit/override registry |
| WU-0C-19 | `ProviderProbeRequest` DTO | ProviderStateMonitor |
| WU-0C-20a | `ProviderStateMonitorResult` DTO | ProviderStateMonitor |
| WU-0C-20 | `ProviderStateMonitor` | ProviderStateMonitor |
| WU-0C-21 | `RenderRequestDto` | RenderEngine core |
| WU-0C-22a | `RenderPolicy` DTO | RenderEngine core |
| WU-0C-22 | `RenderPolicyResolver` | RenderEngine core |
| WU-0C-23a | `RenderPrivilegeFilterInput` DTO | RenderEngine core |
| WU-0C-23b | `RenderPrivilegeFilterResult` DTO | RenderEngine core |
| WU-0C-23 | `RenderPrivilegeFilter` | RenderEngine core |
| WU-0C-24a | `CachePrefixInput` DTO | RenderEngine core and cache locality |
| WU-0C-24 | `CachePrefixHasher` | RenderEngine core and cache locality |
| WU-0C-25a | `RenderBudgetRequest` DTO | RenderEngine plus BudgetLedger integration |
| WU-0C-25 | `RenderBudgetGateAdapter` | RenderEngine plus BudgetLedger integration |
| WU-0C-26a | `RenderResultDto` | RenderEngine core |
| WU-0C-26 | `RenderEngine::render_working_set` | RenderEngine core |
| WU-0C-27 | `OptimizerQueueService` | Optimizer queue shell |
| WU-0C-28 | `OptimizerCycleStateMachine` | Optimizer cycle shell |
| WU-0C-29a | `OptimizerScheduleDecision` DTO | Optimizer scheduler |
| WU-0C-29b | `OptimizerCycleLease` DTO | Optimizer scheduler |
| WU-0C-29 | `OptimizerScheduler` | Optimizer scheduler |
| WU-0C-29c | `IdentityResolverRuntime` | Identity/conflict shell |
| WU-0C-29d | `ConflictRecordWriterShell` | Identity/conflict shell |
| WU-0C-30a | `RecoveryActionDraft` DTO | Recovery-action writer |
| WU-0C-30 | `RecoveryActionWriter` | Recovery-action writer |
| WU-0C-31a | `RecoveryProcessorDecision` DTO | RecoveryAction processor skeleton |
| WU-0C-31 | `RecoveryActionProcessorSkeleton` | RecoveryAction processor skeleton |
| WU-0C-32 | `HookPayloadEnvelope` | Hook/MCP/plugin injection scaffold |
| WU-0C-33a | `PluginCapability` DTO | Hook/MCP/plugin capability boundary |
| WU-0C-33 | `PluginCapabilityMatrix` | Hook/MCP/plugin capability boundary |
| WU-0C-34 | `TauriIpcCommandRouter` | Tauri IPC layer scaffolding |
| WU-0C-35a | `DomainEventDraft` DTO | Channel event emission backbone |
| WU-0C-35 | `IpcEventEmissionBackbone` | Channel event emission backbone |
| WU-0C-36a | `PaneRouteBinding` DTO | UI shell wiring |
| WU-0C-36 | `UiShellPaneRouter` | UI shell wiring |
| WU-0C-37a | `AuditEmitInput` DTO | Audit event writer subscription and emit |
| WU-0C-37 | `AuditEmitPipeline` | Audit event writer subscription and emit |

## Phase 0C Work Units

### WU-0C-01: PolicyDecision DTO

**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

PolicyDecision<O> {
  decision_id: string,
  policy_set_id: string,
  policy_version: string,
  gate_name: string,
  decision: "accepted" | "rejected" | "deferred" | "quarantined" | "user_required",
  reason_code: string,
  input_ref: string,
  output: O,
  audit_event_draft: AuditEventDraft
}

build_policy_decision<O>(
  policy_set: PolicySet,
  gate_name: string,
  input_ref: string,
  output: O,
  decision: AuditDecision,
  reason_code: string
) -> Result<PolicyDecision<O>, PolicyError>

PolicyError:
- EmptyGateName
- EmptyReasonCode
- UnknownPolicySet
- OutputSerializationFailed
```

**Test boundary:** `product-strategy/contracts/wu-0c-01-policy-decision.md`, `src-tauri/src/contracts/policy_decision.rs`, `src/contracts/policy-decision.ts`, fixtures under `product-strategy/contracts/fixtures/wu-0c-01/`

**Code boundary:** `src-tauri/src/policy/policy_decision.rs`, `src-tauri/src/contracts/policy_decision.rs`, `src/contracts/policy-decision.ts`, `src-tauri/tests/wu_0c_01_policy_decision_contract.rs`, `src/test/policy-decision.test.ts`

**Acceptance criteria:**

- [ ] `PolicyDecision<O>` serializes from Rust to JSON and deserializes into TypeScript while preserving `decision_id`, `policy_set_id`, `policy_version`, `gate_name`, `decision`, `reason_code`, `input_ref`, `output`, and `audit_event_draft`.
- [ ] Every `decision` variant round-trips through Rust serde and TypeScript fixture JSON and is reachable through a documented `build_policy_decision` fixture.
- [ ] Calling `build_policy_decision(policy_set, gate_name, input_ref, output, decision, reason_code)` with documented valid inputs returns `Ok(PolicyDecision<O>)` whose `policy_version` equals the supplied `PolicySet` version.
- [ ] Calling `build_policy_decision(...)` with an empty gate name returns `PolicyError::EmptyGateName`.
- [ ] Calling `build_policy_decision(...)` with an empty reason code returns `PolicyError::EmptyReasonCode`.
- [ ] Calling `build_policy_decision(...)` with an unknown policy set returns `PolicyError::UnknownPolicySet`.
- [ ] Calling `build_policy_decision(...)` with a fixture output that cannot serialize returns `PolicyError::OutputSerializationFailed`.
- [ ] The generated `audit_event_draft` cites the same policy set, decision, reason code, input ref, and output ref as the DTO.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-04 `PolicySet`, WU-0B-15 `AuditEvent`, WU-0B-02 `GraphStoreError`/serde prelude.

**Produces:** Generic policy decision DTO consumed by policy registry, gate evaluator, budget gates, render gates, provider gates, recovery gates, IPC, and audit emit pipeline.

**Parallelizable with:** WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24a, WU-0C-30a.

### WU-0C-02a: PolicyGateDescriptor DTO

**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

PolicyGateDescriptor {
  gate_name: string,
  policy_version_field: string,
  input_schema_ref: string,
  output_schema_ref: string,
  decision_reason_codes: string[]
}

validate_policy_gate_descriptor(descriptor: PolicyGateDescriptor) -> Result<PolicyGateDescriptor, PolicyError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-02a-policy-gate-descriptor.md`, `src-tauri/src/contracts/policy_gate_descriptor.rs`, `src/contracts/policy-gate-descriptor.ts`

**Code boundary:** `src-tauri/src/policy/gate_descriptor.rs`, `src-tauri/src/contracts/policy_gate_descriptor.rs`, `src/contracts/policy-gate-descriptor.ts`, `src-tauri/tests/wu_0c_02a_policy_gate_descriptor_contract.rs`, `src/test/policy-gate-descriptor.test.ts`

**Acceptance criteria:**

- [ ] `PolicyGateDescriptor` round-trips through Rust serde and TypeScript fixture JSON with stable gate name, policy version field, input schema ref, output schema ref, and reason-code list.
- [ ] Calling `validate_policy_gate_descriptor(descriptor)` with valid schema refs returns the normalized DTO.
- [ ] Calling `validate_policy_gate_descriptor(descriptor)` with an empty gate name, empty schema ref, or empty reason-code list returns the documented error variant.
- [ ] DTO validation does not execute policies, write audit rows, mutate PolicySet rows, or call provider/render/budget services.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-04 `PolicySet`.

**Produces:** Gate descriptor DTO consumed by WU-0C-02, WU-0C-04, Tauri command routing, and downstream policy explain surfaces.

**Parallelizable with:** WU-0C-06a, WU-0C-07a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24a.

### WU-0C-02: PolicyRegistry

**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions

**Contract:**
```text
schema_object: Rust service object

PolicyRegistry {
  registered_gates: map<string, PolicyGateDescriptor>
}

PolicyRegistry::register_gate(descriptor: PolicyGateDescriptor) -> Result<(), PolicyError>
PolicyRegistry::get_gate(gate_name: string) -> Result<PolicyGateDescriptor, PolicyError>
PolicyRegistry::list_gates() -> Vec<PolicyGateDescriptor>
```

**Test boundary:** `product-strategy/contracts/wu-0c-02-policy-registry.md`, `src-tauri/src/contracts/policy_registry.rs`, fixtures under `product-strategy/contracts/fixtures/wu-0c-02/`

**Code boundary:** `src-tauri/src/policy/registry.rs`, `src-tauri/src/contracts/policy_registry.rs`, `src-tauri/tests/wu_0c_02_policy_registry_contract.rs`

**Acceptance criteria:**

- [ ] Calling `PolicyRegistry::register_gate(descriptor)` with documented valid input stores exactly one descriptor keyed by `gate_name`.
- [ ] Calling `PolicyRegistry::register_gate(descriptor)` twice with the same `gate_name` returns `PolicyError::DuplicateGate`.
- [ ] Calling `PolicyRegistry::register_gate(descriptor)` with an empty `gate_name`, empty schema ref, or empty reason-code list returns the documented error variant.
- [ ] Calling `PolicyRegistry::get_gate(gate_name)` returns the registered descriptor byte-equivalent for known gates and `PolicyError::UnknownGate` for unknown gates.
- [ ] Calling `PolicyRegistry::list_gates()` returns all registered descriptors in deterministic sorted order.
- [ ] The registry does not execute policies, write audit rows, mutate PolicySet rows, or call provider/render/budget services.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-01, WU-0C-02a; incoming WU-0B-04 `PolicySet`.

**Produces:** Versioned gate catalog consumed by `PolicyEngine::evaluate_gate`, Tauri command routing, and downstream policy explain surfaces.

**Parallelizable with:** WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-16, WU-0C-22, WU-0C-30.

### WU-0C-03: PolicyValidationReport

**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

PolicyValidationReport {
  report_id: string,
  policy_set_id: string,
  gate_name: string,
  validation_state: "passed" | "failed" | "not_applicable",
  checked_refs: string[],
  failure_reasons: string[],
  audit_event_draft?: AuditEventDraft
}

build_policy_validation_report(
  policy_set: PolicySet,
  gate_name: string,
  checked_refs: string[],
  failures: string[]
) -> Result<PolicyValidationReport, PolicyError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-03-policy-validation-report.md`, `src-tauri/src/contracts/policy_validation_report.rs`, `src/contracts/policy-validation-report.ts`

**Code boundary:** `src-tauri/src/policy/validation_report.rs`, `src-tauri/src/contracts/policy_validation_report.rs`, `src/contracts/policy-validation-report.ts`, `src-tauri/tests/wu_0c_03_policy_validation_report_contract.rs`, `src/test/policy-validation-report.test.ts`

**Acceptance criteria:**

- [ ] `PolicyValidationReport` serializes from Rust to JSON and deserializes into TypeScript with exactly the documented fields.
- [ ] Every `validation_state` variant round-trips and is reachable through a documented fixture.
- [ ] Calling `build_policy_validation_report(policy_set, gate_name, checked_refs, [])` returns `validation_state = passed`.
- [ ] Calling `build_policy_validation_report(policy_set, gate_name, checked_refs, failures)` with one or more failures returns `validation_state = failed` and preserves every failure reason.
- [ ] Calling `build_policy_validation_report(...)` with an empty gate name returns `PolicyError::EmptyGateName`.
- [ ] Calling `build_policy_validation_report(...)` with empty checked refs and no failures returns `validation_state = not_applicable`.
- [ ] Reports with `validation_state = failed` include an audit event draft; reports with `passed` may omit it.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-01; incoming WU-0B-04 `PolicySet`, WU-0B-15 `AuditEvent`.

**Produces:** Validation report DTO consumed by PolicyEngine, render gate diagnostics, optimizer cycle shell, and recovery processor shell.

**Parallelizable with:** WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-16, WU-0C-22, WU-0C-30.

### WU-0C-04a: PolicyGateInput DTO

**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

PolicyGateInput {
  gate_name: string,
  policy_set_id: string,
  input_ref: string,
  payload: JsonValue
}

validate_policy_gate_input(input: PolicyGateInput) -> Result<PolicyGateInput, PolicyError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-04a-policy-gate-input.md`, `src-tauri/src/contracts/policy_gate_input.rs`, `src/contracts/policy-gate-input.ts`

**Code boundary:** `src-tauri/src/policy/gate_input.rs`, `src-tauri/src/contracts/policy_gate_input.rs`, `src/contracts/policy-gate-input.ts`, `src-tauri/tests/wu_0c_04a_policy_gate_input_contract.rs`, `src/test/policy-gate-input.test.ts`

**Acceptance criteria:**

- [ ] `PolicyGateInput` round-trips through Rust serde and TypeScript fixture JSON with `gate_name`, `policy_set_id`, `input_ref`, and `payload` preserved.
- [ ] Calling `validate_policy_gate_input(input)` with valid input returns the normalized DTO.
- [ ] Calling `validate_policy_gate_input(input)` with empty gate name, policy set ID, or input ref returns the documented `PolicyError` variant.
- [ ] Validation does not read PolicySet rows, evaluate policies, or emit audit drafts.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-04 `PolicySet` ID taxonomy and WU-0B-15 `AuditEvent` draft shape only.

**Produces:** Policy gate input DTO consumed by WU-0C-04 and downstream gate adapters.

**Parallelizable with:** WU-0C-06a, WU-0C-07a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.

### WU-0C-04: PolicyEngine Gate Evaluator

**Parent initiative:** PolicyEngine deterministic gate framework with versioned decisions

**Contract:**
```text
schema_object: Rust service object

PolicyEngine::evaluate_gate(input: PolicyGateInput) -> Result<PolicyDecision<JsonValue>, PolicyError>
PolicyEngine::validate_gate(input: PolicyGateInput) -> Result<PolicyValidationReport, PolicyError>

Routing invariant:
- Gate names must exist in PolicyRegistry.
- Policy version is taken from the active PolicySet row, not from caller-provided payload.
- Evaluation is deterministic per `(gate_name, policy_set_id, input_ref, payload)`.
```

**Test boundary:** `product-strategy/contracts/wu-0c-04-policy-engine-gate-evaluator.md`, `src-tauri/src/contracts/policy_engine.rs`

**Code boundary:** `src-tauri/src/policy/engine.rs`, `src-tauri/src/contracts/policy_engine.rs`, `src-tauri/tests/wu_0c_04_policy_engine_contract.rs`

**Acceptance criteria:**

- [ ] Calling `PolicyEngine::evaluate_gate(input)` with a registered gate and valid policy set returns `Ok(PolicyDecision<JsonValue>)` and preserves gate name, policy version, input ref, output payload, decision, and reason code.
- [ ] Calling `PolicyEngine::evaluate_gate(input)` twice with equivalent inputs returns byte-equivalent decisions.
- [ ] Calling `PolicyEngine::evaluate_gate(input)` with an unknown gate returns `PolicyError::UnknownGate`.
- [ ] Calling `PolicyEngine::evaluate_gate(input)` with an unknown policy set returns `PolicyError::UnknownPolicySet`.
- [ ] Calling `PolicyEngine::validate_gate(input)` with valid input returns `Ok(PolicyValidationReport)` with `validation_state = passed`.
- [ ] Calling `PolicyEngine::validate_gate(input)` with invalid payload returns `validation_state = failed` and cites the failing field.
- [ ] The routing invariant holds across all documented gate inputs; caller-supplied policy-version payloads are ignored or rejected rather than trusted.
- [ ] Evaluation writes no durable GraphStore rows; audit emission is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-01, WU-0C-02, WU-0C-03, WU-0C-04a; incoming WU-0B-04 `PolicySet`.

**Produces:** Deterministic policy gate evaluator consumed by render, budget, provider, optimizer, recovery, worker, and IPC shells.

**Parallelizable with:** WU-0C-10 after WU-0C-08; WU-0C-13, WU-0C-15d, WU-0C-16.

### WU-0C-05: BudgetDecision DTO

**Parent initiative:** BudgetLedger core service

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

BudgetDecision {
  decision_id: string,
  scope_type: BudgetScopeType,
  scope_id: string,
  budget_state: "within" | "near_limit" | "exceeded" | "blocked",
  policy_action: "none" | "warn" | "narrow_scope" | "require_user_approval" | "block",
  ledger_ref?: string,
  reason_code: string
}

build_budget_decision(input: BudgetDecisionInput) -> Result<BudgetDecision, BudgetError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-05-budget-decision.md`, `src-tauri/src/contracts/budget_decision.rs`, `src/contracts/budget-decision.ts`

**Code boundary:** `src-tauri/src/budget/budget_decision.rs`, `src-tauri/src/contracts/budget_decision.rs`, `src/contracts/budget-decision.ts`, `src-tauri/tests/wu_0c_05_budget_decision_contract.rs`, `src/test/budget-decision.test.ts`

**Acceptance criteria:**

- [ ] `BudgetDecision` round-trips through Rust serde and TypeScript fixture JSON with all fields preserved.
- [ ] Every `budget_state` variant round-trips and is reachable through a documented input.
- [ ] Every `policy_action` variant round-trips and is reachable through a documented input.
- [ ] Calling `build_budget_decision(input)` with valid input returns the documented DTO with budget state and policy action preserved.
- [ ] Calling `build_budget_decision(input)` with `budget_state = blocked` and `policy_action != block` returns `BudgetError::InvalidPolicyAction`.
- [ ] Calling `build_budget_decision(input)` with `budget_state = within` and `policy_action = require_user_approval` or `block` returns `BudgetError::InvalidPolicyAction`.
- [ ] Calling `build_budget_decision(input)` with an empty scope ID or reason code returns the documented error variant.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-16 `BudgetLedger`.

**Produces:** Budget decision DTO consumed by budget accounting, budget gates, render budget adapter, optimizer scheduler, reviewer sampling, recovery, and UI cost surface.

**Parallelizable with:** WU-0C-01, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24a, WU-0C-30a.

### WU-0C-06a: BudgetUsageDraft DTO

**Parent initiative:** BudgetLedger core service

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

BudgetUsageDraft {
  workspace_id: string,
  scope_type: BudgetScopeType,
  scope_id: string,
  input_tokens: i64,
  output_tokens: i64,
  cache_read_tokens: i64,
  cache_write_tokens: i64,
  latency_ms: i64,
  provider_cost_estimate: decimal,
  provider_state_id?: string,
  cache_prefix_hash?: string
}

validate_budget_usage_draft(draft: BudgetUsageDraft) -> Result<BudgetUsageDraft, BudgetError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-06a-budget-usage-draft.md`, `src-tauri/src/contracts/budget_usage_draft.rs`, `src/contracts/budget-usage-draft.ts`

**Code boundary:** `src-tauri/src/budget/usage_draft.rs`, `src-tauri/src/contracts/budget_usage_draft.rs`, `src/contracts/budget-usage-draft.ts`, `src-tauri/tests/wu_0c_06a_budget_usage_draft_contract.rs`, `src/test/budget-usage-draft.test.ts`

**Acceptance criteria:**

- [ ] `BudgetUsageDraft` round-trips through Rust serde and TypeScript fixture JSON with all counter, provider, scope, and cache-prefix fields preserved.
- [ ] Calling `validate_budget_usage_draft(draft)` with valid non-negative counters returns the normalized DTO.
- [ ] Calling `validate_budget_usage_draft(draft)` with negative token, latency, or cost values returns `BudgetError::NegativeCounter`.
- [ ] DTO validation writes no BudgetLedger rows and evaluates no budget policy.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-16 `BudgetLedger` field taxonomy and WU-0B-17 `ProviderState` refs.

**Produces:** Usage draft DTO consumed by WU-0C-06 and render/budget integrations.

**Parallelizable with:** WU-0C-04a, WU-0C-07a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.

### WU-0C-06: BudgetLedgerAccountingService

**Parent initiative:** BudgetLedger core service

**Contract:**
```text
schema_object: Rust service object

BudgetLedgerAccountingService::record_usage(draft: BudgetUsageDraft) -> Result<BudgetLedger, BudgetError>
BudgetLedgerAccountingService::totals(scope_type: BudgetScopeType, scope_id: string) -> Result<BudgetTotals, BudgetError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-06-budget-ledger-accounting-service.md`, `src-tauri/src/contracts/budget_accounting.rs`

**Code boundary:** `src-tauri/src/budget/accounting.rs`, `src-tauri/src/contracts/budget_accounting.rs`, `src-tauri/tests/wu_0c_06_budget_accounting_contract.rs`

**Acceptance criteria:**

- [ ] Calling `record_usage(draft)` with valid counters writes exactly one WU-0B-16 `BudgetLedger` row and returns it with every token, cache, latency, cost, provider, prefix-hash, scope, and RecordMeta field preserved.
- [ ] Calling `record_usage(draft)` with negative token, latency, or cost values returns `BudgetError::NegativeCounter` and writes no ledger row.
- [ ] Calling `record_usage(draft)` with an unknown provider state returns `BudgetError::UnknownProviderState` and writes no ledger row.
- [ ] Calling `record_usage(draft)` with `cache_prefix_hash` preserves the exact hash string used by RenderEngine fixtures.
- [ ] Calling `totals(scope_type, scope_id)` returns deterministic summed token/cache/cost/latency totals for that scope only.
- [ ] Calling `totals(scope_type, scope_id)` with an unknown scope returns zero totals plus a documented empty state, not a fabricated ledger row.
- [ ] The service does not evaluate policy actions; gate decisions are owned by WU-0C-07.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-05, WU-0C-06a; incoming WU-0B-16 `BudgetLedger`, WU-0B-17 `ProviderState`.

**Produces:** Accounting service for token/cache/cost/latency rows.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-16, WU-0C-22.

### WU-0C-07a: BudgetCheckRequest DTO

**Parent initiative:** BudgetLedger core service

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

BudgetCheckRequest {
  workspace_id: string,
  scope_type: BudgetScopeType,
  scope_id: string,
  projected_input_tokens: i64,
  projected_output_tokens: i64,
  projected_cache_read_tokens: i64,
  projected_cache_write_tokens: i64,
  provider_state_id?: string,
  policy_set_id: string
}

validate_budget_check_request(request: BudgetCheckRequest) -> Result<BudgetCheckRequest, BudgetError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-07a-budget-check-request.md`, `src-tauri/src/contracts/budget_check_request.rs`, `src/contracts/budget-check-request.ts`

**Code boundary:** `src-tauri/src/budget/check_request.rs`, `src-tauri/src/contracts/budget_check_request.rs`, `src/contracts/budget-check-request.ts`, `src-tauri/tests/wu_0c_07a_budget_check_request_contract.rs`, `src/test/budget-check-request.test.ts`

**Acceptance criteria:**

- [ ] `BudgetCheckRequest` round-trips through Rust serde and TypeScript fixture JSON with all projected usage and policy fields preserved.
- [ ] Calling `validate_budget_check_request(request)` with valid projected usage returns the normalized request.
- [ ] Calling `validate_budget_check_request(request)` with negative projected usage returns `BudgetError::NegativeCounter`.
- [ ] Calling `validate_budget_check_request(request)` with empty workspace, scope, or policy set ID returns the documented error.
- [ ] DTO validation writes no BudgetLedger rows and performs no policy evaluation.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-04 `PolicySet` and WU-0B-16 `BudgetLedger` field taxonomy.

**Produces:** Budget check request DTO consumed by WU-0C-07 and render budget adapters.

**Parallelizable with:** WU-0C-04a, WU-0C-06a, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.

### WU-0C-07: BudgetGateService

**Parent initiative:** BudgetLedger core service

**Contract:**
```text
schema_object: Rust service object

BudgetGateService::check_budget(request: BudgetCheckRequest) -> Result<BudgetDecision, BudgetError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-07-budget-gate-service.md`, `src-tauri/src/contracts/budget_gate.rs`

**Code boundary:** `src-tauri/src/budget/gate.rs`, `src-tauri/src/contracts/budget_gate.rs`, `src-tauri/tests/wu_0c_07_budget_gate_contract.rs`

**Acceptance criteria:**

- [ ] Calling `check_budget(request)` with within-budget projected usage returns `BudgetDecision { budget_state = within, policy_action = none }`.
- [ ] Calling `check_budget(request)` near configured thresholds returns `policy_action = warn` or `narrow_scope` according to the active PolicySet fixture.
- [ ] Calling `check_budget(request)` over hard limits returns `budget_state = blocked` and `policy_action = block`.
- [ ] Calling `check_budget(request)` with negative projected usage returns `BudgetError::NegativeCounter`.
- [ ] Calling `check_budget(request)` with an unknown policy set returns `BudgetError::UnknownPolicySet`.
- [ ] Calling `check_budget(request)` twice with equivalent inputs returns byte-equivalent decisions.
- [ ] The service reads BudgetLedger totals through WU-0C-06 and policy versions through WU-0C-04; it writes no `BudgetLedger` row.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-05, WU-0C-06, WU-0C-07a; incoming WU-0B-04 `PolicySet`, WU-0B-16 `BudgetLedger`.

**Produces:** Deterministic budget gate used by RenderEngine, optimizer scheduler, worker dispatch, reviewer sampling, and recovery preflight.

**Parallelizable with:** WU-0C-10, WU-0C-13, WU-0C-15d, WU-0C-16.

### WU-0C-08: ConfigurationRegistry Runtime Read

**Parent initiative:** ConfigurationRegistry runtime

**Contract:**
```text
schema_object: Rust service object

ConfigurationRegistryRuntimeRead::active(workspace_id: string) -> Result<GraphConfiguration, ConfigurationError>
ConfigurationRegistryRuntimeRead::get(configuration_id: string) -> Result<GraphConfiguration, ConfigurationError>
ConfigurationRegistryRuntimeRead::effective_values(configuration_id: string) -> Result<EffectiveConfiguration, ConfigurationError>

Read invariant:
- Reads select canonical WU-0B-05 GraphConfiguration rows.
- Reads do not create configuration revisions, OptimizerRequest rows, AuditEvent rows, or UI events.
```

**Test boundary:** `product-strategy/contracts/wu-0c-08-configuration-registry-runtime-read.md`, `src-tauri/src/contracts/configuration_registry_read.rs`

**Code boundary:** `src-tauri/src/configuration/runtime_read.rs`, `src-tauri/src/contracts/configuration_registry_read.rs`, `src-tauri/tests/wu_0c_08_configuration_read_contract.rs`

**Acceptance criteria:**

- [ ] Calling `active(workspace_id)` with a valid workspace returns the active GraphConfiguration selected by GraphWorkspace.active_configuration_id.
- [ ] Calling `active(workspace_id)` with an unknown workspace returns `ConfigurationError::UnknownWorkspace`.
- [ ] Calling `get(configuration_id)` returns the exact WU-0B-05 row for known IDs and `ConfigurationError::UnknownConfiguration` for unknown IDs.
- [ ] Calling `effective_values(configuration_id)` returns every effective value plus source ref declared by GraphConfiguration.
- [ ] Every effective-value source variant from WU-0B-05 round-trips through the runtime read API.
- [ ] The read invariant holds across all fixtures; no write, audit, optimizer, or IPC event side effect occurs.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-05 `GraphConfiguration`, WU-0B-06 `GraphWorkspace`, WU-0B-32 registry skeleton/read repository.

**Produces:** Runtime read facade consumed by RenderEngine, optimizer scheduler, configuration write API, provider monitor, and IPC command router.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24a, WU-0C-30a.

### WU-0C-09a: ConfigurationRevisionDraft DTO

**Parent initiative:** ConfigurationRegistry runtime

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

ConfigurationRevisionDraft {
  workspace_id: string,
  base_configuration_id: string,
  changes: JsonObject,
  actor: ActorRef,
  reason: string
}

validate_configuration_revision_draft(draft: ConfigurationRevisionDraft) -> Result<ConfigurationRevisionDraft, ConfigurationError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-09a-configuration-revision-draft.md`, `src-tauri/src/contracts/configuration_revision_draft.rs`, `src/contracts/configuration-revision-draft.ts`

**Code boundary:** `src-tauri/src/configuration/revision_draft.rs`, `src-tauri/src/contracts/configuration_revision_draft.rs`, `src/contracts/configuration-revision-draft.ts`, `src-tauri/tests/wu_0c_09a_configuration_revision_draft_contract.rs`, `src/test/configuration-revision-draft.test.ts`

**Acceptance criteria:**

- [ ] `ConfigurationRevisionDraft` round-trips through Rust serde and TypeScript fixture JSON with workspace, base configuration, changes, actor, and reason preserved.
- [ ] Calling `validate_configuration_revision_draft(draft)` with valid changes returns the normalized draft.
- [ ] Calling `validate_configuration_revision_draft(draft)` with empty workspace, base configuration, or reason returns the documented `ConfigurationError` variant.
- [ ] DTO validation writes no GraphConfiguration, GraphWorkspace, OptimizerRequest, AuditEvent, or IPC event rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-05 `GraphConfiguration`, WU-0B-06 `GraphWorkspace`, WU-0B-15 `AuditEvent` draft shape.

**Produces:** Configuration revision draft DTO consumed by WU-0C-09 and later configuration inspector surfaces.

**Parallelizable with:** WU-0C-04a, WU-0C-06a, WU-0C-07a, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.

### WU-0C-09: ConfigurationRegistry Write API

**Parent initiative:** ConfigurationRegistry runtime

**Contract:**
```text
schema_object: Rust service object

ConfigurationRegistryWriteApi::create_revision(draft: ConfigurationRevisionDraft) -> Result<GraphConfiguration, ConfigurationError>
ConfigurationRegistryWriteApi::activate(workspace_id: string, configuration_id: string, expected_workspace_version: i64) -> Result<GraphWorkspace, ConfigurationError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-09-configuration-registry-write-api.md`, `src-tauri/src/contracts/configuration_registry_write.rs`

**Code boundary:** `src-tauri/src/configuration/write_api.rs`, `src-tauri/src/contracts/configuration_registry_write.rs`, `src-tauri/tests/wu_0c_09_configuration_write_contract.rs`

**Acceptance criteria:**

- [ ] Calling `create_revision(draft)` with valid changes appends one new GraphConfiguration row whose `created_from_configuration_id` equals `base_configuration_id`.
- [ ] Calling `create_revision(draft)` with unknown base configuration returns `ConfigurationError::UnknownConfiguration` and writes no row.
- [ ] Calling `create_revision(draft)` with empty reason returns `ConfigurationError::EmptyReason`.
- [ ] Calling `activate(workspace_id, configuration_id, expected_workspace_version)` with valid inputs updates only GraphWorkspace.active_configuration_id and RecordMeta.
- [ ] Calling `activate(...)` with stale expected workspace version returns `ConfigurationError::OptimisticConflict`.
- [ ] Write API does not create optimizer requests, operator-visible warnings, provider probes, or render snapshots.
- [ ] Every write path is append-only for GraphConfiguration revisions; prior rows remain byte-equivalent after the call.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-08, WU-0C-09a; incoming WU-0B-05 `GraphConfiguration`, WU-0B-06 `GraphWorkspace`, WU-0B-15 `AuditEvent`.

**Produces:** Runtime configuration write API for later VS-005/VS-011 surfaces without operator-visible inspection in Phase 0C.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-12, WU-0C-14, WU-0C-16.

### WU-0C-10: ConfigurationProvenanceResolver

**Parent initiative:** ConfigurationRegistry runtime

**Contract:**
```text
schema_object: Rust service object

ConfigurationProvenanceResolver::effective_source(configuration_id: string, field_path: string) -> Result<EffectiveValueSource, ConfigurationError>
ConfigurationProvenanceResolver::explain(configuration_id: string, field_path: string) -> Result<ConfigurationExplanation, ConfigurationError>
ConfigurationProvenanceResolver::explain_render_inputs(configuration_id: string, render_policy_id: string) -> Result<ConfigurationExplanationRef, ConfigurationError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-10-configuration-provenance-resolver.md`, `src-tauri/src/contracts/configuration_provenance_resolver.rs`

**Code boundary:** `src-tauri/src/configuration/provenance_resolver.rs`, `src-tauri/src/contracts/configuration_provenance_resolver.rs`, `src-tauri/tests/wu_0c_10_configuration_provenance_contract.rs`

**Acceptance criteria:**

- [ ] Calling `effective_source(configuration_id, field_path)` returns exactly one documented effective-value source variant and cites the row or policy ref that produced it.
- [ ] Calling `effective_source(...)` with an unknown field returns `ConfigurationError::UnknownField`.
- [ ] Calling `explain(configuration_id, field_path)` returns a non-empty structured explanation for every GraphConfiguration field.
- [ ] Calling `explain_render_inputs(configuration_id, render_policy_id)` returns a stable `configuration_explanation_ref` usable by WU-0B-21 WorkingSetSnapshot rows.
- [ ] Every effective-value source variant is reachable through a documented fixture and unknown variants are rejected.
- [ ] The resolver is read-only and writes no GraphConfiguration, AuditEvent, OptimizerRequest, WorkingSetSnapshot, or IPC event rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-08; incoming WU-0B-05 `GraphConfiguration`, WU-0B-21 `WorkingSetSnapshot`, WU-0B-32 registry skeleton.

**Produces:** Runtime provenance explanations consumed by RenderEngine, IPC, UI shell, VS-005, and audit emit pipeline.

**Parallelizable with:** WU-0C-04, WU-0C-07, WU-0C-13, WU-0C-15d, WU-0C-16.

### WU-0C-11a: AgentSpawnRequest DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

AgentSpawnRequest {
  agents_binary: string,
  model: string,
  project_dir: string,
  prompt_file: string,
  stdin_ref?: string,
  env: map<string, string>,
  timeout_ms: u64,
  parent_invocation_id?: string,
  resume_session_id?: string,
  route_constraints_ref?: string
}

validate_spawn_request(request: AgentSpawnRequest) -> Result<AgentSpawnRequest, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-11a-agent-spawn-request.md`, `src-tauri/src/contracts/agent_spawn_request.rs`

**Code boundary:** `src-tauri/src/agent_runner/spawn_request.rs`, `src-tauri/src/contracts/agent_spawn_request.rs`, `src-tauri/tests/wu_0c_11a_agent_spawn_request_contract.rs`

**Acceptance criteria:**

- [ ] `AgentSpawnRequest` round-trips through serde while preserving agents binary, model, project dir, prompt file, stdin ref, env, timeout, parent invocation, optional resume session, and route-constraints refs.
- [ ] The documented argv fixture renders as an `agents` invocation over model/project/prompt inputs; it does not render provider CLI commands such as `claude`, `codex`, or `opencode` directly.
- [ ] Calling `validate_spawn_request(request)` with valid inputs returns the normalized request.
- [ ] Calling `validate_spawn_request(request)` with empty `agents_binary`, `model`, `project_dir`, `prompt_file`, or zero timeout returns the documented error variant.
- [ ] Spawn request validation accepts only harness route constraints as opaque refs and never selects provider account, provider alias, quota window, auth profile, resume strategy, or cross-provider porting behavior.
- [ ] Spawn request validation does not parse stderr, launch agents, open transcript files, or write ProviderState, WorkerRun, OrchestratorTurn, QuestionArtifact, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0A-02 `HarnessSettings`, WU-0A-03 `LocalStorageLayout`, WU-0A-09 `TraceContext`, WU-0B-19 `CapabilityFingerprint`.

**Produces:** Spawn request DTO consumed by subprocess supervisor, hook payloads, plugin capability matrix, and AgentRunnerClient.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-19, WU-0C-21, WU-0C-24, WU-0C-30.

**Revision rationale:** Round 4 removes direct provider-CLI routing from the spawn request. The harness records the intended model/project/prompt and invokes `agents`; provider routing, account choice, quota-aware balancing, auth refresh, resume composition, and per-CLI command formation stay owned by `agent-runner`.

### WU-0C-11b: OulipolyInvocationRef DTO and Parser

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

OulipolyInvocationRef {
  invocation_id: string,
  parent_invocation_id?: string,
  raw_stderr_ref: string
}

parse_oulipoly_invocation(stderr_ref: string) -> Result<OulipolyInvocationRef, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-11b-oulipoly-invocation-ref.md`, `src-tauri/src/contracts/oulipoly_invocation_ref.rs`

**Code boundary:** `src-tauri/src/agent_runner/oulipoly_invocation.rs`, `src-tauri/src/contracts/oulipoly_invocation_ref.rs`, `src-tauri/tests/wu_0c_11b_oulipoly_invocation_contract.rs`

**Acceptance criteria:**

- [ ] `OulipolyInvocationRef` round-trips through serde while preserving invocation, parent invocation, and raw stderr refs.
- [ ] Calling `parse_oulipoly_invocation(stderr_ref)` against stderr containing `OULIPOLY_INVOCATION` returns the parsed invocation ID.
- [ ] Calling `parse_oulipoly_invocation(stderr_ref)` against stderr containing `OULIPOLY_PARENT_INVOCATION` returns both child and parent IDs.
- [ ] Calling `parse_oulipoly_invocation(stderr_ref)` with missing invocation markers returns `AgentRunnerError::InvocationMarkerMissing` and does not fabricate IDs.
- [ ] Parser output is substrate evidence only and writes no graph, worker, turn, provider, recovery, or audit rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0A-09 `TraceContext` and stderr evidence refs.

**Produces:** Invocation reference parser consumed by subprocess supervisor, trace reader, hook payloads, plugin capability matrix, and AgentRunnerClient.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-19, WU-0C-21, WU-0C-24, WU-0C-30.

### WU-0C-12a: AgentSpawnResult DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

AgentSpawnResult {
  invocation_id: string,
  parent_invocation_id?: string,
  session_id?: string,
  stdout_ref: string,
  stderr_ref: string,
  exit_status?: i32,
  acceptance_state: "unknown" | "accepted" | "rejected" | "timed_out" | "ambiguous"
}

validate_agent_spawn_result(result: AgentSpawnResult) -> Result<AgentSpawnResult, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-12a-agent-spawn-result.md`, `src-tauri/src/contracts/agent_spawn_result.rs`

**Code boundary:** `src-tauri/src/agent_runner/spawn_result.rs`, `src-tauri/src/contracts/agent_spawn_result.rs`, `src-tauri/tests/wu_0c_12a_agent_spawn_result_contract.rs`

**Acceptance criteria:**

- [ ] `AgentSpawnResult` round-trips through serde and preserves stdout/stderr refs, exit status, invocation IDs, optional session ID, and acceptance state.
- [ ] Every acceptance-state variant round-trips and is reachable through a WU-0A-15 fake `agents` scenario.
- [ ] Calling `validate_agent_spawn_result(result)` with valid refs returns the normalized DTO.
- [ ] Calling `validate_agent_spawn_result(result)` with empty invocation ID, stdout ref, or stderr ref returns the documented error.
- [ ] DTO validation does not spawn, wait for, or cancel subprocesses.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11b; incoming WU-0A-15 `FakeAgentsFixture`.

**Produces:** Spawn result DTO consumed by WU-0C-12 and the AgentRunnerClient facade.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06a, WU-0C-09a, WU-0C-13a, WU-0C-13b, WU-0C-16a, WU-0C-16b.

### WU-0C-12: AgentSubprocessSupervisor

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust service object

AgentSubprocessSupervisor::spawn_agent_process(request: AgentSpawnRequest) -> Result<AgentSpawnResult, AgentRunnerError>
AgentSubprocessSupervisor::cancel(invocation_id: string) -> Result<AgentSpawnResult, AgentRunnerError>
AgentSubprocessSupervisor::wait(invocation_id: string, timeout_ms: u64) -> Result<AgentSpawnResult, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-12-agent-subprocess-supervisor.md`, `src-tauri/src/contracts/agent_subprocess_supervisor.rs`

**Code boundary:** `src-tauri/src/agent_runner/subprocess_supervisor.rs`, `src-tauri/src/contracts/agent_subprocess_supervisor.rs`, `src-tauri/tests/wu_0c_12_agent_subprocess_supervisor_contract.rs`

**Acceptance criteria:**

- [ ] Calling `spawn_agent_process(request)` with a fake success scenario returns `acceptance_state = accepted`, non-empty invocation ID, stdout ref, stderr ref, and exit status `0`.
- [ ] Calling `spawn_agent_process(request)` with nonzero fake exit returns the documented exit status and non-accepted state without throwing away stdout/stderr refs.
- [ ] Calling `wait(invocation_id, timeout_ms)` with timeout fixture returns `AgentRunnerError::TimedOut` or `acceptance_state = timed_out` according to the contract fixture.
- [ ] Calling `cancel(invocation_id)` on a running fake process returns a result with `acceptance_state = ambiguous` or documented cancelled state.
- [ ] The supervisor propagates `OULIPOLY_INVOCATION` and `OULIPOLY_PARENT_INVOCATION` env values and does not invoke the real `/home/nes/.local/bin/agents` in contract tests.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11a, WU-0C-11b, WU-0C-12a; incoming WU-0A-15 `FakeAgentsFixture`, WU-0A-10 `BackendSpanEvent`.

**Produces:** Active subprocess management operation consumed by AgentRunnerClient, provider diagnostics, orchestrator/worker later slices, and subprocess event emission.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-14, WU-0C-16, WU-0C-22.

### WU-0C-13a: SessionCaptureRequest DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

SessionCaptureRequest {
  invocation_id: string,
  stdout_ref?: string,
  stderr_ref?: string,
  trace_ref?: string,
  agent_runner_session_ref?: string
}

validate_session_capture_request(request: SessionCaptureRequest) -> Result<SessionCaptureRequest, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-13a-session-capture-request.md`, `src-tauri/src/contracts/session_capture_request.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_capture_request.rs`, `src-tauri/src/contracts/session_capture_request.rs`, `src-tauri/tests/wu_0c_13a_session_capture_request_contract.rs`

**Acceptance criteria:**

- [ ] `SessionCaptureRequest` round-trips through serde with invocation, stdout/stderr, trace, and agent-runner session refs preserved.
- [ ] Calling `validate_session_capture_request(request)` with a documented `agents` output/source combination returns the normalized request.
- [ ] Calling `validate_session_capture_request(request)` with missing source refs returns `AgentRunnerError::MissingCaptureSource` unless the fixture allows `substrate_gap`.
- [ ] DTO validation does not run transcript locator scripts, inspect provider-specific JSONL, choose provider/account routes, or compose resume commands.
- [ ] DTO validation writes no session, worker, turn, question, or recovery rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11a, WU-0C-11b; incoming WU-0A-15 `FakeAgentsFixture`.

**Produces:** Session capture request DTO consumed by WU-0C-13.

**Parallelizable with:** WU-0C-12a, WU-0C-14a, WU-0C-14b, WU-0C-16a, WU-0C-16b.

**Revision rationale:** Round 4 narrows capture inputs to observations emitted by `agents` and its trace/state surfaces. Per-CLI locator or storage-row interpretation belongs to `agent-runner` and, for override writes, to `SessionOverrideContract` adapters.

### WU-0C-13b: AgentRunnerSessionCapture DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

AgentRunnerSessionCapture {
  invocation_id: string,
  session_id?: string,
  provider_name?: string,
  capture_method: "agents_reported_session" | "trace_session" | "state_db_session" | "substrate_gap",
  source_ref: string,
  capture_state: "captured" | "missing" | "ambiguous" | "substrate_gap"
}

validate_agent_runner_session_capture(capture: AgentRunnerSessionCapture) -> Result<AgentRunnerSessionCapture, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-13b-agent-runner-session-capture.md`, `src-tauri/src/contracts/agent_runner_session_capture.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_capture_dto.rs`, `src-tauri/src/contracts/agent_runner_session_capture.rs`, `src-tauri/tests/wu_0c_13b_agent_session_capture_dto_contract.rs`

**Acceptance criteria:**

- [ ] `AgentRunnerSessionCapture` round-trips through serde with all fields preserved.
- [ ] Every capture method and capture state variant round-trips and is reachable through a documented `agents` fixture.
- [ ] Calling `validate_agent_runner_session_capture(capture)` with valid capture state/session combinations returns the normalized DTO.
- [ ] Calling `validate_agent_runner_session_capture(capture)` with `capture_state = captured` and no `session_id` returns the documented error.
- [ ] Calling `validate_agent_runner_session_capture(capture)` with `provider_name` present preserves it as observed runner evidence and does not use it to select or reroute accounts.
- [ ] DTO validation writes no WorkerRun, OrchestratorTurn, QuestionArtifact, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11b; incoming WU-0A-15 `FakeAgentsFixture`.

**Produces:** Captured session DTO consumed by WU-0C-13, WU-0C-15b, AgentRunnerClient, WorkerRun later slices, and question routing.

**Parallelizable with:** WU-0C-12a, WU-0C-14a, WU-0C-14b, WU-0C-16a, WU-0C-16b.

**Revision rationale:** Round 4 replaces Claude/Codex/opencode capture-method ownership with provider-neutral observations from `agent-runner`. Session-id generation and capture stay delegated to the `agents` binary.

### WU-0C-13: AgentSessionCapture

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust service object

AgentSessionCapture::capture_agent_runner_session(request: SessionCaptureRequest) -> Result<AgentRunnerSessionCapture, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-13-agent-session-capture.md`, `src-tauri/src/contracts/agent_session_capture.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_capture.rs`, `src-tauri/src/contracts/agent_session_capture.rs`, `src-tauri/tests/wu_0c_13_agent_session_capture_contract.rs`

**Acceptance criteria:**

- [ ] Calling `capture_agent_runner_session(request)` for a fake `agents` reported-session fixture returns non-empty `session_id` and `capture_method = agents_reported_session`.
- [ ] Calling `capture_agent_runner_session(request)` for a fake trace-session fixture returns non-empty `session_id`, optional `provider_name`, and `capture_method = trace_session`.
- [ ] Calling `capture_agent_runner_session(request)` for a missing adapter fixture returns `capture_state = substrate_gap` and does not fabricate a session ID.
- [ ] Missing stdout/stderr/trace/state inputs return `AgentRunnerError::MissingCaptureSource` unless the documented fixture allows `substrate_gap`.
- [ ] Capture does not locate, parse, append, truncate, or replace provider transcript files.
- [ ] Capture writes no WorkerRun, OrchestratorTurn, QuestionArtifact, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11a, WU-0C-11b, WU-0C-13a, WU-0C-13b; incoming WU-0A-15 `FakeAgentsFixture`, WU-0B-20 `ToolCallProvenance` only as future consumer refs.

**Produces:** Per-CLI session capture contract consumed by AgentRunnerClient, WorkerRun later slices, question routing, and trace readers.

**Parallelizable with:** WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-15a, WU-0C-15b, WU-0C-16.

**Revision rationale:** Round 4 keeps this WU as a passive session-correlation reader over `agents` evidence. It no longer encodes CLI-specific capture mechanisms or transcript lookup behavior; those are `agent-runner` responsibilities and override writes use WU-0C-N1..WU-0C-N5.

### WU-0C-14a: AgentRunnerTraceEdge DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

AgentRunnerTraceEdge {
  invocation_id: string,
  parent_invocation_id?: string,
  session_id?: string,
  evidence_ref: string
}

validate_agent_runner_trace_edge(edge: AgentRunnerTraceEdge) -> Result<AgentRunnerTraceEdge, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-14a-agent-runner-trace-edge.md`, `src-tauri/src/contracts/agent_runner_trace_edge.rs`

**Code boundary:** `src-tauri/src/agent_runner/trace_edge.rs`, `src-tauri/src/contracts/agent_runner_trace_edge.rs`, `src-tauri/tests/wu_0c_14a_trace_edge_contract.rs`

**Acceptance criteria:**

- [ ] `AgentRunnerTraceEdge` round-trips through serde with invocation, parent, session, and evidence refs preserved.
- [ ] Calling `validate_agent_runner_trace_edge(edge)` with valid refs returns the normalized edge.
- [ ] Calling `validate_agent_runner_trace_edge(edge)` with empty invocation or evidence ref returns the documented error.
- [ ] DTO validation treats trace refs as evidence/substrate only and writes no graph or worker rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11b; incoming WU-0B-09 `EvidenceArtifact`.

**Produces:** Trace edge DTO consumed by WU-0C-14b and WU-0C-14.

**Parallelizable with:** WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-16a, WU-0C-16b.

### WU-0C-14b: AgentRunnerTraceTree DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

AgentRunnerTraceTree {
  root_invocation_id: string,
  invocation_edges: AgentRunnerTraceEdge[],
  evidence_ref: string
}

validate_agent_runner_trace_tree(tree: AgentRunnerTraceTree) -> Result<AgentRunnerTraceTree, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-14b-agent-runner-trace-tree.md`, `src-tauri/src/contracts/agent_runner_trace_tree.rs`

**Code boundary:** `src-tauri/src/agent_runner/trace_tree_dto.rs`, `src-tauri/src/contracts/agent_runner_trace_tree.rs`, `src-tauri/tests/wu_0c_14b_trace_tree_contract.rs`

**Acceptance criteria:**

- [ ] `AgentRunnerTraceTree` round-trips through serde with root invocation, edge list, and evidence ref preserved.
- [ ] Calling `validate_agent_runner_trace_tree(tree)` with a connected parent-before-child fixture returns the normalized tree.
- [ ] Calling `validate_agent_runner_trace_tree(tree)` with a missing root edge or cycle returns the documented parse/validation error.
- [ ] DTO validation writes no GraphNode, WorkerRun, OrchestratorTurn, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-14a; incoming WU-0B-09 `EvidenceArtifact`.

**Produces:** Trace tree DTO consumed by WU-0C-14, AgentRunnerClient, worker board later surfaces, recovery, and subprocess event emission.

**Parallelizable with:** WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-16a, WU-0C-16b.

### WU-0C-14: AgentTraceTreeReader

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust service object

AgentTraceTreeReader::read_trace_tree(root_invocation_id: string) -> Result<AgentRunnerTraceTree, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-14-agent-trace-tree-reader.md`, `src-tauri/src/contracts/agent_trace_tree_reader.rs`

**Code boundary:** `src-tauri/src/agent_runner/trace_tree_reader.rs`, `src-tauri/src/contracts/agent_trace_tree_reader.rs`, `src-tauri/tests/wu_0c_14_agent_trace_tree_reader_contract.rs`

**Acceptance criteria:**

- [ ] Calling `read_trace_tree(root_invocation_id)` with a fake `agents trace --json` fixture returns every invocation edge in deterministic parent-before-child order.
- [ ] Calling `read_trace_tree(root_invocation_id)` with a trace containing nested `OULIPOLY_PARENT_INVOCATION` refs preserves the parent-child tree exactly.
- [ ] Calling `read_trace_tree(root_invocation_id)` with malformed JSON returns `AgentRunnerError::TraceParseFailed`.
- [ ] Calling `read_trace_tree(root_invocation_id)` with an unknown root returns `AgentRunnerError::TraceNotFound`.
- [ ] Trace trees are treated as evidence/substrate only; the reader does not mutate GraphNode, WorkerRun, OrchestratorTurn, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11b, WU-0C-14a, WU-0C-14b; incoming WU-0B-09 `EvidenceArtifact`.

**Produces:** Passive trace tree reader consumed by AgentRunnerClient, worker board later surfaces, recovery, and subprocess event emission.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-16, WU-0C-22.

### WU-0C-15a: SessionTurnRef DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

SessionTurnRef {
  turn_id: string,
  role: "user" | "assistant" | "tool" | "system",
  message_ref: string,
  tool_call_refs: string[],
  evidence_ref: string,
  source_offset?: string,
  source_hash?: string
}

validate_session_turn_ref(turn: SessionTurnRef) -> Result<SessionTurnRef, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-15a-session-turn-ref.md`, `src-tauri/src/contracts/session_turn_ref.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_turn_ref.rs`, `src-tauri/src/contracts/session_turn_ref.rs`, `src-tauri/tests/wu_0c_15a_session_turn_ref_contract.rs`

**Acceptance criteria:**

- [ ] `SessionTurnRef` round-trips through serde with turn, role, message, tool-call, evidence, source offset, and source hash refs preserved.
- [ ] Every role variant round-trips and is reachable through a documented transcript fixture.
- [ ] Calling `validate_session_turn_ref(turn)` with valid refs returns the normalized turn ref.
- [ ] Calling `validate_session_turn_ref(turn)` with empty turn ID, message ref, or evidence ref returns the documented error.
- [ ] DTO validation treats source offsets and hashes as opaque evidence and does not interpret Claude, Codex, opencode, or future CLI record formats.
- [ ] DTO validation creates no ToolCallProvenance, EvidenceArtifact, WorkerRun, OrchestratorTurn, or QuestionArtifact rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-09 `EvidenceArtifact`, WU-0B-20 `ToolCallProvenance`.

**Produces:** Session turn reference DTO consumed by WU-0C-15c, tool-provenance later slices, worker evidence surfaces, and recovery.

**Parallelizable with:** WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-13, WU-0C-16.

**Revision rationale:** Round 4 keeps `SessionTurnRef` as a normalized evidence pointer. Provider-native transcript details move to `TranscriptTurn` / `SessionLocation` in WU-0C-N2 and are read or written only through `SessionOverrideContract` adapters.

### WU-0C-15b: SessionTurnsRequest DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

SessionTurnsRequest {
  invocation_id: string,
  session_id?: string,
  provider_name?: string,
  trace_ref?: string
}

validate_session_turns_request(request: SessionTurnsRequest) -> Result<SessionTurnsRequest, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-15b-session-turns-request.md`, `src-tauri/src/contracts/session_turns_request.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_turns_request.rs`, `src-tauri/src/contracts/session_turns_request.rs`, `src-tauri/tests/wu_0c_15b_session_turns_request_contract.rs`

**Acceptance criteria:**

- [ ] `SessionTurnsRequest` round-trips through serde with invocation, session, provider, and trace refs preserved.
- [ ] Calling `validate_session_turns_request(request)` with valid invocation/session inputs returns the normalized request.
- [ ] Calling `validate_session_turns_request(request)` with missing trace/session evidence returns a documented missing-source state unless the fixture allows a substrate gap.
- [ ] DTO validation does not accept raw transcript paths, locator scripts, or provider-native JSONL records as input.
- [ ] DTO validation writes no transcript, evidence, tool provenance, worker, turn, question, or recovery rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-13b; incoming WU-0B-09 `EvidenceArtifact`.

**Produces:** Session turns request DTO consumed by WU-0C-15d and AgentRunnerClient.

**Parallelizable with:** WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-15a, WU-0C-16.

**Revision rationale:** Round 4 removes direct transcript-locator input from the general session-turn reader. Raw transcript location is owned by `agent-runner` or by the v1 override adapter behind WU-0C-N1.

### WU-0C-15c: SessionTurnsRead DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

SessionTurnsRead {
  invocation_id: string,
  session_id?: string,
  turn_refs: SessionTurnRef[],
  provider_name?: string,
  missing_locator_state?: string,
  substrate_gap_state?: string
}

validate_session_turns_read(read: SessionTurnsRead) -> Result<SessionTurnsRead, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-15c-session-turns-read.md`, `src-tauri/src/contracts/session_turns_read.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_turns_read.rs`, `src-tauri/src/contracts/session_turns_read.rs`, `src-tauri/tests/wu_0c_15c_session_turns_read_contract.rs`

**Acceptance criteria:**

- [ ] `SessionTurnsRead` round-trips through serde with invocation, session, provider, turn refs, missing-locator state, and substrate-gap state preserved.
- [ ] Calling `validate_session_turns_read(read)` with documented turn refs returns the normalized DTO.
- [ ] Calling `validate_session_turns_read(read)` with `substrate_gap_state` does not require fabricated turn refs.
- [ ] Calling `validate_session_turns_read(read)` with an empty invocation ID returns the documented error.
- [ ] DTO validation does not expose raw provider transcript paths or provider-native JSONL line bodies.
- [ ] DTO validation writes no ToolCallProvenance, EvidenceArtifact, WorkerRun, OrchestratorTurn, or QuestionArtifact rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-15a, WU-0C-15b; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-20 `ToolCallProvenance`.

**Produces:** Session turns read DTO consumed by WU-0C-15d, tool provenance later slices, worker evidence surfaces, and recovery.

**Parallelizable with:** WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-16.

**Revision rationale:** Round 4 narrows this DTO to normalized turn evidence and substrate-gap reporting. It does not model per-CLI storage or replacement behavior; canonical transcript material is owned by WU-0C-N2.

### WU-0C-15d: SessionTurnsReader

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust service object

SessionTurnsReader::read_session_turns(request: SessionTurnsRequest) -> Result<SessionTurnsRead, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-15d-session-turns-reader.md`, `src-tauri/src/contracts/session_turns_reader.rs`

**Code boundary:** `src-tauri/src/agent_runner/session_turns_reader.rs`, `src-tauri/src/contracts/session_turns_reader.rs`, `src-tauri/tests/wu_0c_15d_session_turns_reader_contract.rs`

**Acceptance criteria:**

- [ ] Calling `read_session_turns(request)` for a fake `agents` normalized session-turn fixture returns stable turn refs with evidence refs and source hashes.
- [ ] Calling `read_session_turns(request)` for a fixture with parent/sidechain/compaction metadata preserves that metadata through opaque refs without parsing provider-native JSONL.
- [ ] Calling `read_session_turns(request)` for missing transcript-location evidence returns explicit `missing_locator_state` rather than failing silently.
- [ ] Calling `read_session_turns(request)` for a substrate-gap fixture records explicit `substrate_gap_state` and does not fabricate turn refs.
- [ ] The reader is read-only and does not create ToolCallProvenance, EvidenceArtifact, WorkerRun, OrchestratorTurn, QuestionArtifact, or SessionOverrideStore rows.
- [ ] The reader never calls `replace_transcript`, `truncate_after`, or `append_turns`; session mutation is exclusive to WU-0C-N1 implementers.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-13, WU-0C-15a, WU-0C-15b, WU-0C-15c; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-20 `ToolCallProvenance`.

**Produces:** Read-only session turn ingestion contract consumed by tool provenance later slices, worker evidence surfaces, and recovery.

**Parallelizable with:** WU-0C-04, WU-0C-07, WU-0C-10, WU-0C-16.

**Revision rationale:** Round 4 removes research-13-era per-CLI transcript parsing from the general session-turn reader. It now consumes normalized `agent-runner` evidence only; any raw transcript read/write path is centralized under `SessionOverrideContract`.

### WU-0C-16a: ConfigSnapshotRequest DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

ConfigSnapshotRequest {
  workspace_id: string,
  config_roots: string[],
  include_models: boolean,
  include_agents: boolean
}

validate_config_snapshot_request(request: ConfigSnapshotRequest) -> Result<ConfigSnapshotRequest, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-16a-config-snapshot-request.md`, `src-tauri/src/contracts/config_snapshot_request.rs`

**Code boundary:** `src-tauri/src/agent_runner/config_snapshot_request.rs`, `src-tauri/src/contracts/config_snapshot_request.rs`, `src-tauri/tests/wu_0c_16a_config_snapshot_request_contract.rs`

**Acceptance criteria:**

- [ ] `ConfigSnapshotRequest` round-trips through serde with workspace, roots, include-models, and include-agents fields preserved.
- [ ] Calling `validate_config_snapshot_request(request)` with valid roots returns the normalized request.
- [ ] Calling `validate_config_snapshot_request(request)` with path escape attempts returns `AgentRunnerError::ConfigPathRejected`.
- [ ] DTO validation reads no vendor credentials and writes no EvidenceArtifact or ProviderState rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0A-02 `HarnessSettings`, WU-0A-03 `LocalStorageLayout`.

**Produces:** Config snapshot request DTO consumed by WU-0C-16.

**Parallelizable with:** WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-14a, WU-0C-14b.

### WU-0C-16b: AgentRunnerConfigSnapshot DTO

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust struct

AgentRunnerConfigSnapshot {
  providers_toml_ref?: string,
  sessions_toml_ref?: string,
  models_config_ref?: string,
  agents_config_ref?: string,
  redacted_fields: string[]
}

validate_agent_runner_config_snapshot(snapshot: AgentRunnerConfigSnapshot) -> Result<AgentRunnerConfigSnapshot, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-16b-agent-runner-config-snapshot.md`, `src-tauri/src/contracts/agent_runner_config_snapshot.rs`

**Code boundary:** `src-tauri/src/agent_runner/config_snapshot_dto.rs`, `src-tauri/src/contracts/agent_runner_config_snapshot.rs`, `src-tauri/tests/wu_0c_16b_config_snapshot_dto_contract.rs`

**Acceptance criteria:**

- [ ] `AgentRunnerConfigSnapshot` round-trips through serde with all optional refs and redacted fields preserved.
- [ ] Calling `validate_agent_runner_config_snapshot(snapshot)` with redaction refs returns the normalized DTO.
- [ ] Redaction fixture credentials are absent from persisted refs while `redacted_fields` names every removed key.
- [ ] DTO validation writes no ProviderState rows and stores no credential material.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-09 `EvidenceArtifact`.

**Produces:** Redacted config snapshot DTO consumed by WU-0C-16, WU-0C-17a, ProviderStateMonitor, and AgentRunnerClient.

**Parallelizable with:** WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-14a, WU-0C-14b.

### WU-0C-16: AgentRunnerConfigSnapshotReader

**Parent initiative:** CLI subprocess supervisor around `/home/nes/.local/bin/agents`

**Contract:**
```text
schema_object: Rust service object

AgentRunnerConfigSnapshotReader::read_agent_runner_config_snapshot(request: ConfigSnapshotRequest) -> Result<AgentRunnerConfigSnapshot, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-16-agent-runner-config-snapshot-reader.md`, `src-tauri/src/contracts/agent_runner_config_snapshot.rs`

**Code boundary:** `src-tauri/src/agent_runner/config_snapshot.rs`, `src-tauri/src/contracts/agent_runner_config_snapshot.rs`, `src-tauri/tests/wu_0c_16_config_snapshot_contract.rs`

**Acceptance criteria:**

- [ ] Calling `read_agent_runner_config_snapshot(request)` captures providers.toml, sessions.toml, model configuration, and agents configuration refs when fixture files exist.
- [ ] Calling `read_agent_runner_config_snapshot(request)` with missing optional files returns a snapshot with missing refs omitted and no error unless the root is invalid.
- [ ] Redaction fixture credentials are absent from persisted refs while `redacted_fields` names every removed key.
- [ ] Path escape attempts in `config_roots` return `AgentRunnerError::ConfigPathRejected`.
- [ ] The reader stores no credential material and does not claim ownership of vendor auth stores.
- [ ] The reader writes no ProviderState rows; provider interpretation is owned by WU-0C-20.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-16a, WU-0C-16b; incoming WU-0A-02 `HarnessSettings`, WU-0A-03 `LocalStorageLayout`, WU-0B-09 `EvidenceArtifact`.

**Produces:** Redacted config snapshot consumed by provider diagnostics, ProviderStateMonitor, and AgentRunnerClient.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-22.

### WU-0C-17a: ProviderDiagnosticsRequest DTO

**Parent initiative:** CLI subprocess supervisor and provider diagnostics

**Contract:**
```text
schema_object: Rust struct

ProviderDiagnosticsRequest {
  workspace_id: string,
  provider_state_id?: string,
  config_snapshot_ref?: string
}

validate_provider_diagnostics_request(request: ProviderDiagnosticsRequest) -> Result<ProviderDiagnosticsRequest, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-17a-provider-diagnostics-request.md`, `src-tauri/src/contracts/provider_diagnostics_request.rs`

**Code boundary:** `src-tauri/src/agent_runner/provider_diagnostics_request.rs`, `src-tauri/src/contracts/provider_diagnostics_request.rs`, `src-tauri/tests/wu_0c_17a_provider_diagnostics_request_contract.rs`

**Acceptance criteria:**

- [ ] `ProviderDiagnosticsRequest` round-trips through serde with workspace, provider state, and config snapshot refs preserved.
- [ ] Calling `validate_provider_diagnostics_request(request)` with valid refs returns the normalized request.
- [ ] Calling `validate_provider_diagnostics_request(request)` with empty workspace ID returns the documented error.
- [ ] DTO validation mutates no ProviderState, EntitlementSnapshot, CapabilityFingerprint, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-16b; incoming WU-0B-17 through WU-0B-19 provider/capability records.

**Produces:** Provider diagnostics request DTO consumed by WU-0C-17.

**Parallelizable with:** WU-0C-23a, WU-0C-23b, WU-0C-24 after shared prerequisites are met.

### WU-0C-17b: ProviderDiagnosticsResult DTO

**Parent initiative:** CLI subprocess supervisor and provider diagnostics

**Contract:**
```text
schema_object: Rust struct

ProviderDiagnosticsResult {
  provider_state_id?: string,
  capability_fingerprint_id?: string,
  diagnostics_ref: string,
  route_denial_reasons: string[]
}

validate_provider_diagnostics_result(result: ProviderDiagnosticsResult) -> Result<ProviderDiagnosticsResult, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-17b-provider-diagnostics-result.md`, `src-tauri/src/contracts/provider_diagnostics_result.rs`

**Code boundary:** `src-tauri/src/agent_runner/provider_diagnostics_result.rs`, `src-tauri/src/contracts/provider_diagnostics_result.rs`, `src-tauri/tests/wu_0c_17b_provider_diagnostics_result_contract.rs`

**Acceptance criteria:**

- [ ] `ProviderDiagnosticsResult` round-trips through serde with provider state, capability fingerprint, diagnostics ref, and denial reasons preserved.
- [ ] Calling `validate_provider_diagnostics_result(result)` with valid diagnostics returns the normalized result.
- [ ] Route denial reason taxonomy values are preserved exactly; unknown values are retained as unknown observations.
- [ ] DTO validation mutates no ProviderState, EntitlementSnapshot, CapabilityFingerprint, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-17 through WU-0B-19 provider/capability records.

**Produces:** Provider diagnostics result DTO consumed by WU-0C-17 and ProviderStateMonitor.

**Parallelizable with:** WU-0C-23a, WU-0C-23b, WU-0C-24 after shared prerequisites are met.

### WU-0C-17: ProviderDiagnosticsAdapter

**Parent initiative:** CLI subprocess supervisor and provider diagnostics

**Contract:**
```text
schema_object: Rust service object

ProviderDiagnosticsAdapter::read_provider_diagnostics(request: ProviderDiagnosticsRequest) -> Result<ProviderDiagnosticsResult, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-17-provider-diagnostics-adapter.md`, `src-tauri/src/contracts/provider_diagnostics_adapter.rs`

**Code boundary:** `src-tauri/src/agent_runner/provider_diagnostics.rs`, `src-tauri/src/contracts/provider_diagnostics_adapter.rs`, `src-tauri/tests/wu_0c_17_provider_diagnostics_contract.rs`

**Acceptance criteria:**

- [ ] Calling `read_provider_diagnostics(request)` with valid fake diagnostics returns a diagnostics ref and preserves route denial reasons.
- [ ] Calling `read_provider_diagnostics(request)` with unknown provider state returns `AgentRunnerError::UnknownProviderState`.
- [ ] Calling `read_provider_diagnostics(request)` with malformed diagnostics fixture returns `AgentRunnerError::DiagnosticsParseFailed`.
- [ ] Route denial reason taxonomy values are preserved exactly; unknown values are returned as unknown observations, not silently mapped to eligible.
- [ ] Diagnostics are substrate observations only; the adapter does not mutate ProviderState, EntitlementSnapshot, CapabilityFingerprint, or RecoveryAction rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-12, WU-0C-13, WU-0C-14, WU-0C-15d, WU-0C-16, WU-0C-17a, WU-0C-17b; incoming WU-0B-17 through WU-0B-19 provider/capability records.

**Produces:** Provider diagnostics read operation consumed by AgentRunnerClient and ProviderStateMonitor.

**Parallelizable with:** WU-0C-23, WU-0C-24 after shared prerequisites are met.

### WU-0C-18: AgentRunnerClient Facade

**Parent initiative:** Unified agent-runner wrapper API

**Contract:**
```text
schema_object: Rust service object

AgentRunnerClient::spawn(request: AgentSpawnRequest) -> Result<AgentSpawnResult, AgentRunnerError>
AgentRunnerClient::capture_session(request: SessionCaptureRequest) -> Result<AgentRunnerSessionCapture, AgentRunnerError>
AgentRunnerClient::read_trace_tree(root_invocation_id: string) -> Result<AgentRunnerTraceTree, AgentRunnerError>
AgentRunnerClient::read_session_turns(request: SessionTurnsRequest) -> Result<SessionTurnsRead, AgentRunnerError>
AgentRunnerClient::read_config_snapshot(request: ConfigSnapshotRequest) -> Result<AgentRunnerConfigSnapshot, AgentRunnerError>
AgentRunnerClient::provider_diagnostics(request: ProviderDiagnosticsRequest) -> Result<ProviderDiagnosticsResult, AgentRunnerError>
AgentRunnerClient::cancel(invocation_id: string) -> Result<AgentSpawnResult, AgentRunnerError>
AgentRunnerClient::resume(invocation_id: string, answer_payload_ref: string) -> Result<AgentSpawnResult, AgentRunnerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-18-agent-runner-client-facade.md`, `src-tauri/src/contracts/agent_runner_client.rs`

**Code boundary:** `src-tauri/src/agent_runner/client.rs`, `src-tauri/src/contracts/agent_runner_client.rs`, `src-tauri/tests/wu_0c_18_agent_runner_client_contract.rs`

**Acceptance criteria:**

- [ ] Calling `spawn(request)` delegates to WU-0C-12 and returns the documented `AgentSpawnResult`.
- [ ] Calling `capture_session(request)` delegates to WU-0C-13 and returns the documented `AgentRunnerSessionCapture`.
- [ ] Calling `read_trace_tree(root_invocation_id)` delegates to WU-0C-14 and returns the documented trace tree.
- [ ] Calling `read_session_turns(request)` delegates to WU-0C-15d and returns the documented session read result.
- [ ] Calling `read_config_snapshot(request)` delegates to WU-0C-16 and returns the documented config snapshot.
- [ ] Calling `provider_diagnostics(request)` delegates to WU-0C-17 and returns the documented diagnostics result.
- [ ] Calling `cancel(invocation_id)` delegates to WU-0C-12 and preserves stdout/stderr refs.
- [ ] Calling `resume(invocation_id, answer_payload_ref)` returns a spawn result or documented resume error without writing QuestionArtifact or RecoveryAction rows in Phase 0C.
- [ ] The facade exposes no method that opens, rewrites, truncates, appends, or migrates provider transcript files; consumers that need session write-back depend on WU-0C-N1 instead.
- [ ] The facade does not choose provider accounts, rewrite `providers.toml` or `sessions.toml`, mutate quota scripts, or implement cross-provider session porting.
- [ ] Every facade method has a fake-backed success fixture and a documented invalid-input/error fixture.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-12, WU-0C-13, WU-0C-14, WU-0C-15d, WU-0C-16, WU-0C-17.

**Produces:** Single wrapper API for active subprocess management, passive trace/state/config reads, provider diagnostics, resume attempts, and test fakes.

**Parallelizable with:** none in the agent-runner family; it is the join point for WU-0C-12 through WU-0C-17.

**Revision rationale:** Round 4 keeps `AgentRunnerClient` as the subprocess/trace/config/diagnostics facade only. Session transcript write-back moved to WU-0C-N1..WU-0C-N5, and provider routing/resume/session porting remain delegated to `agent-runner`.

### WU-0C-N2: TranscriptTurn, SessionLocation, and SessionMetadata DTOs

**Parent initiative:** SessionOverrideContract foundation

**Contract:**
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

**Test boundary:** `product-strategy/contracts/wu-0c-n2-session-override-dtos.md`, `src-tauri/src/contracts/session_override_dtos.rs`, `src/contracts/session-override-dtos.ts`

**Code boundary:** `src-tauri/src/session_override/dtos.rs`, `src-tauri/src/contracts/session_override_dtos.rs`, `src/contracts/session-override-dtos.ts`, `src-tauri/tests/wu_0c_n2_session_override_dtos_contract.rs`, `src/test/session-override-dtos.test.ts`

**Acceptance criteria:**

- [ ] `TranscriptTurn`, `SessionLocation`, `SessionMetadata`, `OverridePreconditions`, `OverrideReceipt`, and `SessionOverrideError` round-trip between Rust JSON fixtures and TypeScript DTO fixtures without field loss.
- [ ] Every role, storage-kind, mutability, transcript-state, compaction-state, operation, and error variant is reachable through a named fixture.
- [ ] `TranscriptTurn` validation rejects empty `turn_id`, missing `content_ref`, missing `source_hash`, negative/duplicate `turn_index`, and unsupported role strings with documented errors.
- [ ] `SessionLocation` validation rejects `mutability = idle_writable` without a storage kind that the active adapter declares writable.
- [ ] `OverridePreconditions` validation rejects empty `expected_preimage_hash` and permits missing graph/segment refs only for documented read-only operations.
- [ ] DTO validation treats `provider_native_kind`, `source_offsets`, and `transcript_path` as adapter-owned evidence and never parses provider JSONL record bodies.
- [ ] DTO validation writes no transcript files, state.db rows, EvidenceArtifact rows, AuditEvent rows, GraphNode rows, or SessionOverrideStore registry rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-21 `WorkingSetSnapshot`.

**Produces:** Canonical session-override DTOs consumed by WU-0C-N1, WU-0C-N3, WU-0C-N4, WU-0C-N5, VS-010, VS-012, VS-018, VS-020, and VS-021.

**Parallelizable with:** WU-0C-11a, WU-0C-15a, WU-0C-16a, WU-0C-19, WU-0C-21 after shared 0A/0B dependencies exist.

**Single-concern PR constraint:** This WU may add only DTOs, validators, fixture JSON, and generated TypeScript types. It may not add adapter file/DB access, trait methods, IPC commands, or registry persistence.

**Blocked-on:** None for the DTO contract. v2 population fields are placeholders until `agents session locate/export/import-replace`, pause-handshake, and schema-version probe land.

### WU-0C-N1: SessionOverrideContract Trait

**Parent initiative:** SessionOverrideContract foundation

**Contract:**
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

**Test boundary:** `product-strategy/contracts/wu-0c-n1-session-override-contract.md`, `src-tauri/src/contracts/session_override_contract.rs`, `src/contracts/session-override-contract.ts`

**Code boundary:** `src-tauri/src/session_override/contract.rs`, `src-tauri/src/contracts/session_override_contract.rs`, `src/contracts/session-override-contract.ts`, `src-tauri/tests/wu_0c_n1_session_override_contract.rs`, `src/test/session-override-contract.test.ts`

**Acceptance criteria:**

- [ ] The Rust trait and TypeScript interface expose exactly the seven operations listed in the contract with the documented argument and result DTOs.
- [ ] A fake in-memory adapter can return success for `locate_session`, `read_transcript`, `get_session_metadata`, and each write operation using WU-0C-N2 DTO fixtures.
- [ ] A fake adapter error fixture maps each `SessionOverrideError` variant to a stable Rust enum discriminant and TypeScript string without lossy catch-all mapping.
- [ ] `replace_transcript`, `truncate_after`, and `append_turns` test fixtures prove `schema_version_probe()` is invoked before mutation by failing the operation when the fake probe returns `UnsupportedSchema`.
- [ ] Write methods reject missing `OverridePreconditions` and return `PreimageMismatch` or `SessionBusy` before invoking the fake mutation hook when the preconditions fail.
- [ ] `read_transcript` returns `Vec<TranscriptTurn>` and does not expose raw mutable file descriptors, SQLite connections, or provider-specific parser objects.
- [ ] The trait module contains no provider routing, quota/account selection, auth refresh, resume composition, cross-provider porting, or session-id generation logic.
- [ ] The TypeScript interface is usable by tests without importing Tauri command-router modules.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-N2; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`.

**Produces:** Versioned session write-back trait consumed by WU-0C-N3, WU-0C-N5, VS-010 turn-decomposition/detail-injection, VS-012 repack planner, VS-018 worker-output reintegration, VS-020 recovery, and VS-021 reroute governance.

**Parallelizable with:** WU-0C-12, WU-0C-13, WU-0C-14, WU-0C-16 after WU-0C-N2 lands.

**Single-concern PR constraint:** This WU may add only the trait/interface, fake adapter, and trait-level tests. It may not implement `state.db` access, JSONL rendering, crash recovery, registry persistence, or IPC routes.

**Blocked-on:** None for v1 trait acceptance. v2 adapter migration is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe.

### WU-0C-N4: AgentRunnerSchemaProbe

**Parent initiative:** SessionOverrideContract v1 compatibility probe

**Contract:**
```text
schema_object: Rust service object

SchemaProbe {
  agents_binary_ref: string,
  agents_version_or_commit: string,
  state_db_path: string,
  supported: boolean,
  supported_range: string,
  table_fingerprints: map<string, string>,
  storage_capabilities: map<string, "read_write" | "read_only" | "unsupported">,
  refusal_reason?: SessionOverrideError
}

AgentRunnerSchemaProbe::probe(state_db_path: string, agents_binary: string) -> Result<SchemaProbe, SessionOverrideError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-n4-agent-runner-schema-probe.md`, `src-tauri/src/contracts/agent_runner_schema_probe.rs`

**Code boundary:** `src-tauri/src/session_override/schema_probe.rs`, `src-tauri/src/contracts/agent_runner_schema_probe.rs`, `src-tauri/tests/wu_0c_n4_agent_runner_schema_probe_contract.rs`

**Acceptance criteria:**

- [ ] Calling `probe(state_db_path, agents_binary)` against a fixture with `invocations`, `session_turns`, `session_chains`, and `session_chain_segments` matching the pinned range returns `supported = true`.
- [ ] The probe validates required columns for `invocations.session_id`, `invocations.session_capture_method`, `invocations.resume_acceptance_status`, `session_turns.provider_name`, `session_turns.session_id`, `session_turns.turn_id`, `session_turns.parent_turn_id`, `session_turns.is_sidechain`, `session_turns.is_compaction_boundary`, `session_turns.source_file`, `session_chains.chain_id`, and `session_chain_segments.ended_at`.
- [ ] Missing required tables, missing required columns, incompatible indexes, or unreadable DB files return `SessionOverrideError::UnsupportedSchema` or `DbFailure` before any write-capability is reported.
- [ ] Unknown or unparseable `agents_binary` version/commit returns `UnsupportedSchema` unless the fixture explicitly marks a test-only fake binary.
- [ ] The probe reports `claude_code_jsonl` and `codex_jsonl` storage as `read_write`, `read_only`, or `unsupported` only from the pinned fixture capability table; it does not infer support heuristically from path names.
- [ ] Probe output includes deterministic table fingerprints for all required tables and indexes so audit records can cite the checked surface.
- [ ] The service opens SQLite read-only and performs no schema migration, `PRAGMA user_version` update, table creation, transcript write, or agent-runner config edit.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-N2; incoming WU-0A-03 `LocalStorageLayout`, WU-0A-15 `FakeAgentsFixture`.

**Produces:** Runtime compatibility probe consumed by WU-0C-N3 and v1 refusal paths in WU-0C-N1 tests.

**Parallelizable with:** WU-0C-12a, WU-0C-13a, WU-0C-14a, WU-0C-16b after WU-0C-N2 lands.

**Single-concern PR constraint:** This WU may read fixture SQLite metadata and binary-version evidence only. It may not locate sessions, parse transcript bodies, mutate state, or write override registry records.

**Blocked-on:** v1 ships with local table/binary probing. Replacement by an upstream supported-surface probe is blocked on `agents schema-version probe` or equivalent.

### WU-0C-N3: AgentRunnerDbAdapter v1

**Parent initiative:** SessionOverrideContract v1 adapter

**Contract:**
```text
schema_object: Rust service object implementing SessionOverrideContract

AgentRunnerDbAdapter::new(state_db_path, sessions_config_ref, agents_binary, lock_root) -> AgentRunnerDbAdapter

Implements:
- schema_version_probe
- locate_session
- read_transcript
- replace_transcript
- truncate_after
- append_turns
- get_session_metadata

v1 write protocol:
- refuse outside pinned SchemaProbe supported range
- locate exactly one active session/provider/segment
- prove session idle by lock + stable mtime + SQLite non-busy observations
- write same-directory temp JSONL and fsync where available
- record pending override before rename
- atomic rename final transcript
- update minimum state rows in one SQLite transaction
- commit or quarantine pending override after crash recovery
```

**Test boundary:** `product-strategy/contracts/wu-0c-n3-agent-runner-db-adapter.md`, `src-tauri/src/contracts/agent_runner_db_adapter.rs`

**Code boundary:** `src-tauri/src/session_override/agent_runner_db_adapter.rs`, `src-tauri/src/session_override/jsonl_render.rs`, `src-tauri/src/contracts/agent_runner_db_adapter.rs`, `src-tauri/tests/wu_0c_n3_agent_runner_db_adapter_contract.rs`

**Acceptance criteria:**

- [ ] `locate_session(session_id)` reads fixture `state.db` rows plus configured transcript locator output and returns exactly one `SessionLocation` for a supported session.
- [ ] `locate_session(session_id)` returns `SessionNotFound`, `AmbiguousSession`, or `UnsupportedStorage` for missing, duplicate, or unsupported fixtures and performs no file mutation.
- [ ] `read_transcript(session_id)` returns ordered `TranscriptTurn` fixtures with source offsets and hashes preserved for supported plaintext JSONL sessions; malformed records become `unsupported_record` turns rather than being dropped.
- [ ] `replace_transcript(session_id, new_jsonl, preconditions)` refuses on unsupported schema, busy session, preimage mismatch, unsupported storage, or adapter render failure before renaming any file.
- [ ] Successful `replace_transcript` writes a same-directory temp file, records a pending override, atomically renames it, updates only required session-turn/chain consistency rows, and returns an `OverrideReceipt` with preimage/postimage hashes.
- [ ] `truncate_after(session_id, turn_index, preconditions)` truncates only at a valid turn boundary and rejects boundaries that split a tool-call/result dependency or compaction-boundary invariant.
- [ ] `append_turns(session_id, turns, preconditions)` appends only adapter-renderable `TranscriptTurn` records, rejects duplicate turn IDs, and refuses unsupported append cases before file mutation.
- [ ] Crash-injection fixtures after temp write, after rename, and after DB transaction recover deterministically to committed, rolled-back, or `QuarantinedStorageConflict` states.
- [ ] The adapter uses flock-style/session-idle locking and returns `SessionBusy` when it cannot prove no in-flight `agents` write owns the same session.
- [ ] The adapter never edits `agents`, `providers.toml`, `sessions.toml`, model TOMLs, auth stores, quota scripts, provider credentials, provider routing policy, or cross-provider migration settings.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16; incoming WU-0A-03 `LocalStorageLayout`, WU-0A-15 `FakeAgentsFixture`, WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`.

**Produces:** Shipping v1 `SessionOverrideContract` implementation consumed by VS-010, VS-012, VS-018, VS-020, and VS-021 until the v2 CLI adapter lands.

**Parallelizable with:** WU-0C-17, WU-0C-20a, WU-0C-23, WU-0C-24 after WU-0C-N1/N2/N4 and WU-0C-16 land.

**Single-concern PR constraint:** This WU owns only the v1 direct DB/JSONL adapter and its fixtures. It may not add UI flows, provider routing, resume composition, worker launch behavior, optimizer behavior, or the future v2 CLI adapter.

**Blocked-on:** v1 ships now under schema-version pinning and idle-only writes. v2 swap-later is blocked on `agents session locate`, `agents session export`, `agents session import-replace`, pause-handshake, and schema-version probe. Atomic mid-session override remains blocked on `agents pause-handshake`.

### WU-0C-N5: SessionOverrideStore Registry

**Parent initiative:** SessionOverrideContract audit/override registry

**Contract:**
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

**Test boundary:** `product-strategy/contracts/wu-0c-n5-session-override-store.md`, `src-tauri/src/contracts/session_override_store.rs`

**Code boundary:** `src-tauri/src/session_override/store.rs`, `src-tauri/src/contracts/session_override_store.rs`, `src-tauri/tests/wu_0c_n5_session_override_store_contract.rs`

**Acceptance criteria:**

- [ ] `SessionOverrideRecord` round-trips through Rust serde and TypeScript fixture JSON while preserving adapter kind, operation, state, hashes, schema probe, audit, evidence, and timestamps.
- [ ] `begin_pending(record)` creates exactly one pending record for a unique `override_id` and rejects duplicate IDs.
- [ ] `commit(override_id, receipt)` transitions only `pending -> committed`, records postimage/audit/evidence refs, and rejects commit attempts for rolled-back or quarantined records.
- [ ] `rollback(override_id, reason)` transitions only `pending -> rolled_back` and preserves preimage hash plus reason evidence.
- [ ] `quarantine(override_id, reason)` transitions pending or crash-recovery records to `quarantined_storage_conflict` and requires preserved/replayed/discarded placeholder refs.
- [ ] `list_by_session(session_id)` returns records in created-at order and never exposes raw transcript contents.
- [ ] Registry writes append AuditEvent drafts through WU-0C-37-compatible refs but do not require the durable audit pipeline to call back into adapter write methods.
- [ ] The registry does not mutate transcript files, run schema probes, locate sessions, or implement adapter crash recovery by itself.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-N1, WU-0C-N2; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-31 `RecoveryAction`.

**Produces:** Workspace-level override ledger consumed by WU-0C-N3 crash recovery, VS-001 evidence inspectors, VS-003 audit surfaces, VS-020 recovery, and VS-021 reroute governance.

**Parallelizable with:** WU-0C-25, WU-0C-27, WU-0C-29d after WU-0C-N1/N2 land.

**Single-concern PR constraint:** This WU owns only the registry record/service and lifecycle transitions. It may not perform file writes, DB adapter probes, UI rendering, recovery execution, or provider routing.

**Blocked-on:** None for the v1 registry. v2 adapter-kind activation is blocked on `agents session locate/export/import-replace` and schema-version probe.

### WU-0C-19: ProviderProbeRequest DTO

**Parent initiative:** ProviderStateMonitor

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

ProviderProbeRequest {
  probe_id: string,
  workspace_id: string,
  provider: ProviderKind,
  cli: ProviderCli,
  account_ref?: string,
  requested_model?: string,
  invocation_id?: string,
  started_at: Timestamp,
  freshness_deadline: Timestamp,
  redaction_policy_ref: string
}

validate_provider_probe_request(request: ProviderProbeRequest) -> Result<ProviderProbeRequest, ProviderMonitorError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-19-provider-probe-request.md`, `src-tauri/src/contracts/provider_probe_request.rs`, `src/contracts/provider-probe-request.ts`

**Code boundary:** `src-tauri/src/provider_monitor/probe_request.rs`, `src-tauri/src/contracts/provider_probe_request.rs`, `src/contracts/provider-probe-request.ts`, `src-tauri/tests/wu_0c_19_provider_probe_request_contract.rs`, `src/test/provider-probe-request.test.ts`

**Acceptance criteria:**

- [ ] `ProviderProbeRequest` round-trips through Rust serde and TypeScript fixture JSON while preserving every field.
- [ ] Every provider and CLI variant inherited from WU-0B-17 round-trips and is reachable through a documented probe fixture.
- [ ] Calling `validate_provider_probe_request(request)` with valid input returns the normalized request.
- [ ] Calling `validate_provider_probe_request(request)` with empty workspace ID, probe ID, provider, CLI, or redaction policy returns the documented error variant.
- [ ] Calling `validate_provider_probe_request(request)` with `freshness_deadline < started_at` returns `ProviderMonitorError::InvalidFreshnessDeadline`.
- [ ] Request validation stores no vendor credentials and writes no ProviderState rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-17 `ProviderState`, WU-0B-18 `EntitlementSnapshot`, WU-0B-19 `CapabilityFingerprint`.

**Produces:** Provider probe request DTO consumed by ProviderStateMonitor and IPC command router.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-21, WU-0C-24a, WU-0C-30a.

### WU-0C-20a: ProviderStateMonitorResult DTO

**Parent initiative:** ProviderStateMonitor

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

ProviderStateMonitorResult {
  probe_id: string,
  provider_state_id: string,
  entitlement_snapshot_ids: string[],
  capability_fingerprint_ids: string[],
  audit_event_draft: AuditEventDraft,
  redacted_evidence_ref: string,
  freshness_state: "fresh" | "stale" | "probe_failed" | "manual"
}

validate_provider_state_monitor_result(result: ProviderStateMonitorResult) -> Result<ProviderStateMonitorResult, ProviderMonitorError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-20a-provider-state-monitor-result.md`, `src-tauri/src/contracts/provider_state_monitor_result.rs`, `src/contracts/provider-state-monitor-result.ts`

**Code boundary:** `src-tauri/src/provider_monitor/monitor_result.rs`, `src-tauri/src/contracts/provider_state_monitor_result.rs`, `src/contracts/provider-state-monitor-result.ts`, `src-tauri/tests/wu_0c_20a_provider_state_monitor_result_contract.rs`, `src/test/provider-state-monitor-result.test.ts`

**Acceptance criteria:**

- [ ] `ProviderStateMonitorResult` round-trips through Rust serde and TypeScript fixture JSON with probe, provider state, entitlement, capability, audit draft, evidence ref, and freshness fields preserved.
- [ ] Every freshness state variant round-trips and is reachable through a documented fixture.
- [ ] Calling `validate_provider_state_monitor_result(result)` with valid IDs returns the normalized DTO.
- [ ] Calling `validate_provider_state_monitor_result(result)` with empty probe ID, provider state ID, or evidence ref returns the documented error.
- [ ] DTO validation stores no secrets and writes no provider, entitlement, capability, evidence, or audit rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-19; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-17 `ProviderState`, WU-0B-18 `EntitlementSnapshot`, WU-0B-19 `CapabilityFingerprint`.

**Produces:** Provider state monitor result DTO consumed by WU-0C-20, RenderEngine, worker dispatch later slices, provider panel later slices, recovery, and reroute.

**Parallelizable with:** WU-0C-22a, WU-0C-23a, WU-0C-23b, WU-0C-24 after WU-0C-19 complete.

### WU-0C-20: ProviderStateMonitor

**Parent initiative:** ProviderStateMonitor

**Contract:**
```text
schema_object: Rust service object

ProviderStateMonitor::monitor_provider_state(request: ProviderProbeRequest) -> Result<ProviderStateMonitorResult, ProviderMonitorError>
ProviderStateMonitor::mark_stale(provider_state_id: string, reason: string) -> Result<ProviderState, ProviderMonitorError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-20-provider-state-monitor.md`, `src-tauri/src/contracts/provider_state_monitor.rs`

**Code boundary:** `src-tauri/src/provider_monitor/monitor.rs`, `src-tauri/src/contracts/provider_state_monitor.rs`, `src-tauri/tests/wu_0c_20_provider_state_monitor_contract.rs`

**Acceptance criteria:**

- [ ] Calling `monitor_provider_state(request)` with valid fake diagnostics writes or updates ProviderState, EntitlementSnapshot, and CapabilityFingerprint rows through their WU-0B repositories and returns their IDs.
- [ ] Calling `monitor_provider_state(request)` with provider diagnostics failure returns `ProviderMonitorError::ProbeFailed` and records `freshness_state = probe_failed` without storing secrets.
- [ ] Calling `mark_stale(provider_state_id, reason)` changes only freshness/confidence/RecordMeta fields documented by WU-0B-17.
- [ ] The monitor redacts provider config evidence and sets `secret_material_stored = false` on every ProviderState row.
- [ ] The monitor emits only an audit event draft; durable audit writing is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-18, WU-0C-19, WU-0C-20a; incoming WU-0B-09 `EvidenceArtifact`, WU-0B-15 `AuditEvent`, WU-0B-17 `ProviderState`, WU-0B-18 `EntitlementSnapshot`, WU-0B-19 `CapabilityFingerprint`.

**Produces:** Provider state monitoring service consumed by RenderEngine, worker dispatch later slices, provider panel later slices, recovery, and reroute.

**Parallelizable with:** WU-0C-23, WU-0C-24 after WU-0C-18 and WU-0C-19 complete.

### WU-0C-21: RenderRequestDto

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

RenderRequestDto {
  workspace_id: string,
  graph_snapshot_id: string,
  agent_walk_state_id: string,
  configuration_id: string,
  target_actor: "orchestrator" | "worker",
  target_cli: "claude" | "codex" | "opencode",
  target_model: string,
  render_policy_id: string,
  policy_set_id: string
}

validate_render_request(request: RenderRequestDto) -> Result<RenderRequestDto, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-21-render-request-dto.md`, `src-tauri/src/contracts/render_request.rs`, `src/contracts/render-request.ts`

**Code boundary:** `src-tauri/src/render/render_request.rs`, `src-tauri/src/contracts/render_request.rs`, `src/contracts/render-request.ts`, `src-tauri/tests/wu_0c_21_render_request_contract.rs`, `src/test/render-request.test.ts`

**Acceptance criteria:**

- [ ] `RenderRequestDto` round-trips through Rust serde and TypeScript fixture JSON with every field preserved.
- [ ] Every `target_actor` and `target_cli` variant round-trips and is reachable through a documented fixture.
- [ ] Calling `validate_render_request(request)` with valid refs returns the normalized request.
- [ ] Calling `validate_render_request(request)` with an unknown graph snapshot, walk state, configuration, or policy set returns the documented `RenderError` variant.
- [ ] Calling `validate_render_request(request)` with empty target model or render policy ID returns `RenderError::InvalidRequest`.
- [ ] Request validation does not create WorkingSetSnapshot, BudgetLedger, AuditEvent, or EvidenceArtifact rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-04 `PolicySet`, WU-0B-05 `GraphConfiguration`, WU-0B-14 `GraphSnapshot`, WU-0B-23 `AgentWalkState`.

**Produces:** Render request DTO consumed by RenderEngine, IPC command router, render fixtures, and VS-001.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-24a, WU-0C-30a.

### WU-0C-22a: RenderPolicy DTO

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

RenderPolicy {
  render_policy_id: string,
  max_nodes: i64,
  max_evidence_items: i64,
  max_depth: i64,
  token_ceiling: i64,
  required_pin_behavior: "fail_closed" | "allow_exception"
}

validate_render_policy(policy: RenderPolicy) -> Result<RenderPolicy, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-22a-render-policy.md`, `src-tauri/src/contracts/render_policy.rs`, `src/contracts/render-policy.ts`

**Code boundary:** `src-tauri/src/render/render_policy.rs`, `src-tauri/src/contracts/render_policy.rs`, `src/contracts/render-policy.ts`, `src-tauri/tests/wu_0c_22a_render_policy_contract.rs`, `src/test/render-policy.test.ts`

**Acceptance criteria:**

- [ ] `RenderPolicy` round-trips through Rust serde and TypeScript fixture JSON with all caps and required-pin behavior preserved.
- [ ] Every `required_pin_behavior` variant round-trips and is reachable through configuration fixtures.
- [ ] Calling `validate_render_policy(policy)` with positive caps returns the normalized DTO.
- [ ] Calling `validate_render_policy(policy)` with negative or zero caps returns `RenderError::InvalidRenderPolicy`.
- [ ] DTO validation creates no render snapshots, audit rows, or configuration provenance rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-21; incoming WU-0B-05 `GraphConfiguration`.

**Produces:** Resolved render policy DTO consumed by WU-0C-22, privilege filtering, budget adapter, and RenderEngine core.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-16.

### WU-0C-22: RenderPolicyResolver

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust service object

RenderPolicyResolver::resolve(request: RenderRequestDto) -> Result<RenderPolicy, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-22-render-policy-resolver.md`, `src-tauri/src/contracts/render_policy_resolver.rs`

**Code boundary:** `src-tauri/src/render/policy_resolver.rs`, `src-tauri/src/contracts/render_policy_resolver.rs`, `src-tauri/tests/wu_0c_22_render_policy_resolver_contract.rs`

**Acceptance criteria:**

- [ ] Calling `resolve(request)` returns a policy derived from the request's GraphConfiguration effective values.
- [ ] Calling `resolve(request)` with an unknown render policy ID returns `RenderError::UnknownRenderPolicy`.
- [ ] Calling `resolve(request)` with negative or zero caps in configuration returns `RenderError::InvalidRenderPolicy`.
- [ ] Resolution cites configuration provenance through WU-0C-10 and does not create render snapshots or audit rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-08, WU-0C-10, WU-0C-21, WU-0C-22a.

**Produces:** Resolved render policy consumed by privilege filtering, budget adapter, and RenderEngine core.

**Parallelizable with:** WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-12, WU-0C-14, WU-0C-16.

### WU-0C-23a: RenderPrivilegeFilterInput DTO

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

RenderPrivilegeFilterInput {
  graph_snapshot_id: string,
  node_ids: string[],
  evidence_pointer_ids: string[],
  target_actor: "orchestrator" | "worker",
  policy_set_id: string
}

validate_render_privilege_filter_input(input: RenderPrivilegeFilterInput) -> Result<RenderPrivilegeFilterInput, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-23a-render-privilege-filter-input.md`, `src-tauri/src/contracts/render_privilege_filter_input.rs`, `src/contracts/render-privilege-filter-input.ts`

**Code boundary:** `src-tauri/src/render/privilege_filter_input.rs`, `src-tauri/src/contracts/render_privilege_filter_input.rs`, `src/contracts/render-privilege-filter-input.ts`, `src-tauri/tests/wu_0c_23a_render_privilege_filter_input_contract.rs`, `src/test/render-privilege-filter-input.test.ts`

**Acceptance criteria:**

- [ ] `RenderPrivilegeFilterInput` round-trips through Rust serde and TypeScript fixture JSON with snapshot, node, evidence, actor, and policy refs preserved.
- [ ] Calling `validate_render_privilege_filter_input(input)` with valid refs returns the normalized DTO.
- [ ] Calling `validate_render_privilege_filter_input(input)` with unknown actor or empty snapshot/policy refs returns the documented error.
- [ ] DTO validation writes no graph, evidence, working-set, policy decision, or audit rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-07 `GraphNode`, WU-0B-10 `ProvenancePointer`, WU-0B-11 `SummaryContract`, WU-0B-14 `GraphSnapshot`.

**Produces:** Privilege filter input DTO consumed by WU-0C-23.

**Parallelizable with:** WU-0C-17a, WU-0C-17b, WU-0C-20a, WU-0C-24 after policy prerequisites.

### WU-0C-23b: RenderPrivilegeFilterResult DTO

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

RenderPrivilegeFilterResult {
  allowed_node_ids: string[],
  allowed_evidence_pointer_ids: string[],
  blocked_refs: { ref_id: string, reason_code: string }[],
  policy_decision?: PolicyDecision<JsonValue>
}

validate_render_privilege_filter_result(result: RenderPrivilegeFilterResult) -> Result<RenderPrivilegeFilterResult, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-23b-render-privilege-filter-result.md`, `src-tauri/src/contracts/render_privilege_filter_result.rs`, `src/contracts/render-privilege-filter-result.ts`

**Code boundary:** `src-tauri/src/render/privilege_filter_result.rs`, `src-tauri/src/contracts/render_privilege_filter_result.rs`, `src/contracts/render-privilege-filter-result.ts`, `src-tauri/tests/wu_0c_23b_render_privilege_filter_result_contract.rs`, `src/test/render-privilege-filter-result.test.ts`

**Acceptance criteria:**

- [ ] `RenderPrivilegeFilterResult` round-trips through Rust serde and TypeScript fixture JSON with allowed refs, blocked refs, reason codes, and optional policy decision preserved.
- [ ] Calling `validate_render_privilege_filter_result(result)` with valid allowed/blocked refs returns the normalized DTO.
- [ ] Calling `validate_render_privilege_filter_result(result)` with a blocked ref missing a reason code returns the documented error.
- [ ] DTO validation writes no graph, evidence, working-set, policy decision, or audit rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-01; incoming WU-0B-07 `GraphNode`, WU-0B-10 `ProvenancePointer`, WU-0B-11 `SummaryContract`, WU-0B-14 `GraphSnapshot`.

**Produces:** Privilege filter result DTO consumed by WU-0C-23 and RenderEngine core.

**Parallelizable with:** WU-0C-17a, WU-0C-17b, WU-0C-20a, WU-0C-24 after policy prerequisites.

### WU-0C-23: RenderPrivilegeFilter

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust service object

RenderPrivilegeFilter::filter(input: RenderPrivilegeFilterInput) -> Result<RenderPrivilegeFilterResult, RenderError>

Routing invariant:
- Poison-quarantined, deleted, and unresolved-conflict nodes are blocked unless policy explicitly permits a labeled evidence-only render.
- Lower-privilege evidence cannot become higher-privilege instruction text.
```

**Test boundary:** `product-strategy/contracts/wu-0c-23-render-privilege-filter.md`, `src-tauri/src/contracts/render_privilege_filter.rs`

**Code boundary:** `src-tauri/src/render/privilege_filter.rs`, `src-tauri/src/contracts/render_privilege_filter.rs`, `src-tauri/tests/wu_0c_23_render_privilege_filter_contract.rs`

**Acceptance criteria:**

- [ ] Calling `filter(input)` with trusted nodes and evidence returns all allowed refs unchanged.
- [ ] Calling `filter(input)` with poison-quarantined nodes returns blocked refs with documented reason code.
- [ ] Calling `filter(input)` with deleted nodes returns blocked refs with documented reason code.
- [ ] Calling `filter(input)` with unresolved-conflict refs returns blocked refs unless a fixture PolicySet explicitly allows labeled evidence-only rendering.
- [ ] The lower-privilege routing invariant holds across model/tool/worker/optimizer/reviewer evidence fixtures.
- [ ] Calling `filter(input)` with unknown node or evidence refs returns `RenderError::UnknownRenderRef`.
- [ ] The filter writes no GraphNode, EvidenceArtifact, WorkingSetSnapshot, PolicyDecision, or AuditEvent rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-23a, WU-0C-23b; incoming WU-0B-07 `GraphNode`, WU-0B-10 `ProvenancePointer`, WU-0B-11 `SummaryContract`, WU-0B-14 `GraphSnapshot`.

**Produces:** Privilege and poison filtering used by RenderEngine core and later quarantine surfaces.

**Parallelizable with:** WU-0C-17, WU-0C-20, WU-0C-24 after policy prerequisites.

### WU-0C-24a: CachePrefixInput DTO

**Parent initiative:** RenderEngine core and cache locality

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

CachePrefixInput {
  target_cli: string,
  target_model: string,
  policy_set_id: string,
  configuration_id: string,
  graph_snapshot_id: string,
  stable_prefix_parts: string[]
}

validate_cache_prefix_input(input: CachePrefixInput) -> Result<CachePrefixInput, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-24a-cache-prefix-input.md`, `src-tauri/src/contracts/cache_prefix_input.rs`, `src/contracts/cache-prefix-input.ts`

**Code boundary:** `src-tauri/src/render/cache_prefix_input.rs`, `src-tauri/src/contracts/cache_prefix_input.rs`, `src/contracts/cache-prefix-input.ts`, `src-tauri/tests/wu_0c_24a_cache_prefix_input_contract.rs`, `src/test/cache-prefix-input.test.ts`

**Acceptance criteria:**

- [ ] `CachePrefixInput` round-trips through Rust serde and TypeScript fixture JSON with target CLI, target model, policy set, configuration, snapshot, and stable prefix parts preserved.
- [ ] Calling `validate_cache_prefix_input(input)` with valid refs and non-empty stable prefix parts returns the normalized DTO.
- [ ] Calling `validate_cache_prefix_input(input)` with an empty stable prefix returns `RenderError::EmptyPrefix`.
- [ ] DTO validation reads no provider credentials and writes no BudgetLedger or WorkingSetSnapshot rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-04 `PolicySet`, WU-0B-05 `GraphConfiguration`, WU-0B-14 `GraphSnapshot`.

**Produces:** Cache prefix input DTO consumed by WU-0C-24 and render budget accounting.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-30a.

### WU-0C-24: CachePrefixHasher

**Parent initiative:** RenderEngine core and cache locality

**Contract:**
```text
schema_object: Rust service object

CachePrefixHasher::compute_prefix_hash(input: CachePrefixInput) -> Result<string, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-24-cache-prefix-hasher.md`, `src-tauri/src/contracts/cache_prefix_hasher.rs`

**Code boundary:** `src-tauri/src/render/cache_prefix.rs`, `src-tauri/src/contracts/cache_prefix_hasher.rs`, `src-tauri/tests/wu_0c_24_cache_prefix_contract.rs`

**Acceptance criteria:**

- [ ] Calling `compute_prefix_hash(input)` with valid input returns a non-empty deterministic hash.
- [ ] Calling `compute_prefix_hash(input)` twice with equivalent input returns byte-equivalent hashes.
- [ ] Changing target CLI, target model, policy set, configuration, graph snapshot, or stable prefix part changes the hash in documented fixtures.
- [ ] Stable prefix parts are ordered deterministically before hashing or rejected if order-sensitive fixture says so.
- [ ] Calling `compute_prefix_hash(input)` with an empty stable prefix returns `RenderError::EmptyPrefix`.
- [ ] The hasher reads no provider credentials and writes no BudgetLedger or WorkingSetSnapshot rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-24a; incoming WU-0B-04 `PolicySet`, WU-0B-05 `GraphConfiguration`, WU-0B-14 `GraphSnapshot`.

**Produces:** Cache prefix hash operation consumed by RenderEngine core and BudgetLedger accounting.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-30a.

### WU-0C-25a: RenderBudgetRequest DTO

**Parent initiative:** RenderEngine plus BudgetLedger integration

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

RenderBudgetRequest {
  render_request: RenderRequestDto,
  projected_token_estimate: i64,
  projected_cache_read_tokens: i64,
  projected_cache_write_tokens: i64,
  cache_prefix_hash: string,
  provider_state_id?: string
}

validate_render_budget_request(request: RenderBudgetRequest) -> Result<RenderBudgetRequest, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-25a-render-budget-request.md`, `src-tauri/src/contracts/render_budget_request.rs`, `src/contracts/render-budget-request.ts`

**Code boundary:** `src-tauri/src/render/budget_request.rs`, `src-tauri/src/contracts/render_budget_request.rs`, `src/contracts/render-budget-request.ts`, `src-tauri/tests/wu_0c_25a_render_budget_request_contract.rs`, `src/test/render-budget-request.test.ts`

**Acceptance criteria:**

- [ ] `RenderBudgetRequest` round-trips through Rust serde and TypeScript fixture JSON with render request, projected tokens, cache tokens, cache prefix hash, and provider state preserved.
- [ ] Calling `validate_render_budget_request(request)` with non-negative projected usage returns the normalized DTO.
- [ ] Calling `validate_render_budget_request(request)` with negative projected tokens returns `RenderError::BudgetInputInvalid`.
- [ ] DTO validation writes no BudgetLedger, WorkingSetSnapshot, or AuditEvent rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-21, WU-0C-24; incoming WU-0B-17 `ProviderState`.

**Produces:** Render budget request DTO consumed by WU-0C-25 and RenderEngine core.

**Parallelizable with:** WU-0C-23 after WU-0C-07 and WU-0C-21 complete.

### WU-0C-25: RenderBudgetGateAdapter

**Parent initiative:** RenderEngine plus BudgetLedger integration

**Contract:**
```text
schema_object: Rust service object

RenderBudgetGateAdapter::check_render_budget(request: RenderBudgetRequest) -> Result<BudgetDecision, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-25-render-budget-gate-adapter.md`, `src-tauri/src/contracts/render_budget_gate_adapter.rs`

**Code boundary:** `src-tauri/src/render/budget_adapter.rs`, `src-tauri/src/contracts/render_budget_gate_adapter.rs`, `src-tauri/tests/wu_0c_25_render_budget_adapter_contract.rs`

**Acceptance criteria:**

- [ ] Calling `check_render_budget(request)` with within-budget inputs returns `BudgetDecision` with `budget_state = within`.
- [ ] Calling `check_render_budget(request)` with over-budget render inputs returns `RenderError::Budget(BudgetDecision)` and preserves the budget decision fields.
- [ ] Calling `check_render_budget(request)` with negative projected tokens returns `RenderError::BudgetInputInvalid`.
- [ ] Calling `check_render_budget(request)` includes cache prefix hash and provider state in the delegated BudgetCheckRequest.
- [ ] Equivalent render budget requests return byte-equivalent decisions.
- [ ] The adapter delegates gate evaluation to WU-0C-07 and writes no BudgetLedger row.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-07, WU-0C-21, WU-0C-24, WU-0C-25a.

**Produces:** Render-specific budget adapter consumed by RenderEngine core.

**Parallelizable with:** WU-0C-23 after WU-0C-07 and WU-0C-21 complete.

### WU-0C-26a: RenderResultDto

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

RenderResultDto {
  working_set_id: string,
  graph_snapshot_id: string,
  configuration_id: string,
  rendered_blob_ref: string,
  token_estimate: i64,
  cache_prefix_hash: string,
  provider_state_id?: string,
  capability_fingerprint_id?: string,
  configuration_explanation_ref: string,
  audit_event_draft: AuditEventDraft
}

validate_render_result(result: RenderResultDto) -> Result<RenderResultDto, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-26a-render-result-dto.md`, `src-tauri/src/contracts/render_result.rs`, `src/contracts/render-result.ts`

**Code boundary:** `src-tauri/src/render/render_result.rs`, `src-tauri/src/contracts/render_result.rs`, `src/contracts/render-result.ts`, `src-tauri/tests/wu_0c_26a_render_result_contract.rs`, `src/test/render-result.test.ts`

**Acceptance criteria:**

- [ ] `RenderResultDto` round-trips through Rust serde and TypeScript fixture JSON with working set, snapshot, configuration, blob, token, cache, provider, capability, configuration explanation, and audit draft fields preserved.
- [ ] Calling `validate_render_result(result)` with valid refs returns the normalized DTO.
- [ ] Calling `validate_render_result(result)` with empty working set, rendered blob, cache prefix, or configuration explanation ref returns the documented error.
- [ ] DTO validation writes no WorkingSetSnapshot, BudgetLedger, EvidenceArtifact, or AuditEvent rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-21, WU-0C-22a, WU-0C-24; incoming WU-0B-15 `AuditEvent`, WU-0B-17 `ProviderState`, WU-0B-19 `CapabilityFingerprint`, WU-0B-21 `WorkingSetSnapshot`.

**Produces:** Render result DTO consumed by WU-0C-26, IPC, audit, and Phase 1+ render consumers.

**Parallelizable with:** WU-0C-25 after WU-0C-21 and WU-0C-24 complete.

### WU-0C-26: RenderEngine Core

**Parent initiative:** RenderEngine core

**Contract:**
```text
schema_object: Rust service object

RenderEngine::render_working_set(request: RenderRequestDto) -> Result<RenderResultDto, RenderError>
RenderEngine::golden_fixture(cli: string, shape_name: string) -> Result<RenderResultDto, RenderError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-26-render-engine-core.md`, `src-tauri/src/contracts/render_engine.rs`, CLI golden fixtures under `product-strategy/contracts/fixtures/wu-0c-26/`

**Code boundary:** `src-tauri/src/render/engine.rs`, `src-tauri/src/contracts/render_engine.rs`, `src-tauri/tests/wu_0c_26_render_engine_contract.rs`

**Acceptance criteria:**

- [ ] Calling `render_working_set(request)` with valid inputs returns `RenderResultDto` and creates one WU-0B-21 `WorkingSetSnapshot` row with non-null rendered blob ref, token estimate, cache prefix hash, configuration explanation ref, and graph snapshot ID.
- [ ] Calling `render_working_set(request)` preserves all pinned, unpacked, summary, evidence pointer, depth, and evicted refs selected from GraphSnapshot and AgentWalkState fixtures.
- [ ] Calling `render_working_set(request)` for Claude, Codex, and opencode golden fixtures returns the documented CLI-specific shape.
- [ ] Calling `render_working_set(request)` with budget denial returns `RenderError::Budget(...)` and creates no WorkingSetSnapshot row.
- [ ] Calling `render_working_set(request)` with blocked privilege refs returns `RenderError::PrivilegeBlocked(...)` unless policy permits labeled evidence-only rendering.
- [ ] Re-rendering the same `(GraphSnapshot, AgentWalkState, GraphConfiguration, PolicySet, target)` tuple returns byte-equivalent `RenderResultDto` except for documented row IDs and timestamps.
- [ ] Calling `golden_fixture(cli, shape_name)` returns the documented fixture or `RenderError::UnknownGoldenFixture`; every CLI shape fixture is reachable.
- [ ] The engine emits only an audit event draft; durable audit writing is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-21, WU-0C-22, WU-0C-23, WU-0C-24, WU-0C-25, WU-0C-26a; incoming WU-0B-10, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-17, WU-0B-19, WU-0B-21, WU-0B-23.

**Produces:** RenderEngine service and golden render fixtures consumed by Phase 1 VS-001 and later orchestrator/worker slices.

**Parallelizable with:** none in render family; it is the join point for WU-0C-21 through WU-0C-25.

### WU-0C-27: OptimizerQueueService

**Parent initiative:** Optimizer queue shell

**Contract:**
```text
schema_object: Rust service object

OptimizerQueueService::enqueue(request: OptimizerRequestDraft) -> Result<OptimizerRequest, OptimizerError>
OptimizerQueueService::next_batch(workspace_id: string, limit: i64) -> Result<Vec<OptimizerRequest>, OptimizerError>
OptimizerQueueService::mark_state(optimizer_request_id: string, advisory_state: OptimizerAdvisoryState) -> Result<OptimizerRequest, OptimizerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-27-optimizer-queue-service.md`, `src-tauri/src/contracts/optimizer_queue_service.rs`

**Code boundary:** `src-tauri/src/optimizer/queue_service.rs`, `src-tauri/src/contracts/optimizer_queue_service.rs`, `src-tauri/tests/wu_0c_27_optimizer_queue_contract.rs`

**Acceptance criteria:**

- [ ] Calling `enqueue(request)` with each documented `source_type` persists an OptimizerRequest row and leaves graph truth unchanged.
- [ ] Every OptimizerRequest `source_type`, `request_type`, `priority_hint`, and `advisory_state` variant from WU-0B-25 is reachable through fixtures.
- [ ] Calling `next_batch(workspace_id, limit)` returns queued requests in deterministic priority/time order and never returns ignored, superseded, or converted requests.
- [ ] Calling `mark_state(id, advisory_state)` exercises every valid advisory-state transition and rejects every invalid transition with a documented error variant.
- [ ] Calling `mark_state(id, converted_to_optimizer_edit)` requires a converted OptimizerEdit ref.
- [ ] Queue operations do not launch `glm`, create OptimizerEdit rows, mutate graph topology, or emit operator-visible optimizer UI state in Phase 0C.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-07; incoming WU-0B-25 `OptimizerRequest`, WU-0B-14 `GraphSnapshot`.

**Produces:** Advisory optimizer queue shell consumed by OptimizerCycle, OptimizerScheduler, VS-009/010/011/018/020 emitters.

**Parallelizable with:** WU-0C-31 after WU-0C-30; WU-0C-32 after agent-runner basics; no dependency between them.

### WU-0C-28: OptimizerCycleStateMachine

**Parent initiative:** Optimizer cycle shell

**Contract:**
```text
schema_object: Rust state machine

OptimizerCycleState:
- queued
- scoped
- snapshot_read
- drafting
- schema_validation
- contract_validation
- policy_validation
- review_sampling
- merge_attempt
- merged
- conflicted
- rejected
- reverted

OptimizerCycleStateMachine::transition(cycle_id: string, from: OptimizerCycleState, to: OptimizerCycleState, reason: string) -> Result<OptimizerCycleState, OptimizerError>
OptimizerCycleStateMachine::initial_for(request: OptimizerRequest) -> Result<OptimizerCycleState, OptimizerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-28-optimizer-cycle-state-machine.md`, `src-tauri/src/contracts/optimizer_cycle_state_machine.rs`

**Code boundary:** `src-tauri/src/optimizer/cycle_state_machine.rs`, `src-tauri/src/contracts/optimizer_cycle_state_machine.rs`, `src-tauri/tests/wu_0c_28_optimizer_cycle_state_contract.rs`

**Acceptance criteria:**

- [ ] Every `OptimizerCycleState` variant round-trips through serde and is reachable through a documented transition fixture.
- [ ] Calling `initial_for(request)` returns `queued` for a queued OptimizerRequest and rejects ignored/superseded/converted requests.
- [ ] Every documented valid transition is exercised: queued->scoped->snapshot_read->drafting->schema_validation->contract_validation->policy_validation->review_sampling->merge_attempt->merged; validation failures to rejected/conflicted; merge_attempt->conflicted; merged->reverted.
- [ ] Every documented invalid transition is rejected with `OptimizerError::InvalidTransition`.
- [ ] Every state is reachable from the initial state.
- [ ] Transitions do not create OptimizerEdit, ConflictRecord, NodeRevision, GraphEdge, IdentityEvent, or AuditEvent rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-27; incoming WU-0B-25 `OptimizerRequest`, WU-0B-26 `OptimizerEdit`, WU-0B-27 `ConflictRecord`.

**Produces:** Optimizer cycle lifecycle shell consumed by OptimizerScheduler and later VS-010 implementation.

**Parallelizable with:** none in optimizer family after WU-0C-27; it gates WU-0C-29.

### WU-0C-29a: OptimizerScheduleDecision DTO

**Parent initiative:** Optimizer scheduler

**Contract:**
```text
schema_object: Rust struct

OptimizerScheduleDecision {
  decision: "run" | "skip_no_work" | "skip_budget" | "skip_policy",
  request_ids: string[],
  budget_decision?: BudgetDecision,
  policy_decision?: PolicyDecision<JsonValue>
}

validate_optimizer_schedule_decision(decision: OptimizerScheduleDecision) -> Result<OptimizerScheduleDecision, OptimizerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-29a-optimizer-schedule-decision.md`, `src-tauri/src/contracts/optimizer_schedule_decision.rs`

**Code boundary:** `src-tauri/src/optimizer/schedule_decision.rs`, `src-tauri/src/contracts/optimizer_schedule_decision.rs`, `src-tauri/tests/wu_0c_29a_optimizer_schedule_decision_contract.rs`

**Acceptance criteria:**

- [ ] `OptimizerScheduleDecision` round-trips through serde with decision, request IDs, budget decision, and policy decision preserved.
- [ ] Every decision variant is reachable through a documented fixture.
- [ ] Calling `validate_optimizer_schedule_decision(decision)` rejects `run` decisions with no request IDs and accepts skip decisions with documented empty-request fixtures.
- [ ] DTO validation launches no optimizer, drafts no edits, and mutates no graph truth.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-01, WU-0C-05; incoming WU-0B-25 `OptimizerRequest`.

**Produces:** Scheduler decision DTO consumed by WU-0C-29 and later reviewer sampling.

**Parallelizable with:** WU-0C-31a, WU-0C-33a after their prerequisites.

### WU-0C-29b: OptimizerCycleLease DTO

**Parent initiative:** Optimizer scheduler

**Contract:**
```text
schema_object: Rust struct

OptimizerCycleLease {
  lease_id: string,
  workspace_id: string,
  optimizer_request_id: string,
  cycle_id: string,
  claimed_at: Timestamp,
  expires_at: Timestamp
}

validate_optimizer_cycle_lease(lease: OptimizerCycleLease) -> Result<OptimizerCycleLease, OptimizerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-29b-optimizer-cycle-lease.md`, `src-tauri/src/contracts/optimizer_cycle_lease.rs`

**Code boundary:** `src-tauri/src/optimizer/cycle_lease.rs`, `src-tauri/src/contracts/optimizer_cycle_lease.rs`, `src-tauri/tests/wu_0c_29b_optimizer_cycle_lease_contract.rs`

**Acceptance criteria:**

- [ ] `OptimizerCycleLease` round-trips through serde with lease, workspace, request, cycle, claim, and expiry fields preserved.
- [ ] Calling `validate_optimizer_cycle_lease(lease)` with `expires_at > claimed_at` returns the normalized lease.
- [ ] Calling `validate_optimizer_cycle_lease(lease)` with empty IDs or expired-at-before-claimed-at returns the documented error.
- [ ] DTO validation does not claim requests, transition cycles, draft edits, or mutate graph truth.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-28; incoming WU-0B-25 `OptimizerRequest`.

**Produces:** Optimizer cycle lease DTO consumed by WU-0C-29.

**Parallelizable with:** WU-0C-31a, WU-0C-33a after their prerequisites.

### WU-0C-29: OptimizerScheduler

**Parent initiative:** Optimizer scheduler

**Contract:**
```text
schema_object: Rust service object

OptimizerScheduler::tick(workspace_id: string, budget_scope_id: string) -> Result<OptimizerScheduleDecision, OptimizerError>
OptimizerScheduler::claim_cycle(workspace_id: string, request_id: string) -> Result<OptimizerCycleLease, OptimizerError>
OptimizerScheduler::release_cycle(lease_id: string, final_state: OptimizerCycleState) -> Result<(), OptimizerError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-29-optimizer-scheduler.md`, `src-tauri/src/contracts/optimizer_scheduler.rs`

**Code boundary:** `src-tauri/src/optimizer/scheduler.rs`, `src-tauri/src/contracts/optimizer_scheduler.rs`, `src-tauri/tests/wu_0c_29_optimizer_scheduler_contract.rs`

**Acceptance criteria:**

- [ ] Calling `tick(workspace_id, budget_scope_id)` with queued work and allowed budget returns `decision = run` with request IDs.
- [ ] Calling `tick(...)` with no queued requests returns `decision = skip_no_work`.
- [ ] Calling `tick(...)` with budget denial returns `decision = skip_budget` and preserves the BudgetDecision.
- [ ] Calling `tick(...)` with policy denial returns `decision = skip_policy` and preserves the PolicyDecision.
- [ ] Calling `claim_cycle(workspace_id, request_id)` returns a lease for an unclaimed queued request and rejects already-claimed requests.
- [ ] Calling `release_cycle(lease_id, final_state)` accepts only terminal or paused documented cycle states and rejects invalid releases.
- [ ] The scheduler does not launch `glm`, draft edits, merge edits, or mutate graph truth in Phase 0C.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-07, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b.

**Produces:** Long-running optimizer loop shell consumed by Phase 2 VS-010 and later reviewer sampling.

**Parallelizable with:** WU-0C-31, WU-0C-32, WU-0C-33 after their prerequisites.

### WU-0C-29c: IdentityResolverRuntime

**Parent initiative:** Identity/conflict shell

**Contract:**
```text
schema_object: Rust service object

IdentityResolverRuntime::record_identity_event(event: IdentityEvent) -> Result<IdentityEvent, IdentityConflictError>
IdentityResolverRuntime::latest_identity_event(logical_node_id: string) -> Result<Option<IdentityEvent>, IdentityConflictError>

Routing invariant:
- Phase 0C records identity shell events only; it does not move, split, merge, restore, delete, or forward graph topology.
```

**Test boundary:** `product-strategy/contracts/wu-0c-29c-identity-resolver-runtime.md`, `src-tauri/src/contracts/identity_resolver_runtime.rs`

**Code boundary:** `src-tauri/src/identity/identity_resolver_runtime.rs`, `src-tauri/src/contracts/identity_resolver_runtime.rs`, `src-tauri/tests/wu_0c_29c_identity_resolver_runtime_contract.rs`

**Acceptance criteria:**

- [ ] Calling `record_identity_event(event)` with every WU-0B-13 `IdentityEvent` event-type variant writes exactly one durable IdentityEvent row and returns it with all Phase 0B fields preserved.
- [ ] Calling `record_identity_event(event)` with unknown node/version refs returns the documented error and writes no row.
- [ ] Calling `latest_identity_event(logical_node_id)` returns the newest event for that logical node or `None` for no events.
- [ ] The routing invariant holds across move, split, merge, forwarding, restore, and delete fixtures: no GraphNode, GraphEdge, NodeRevision, GraphSnapshot, or ConflictRecord row is mutated.
- [ ] The service emits only audit event drafts; durable audit writing is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04; incoming WU-0B-07 `GraphNode`, WU-0B-08 `GraphEdge`, WU-0B-12 `NodeRevision`, WU-0B-13 `IdentityEvent`, WU-0B-14 `GraphSnapshot`, WU-0B-15 `AuditEvent`.

**Produces:** Identity resolver runtime shell consumed by VS-012, VS-013, VS-017, VS-018, VS-020, and VS-021.

**Parallelizable with:** WU-0C-29d after WU-0C-04 if write files remain disjoint; WU-0C-31a, WU-0C-33a.

### WU-0C-29d: ConflictRecordWriterShell

**Parent initiative:** Identity/conflict shell

**Contract:**
```text
schema_object: Rust service object

ConflictRecordWriterShell::record_conflict(record: ConflictRecord) -> Result<ConflictRecord, IdentityConflictError>
ConflictRecordWriterShell::mark_conflict_state(conflict_record_id: string, state: ConflictState, reason: string) -> Result<ConflictRecord, IdentityConflictError>

Routing invariant:
- Phase 0C records conflict workflow shell state only; it does not resolve conflicts, merge edits, mutate topology, or launch optimizer/recovery execution.
```

**Test boundary:** `product-strategy/contracts/wu-0c-29d-conflict-record-writer-shell.md`, `src-tauri/src/contracts/conflict_record_writer_shell.rs`

**Code boundary:** `src-tauri/src/identity/conflict_record_writer_shell.rs`, `src-tauri/src/contracts/conflict_record_writer_shell.rs`, `src-tauri/tests/wu_0c_29d_conflict_record_writer_contract.rs`

**Acceptance criteria:**

- [ ] Calling `record_conflict(record)` with every WU-0B-27 conflict-type variant writes exactly one durable ConflictRecord row and returns it with all Phase 0B fields preserved.
- [ ] Calling `record_conflict(record)` with unknown identity, edit, snapshot, provider, worker, question, or budget refs returns the documented error and writes no row.
- [ ] Calling `mark_conflict_state(id, state, reason)` exercises every WU-0B-27 documented workflow state transition and rejects invalid transitions with a documented error.
- [ ] The routing invariant holds across identity, content, summary, configuration, edge, worker-overlap, question-route, tool-protocol, provider-state, and budget conflict fixtures: no GraphNode, GraphEdge, NodeRevision, OptimizerEdit, IdentityEvent, RecoveryAction, or AuditEvent row is mutated.
- [ ] The service emits only audit event drafts; durable audit writing is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-29c; incoming WU-0B-13 `IdentityEvent`, WU-0B-15 `AuditEvent`, WU-0B-25 `OptimizerRequest`, WU-0B-26 `OptimizerEdit`, WU-0B-27 `ConflictRecord`, WU-0B-29 `WorkerRun`, WU-0B-30 `QuestionArtifact`, WU-0B-31 `RecoveryAction`.

**Produces:** Conflict-record write shell consumed by VS-012, VS-013, VS-017, VS-018, VS-020, and VS-021.

**Parallelizable with:** WU-0C-31a, WU-0C-33a after WU-0C-04/WU-0C-29c.

### WU-0C-30a: RecoveryActionDraft DTO

**Parent initiative:** Recovery-action writer

**Contract:**
```text
schema_object: Rust struct

RecoveryActionDraft {
  action_type: RecoveryActionType,
  cause: RecoveryCause,
  precondition_ref: string,
  worker_run_ref?: string,
  affected_session_ids: string[],
  affected_node_ids: string[],
  affected_edit_ids: string[],
  affected_provider_state_ids: string[],
  provider_failure_cause: ProviderFailureCause,
  side_effect_classification: SideEffectClassification,
  user_confirmation_state: UserConfirmationState,
  preserved_ref?: string,
  replayed_ref?: string,
  discarded_ref?: string
}

validate_recovery_action_draft(draft: RecoveryActionDraft) -> Result<RecoveryActionDraft, RecoveryError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-30a-recovery-action-draft.md`, `src-tauri/src/contracts/recovery_action_draft.rs`

**Code boundary:** `src-tauri/src/recovery/action_draft.rs`, `src-tauri/src/contracts/recovery_action_draft.rs`, `src-tauri/tests/wu_0c_30a_recovery_action_draft_contract.rs`

**Acceptance criteria:**

- [ ] `RecoveryActionDraft` round-trips through serde with action, cause, precondition, affected refs, side-effect, confirmation, and preserved/replayed/discarded refs preserved.
- [ ] Every RecoveryAction action type, cause, provider failure cause, side-effect classification, and confirmation-state variant from WU-0B-31 round-trips through draft fixtures.
- [ ] Calling `validate_recovery_action_draft(draft)` with valid refs returns the normalized draft.
- [ ] Calling `validate_recovery_action_draft(draft)` with an empty precondition ref or invalid high-consequence confirmation state returns the documented error.
- [ ] DTO validation writes no RecoveryAction, AuditEvent, ProviderState, OptimizerEdit, or ConflictRecord rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-17 `ProviderState`, WU-0B-26 `OptimizerEdit`, WU-0B-27 `ConflictRecord`, WU-0B-29 `WorkerRun`, WU-0B-30 `QuestionArtifact`, WU-0B-31 `RecoveryAction`.

**Produces:** Recovery action draft DTO consumed by WU-0C-30 and failed-resume handoff slices.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24.

### WU-0C-30: RecoveryActionWriter

**Parent initiative:** Recovery-action writer

**Contract:**
```text
schema_object: Rust service object

RecoveryActionWriter::plan(draft: RecoveryActionDraft) -> Result<RecoveryAction, RecoveryError>
RecoveryActionWriter::mark_result(recovery_action_id: string, result_state: RecoveryResultState) -> Result<RecoveryAction, RecoveryError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-30-recovery-action-writer.md`, `src-tauri/src/contracts/recovery_action_writer.rs`

**Code boundary:** `src-tauri/src/recovery/action_writer.rs`, `src-tauri/src/contracts/recovery_action_writer.rs`, `src-tauri/tests/wu_0c_30_recovery_writer_contract.rs`

**Acceptance criteria:**

- [ ] Calling `plan(draft)` with valid input writes one WU-0B-31 RecoveryAction row in `planned` result state and returns it.
- [ ] Every RecoveryAction action type, cause, provider failure cause, side-effect classification, confirmation state, and result state variant from WU-0B-31 round-trips through writer fixtures.
- [ ] Calling `plan(draft)` for high-consequence actions requires `user_confirmation_state = required` or `granted` unless policy fixture allows not_required.
- [ ] Calling `plan(draft)` with unknown worker, provider, edit, or node refs returns the documented error and writes no row.
- [ ] Calling `mark_result(id, result_state)` exercises valid planned->applied, planned->failed, planned->partially_applied, applied->reverted, and partially_applied->reverted transitions.
- [ ] Invalid result transitions are rejected and leave the row unchanged.
- [ ] The writer emits only audit event drafts; durable audit writing is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-30a; incoming WU-0B-15 `AuditEvent`, WU-0B-17 `ProviderState`, WU-0B-26 `OptimizerEdit`, WU-0B-27 `ConflictRecord`, WU-0B-29 `WorkerRun`, WU-0B-30 `QuestionArtifact`, WU-0B-31 `RecoveryAction`.

**Produces:** Durable recovery-action writer consumed by VS-017 failed-resume handoff, VS-020 recovery preflight, and provider reroute.

**Parallelizable with:** WU-0C-01, WU-0C-05, WU-0C-08, WU-0C-11a, WU-0C-11b, WU-0C-19, WU-0C-21, WU-0C-24a.

### WU-0C-31a: RecoveryProcessorDecision DTO

**Parent initiative:** RecoveryAction processor skeleton

**Contract:**
```text
schema_object: Rust struct

RecoveryProcessorDecision {
  recovery_action_id: string,
  processor_state: "detected" | "classified" | "preflight" | "requires_user" | "blocked_execution_deferred",
  next_required_input?: string,
  audit_event_draft?: AuditEventDraft
}

validate_recovery_processor_decision(decision: RecoveryProcessorDecision) -> Result<RecoveryProcessorDecision, RecoveryError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-31a-recovery-processor-decision.md`, `src-tauri/src/contracts/recovery_processor_decision.rs`

**Code boundary:** `src-tauri/src/recovery/processor_decision.rs`, `src-tauri/src/contracts/recovery_processor_decision.rs`, `src-tauri/tests/wu_0c_31a_recovery_processor_decision_contract.rs`

**Acceptance criteria:**

- [ ] `RecoveryProcessorDecision` round-trips through serde with action, state, next input, and audit draft fields preserved.
- [ ] Every processor state variant is reachable through a documented fixture.
- [ ] Calling `validate_recovery_processor_decision(decision)` with valid state/input combinations returns the normalized DTO.
- [ ] Calling `validate_recovery_processor_decision(decision)` with `processor_state = requires_user` and no next required input returns the documented error.
- [ ] DTO validation executes no rollback, retry, resume, cancel, quarantine, substitution, or audit append side effects.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-30; incoming WU-0B-15 `AuditEvent`, WU-0B-31 `RecoveryAction`.

**Produces:** Recovery processor decision DTO consumed by WU-0C-31 and failed-resume UX slices.

**Parallelizable with:** WU-0C-29a, WU-0C-29b, WU-0C-33a after prerequisites.

### WU-0C-31: RecoveryActionProcessorSkeleton

**Parent initiative:** RecoveryAction processor skeleton

**Contract:**
```text
schema_object: Rust service object

RecoveryActionProcessorSkeleton::preflight(recovery_action_id: string) -> Result<RecoveryProcessorDecision, RecoveryError>
RecoveryActionProcessorSkeleton::classify(recovery_action_id: string) -> Result<RecoveryProcessorDecision, RecoveryError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-31-recovery-action-processor-skeleton.md`, `src-tauri/src/contracts/recovery_action_processor.rs`

**Code boundary:** `src-tauri/src/recovery/processor_skeleton.rs`, `src-tauri/src/contracts/recovery_action_processor.rs`, `src-tauri/tests/wu_0c_31_recovery_processor_contract.rs`

**Acceptance criteria:**

- [ ] Calling `classify(recovery_action_id)` returns side-effect and provider-failure classification for an existing RecoveryAction.
- [ ] Calling `classify(recovery_action_id)` with an unknown ID returns `RecoveryError::UnknownRecoveryAction`.
- [ ] Calling `preflight(recovery_action_id)` for a not-required confirmation action returns `processor_state = preflight` or `blocked_execution_deferred` according to fixtures.
- [ ] Calling `preflight(recovery_action_id)` for required confirmation action returns `processor_state = requires_user`.
- [ ] The skeleton never executes rollback, retry, resume, cancel, quarantine, or substitution side effects in Phase 0C.
- [ ] Processor decisions may emit audit event drafts only; durable audit writing is delegated to WU-0C-37.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-18, WU-0C-20, WU-0C-30, WU-0C-31a.

**Produces:** Non-executing recovery processor shell consumed by VS-017 and VS-020.

**Parallelizable with:** WU-0C-27 after WU-0C-30; WU-0C-32 and WU-0C-33 after agent-runner prerequisites.

### WU-0C-32: HookPayloadEnvelope

**Parent initiative:** Hook/MCP/plugin injection scaffold

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

HookPayload {
  cli: "claude" | "codex" | "opencode",
  hook_kind: "UserPromptSubmit" | "SessionStart" | "PreToolUse" | "PostToolUse" | "Stop",
  session_id: string,
  invocation_id: string,
  payload_ref: string,
  correlation_key: string
}

build_hook_payload(cli, hook_kind, session_id, invocation_id, payload_ref) -> Result<HookPayload, HookError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-32-hook-payload-envelope.md`, `src-tauri/src/contracts/hook_payload.rs`, `src/contracts/hook-payload.ts`

**Code boundary:** `src-tauri/src/hooks/hook_payload.rs`, `src-tauri/src/contracts/hook_payload.rs`, `src/contracts/hook-payload.ts`, `src-tauri/tests/wu_0c_32_hook_payload_contract.rs`, `src/test/hook-payload.test.ts`

**Acceptance criteria:**

- [ ] `HookPayload` round-trips through Rust serde and TypeScript fixture JSON with every field preserved.
- [ ] Every `hook_kind` variant round-trips and is reachable through a documented fixture.
- [ ] Calling `build_hook_payload(...)` with valid input returns a payload with non-empty correlation key.
- [ ] Calling `build_hook_payload(...)` with unknown CLI or hook kind returns the documented error variant.
- [ ] Calling `build_hook_payload(...)` with empty session ID, invocation ID, or payload ref returns `HookError::InvalidHookPayload`.
- [ ] Correlation keys are stable across pre/post hook transformations for the same session/invocation/payload tuple.
- [ ] Hook payload construction does not invoke subprocesses, write ToolCallProvenance, or mutate graph state.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11a, WU-0C-11b, WU-0C-12, WU-0C-13.

**Produces:** Hook envelope consumed by plugin capability matrix, tool provenance later slices, and worker/orchestrator injection surfaces.

**Parallelizable with:** WU-0C-31 after WU-0C-13; no dependency between them.

### WU-0C-33a: PluginCapability DTO

**Parent initiative:** Hook/MCP/plugin capability boundary

**Contract:**
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

**Test boundary:** `product-strategy/contracts/wu-0c-33a-plugin-capability.md`, `src-tauri/src/contracts/plugin_capability.rs`

**Code boundary:** `src-tauri/src/hooks/plugin_capability.rs`, `src-tauri/src/contracts/plugin_capability.rs`, `src-tauri/tests/wu_0c_33a_plugin_capability_contract.rs`

**Acceptance criteria:**

- [ ] `PluginCapability` round-trips through serde with CLI, injection surface, capability flags, and unsupported reason preserved.
- [ ] Every `injection_surface` variant is reachable through a documented fixture.
- [ ] Calling `validate_plugin_capability(capability)` rejects unsupported CLIs that report transform/intercept/MCP support as true.
- [ ] DTO validation does not install plugins, create MCP resources, launch agents, or mutate CapabilityFingerprint rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-32; incoming WU-0B-19 `CapabilityFingerprint`.

**Produces:** Plugin capability DTO consumed by WU-0C-33 and provider capability surfaces.

**Parallelizable with:** WU-0C-29, WU-0C-31 after prerequisites.

### WU-0C-33: PluginCapabilityMatrix

**Parent initiative:** Hook/MCP/plugin capability boundary

**Contract:**
```text
schema_object: Rust service object

PluginCapabilityMatrix::capability_matrix_for_cli(cli: string) -> Result<PluginCapability, HookError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-33-plugin-capability-matrix.md`, `src-tauri/src/contracts/plugin_capability_matrix.rs`

**Code boundary:** `src-tauri/src/hooks/plugin_capability_matrix.rs`, `src-tauri/src/contracts/plugin_capability_matrix.rs`, `src-tauri/tests/wu_0c_33_plugin_capability_contract.rs`

**Acceptance criteria:**

- [ ] Calling `capability_matrix_for_cli("claude")` returns the documented hook/MCP capability flags.
- [ ] Calling `capability_matrix_for_cli("codex")` returns the documented wrapper/hook capability flags.
- [ ] Calling `capability_matrix_for_cli("opencode")` returns the documented plugin transform or substrate-gap capability flags.
- [ ] Calling `capability_matrix_for_cli("unknown")` returns `HookError::UnsupportedCli` and a non-empty unsupported reason.
- [ ] Unsupported CLIs never report `supports_prompt_transform`, `supports_tool_intercept`, or `supports_mcp_resource` as true.
- [ ] The matrix does not install plugins, create MCP resources, launch agents, or mutate CapabilityFingerprint rows in Phase 0C.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-11a, WU-0C-11b, WU-0C-12, WU-0C-13, WU-0C-32, WU-0C-33a; incoming WU-0B-19 `CapabilityFingerprint`.

**Produces:** Hook/plugin capability boundary consumed by VS-003, VS-009, VS-015, VS-017, and provider capability surfaces.

**Parallelizable with:** WU-0C-29 and WU-0C-31 after prerequisites.

### WU-0C-34: TauriIpcCommandRouter

**Parent initiative:** Tauri IPC layer scaffolding

**Contract:**
```text
schema_object: IPC command router

HarnessCommand additions routed in Phase 0C:
- render_working_set
- evaluate_policy_gate
- check_budget
- read_configuration
- write_configuration_revision
- monitor_provider_state
- read_agent_runner_trace
- read_session_turns
- locate_session_override
- get_session_override_metadata
- list_session_overrides
- enqueue_optimizer_request
- record_identity_event
- record_conflict
- plan_recovery_action

TauriIpcCommandRouter::route(command: HarnessCommand, args: JsonValue) -> Result<JsonValue, IpcCommandError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-34-tauri-ipc-command-router.md`, `src-tauri/src/contracts/tauri_ipc_command_router.rs`, `src/contracts/tauri-ipc-command-router.ts`

**Code boundary:** `src-tauri/src/ipc/command_router.rs`, `src-tauri/src/contracts/tauri_ipc_command_router.rs`, `src/contracts/tauri-ipc-command-router.ts`, `src-tauri/tests/wu_0c_34_ipc_command_router_contract.rs`, `src/test/tauri-ipc-command-router.test.ts`

**Acceptance criteria:**

- [ ] Every Phase 0C `HarnessCommand` addition round-trips through Rust command-name fixtures and TypeScript command fixtures.
- [ ] Calling `route(command, args)` for each documented command delegates to the owning WU service and returns the documented DTO shape.
- [ ] Calling `route("unknown", args)` returns `IpcCommandError::UnknownCommand`.
- [ ] Calling `route(command, malformed_args)` returns `IpcCommandError::ArgumentDeserializationFailed`.
- [ ] Routing preserves workspace ID, trace context, and command name in the service call fixture.
- [ ] Router tests prove no command invokes a value-slice `VS-*` handler or operator-visible feature behavior in Phase 0C.
- [ ] Session-override commands expose locate/metadata/registry reads only in Phase 0C; the router does not expose direct `replace_transcript`, `truncate_after`, or `append_turns` UI commands.
- [ ] The router does not emit events directly; event emission is owned by WU-0C-35.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-04, WU-0C-07, WU-0C-08, WU-0C-09, WU-0C-10, WU-0C-18, WU-0C-20, WU-0C-26, WU-0C-27, WU-0C-29c, WU-0C-29d, WU-0C-30, WU-0C-N1, WU-0C-N5; incoming WU-0A-07 `HarnessCommand`, WU-0A-04 `HarnessAppState`.

**Produces:** Typed IPC command routing layer consumed by UI shell and later value slices.

**Parallelizable with:** WU-0C-35 after shared service prerequisites if command registration files are coordinated.

### WU-0C-35a: DomainEventDraft DTO

**Parent initiative:** Channel event emission backbone

**Contract:**
```text
schema_object: Rust struct + TypeScript DTO

DomainEventDraft {
  workspace_id: string,
  topic: EventTopic,
  payload: JsonValue,
  trace_context_id?: string,
  audit_event_ref?: string
}

validate_domain_event_draft(draft: DomainEventDraft) -> Result<DomainEventDraft, IpcEventError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-35a-domain-event-draft.md`, `src-tauri/src/contracts/domain_event_draft.rs`, `src/contracts/domain-event-draft.ts`

**Code boundary:** `src-tauri/src/ipc/domain_event_draft.rs`, `src-tauri/src/contracts/domain_event_draft.rs`, `src/contracts/domain-event-draft.ts`, `src-tauri/tests/wu_0c_35a_domain_event_draft_contract.rs`, `src/test/domain-event-draft.test.ts`

**Acceptance criteria:**

- [ ] `DomainEventDraft` round-trips through Rust serde and TypeScript fixture JSON with workspace, topic, payload, trace context, and audit ref preserved.
- [ ] Calling `validate_domain_event_draft(draft)` with every documented EventTopic returns the normalized DTO.
- [ ] Calling `validate_domain_event_draft(draft)` with unknown topic or empty workspace ID returns the documented error.
- [ ] DTO validation does not emit events, subscribe listeners, or decide domain semantics.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0A-05 `EventTopic`, WU-0A-09 `TraceContext`, WU-0B-15 `AuditEvent`.

**Produces:** Domain event draft DTO consumed by WU-0C-35 and all event-emitting Phase 0C services.

**Parallelizable with:** WU-0C-34 after shared registration files are coordinated.

### WU-0C-35: IpcEventEmissionBackbone

**Parent initiative:** Channel event emission backbone

**Contract:**
```text
schema_object: Rust service object

IpcEventEmissionBackbone::emit(draft: DomainEventDraft) -> Result<IpcEvent<JsonValue>, IpcEventError>
IpcEventEmissionBackbone::subscribe(workspace_id: string, topic: EventTopic) -> Result<SubscriptionHandle, IpcEventError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-35-ipc-event-emission-backbone.md`, `src-tauri/src/contracts/ipc_event_emission_backbone.rs`, `src/contracts/ipc-event-emission-backbone.ts`

**Code boundary:** `src-tauri/src/ipc/event_backbone.rs`, `src-tauri/src/contracts/ipc_event_emission_backbone.rs`, `src/contracts/ipc-event-emission-backbone.ts`, `src-tauri/tests/wu_0c_35_ipc_event_backbone_contract.rs`, `src/test/ipc-event-emission-backbone.test.ts`

**Acceptance criteria:**

- [ ] Calling `emit(draft)` with every documented EventTopic returns an `IpcEvent<JsonValue>` with the same workspace ID, topic, payload, trace context, and audit ref.
- [ ] Calling `emit(draft)` with unknown topic returns `IpcEventError::UnknownTopic`.
- [ ] Calling `emit(draft)` with empty workspace ID returns `IpcEventError::EmptyWorkspaceId`.
- [ ] Calling `subscribe(workspace_id, topic)` registers a subscription through WU-0A-08 and returns a non-empty handle.
- [ ] Emission preserves topic isolation: a render event is delivered only to render subscribers, audit events only to audit subscribers, and runtime events only to runtime subscribers.
- [ ] The backbone does not decide domain semantics; payload production remains owned by the emitting service.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-35a; incoming WU-0A-05 `EventTopic`, WU-0A-06 `IpcEvent<T>`, WU-0A-08 `subscribe_workspace_events`, WU-0A-09 `TraceContext`, WU-0A-10 `BackendSpanEvent`.

**Produces:** Domain event emission backbone consumed by UI shell, audit emit pipeline, render/provider/optimizer/budget/recovery events, and later value slices.

**Parallelizable with:** WU-0C-34 after shared registration files are coordinated.

### WU-0C-36a: PaneRouteBinding DTO

**Parent initiative:** UI shell wiring

**Contract:**
```text
schema_object: TypeScript DTO

PaneRouteBinding {
  paneId: PaneId,
  command?: HarnessCommand,
  eventTopics: EventTopic[],
  shellRegionStateSelector: string
}

validatePaneRouteBinding(binding: PaneRouteBinding): Result<PaneRouteBinding, UiShellError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-36a-pane-route-binding.md`, `src/contracts/pane-route-binding.ts`, fixtures under `product-strategy/contracts/fixtures/wu-0c-36a/`

**Code boundary:** `src/shell/pane-route-binding.ts`, `src/contracts/pane-route-binding.ts`, `src/test/pane-route-binding.test.ts`

**Acceptance criteria:**

- [ ] `PaneRouteBinding` fixture JSON parses with exact pane ID, command, event topics, and selector fields.
- [ ] Calling `validatePaneRouteBinding(binding)` accepts every seeded Phase 0C PaneId binding.
- [ ] Calling `validatePaneRouteBinding(binding)` with an unknown pane, command, topic, or empty selector returns the documented error.
- [ ] DTO validation does not route commands, subscribe events, render panes, or query GraphStore.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0A-05 `EventTopic`, WU-0A-07 `HarnessCommand`, WU-0A-11 `PaneId`, WU-0A-12 `ShellRegionState`.

**Produces:** Pane route binding DTO consumed by WU-0C-36 and later seeded UI panes.

**Parallelizable with:** WU-0C-37a after WU-0C-35 if UI and audit files are disjoint.

### WU-0C-36: UiShellPaneRouter

**Parent initiative:** UI shell wiring

**Contract:**
```text
schema_object: TypeScript service object

UiShellPaneRouter::bindingForPane(paneId: PaneId) -> Result<PaneRouteBinding, UiShellError>
UiShellPaneRouter::deriveShellRegionState(workspaceId: string, activePane: PaneId, selectedNodeId?: string) -> Result<ShellRegionState, UiShellError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-36-ui-shell-pane-router.md`, `src/contracts/ui-shell-pane-router.ts`, fixtures under `product-strategy/contracts/fixtures/wu-0c-36/`

**Code boundary:** `src/shell/ui-shell-pane-router.ts`, `src/contracts/ui-shell-pane-router.ts`, `src/test/ui-shell-pane-router.test.tsx`, `e2e/ui-shell-pane-router.spec.ts`

**Acceptance criteria:**

- [ ] Calling `bindingForPane(paneId)` returns a binding for every WU-0A-11 PaneId variant.
- [ ] Calling `bindingForPane(unknown)` returns `UiShellError::UnknownPaneId`.
- [ ] Calling `deriveShellRegionState(workspaceId, activePane, selectedNodeId)` returns a ShellRegionState preserving workspace, active pane, selected node, and notification counts from seeded events.
- [ ] `deriveShellRegionState(...)` rejects empty workspace IDs and unknown pane IDs with documented errors.
- [ ] Pane routing delegates to WU-0C-34 for commands and WU-0C-35 for events; it does not query GraphStore directly.
- [ ] Every seeded Phase 0C pane remains inert: it can route, subscribe, and render placeholders backed by fixtures, but claims no VS-specific operator-visible behavior.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-34, WU-0C-35, WU-0C-36a; incoming WU-0A-11 `PaneId`, WU-0A-12 `ShellRegionState`, WU-0A-13 workspace route shell, WU-0A-14b `renderWithHarness`.

**Produces:** UI shell pane routing and ShellRegionState delegation consumed by Phase 1+ UI surfaces.

**Parallelizable with:** WU-0C-37 after WU-0C-35 if UI and audit files are disjoint.

### WU-0C-37a: AuditEmitInput DTO

**Parent initiative:** Audit event writer subscription and emit

**Contract:**
```text
schema_object: Rust struct

AuditEmitInput {
  workspace_id: string,
  audit_event_draft: AuditEventDraft,
  trace_context_id?: string,
  emit_topic?: EventTopic
}

validate_audit_emit_input(input: AuditEmitInput) -> Result<AuditEmitInput, AuditEmitError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-37a-audit-emit-input.md`, `src-tauri/src/contracts/audit_emit_input.rs`

**Code boundary:** `src-tauri/src/audit/audit_emit_input.rs`, `src-tauri/src/contracts/audit_emit_input.rs`, `src-tauri/tests/wu_0c_37a_audit_emit_input_contract.rs`

**Acceptance criteria:**

- [ ] `AuditEmitInput` round-trips through serde with workspace ID, audit event draft, trace context, and optional emit topic preserved.
- [ ] Calling `validate_audit_emit_input(input)` with valid draft refs returns the normalized DTO.
- [ ] Calling `validate_audit_emit_input(input)` with empty workspace ID or unknown emit topic returns the documented error.
- [ ] DTO validation appends no AuditEvent row and emits no IPC event.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** Incoming WU-0B-15 `AuditEvent`, WU-0A-09 `TraceContext`, WU-0A-05 `EventTopic`.

**Produces:** Audit emit input DTO consumed by WU-0C-37 and Phase 0C service emitters.

**Parallelizable with:** WU-0C-36a after WU-0C-35 if UI and audit files are disjoint.

### WU-0C-37: AuditEmitPipeline

**Parent initiative:** Audit event writer subscription and emit

**Contract:**
```text
schema_object: Rust service object

AuditEmitPipeline::append_and_emit(input: AuditEmitInput) -> Result<AuditEvent, AuditEmitError>
AuditEmitPipeline::subscribe_writer(source_name: string, event_draft_stream: AuditDraftStream) -> Result<AuditWriterSubscription, AuditEmitError>
```

**Test boundary:** `product-strategy/contracts/wu-0c-37-audit-emit-pipeline.md`, `src-tauri/src/contracts/audit_emit_pipeline.rs`

**Code boundary:** `src-tauri/src/audit/emit_pipeline.rs`, `src-tauri/src/contracts/audit_emit_pipeline.rs`, `src-tauri/tests/wu_0c_37_audit_emit_pipeline_contract.rs`

**Acceptance criteria:**

- [ ] Calling `append_and_emit(input)` appends exactly one WU-0B-15 AuditEvent row and returns it byte-equivalent to the draft plus server-owned ID/timestamp fields.
- [ ] Calling `append_and_emit(input)` with invalid policy, configuration, provider, input, or output refs returns the documented error and emits no IPC event.
- [ ] Calling `append_and_emit(input)` with `emit_topic = audit` emits exactly one WU-0C-35 `IpcEvent` on the audit topic after the row is durable.
- [ ] Calling `subscribe_writer(source_name, stream)` registers a source and rejects duplicate source names.
- [ ] Writer subscription preserves event order per source and does not reorder drafts across the same source.
- [ ] The pipeline is append-only: it exposes no update or delete path for AuditEvent rows.

**Pipeline phases:** Core path Phases 2.5 through 10 as described in the Pipeline Reference.

**Dependencies:** WU-0C-35, WU-0C-37a; incoming WU-0B-15 `AuditEvent`, WU-0A-09 `TraceContext`, WU-0A-05 `EventTopic`.

**Produces:** Durable audit writer subscription and emit pipeline consumed by all Phase 0C services and later value slices.

**Parallelizable with:** WU-0C-36 after WU-0C-35 if write sets remain disjoint.

## Round 4 Refactor Ledger

Existing WUs audited for SessionOverrideContract boundary violations:

| WU | Pre-r4 risk | Round 4 scope reduction | New dependency impact | Consolidation/removal |
|---|---|---|---|---|
| WU-0C-11a `AgentSpawnRequest` | Spawn request encoded direct provider-CLI variants and could imply harness-owned provider routing. | Replaced direct CLI field with `agents_binary`, model/project/prompt inputs, and opaque route-constraints refs. | No SessionOverrideContract dependency; no session mutation. | Kept. |
| WU-0C-13a `SessionCaptureRequest` | Request accepted transcript-locator and opencode-row style inputs. | Narrowed to `agents` output, trace, and runner session refs; no locator execution or per-CLI storage interpretation. | No SessionOverrideContract dependency; capture remains read-only. | Kept. |
| WU-0C-13b `AgentRunnerSessionCapture` | Capture methods named provider-specific session-id mechanisms. | Replaced with provider-neutral `agents_reported_session`, `trace_session`, `state_db_session`, and `substrate_gap` observations. | No SessionOverrideContract dependency; session-id capture remains `agent-runner` owned. | Kept. |
| WU-0C-13 `AgentSessionCapture` | Acceptance criteria named Claude/Codex/opencode capture behavior. | Uses fake `agents` session evidence and refuses to locate, parse, append, truncate, or replace transcripts. | No SessionOverrideContract dependency; future write-back uses WU-0C-N1. | Kept. |
| WU-0C-15a `SessionTurnRef` | Turn refs could be mistaken for provider-native transcript records. | Added opaque source offset/hash evidence and explicitly forbids provider-native record parsing. | No write dependency; canonical transcript data lives in WU-0C-N2. | Kept. |
| WU-0C-15b `SessionTurnsRequest` | Request accepted raw transcript locator input. | Removed raw locator/path input; reads normalized invocation/session/trace evidence only. | No write dependency; raw location is adapter-owned in WU-0C-N3. | Kept. |
| WU-0C-15c `SessionTurnsRead` | DTO exposed transcript locator/opencode-gap details. | Replaced with provider-neutral missing-locator/substrate-gap reporting and no raw transcript body exposure. | No write dependency; canonical transcript DTOs live in WU-0C-N2. | Kept. |
| WU-0C-15d `SessionTurnsReader` | Criteria used Claude/Codex transcript fixtures and direct parsing language. | Reads normalized `agent-runner` evidence only; never calls `replace_transcript`, `truncate_after`, or `append_turns`. | No mutation dependency; write consumers depend on WU-0C-N1. | Kept. |
| WU-0C-18 `AgentRunnerClient` | Facade could become a catch-all for session storage, resume, and provider routing. | Explicitly limited to subprocess/trace/config/diagnostics/read-only operations and excludes transcript writes. | No write dependency; SessionOverrideContract is a separate service family. | Kept. |
| WU-0C-31 `RecoveryActionProcessorSkeleton` | Recovery could need override quarantine state. | Dependency graph now admits WU-0C-N5 registry as recovery metadata input while preserving no-side-effect skeleton behavior. | Consumes WU-0C-N5 metadata only; live replacement remains blocked on pause-handshake. | Kept. |
| WU-0C-34 `TauriIpcCommandRouter` | IPC could accidentally expose raw write operations as Phase 0C commands. | Adds locate/metadata/registry read commands only; no UI command for replace/truncate/append in Phase 0C. | Depends on WU-0C-N1 and WU-0C-N5 for read surfaces. | Kept. |

No WU became empty after refactor. No WU was removed or merged. The general rule after round 4 is: read-only observation of `agents` subprocess/trace/config/session-turn evidence stays in WU-0C-11a..WU-0C-18; any transcript content mutation or raw storage write-back goes through WU-0C-N1 and the active adapter.

## Dependency Graph

Topological order with incoming cross-phase edges:

```text
Incoming Phase 0A: WU-0A-02 settings, WU-0A-03 local storage, WU-0A-04 app state, WU-0A-05 EventTopic, WU-0A-06 IpcEvent, WU-0A-07 HarnessCommand, WU-0A-08 subscribe_workspace_events, WU-0A-09 TraceContext, WU-0A-10 BackendSpanEvent, WU-0A-11 PaneId, WU-0A-12 ShellRegionState, WU-0A-13 route shell, WU-0A-14b renderWithHarness, WU-0A-15 FakeAgentsFixture.
Incoming Phase 0B: WU-0B-01..WU-0B-32 canonical GraphStore repositories and schemas, including WU-0B-13 IdentityEvent and WU-0B-27 ConflictRecord.
```

Detailed Phase 0C edges:

```text
WU-0C-01 <- WU-0B-04, WU-0B-15
WU-0C-02a <- WU-0B-04
WU-0C-02 <- WU-0C-01, WU-0C-02a, WU-0B-04
WU-0C-03 <- WU-0C-01, WU-0B-04, WU-0B-15
WU-0C-04a <- WU-0B-04, WU-0B-15
WU-0C-04 <- WU-0C-01, WU-0C-02, WU-0C-03, WU-0C-04a, WU-0B-04

WU-0C-05 <- WU-0B-16
WU-0C-06a <- WU-0B-16, WU-0B-17
WU-0C-06 <- WU-0C-05, WU-0C-06a, WU-0B-16, WU-0B-17
WU-0C-07a <- WU-0B-04, WU-0B-16
WU-0C-07 <- WU-0C-04, WU-0C-05, WU-0C-06, WU-0C-07a, WU-0B-04, WU-0B-16

WU-0C-08 <- WU-0B-05, WU-0B-06, WU-0B-32
WU-0C-09a <- WU-0B-05, WU-0B-06, WU-0B-15
WU-0C-09 <- WU-0C-08, WU-0C-09a, WU-0B-05, WU-0B-06, WU-0B-15
WU-0C-10 <- WU-0C-08, WU-0B-05, WU-0B-21, WU-0B-32

WU-0C-11a <- WU-0A-02, WU-0A-03, WU-0A-09, WU-0B-19
WU-0C-11b <- WU-0A-09
WU-0C-12a <- WU-0C-11b, WU-0A-15
WU-0C-12 <- WU-0C-11a, WU-0C-11b, WU-0C-12a, WU-0A-10, WU-0A-15
WU-0C-13a <- WU-0C-11a, WU-0C-11b, WU-0A-15
WU-0C-13b <- WU-0C-11b, WU-0A-15
WU-0C-13 <- WU-0C-11a, WU-0C-11b, WU-0C-13a, WU-0C-13b, WU-0A-15, WU-0B-20
WU-0C-14a <- WU-0C-11b, WU-0B-09
WU-0C-14b <- WU-0C-14a, WU-0B-09
WU-0C-14 <- WU-0C-11b, WU-0C-14a, WU-0C-14b, WU-0B-09
WU-0C-15a <- WU-0B-09, WU-0B-20
WU-0C-15b <- WU-0C-13b, WU-0B-09
WU-0C-15c <- WU-0C-15a, WU-0C-15b, WU-0B-09, WU-0B-20
WU-0C-15d <- WU-0C-13, WU-0C-15a, WU-0C-15b, WU-0C-15c, WU-0B-09, WU-0B-20
WU-0C-16a <- WU-0A-02, WU-0A-03
WU-0C-16b <- WU-0B-09
WU-0C-16 <- WU-0C-16a, WU-0C-16b, WU-0A-02, WU-0A-03, WU-0B-09
WU-0C-17a <- WU-0C-16b, WU-0B-17, WU-0B-18, WU-0B-19
WU-0C-17b <- WU-0B-17, WU-0B-18, WU-0B-19
WU-0C-17 <- WU-0C-12, WU-0C-13, WU-0C-14, WU-0C-15d, WU-0C-16, WU-0C-17a, WU-0C-17b, WU-0B-17, WU-0B-18, WU-0B-19
WU-0C-18 <- WU-0C-12, WU-0C-13, WU-0C-14, WU-0C-15d, WU-0C-16, WU-0C-17

WU-0C-N2 <- WU-0B-09, WU-0B-15, WU-0B-21
WU-0C-N1 <- WU-0C-N2, WU-0B-09, WU-0B-15
WU-0C-N4 <- WU-0C-N2, WU-0A-03, WU-0A-15
WU-0C-N3 <- WU-0C-N1, WU-0C-N2, WU-0C-N4, WU-0C-N5, WU-0C-16, WU-0A-03, WU-0A-15, WU-0B-09, WU-0B-15
WU-0C-N5 <- WU-0C-N1, WU-0C-N2, WU-0B-09, WU-0B-15, WU-0B-31

WU-0C-19 <- WU-0B-17, WU-0B-18, WU-0B-19
WU-0C-20a <- WU-0C-19, WU-0B-09, WU-0B-15, WU-0B-17, WU-0B-18, WU-0B-19
WU-0C-20 <- WU-0C-18, WU-0C-19, WU-0C-20a, WU-0B-09, WU-0B-15, WU-0B-17, WU-0B-18, WU-0B-19

WU-0C-21 <- WU-0B-04, WU-0B-05, WU-0B-14, WU-0B-23
WU-0C-22a <- WU-0C-21, WU-0B-05
WU-0C-22 <- WU-0C-08, WU-0C-10, WU-0C-21, WU-0C-22a
WU-0C-23a <- WU-0B-07, WU-0B-10, WU-0B-11, WU-0B-14
WU-0C-23b <- WU-0C-01, WU-0B-07, WU-0B-10, WU-0B-11, WU-0B-14
WU-0C-23 <- WU-0C-04, WU-0C-23a, WU-0C-23b, WU-0B-07, WU-0B-10, WU-0B-11, WU-0B-14
WU-0C-24a <- WU-0B-04, WU-0B-05, WU-0B-14
WU-0C-24 <- WU-0C-24a, WU-0B-04, WU-0B-05, WU-0B-14
WU-0C-25a <- WU-0C-21, WU-0C-24, WU-0B-17
WU-0C-25 <- WU-0C-07, WU-0C-21, WU-0C-24, WU-0C-25a
WU-0C-26a <- WU-0C-21, WU-0C-22a, WU-0C-24, WU-0B-15, WU-0B-17, WU-0B-19, WU-0B-21
WU-0C-26 <- WU-0C-21, WU-0C-22, WU-0C-23, WU-0C-24, WU-0C-25, WU-0C-26a, WU-0B-10, WU-0B-11, WU-0B-14, WU-0B-15, WU-0B-17, WU-0B-19, WU-0B-21, WU-0B-23

WU-0C-27 <- WU-0C-04, WU-0C-07, WU-0B-14, WU-0B-25
WU-0C-28 <- WU-0C-27, WU-0B-25, WU-0B-26, WU-0B-27
WU-0C-29a <- WU-0C-01, WU-0C-05, WU-0B-25
WU-0C-29b <- WU-0C-28, WU-0B-25
WU-0C-29 <- WU-0C-04, WU-0C-07, WU-0C-27, WU-0C-28, WU-0C-29a, WU-0C-29b
WU-0C-29c <- WU-0C-04, WU-0B-07, WU-0B-08, WU-0B-12, WU-0B-13, WU-0B-14, WU-0B-15
WU-0C-29d <- WU-0C-04, WU-0C-29c, WU-0B-13, WU-0B-15, WU-0B-25, WU-0B-26, WU-0B-27, WU-0B-29, WU-0B-30, WU-0B-31

WU-0C-30a <- WU-0B-17, WU-0B-26, WU-0B-27, WU-0B-29, WU-0B-30, WU-0B-31
WU-0C-30 <- WU-0C-30a, WU-0B-15, WU-0B-17, WU-0B-26, WU-0B-27, WU-0B-29, WU-0B-30, WU-0B-31
WU-0C-31a <- WU-0C-30, WU-0B-15, WU-0B-31
WU-0C-31 <- WU-0C-04, WU-0C-18, WU-0C-20, WU-0C-30, WU-0C-31a, WU-0C-N5

WU-0C-32 <- WU-0C-11a, WU-0C-11b, WU-0C-12, WU-0C-13
WU-0C-33a <- WU-0C-32, WU-0B-19
WU-0C-33 <- WU-0C-11a, WU-0C-11b, WU-0C-12, WU-0C-13, WU-0C-32, WU-0C-33a, WU-0B-19

WU-0C-34 <- WU-0C-04, WU-0C-07, WU-0C-08, WU-0C-09, WU-0C-10, WU-0C-18, WU-0C-20, WU-0C-26, WU-0C-27, WU-0C-29c, WU-0C-29d, WU-0C-30, WU-0C-N1, WU-0C-N5, WU-0A-04, WU-0A-07
WU-0C-35a <- WU-0A-05, WU-0A-09, WU-0B-15
WU-0C-35 <- WU-0C-35a, WU-0A-05, WU-0A-06, WU-0A-08, WU-0A-09, WU-0A-10
WU-0C-36a <- WU-0A-05, WU-0A-07, WU-0A-11, WU-0A-12
WU-0C-36 <- WU-0C-34, WU-0C-35, WU-0C-36a, WU-0A-11, WU-0A-12, WU-0A-13, WU-0A-14b
WU-0C-37a <- WU-0B-15, WU-0A-05, WU-0A-09
WU-0C-37 <- WU-0C-35, WU-0C-37a, WU-0B-15, WU-0A-05, WU-0A-09
```

Acyclicity check:

- All split DTO WUs sit upstream of their corresponding method-bearing service WUs.
- `WU-0C-22` is strictly after `WU-0C-10`, and `WU-0C-23` is strictly after `WU-0C-04`; neither remains in the same wave as its prerequisite.
- `WU-0C-29c` and `WU-0C-29d` record existing Phase 0B identity/conflict schemas without topology mutation, so they do not create a cycle with optimizer or recovery services.
- `WU-0C-N1..WU-0C-N5` sit after their DTO/probe prerequisites and before any downstream session transcript write-back consumer; adapter WUs do not depend on VS-010/012/018 value behavior.
- `WU-0C-34` command routing depends on service surfaces, but services do not depend on command routing.
- `WU-0C-37` audit emit depends on event emission and AuditEvent repository, while services emit only audit drafts, so no service-to-audit-to-service cycle exists.

## Critical Path

Longest Phase 0C internal path:

```text
WU-0C-01 PolicyDecision DTO
  -> WU-0C-02 PolicyRegistry
  -> WU-0C-04 PolicyEngine Gate Evaluator
  -> WU-0C-07 BudgetGateService
  -> WU-0C-25 RenderBudgetGateAdapter
  -> WU-0C-26 RenderEngine Core
  -> WU-0C-34 TauriIpcCommandRouter
  -> WU-0C-36 UiShellPaneRouter
```

Agent-runner/provider/recovery path:

```text
WU-0C-11b OulipolyInvocationRef
  -> WU-0C-12a AgentSpawnResult
  -> WU-0C-12 AgentSubprocessSupervisor
  -> WU-0C-17 ProviderDiagnosticsAdapter
  -> WU-0C-18 AgentRunnerClient
  -> WU-0C-20 ProviderStateMonitor
  -> WU-0C-31 RecoveryActionProcessorSkeleton
```

Session override path:

```text
WU-0C-N2 TranscriptTurn/SessionLocation/SessionMetadata DTOs
  -> WU-0C-N1 SessionOverrideContract trait
  -> WU-0C-N5 SessionOverrideStore registry
WU-0C-N2 TranscriptTurn/SessionLocation/SessionMetadata DTOs
  -> WU-0C-N4 AgentRunnerSchemaProbe
  -> WU-0C-N3 AgentRunnerDbAdapter v1
```

Optimizer and identity/conflict paths:

```text
WU-0C-04 PolicyEngine Gate Evaluator
  -> WU-0C-07 BudgetGateService
  -> WU-0C-27 OptimizerQueueService
  -> WU-0C-28 OptimizerCycleStateMachine
  -> WU-0C-29b OptimizerCycleLease
  -> WU-0C-29 OptimizerScheduler

WU-0C-04 PolicyEngine Gate Evaluator
  -> WU-0C-29c IdentityResolverRuntime
  -> WU-0C-29d ConflictRecordWriterShell
```

The critical path gates Phase 1 VS-001 because RenderEngine needs policy, budget, configuration, provider/capability, working-set, IPC, UI shell, and audit-draft compatibility before an imposed-context inspector can be accepted.

## Parallelization Map

Parallel groups are derived by topological depth from the graph above. No wave contains a direct or transitive dependency edge between members.

| Wave | WUs | Max concurrent worktrees | Notes |
|---:|---|---:|---|
| 1 | WU-0C-01, WU-0C-02a, WU-0C-04a, WU-0C-05, WU-0C-06a, WU-0C-07a, WU-0C-08, WU-0C-09a, WU-0C-11a, WU-0C-11b, WU-0C-15a, WU-0C-16a, WU-0C-16b, WU-0C-17b, WU-0C-N2, WU-0C-19, WU-0C-21, WU-0C-23a, WU-0C-24a, WU-0C-30a, WU-0C-35a, WU-0C-36a, WU-0C-37a | 23 | External 0A/0B-only DTOs, primitive service DTOs, and session-override DTOs. |
| 2 | WU-0C-02, WU-0C-03, WU-0C-06, WU-0C-09, WU-0C-10, WU-0C-12a, WU-0C-13a, WU-0C-13b, WU-0C-14a, WU-0C-17a, WU-0C-N1, WU-0C-N4, WU-0C-20a, WU-0C-22a, WU-0C-23b, WU-0C-24, WU-0C-29a, WU-0C-30, WU-0C-35 | 19 | First derived DTOs, leaf services, SessionOverrideContract trait, and schema probe. |
| 3 | WU-0C-04, WU-0C-12, WU-0C-13, WU-0C-14b, WU-0C-15b, WU-0C-16, WU-0C-N5, WU-0C-22, WU-0C-25a, WU-0C-26a, WU-0C-31a, WU-0C-37 | 12 | Policy, agent-runner basics, override registry, render policy resolver, render budget/result DTOs, audit emit. |
| 4 | WU-0C-07, WU-0C-14, WU-0C-15c, WU-0C-N3, WU-0C-23, WU-0C-29c, WU-0C-32 | 7 | Budget gate, trace reader, session override v1 adapter, privilege filter, identity shell, hook payload. |
| 5 | WU-0C-15d, WU-0C-25, WU-0C-27, WU-0C-29d, WU-0C-33a | 5 | Session turns service, render budget adapter, optimizer queue, conflict writer, plugin capability DTO. |
| 6 | WU-0C-17, WU-0C-26, WU-0C-28, WU-0C-33 | 4 | Provider diagnostics, render core, optimizer cycle, plugin matrix. |
| 7 | WU-0C-18, WU-0C-29b | 2 | Agent facade and optimizer lease DTO. |
| 8 | WU-0C-20, WU-0C-29 | 2 | Provider monitor and optimizer scheduler. |
| 9 | WU-0C-31, WU-0C-34 | 2 | Recovery processor and command router join service surfaces. |
| 10 | WU-0C-36 | 1 | UI shell routing joins commands and events. |

Parallel write rule:

- Do not let same-wave WUs edit the same generated Rust contract index, TypeScript contract index, Tauri command registration module, or event-topic registry in parallel.
- If codegen produces shared indexes, land per-WU generated files independently and reserve shared-index updates for a small integration patch after the wave.
- No same-wave WU shares a migration file; WU-0C-N5 is the only round-4 WU that may require a small override-ledger persistence addition, and it is not a canonical workspace session-id table.

## Run Report (D1/D2/D3/D4)

### Rule D1 - Per-Schema-Object and Per-Service Granularity

Phase 0C schema/service ownership now enumerates every Rust struct, TypeScript DTO, IPC command type, state machine, and service object declared in this artifact. No ad-hoc D1 exceptions are used.

| Object or component | Owning WU | D1 status |
|---|---|---|
| `PolicyDecision` | WU-0C-01 | Own WU. |
| `PolicyGateDescriptor` | WU-0C-02a | Own WU. |
| `PolicyRegistry` | WU-0C-02 | Own WU. |
| `PolicyValidationReport` | WU-0C-03 | Own WU. |
| `PolicyGateInput` | WU-0C-04a | Own WU. |
| `PolicyEngine` gate service | WU-0C-04 | Own WU. |
| `BudgetDecision` | WU-0C-05 | Own WU. |
| `BudgetUsageDraft` | WU-0C-06a | Own WU. |
| `BudgetLedgerAccountingService` | WU-0C-06 | Own WU. |
| `BudgetCheckRequest` | WU-0C-07a | Own WU. |
| `BudgetGateService` | WU-0C-07 | Own WU. |
| `ConfigurationRegistryRuntimeRead` | WU-0C-08 | Own WU. |
| `ConfigurationRevisionDraft` | WU-0C-09a | Own WU. |
| `ConfigurationRegistryWriteApi` | WU-0C-09 | Own WU. |
| `ConfigurationProvenanceResolver` | WU-0C-10 | Own WU. |
| `AgentSpawnRequest` | WU-0C-11a | Own WU. |
| `OulipolyInvocationRef` | WU-0C-11b | Own WU. |
| `AgentSpawnResult` | WU-0C-12a | Own WU. |
| `AgentSubprocessSupervisor` | WU-0C-12 | Own WU. |
| `SessionCaptureRequest` | WU-0C-13a | Own WU. |
| `AgentRunnerSessionCapture` | WU-0C-13b | Own WU. |
| `AgentSessionCapture` | WU-0C-13 | Own WU. |
| `AgentRunnerTraceEdge` | WU-0C-14a | Own WU. |
| `AgentRunnerTraceTree` | WU-0C-14b | Own WU. |
| `AgentTraceTreeReader` | WU-0C-14 | Own WU. |
| `SessionTurnRef` | WU-0C-15a | Own WU. |
| `SessionTurnsRequest` | WU-0C-15b | Own WU. |
| `SessionTurnsRead` | WU-0C-15c | Own WU. |
| `SessionTurnsReader` | WU-0C-15d | Own WU. |
| `ConfigSnapshotRequest` | WU-0C-16a | Own WU. |
| `AgentRunnerConfigSnapshot` | WU-0C-16b | Own WU. |
| `AgentRunnerConfigSnapshotReader` | WU-0C-16 | Own WU. |
| `ProviderDiagnosticsRequest` | WU-0C-17a | Own WU. |
| `ProviderDiagnosticsResult` | WU-0C-17b | Own WU. |
| `ProviderDiagnosticsAdapter` | WU-0C-17 | Own WU. |
| `AgentRunnerClient` facade | WU-0C-18 | Own WU; delegates to separately owned DTO/service WUs. |
| `TranscriptTurn` / `SessionLocation` / `SessionMetadata` DTO family | WU-0C-N2 | Own WU. |
| `SessionOverrideContract` trait | WU-0C-N1 | Own WU. |
| `AgentRunnerSchemaProbe` | WU-0C-N4 | Own WU. |
| `AgentRunnerDbAdapter` v1 | WU-0C-N3 | Own WU. |
| `SessionOverrideStore` registry | WU-0C-N5 | Own WU. |
| `ProviderProbeRequest` | WU-0C-19 | Own WU. |
| `ProviderStateMonitorResult` | WU-0C-20a | Own WU. |
| `ProviderStateMonitor` | WU-0C-20 | Own WU. |
| `RenderRequestDto` | WU-0C-21 | Own WU. |
| `RenderPolicy` | WU-0C-22a | Own WU. |
| `RenderPolicyResolver` | WU-0C-22 | Own WU. |
| `RenderPrivilegeFilterInput` | WU-0C-23a | Own WU. |
| `RenderPrivilegeFilterResult` | WU-0C-23b | Own WU. |
| `RenderPrivilegeFilter` | WU-0C-23 | Own WU. |
| `CachePrefixInput` | WU-0C-24a | Own WU. |
| `CachePrefixHasher` | WU-0C-24 | Own WU. |
| `RenderBudgetRequest` | WU-0C-25a | Own WU. |
| `RenderBudgetGateAdapter` | WU-0C-25 | Own WU. |
| `RenderResultDto` | WU-0C-26a | Own WU. |
| `RenderEngine` service | WU-0C-26 | Own WU. |
| `OptimizerQueueService` | WU-0C-27 | Own WU. |
| `OptimizerCycleStateMachine` | WU-0C-28 | Own WU. |
| `OptimizerScheduleDecision` | WU-0C-29a | Own WU. |
| `OptimizerCycleLease` | WU-0C-29b | Own WU. |
| `OptimizerScheduler` | WU-0C-29 | Own WU. |
| `IdentityResolverRuntime` | WU-0C-29c | Own WU. |
| `ConflictRecordWriterShell` | WU-0C-29d | Own WU. |
| `RecoveryActionDraft` | WU-0C-30a | Own WU. |
| `RecoveryActionWriter` | WU-0C-30 | Own WU. |
| `RecoveryProcessorDecision` | WU-0C-31a | Own WU. |
| `RecoveryActionProcessorSkeleton` | WU-0C-31 | Own WU. |
| `HookPayload` | WU-0C-32 | Own WU. |
| `PluginCapability` | WU-0C-33a | Own WU. |
| `PluginCapabilityMatrix` | WU-0C-33 | Own WU. |
| `TauriIpcCommandRouter` | WU-0C-34 | Own WU. |
| `DomainEventDraft` | WU-0C-35a | Own WU. |
| `IpcEventEmissionBackbone` | WU-0C-35 | Own WU. |
| `PaneRouteBinding` | WU-0C-36a | Own WU. |
| `UiShellPaneRouter` | WU-0C-36 | Own WU. |
| `AuditEmitInput` | WU-0C-37a | Own WU. |
| `AuditEmitPipeline` | WU-0C-37 | Own WU. |

### Rule D2 - Functional Contract Criteria

Every WU declaring a function, method, enum, state machine, or routing invariant has binary criteria in its Acceptance criteria block. New split DTO WUs include validator criteria, variant reachability where applicable, and no-side-effect criteria.

| WU group | D2 coverage |
|---|---|
| Policy/budget/config | DTO validators plus `PolicyEngine`, budget accounting/gate, config read/write/provenance method criteria. |
| Agent-runner | Split DTO validators plus subprocess, capture, trace, normalized session turns, config snapshot, diagnostics, and facade method criteria; no provider routing or transcript write-back. |
| Session override | DTO validators, trait-level fake adapter criteria, schema probe compatibility criteria, v1 adapter locate/read/write/crash-recovery criteria, and registry lifecycle criteria. |
| Provider/render | Provider probe/monitor validators, render policy/privilege/budget/result validators, cache hash, render core, and golden fixture criteria. |
| Optimizer/identity/conflict/recovery | Queue/state-machine/scheduler criteria, identity/conflict routing invariants, recovery writer/processor transitions. |
| Hook/IPC/UI/audit | Hook/plugin validators, command routing invariant, event topic isolation, pane routing, audit append/order invariants. |

No WU relies on generic "service works" language. Method criteria name the exact call, valid result shape, and documented error variants. Enum criteria require variant round-trip plus reachability through fixture inputs. State machines require valid transitions, invalid transition rejection, and reachability from initial states.

### Rule D3 - Fix-Created Prevention

Orphan check:

- Every DTO removed from a service contract now has its own WU with Contract, Test boundary, Code boundary, Acceptance criteria, Dependencies, Produces, and Parallelizable entries.
- F-1 is closed by WU-0C-15a `SessionTurnRef`, WU-0C-15b `SessionTurnsRequest`, WU-0C-15c `SessionTurnsRead`, and WU-0C-15d `SessionTurnsReader`.
- F-2 is closed systemically across policy, budget, config, agent-runner, provider, render, optimizer, recovery, hook/plugin, IPC, UI, and audit DTO/service families.
- F-3 is closed by WU-0C-11a `AgentSpawnRequest` and WU-0C-11b `OulipolyInvocationRef` + parser.
- Coverage F-1 is closed by WU-0C-29c `IdentityResolverRuntime` and WU-0C-29d `ConflictRecordWriterShell`; Phase 0B remains schema owner for `IdentityEvent` and `ConflictRecord`.

Edge re-route check:

- All original incoming edges into bundled WUs were routed to either the DTO WU, the service WU, or both.
- `WU-0C-12/13/14/15d/16/17/18` now depend on the exact agent-runner DTO/service prerequisites rather than broad WU-0C-11 or WU-0C-15 bundles.
- RenderEngine joins `WU-0C-22a`, `WU-0C-23a`, `WU-0C-23b`, `WU-0C-25a`, and `WU-0C-26a` explicitly before WU-0C-26.
- Audit writing remains centralized in WU-0C-37; services emit drafts and do not depend on durable audit append.

Regression check:

- Phase 0A UI/IPC shell primitives remain canonical; Phase 0C extends them without moving files or changing meanings.
- Phase 0B durable schemas remain canonical; Phase 0C does not duplicate `IdentityEvent`, `ConflictRecord`, `PolicySet`, `BudgetLedger`, `ProviderState`, `WorkingSetSnapshot`, `OptimizerRequest`, `RecoveryAction`, or `AuditEvent` ownership.
- Parallelization Map is re-derived by topological depth from the declared graph and contains no intra-wave dependencies.
- SessionOverrideContract WUs add five new nodes: DTOs in wave 1, trait/probe in wave 2, registry in wave 3, and v1 adapter in wave 4. Existing session-turn WUs were narrowed to normalized evidence; no WU became empty or required removal.

### Rule D4 - Watch-Signal Compliance

| Active watch signal | Phase 0C round 4 handling |
|---|---|
| `bundling-family` | Applied Rule D1 strictly to every named DTO/service family, including the parent-loop SessionTurns pattern and every input/output DTO listed in R1-DECOMP-F02. |
| `state-machine-criteria-family` | Preserved the round-1 D2 criteria and added binary criteria for all new validators, identity/conflict methods, and routing invariants. |
| `fix-created-family` | Round 4 classification is fix-created-family gen 0 from the proposal-r5/engineering-roadmap-r4 cascade. New concerns are assigned to WU-0C-N1..WU-0C-N5; existing agent-runner/session-turn WUs are narrowed rather than deleted. |
| `dependency-encoding-family` | Foundation-row outgoing blocks now declared for all Phase 0C foundation rows in engineering-roadmap lines 23-46; per-VS Outgoing-to-Phase-1 / Outgoing-to-Phase-2+ blocks systematically derived from the (foundation-row, needed-by VS) pairs in the foundation table cross-checked against per-slice Foundation dependencies (lines 50-475); cross-slice OptimizerRequest contract explicitly enumerated. |
| `session-override-boundary-family` | New in round 4. Provider routing, account/quota/auth, resume composition, session porting, and per-CLI storage knowledge are excluded from general harness WUs; direct DB/JSONL writes are isolated to WU-0C-N3 behind WU-0C-N1. |

Self-classification:

- same-label: 0 known in round 2 after edits.
- same-family: round 1 had same-family bundling and dependency-map findings inherited from the parent loop; this revision applies systemic D1 and topological-wave fixes instead of named-WU patches.
- fix-created: 0 known after re-route audit.
- two-generation: historical only.
- named three-generation / four-generation: parent-loop bundling-family and dependency-encoding-family acknowledged; this round explicitly closes the same-family recurrence.
- round-3 brownfield: Stitch Notes outgoing-edge declarations re-derived systematically from engineering-roadmap Phase 0 foundation table; same-family `dependency-encoding-family` at generation 2 closed by re-derivation rather than per-edge patch.
- round-4 brownfield: SessionOverrideContract cascade integrated from proposal-r5 and engineering-roadmap-r4; fix-created-family gen 0 watch active for new adapter/trait dependencies.

## Stitch Notes

Incoming edges from Phase 0A to preserve:

- WU-0C-11a through WU-0C-18 consume settings, storage, tracing, backend spans, and fake `agents`; do not create a second fake subprocess substrate.
- WU-0C-34 consumes `HarnessCommand`; do not define command-name strings outside the 0A command taxonomy extension path.
- WU-0C-35a/WU-0C-35 consume `EventTopic`, `IpcEvent<T>`, and `subscribe_workspace_events`; do not create a second event bus.
- WU-0C-36a/WU-0C-36 consume `PaneId`, `ShellRegionState`, the workspace route shell, and `renderWithHarness`; do not create a second shell view-state model.
- WU-0C-37a/WU-0C-37 consume `TraceContext` and `EventTopic` for audit event emission.
- WU-0C-N3 consumes WU-0A-03 local storage and WU-0A-15 fake `agents`/temp SQLite fixtures; do not create a second local fixture substrate.

Incoming edges from Phase 0B to preserve:

- PolicyEngine uses WU-0B-04 `PolicySet` and WU-0B-15 `AuditEvent` only through repository/service boundaries.
- Budget services use WU-0B-16 `BudgetLedger` and WU-0B-17 `ProviderState`; budget policy actions keep WU-0B-16 semantics.
- Configuration runtime uses WU-0B-05 `GraphConfiguration`, WU-0B-06 `GraphWorkspace`, and WU-0B-32 registry skeleton/read repository.
- ProviderStateMonitor writes WU-0B-17, WU-0B-18, WU-0B-19 records and never stores secrets.
- RenderEngine consumes WU-0B-10, WU-0B-11, WU-0B-14, WU-0B-17, WU-0B-19, WU-0B-21, and WU-0B-23.
- Optimizer shell consumes WU-0B-25, WU-0B-26, and WU-0B-27 but does not run `glm` or merge graph edits.
- Identity/conflict shell writes WU-0B-13 `IdentityEvent` and WU-0B-27 `ConflictRecord` records without mutating topology or resolving conflicts.
- Recovery shell consumes WU-0B-31 and related worker/question/provider/conflict refs without executing recovery side effects.
- SessionOverrideContract WU-0C-N1..WU-0C-N5 consume WU-0B-09 EvidenceArtifact and WU-0B-15 AuditEvent refs for receipts/refusals, and WU-0B-31 RecoveryAction refs for quarantine handoff. They do not define a new graph source of truth.

Outgoing edges by engineering-roadmap Phase 0 foundation row:

- Append-only audit writer / AuditEvent base table WU-0C-37 feed VS-001, VS-002, VS-003, VS-004, VS-005, VS-006, VS-007, VS-008, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-015, VS-016, VS-017, VS-018, VS-019, VS-020, and VS-021.
- RenderEngine core WU-0C-21..WU-0C-26 feed VS-001, VS-002, VS-004, VS-005, VS-006, VS-007, VS-008, VS-009, VS-014, VS-015, VS-017, and VS-020.
- PolicyEngine deterministic gate framework WU-0C-01..WU-0C-04 feed VS-002, VS-003, VS-004, VS-006, VS-008, VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-015, VS-017, VS-018, VS-019, VS-020, and VS-021.
- ConfigurationRegistry WU-0C-08..WU-0C-10 feed VS-001, VS-002, VS-004, VS-005, VS-009, VS-010, VS-011, VS-015, and VS-020.
- BudgetLedger WU-0C-05..WU-0C-07 feed VS-001, VS-004, VS-008, VS-009, VS-010, VS-015, VS-016, VS-019, VS-020, and VS-021.
- ProviderStateMonitor and provider/capability records WU-0C-19/WU-0C-20 feed VS-001, VS-006, VS-015, VS-016, VS-017, VS-020, and VS-021.
- CLI subprocess supervisor around `agents` WU-0C-11a..WU-0C-18 feed VS-001, VS-003, VS-006, VS-009, VS-015, VS-016, VS-017, VS-020, and VS-021.
- SessionOverrideContract WU-0C-N1..WU-0C-N5 feed VS-010, VS-012, VS-018, VS-020, VS-021, and downstream Phase 1/2/3 WUs that need transcript write-back receipts, refusal reasons, or override metadata.
- Optimizer queue, OptimizerRequest/OptimizerEdit store, and merge-validation shell WU-0C-27..WU-0C-29 feed VS-009, VS-010, VS-011, VS-012, VS-013, VS-014, VS-018, VS-019, and VS-020.
- Identity/conflict shell WU-0C-29c/WU-0C-29d feed VS-012, VS-013, VS-017, VS-018, VS-020, and VS-021.
- RecoveryAction schema, side-effect taxonomy, and audit linkage WU-0C-30/WU-0C-31 feed VS-003, VS-006, VS-013, VS-017, VS-018, VS-020, and VS-021.
- Hook/MCP/plugin scaffold WU-0C-32/WU-0C-33 feed VS-001, VS-003, VS-006, VS-008, VS-009, VS-015, VS-017, and VS-018.
- Tauri IPC commands and Channel events WU-0C-34/WU-0C-35 feed VS-001, VS-005, VS-006, VS-007, VS-010, VS-016, VS-017, VS-020, and VS-021.
- Single-tab UI shell, route/layout primitives, and seeded graph-addressed panes WU-0C-36 feed VS-001, VS-005, VS-006, VS-007, VS-010, VS-016, VS-017, VS-019, VS-020, and VS-021.

Cross-slice contract: OptimizerRequest emission (engineering-roadmap lines 606-616). Emitters -> WU-0C-27 (OptimizerQueueService): VS-005 (user_surface configuration warnings), VS-009 (orchestrator_turn), VS-010 (backend_signal), VS-011 (user_surface), VS-018 (worker_output), VS-020 (recovery_action). All six VSes consume WU-0C-27 directly; the queue does not interpret payloads, and Phase 2 owns the optimizer cycle.

Outgoing to Phase 1:

- VS-001 consumes configuration WU-0C-08..WU-0C-10; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; agent-runner WU-0C-11a..WU-0C-18; provider WU-0C-19/WU-0C-20; hook/plugin WU-0C-32/WU-0C-33; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.
- VS-002 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; render WU-0C-21..WU-0C-26; UI WU-0C-36; and audit WU-0C-37.
- VS-003 consumes policy WU-0C-04; agent-runner WU-0C-11a..WU-0C-18; recovery WU-0C-30/WU-0C-31; hook/plugin WU-0C-32/WU-0C-33; IPC/events WU-0C-34/WU-0C-35; and audit WU-0C-37.
- VS-004 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.
- VS-005 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; render WU-0C-21..WU-0C-26; optimizer queue WU-0C-27; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.
- VS-006 consumes policy WU-0C-04; render WU-0C-21..WU-0C-26; agent-runner WU-0C-11a..WU-0C-18; provider WU-0C-19/WU-0C-20; recovery WU-0C-30/WU-0C-31; hook/plugin WU-0C-32/WU-0C-33; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.
- VS-007 consumes render WU-0C-21..WU-0C-26; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.

Outgoing to Phase 2+:

- VS-008 consumes policy WU-0C-04; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; hook/plugin WU-0C-32/WU-0C-33; and audit WU-0C-37.
- VS-009 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; agent-runner WU-0C-11a..WU-0C-18; provider WU-0C-19/WU-0C-20; optimizer WU-0C-27..WU-0C-29; hook/plugin WU-0C-32/WU-0C-33; IPC/events WU-0C-34/WU-0C-35; and audit WU-0C-37.
- VS-010 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; optimizer WU-0C-27..WU-0C-29; SessionOverrideContract WU-0C-N1..WU-0C-N5 for turn-decomposition/detail-injection write-back; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37. **Blocked-on:** v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-010 uses WU-0C-N3 v1 idle-only write-back.
- VS-011 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; optimizer WU-0C-27..WU-0C-29; UI WU-0C-36; and audit WU-0C-37.
- VS-012 consumes policy WU-0C-04; optimizer WU-0C-27..WU-0C-29; identity/conflict shell WU-0C-29c/WU-0C-29d; SessionOverrideContract WU-0C-N1..WU-0C-N5 for repack transcript replacement; and audit WU-0C-37; Phase 3 still owns topology mutation and conflict-resolution behavior. **Blocked-on:** v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-012 uses WU-0C-N3 v1 idle-only replacement.
- VS-013 consumes policy WU-0C-04; optimizer WU-0C-27..WU-0C-29; identity/conflict shell WU-0C-29c/WU-0C-29d; recovery WU-0C-30/WU-0C-31; and audit WU-0C-37; Phase 3 still owns topology mutation and conflict-resolution behavior.
- VS-014 consumes policy WU-0C-04; render WU-0C-21..WU-0C-26; optimizer WU-0C-27..WU-0C-29; UI WU-0C-36; and audit WU-0C-37.
- VS-015 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; agent-runner WU-0C-11a..WU-0C-18; provider WU-0C-19/WU-0C-20; hook/plugin WU-0C-32/WU-0C-33; and audit WU-0C-37.
- VS-016 consumes budget WU-0C-05..WU-0C-07; agent-runner WU-0C-11a..WU-0C-18; provider WU-0C-19/WU-0C-20; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.
- VS-017 consumes policy WU-0C-04; render WU-0C-21..WU-0C-26; agent-runner WU-0C-11a..WU-0C-18; provider WU-0C-19/WU-0C-20; identity/conflict shell WU-0C-29c/WU-0C-29d; recovery WU-0C-30/WU-0C-31; hook/plugin WU-0C-32/WU-0C-33; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37.
- VS-018 consumes policy WU-0C-04; optimizer WU-0C-27..WU-0C-29; identity/conflict shell WU-0C-29c/WU-0C-29d; SessionOverrideContract WU-0C-N1..WU-0C-N5 for accepted worker-output write-back; recovery WU-0C-30/WU-0C-31; hook/plugin WU-0C-32/WU-0C-33; UI WU-0C-36; and audit WU-0C-37. **Blocked-on:** v2 adapter migration needs `agents session locate/export/import-replace`; atomic mid-session override needs `agents pause-handshake`. Until then VS-018 may stage accepted output and uses WU-0C-N3 only when session-idle.
- VS-019 consumes policy WU-0C-04; budget WU-0C-05..WU-0C-07; agent-runner WU-0C-11a..WU-0C-18; optimizer WU-0C-27..WU-0C-29; UI WU-0C-36; and audit WU-0C-37.
- VS-020 consumes policy WU-0C-04; configuration WU-0C-08..WU-0C-10; budget WU-0C-05..WU-0C-07; render WU-0C-21..WU-0C-26; agent-runner WU-0C-11a..WU-0C-18; SessionOverrideContract WU-0C-N1..WU-0C-N5 for refusal/quarantine metadata; provider WU-0C-19/WU-0C-20; optimizer WU-0C-27..WU-0C-29; identity/conflict shell WU-0C-29c/WU-0C-29d; recovery WU-0C-30/WU-0C-31; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37. **Blocked-on:** recovery actions that need live mid-session replacement require `agents pause-handshake`; v2 replacement requires `agents session import-replace`.
- VS-021 consumes policy WU-0C-04; budget WU-0C-05..WU-0C-07; agent-runner WU-0C-11a..WU-0C-18; SessionOverrideContract WU-0C-N1..WU-0C-N5 for changed-session-contract evidence; provider WU-0C-19/WU-0C-20; identity/conflict shell WU-0C-29c/WU-0C-29d; recovery WU-0C-30/WU-0C-31; IPC/events WU-0C-34/WU-0C-35; UI WU-0C-36; and audit WU-0C-37. **Blocked-on:** v2 reroute/write-back migration requires `agents session locate/export/import-replace` and schema-version probe.

Cross-phase outgoing edges added in round 4:

- SessionOverrideContract WU-0C-N1..WU-0C-N5 -> Phase 1 worker-launcher and worker-output-reintegration WUs for override receipt display, refusal surfacing, and accepted-output write-back prerequisites.
- SessionOverrideContract WU-0C-N1..WU-0C-N5 -> Phase 2 turn-decomposition and detail-injection-router WUs; those WUs may produce canonical turns or packed transcripts but must call `append_turns`, `truncate_after`, or `replace_transcript` through WU-0C-N1.
- SessionOverrideContract WU-0C-N1..WU-0C-N5 -> Phase 3 repack-planner WUs; those WUs may plan packed transcript replacement but must not open CLI JSONL files directly.
- Downstream Phase 1/2/3 revisions should add explicit incoming-from-Phase-0C edges back to WU-0C-N1..WU-0C-N5 where worker launch, worker reintegration, turn decomposition, detail injection, or repack planning consumes session override receipts/refusals.

Explicit non-ownership notes:

- Phase 0C does not own GraphStore migrations or durable schema tables.
- Phase 0C does not own operator-visible panes beyond inert route/event/command wiring.
- Phase 0C does not launch real workers, resume questions as a product workflow, run optimizer edit drafting, merge optimizer edits, resolve conflicts, mutate topology, execute recovery side effects, sample reviewers, or reroute providers.
- Phase 0C does not replace `agent-runner`; it creates a subprocess/trace/config wrapper around the installed `/home/nes/.local/bin/agents` substrate plus a versioned session-override trait.
- Phase 0C does not reimplement provider routing, multi-account load balancing, quota tracking, auth refresh, `--resume` mechanics, cross-provider session porting, or session-id capture.
- Direct `state.db` and per-CLI JSONL mutation is allowed only inside WU-0C-N3 `AgentRunnerDbAdapter` v1, pinned by WU-0C-N4 and replaceable by a future WU-0C-N1-compatible `AgentRunnerCliAdapter` after `agents session locate/export/import-replace`, pause-handshake, and schema-version probe land.
