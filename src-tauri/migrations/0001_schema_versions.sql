CREATE TABLE IF NOT EXISTS schema_versions (
  version INTEGER PRIMARY KEY,
  migration_name TEXT NOT NULL,
  applied_at TEXT NOT NULL,
  checksum TEXT NOT NULL,
  execution_ms INTEGER NOT NULL,
  applied_by TEXT NOT NULL,
  rollback_state TEXT NOT NULL CHECK (
    rollback_state IN (
      'applied',
      'rollback_unsupported',
      'rollback_planned',
      'rollback_failed'
    )
  )
);
