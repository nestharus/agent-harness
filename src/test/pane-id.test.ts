import { describe, expect, it } from "vitest";

import inertPanes from "../../product-strategy/contracts/fixtures/wu-0a-11/inert-panes.json";
import paneIds from "../../product-strategy/contracts/fixtures/wu-0a-11/pane-ids.json";
import parseErrors from "../../product-strategy/contracts/fixtures/wu-0a-11/parse-errors.json";
import parseSuccess from "../../product-strategy/contracts/fixtures/wu-0a-11/parse-success.json";
import phase0aLivePanes from "../../product-strategy/contracts/fixtures/wu-0a-11/phase-0a-live-panes.json";
import {
  PANE_IDS,
  parsePaneId,
  type PaneId,
  type PaneIdParseResult,
} from "../contracts/pane-id";
import {
  PHASE_0A_LIVE_PANE_IDS,
  getPhase0aPaneMode,
  isPhase0aLivePane,
} from "../shell/pane-id";

describe("PaneId contract", () => {
  it("keeps the TypeScript union aligned with the canonical pane taxonomy", () => {
    // Risk: TypeScript accepts pane strings outside the WU contract. Level:
    // unit/typecheck. Source: proposal test-intent "TypeScript taxonomy parity".
    const expected = [
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
      "runtimeStatus",
    ] satisfies PaneId[];
    const selected: PaneId = "runtimeStatus";
    // @ts-expect-error "unknown" is outside the documented PaneId union.
    const rejected: PaneId = "unknown";

    expect(PANE_IDS).toEqual(expected);
    expect(PANE_IDS).toEqual(paneIds);
    expect(selected).toBe("runtimeStatus");
    expect(rejected).toBe("unknown");
  });

  it("parses every documented pane id from the shared fixture", () => {
    // Risk: documented shell pane identifiers cannot be parsed by callers.
    // Level: unit. Source: proposal test-intent "Parser success behavior".
    for (const parseCase of parseSuccess) {
      expect(parsePaneId(parseCase.raw)).toEqual(parseCase.result as PaneIdParseResult);
    }
  });

  it("returns the documented error result shape for empty and unknown pane ids", () => {
    // Risk: unknown pane identifiers silently enter shell state. Level: unit.
    // Source: proposal test-intent "Parser error behavior".
    for (const parseCase of parseErrors) {
      expect(parsePaneId(parseCase.raw)).toEqual(parseCase.result as PaneIdParseResult);
    }
  });

  it("keeps runtimeStatus live and every other Phase 0A pane inert", () => {
    // Risk: Phase 0A UI claims unseeded domain data. Level: unit. Source:
    // proposal test-intent "Phase 0A live/inert gating".
    expect(PHASE_0A_LIVE_PANE_IDS).toEqual(phase0aLivePanes);
    expect(PHASE_0A_LIVE_PANE_IDS).toEqual(["runtimeStatus"]);

    for (const paneId of inertPanes) {
      expect(PANE_IDS).toContain(paneId);
      expect(getPhase0aPaneMode(paneId as PaneId)).toBe("inertPlaceholder");
      expect(isPhase0aLivePane(paneId as PaneId)).toBe(false);
    }

    expect(getPhase0aPaneMode("runtimeStatus")).toBe("liveRuntimeScaffold");
    expect(isPhase0aLivePane("runtimeStatus")).toBe(true);
  });
});
