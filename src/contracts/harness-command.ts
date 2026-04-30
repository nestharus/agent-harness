import { EVENT_TOPICS, type EventTopic } from "./event-topic";
import { parseHarnessSettings, type HarnessSettings } from "./harness-settings";

export const HARNESS_COMMANDS = [
  "get_harness_settings",
  "subscribe_workspace_events",
  "ping_runtime",
] as const;

export type HarnessCommand = (typeof HARNESS_COMMANDS)[number];

export const CommandError = {
  UnknownCommand: "UnknownCommand",
  ArgumentSerializationFailed: "ArgumentSerializationFailed",
  InvokeRejected: "InvokeRejected",
  ResponseDeserializationFailed: "ResponseDeserializationFailed",
} as const;

export const COMMAND_ERRORS = [
  CommandError.UnknownCommand,
  CommandError.ArgumentSerializationFailed,
  CommandError.InvokeRejected,
  CommandError.ResponseDeserializationFailed,
] as const;

export type CommandError = (typeof COMMAND_ERRORS)[number];

export class CommandErrorFailure extends Error {
  readonly kind: CommandError;
  readonly command?: string;

  constructor(kind: CommandError, options: { command?: string; cause?: unknown } = {}) {
    super(kind, { cause: options.cause });
    this.name = "CommandErrorFailure";
    this.kind = kind;
    this.command = options.command;
  }
}

export type EmptyCommandArgs = Record<string, never>;

export interface SubscribeWorkspaceEventsArgs {
  topic: EventTopic;
  channelId: string;
}

export interface SubscribeWorkspaceEventsAck {
  subscribed: true;
  topic: EventTopic;
  channelId: string;
}

export interface PingRuntimeResponse {
  ok: true;
  command: "ping_runtime";
  runtime: "agent-harness";
  phase: "0A";
}

export interface HarnessCommandArgsByCommand {
  get_harness_settings: EmptyCommandArgs;
  subscribe_workspace_events: SubscribeWorkspaceEventsArgs;
  ping_runtime: EmptyCommandArgs;
}

export interface HarnessCommandResponseByCommand {
  get_harness_settings: HarnessSettings;
  subscribe_workspace_events: SubscribeWorkspaceEventsAck;
  ping_runtime: PingRuntimeResponse;
}

export function parseHarnessCommand(value: unknown): HarnessCommand {
  if (typeof value === "string" && isHarnessCommand(value)) {
    return value;
  }

  throw new CommandErrorFailure(CommandError.UnknownCommand, {
    command: typeof value === "string" ? value : undefined,
  });
}

export function parseCommandError(value: unknown): CommandError {
  if (typeof value !== "string" || !isCommandError(value)) {
    throw new Error("Invalid CommandError");
  }

  return value;
}

export function parseEmptyCommandArgs(value: unknown): EmptyCommandArgs {
  if (!isRecord(value) || Object.keys(value).length !== 0) {
    throw new Error("Invalid EmptyCommandArgs");
  }

  return value as EmptyCommandArgs;
}

export function parseSubscribeWorkspaceEventsArgs(value: unknown): SubscribeWorkspaceEventsArgs {
  if (!isRecord(value) || !hasExactKeys(value, ["channelId", "topic"])) {
    throw new Error("Invalid SubscribeWorkspaceEventsArgs");
  }

  if (typeof value.channelId !== "string" || !isEventTopic(value.topic)) {
    throw new Error("Invalid SubscribeWorkspaceEventsArgs");
  }

  return {
    topic: value.topic,
    channelId: value.channelId,
  };
}

export function parseSubscribeWorkspaceEventsAck(value: unknown): SubscribeWorkspaceEventsAck {
  if (!isRecord(value) || !hasExactKeys(value, ["channelId", "subscribed", "topic"])) {
    throw new Error("Invalid SubscribeWorkspaceEventsAck");
  }

  if (value.subscribed !== true || typeof value.channelId !== "string" || !isEventTopic(value.topic)) {
    throw new Error("Invalid SubscribeWorkspaceEventsAck");
  }

  return {
    subscribed: true,
    topic: value.topic,
    channelId: value.channelId,
  };
}

export function parsePingRuntimeResponse(value: unknown): PingRuntimeResponse {
  if (!isRecord(value) || !hasExactKeys(value, ["command", "ok", "phase", "runtime"])) {
    throw new Error("Invalid PingRuntimeResponse");
  }

  if (
    value.ok !== true ||
    value.command !== "ping_runtime" ||
    value.runtime !== "agent-harness" ||
    value.phase !== "0A"
  ) {
    throw new Error("Invalid PingRuntimeResponse");
  }

  return {
    ok: true,
    command: "ping_runtime",
    runtime: "agent-harness",
    phase: "0A",
  };
}

export function parseHarnessCommandResponse<TCommand extends HarnessCommand>(
  command: TCommand,
  value: unknown,
): HarnessCommandResponseByCommand[TCommand] {
  switch (command) {
    case "get_harness_settings":
      return parseHarnessSettings(value) as HarnessCommandResponseByCommand[TCommand];
    case "subscribe_workspace_events":
      return parseSubscribeWorkspaceEventsAck(value) as HarnessCommandResponseByCommand[TCommand];
    case "ping_runtime":
      return parsePingRuntimeResponse(value) as HarnessCommandResponseByCommand[TCommand];
  }
}

function isHarnessCommand(value: string): value is HarnessCommand {
  return (HARNESS_COMMANDS as readonly string[]).includes(value);
}

function isCommandError(value: string): value is CommandError {
  return (COMMAND_ERRORS as readonly string[]).includes(value);
}

function isEventTopic(value: unknown): value is EventTopic {
  return typeof value === "string" && (EVENT_TOPICS as readonly string[]).includes(value);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function hasExactKeys(value: Record<string, unknown>, expectedKeys: string[]): boolean {
  const actual = Object.keys(value).sort();
  const expected = [...expectedKeys].sort();

  return actual.length === expected.length && actual.every((key, index) => key === expected[index]);
}
