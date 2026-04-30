CREATE TABLE graph_workspaces (
  row_id INTEGER PRIMARY KEY,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  schema_version INTEGER NOT NULL,
  active_orchestrator_id_value TEXT NOT NULL,
  active_orchestrator_id_namespace TEXT NOT NULL,
  current_graph_version INTEGER NOT NULL,
  storage_root TEXT NOT NULL,
  policy_set_id_value TEXT NOT NULL,
  policy_set_id_namespace TEXT NOT NULL,
  active_configuration_id_value TEXT NOT NULL,
  active_configuration_id_namespace TEXT NOT NULL,
  is_active_default INTEGER NOT NULL CHECK (is_active_default IN (0, 1)),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  UNIQUE(workspace_id_value, workspace_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version),
  FOREIGN KEY(policy_set_id_value, policy_set_id_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(active_configuration_id_value, active_configuration_id_namespace)
    REFERENCES graph_configurations(configuration_id_value, configuration_id_namespace)
);

CREATE INDEX idx_graph_workspaces_workspace_id
  ON graph_workspaces(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_graph_workspaces_policy_set_id
  ON graph_workspaces(policy_set_id_value, policy_set_id_namespace);

CREATE INDEX idx_graph_workspaces_active_configuration_id
  ON graph_workspaces(active_configuration_id_value, active_configuration_id_namespace);

CREATE UNIQUE INDEX idx_graph_workspaces_one_active_default
  ON graph_workspaces(is_active_default)
  WHERE is_active_default = 1;
