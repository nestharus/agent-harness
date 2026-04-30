import { describe, expect, it } from "vitest";

import canonicalContext from "../../product-strategy/contracts/fixtures/wu-0a-09/canonical-context.json";
import createErrors from "../../product-strategy/contracts/fixtures/wu-0a-09/create-errors.json";
import invalidContexts from "../../product-strategy/contracts/fixtures/wu-0a-09/invalid-contexts.json";
import minimalContext from "../../product-strategy/contracts/fixtures/wu-0a-09/minimal-context.json";
import traceActors from "../../product-strategy/contracts/fixtures/wu-0a-09/trace-actors.json";
import traceContextErrors from "../../product-strategy/contracts/fixtures/wu-0a-09/trace-context-errors.json";
import {
  TRACE_ACTORS,
  TRACE_CONTEXT_ERRORS,
  parseTraceContext,
  parseTraceContextError,
  type TraceActor,
  type TraceContext,
  type TraceContextError,
} from "../contracts/trace-context";

describe("TraceContext contract", () => {
  it("keeps the TypeScript actor union aligned with the canonical Rust fixture", () => {
    // Risk: TypeScript accepts actor strings Rust rejects. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript DTO and error
    // parity".
    const expected = [
      "user",
      "orchestrator",
      "worker",
      "optimizer",
      "reviewer",
      "backend",
      "system",
    ] satisfies TraceActor[];
    const selected: TraceActor = "worker";
    // @ts-expect-error "unknown" is outside the documented TraceActor union.
    const rejected: TraceActor = "unknown";

    expect(TRACE_ACTORS).toEqual(expected);
    expect(TRACE_ACTORS).toEqual(traceActors);
    expect(selected).toBe("worker");
    expect(rejected).toBe("unknown");
  });

  it("round-trips full and minimal TraceContext fixture shapes", () => {
    // Risk: TypeScript DTO drifts from the Rust serde shape. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript DTO and error
    // parity".
    const canonical = parseTraceContext(canonicalContext);
    const minimal = parseTraceContext(minimalContext);

    expect(canonical).toEqual(canonicalContext);
    expect(minimal).toEqual(minimalContext);

    const typedCanonical: TraceContext = canonical;
    const typedMinimal: TraceContext = minimal;
    expect(typedCanonical.audit_event_ref).toBe("audit:event:99");
    expect(typedMinimal.invocation_id).toBeUndefined();
  });

  it("rejects invalid TraceContext fixture shapes", () => {
    // Risk: TypeScript accepts values Rust rejects. Level: unit. Source:
    // proposal test-intent "TypeScript DTO and error parity".
    for (const invalidContext of invalidContexts) {
      expect(() => parseTraceContext(invalidContext.value), invalidContext.name).toThrow(
        "Invalid TraceContext",
      );
    }
  });

  it("keeps TraceContextError variants aligned with fixtures and create errors", () => {
    // Risk: TypeScript error taxonomy drifts from Rust. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript DTO and error
    // parity".
    const expected = [
      "EmptyWorkspaceId",
      "UnknownActor",
      "ParentWithoutInvocation",
    ] satisfies TraceContextError[];
    const selected: TraceContextError = "UnknownActor";
    // @ts-expect-error "OtherError" is outside the documented error union.
    const rejected: TraceContextError = "OtherError";

    expect(TRACE_CONTEXT_ERRORS).toEqual(expected);
    expect(TRACE_CONTEXT_ERRORS).toEqual(traceContextErrors);
    expect(selected).toBe("UnknownActor");
    expect(rejected).toBe("OtherError");

    for (const errorCase of createErrors) {
      expect(parseTraceContextError(errorCase.expected_error)).toBe(errorCase.expected_error);
    }
    expect(() => parseTraceContextError("OtherError")).toThrow("Invalid TraceContextError");
  });
});
