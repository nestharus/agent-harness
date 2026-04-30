import { EVENT_TOPICS, type EventTopic } from "./event-topic";

export interface SubscribeWorkspaceEventsArgs {
  workspace_id: string;
  topic: EventTopic;
  channel: string;
}

export const SUBSCRIBE_WORKSPACE_EVENTS_ERRORS = [
  "EmptyWorkspaceId",
  "UnknownTopic",
  "ChannelUnavailable",
  "AppStateUnavailable",
] as const;

export type SubscribeWorkspaceEventsError = (typeof SUBSCRIBE_WORKSPACE_EVENTS_ERRORS)[number];

export function parseSubscribeWorkspaceEventsArgs(value: unknown): SubscribeWorkspaceEventsArgs {
  if (!isRecord(value) || !hasExactKeys(value, ["channel", "topic", "workspace_id"])) {
    throw new Error("Invalid SubscribeWorkspaceEventsArgs");
  }

  if (
    typeof value.workspace_id !== "string" ||
    value.workspace_id.length === 0 ||
    typeof value.channel !== "string" ||
    value.channel.length === 0 ||
    typeof value.topic !== "string" ||
    !isEventTopic(value.topic)
  ) {
    throw new Error("Invalid SubscribeWorkspaceEventsArgs");
  }

  return {
    workspace_id: value.workspace_id,
    topic: value.topic,
    channel: value.channel,
  };
}

export function parseSubscribeWorkspaceEventsResponse(value: unknown): string {
  if (typeof value !== "string" || value.length === 0) {
    throw new Error("Invalid SubscribeWorkspaceEventsResponse");
  }

  return value;
}

export function parseSubscribeWorkspaceEventsError(value: unknown): SubscribeWorkspaceEventsError {
  if (typeof value !== "string" || !isSubscribeWorkspaceEventsError(value)) {
    throw new Error("Invalid SubscribeWorkspaceEventsError");
  }

  return value;
}

function isEventTopic(value: string): value is EventTopic {
  return (EVENT_TOPICS as readonly string[]).includes(value);
}

function isSubscribeWorkspaceEventsError(value: string): value is SubscribeWorkspaceEventsError {
  return (SUBSCRIBE_WORKSPACE_EVENTS_ERRORS as readonly string[]).includes(value);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function hasExactKeys(value: Record<string, unknown>, expectedKeys: string[]): boolean {
  const actual = Object.keys(value).sort();
  const expected = [...expectedKeys].sort();

  return actual.length === expected.length && actual.every((key, index) => key === expected[index]);
}
