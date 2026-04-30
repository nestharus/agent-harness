import { type PaneId } from "../contracts/pane-id";

export interface ShellRegionState {
  workspaceId: string;
  activePane: PaneId;
  selectedNodeId?: string;
  actionNeededCount: number;
  passiveProgressCount: number;
  runtimeConnected: boolean;
}
