export interface TempHarnessHandle {
  workspace_id: string;
  database_path: string;
  fixture_manifest_id: string;
  app_state_ready: boolean;
}

export const TEMP_HARNESS_ERRORS = [
  "UnknownSeed",
  "DatabaseCreateFailed",
  "AppStateInitFailed",
  "RealAgentsInvocationAttempted",
  "FixtureManifestMissing",
] as const;

export type TempHarnessError = (typeof TEMP_HARNESS_ERRORS)[number];

export class TempHarnessErrorFailure extends Error {
  readonly kind: TempHarnessError;
  readonly seedName?: string;

  constructor(kind: TempHarnessError, options: { seedName?: string; cause?: unknown } = {}) {
    super(kind, { cause: options.cause });
    this.name = "TempHarnessErrorFailure";
    this.kind = kind;
    this.seedName = options.seedName;
  }
}

export function parseTempHarnessHandle(value: unknown): TempHarnessHandle {
  if (!isTempHarnessHandle(value)) {
    throw new Error("Invalid TempHarnessHandle");
  }

  return value;
}

export function parseTempHarnessError(value: unknown): TempHarnessError {
  if (typeof value !== "string" || !isTempHarnessError(value)) {
    throw new Error("Invalid TempHarnessError");
  }

  return value;
}

function isTempHarnessHandle(value: unknown): value is TempHarnessHandle {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  const allowedKeys = [
    "app_state_ready",
    "database_path",
    "fixture_manifest_id",
    "workspace_id",
  ];

  if (keys.length !== allowedKeys.length || keys.some((key, index) => key !== allowedKeys[index])) {
    return false;
  }

  return (
    typeof record.workspace_id === "string" &&
    typeof record.database_path === "string" &&
    typeof record.fixture_manifest_id === "string" &&
    typeof record.app_state_ready === "boolean"
  );
}

function isTempHarnessError(value: string): value is TempHarnessError {
  return (TEMP_HARNESS_ERRORS as readonly string[]).includes(value);
}
