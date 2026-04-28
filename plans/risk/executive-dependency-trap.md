# Executive — Dependency Trap Risk Assessment

**Rating: LOW**

## Findings

### F-1. Critical-path length is reasonable

**Severity: NONE**

The longest declared dependency chain in the executive roadmap traverses 6 value slices (5 edges) out of 21 total slices, matching the 7-phase phase count exactly. Representative critical paths:

- `VS-003` (provenance) → `VS-009` (bounded turns) → `VS-015` (worker dispatch) → `VS-018` (reintegration staging) → `VS-020` (recovery accounting) → `VS-021` (provider reroute)
- `VS-006` (provider preflight) → `VS-009` → `VS-015` → `VS-017` (routed questions) → `VS-020` → `VS-021`
- `VS-002` (summary contracts) → `VS-010` (summary refresh) → `VS-013` (snapshot conflicts) → `VS-018` → `VS-020` → `VS-021`

Six nodes deep on a 21-slice graph (≈29%) is short relative to a 7-phase roadmap and matches what the proposal architecture demands: provenance/budget/preflight precede bounded turns, bounded turns precede dispatch, dispatch precedes reintegration, reintegration precedes recovery accounting, and recovery accounting precedes provider-aware reroute. Each link is required by a proposal commitment (`Snapshot-Walk-Then-Merge`, `Provider State as Observable State`, `Questions as Continuations`, `WorkerDispatcher` not directly merging output, `RecoveryAction` preceding reroute), so the path cannot be shortened by reordering without violating the proposal.

**Recommendation:** No action required.

---

### F-2. Multiple foundational slices block 3+ downstream — single-point-of-failure pattern is structural, not fragile

**Severity: LOW**

Direct downstream fanout (count of declared dependents) per slice:

| Slice | Direct downstream | Downstream slices |
|---|---|---|
| `VS-003` Tool provenance | 6 | `VS-009`, `VS-010`, `VS-014`, `VS-018`, `VS-019`, `VS-020` |
| `VS-010` Summary refresh / stale | 6 | `VS-011`, `VS-012`, `VS-013`, `VS-014`, `VS-018`, `VS-019` |
| `VS-004` Budget gates | 5 | `VS-008`, `VS-009`, `VS-010`, `VS-015`, `VS-019` |
| `VS-006` Provider preflight | 5 | `VS-009`, `VS-015`, `VS-017`, `VS-020`, `VS-021` |
| `VS-015` Worker dispatch | 4 | `VS-016`, `VS-017`, `VS-018`, `VS-021` |
| `VS-002` Summary contract | 3 | `VS-008`, `VS-010`, `VS-014` |

Six slices each block 3+ downstream. Read strictly, the LOW criterion ("no single slice blocks more than 2 downstream") is not satisfied. However, the spirit of the criterion — a *fragile* bottleneck — is satisfied for the following reasons:

1. Each high-fanout slice corresponds to a proposal-level architectural primitive that is consumed by many later capabilities by design:
   - `VS-003` produces `ToolCallProvenance` / `EvidenceArtifact` / `AuditEvent`, which the proposal defines as "part of the graph contract — raw evidence preserved separately from derived summaries, addressable from any node that derives from it" (`problem.md` §10).
   - `VS-010` is the first `OptimizerEdit` producer, and the `Optimizer` component is named in the proposal as the only writer of summary regenerations, stale markers, cross-references, repacks, and topology edits.
   - `VS-004` instruments `BudgetLedger` over renders, turns, workers, optimizer passes, and reviewer passes — by definition consumed by every later actor (proposal commitment "Cost as Correctness").
   - `VS-006` produces `ProviderState` / `EntitlementSnapshot` / `CapabilityFingerprint`, which the proposal commits to consulting before every worker launch, recovery, and reroute.
   - `VS-015` is the only producer of `WorkerSlice` / `WorkerRun`, by construction.
   - `VS-002` is the only producer of `SummaryContract` validation, by construction.

2. None of these slices can be split without re-decomposing the proposal's component boundaries. `VS-003` cannot ship "tool calls only" without `EvidenceArtifact` because the provenance contract requires both. `VS-006` cannot ship "auth probe only" without `CapabilityFingerprint` because the proposal commits to `ProviderStateMonitor` producing both. `VS-010` is the smallest viable optimizer increment; it already excludes split/merge/reparent/cross-references (those are `VS-012`).

3. The bottlenecks are concentrated in Phase 1 (instrumentation) and the first slice of Phase 2 (`VS-010`). These are narrow, well-defined capabilities whose implementation risk is low relative to the later workflow slices that consume them. Fanout × per-slice risk (the operative measure of fragility) is dominated by `VS-010`, which is the most consequential bottleneck because it owns first contact with concurrent optimizer mutation (`problem.md` §3) and must demonstrate snapshot-walk-then-merge before four Phase 3 slices can build on it.

