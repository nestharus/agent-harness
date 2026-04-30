import { createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

import { ShellRoot } from "./ShellRoot";

const rootRoute = createRootRoute({
  component: ShellRoot,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: ShellRoot,
});

const routeTree = rootRoute.addChildren([indexRoute]);

export function createAppRouter() {
  return createRouter({ routeTree });
}

export type AppRouter = ReturnType<typeof createAppRouter>;

declare module "@tanstack/react-router" {
  interface Register {
    router: AppRouter;
  }
}
