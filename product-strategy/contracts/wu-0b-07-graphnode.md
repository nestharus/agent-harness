# WU-0B-07 GraphNode Contract

`GraphNode` is the stable logical object visible to agents and UI. This contract defines only the GraphStore schema and repository boundary.

## Row

```text
GraphNode {
  node_id: OpaqueId<GraphNode>,
  workspace_id: OpaqueId<GraphWorkspace>,
  kind: GraphNodeKind,
  title: String,
  lifecycle_state: LifecycleState,
  current_revision_id: Option<OpaqueId<NodeRevisionRef>>,
  canonical_parent_edge_id: Option<OpaqueId<GraphEdgeRef>>,
  privilege_origin: PrivilegeOrigin,
  trust_state: TrustState,
  created_from_ref: Option<JsonField<serde_json::Value>>,
  deleted_at: Option<Timestamp>,
  meta: RecordMeta
}
```

`current_revision_id` and `canonical_parent_edge_id` are nullable soft references because their target tables are owned by later WUs. WU-0B-07 rejects non-null values as `UnknownRef` until those tables exist.

## Enums

- `GraphNodeKind`: `initiative`, `work_unit`, `summary`, `evidence`, `question`, `decision`, `blocker`, `worker_output`, `recovery`, `policy_note`, `archive`
- `LifecycleState`: `active`, `packed`, `unpacked`, `blocked`, `stale`, `recovering`, `archived`, `quarantined`, `deleted`
- `PrivilegeOrigin`: `system`, `user`, `tool`, `model`, `worker`, `optimizer`, `reviewer`
- `TrustState`: `trusted`, `derived`, `unverified`, `suspect`, `poison_quarantined`

Unknown enum values are rejected by serde and by SQLx decode.

## Repository

- `insert_graph_node(GraphNode) -> Result<GraphNode, GraphStoreError>`
- `get_graph_node(&OpaqueId<GraphNode>) -> Result<Option<GraphNode>, GraphStoreError>`
- `list_graph_nodes_by_workspace(&OpaqueId<GraphWorkspace>) -> Result<Vec<GraphNode>, GraphStoreError>`
- `transition_lifecycle(&OpaqueId<GraphNode>, LifecycleState) -> Result<GraphNode, GraphStoreError>`

Repository writes run in explicit transactions and roll back on FK, enum, soft-ref, routing, or state-transition failure.

## Lifecycle Matrix

Valid:

- `active -> packed`
- `packed -> unpacked`
- `active -> blocked`
- `blocked -> active`
- `active -> stale`
- `stale -> active`
- any non-deleted state -> `archived`
- any non-deleted state -> `quarantined`
- `quarantined -> recovering`

Invalid examples include `packed -> active`, `recovering -> active`, and any transition out of `deleted`.
