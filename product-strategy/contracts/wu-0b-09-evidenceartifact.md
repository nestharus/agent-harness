# WU-0B-09 EvidenceArtifact Contract

`EvidenceArtifact` is the inert raw or normalized source content record. This WU owns only the GraphStore schema, typed Rust contract, repository insert/get/list boundary, and fixtures.

## Row

```text
EvidenceArtifact {
  evidence_id: OpaqueId<EvidenceArtifact>,
  workspace_id: OpaqueId<GraphWorkspace>,
  source_type: SourceType,
  source_uri: String,
  source_session_id: Option<String>,
  tool_protocol: ToolProtocol,
  correlation_key: Option<String>,
  content_hash: Option<String>,
  blob_ref: String,
  privilege_origin: PrivilegeOrigin,
  capture_state: CaptureState,
  captured_at: Timestamp,
  meta: RecordMeta
}
```

`blob_ref` is the storage payload reference for this WU. When `content_hash` is present, `blob_ref` points to the content-bearing payload. When `capture_state = failed` and `content_hash` is absent, `blob_ref` is the required failure payload reference. No separate `failure_payload_ref` column is created.

## Enums

- `SourceType`: `cli_transcript`, `tool_call`, `tool_result`, `file_read`, `file_patch`, `command_output`, `user_message`, `worker_output`, `optimizer_prompt`, `reviewer_output`, `external_document`
- `ToolProtocol`: `claude`, `codex`, `opencode`, `mcp`, `shell`, `none`
- `CaptureState`: `captured`, `partial`, `failed`, `redacted`, `quarantined`
- `PrivilegeOrigin`: reused from WU-0B-07 GraphNode because the seven variants are identical: `system`, `user`, `tool`, `model`, `worker`, `optimizer`, `reviewer`

Unknown enum values are rejected by serde and SQLx decode.

## Repository

- `insert_evidence_artifact(EvidenceArtifact) -> Result<EvidenceArtifact, GraphStoreError>`
- `get_evidence_artifact(&OpaqueId<EvidenceArtifact>) -> Result<Option<EvidenceArtifact>, GraphStoreError>`
- `list_evidence_artifacts_by_workspace(&OpaqueId<GraphWorkspace>) -> Result<Vec<EvidenceArtifact>, GraphStoreError>`

Writes run inside explicit transactions and roll back on FK, enum, routing, content-state invariant, or blob path validation failure.

## Validation

Before insert, the repository validates every opaque ID, `RecordMeta`, non-empty `source_uri`, non-empty optional strings when present, `captured_at`, and `blob_ref`.

`content_hash` must be non-empty for `captured`, `partial`, `redacted`, and `quarantined`. `failed` may omit `content_hash` only when `blob_ref` is non-empty and resolves to a payload under the workspace storage root.

`blob_ref` is always interpreted relative to `graph_workspaces.storage_root`. Absolute paths, `..` components, missing payloads, and symlinks that canonicalize outside the workspace storage root are rejected as `GraphStoreError::InvariantViolation`.

## No Side Effects

This WU must not add Tauri commands, UI panes, operator-visible behavior, agent invocation, provider probes, optimizer execution, recovery execution, or any durable table other than `evidence_artifacts`.
