import { parsePaneId, type PaneId } from "./pane-id";
import { parseShellRegionState } from "./shell-region-state";
import { DEFAULT_PHASE_0A_PANE_ID } from "../shell/pane-id";
import { type ShellRegionState } from "../shell/shell-region-state";

export interface WorkspaceRouteSearch {
  pane?: string;
}

export interface WorkspaceShellRouteInput {
  workspaceId: string;
  nodeId?: string;
  pane?: unknown;
}

export function parseWorkspaceRouteSearch(search: Record<string, unknown>): WorkspaceRouteSearch {
  return typeof search.pane === "string" ? { pane: search.pane } : {};
}

export function deriveWorkspaceShellRegionState(
  input: WorkspaceShellRouteInput,
): ShellRegionState {
  const activePane = deriveActivePane(input.pane);
  const state: ShellRegionState = {
    workspaceId: input.workspaceId,
    activePane,
    ...(typeof input.nodeId === "string" ? { selectedNodeId: input.nodeId } : {}),
    actionNeededCount: 0,
    passiveProgressCount: 0,
    runtimeConnected: false,
  };

  const parsed = parseShellRegionState(state);
  if (!parsed.ok) {
    throw new Error(`Invalid workspace shell route state: ${parsed.error.kind}`);
  }

  return parsed.value;
}

function deriveActivePane(rawPane: unknown): PaneId {
  if (typeof rawPane !== "string") {
    return DEFAULT_PHASE_0A_PANE_ID;
  }

  const parsed = parsePaneId(rawPane);
  return parsed.ok ? parsed.value : DEFAULT_PHASE_0A_PANE_ID;
}
