import { parseTraceContext, type TraceContext } from "./trace-context";

export const BACKEND_SPAN_STATUSES = ["started", "completed", "failed"] as const;

export type BackendSpanStatus = (typeof BACKEND_SPAN_STATUSES)[number];

export interface BackendSpanEvent {
  span_event_id: string;
  trace_context: TraceContext;
  span_name: string;
  started_at: string;
  completed_at?: string;
  status: BackendSpanStatus;
  error_ref?: string;
}

export const BACKEND_SPAN_ERRORS = [
  "EmptySpanName",
  "CompletedBeforeStarted",
  "FailedWithoutErrorRef",
  "TraceContextInvalid",
] as const;

export type BackendSpanError = (typeof BACKEND_SPAN_ERRORS)[number];

export function parseBackendSpanEvent(value: unknown): BackendSpanEvent {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Invalid BackendSpanEvent");
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  const allowedKeys = [
    "span_event_id",
    "span_name",
    "started_at",
    "status",
    "trace_context",
  ];

  if (typeof record.completed_at !== "undefined") {
    allowedKeys.push("completed_at");
  }
  if (typeof record.error_ref !== "undefined") {
    allowedKeys.push("error_ref");
  }
  allowedKeys.sort();

  if (keys.length !== allowedKeys.length || keys.some((key, index) => key !== allowedKeys[index])) {
    throw new Error("Invalid BackendSpanEvent");
  }

  if (
    typeof record.span_event_id !== "string" ||
    record.span_event_id.length === 0 ||
    typeof record.span_name !== "string" ||
    typeof record.started_at !== "string" ||
    typeof record.status !== "string" ||
    !isBackendSpanStatus(record.status) ||
    (typeof record.completed_at !== "undefined" && typeof record.completed_at !== "string") ||
    (typeof record.error_ref !== "undefined" && typeof record.error_ref !== "string")
  ) {
    throw new Error("Invalid BackendSpanEvent");
  }

  let traceContext: TraceContext;
  try {
    traceContext = parseTraceContext(record.trace_context);
  } catch {
    throw new Error("Invalid BackendSpanEvent");
  }

  return {
    span_event_id: record.span_event_id,
    trace_context: traceContext,
    span_name: record.span_name,
    started_at: record.started_at,
    ...(typeof record.completed_at === "string" ? { completed_at: record.completed_at } : {}),
    status: record.status,
    ...(typeof record.error_ref === "string" ? { error_ref: record.error_ref } : {}),
  };
}

export function parseBackendSpanStatus(value: unknown): BackendSpanStatus {
  if (typeof value !== "string" || !isBackendSpanStatus(value)) {
    throw new Error("Invalid BackendSpanStatus");
  }

  return value;
}

export function parseBackendSpanError(value: unknown): BackendSpanError {
  if (typeof value !== "string" || !isBackendSpanError(value)) {
    throw new Error("Invalid BackendSpanError");
  }

  return value;
}

function isBackendSpanStatus(value: string): value is BackendSpanStatus {
  return (BACKEND_SPAN_STATUSES as readonly string[]).includes(value);
}

function isBackendSpanError(value: string): value is BackendSpanError {
  return (BACKEND_SPAN_ERRORS as readonly string[]).includes(value);
}
