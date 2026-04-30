import { useParams, useSearch } from "@tanstack/react-router";

import {
  deriveWorkspaceShellRegionState,
  type WorkspaceRouteSearch,
} from "../contracts/workspace-route-shell";
import { PANE_IDS } from "../contracts/pane-id";
import { getPhase0aPaneMode } from "./pane-id";

interface WorkspaceRouteParams {
  workspaceId?: string;
  nodeId?: string;
}

const paneLabels: Record<(typeof PANE_IDS)[number], string> = {
  initiativeMap: "Initiative map",
  currentFocus: "Current focus",
  workingSetInspector: "Working set inspector",
  configurationInspector: "Configuration inspector",
  providerPanel: "Provider panel",
  questionQueue: "Question queue",
  workerBoard: "Worker board",
  optimizerLog: "Optimizer log",
  evidenceDrilldown: "Evidence drilldown",
  costSurface: "Cost surface",
  recoverySurface: "Recovery surface",
  runtimeStatus: "Runtime status",
};

export function WorkspaceShell() {
  const params = useParams({ strict: false }) as WorkspaceRouteParams;
  const search = useSearch({ strict: false }) as WorkspaceRouteSearch;

  if (typeof params.workspaceId !== "string" || params.workspaceId.length === 0) {
    throw new Error("WorkspaceShell requires a workspaceId route param");
  }

  const shellState = deriveWorkspaceShellRegionState({
    workspaceId: params.workspaceId,
    nodeId: params.nodeId,
    pane: search.pane,
  });

  return (
    <main
      className="workspace-shell"
      data-testid="workspace-shell"
      data-active-pane={shellState.activePane}
    >
      <section className="workspace-shell__summary" aria-labelledby="workspace-shell-title">
        <p className="shell-kicker">Phase 0A Workspace</p>
        <h1 id="workspace-shell-title">Workspace Shell</h1>
        <dl className="workspace-shell__state-list" aria-label="Shell region state">
          <div>
            <dt>Workspace</dt>
            <dd data-testid="workspace-id">{shellState.workspaceId}</dd>
          </div>
          <div>
            <dt>Selected node</dt>
            <dd data-testid="selected-node-id">{shellState.selectedNodeId ?? "none"}</dd>
          </div>
          <div>
            <dt>Active pane</dt>
            <dd data-testid="active-pane">{shellState.activePane}</dd>
          </div>
          <div>
            <dt>Runtime connected</dt>
            <dd data-testid="runtime-connected">{String(shellState.runtimeConnected)}</dd>
          </div>
        </dl>
        <script type="application/json" data-testid="shell-region-state-json">
          {JSON.stringify(shellState)}
        </script>
      </section>

      <section className="workspace-shell__runtime" aria-labelledby="runtime-status-title">
        <h2 id="runtime-status-title">Runtime status</h2>
        <div className="workspace-shell__runtime-grid">
          <span>Connected</span>
          <strong>{String(shellState.runtimeConnected)}</strong>
          <span>Action needed</span>
          <strong>{shellState.actionNeededCount}</strong>
          <span>Passive progress</span>
          <strong>{shellState.passiveProgressCount}</strong>
        </div>
      </section>

      <section className="workspace-shell__panes" aria-label="Workspace panes">
        {PANE_IDS.map((paneId) => {
          const paneMode = getPhase0aPaneMode(paneId);
          const isActive = paneId === shellState.activePane;

          return (
            <article
              className="workspace-shell__pane"
              data-pane-id={paneId}
              data-pane-mode={paneMode}
              data-active={String(isActive)}
              key={paneId}
            >
              <h2>{paneLabels[paneId]}</h2>
              <p>{paneMode === "liveRuntimeScaffold" ? "Phase 0A runtime state" : "Inert placeholder"}</p>
            </article>
          );
        })}
      </section>
    </main>
  );
}
