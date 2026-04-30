import { createRoute, type AnyRoute } from "@tanstack/react-router";

import { parseWorkspaceRouteSearch } from "../contracts/workspace-route-shell";
import { WorkspaceShell } from "../shell/workspace-shell";

export function createWorkspaceRoute(parentRoute: AnyRoute) {
  return createRoute({
    getParentRoute: () => parentRoute,
    path: "/workspace/$workspaceId",
    validateSearch: parseWorkspaceRouteSearch,
    component: WorkspaceShell,
  });
}
