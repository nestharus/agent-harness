CREATE TABLE evidence_artifacts (
  row_id INTEGER PRIMARY KEY,
  evidence_id_value TEXT NOT NULL,
  evidence_id_namespace TEXT NOT NULL,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  source_type TEXT NOT NULL CHECK (
    source_type IN (
      'cli_transcript',
      'tool_call',
      'tool_result',
      'file_read',
      'file_patch',
      'command_output',
      'user_message',
      'worker_output',
      'optimizer_prompt',
      'reviewer_output',
      'external_document'
    )
  ),
  source_uri TEXT NOT NULL CHECK (trim(source_uri) <> ''),
  source_session_id TEXT CHECK (source_session_id IS NULL OR trim(source_session_id) <> ''),
  tool_protocol TEXT NOT NULL CHECK (
    tool_protocol IN (
      'claude',
      'codex',
      'opencode',
      'mcp',
      'shell',
      'none'
    )
  ),
  correlation_key TEXT CHECK (correlation_key IS NULL OR trim(correlation_key) <> ''),
  content_hash TEXT CHECK (content_hash IS NULL OR trim(content_hash) <> ''),
  blob_ref TEXT NOT NULL CHECK (trim(blob_ref) <> ''),
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
  capture_state TEXT NOT NULL CHECK (
    capture_state IN (
      'captured',
      'partial',
      'failed',
      'redacted',
      'quarantined'
    )
  ),
  captured_at TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 9,
  UNIQUE(evidence_id_value, evidence_id_namespace),
  CHECK (
    capture_state = 'failed'
    OR content_hash IS NOT NULL
  ),
  FOREIGN KEY(workspace_id_value, workspace_id_namespace)
    REFERENCES graph_workspaces(workspace_id_value, workspace_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_evidence_artifacts_workspace_id
  ON evidence_artifacts(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_evidence_artifacts_content_hash
  ON evidence_artifacts(content_hash);

CREATE INDEX idx_evidence_artifacts_capture_state
  ON evidence_artifacts(capture_state);
