import { describe, expect, it } from "vitest";

import canonicalArgs from "../../product-strategy/contracts/fixtures/wu-0a-08/canonical-args.json";
import canonicalResponse from "../../product-strategy/contracts/fixtures/wu-0a-08/canonical-response.json";
import errorsFixture from "../../product-strategy/contracts/fixtures/wu-0a-08/subscribe-workspace-events-errors.json";
import invalidShapes from "../../product-strategy/contracts/fixtures/wu-0a-08/invalid-shapes.json";
import {
  SUBSCRIBE_WORKSPACE_EVENTS_ERRORS,
  parseSubscribeWorkspaceEventsArgs,
  parseSubscribeWorkspaceEventsError,
  parseSubscribeWorkspaceEventsResponse,
  type SubscribeWorkspaceEventsArgs,
  type SubscribeWorkspaceEventsError,
} from "../contracts/subscribe-workspace-events";

describe("SubscribeWorkspaceEvents contract", () => {
  it("round-trips canonical args, response, and error fixtures", () => {
    // Risk: Rust/TS contract drift. Level: unit/typecheck. Source: WU-0A-08
    // proposal test-intent "TypeScript DTO/parser".
    const args = parseSubscribeWorkspaceEventsArgs(canonicalArgs);
    const response = parseSubscribeWorkspaceEventsResponse(canonicalResponse);

    const typedArgs: SubscribeWorkspaceEventsArgs = {
      workspace_id: "workspace-alpha",
      topic: "runtime",
      channel: "__TAURI_CHANNEL__42",
    };
    const selectedError: SubscribeWorkspaceEventsError = "ChannelUnavailable";
    // @ts-expect-error "OtherError" is outside the documented error union.
    const rejectedError: SubscribeWorkspaceEventsError = "OtherError";
    // @ts-expect-error "not-a-topic" is outside the documented EventTopic union.
    const rejectedTopic: SubscribeWorkspaceEventsArgs = { ...typedArgs, topic: "not-a-topic" };

    expect(args).toEqual(typedArgs);
    expect(response).toBe("sub-1770000000000000000-1");
    expect(SUBSCRIBE_WORKSPACE_EVENTS_ERRORS).toEqual(errorsFixture);
    expect(selectedError).toBe("ChannelUnavailable");
    expect(rejectedError).toBe("OtherError");
    expect(rejectedTopic.topic).toBe("not-a-topic");

    for (const errorName of errorsFixture) {
      expect(parseSubscribeWorkspaceEventsError(errorName)).toBe(errorName);
    }
  });

  it("rejects malformed DTO, response, and error shapes", () => {
    // Risk: TypeScript accepts values Rust rejects. Level: unit. Source:
    // WU-0A-08 proposal test-intent "TypeScript DTO/parser".
    for (const invalidShape of invalidShapes) {
      if (invalidShape.parser === "args") {
        expect(
          () => parseSubscribeWorkspaceEventsArgs(invalidShape.value),
          invalidShape.name,
        ).toThrow("Invalid SubscribeWorkspaceEventsArgs");
      } else if (invalidShape.parser === "response") {
        expect(
          () => parseSubscribeWorkspaceEventsResponse(invalidShape.value),
          invalidShape.name,
        ).toThrow("Invalid SubscribeWorkspaceEventsResponse");
      } else {
        expect(
          () => parseSubscribeWorkspaceEventsError(invalidShape.value),
          invalidShape.name,
        ).toThrow("Invalid SubscribeWorkspaceEventsError");
      }
    }
  });
});
