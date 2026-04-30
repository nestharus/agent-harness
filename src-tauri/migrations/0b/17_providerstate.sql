CREATE TABLE provider_states (
  row_id INTEGER PRIMARY KEY,
  provider_state_id_value TEXT NOT NULL,
  provider_state_id_namespace TEXT NOT NULL,
  workspace_id_value TEXT NOT NULL,
  workspace_id_namespace TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (
    provider IN (
      'anthropic',
      'openai',
      'google',
      'local_runtime',
      'openai_compatible',
      'other'
    )
  ),
  cli TEXT NOT NULL CHECK (
    cli IN (
      'claude',
      'codex',
      'opencode',
      'agent_runner'
    )
  ),
  account_ref TEXT NOT NULL CHECK (trim(account_ref) <> ''),
  auth_state TEXT NOT NULL CHECK (
    auth_state IN (
      'present',
      'missing',
      'expired',
      'invalid',
      'unknown'
    )
  ),
  billing_state TEXT NOT NULL CHECK (
    billing_state IN (
      'healthy',
      'near_limit',
      'over_limit',
      'payment_required',
      'unknown'
    )
  ),
  quota_state TEXT NOT NULL CHECK (
    quota_state IN (
      'available',
      'rate_limited',
      'exhausted',
      'unknown'
    )
  ),
  network_state TEXT NOT NULL CHECK (
    network_state IN (
      'available',
      'blocked_by_sandbox',
      'blocked_by_host',
      'degraded',
      'unknown'
    )
  ),
  runtime_state TEXT NOT NULL CHECK (
    runtime_state IN (
      'installed',
      'missing',
      'wrong_version',
      'unreachable',
      'not_applicable',
      'unknown'
    )
  ),
  sandbox_constraints TEXT NOT NULL CHECK (json_valid(sandbox_constraints)),
  store_locations_checked TEXT NOT NULL CHECK (json_valid(store_locations_checked)),
  secret_material_stored INTEGER NOT NULL CHECK (secret_material_stored = 0),
  freshness TEXT NOT NULL CHECK (
    freshness IN (
      'fresh',
      'stale',
      'probe_failed',
      'manual'
    )
  ),
  confidence TEXT NOT NULL CHECK (
    confidence IN (
      'high',
      'medium',
      'low'
    )
  ),
  last_probe_at TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  actor TEXT NOT NULL CHECK (json_valid(actor)),
  record_policy_version TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 17,
  UNIQUE(provider_state_id_value, provider_state_id_namespace),
  CHECK (json_type(store_locations_checked) = 'array'),
  FOREIGN KEY(workspace_id_value, workspace_id_namespace)
    REFERENCES graph_workspaces(workspace_id_value, workspace_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

CREATE INDEX idx_provider_states_workspace_id
  ON provider_states(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_provider_states_provider
  ON provider_states(provider);

CREATE INDEX idx_provider_states_cli
  ON provider_states(cli);

CREATE INDEX idx_provider_states_freshness
  ON provider_states(freshness);

CREATE INDEX idx_provider_states_last_probe_at
  ON provider_states(last_probe_at);

ALTER TABLE audit_events RENAME TO audit_events_wu_0b_15;

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
  FOREIGN KEY(provider_state_id_value, provider_state_id_namespace)
    REFERENCES provider_states(provider_state_id_value, provider_state_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

INSERT INTO audit_events
  (row_id, audit_event_id_value, audit_event_id_namespace,
   workspace_id_value, workspace_id_namespace, event_type, actor,
   policy_set_id_value, policy_set_id_namespace,
   configuration_id_value, configuration_id_namespace,
   provider_state_id_value, provider_state_id_namespace,
   input_refs, output_refs, decision, reason_code, created_at,
   record_created_at, record_updated_at, record_actor,
   record_policy_version, schema_version)
SELECT row_id, audit_event_id_value, audit_event_id_namespace,
       workspace_id_value, workspace_id_namespace, event_type, actor,
       policy_set_id_value, policy_set_id_namespace,
       configuration_id_value, configuration_id_namespace,
       CASE
         WHEN provider_state_id_value IS NOT NULL
          AND EXISTS (
            SELECT 1 FROM provider_states
            WHERE provider_state_id_value = audit_events_wu_0b_15.provider_state_id_value
              AND provider_state_id_namespace = audit_events_wu_0b_15.provider_state_id_namespace
          )
         THEN provider_state_id_value
         ELSE NULL
       END,
       CASE
         WHEN provider_state_id_value IS NOT NULL
          AND EXISTS (
            SELECT 1 FROM provider_states
            WHERE provider_state_id_value = audit_events_wu_0b_15.provider_state_id_value
              AND provider_state_id_namespace = audit_events_wu_0b_15.provider_state_id_namespace
          )
         THEN provider_state_id_namespace
         ELSE NULL
       END,
       input_refs, output_refs, decision, reason_code, created_at,
       record_created_at, record_updated_at, record_actor,
       record_policy_version, schema_version
FROM audit_events_wu_0b_15;

DROP TABLE audit_events_wu_0b_15;

CREATE INDEX idx_audit_events_workspace_id
  ON audit_events(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_audit_events_policy_set_id
  ON audit_events(policy_set_id_value, policy_set_id_namespace);

CREATE INDEX idx_audit_events_decision
  ON audit_events(decision);

CREATE INDEX idx_audit_events_created_at
  ON audit_events(created_at);

ALTER TABLE budget_ledgers RENAME TO budget_ledgers_wu_0b_16;

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
  FOREIGN KEY(provider_state_id_value, provider_state_id_namespace)
    REFERENCES provider_states(provider_state_id_value, provider_state_id_namespace),
  FOREIGN KEY(schema_version) REFERENCES schema_versions(version)
);

INSERT INTO budget_ledgers
  (row_id, budget_ledger_id_value, budget_ledger_id_namespace,
   workspace_id_value, workspace_id_namespace,
   scope_type, scope_id_value, scope_id_namespace,
   input_tokens, output_tokens, cache_read_tokens,
   cache_write_tokens, latency_ms, provider_cost_estimate,
   provider_state_id_value, provider_state_id_namespace,
   cache_prefix_hash, budget_state, policy_action,
   created_at, updated_at, actor, record_policy_version,
   schema_version)
SELECT row_id, budget_ledger_id_value, budget_ledger_id_namespace,
       workspace_id_value, workspace_id_namespace,
       scope_type, scope_id_value, scope_id_namespace,
       input_tokens, output_tokens, cache_read_tokens,
       cache_write_tokens, latency_ms, provider_cost_estimate,
       CASE
         WHEN provider_state_id_value IS NOT NULL
          AND EXISTS (
            SELECT 1 FROM provider_states
            WHERE provider_state_id_value = budget_ledgers_wu_0b_16.provider_state_id_value
              AND provider_state_id_namespace = budget_ledgers_wu_0b_16.provider_state_id_namespace
          )
         THEN provider_state_id_value
         ELSE NULL
       END,
       CASE
         WHEN provider_state_id_value IS NOT NULL
          AND EXISTS (
            SELECT 1 FROM provider_states
            WHERE provider_state_id_value = budget_ledgers_wu_0b_16.provider_state_id_value
              AND provider_state_id_namespace = budget_ledgers_wu_0b_16.provider_state_id_namespace
          )
         THEN provider_state_id_namespace
         ELSE NULL
       END,
       cache_prefix_hash, budget_state, policy_action,
       created_at, updated_at, actor, record_policy_version,
       schema_version
FROM budget_ledgers_wu_0b_16;

DROP TABLE budget_ledgers_wu_0b_16;

CREATE INDEX idx_budget_ledgers_workspace_id
  ON budget_ledgers(workspace_id_value, workspace_id_namespace);

CREATE INDEX idx_budget_ledgers_scope_type
  ON budget_ledgers(scope_type);

CREATE INDEX idx_budget_ledgers_budget_state
  ON budget_ledgers(budget_state);
