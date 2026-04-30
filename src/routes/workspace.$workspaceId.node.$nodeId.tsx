import { createRoute, type AnyRoute } from "@tanstack/react-router";

import { parseWorkspaceRouteSearch } from "../contracts/workspace-route-shell";
import { WorkspaceShell } from "../shell/workspace-shell";

export function createWorkspaceNodeRoute(parentRoute: AnyRoute) {
  return createRoute({
    getParentRoute: () => parentRoute,
    path: "/workspace/$workspaceId/node/$nodeId",
    validateSearch: parseWorkspaceRouteSearch,
    component: WorkspaceShell,
  });
}
