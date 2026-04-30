import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import paneIds from "../../product-strategy/contracts/fixtures/wu-0a-11/pane-ids.json";
import routeFixtures from "../../product-strategy/contracts/fixtures/wu-0a-13/route-fixtures.json";
import roundTripStates from "../../product-strategy/contracts/fixtures/wu-0a-13/shell-region-state-round-trips.json";
import sideEffectAbsence from "../../product-strategy/contracts/fixtures/wu-0a-13/side-effect-absence.json";
import { AppShell } from "../App";
import { parseShellRegionState, type ShellRegionStateParseResult } from "../contracts/shell-region-state";
import { type ShellRegionState } from "../shell/shell-region-state";

const sideEffectSpies = vi.hoisted(() => ({
  invoke: vi.fn(),
  readFile: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: sideEffectSpies.invoke,
}));

vi.mock("node:fs/promises", () => ({
  readFile: sideEffectSpies.readFile,
}));

interface RouteFixture {
  name: string;
  path: string;
  expectedState: ShellRegionState;
}

function renderPath(path: string): void {
  window.history.pushState({}, "Workspace route shell test", path);
  render(<AppShell />);
}

async function readShellState(): Promise<ShellRegionState> {
  const stateElement = await screen.findByTestId("shell-region-state-json");
  const rawState = JSON.parse(stateElement.textContent ?? "null") as unknown;
  const parsed = parseShellRegionState(rawState);

  expect(parsed).toEqual({
    ok: true,
    value: rawState as ShellRegionState,
  } satisfies ShellRegionStateParseResult);

  if (!parsed.ok) {
    throw new Error("expected valid ShellRegionState");
  }

  return parsed.value;
}

describe("Workspace route shell", () => {
  beforeEach(() => {
    window.history.pushState({}, "Workspace route shell test", "/");
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("renders /workspace/:workspaceId as exactly one shell with copied workspaceId", async () => {
    // Risk: route nesting duplicates shells or loses params. Level:
    // component. Source: proposal test-intent "Route render invariant".
    const fixture = (routeFixtures as RouteFixture[]).find(
      (candidate) => candidate.name === "workspace route default pane",
    );
    expect(fixture).toBeDefined();

    renderPath(fixture!.path);

    expect(await screen.findAllByTestId("workspace-shell")).toHaveLength(1);
    expect(await screen.findByTestId("workspace-id")).toHaveTextContent(
      fixture!.expectedState.workspaceId,
    );
    expect(await readShellState()).toEqual(fixture!.expectedState);
  });

  it("renders /workspace/:workspaceId/node/:nodeId as exactly one shell with copied selectedNodeId", async () => {
    // Risk: route nesting duplicates shells or loses params. Level:
    // component. Source: proposal test-intent "Route render invariant".
    const fixture = (routeFixtures as RouteFixture[]).find(
      (candidate) => candidate.name === "workspace node route default pane",
    );
    expect(fixture).toBeDefined();

    renderPath(fixture!.path);

    expect(await screen.findAllByTestId("workspace-shell")).toHaveLength(1);
    expect(await screen.findByTestId("selected-node-id")).toHaveTextContent(
      fixture!.expectedState.selectedNodeId!,
    );
    expect(await readShellState()).toEqual(fixture!.expectedState);
  });

  it.each(paneIds as ShellRegionState["activePane"][])(
    "maps ?pane=%s to ShellRegionState.activePane",
    async (paneId) => {
      // Risk: route search accepts only a subset of panes. Level: component.
      // Source: proposal test-intent "Pane query coverage".
      renderPath(`/workspace/workspace-alpha?pane=${paneId}`);

      expect(await screen.findAllByTestId("workspace-shell")).toHaveLength(1);
      expect(await screen.findByTestId("active-pane")).toHaveTextContent(paneId);
      expect(await readShellState()).toMatchObject({
        workspaceId: "workspace-alpha",
        activePane: paneId,
      } satisfies Partial<ShellRegionState>);
    },
  );

  it("falls unknown pane queries back to runtimeStatus without workspace mutation", async () => {
    // Risk: bad search state leaks into the shell. Level: component. Source:
    // proposal test-intent "Unknown-pane fallback".
    const fixture = (routeFixtures as RouteFixture[]).find(
      (candidate) => candidate.name === "unknown pane query falls back",
    );
    expect(fixture).toBeDefined();

    renderPath(fixture!.path);

    const state = await readShellState();
    expect(state).toEqual(fixture!.expectedState);
    expect(await screen.findByTestId("active-pane")).toHaveTextContent("runtimeStatus");
    expect(await screen.findByTestId("workspace-id")).toHaveTextContent("workspace-alpha");
  });

  it("round-trips all WU-0A-13 ShellRegionState fixtures through the WU-0A-12 parser", () => {
    // Risk: route fixtures drift from ShellRegionState. Level: component
    // fixture. Source: proposal test-intent "Route render invariant".
    expect((routeFixtures as RouteFixture[]).map((fixture) => fixture.expectedState)).toEqual(
      roundTripStates,
    );

    for (const state of roundTripStates) {
      expect(parseShellRegionState(state), JSON.stringify(state)).toEqual({
        ok: true,
        value: state as ShellRegionState,
      } satisfies ShellRegionStateParseResult);
    }
  });

  it("does not invoke agents, provider, GraphStore, migration, or runtime IPC on render", async () => {
    // Risk: route render accidentally enters Phase 0B+ domains. Level:
    // component/static boundary. Source: proposal test-intent
    // "Side-effect absence".
    renderPath("/workspace/workspace-alpha?pane=providerPanel");

    expect(await screen.findAllByTestId("workspace-shell")).toHaveLength(1);
    expect(sideEffectAbsence.phase0aRegisteredCommandsRemain).toEqual([
      "subscribe_workspace_events",
    ]);
    expect(sideEffectSpies.invoke).not.toHaveBeenCalled();
    expect(sideEffectSpies.readFile).not.toHaveBeenCalled();
  });
});
