export const TRACE_ACTORS = [
  "user",
  "orchestrator",
  "worker",
  "optimizer",
  "reviewer",
  "backend",
  "system",
] as const;

export type TraceActor = (typeof TRACE_ACTORS)[number];

export interface TraceContext {
  correlation_id: string;
  workspace_id: string;
  invocation_id?: string;
  parent_invocation_id?: string;
  actor: TraceActor;
  graph_ref?: string;
  audit_event_ref?: string;
}

export const TRACE_CONTEXT_ERRORS = [
  "EmptyWorkspaceId",
  "UnknownActor",
  "ParentWithoutInvocation",
] as const;

export type TraceContextError = (typeof TRACE_CONTEXT_ERRORS)[number];

export function parseTraceContext(value: unknown): TraceContext {
  if (!isTraceContext(value)) {
    throw new Error("Invalid TraceContext");
  }

  return value;
}

export function parseTraceActor(value: unknown): TraceActor {
  if (typeof value !== "string" || !isTraceActor(value)) {
    throw new Error("Invalid TraceActor");
  }

  return value;
}

export function parseTraceContextError(value: unknown): TraceContextError {
  if (typeof value !== "string" || !isTraceContextError(value)) {
    throw new Error("Invalid TraceContextError");
  }

  return value;
}

function isTraceContext(value: unknown): value is TraceContext {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  const allowedKeys = ["actor", "correlation_id", "workspace_id"];

  for (const optionalKey of [
    "audit_event_ref",
    "graph_ref",
    "invocation_id",
    "parent_invocation_id",
  ]) {
    if (typeof record[optionalKey] !== "undefined") {
      allowedKeys.push(optionalKey);
    }
  }
  allowedKeys.sort();

  if (keys.length !== allowedKeys.length || keys.some((key, index) => key !== allowedKeys[index])) {
    return false;
  }

  return (
    typeof record.correlation_id === "string" &&
    typeof record.workspace_id === "string" &&
    typeof record.actor === "string" &&
    isTraceActor(record.actor) &&
    (typeof record.invocation_id === "undefined" || typeof record.invocation_id === "string") &&
    (typeof record.parent_invocation_id === "undefined" ||
      typeof record.parent_invocation_id === "string") &&
    (typeof record.graph_ref === "undefined" || typeof record.graph_ref === "string") &&
    (typeof record.audit_event_ref === "undefined" ||
      typeof record.audit_event_ref === "string")
  );
}

function isTraceActor(value: string): value is TraceActor {
  return (TRACE_ACTORS as readonly string[]).includes(value);
}

function isTraceContextError(value: string): value is TraceContextError {
  return (TRACE_CONTEXT_ERRORS as readonly string[]).includes(value);
}
