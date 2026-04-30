export interface LocalStorageLayout {
  storage_root: string;
  database_path: string;
  evidence_root: string;
  fixture_root: string;
  log_root: string;
  temp_root: string;
}

export const STORAGE_LAYOUT_ERRORS = [
  "StorageRootEscapesWorkspace",
  "DatabasePathOutsideStorageRoot",
  "EvidenceRootOutsideStorageRoot",
  "FixtureRootOutsideStorageRoot",
  "LogRootOutsideStorageRoot",
  "TempRootOutsideStorageRoot",
] as const;

export type StorageLayoutError = (typeof STORAGE_LAYOUT_ERRORS)[number];

export function parseLocalStorageLayout(value: unknown): LocalStorageLayout {
  if (!isLocalStorageLayout(value)) {
    throw new Error("Invalid LocalStorageLayout");
  }

  return value;
}

export function parseStorageLayoutError(value: unknown): StorageLayoutError {
  if (typeof value !== "string" || !isStorageLayoutError(value)) {
    throw new Error("Invalid StorageLayoutError");
  }

  return value;
}

function isLocalStorageLayout(value: unknown): value is LocalStorageLayout {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  const allowedKeys = [
    "database_path",
    "evidence_root",
    "fixture_root",
    "log_root",
    "storage_root",
    "temp_root",
  ];

  if (keys.length !== allowedKeys.length || keys.some((key, index) => key !== allowedKeys[index])) {
    return false;
  }

  return (
    typeof record.storage_root === "string" &&
    typeof record.database_path === "string" &&
    typeof record.evidence_root === "string" &&
    typeof record.fixture_root === "string" &&
    typeof record.log_root === "string" &&
    typeof record.temp_root === "string"
  );
}

function isStorageLayoutError(value: string): value is StorageLayoutError {
  return (STORAGE_LAYOUT_ERRORS as readonly string[]).includes(value);
}
