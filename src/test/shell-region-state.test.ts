import { describe, expect, it } from "vitest";

import paneIds from "../../product-strategy/contracts/fixtures/wu-0a-11/pane-ids.json";
import canonicalStates from "../../product-strategy/contracts/fixtures/wu-0a-12/canonical-states.json";
import parseErrors from "../../product-strategy/contracts/fixtures/wu-0a-12/parse-errors.json";
import shellRegionStateErrors from "../../product-strategy/contracts/fixtures/wu-0a-12/shell-region-state-errors.json";
import {
  SHELL_REGION_STATE_ERRORS,
  parseShellRegionState,
  type ShellRegionStateError,
  type ShellRegionStateParseResult,
} from "../contracts/shell-region-state";
import { type ShellRegionState } from "../shell/shell-region-state";

describe("ShellRegionState contract", () => {
  it("round-trips canonical ShellRegionState fixtures and enforces the DTO type", () => {
    // Risk: TypeScript DTO drifts from the WU contract. Level:
    // unit/typecheck. Source: proposal test-intent "DTO fixture parity".
    const selected: ShellRegionState = {
      workspaceId: "workspace-alpha",
      activePane: "runtimeStatus",
      selectedNodeId: "node-runtime",
      actionNeededCount: 1,
      passiveProgressCount: 2,
      runtimeConnected: true,
    };
    // @ts-expect-error "unknown" is outside the documented PaneId union.
    const rejectedPane: ShellRegionState = { ...selected, activePane: "unknown" };
    const rejectedRuntimeConnected: ShellRegionState = {
      ...selected,
      // @ts-expect-error runtimeConnected must remain a boolean.
      runtimeConnected: "true",
    };

    for (const state of canonicalStates) {
      const typedState = state as ShellRegionState;

      expect(parseShellRegionState(state)).toEqual({
        ok: true,
        value: typedState,
      } satisfies ShellRegionStateParseResult);
    }
    expect(selected.selectedNodeId).toBe("node-runtime");
    expect(rejectedPane.activePane).toBe("unknown");
    expect(rejectedRuntimeConnected.runtimeConnected).toBe("true");
  });

  it("keeps canonical ShellRegionState fixtures aligned with the PaneId fixture order", () => {
    // Risk: shell state rejects a pane documented by WU-0A-11. Level: unit.
    // Source: proposal test-intent "Pane variant coverage".
    expect(canonicalStates.map((state) => state.activePane)).toEqual(paneIds);
  });

  it.each(paneIds as ShellRegionState["activePane"][])(
    "accepts %s as the active pane",
    (paneId) => {
      // Risk: shell state rejects a pane documented by WU-0A-11. Level: unit.
      // Source: proposal test-intent "Pane variant coverage".
      const state: ShellRegionState = {
        workspaceId: "workspace-alpha",
        activePane: paneId,
        actionNeededCount: 0,
        passiveProgressCount: 0,
        runtimeConnected: true,
      };

      expect(parseShellRegionState(state)).toEqual({
        ok: true,
        value: state,
      } satisfies ShellRegionStateParseResult);
    },
  );

  it("keeps ShellRegionStateError variants aligned with fixtures", () => {
    // Risk: invalid shell state silently enters the shell view model. Level:
    // unit. Source: proposal test-intent "Parser error behavior".
    const expected = [
      "EmptyWorkspaceId",
      "UnknownPaneId",
      "NegativeActionNeededCount",
      "NegativePassiveProgressCount",
      "InvalidRuntimeConnected",
    ] satisfies ShellRegionStateError[];
    const selected: ShellRegionStateError = "UnknownPaneId";
    // @ts-expect-error "OtherError" is outside the documented error union.
    const rejected: ShellRegionStateError = "OtherError";

    expect(SHELL_REGION_STATE_ERRORS).toEqual(expected);
    expect(SHELL_REGION_STATE_ERRORS).toEqual(shellRegionStateErrors);
    expect(selected).toBe("UnknownPaneId");
    expect(rejected).toBe("OtherError");
  });

  it.each(parseErrors)("returns $result.error.kind for $name", (parseCase) => {
    // Risk: invalid shell state silently enters the shell view model. Level:
    // unit. Source: proposal test-intent "Parser error behavior".
    expect(parseShellRegionState(parseCase.input), parseCase.name).toEqual(
      parseCase.result as ShellRegionStateParseResult,
    );
  });

  it("documents parser purity as an import-boundary assumption", () => {
    // Risk: parser starts causing hidden side effects. Level: unit/static
    // assumption. Source: proposal test-intent "Parser purity assumption".
    // The Phase 0A scaffold has no shell store, router mutation API, or IPC
    // client for this parser to mutate; the contract parser is tested as a
    // pure function over fixture input.
    const input = canonicalStates[0] as ShellRegionState;

    expect(parseShellRegionState(input)).toEqual({
      ok: true,
      value: input,
    } satisfies ShellRegionStateParseResult);
  });
});
