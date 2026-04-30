import { describe, expect, it } from "vitest";

import backendSpanErrors from "../../product-strategy/contracts/fixtures/wu-0a-10/backend-span-errors.json";
import backendSpanStatuses from "../../product-strategy/contracts/fixtures/wu-0a-10/backend-span-statuses.json";
import completedHappyPath from "../../product-strategy/contracts/fixtures/wu-0a-10/completed-happy-path.json";
import errorCompletedBeforeStarted from "../../product-strategy/contracts/fixtures/wu-0a-10/error-completed-before-started.json";
import errorEmptySpanName from "../../product-strategy/contracts/fixtures/wu-0a-10/error-empty-span-name.json";
import errorFailedWithoutErrorRef from "../../product-strategy/contracts/fixtures/wu-0a-10/error-failed-without-error-ref.json";
import errorTraceContextInvalid from "../../product-strategy/contracts/fixtures/wu-0a-10/error-trace-context-invalid.json";
import failedHappyPath from "../../product-strategy/contracts/fixtures/wu-0a-10/failed-happy-path.json";
import invalidEvents from "../../product-strategy/contracts/fixtures/wu-0a-10/invalid-events.json";
import runtimeOnlyTopicEvent from "../../product-strategy/contracts/fixtures/wu-0a-10/runtime-only-topic-event.json";
import startedHappyPath from "../../product-strategy/contracts/fixtures/wu-0a-10/started-happy-path.json";
import { parseIpcEvent, type IpcEvent } from "../contracts/ipc-event";
import {
  BACKEND_SPAN_ERRORS,
  BACKEND_SPAN_STATUSES,
  parseBackendSpanError,
  parseBackendSpanEvent,
  parseBackendSpanStatus,
  type BackendSpanError,
  type BackendSpanEvent,
  type BackendSpanStatus,
} from "../contracts/backend-span-event";

describe("BackendSpanEvent contract", () => {
  it("keeps the TypeScript status union aligned with the canonical Rust fixture", () => {
    // Risk: status taxonomy drift. Level: unit/typecheck. Source: proposal
    // test-intent "TypeScript DTO and parser parity".
    const expected = ["started", "completed", "failed"] satisfies BackendSpanStatus[];
    const selected: BackendSpanStatus = "completed";
    // @ts-expect-error "unknown" is outside the documented BackendSpanStatus union.
    const rejected: BackendSpanStatus = "unknown";

    expect(BACKEND_SPAN_STATUSES).toEqual(expected);
    expect(BACKEND_SPAN_STATUSES).toEqual(backendSpanStatuses);
    expect(selected).toBe("completed");
    expect(rejected).toBe("unknown");

    for (const status of backendSpanStatuses) {
      expect(parseBackendSpanStatus(status)).toBe(status);
    }
    expect(() => parseBackendSpanStatus("unknown")).toThrow("Invalid BackendSpanStatus");
  });

  it("round-trips started, completed, and failed BackendSpanEvent fixture shapes", () => {
    // Risk: TypeScript DTO drifts from the Rust serde shape. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript DTO and parser
    // parity".
    const started = parseBackendSpanEvent(startedHappyPath.expected);
    const completed = parseBackendSpanEvent(completedHappyPath.expected);
    const failed = parseBackendSpanEvent(failedHappyPath.expected);

    expect(started).toEqual(startedHappyPath.expected);
    expect(completed).toEqual(completedHappyPath.expected);
    expect(failed).toEqual(failedHappyPath.expected);

    const typedStarted: BackendSpanEvent = started;
    const typedCompleted: BackendSpanEvent = completed;
    const typedFailed: BackendSpanEvent = failed;

    expect(typedStarted.completed_at).toBeUndefined();
    expect(typedCompleted.completed_at).toBe("1000000001");
    expect(typedFailed.error_ref).toBe("error:runtime-bootstrap");
  });

  it("parses BackendSpanEvent payloads through the generic runtime IpcEvent parser", () => {
    // Risk: runtime-topic envelope drift. Level: unit/typecheck. Source:
    // proposal test-intent "TypeScript DTO and parser parity".
    const envelope = parseIpcEvent(runtimeOnlyTopicEvent, parseBackendSpanEvent);
    const typedEnvelope: IpcEvent<BackendSpanEvent> = envelope;

    expect(envelope).toEqual(runtimeOnlyTopicEvent);
    expect(typedEnvelope.topic).toBe("runtime");
    expect(typedEnvelope.payload.status).toBe("completed");
  });

  it("rejects invalid BackendSpanEvent fixture shapes", () => {
    // Risk: TypeScript accepts values Rust rejects. Level: unit. Source:
    // proposal test-intent "TypeScript DTO and parser parity".
    for (const invalidEvent of invalidEvents) {
      expect(() => parseBackendSpanEvent(invalidEvent.value), invalidEvent.name).toThrow(
        "Invalid BackendSpanEvent",
      );
    }
  });

  it("keeps BackendSpanError variants aligned with fixtures and documented inputs", () => {
    // Risk: TypeScript error taxonomy drifts from Rust. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript DTO and parser
    // parity".
    const expected = [
      "EmptySpanName",
      "CompletedBeforeStarted",
      "FailedWithoutErrorRef",
      "TraceContextInvalid",
    ] satisfies BackendSpanError[];
    const selected: BackendSpanError = "TraceContextInvalid";
    // @ts-expect-error "OtherError" is outside the documented error union.
    const rejected: BackendSpanError = "OtherError";

    expect(BACKEND_SPAN_ERRORS).toEqual(expected);
    expect(BACKEND_SPAN_ERRORS).toEqual(backendSpanErrors);
    expect(selected).toBe("TraceContextInvalid");
    expect(rejected).toBe("OtherError");

    for (const errorCase of [
      errorEmptySpanName,
      errorCompletedBeforeStarted,
      errorFailedWithoutErrorRef,
      errorTraceContextInvalid,
    ]) {
      expect(parseBackendSpanError(errorCase.expected_error)).toBe(errorCase.expected_error);
    }
    expect(errorTraceContextInvalid.delegated_trace_context_error).toBe("EmptyWorkspaceId");
    expect(() => parseBackendSpanError("OtherError")).toThrow("Invalid BackendSpanError");
  });
});
