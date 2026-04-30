import { describe, expect, it } from "vitest";

import buildErrors from "../../product-strategy/contracts/fixtures/wu-0a-06/build-errors.json";
import canonicalEvent from "../../product-strategy/contracts/fixtures/wu-0a-06/canonical-event.json";
import invalidEvents from "../../product-strategy/contracts/fixtures/wu-0a-06/invalid-events.json";
import ipcEventErrors from "../../product-strategy/contracts/fixtures/wu-0a-06/ipc-event-errors.json";
import minimalEvent from "../../product-strategy/contracts/fixtures/wu-0a-06/minimal-event.json";
import {
  IPC_EVENT_ERRORS,
  parseIpcEvent,
  parseIpcEventError,
  type IpcEvent,
  type IpcEventError,
} from "../contracts/ipc-event";
import type { EventTopic } from "../contracts/event-topic";

interface RuntimePingPayload {
  kind: string;
  sequence?: number;
  ok?: boolean;
}

function parseRuntimePingPayload(value: unknown): RuntimePingPayload {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Invalid RuntimePingPayload");
  }

  const record = value as Record<string, unknown>;

  if (
    typeof record.kind !== "string" ||
    (typeof record.sequence !== "undefined" && typeof record.sequence !== "number") ||
    (typeof record.ok !== "undefined" && typeof record.ok !== "boolean")
  ) {
    throw new Error("Invalid RuntimePingPayload");
  }

  return {
    kind: record.kind,
    ...(typeof record.sequence === "number" ? { sequence: record.sequence } : {}),
    ...(typeof record.ok === "boolean" ? { ok: record.ok } : {}),
  };
}

describe("IpcEvent contract", () => {
  it("round-trips full and minimal generic IpcEvent fixture shapes", () => {
    // Risk: TypeScript accepts values Rust rejects or erases payload shape.
    // Level: unit/typecheck. Source: proposal test-intent "TypeScript DTO and
    // parser parity".
    const canonical = parseIpcEvent(canonicalEvent, parseRuntimePingPayload);
    const minimal = parseIpcEvent(minimalEvent, parseRuntimePingPayload);

    expect(canonical).toEqual(canonicalEvent);
    expect(minimal).toEqual(minimalEvent);

    const typedCanonical: IpcEvent<RuntimePingPayload> = canonical;
    const typedMinimal: IpcEvent<RuntimePingPayload> = minimal;
    const topic: EventTopic = typedCanonical.topic;

    expect(typedCanonical.trace_context_id).toBe("trace-contract-001");
    expect(typedCanonical.payload.sequence).toBe(7);
    expect(typedMinimal.trace_context_id).toBeUndefined();
    expect(topic).toBe("runtime");
  });

  it("enforces generic payload and optional trace_context_id at type level", () => {
    // Risk: TypeScript accepts values Rust rejects or erases payload shape.
    // Level: unit/typecheck. Source: proposal test-intent "TypeScript DTO and
    // parser parity".
    const accepted: IpcEvent<RuntimePingPayload> = {
      event_id: "event-contract-typecheck",
      workspace_id: "workspace-alpha",
      topic: "runtime",
      payload: { kind: "runtime_ping" },
      created_at: "2026-04-29T00:00:00.000Z",
    };
    const acceptedWithTrace: IpcEvent<RuntimePingPayload> = {
      ...accepted,
      trace_context_id: "trace-contract-typecheck",
    };
    // @ts-expect-error payload.kind must satisfy the generic payload shape.
    const rejectedPayload: IpcEvent<RuntimePingPayload> = { ...accepted, payload: { kind: 42 } };
    // @ts-expect-error "unknown" is outside the documented EventTopic union.
    const rejectedTopic: IpcEvent<RuntimePingPayload> = { ...accepted, topic: "unknown" };

    expect(accepted.trace_context_id).toBeUndefined();
    expect(acceptedWithTrace.trace_context_id).toBe("trace-contract-typecheck");
    expect(rejectedPayload.payload).toEqual({ kind: 42 });
    expect(rejectedTopic.topic).toBe("unknown");
  });

  it("rejects invalid IpcEvent fixture shapes", () => {
    // Risk: TypeScript accepts values Rust rejects. Level: unit. Source:
    // proposal test-intent "TypeScript DTO and parser parity".
    for (const invalidEvent of invalidEvents) {
      expect(() => parseIpcEvent(invalidEvent.value, parseRuntimePingPayload), invalidEvent.name)
        .toThrow("Invalid IpcEvent");
    }
  });

  it("keeps IpcEventError variants aligned with fixtures and build errors", () => {
    // Risk: TypeScript error taxonomy drifts from Rust. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript DTO and parser
    // parity".
    const expected = [
      "EmptyEventId",
      "EmptyWorkspaceId",
      "UnknownTopic",
      "PayloadSerializationFailed",
    ] satisfies IpcEventError[];
    const selected: IpcEventError = "PayloadSerializationFailed";
    // @ts-expect-error "OtherError" is outside the documented error union.
    const rejected: IpcEventError = "OtherError";

    expect(IPC_EVENT_ERRORS).toEqual(expected);
    expect(IPC_EVENT_ERRORS).toEqual(ipcEventErrors);
    expect(selected).toBe("PayloadSerializationFailed");
    expect(rejected).toBe("OtherError");

    for (const errorCase of buildErrors) {
      expect(parseIpcEventError(errorCase.expected_error)).toBe(errorCase.expected_error);
    }
    expect(() => parseIpcEventError("OtherError")).toThrow("Invalid IpcEventError");
  });
});
