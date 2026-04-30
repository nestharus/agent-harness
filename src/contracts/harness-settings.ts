export const HARNESS_LOG_LEVELS = ["trace", "debug", "info", "warn", "error"] as const;

export type HarnessLogLevel = (typeof HARNESS_LOG_LEVELS)[number];

export interface HarnessSettings {
  workspace_id: string;
  storage_root: string;
  database_path: string;
  agent_runner_bin: string;
  log_level: HarnessLogLevel;
  profile_name?: string;
}

export const SETTINGS_ERRORS = [
  "EmptyWorkspaceId",
  "EmptyStorageRoot",
  "EmptyDatabasePath",
  "MissingAgentRunnerBin",
  "InvalidLogLevel",
  "ConfigFileUnreadable",
] as const;

export type SettingsError = (typeof SETTINGS_ERRORS)[number];

export function parseHarnessSettings(value: unknown): HarnessSettings {
  if (!isHarnessSettings(value)) {
    throw new Error("Invalid HarnessSettings");
  }

  return value;
}

export function parseSettingsError(value: unknown): SettingsError {
  if (typeof value !== "string" || !isSettingsError(value)) {
    throw new Error("Invalid SettingsError");
  }

  return value;
}

function isHarnessSettings(value: unknown): value is HarnessSettings {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  const allowedKeys = [
    "agent_runner_bin",
    "database_path",
    "log_level",
    "storage_root",
    "workspace_id",
  ];

  if (typeof record.profile_name !== "undefined") {
    allowedKeys.push("profile_name");
  }
  allowedKeys.sort();

  if (keys.length !== allowedKeys.length || keys.some((key, index) => key !== allowedKeys[index])) {
    return false;
  }

  return (
    typeof record.workspace_id === "string" &&
    typeof record.storage_root === "string" &&
    typeof record.database_path === "string" &&
    typeof record.agent_runner_bin === "string" &&
    typeof record.log_level === "string" &&
    isHarnessLogLevel(record.log_level) &&
    (typeof record.profile_name === "undefined" || typeof record.profile_name === "string")
  );
}

function isHarnessLogLevel(value: string): value is HarnessLogLevel {
  return (HARNESS_LOG_LEVELS as readonly string[]).includes(value);
}

function isSettingsError(value: string): value is SettingsError {
  return (SETTINGS_ERRORS as readonly string[]).includes(value);
}
