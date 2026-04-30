# WU-0A-03 Contract: LocalStorageLayout DTO

## Ownership

This WU owns the Phase 0A local storage layout DTO, storage-layout derivation helper, and storage-layout error taxonomy. It does not own settings loading, directory creation, SQLite opening, app-state construction, GraphStore migrations, provider configuration, subprocess supervision, or IPC command registration.

## DTO

`LocalStorageLayout` serializes as a JSON object with exactly these fields:

- `storage_root`: string
- `database_path`: string
- `evidence_root`: string
- `fixture_root`: string
- `log_root`: string
- `temp_root`: string

Unknown DTO fields are rejected by the Rust contract type and the TypeScript parser.

## Derivation

```text
derive_local_storage_layout(settings: HarnessSettings) -> Result<LocalStorageLayout, StorageLayoutError>
```

Rules:

1. Validate `settings.storage_root` lexically against the workspace fixture boundary. Relative storage roots must not use parent traversal to escape that boundary.
2. Validate `settings.database_path` is inside `settings.storage_root`.
3. Return a `LocalStorageLayout` with `storage_root` equal to `settings.storage_root` and `database_path` equal to `settings.database_path`.
4. Derive child roots under `storage_root` using these stable names:
   - `evidence_root`: `evidence`
   - `fixture_root`: `fixtures`
   - `log_root`: `logs`
   - `temp_root`: `tmp`
5. Validate every layout path is inside `storage_root`.

The containment check is lexical and does not create directories, canonicalize symlinks, open the database, or inspect the host filesystem.

## Errors

`StorageLayoutError` serializes as one of these exact strings:

- `StorageRootEscapesWorkspace`
- `DatabasePathOutsideStorageRoot`
- `EvidenceRootOutsideStorageRoot`
- `FixtureRootOutsideStorageRoot`
- `LogRootOutsideStorageRoot`
- `TempRootOutsideStorageRoot`

Unknown error strings are rejected by Rust serde and by the TypeScript parser.

## Canonical Fixtures

Fixtures live under `product-strategy/contracts/fixtures/wu-0a-03/`:

- `canonical-layout.json`: canonical DTO shape.
- `derive-success.json`: settings input and expected derived layout.
- `derive-errors.json`: settings inputs that must reach storage-root and database containment errors.
- `invalid-layouts.json`: configured layout inputs that must reach database and child-root containment errors.
- `invalid-layout-shapes.json`: TypeScript parser rejection cases.
- `storage-layout-errors.json`: all documented error variants.
- `invalid-storage-layout-error.json`: unknown error string rejection fixture.

## Test Handoff

- Rust contract test: `src-tauri/tests/local_storage_layout_contract.rs`.
- TypeScript contract test: `src/test/local-storage-layout.test.ts`.

Every test group carries a risk annotation mapped to the proposal test-intent track.
