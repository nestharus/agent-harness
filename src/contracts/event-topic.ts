export const EVENT_TOPICS = [
  "graph",
  "render",
  "provider",
  "optimizer",
  "worker",
  "question",
  "recovery",
  "budget",
  "audit",
  "runtime",
] as const;

export type EventTopic = (typeof EVENT_TOPICS)[number];

export const EVENT_TOPIC_ERRORS = ["UnknownTopic", "EmptyTopic"] as const;

export type EventTopicError = (typeof EVENT_TOPIC_ERRORS)[number];

export const PHASE_0A_LIVE_TOPICS = ["runtime"] as const satisfies readonly EventTopic[];

export class EventTopicParseError extends Error {
  readonly kind: EventTopicError;

  constructor(kind: EventTopicError) {
    super(kind);
    this.name = "EventTopicParseError";
    this.kind = kind;
  }
}

export function parseEventTopic(raw: string): EventTopic {
  if (raw === "") {
    throw new EventTopicParseError("EmptyTopic");
  }

  if (isEventTopic(raw)) {
    return raw;
  }

  throw new EventTopicParseError("UnknownTopic");
}

export function isPhase0aLiveTopic(topic: EventTopic): boolean {
  return (PHASE_0A_LIVE_TOPICS as readonly EventTopic[]).includes(topic);
}

function isEventTopic(value: string): value is EventTopic {
  return (EVENT_TOPICS as readonly string[]).includes(value);
}
