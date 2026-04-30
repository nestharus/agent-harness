import { type PaneId } from "../contracts/pane-id";

export type Phase0aPaneMode = "inertPlaceholder" | "liveRuntimeScaffold";

export const DEFAULT_PHASE_0A_PANE_ID = "runtimeStatus" as const satisfies PaneId;

export const PHASE_0A_LIVE_PANE_IDS = ["runtimeStatus"] as const satisfies readonly PaneId[];

export const PHASE_0A_INERT_PANE_IDS = [
  "initiativeMap",
  "currentFocus",
  "workingSetInspector",
  "configurationInspector",
  "providerPanel",
  "questionQueue",
  "workerBoard",
  "optimizerLog",
  "evidenceDrilldown",
  "costSurface",
  "recoverySurface",
] as const satisfies readonly PaneId[];

export const PHASE_0A_PANE_MODES = {
  initiativeMap: "inertPlaceholder",
  currentFocus: "inertPlaceholder",
  workingSetInspector: "inertPlaceholder",
  configurationInspector: "inertPlaceholder",
  providerPanel: "inertPlaceholder",
  questionQueue: "inertPlaceholder",
  workerBoard: "inertPlaceholder",
  optimizerLog: "inertPlaceholder",
  evidenceDrilldown: "inertPlaceholder",
  costSurface: "inertPlaceholder",
  recoverySurface: "inertPlaceholder",
  runtimeStatus: "liveRuntimeScaffold",
} as const satisfies Record<PaneId, Phase0aPaneMode>;

export function getPhase0aPaneMode(paneId: PaneId): Phase0aPaneMode {
  return PHASE_0A_PANE_MODES[paneId];
}

export function isPhase0aLivePane(paneId: PaneId): boolean {
  return getPhase0aPaneMode(paneId) === "liveRuntimeScaffold";
}