4. The roadmap acknowledges the foundational positioning at line 11: *"The roadmap floats cost observability and provider/configuration legibility earlier than their dependency depth might otherwise suggest. P12 says cost is correctness, and D17/D18 are load-bearing market surfaces."* This statement covers `VS-004`, `VS-005`, and `VS-006` but does not extend to `VS-003`, `VS-010`, or `VS-002`.

**Recommendation:** No reordering action required — splitting these slices would violate proposal-level component boundaries and the dependency multiplication is architectural. Optionally, the executive proposer could extend the line-11 acknowledgment to cover `VS-003` (provenance as the universal evidence contract), `VS-002` (summary contracts as the only render-time validity gate), and `VS-010` (the first concurrent-mutation surface) so that downstream readers understand cascading-delay exposure if any of these foundational slices slips.

---

### F-3. No value-dependency inversions

**Severity: NONE**

Pain severity 5 (top third): `VS-001`, `VS-003`, `VS-004`, `VS-008`, `VS-009`, `VS-015`. Pain severity 3 (bottom third — single slice): `VS-012` (Repack hierarchy). Pain severity 4 (middle): all others.

No top-third slice is blocked by a bottom-third slice. `VS-012` is a leaf — no slice declares dependency on it. The chains into top-third slices are all from depth-0 Phase 1 instrumentation (`VS-001`, `VS-003`, `VS-004`, `VS-006`) or from same-tier slices (`VS-009` ← `VS-001`, `VS-003`, `VS-004`, `VS-006`; `VS-015` ← `VS-006`, `VS-004`, `VS-009`, `VS-007`).

The closest pattern that resembles an inversion is `VS-008` (pain 5, Phase 2) blocked by `VS-002` (pain 4, Phase 1), but the dependency is genuine: pack/unpack tools render through summary contracts, so the contract gate must exist before the navigation tools can be trusted. The proposal commitment `Summary Contract` makes `VS-002` an architectural prerequisite of every subsequent pack/unpack render. The 5-vs-4 gap is a single tier and does not constitute an inversion under the top-third / bottom-third test.

**Recommendation:** No action required.

---

### F-4. No undeclared dependencies between value slices

**Severity: LOW**

Schema-object overlap was checked across all 21 slices. The pattern is consistent: shared schema objects either (a) trace through declared dependencies or (b) are forward-references where the slice consumes the *type definition* (Phase 0 substrate) but not yet any *records*.

Spot checks of shared schema objects:

- `WorkingSetSnapshot` — produced first in `VS-001`, consumed by `VS-004`, `VS-005`, `VS-006` (co-Phase-1 — share Phase 0 substrate), `VS-008` (declares `VS-001` ✓), `VS-009` (declares `VS-001` ✓), `VS-015` (transitive via `VS-009` → `VS-001`), `VS-017` (transitive via `VS-015` → `VS-009` → `VS-001`). No undeclared edges.
- `OptimizerEdit` — first producer is `VS-010`. Earlier mentions (`VS-004` records budget, `VS-007` displays in optimizer-log pane) are display/record references against the Phase 0 schema substrate; both Phase 1 slices simply have empty optimizer-log content until `VS-010` ships. Phase 2/3 edit-type extenders (`VS-011`, `VS-012`, `VS-013`, `VS-014`) and Phase 6 consumer (`VS-019`) all declare `VS-010` ✓.
- `BudgetLedger` — first producer is `VS-004`. All other slices that record or read budgets (`VS-008`, `VS-009`, `VS-010`, `VS-015`, `VS-016`, `VS-019`, `VS-021`) trace through declared dependencies on `VS-004` directly or transitively.
- `ConflictRecord` — introduced in `VS-013`, consumed by `VS-018` (declares `VS-013` ✓) and `VS-020` (declares `VS-013` ✓). `VS-019` references it transitively through `VS-018`.
- `IdentityEvent` — appears in `VS-012` and `VS-013`. Both depend transitively on `VS-009` and `VS-010`; the Phase 0 schema substrate provides the type. No producer-before-consumer ordering issue.
- `QuestionArtifact` — first producer is `VS-017` (Phase 5). `VS-007` (Phase 1) lists it among its schema objects but only renders the question queue pane, which stays empty until `VS-017` ships. `VS-020` (Phase 6) declares `VS-017` ✓.
- `RecoveryAction` — first producer is `VS-020` (Phase 6). Earlier mentions in `VS-003`, `VS-006`, `VS-007`, `VS-013`, `VS-017` are display/cross-link references against the Phase 0 substrate; the user surface and provenance slices simply have no recovery records to render until `VS-020` ships.
- `CapabilityFingerprint` — first producer is `VS-006` (Phase 1). `VS-001` lists it for display; both are co-Phase-1 and share Phase 0 substrate. `VS-015`, `VS-016`, `VS-021` declare `VS-006` ✓ directly; `VS-017` declares `VS-006` ✓.
- `PolicySet` — substrate of the `PolicyEngine` component (Phase 0). All slices reference it as configuration, not as records produced by any value slice.

