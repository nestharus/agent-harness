import { describe, expect, it } from "vitest";

import alternateCommandNames from "../../product-strategy/contracts/fixtures/wu-0a-07/alternate-command-names.json";
import commandErrors from "../../product-strategy/contracts/fixtures/wu-0a-07/command-errors.json";
import commandNames from "../../product-strategy/contracts/fixtures/wu-0a-07/command-names.json";
import getHarnessSettingsHappyPath from "../../product-strategy/contracts/fixtures/wu-0a-07/get-harness-settings-happy-path.json";
import invalidPingRuntimeResponse from "../../product-strategy/contracts/fixtures/wu-0a-07/invalid-ping-runtime-response.json";
import pingRuntimeHappyPath from "../../product-strategy/contracts/fixtures/wu-0a-07/ping-runtime-happy-path.json";
import subscribeWorkspaceEventsHappyPath from "../../product-strategy/contracts/fixtures/wu-0a-07/subscribe-workspace-events-happy-path.json";
import {
  COMMAND_ERRORS,
  CommandError,
  CommandErrorFailure,
  HARNESS_COMMANDS,
  parseCommandError,
  parseHarnessCommand,
  parsePingRuntimeResponse,
  parseSubscribeWorkspaceEventsAck,
  parseSubscribeWorkspaceEventsArgs,
  type HarnessCommand,
  type PingRuntimeResponse,
  type SubscribeWorkspaceEventsAck,
} from "../contracts/harness-command";
import { createInvokeCommand, type TauriInvokeShim } from "../ipc/invoke-command";

interface InvokeCall {
  command: string;
  args: unknown;
}

function resolvingShim(response: unknown, calls: InvokeCall[]): TauriInvokeShim {
  return async <TResponse>(command: string, args: unknown): Promise<TResponse> => {
    calls.push({ command, args });
    return response as TResponse;
  };
}

function rejectingShim(error: unknown, calls: InvokeCall[]): TauriInvokeShim {
  return async <TResponse>(command: string, args: unknown): Promise<TResponse> => {
    calls.push({ command, args });
    throw error;
  };
}

async function expectCommandError(promise: Promise<unknown>, kind: CommandError): Promise<void> {
  await expect(promise).rejects.toMatchObject({
    name: "CommandErrorFailure",
    kind,
  });
}

