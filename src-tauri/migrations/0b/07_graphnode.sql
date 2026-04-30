CREATE TABLE graph_nodes (
  row_id INTEGER PRIMARY KEY,
  node_id_value TEXT NOT NULL,
  node_id_namespace TEXT NOT NULL,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  kind TEXT NOT NULL CHECK (
    kind IN (
      'initiative',
      'work_unit',
      'summary',
      'evidence',
      'question',
      'decision',
      'blocker',
      'worker_output',
      'recovery',
      'policy_note',
      'archive'
    )
  ),
  title TEXT NOT NULL,
  lifecycle_state TEXT NOT NULL CHECK (
    lifecycle_state IN (
      'active',
      'packed',
      'unpacked',
      'blocked',
      'stale',
      'recovering',
      'archived',
      'quarantined',
      'deleted'
    )
  ),
  current_revision_id_value TEXT,
  current_revision_id_namespace TEXT,
  canonical_parent_edge_id_value TEXT,
  canonical_parent_edge_id_namespace TEXT,
  privilege_origin TEXT NOT NULL CHECK (
    privilege_origin IN (
      'system',
      'user',
      'tool',
      'model',
      'worker',
      'optimizer',
      'reviewer'
    )
  ),
  trust_state TEXT NOT NULL CHECK (
    trust_state IN (
      'trusted',
      'derived',
      'unverified',
      'suspect',
      'poison_quarantined'
    )
  ),
  created_from_ref TEXT CHECK (created_from_ref IS NULL OR json_valid(created_from_ref)),
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 7,
  UNIQUE(node_id_value, node_id_namespace),
  CHECK (
    (current_revision_id_value IS NULL AND current_revision_id_namespace IS NULL)
    OR (current_revision_id_value IS NOT NULL AND current_revision_id_namespace IS NOT NULL)
  ),
  CHECK (
    (canonical_parent_edge_id_value IS NULL AND canonical_parent_edge_id_namespace IS NULL)
    OR (
      canonical_parent_edge_id_value IS NOT NULL
      AND canonical_parent_edge_id_namespace IS NOT NULL
    )
  ),
  CHECK (
    (lifecycle_state = 'deleted' AND deleted_at IS NOT NULL)
    OR (lifecycle_state <> 'deleted' AND deleted_at IS NULL)
  ),
  FOREIGN KEY(workspace_id_value, workspace_id_namespace)
    REFERENCES graph_workspaces(workspace_id_value, workspace_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_graph_nodes_node_id
  ON graph_nodes(node_id_value, node_id_namespace);

CREATE INDEX idx_graph_nodes_workspace_id
  ON graph_nodes(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_graph_nodes_lifecycle_state
  ON graph_nodes(lifecycle_state);

CREATE INDEX idx_graph_nodes_kind
  ON graph_nodes(kind);
