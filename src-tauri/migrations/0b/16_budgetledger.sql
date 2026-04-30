CREATE TABLE budget_ledgers (
  row_id INTEGER PRIMARY KEY,
  budget_ledger_id_value TEXT NOT NULL,
  budget_ledger_id_namespace TEXT NOT NULL,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  scope_type TEXT NOT NULL CHECK (
    scope_type IN (
      'workspace',
      'initiative',
      'orchestrator_turn',
      'worker_run',
      'optimizer_pass',
      'reviewer_pass',
      'render'
    )
  ),
  scope_id_value TEXT NOT NULL,
  scope_id_namespace TEXT NOT NULL,
  input_tokens INTEGER NOT NULL CHECK (input_tokens >= 0),
  output_tokens INTEGER NOT NULL CHECK (output_tokens >= 0),
  cache_read_tokens INTEGER NOT NULL CHECK (cache_read_tokens >= 0),
  cache_write_tokens INTEGER NOT NULL CHECK (cache_write_tokens >= 0),
  latency_ms INTEGER NOT NULL CHECK (latency_ms >= 0),
  provider_cost_estimate INTEGER NOT NULL CHECK (provider_cost_estimate >= 0),
  provider_state_id_value TEXT,
  provider_state_id_namespace TEXT,
  cache_prefix_hash TEXT,
  budget_state TEXT NOT NULL CHECK (
    budget_state IN (
      'within',
      'near_limit',
      'exceeded',
      'blocked'
    )
  ),
  policy_action TEXT NOT NULL CHECK (
    policy_action IN (
      'none',
      'warn',
      'narrow_scope',
      'require_user_approval',
      'block'
    )
  ),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 16,
  UNIQUE(budget_ledger_id_value, budget_ledger_id_namespace),
  CHECK (
    (provider_state_id_value IS NULL AND provider_state_id_namespace IS NULL)
    OR (provider_state_id_value IS NOT NULL AND provider_state_id_namespace IS NOT NULL)
  ),
  CHECK (
    (budget_state = 'within' AND policy_action IN ('none', 'warn'))
    OR (budget_state = 'near_limit' AND policy_action IN ('warn', 'narrow_scope'))
    OR (budget_state = 'exceeded' AND policy_action IN ('require_user_approval', 'block', 'narrow_scope'))
    OR (budget_state = 'blocked' AND policy_action = 'block')
  ),
  FOREIGN KEY(workspace_id_value, workspace_id_namespace)
    REFERENCES graph_workspaces(workspace_id_value, workspace_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_budget_ledgers_workspace_id
  ON budget_ledgers(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_budget_ledgers_scope_type
  ON budget_ledgers(scope_type);

CREATE INDEX idx_budget_ledgers_budget_state
  ON budget_ledgers(budget_state);
