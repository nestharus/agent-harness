import { invoke as tauriInvoke } from "@tauri-apps/api/core";

import {
  CommandError,
  CommandErrorFailure,
  parseHarnessCommand,
  parseHarnessCommandResponse,
  type HarnessCommand,
  type HarnessCommandArgsByCommand,
  type HarnessCommandResponseByCommand,
} from "../contracts/harness-command";

export type TauriInvokeShim = <TResponse>(command: string, args: unknown) => Promise<TResponse>;

export type InvokeCommand = <TCommand extends HarnessCommand>(
  command: TCommand,
  args: HarnessCommandArgsByCommand[TCommand],
) => Promise<HarnessCommandResponseByCommand[TCommand]>;

export type InvokeCommandFixtureResponses = Partial<{
  [TCommand in HarnessCommand]: HarnessCommandResponseByCommand[TCommand];
}>;

let activeFixtureResponses: ReadonlyMap<HarnessCommand, unknown> | undefined;

export function seedInvokeCommandFixtures(responses: InvokeCommandFixtureResponses): void {
  activeFixtureResponses = new Map(
    Object.entries(responses) as Array<[HarnessCommand, unknown]>,
  );
}

export function clearInvokeCommandFixtures(): void {
  activeFixtureResponses = undefined;
}

const defaultInvokeShim: TauriInvokeShim = <TResponse>(
  command: string,
  args: unknown,
): Promise<TResponse> => {
  if (activeFixtureResponses) {
    const parsedCommand = parseHarnessCommand(command);

    if (activeFixtureResponses.has(parsedCommand)) {
      return Promise.resolve(activeFixtureResponses.get(parsedCommand) as TResponse);
    }

    throw new CommandErrorFailure(CommandError.InvokeRejected, {
      command: parsedCommand,
      cause: new Error(`No invoke fixture seeded for ${parsedCommand}`),
    });
  }

  return tauriInvoke<TResponse>(command, args as Record<string, unknown>);
};

export function createInvokeCommand(invokeShim: TauriInvokeShim = defaultInvokeShim): InvokeCommand {
  return async <TCommand extends HarnessCommand>(
    command: TCommand,
    args: HarnessCommandArgsByCommand[TCommand],
  ): Promise<HarnessCommandResponseByCommand[TCommand]> => {
    const parsedCommand = parseHarnessCommand(command) as TCommand;

    try {
      assertJsonSerializable(args);
    } catch (error) {
      throw new CommandErrorFailure(CommandError.ArgumentSerializationFailed, {
        command: parsedCommand,
        cause: error,
      });
    }

    let rawResponse: unknown;
    try {
      rawResponse = await invokeShim<unknown>(parsedCommand, args);
    } catch (error) {
      throw new CommandErrorFailure(CommandError.InvokeRejected, {
        command: parsedCommand,
        cause: error,
      });
    }

    try {
      return parseHarnessCommandResponse(parsedCommand, rawResponse);
    } catch (error) {
      throw new CommandErrorFailure(CommandError.ResponseDeserializationFailed, {
        command: parsedCommand,
        cause: error,
      });
    }
  };
}

export const invokeCommand = createInvokeCommand();

function assertJsonSerializable(value: unknown): void {
  validateJsonValue(value, new WeakSet<object>());
  JSON.stringify(value);
}

function validateJsonValue(value: unknown, seen: WeakSet<object>): void {
  if (value === null) {
    return;
  }

  switch (typeof value) {
    case "string":
    case "boolean":
      return;
    case "number":
      if (!Number.isFinite(value)) {
        throw new TypeError("non-finite numbers are not JSON serializable");
      }
      return;
    case "bigint":
    case "function":
    case "symbol":
    case "undefined":
      throw new TypeError(`${typeof value} values are not JSON serializable`);
    case "object":
      validateJsonObject(value, seen);
      return;
  }
}

function validateJsonObject(value: object, seen: WeakSet<object>): void {
  if (seen.has(value)) {
    throw new TypeError("circular references are not JSON serializable");
  }

  seen.add(value);

  if (Array.isArray(value)) {
    for (const item of value) {
      validateJsonValue(item, seen);
    }
    seen.delete(value);
    return;
  }

  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw new TypeError("only plain objects are JSON serializable as invoke args");
  }

  for (const key of Reflect.ownKeys(value)) {
    if (typeof key === "symbol") {
      throw new TypeError("symbol keys are not JSON serializable");
    }

    validateJsonValue((value as Record<string, unknown>)[key], seen);
  }

  seen.delete(value);
}
