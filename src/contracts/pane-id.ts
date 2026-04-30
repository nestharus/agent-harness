export const PANE_IDS = [
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
] as const;

export type PaneId = (typeof PANE_IDS)[number];

export const PANE_ID_ERRORS = ["EmptyPaneId", "UnknownPaneId"] as const;

export type PaneIdError = (typeof PANE_ID_ERRORS)[number];

export interface PaneIdParseFailure {
  kind: PaneIdError;
  raw: string;
}

export type PaneIdParseResult =
  | {
      ok: true;
      value: PaneId;
    }
  | {
      ok: false;
      error: PaneIdParseFailure;
    };

export function parsePaneId(raw: string): PaneIdParseResult {
  if (raw === "") {
    return {
      ok: false,
      error: {
        kind: "EmptyPaneId",
        raw,
      },
    };
  }

  if (isPaneId(raw)) {
    return {
      ok: true,
      value: raw,
    };
  }

  return {
    ok: false,
    error: {
      kind: "UnknownPaneId",
      raw,
    },
  };
}

export function isPaneId(value: string): value is PaneId {
  return (PANE_IDS as readonly string[]).includes(value);
}
