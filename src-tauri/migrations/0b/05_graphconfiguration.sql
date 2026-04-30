CREATE TABLE graph_configurations (
  row_id INTEGER PRIMARY KEY,
  configuration_id_value TEXT NOT NULL,
  configuration_id_namespace TEXT NOT NULL,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  configuration_version INTEGER NOT NULL,
  schema_profile TEXT NOT NULL,
  summary_contract_template_ids TEXT NOT NULL CHECK (json_valid(summary_contract_template_ids)),
  optimizer_policy_ref_value TEXT NOT NULL,
  optimizer_policy_ref_namespace TEXT NOT NULL,
  render_policy_ref_value TEXT NOT NULL,
  render_policy_ref_namespace TEXT NOT NULL,
  memory_policy_ref_value TEXT NOT NULL,
  memory_policy_ref_namespace TEXT NOT NULL,
  provider_routing_policy_ref_value TEXT NOT NULL,
  provider_routing_policy_ref_namespace TEXT NOT NULL,
  capability_fingerprint_policy_ref_value TEXT NOT NULL,
  capability_fingerprint_policy_ref_namespace TEXT NOT NULL,
  effective_value_sources TEXT NOT NULL CHECK (json_valid(effective_value_sources)),
  created_from_configuration_id_value TEXT,
  created_from_configuration_id_namespace TEXT,
  validation_state TEXT NOT NULL CHECK (
    validation_state IN (
      'valid',
      'valid_with_warnings',
      'invalid_schema',
      'invalid_provider_route',
      'invalid_budget',
      'invalid_index',
      'needs_user_attention'
    )
  ),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 5,
  UNIQUE(configuration_id_value, configuration_id_namespace),
  FOREIGN KEY(optimizer_policy_ref_value, optimizer_policy_ref_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(render_policy_ref_value, render_policy_ref_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(memory_policy_ref_value, memory_policy_ref_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(provider_routing_policy_ref_value, provider_routing_policy_ref_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(capability_fingerprint_policy_ref_value, capability_fingerprint_policy_ref_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_graph_configurations_configuration_id
  ON graph_configurations(configuration_id_value, configuration_id_namespace);

CREATE INDEX idx_graph_configurations_workspace_id
  ON graph_configurations(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_graph_configurations_created_from
  ON graph_configurations(created_from_configuration_id_value, created_from_configuration_id_namespace);
