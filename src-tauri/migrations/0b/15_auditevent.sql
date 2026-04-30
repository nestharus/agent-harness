CREATE TABLE audit_events (
  row_id INTEGER PRIMARY KEY,
  audit_event_id_value TEXT NOT NULL,
  audit_event_id_namespace TEXT NOT NULL,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  event_type TEXT NOT NULL CHECK (trim(event_type) <> ''),
  actor TEXT NOT NULL CHECK (trim(actor) <> ''),
  policy_set_id_value TEXT NOT NULL,
  policy_set_id_namespace TEXT NOT NULL,
  configuration_id_value TEXT,
  configuration_id_namespace TEXT,
  provider_state_id_value TEXT,
  provider_state_id_namespace TEXT,
  input_refs TEXT NOT NULL CHECK (json_valid(input_refs)),
  output_refs TEXT NOT NULL CHECK (json_valid(output_refs)),
  decision TEXT NOT NULL CHECK (
    decision IN (
      'accepted',
      'rejected',
      'deferred',
      'quarantined',
      'user_required'
    )
  ),
  reason_code TEXT NOT NULL CHECK (trim(reason_code) <> ''),
  created_at TEXT NOT NULL,
  record_created_at TEXT NOT NULL,
  record_updated_at TEXT NOT NULL,
  record_actor TEXT NOT NULL CHECK (json_valid(record_actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 15,
  UNIQUE(audit_event_id_value, audit_event_id_namespace),
  CHECK (
    (configuration_id_value IS NULL AND configuration_id_namespace IS NULL)
    OR (configuration_id_value IS NOT NULL AND configuration_id_namespace IS NOT NULL)
  ),
  CHECK (
    (provider_state_id_value IS NULL AND provider_state_id_namespace IS NULL)
    OR (provider_state_id_value IS NOT NULL AND provider_state_id_namespace IS NOT NULL)
  ),
  CHECK (
    json_type(input_refs) = 'array'
    AND json_type(output_refs) = 'array'
  ),
  CHECK (
    json_array_length(output_refs) = 0
    OR json_array_length(input_refs) > 0
  ),
  FOREIGN KEY(workspace_id_value, workspace_id_namespace)
    REFERENCES graph_workspaces(workspace_id_value, workspace_id_namespace),
  FOREIGN KEY(policy_set_id_value, policy_set_id_namespace)
    REFERENCES policy_sets(policy_set_id_value, policy_set_id_namespace),
  FOREIGN KEY(configuration_id_value, configuration_id_namespace)
    REFERENCES graph_configurations(configuration_id_value, configuration_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_audit_events_workspace_id
  ON audit_events(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_audit_events_policy_set_id
  ON audit_events(policy_set_id_value, policy_set_id_namespace);

CREATE INDEX idx_audit_events_decision
  ON audit_events(decision);

CREATE INDEX idx_audit_events_created_at
  ON audit_events(created_at);
