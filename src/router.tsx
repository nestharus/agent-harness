import { Outlet, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

import { ShellRoot } from "./ShellRoot";
import { createWorkspaceRoute } from "./routes/workspace.$workspaceId";
import { createWorkspaceNodeRoute } from "./routes/workspace.$workspaceId.node.$nodeId";

function RootOutlet() {
  return <Outlet />;
}

const rootRoute = createRootRoute({
  component: RootOutlet,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: ShellRoot,
});

const workspaceRoute = createWorkspaceRoute(rootRoute);
const workspaceNodeRoute = createWorkspaceNodeRoute(rootRoute);

const routeTree = rootRoute.addChildren([indexRoute, workspaceRoute, workspaceNodeRoute]);

export function createAppRouter() {
  return createRouter({ routeTree });
}

export type AppRouter = ReturnType<typeof createAppRouter>;

declare module "@tanstack/react-router" {
  interface Register {
    router: AppRouter;
  }
}
