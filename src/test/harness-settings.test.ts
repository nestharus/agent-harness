import { describe, expect, it } from "vitest";

import canonicalSettings from "../../product-strategy/contracts/fixtures/wu-0a-02/canonical-settings.json";
import invalidSettingsError from "../../product-strategy/contracts/fixtures/wu-0a-02/invalid-settings-error.json";
import settingsErrors from "../../product-strategy/contracts/fixtures/wu-0a-02/settings-errors.json";
import {
  HARNESS_LOG_LEVELS,
  parseHarnessSettings,
  parseSettingsError,
  type HarnessLogLevel,
  type HarnessSettings,
  type SettingsError,
} from "../contracts/harness-settings";

describe("HarnessSettings contract", () => {
  it("parses the canonical fixture with the exact Rust-owned DTO field set", () => {
    // Risk: TypeScript accepts shapes Rust does not own. Level: unit. Source:
    // proposal test-intent "TypeScript DTO and parser shape".
    const parsed = parseHarnessSettings(canonicalSettings);
    const expected: HarnessSettings = parsed;

    expect(expected).toEqual(canonicalSettings);
    expect(Object.keys(parsed).sort()).toEqual([
      "agent_runner_bin",
      "database_path",
      "log_level",
      "profile_name",
      "storage_root",
      "workspace_id",
    ]);
  });

  it("rejects invalid HarnessSettings shapes", () => {
    // Risk: TypeScript accepts shapes Rust does not own. Level: unit. Source:
    // proposal test-intent "TypeScript DTO and parser shape".
    expect(() =>
      parseHarnessSettings({
        ...canonicalSettings,
        log_level: "verbose",
      }),
    ).toThrow("Invalid HarnessSettings");
    expect(() =>
      parseHarnessSettings({
        ...canonicalSettings,
        extra_field: true,
      }),
    ).toThrow("Invalid HarnessSettings");
    expect(() =>
      parseHarnessSettings({
        ...canonicalSettings,
        profile_name: 42,
      }),
    ).toThrow("Invalid HarnessSettings");
  });

  it("round-trips documented SettingsError strings and rejects unknown variants", () => {
    // Risk: Rust/TS error taxonomy drift. Level: particular-integration.
    // Source: WU acceptance criteria and contract error fixtures.
    const expectedErrors = [
      "EmptyWorkspaceId",
      "EmptyStorageRoot",
      "EmptyDatabasePath",
      "MissingAgentRunnerBin",
      "InvalidLogLevel",
      "ConfigFileUnreadable",
    ] satisfies SettingsError[];

    expect(settingsErrors.map(parseSettingsError)).toEqual(expectedErrors);
    expect(() => parseSettingsError(invalidSettingsError)).toThrow("Invalid SettingsError");
  });

  it("enforces the documented log-level union at type level", () => {
    // Risk: widened log-level type. Level: unit/typecheck. Source: documented
    // log-level union and proposal test-intent.
    const accepted = ["trace", "debug", "info", "warn", "error"] satisfies HarnessLogLevel[];
    const selected: HarnessLogLevel = "trace";
    // @ts-expect-error "verbose" is outside the documented log-level union.
    const rejected: HarnessLogLevel = "verbose";

    expect(HARNESS_LOG_LEVELS).toEqual(accepted);
    expect(selected).toBe("trace");
    expect(rejected).toBe("verbose");
  });
});
