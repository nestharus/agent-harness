import { expect, test } from "@playwright/test";

test.describe("Phase 0A inert shell route", () => {
  test("serves a visible application root without subprocess work", async ({ page }) => {
    // Risk: browser-served frontend route is missing or not inert. Level:
    // end-to-end. Source: proposal test-intent "Inert route e2e".
    await page.goto("/");

    await expect(page.getByTestId("app-root")).toBeVisible();
    await expect(page.getByRole("heading", { name: "Agent Harness" })).toBeVisible();
  });
});
