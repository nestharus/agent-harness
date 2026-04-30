# WU-0B-05 Shortcut Gate Review

**Severity:** LOW

## Result

The implementation does not stub or shortcut the contract surface. Both enums
have real serde and SQLx `Type`/`Encode`/`Decode` impls, FK enforcement is
delegated to SQLite with `PRAGMA foreign_keys = ON` enabled per insert,
transaction rollback is exercised on every documented failure path, the
revision-parent check runs a real `SELECT` against `graph_configurations`, and
JSON columns flow through `JsonField<T>`.

## Verification

- `src-tauri/src/graphstore/graphconfiguration.rs:579-611` defines an
  `impl_sqlite_text_enum!` macro and applies it to both `EffectiveValueSource`
  and `ValidationState`. Encode/Decode go through real `String` codecs and
  unknown values produce `GraphStoreError::InvalidEnum`.
- The variant tests (`every_effective_value_source_variant_*`,
  `every_validation_state_variant_*`) write each variant through `INSERT INTO
  source_codec` / `state_codec` and read it back through `try_get`, exercising
  the real SQLx codec round-trip.
- FK enforcement is real:
  `sqlx::query("PRAGMA foreign_keys = ON").execute(&self.pool)` runs against the
  `max_connections(1)` pool (see `app_state.rs:248`), then the
  `transaction-rollback-fk.json` fixture proves that a missing policy_set
  produces `UnknownRef` and rolls back to the original row count.
- `validate_revision_parent`
  (`graphstore/graphconfiguration.rs:298-326`) issues
  `SELECT workspace_id_value, workspace_id_namespace, created_at FROM
  graph_configurations WHERE configuration_id_value = ? AND
  configuration_id_namespace = ?` on the open transaction. It returns
  `UnknownRef` when the parent is absent, and `InvalidTransition` when
  workspace mismatches OR `parent.created_at >= record.meta.created_at`.
  All three branches are hit by
  `created_from_configuration_id_is_null_or_older_same_workspace_revision`.
- `effective_value_sources` is typed as
  `JsonField<Vec<EffectiveValueSourceEntry>>` on both the struct and the
  `FromRow` row, so it flows through the WU-0B-02 JSON serialization prelude.
- No `unwrap()`, `todo!`, `unimplemented!`, or `TODO` markers in production
  paths (graphstore/graphconfiguration.rs, contracts/graphconfiguration.rs).

## Findings

### WU-0B-05-SHORTCUT-F01 (NIT)

`PRAGMA foreign_keys = ON` is executed at the top of every
`insert_graph_configuration` call rather than being attached to the pool's
`SqliteConnectOptions`. Because the pool is `max_connections(1)`, the PRAGMA
sticks for the rest of the session, and the rollback-FK test passes; however, a
single-connection assumption is implicit. If pool sizing changes elsewhere, the
PRAGMA would no longer reach every connection. Not a violation of any 4-gate
criterion — flagged for future consideration only.
