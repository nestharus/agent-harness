import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import {
  RouterContextProvider,
  createMemoryHistory,
  createRootRoute,
  createRouter,
} from "@tanstack/react-router";
import { render, type RenderResult } from "@testing-library/react";
import type { ReactNode } from "react";
import { afterEach } from "vitest";

import canonicalSeedRenders from "../../product-strategy/contracts/fixtures/wu-0a-14b/canonical-seed-renders.json";
import ipcSeeding from "../../product-strategy/contracts/fixtures/wu-0a-14b/ipc-seeding.json";
import seedFailures from "../../product-strategy/contracts/fixtures/wu-0a-14b/seed-failure.json";
import {
  TempHarnessErrorFailure,
  parseTempHarnessError,
  parseTempHarnessHandle,
  type TempHarnessHandle,
} from "../contracts/temp-harness";
import {
  clearInvokeCommandFixtures,
  seedInvokeCommandFixtures,
  type InvokeCommandFixtureResponses,
} from "../ipc/invoke-command";

afterEach(() => {
  clearInvokeCommandFixtures();
});

interface SeedRenderFixture {
  seed_name: string;
  handle: unknown;
}

interface IpcSeedFixture {
  seed_name: string;
  responses: InvokeCommandFixtureResponses;
}

interface SeedFailureFixture {
  seed_name: string;
  expected_error: string;
}

export function renderWithHarness(ui: ReactNode, seedName: string): RenderResult {
  resolveTempHarnessSeed(seedName);
  seedHarnessIpcFixtures(seedName);

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });
  let currentUi = ui;

  function HarnessRoot() {
    return <>{currentUi}</>;
  }

  const rootRoute = createRootRoute({
    component: HarnessRoot,
  });
  const router = createRouter({
    routeTree: rootRoute,
    history: createMemoryHistory({ initialEntries: ["/"] }),
  });

  function createTree() {
    return (
      <QueryClientProvider client={queryClient}>
        <RouterContextProvider router={router}>{currentUi}</RouterContextProvider>
      </QueryClientProvider>
    );
  }

  try {
    const result = render(createTree());
    const originalRerender = result.rerender;
    const originalUnmount = result.unmount;

    return {
      ...result,
      rerender(nextUi: ReactNode) {
        currentUi = nextUi;
        originalRerender(createTree());
      },
      unmount() {
        originalUnmount();
        clearInvokeCommandFixtures();
        queryClient.clear();
      },
    };
  } catch (error) {
    clearInvokeCommandFixtures();
    queryClient.clear();
    throw error;
  }
}

function resolveTempHarnessSeed(seedName: string): TempHarnessHandle {
  const failure = (seedFailures as SeedFailureFixture[]).find(
    (candidate) => candidate.seed_name === seedName,
  );

  if (failure) {
    throw new TempHarnessErrorFailure(parseTempHarnessError(failure.expected_error), {
      seedName,
    });
  }

  const seed = (canonicalSeedRenders as SeedRenderFixture[]).find(
    (candidate) => candidate.seed_name === seedName,
  );

  if (!seed) {
    throw new TempHarnessErrorFailure("UnknownSeed", { seedName });
  }

  return parseTempHarnessHandle(seed.handle);
}

function seedHarnessIpcFixtures(seedName: string): void {
  const ipcSeed = (ipcSeeding as IpcSeedFixture[]).find(
    (candidate) => candidate.seed_name === seedName,
  );

  if (!ipcSeed) {
    throw new TempHarnessErrorFailure("FixtureManifestMissing", {
      seedName,
      cause: new Error(`Missing IPC fixture manifest for seed ${seedName}`),
    });
  }

  seedInvokeCommandFixtures(ipcSeed.responses);
}