The "schema objects" lists in the executive roadmap are deliberately broad (they include every type the slice's renders, records, or inspections touch), which causes apparent overlap. The actual *production* relationships are clean: each consumer is reachable through declared dependencies from its first producer.

**Recommendation:** No action required. If a future revision wants to make the forward-reference convention explicit, the roadmap could note that schema-object lists include display/cross-link references to types not yet produced, with empty-state behavior until the producing slice ships.

---

### F-5. Phases group correctly — no intra-phase ordering dependencies

**Severity: NONE**

Per-phase dependency check:

- **Phase 1** (`VS-001..VS-007`): all declared depth 0; no slice in Phase 1 declares dependency on another Phase 1 slice. Schema overlap (e.g., `VS-001` and `VS-006` both reference `CapabilityFingerprint`; `VS-005` and `VS-006` both reference `ProviderState`) is forward-reference / co-substrate, not declared ordering.
- **Phase 2** (`VS-008`, `VS-009`, `VS-010`): each declares only Phase 1 upstreams.
- **Phase 3** (`VS-011`, `VS-012`, `VS-013`, `VS-014`): each declares only Phase 1/2 upstreams.
- **Phase 4** (`VS-015`): single slice, declares only Phase 1/2 upstreams.
- **Phase 5** (`VS-016`, `VS-017`, `VS-018`): each declares only Phase 1–4 upstreams.
- **Phase 6** (`VS-019`, `VS-020`): each declares only Phase 1–5 upstreams.
- **Phase 7** (`VS-021`): single slice, declares only Phase 1, 4, 6 upstreams.

Topological sort succeeds. No slice depends on itself, no A-requires-B-requires-A pattern, and no Phase-N slice declares dependency on a Phase-N+K slice.

**Recommendation:** No action required.

---

### F-6. No cycles or physically impossible orderings

**Severity: NONE**

The full declared dependency graph (21 nodes, 47 edges) is acyclic. The largest fanout single-source-of-failure is `VS-003` and `VS-010` at 6 direct downstream each (covered in F-2). No back-edge exists; no slice indirectly depends on itself.

**Recommendation:** No action required.

---

## Summary table

| ID  | Finding                                                              | Severity |
|-----|----------------------------------------------------------------------|----------|
| F-1 | Critical-path length is reasonable (6 nodes / 5 edges of 21 slices)  | NONE     |
| F-2 | Six foundational slices block 3+ downstream — structural, not fragile | LOW      |
| F-3 | No value-dependency inversions                                        | NONE     |
| F-4 | No undeclared dependencies between value slices                       | LOW      |
| F-5 | Phases group correctly — no intra-phase ordering dependencies         | NONE     |
| F-6 | No cycles or physically impossible orderings                          | NONE     |

## What LOW requires

The LOW rating depends on the following conditions remaining true. Any later revision should re-check these.

1. **Critical-path length stays bounded by phase count.** Adding a new slice that creates a 7+ node chain (longer than the 7 phases) would indicate a phase boundary needs splitting.
2. **No new slice introduces a dependency on `VS-012` or other leaves that would convert a leaf into a bottleneck.** `VS-012` (pain 3) is currently the only bottom-third slice, and it must remain a leaf for F-3 to hold.
3. **High-fanout foundational slices (`VS-003`, `VS-004`, `VS-006`, `VS-010`, `VS-015`, `VS-002`) remain narrow.** If any of them is expanded to bundle additional capabilities, the bottleneck becomes operationally fragile rather than structural and the rationale in F-2 stops applying.
4. **Schema-object lists remain forward-reference-style display lists, not declared dependencies.** If a Phase-N slice begins to *require* records produced only by a Phase-N+K slice (rather than displaying them when present), that constitutes a hidden dependency and breaks F-4.
5. **Phase boundaries continue to gate ordering dependencies.** No slice should declare a same-phase dependency. If a future revision moves a Phase-3 slice into Phase 2 with an unresolved Phase-2 dependency, F-5 breaks.
6. **No new edge points backward in phase order.** A Phase-N slice depending on a Phase-N+K slice would violate F-6 (and physical orderability).
7. **`VS-010` keeps its current scope.** The summary-refresh/stale-marker slice is the most-consumed Phase 2 capability and the first slice to touch concurrent-optimizer mutation; expanding it to include cross-references, splits, or merges would push fragility past the threshold where "structural" stops covering it.
