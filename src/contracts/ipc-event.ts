import { EVENT_TOPICS, type EventTopic } from "./event-topic";

export interface IpcEvent<T> {
  event_id: string;
  workspace_id: string;
  topic: EventTopic;
  payload: T;
  created_at: string;
  trace_context_id?: string;
}

export const IPC_EVENT_ERRORS = [
  "EmptyEventId",
  "EmptyWorkspaceId",
  "UnknownTopic",
  "PayloadSerializationFailed",
] as const;

export type IpcEventError = (typeof IPC_EVENT_ERRORS)[number];

export function parseIpcEvent<T>(
  value: unknown,
  parsePayload: (payload: unknown) => T,
): IpcEvent<T> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Invalid IpcEvent");
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  const allowedKeys = ["created_at", "event_id", "payload", "topic", "workspace_id"];

  if (typeof record.trace_context_id !== "undefined") {
    allowedKeys.push("trace_context_id");
  }
  allowedKeys.sort();

  if (keys.length !== allowedKeys.length || keys.some((key, index) => key !== allowedKeys[index])) {
    throw new Error("Invalid IpcEvent");
  }

  if (
    typeof record.event_id !== "string" ||
    record.event_id.length === 0 ||
    typeof record.workspace_id !== "string" ||
    typeof record.topic !== "string" ||
    !isEventTopic(record.topic) ||
    typeof record.created_at !== "string" ||
    record.created_at.length === 0 ||
    (typeof record.trace_context_id !== "undefined" &&
      typeof record.trace_context_id !== "string")
  ) {
    throw new Error("Invalid IpcEvent");
  }

  let payload: T;
  try {
    payload = parsePayload(record.payload);
  } catch {
    throw new Error("Invalid IpcEvent");
  }

  return {
    event_id: record.event_id,
    workspace_id: record.workspace_id,
    topic: record.topic,
    payload,
    created_at: record.created_at,
    ...(typeof record.trace_context_id === "string"
      ? { trace_context_id: record.trace_context_id }
      : {}),
  };
}

export function parseIpcEventError(value: unknown): IpcEventError {
  if (typeof value !== "string" || !isIpcEventError(value)) {
    throw new Error("Invalid IpcEventError");
  }

  return value;
}

function isEventTopic(value: string): value is EventTopic {
  return (EVENT_TOPICS as readonly string[]).includes(value);
}

function isIpcEventError(value: string): value is IpcEventError {
  return (IPC_EVENT_ERRORS as readonly string[]).includes(value);
}