describe("HarnessCommand contract", () => {
  it("keeps the TypeScript union aligned with the canonical command fixture", () => {
    // Risk: TS accepts command names Rust rejects. Level: unit/typecheck.
    // Source: proposal test-intent "TypeScript command taxonomy and parser
    // parity".
    const expected = [
      "get_harness_settings",
      "subscribe_workspace_events",
      "ping_runtime",
    ] satisfies HarnessCommand[];
    const selected: HarnessCommand = "ping_runtime";
    // @ts-expect-error "unknown" is outside the documented HarnessCommand union.
    const rejected: HarnessCommand = "unknown";

    expect(HARNESS_COMMANDS).toEqual(expected);
    expect(HARNESS_COMMANDS).toEqual(commandNames);
    expect(selected).toBe("ping_runtime");
    expect(rejected).toBe("unknown");

    for (const commandName of commandNames) {
      expect(parseHarnessCommand(commandName)).toBe(commandName);
    }
  });

  it("rejects alternate command forms from the shared fixture", () => {
    // Risk: TS accepts command names Rust rejects. Level: unit. Source:
    // proposal test-intent "TypeScript command taxonomy and parser parity".
    for (const commandName of alternateCommandNames) {
      expect(() => parseHarnessCommand(commandName), commandName).toThrow(CommandErrorFailure);
      try {
        parseHarnessCommand(commandName);
      } catch (error) {
        expect((error as CommandErrorFailure).kind).toBe(CommandError.UnknownCommand);
      }
    }
  });

  it("keeps CommandError variants aligned with fixtures", () => {
    // Risk: errors are renamed or unreachable. Level: unit/typecheck. Source:
    // proposal test-intent "CommandError reachability".
    const expected = [
      "UnknownCommand",
      "ArgumentSerializationFailed",
      "InvokeRejected",
      "ResponseDeserializationFailed",
    ] satisfies CommandError[];
    const selected: CommandError = "InvokeRejected";
    // @ts-expect-error "OtherError" is outside the documented CommandError union.
    const rejected: CommandError = "OtherError";

    expect(COMMAND_ERRORS).toEqual(expected);
    expect(COMMAND_ERRORS).toEqual(commandErrors);
    expect(selected).toBe("InvokeRejected");
    expect(rejected).toBe("OtherError");

    for (const errorName of commandErrors) {
      expect(parseCommandError(errorName)).toBe(errorName);
    }
    expect(() => parseCommandError("OtherError")).toThrow("Invalid CommandError");
  });

  it("resolves ping_runtime with the documented fixture response through the invoke shim", async () => {
    // Risk: helper erases command-specific contracts or calls real runtime
    // paths. Level: unit. Source: proposal test-intent "Typed invoke happy
    // paths".
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(resolvingShim(pingRuntimeHappyPath.response, calls));

    const response = await invokeCommand("ping_runtime", {});
    const typed: PingRuntimeResponse = response;

    expect(response).toEqual(pingRuntimeHappyPath.response);
    expect(parsePingRuntimeResponse(response)).toEqual(pingRuntimeHappyPath.response);
    expect(typed.command).toBe("ping_runtime");
    expect(calls).toEqual([
      {
        command: pingRuntimeHappyPath.command,
        args: pingRuntimeHappyPath.args,
      },
    ]);
  });

  it("preserves get_harness_settings command and empty args while parsing HarnessSettings", async () => {
    // Risk: helper erases command-specific contracts or calls real runtime
    // paths. Level: unit. Source: proposal test-intent "Typed invoke happy
    // paths".
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(
      resolvingShim(getHarnessSettingsHappyPath.response, calls),
    );

    const response = await invokeCommand("get_harness_settings", {});

    expect(response).toEqual(getHarnessSettingsHappyPath.response);
    expect(response.workspace_id).toBe("workspace-alpha");
    expect(calls).toEqual([
      {
        command: getHarnessSettingsHappyPath.command,
        args: getHarnessSettingsHappyPath.args,
      },
    ]);
  });

  it("preserves subscribe_workspace_events topic and channel registration args without a producer", async () => {
    // Risk: helper erases command-specific contracts or opens real runtime
    // paths. Level: unit. Source: proposal test-intent "Typed invoke happy
    // paths".
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(
      resolvingShim(subscribeWorkspaceEventsHappyPath.response, calls),
    );
    const args = parseSubscribeWorkspaceEventsArgs(subscribeWorkspaceEventsHappyPath.args);

    const response = await invokeCommand("subscribe_workspace_events", args);
    const typed: SubscribeWorkspaceEventsAck = response;

    expect(response).toEqual(subscribeWorkspaceEventsHappyPath.response);
    expect(parseSubscribeWorkspaceEventsAck(response)).toEqual(
      subscribeWorkspaceEventsHappyPath.response,
    );
    expect(typed.topic).toBe("runtime");
    expect(calls).toEqual([
      {
        command: subscribeWorkspaceEventsHappyPath.command,
        args: subscribeWorkspaceEventsHappyPath.args,
      },
    ]);
  });

  it("rejects unknown commands before invoking the shim", async () => {
    // Risk: errors are renamed or unreachable. Level: unit. Source: proposal
    // test-intent "CommandError reachability".
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(resolvingShim(pingRuntimeHappyPath.response, calls));

    await expectCommandError(
      invokeCommand("unknown" as HarnessCommand, {}),
      CommandError.UnknownCommand,
    );
    expect(calls).toEqual([]);
  });

  it("rejects non-serializable arguments before invoking the shim", async () => {
    // Risk: errors are renamed or unreachable. Level: unit. Source: proposal
    // test-intent "CommandError reachability" and assumption A3.
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(resolvingShim(pingRuntimeHappyPath.response, calls));
    const circularArgs: Record<string, unknown> = {};
    circularArgs.self = circularArgs;

    await expectCommandError(
      invokeCommand("ping_runtime", circularArgs as never),
      CommandError.ArgumentSerializationFailed,
    );
    await expectCommandError(
      invokeCommand("ping_runtime", { sequence: BigInt(1) } as never),
      CommandError.ArgumentSerializationFailed,
    );
    expect(calls).toEqual([]);
  });

  it("maps a rejected invoke shim to InvokeRejected", async () => {
    // Risk: errors are renamed or leak raw Tauri errors. Level: unit. Source:
    // proposal test-intent "CommandError reachability".
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(rejectingShim(new Error("tauri unavailable"), calls));

    await expectCommandError(invokeCommand("ping_runtime", {}), CommandError.InvokeRejected);
    expect(calls).toEqual([
      {
        command: "ping_runtime",
        args: {},
      },
    ]);
  });

  it("maps invalid fixture responses to ResponseDeserializationFailed", async () => {
    // Risk: errors are renamed or bad responses bypass command-specific
    // parsers. Level: unit. Source: proposal test-intent "CommandError
    // reachability".
    const calls: InvokeCall[] = [];
    const invokeCommand = createInvokeCommand(resolvingShim(invalidPingRuntimeResponse, calls));

    await expectCommandError(
      invokeCommand("ping_runtime", {}),
      CommandError.ResponseDeserializationFailed,
    );
    expect(calls).toEqual([
      {
        command: "ping_runtime",
        args: {},
      },
    ]);
  });
});
