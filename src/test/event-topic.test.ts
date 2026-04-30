import { describe, expect, it } from "vitest";

import eventTopics from "../../product-strategy/contracts/fixtures/wu-0a-05/event-topics.json";
import inertTopics from "../../product-strategy/contracts/fixtures/wu-0a-05/inert-topics.json";
import parseErrors from "../../product-strategy/contracts/fixtures/wu-0a-05/parse-errors.json";
import parseSuccess from "../../product-strategy/contracts/fixtures/wu-0a-05/parse-success.json";
import phase0aLiveTopics from "../../product-strategy/contracts/fixtures/wu-0a-05/phase-0a-live-topics.json";
import {
  EVENT_TOPICS,
  EventTopicParseError,
  PHASE_0A_LIVE_TOPICS,
  isPhase0aLiveTopic,
  parseEventTopic,
  type EventTopic,
  type EventTopicError,
} from "../contracts/event-topic";

describe("EventTopic contract", () => {
  it("keeps the TypeScript union aligned with the canonical Rust fixture", () => {
    // Risk: TypeScript accepts topic strings Rust rejects. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript taxonomy and
    // parser parity".
    const expected = [
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
    ] satisfies EventTopic[];
    const selected: EventTopic = "runtime";
    // @ts-expect-error "unknown" is outside the documented EventTopic union.
    const rejected: EventTopic = "unknown";

    expect(EVENT_TOPICS).toEqual(expected);
    expect(EVENT_TOPICS).toEqual(eventTopics);
    expect(selected).toBe("runtime");
    expect(rejected).toBe("unknown");
  });

  it("parses every documented lowercase topic from the shared fixture", () => {
    // Risk: TypeScript accepts topic strings Rust rejects. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript taxonomy and
    // parser parity".
    for (const parseCase of parseSuccess) {
      expect(parseEventTopic(parseCase.raw)).toBe(parseCase.topic);
    }
  });

  it("returns the documented parse errors for empty and unknown topics", () => {
    // Risk: TypeScript accepts topic strings Rust rejects. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript taxonomy and
    // parser parity".
    for (const parseCase of parseErrors) {
      expect(() => parseEventTopic(parseCase.raw)).toThrow(EventTopicParseError);

      try {
        parseEventTopic(parseCase.raw);
      } catch (error) {
        expect(error).toBeInstanceOf(EventTopicParseError);
        expect((error as EventTopicParseError).kind).toBe(parseCase.error as EventTopicError);
      }
    }
  });

  it("keeps later-domain placeholders inert and runtime live in Phase 0A", () => {
    // Risk: Phase 0A accidentally exposes later-domain event payload producers.
    // Level: unit. Source: proposal test-intent "Rust Phase 0A live-topic
    // gating".
    expect(PHASE_0A_LIVE_TOPICS).toEqual(phase0aLiveTopics);
    expect(PHASE_0A_LIVE_TOPICS).toEqual(["runtime"]);

    for (const topic of inertTopics) {
      expect(EVENT_TOPICS).toContain(topic);
      expect(isPhase0aLiveTopic(topic as EventTopic)).toBe(false);
    }

    expect(isPhase0aLiveTopic("runtime")).toBe(true);
  });
});
