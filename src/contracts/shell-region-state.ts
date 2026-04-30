import { parsePaneId } from "./pane-id";
import { type ShellRegionState } from "../shell/shell-region-state";

export const SHELL_REGION_STATE_ERRORS = [
  "EmptyWorkspaceId",
  "UnknownPaneId",
  "NegativeActionNeededCount",
  "NegativePassiveProgressCount",
  "InvalidRuntimeConnected",
] as const;

export type ShellRegionStateError = (typeof SHELL_REGION_STATE_ERRORS)[number];

export interface ShellRegionStateParseFailure {
  kind: ShellRegionStateError;
}

export type ShellRegionStateParseResult =
  | {
      ok: true;
      value: ShellRegionState;
    }
  | {
      ok: false;
      error: ShellRegionStateParseFailure;
    };

export function parseShellRegionState(input: unknown): ShellRegionStateParseResult {
  if (typeof input !== "object" || input === null || Array.isArray(input)) {
    return failure("EmptyWorkspaceId");
  }

  const record = input as Record<string, unknown>;

  if (typeof record.workspaceId !== "string" || record.workspaceId.length === 0) {
    return failure("EmptyWorkspaceId");
  }

  if (typeof record.activePane !== "string") {
    return failure("UnknownPaneId");
  }

  const activePane = parsePaneId(record.activePane);
  if (!activePane.ok) {
    return failure("UnknownPaneId");
  }

  if (
    typeof record.actionNeededCount !== "number" ||
    !Number.isFinite(record.actionNeededCount) ||
    record.actionNeededCount < 0
  ) {
    return failure("NegativeActionNeededCount");
  }

  if (
    typeof record.passiveProgressCount !== "number" ||
    !Number.isFinite(record.passiveProgressCount) ||
    record.passiveProgressCount < 0
  ) {
    return failure("NegativePassiveProgressCount");
  }

  if (typeof record.runtimeConnected !== "boolean") {
    return failure("InvalidRuntimeConnected");
  }

  if (
    typeof record.selectedNodeId !== "undefined" &&
    typeof record.selectedNodeId !== "string"
  ) {
    return failure("UnknownPaneId");
  }

  return {
    ok: true,
    value: {
      workspaceId: record.workspaceId,
      activePane: activePane.value,
      ...(typeof record.selectedNodeId === "string"
        ? { selectedNodeId: record.selectedNodeId }
        : {}),
      actionNeededCount: record.actionNeededCount,
      passiveProgressCount: record.passiveProgressCount,
      runtimeConnected: record.runtimeConnected,
    },
  };
}

function failure(kind: ShellRegionStateError): ShellRegionStateParseResult {
  return {
    ok: false,
    error: {
      kind,
    },
  };
}
