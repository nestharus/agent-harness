import { DEFAULT_PHASE_0A_PANE_ID, getPhase0aPaneMode } from "./shell/pane-id";

export function ShellRoot() {
  const activePaneId = DEFAULT_PHASE_0A_PANE_ID;
  const activePaneMode = getPhase0aPaneMode(activePaneId);

  return (
    <main className="shell-root" data-testid="app-root">
      <section
        className="shell-panel"
        aria-labelledby="shell-title"
        data-pane-id={activePaneId}
        data-pane-mode={activePaneMode}
      >
        <p className="shell-kicker">Phase 0A</p>
        <h1 id="shell-title">Agent Harness</h1>
        <p className="shell-copy">
          Repository and runtime scaffold for local desktop startup checks.
        </p>
      </section>
    </main>
  );
}
