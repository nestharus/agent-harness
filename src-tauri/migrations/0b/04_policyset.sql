CREATE TABLE policy_sets (
  row_id INTEGER PRIMARY KEY,
  policy_set_id_value TEXT NOT NULL,
  policy_set_id_namespace TEXT NOT NULL,
  summary_contract_version TEXT NOT NULL CHECK (json_valid(summary_contract_version)),
  render_policy_version TEXT NOT NULL CHECK (json_valid(render_policy_version)),
  identity_policy_version TEXT NOT NULL CHECK (json_valid(identity_policy_version)),
  privilege_policy_version TEXT NOT NULL CHECK (json_valid(privilege_policy_version)),
  tool_protocol_policy_version TEXT NOT NULL CHECK (json_valid(tool_protocol_policy_version)),
  budget_policy_version TEXT NOT NULL CHECK (json_valid(budget_policy_version)),
  review_sampling_policy_version TEXT NOT NULL CHECK (json_valid(review_sampling_policy_version)),
  recovery_policy_version TEXT NOT NULL CHECK (json_valid(recovery_policy_version)),
  configuration_policy_version TEXT NOT NULL CHECK (json_valid(configuration_policy_version)),
  provider_policy_version TEXT NOT NULL CHECK (json_valid(provider_policy_version)),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 4,
  UNIQUE(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_policy_sets_policy_set_id
  ON policy_sets(policy_set_id_value, policy_set_id_namespace);

CREATE INDEX idx_policy_sets_namespace
  ON policy_sets(policy_set_id_namespace);
