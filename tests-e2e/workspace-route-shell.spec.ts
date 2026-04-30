import { expect, test } from "@playwright/test";

import paneIds from "../product-strategy/contracts/fixtures/wu-0a-11/pane-ids.json" with { type: "json" };

test.describe("Workspace route shell", () => {
  test("serves exactly one shell at the workspace route", async ({ page }) => {
    // Risk: browser-served workspace route is missing or duplicates shell
    // roots. Level: end-to-end. Source: proposal test-intent "Browser route
    // smoke".
    await page.goto("/workspace/test-workspace");

    await expect(page.getByTestId("workspace-shell")).toHaveCount(1);
    await expect(page.getByTestId("workspace-id")).toHaveText("test-workspace");
    await expect(page.getByTestId("active-pane")).toHaveText("runtimeStatus");
  });

  for (const paneId of paneIds) {
    test(`serves active pane query ${paneId}`, async ({ page }) => {
      // Risk: browser routing fails for a documented pane query. Level:
      // end-to-end. Source: proposal test-intent "Pane query coverage".
      await page.goto(`/workspace/test-workspace?pane=${paneId}`);

      await expect(page.getByTestId("workspace-shell")).toHaveCount(1);
      await expect(page.getByTestId("active-pane")).toHaveText(paneId);
    });
  }

  test("falls unknown pane query back to runtimeStatus", async ({ page }) => {
    // Risk: browser route leaks unknown query state into the shell. Level:
    // end-to-end. Source: proposal test-intent "Unknown-pane fallback".
    await page.goto("/workspace/test-workspace?pane=unknown");

    await expect(page.getByTestId("workspace-shell")).toHaveCount(1);
    await expect(page.getByTestId("active-pane")).toHaveText("runtimeStatus");
  });
});
